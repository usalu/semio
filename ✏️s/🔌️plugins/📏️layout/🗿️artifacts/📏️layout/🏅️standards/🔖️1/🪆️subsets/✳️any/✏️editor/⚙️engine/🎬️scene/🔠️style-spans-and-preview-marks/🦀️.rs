//! 🔠️ Style spans and preview marks of the layout scene: the story run glyphs, the style spans and span style resolution, the placed glyphs and the painted preview marks. See [`super`].

use super::*;

//#region ⚙️Scene
pub(super) fn story_run_glyphs(engine: &mut LayoutEngine, doc: &LayoutSnapshot, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32, base_x: f32, base_y: f32) -> Vec<DisplayGlyph> {
    let mut pen = 0.0;
    let mut glyphs = Vec::new();
    for span in style_spans(story, paragraph, doc) {
        if span.start >= span.end {
            continue;
        }
        let mut local = paragraph.clone();
        let ratio = paragraph.leading / paragraph.font_size.max(1.0);
        local.font_size = span.font_size as f64;
        local.leading = span.font_size as f64 * ratio;
        local.font_weight = span.weight.max(1.0) as u32;
        local.tracking = span.tracking as f64;
        let slice = TextStory { id: String::new(), content: story.content[span.start..span.end].to_string(), style_runs: Vec::new() };
        let (shaped, _) = layout_story_in_frame(engine, &slice, &local, (frame_width - pen).max(1.0), frame_height);
        for glyph in &shaped.glyphs {
            glyphs.push(DisplayGlyph { font: glyph.font, glyph_id: glyph.glyph_id as u32, font_size: span.font_size, x: base_x + pen + glyph.x, y: base_y + glyph.y, color: span.color.clone(), italic: span.italic });
        }
        pen += shaped.width;
    }
    glyphs
}

pub(super) struct StyleSpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) font_size: f32,
    pub(super) color: DisplayColor,
    pub(super) weight: f32,
    pub(super) tracking: f32,
    pub(super) italic: bool,
}

pub(super) fn style_spans(story: &TextStory, paragraph: &ParagraphStyle, doc: &LayoutSnapshot) -> Vec<StyleSpan> {
    let len = story.content.len();
    let mut owners: Vec<Option<usize>> = vec![None; len];
    for (index, run) in story.style_runs.iter().enumerate() {
        let start = usize::try_from(run.start).unwrap_or(len).min(len);
        let end = usize::try_from(run.end).unwrap_or(len).min(len);
        if start > end || !story.content.is_char_boundary(start) || !story.content.is_char_boundary(end) {
            continue;
        }
        for slot in owners.iter_mut().take(end).skip(start) {
            *slot = Some(index);
        }
    }
    let mut spans = Vec::new();
    let mut cursor = 0;
    while cursor < len {
        if !story.content.is_char_boundary(cursor) {
            cursor += 1;
            continue;
        }
        let owner = owners[cursor];
        let mut end = cursor + 1;
        while end < len && owners[end] == owner {
            end += 1;
        }
        while end < len && !story.content.is_char_boundary(end) {
            end += 1;
        }
        let (font_size, color, weight, tracking, italic) = span_style(owner, story, paragraph, doc);
        spans.push(StyleSpan { start: cursor, end, font_size, color, weight, tracking, italic });
        cursor = end;
    }
    spans
}

fn span_style(owner: Option<usize>, story: &TextStory, paragraph: &ParagraphStyle, doc: &LayoutSnapshot) -> (f32, DisplayColor, f32, f32, bool) {
    let plain = (paragraph.font_size as f32, DisplayColor([0.0, 0.0, 0.0, 1.0]), paragraph.font_weight as f32, paragraph.tracking as f32, false);
    let Some(index) = owner else { return plain };
    let run = &story.style_runs[index];
    let mut size = paragraph.font_size;
    let mut weight = paragraph.font_weight as f64;
    let mut tracking = paragraph.tracking;
    let mut italic = false;
    let mut color = [0.0, 0.0, 0.0, 1.0];
    if let Some(id) = &run.paragraph_style_id {
        if let Some(style) = doc.paragraph_styles.iter().find(|style| style.id == *id) {
            size = style.font_size;
            weight = style.font_weight as f64;
            tracking = style.tracking;
        }
    }
    if let Some(id) = &run.character_style_id {
        if let Some(style) = doc.character_styles.iter().find(|style| style.id == *id) {
            if let Some(font_size) = style.font_size {
                size = font_size;
            }
            if let Some(font_weight) = style.font_weight {
                weight = font_weight as f64;
            }
            if let Some(value) = style.tracking {
                tracking = value;
            }
            if let Some(value) = style.italic {
                italic = value;
            }
            if let Some(rgba) = style.color {
                color = rgba;
            }
        }
    }
    (size as f32, DisplayColor(color), weight as f32, tracking as f32, italic)
}

pub(super) fn placed_kind_glyphs(engine: Option<&mut LayoutEngine>, doc: &LayoutSnapshot, bounds: &LayoutBounds, kind: &str) -> Vec<DisplayGlyph> {
    let Some(engine) = engine else { return Vec::new() };
    let story = TextStory { id: String::new(), content: kind.to_string(), style_runs: Vec::new() };
    let mut paragraph = default_paragraph(doc);
    paragraph.font_size = 11.0;
    paragraph.leading = 13.2;
    paragraph.alignment = "left".into();
    let (shaped, _) = layout_story_in_frame(engine, &story, &paragraph, (bounds.width - 8.0).max(1.0) as f32, (bounds.height - 8.0).max(1.0) as f32);
    shaped.glyphs.iter().map(|glyph| DisplayGlyph { font: glyph.font, glyph_id: glyph.glyph_id as u32, font_size: 11.0, x: bounds.x as f32 + 4.0 + glyph.x, y: bounds.y as f32 + 4.0 + glyph.y, color: DisplayColor([0.1, 0.1, 0.1, 1.0]), italic: false }).collect()
}

pub(super) fn paint_preview_mark(scene: &mut Scene, transform: Affine, image: &DisplayImage) {
    let inset = 4.0_f64.min(image.width as f64 * 0.2).min(image.height as f64 * 0.2);
    if image.preview.is_empty() || inset < 0.5 {
        return;
    }
    let left = image.x as f64 + inset;
    let top = image.y as f64 + inset;
    let right = image.x as f64 + image.width as f64 - inset;
    let bottom = image.y as f64 + image.height as f64 - inset;
    let color = Color::new([0.2, 0.25, 0.3, 1.0]);
    let stroke = Stroke::new(1.5);
    match image.preview.as_str() {
        "page" => {
            scene.stroke(&stroke, transform, color, None, &Rect::new(left, top, right, bottom));
        }
        "stroke" => {
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, top), Point::new(right, bottom)));
        }
        "map" => {
            let mid_x = (left + right) * 0.5;
            let mid_y = (top + bottom) * 0.5;
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(mid_x, top), Point::new(mid_x, bottom)));
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, mid_y), Point::new(right, mid_y)));
        }
        "curve" => {
            let mid_x = (left + right) * 0.5;
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, bottom), Point::new(mid_x, top)));
            scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(mid_x, top), Point::new(right, bottom)));
        }
        "grid" => {
            let step_x = (right - left) / 3.0;
            let step_y = (bottom - top) / 3.0;
            for step in 1..3 {
                let y = top + step_y * step as f64;
                let x = left + step_x * step as f64;
                scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(left, y), Point::new(right, y)));
                scene.stroke(&stroke, transform, color, None, &Line::new(Point::new(x, top), Point::new(x, bottom)));
            }
        }
        _ => {}
    }
}

/// 🖼️ Chooses the fixed preview vocabulary shared by live and exported missing assets.
pub fn preview_mark(kind: &str) -> String {
    if kind.is_empty() {
        return String::new();
    }
    let mark = if kind.contains("pdf") || kind.contains("note") || kind.contains("writer") || kind.contains("forms") || kind.contains("sequence") || kind.contains("presentation") {
        "page"
    } else if kind.contains("draw") || kind.contains("dwg") || kind.contains("dxf") || kind.contains("cad") {
        "stroke"
    } else if kind.contains("map") || kind.contains("gis") || kind.contains("terrain") {
        "map"
    } else if kind.contains("svg") || kind.contains("fem") || kind.contains("equation") || kind.contains("flow") || kind.contains("wires") {
        "curve"
    } else {
        "grid"
    };
    mark.into()
}

pub(super) fn color_from(c: &DisplayColor) -> Color {
    Color::new(c.0)
}
//#endregion ⚙️Scene
