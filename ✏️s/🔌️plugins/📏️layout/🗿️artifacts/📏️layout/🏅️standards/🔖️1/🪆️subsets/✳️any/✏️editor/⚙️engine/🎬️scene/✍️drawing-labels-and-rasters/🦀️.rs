//! ✍️ Drawing labels and rasters of the layout scene: the drawing label collection, fitting and shaping, the background scene, the drawing raster collection with its png data urls and the raster placement. See [`super`].

use super::*;

//#region ⚙️Scene
pub(super) struct DrawingLabel {
    pub(super) text: String,
    pub(super) x: f32,
    pub(super) y: f32,
}

pub(super) fn background_scene(doc: &LayoutSnapshot) -> Option<VectorScene> {
    semio_s_artifact_layout_layout_model::background_drawing_scene(doc).ok().flatten()
}

pub(super) fn raw_drawing_labels(scene: Option<&VectorScene>) -> Vec<DrawingLabel> {
    let Some(drawing) = scene else { return Vec::new() };
    let mut labels = Vec::new();
    for layer in &drawing.layers {
        if !layer.visible {
            continue;
        }
        collect_drawing_text(&layer.root, 0.0, 0.0, &mut labels);
    }
    labels
}

fn collect_drawing_text(node: &VectorNode, tx: f64, ty: f64, labels: &mut Vec<DrawingLabel>) {
    match node {
        VectorNode::Group { transform, children } => {
            let next_x = tx + transform.translation.x;
            let next_y = ty + transform.translation.y;
            for child in children {
                collect_drawing_text(child, next_x, next_y, labels);
            }
        }
        VectorNode::Text { value, at, .. } => {
            let text = value.trim();
            if !text.is_empty() && labels.len() < 32 {
                labels.push(DrawingLabel { text: text.to_string(), x: (at.x + tx) as f32, y: (at.y + ty) as f32 });
            }
        }
        VectorNode::Path { .. } | VectorNode::Image { .. } => {}
    }
}

pub(super) fn fit_from_points(points: impl Iterator<Item = (f32, f32)>, page_width: f64, page_height: f64) -> Option<(f32, f32, f32)> {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for (x, y) in points {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    if !min_x.is_finite() {
        return None;
    }
    let width = (max_x - min_x).max(0.0);
    let height = (max_y - min_y).max(0.0);
    let scale_x = if width > 0.0 { page_width as f32 / width } else { 1.0 };
    let scale_y = if height > 0.0 { page_height as f32 / height } else { 1.0 };
    let scale = scale_x.min(scale_y);
    let ox = (page_width as f32 - width * scale) * 0.5 - min_x * scale;
    let oy = (page_height as f32 - height * scale) * 0.5 - min_y * scale;
    Some((scale, ox, oy))
}

pub(super) fn stroke_points(strokes: &[DisplayStroke]) -> impl Iterator<Item = (f32, f32)> + '_ {
    strokes.iter().flat_map(|stroke| stroke.points.iter().copied())
}

pub(super) fn place_drawing_labels(labels: &[DrawingLabel], strokes: &[DisplayStroke], bounds: &LayoutBounds) -> Vec<DrawingLabel> {
    let fit = fit_from_points(stroke_points(strokes), bounds.width, bounds.height).or_else(|| fit_from_points(labels.iter().map(|label| (label.x, label.y)), bounds.width, bounds.height));
    let Some((scale, ox, oy)) = fit else { return Vec::new() };
    let center_x = (bounds.x + bounds.width * 0.5) as f32;
    let center_y = (bounds.y + bounds.height * 0.5) as f32;
    let radians = bounds.rotation as f32;
    let (sin, cos) = radians.sin_cos();
    labels
        .iter()
        .map(|label| {
            let page_x = label.x * scale + ox + bounds.x as f32;
            let page_y = label.y * scale + oy + bounds.y as f32;
            let (x, y) = if bounds.rotation.abs() < 1.0e-6 {
                (page_x, page_y)
            } else {
                let dx = page_x - center_x;
                let dy = page_y - center_y;
                (center_x + dx * cos - dy * sin, center_y + dx * sin + dy * cos)
            };
            DrawingLabel { text: label.text.clone(), x, y }
        })
        .collect()
}

pub(super) fn shape_drawing_label(engine: &mut LayoutEngine, doc: &LayoutSnapshot, text: &str, x: f32, y: f32) -> DisplayTextRun {
    let story = TextStory { id: String::new(), content: text.to_string(), style_runs: Vec::new() };
    let mut paragraph = default_paragraph(doc);
    paragraph.font_size = 11.0;
    paragraph.leading = 13.2;
    paragraph.alignment = "left".into();
    let (shaped, _) = layout_story_in_frame(engine, &story, &paragraph, 1000.0, 1000.0);
    let glyphs = shaped.glyphs.iter().map(|glyph| DisplayGlyph { glyph_id: glyph.glyph_id as u32, font_size: 11.0, x: x + glyph.x, y: y + glyph.y, color: DisplayColor([0.1, 0.1, 0.1, 1.0]), italic: false }).collect();
    DisplayTextRun { object_id: "drawing-label".into(), glyphs, content: text.to_string(), origin_x: x, origin_y: y, font_size: 11.0, stack: 0, color: [0.0, 0.0, 0.0, 1.0] }
}

pub(super) fn unshaped_drawing_label(text: &str, x: f32, y: f32) -> DisplayTextRun {
    DisplayTextRun { object_id: "drawing-label".into(), glyphs: Vec::new(), content: text.to_string(), origin_x: x, origin_y: y, font_size: 11.0, stack: 0, color: [0.0, 0.0, 0.0, 1.0] }
}

pub(super) struct DrawingRaster {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) width: f32,
    pub(super) height: f32,
    data_url: String,
}

fn png_data_url(bytes: &[u8]) -> Option<String> {
    if bytes.len() < 8 || bytes.len() > 8192 {
        return None;
    }
    semio_framework_pixels::decode_png(bytes).ok()?;
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut encoded = String::new();
    for chunk in bytes.chunks(3) {
        let a = chunk[0];
        let b = chunk.get(1).copied().unwrap_or(0);
        let c = chunk.get(2).copied().unwrap_or(0);
        encoded.push(TABLE[(a >> 2) as usize] as char);
        encoded.push(TABLE[((a & 3) << 4 | b >> 4) as usize] as char);
        encoded.push(if chunk.len() > 1 { TABLE[((b & 15) << 2 | c >> 6) as usize] as char } else { '=' });
        encoded.push(if chunk.len() > 2 { TABLE[(c & 63) as usize] as char } else { '=' });
    }
    Some(format!("data:image/png;base64,{encoded}"))
}

pub(super) fn raw_drawing_rasters(scene: Option<&VectorScene>) -> Vec<DrawingRaster> {
    let Some(drawing) = scene else { return Vec::new() };
    let mut rasters = Vec::new();
    for layer in &drawing.layers {
        if !layer.visible {
            continue;
        }
        collect_drawing_rasters(&layer.root, 0.0, 0.0, &mut rasters);
    }
    rasters
}

fn collect_drawing_rasters(node: &VectorNode, tx: f64, ty: f64, rasters: &mut Vec<DrawingRaster>) {
    match node {
        VectorNode::Group { transform, children } => {
            let next_x = tx + transform.translation.x;
            let next_y = ty + transform.translation.y;
            for child in children {
                collect_drawing_rasters(child, next_x, next_y, rasters);
            }
        }
        VectorNode::Image { at, width, height, mime, bytes } => {
            if rasters.len() >= 8 || *width <= 0.0 || *height <= 0.0 || !mime.contains("png") {
                return;
            }
            let Some(data_url) = png_data_url(bytes) else { return };
            rasters.push(DrawingRaster { x: (at.x + tx) as f32, y: (at.y + ty) as f32, width: *width as f32, height: *height as f32, data_url });
        }
        VectorNode::Path { .. } | VectorNode::Text { .. } => {}
    }
}

pub(super) fn push_drawing_raster(images: &mut Vec<DisplayImage>, raster: &DrawingRaster, x: f32, y: f32, width: f32, height: f32, rotation: f32) {
    if width <= 0.0 || height <= 0.0 {
        return;
    }
    images.push(DisplayImage { object_id: format!("drawing-image-{}", images.len()), x, y, width, height, rotation, placeholder: false, proxy_data_url: Some(raster.data_url.clone()), preview: String::new(), stack: 0 });
}

pub(super) fn place_drawing_rasters(rasters: &[DrawingRaster], strokes: &[DisplayStroke], bounds: &LayoutBounds) -> Vec<DisplayImage> {
    let Some((scale, ox, oy)) = fit_from_points(stroke_points(strokes), bounds.width, bounds.height).or_else(|| fit_from_points(rasters.iter().flat_map(|raster| [(raster.x, raster.y), (raster.x + raster.width, raster.y + raster.height)]), bounds.width, bounds.height)) else { return Vec::new() };
    let mut images = Vec::new();
    for raster in rasters {
        push_drawing_raster(&mut images, raster, raster.x * scale + ox + bounds.x as f32, raster.y * scale + oy + bounds.y as f32, raster.width * scale, raster.height * scale, bounds.rotation as f32);
    }
    images
}
//#endregion ⚙️Scene
