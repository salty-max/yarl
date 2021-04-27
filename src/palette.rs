use std::collections::HashMap;

use crate::prelude::*;

pub fn color_from_palette(c: &str) -> Result<RGB, String> {
    let mut palette = HashMap::new();
    palette.insert("black", "#000000");
    palette.insert("dark_blue", "#1D2B53");
    palette.insert("dark_purple", "#7E2553");
    palette.insert("dark_green", "#008751");
    palette.insert("brown", "#AB5236");
    palette.insert("dark_grey", "#5F574F");
    palette.insert("light_grey", "#C2C3C7");
    palette.insert("dark_grey", "#5F574F");
    palette.insert("white", "#FFF1E8");
    palette.insert("red", "#FF004D");
    palette.insert("orange", "#FFA300");
    palette.insert("yellow", "#FFEC27");
    palette.insert("green", "#00E436");
    palette.insert("blue", "#29ADFF");
    palette.insert("lavender", "#83769C");
    palette.insert("pink", "#FF77A8");
    palette.insert("light_peach", "#FFCCAA");

    let mut color: Option<RGB> = None;

    for (k, v) in palette.iter() {
        if &c == k {
            color = Some(RGB::from_hex(v).unwrap());
        }
    }

    match color {
        Some(color) => Ok(color),
        None => Err(String::from("Invalid color")),
    }
}
