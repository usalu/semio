use crate::document::{LayoutBounds, LayoutRect, Page};

#[derive(Clone, Debug)]
pub struct DisplayColor(pub [f32; 4]);

#[derive(Clone, Debug)]
pub struct DisplayGlyph {
    pub glyph_id: u32,
    pub font_size: f32,
    pub x: f32,
    pub y: f32,
    pub color: DisplayColor,
}

#[derive(Clone, Debug)]
pub struct DisplayRect {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub fill: Option<DisplayColor>,
    pub stroke: Option<DisplayColor>,
    pub inherited: bool,
    pub selected: bool,
}

#[derive(Clone, Debug)]
pub struct DisplayGuide {
    pub rect: LayoutRect,
    pub kind: String,
}

#[derive(Clone, Debug)]
pub struct DisplayTextRun {
    pub object_id: String,
    pub glyphs: Vec<DisplayGlyph>,
}

#[derive(Clone, Debug)]
pub struct DisplayImage {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub placeholder: bool,
}

#[derive(Clone, Debug)]
pub struct DisplayList {
    pub page_id: String,
    pub page_width: f32,
    pub page_height: f32,
    pub rects: Vec<DisplayRect>,
    pub text_runs: Vec<DisplayTextRun>,
    pub images: Vec<DisplayImage>,
    pub guides: Vec<DisplayGuide>,
}

impl DisplayList {
    pub fn hit_test(&self, x: f32, y: f32) -> Option<String> {
        for rect in self.rects.iter().rev() {
            if x >= rect.x && x <= rect.x + rect.width && y >= rect.y && y <= rect.y + rect.height {
                return Some(rect.object_id.clone());
            }
        }
        for image in self.images.iter().rev() {
            if x >= image.x && x <= image.x + image.width && y >= image.y && y <= image.y + image.height {
                return Some(image.object_id.clone());
            }
        }
        None
    }
}

pub fn page_margin_guides(page: &Page) -> Vec<DisplayGuide> {
    vec![
        DisplayGuide {
            rect: LayoutRect {
                x: page.margins.left,
                y: page.margins.top,
                width: page.width - page.margins.left - page.margins.right,
                height: page.height - page.margins.top - page.margins.bottom,
            },
            kind: "margin".into(),
        },
    ]
}

pub fn bounds_to_display_rect(object_id: &str, bounds: &LayoutBounds, inherited: bool, selected: bool, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> DisplayRect {
    DisplayRect {
        object_id: object_id.into(),
        x: bounds.x as f32,
        y: bounds.y as f32,
        width: bounds.width as f32,
        height: bounds.height as f32,
        fill: fill.map(DisplayColor),
        stroke: stroke.map(DisplayColor),
        inherited,
        selected,
    }
}
