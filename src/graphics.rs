use crate::graphics::Colors::{Black, DarkGrey, LightGrey, White};
use crate::ram::Ram;

#[derive(Clone, Copy)]
enum Colors {
    Black,
    DarkGrey,
    LightGrey,
    White,
}
type Tiles = [[u8; 2]; 8];

struct PictureProcessingUnit {
    mem: &'static Ram<0x2000>,
    screen: [[Tiles; 18]; 20],
    palette: [Colors; 4],
}

impl PictureProcessingUnit {
    pub fn new(vram: &'static Ram<0x2000>) -> Self {
        PictureProcessingUnit {
            mem: vram,
            screen: [[[[0; 2]; 8]; 18]; 20],
            palette: [White, LightGrey, DarkGrey, Black],
        }
    }

    pub fn update_palette(&mut self, new_palette: [Colors; 4]) {
        self.palette = new_palette;
    }

    pub fn update_palette_color(&mut self, target_color: Colors, indexes: &[u8]) {
        for i in indexes {
            self.palette[*i as usize] = target_color;
        }
    }

    pub fn get_pixel_color(&self, x: u8, y: u8) -> Colors {
        let x_tile = (x / 18) as usize;
        let y_tile = (y / 20) as usize;

        let col_offset = 1 << (x % 8);
        let tile_row = self.screen[y_tile][x_tile][(y % 8) as usize];

        let high_bit = if (tile_row[1] & col_offset) != 0 {
            2
        } else {
            0
        };
        let low_bit = if (tile_row[0] & col_offset) != 0 {
            1
        } else {
            0
        };

        self.palette[(high_bit + low_bit) as usize]
    }
}