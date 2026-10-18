//! 🎨️ Display list and hit testing of the layout scene: the display colour, glyph, rectangle, guide, text run, image and stroke records, the display list, the stacking and paint order helpers, the page frame hit test and the margin guides. See [`super`].

use super::*;

//#region 🖼️Display
#[derive(Clone, Debug)]
pub struct DisplayColor(pub [f32; 4]);

#[derive(Clone, Debug)]
pub struct DisplayGlyph {
    pub glyph_id: u32,
    pub font_size: f32,
    pub x: f32,
    pub y: f32,
    pub color: DisplayColor,
    pub italic: bool,
}

#[derive(Clone, Debug)]
pub struct DisplayRect {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub fill: Option<DisplayColor>,
    pub stroke: Option<DisplayColor>,
    pub inherited: bool,
    pub selected: bool,
    pub hovered: bool,
    pub stack: u32,
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
    pub content: String,
    pub origin_x: f32,
    pub origin_y: f32,
    pub font_size: f32,
    pub stack: u32,
    pub color: [f32; 4],
}

#[derive(Clone, Debug)]
pub struct DisplayImage {
    pub object_id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub placeholder: bool,
    pub proxy_data_url: Option<String>,
    pub preview: String,
    pub stack: u32,
}

#[derive(Clone, Debug)]
pub struct DisplayStroke {
    pub points: Vec<(f32, f32)>,
    pub closed: bool,
    pub color: [f32; 4],
    pub width: f32,
    pub fill: Option<[f32; 4]>,
    pub stack: u32,
}

pub struct DisplayList {
    pub page_id: String,
    pub page_width: f32,
    pub page_height: f32,
    pub strokes: Vec<DisplayStroke>,
    pub frame_strokes: Vec<DisplayStroke>,
    pub rects: Vec<DisplayRect>,
    pub text_runs: Vec<DisplayTextRun>,
    pub images: Vec<DisplayImage>,
    pub guides: Vec<DisplayGuide>,
}

pub enum StackPiece<'a> {
    Stroke(&'a DisplayStroke),
    Rect(&'a DisplayRect),
    Image(&'a DisplayImage),
    FrameStroke(&'a DisplayStroke),
    Text(&'a DisplayTextRun),
}

pub fn stack_pieces(list: &DisplayList) -> Vec<StackPiece<'_>> {
    let mut ranked: Vec<(u32, u8, StackPiece<'_>)> = Vec::new();
    for stroke in &list.strokes {
        ranked.push((stroke.stack, 0, StackPiece::Stroke(stroke)));
    }
    for rect in &list.rects {
        ranked.push((rect.stack, 1, StackPiece::Rect(rect)));
    }
    for image in &list.images {
        ranked.push((image.stack, 2, StackPiece::Image(image)));
    }
    for stroke in &list.frame_strokes {
        ranked.push((stroke.stack, 3, StackPiece::FrameStroke(stroke)));
    }
    for run in &list.text_runs {
        ranked.push((run.stack, 4, StackPiece::Text(run)));
    }
    ranked.sort_by_key(|(stack, rank, _)| (*stack, *rank));
    ranked.into_iter().map(|(_, _, piece)| piece).collect()
}

pub fn paint_order_ids(list: &DisplayList) -> Vec<String> {
    stack_pieces(list)
        .into_iter()
        .filter_map(|piece| match piece {
            StackPiece::Rect(rect) => Some(rect.object_id.clone()),
            StackPiece::Image(image) if !image.object_id.starts_with("drawing-") => Some(image.object_id.clone()),
            _ => None,
        })
        .collect()
}

impl DisplayList {
    pub fn hit_test(&self, x: f32, y: f32) -> Option<String> {
        for piece in stack_pieces(self).into_iter().rev() {
            match piece {
                StackPiece::Rect(rect) if x >= rect.x && x <= rect.x + rect.width && y >= rect.y && y <= rect.y + rect.height => return Some(rect.object_id.clone()),
                StackPiece::Image(image) if !image.object_id.starts_with("drawing-") && x >= image.x && x <= image.x + image.width && y >= image.y && y <= image.y + image.height => return Some(image.object_id.clone()),
                _ => {}
            }
        }
        None
    }
}

/// 👆 Topmost visible frame whose axis-aligned bounds contain `(x, y)`.
///
/// Parent frames are tested first and page frames after, matching paint order.
/// This does not construct a [`LayoutEngine`] and does not shape text.
pub fn hit_test_page_frames(doc: &LayoutSnapshot, page: &Page, x: f32, y: f32) -> Option<String> {
    let mut hit = None;
    let mut consider = |frame: &Frame| {
        if !frame.visible() {
            return;
        }
        let bounds = frame.bounds();
        if frame_bounds_contain(bounds, x, y) {
            hit = Some(frame.id().to_string());
        }
    };
    if let Some(parent_id) = &page.parent_page_id {
        if let Some(parent) = doc.parent_pages.iter().find(|parent| parent.id == *parent_id) {
            for frame in &parent.frames {
                consider(frame);
            }
        }
    }
    for frame in &page.frames {
        consider(frame);
    }
    hit
}

pub fn page_margin_guides(page: &Page) -> Vec<DisplayGuide> {
    vec![DisplayGuide { rect: LayoutRect { x: page.margins.left, y: page.margins.top, width: page.width - page.margins.left - page.margins.right, height: page.height - page.margins.top - page.margins.bottom }, kind: "margin".into() }]
}

pub fn bounds_to_display_rect(object_id: &str, bounds: &LayoutBounds, inherited: bool, selected: bool, hovered: bool, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> DisplayRect {
    DisplayRect { object_id: object_id.into(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, rotation: bounds.rotation as f32, fill: fill.map(DisplayColor), stroke: stroke.map(DisplayColor), inherited, selected, hovered, stack: 0 }
}

fn frame_bounds_contain(bounds: &LayoutBounds, x: f32, y: f32) -> bool {
    let half_w = (bounds.width as f32) * 0.5;
    let half_h = (bounds.height as f32) * 0.5;
    let cx = bounds.x as f32 + half_w;
    let cy = bounds.y as f32 + half_h;
    let dx = x - cx;
    let dy = y - cy;
    let (local_x, local_y) = if bounds.rotation.abs() < 1.0e-6 {
        (dx, dy)
    } else {
        let (sin, cos) = (-bounds.rotation).sin_cos();
        (dx * cos as f32 - dy * sin as f32, dx * sin as f32 + dy * cos as f32)
    };
    local_x.abs() <= half_w && local_y.abs() <= half_h
}
//#endregion 🖼️Display
