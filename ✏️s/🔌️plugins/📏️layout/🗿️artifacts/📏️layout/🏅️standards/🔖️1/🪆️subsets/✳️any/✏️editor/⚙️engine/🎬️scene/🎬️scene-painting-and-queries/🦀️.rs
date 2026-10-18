//! 🎬️ Scene painting and queries of the layout scene: the drop preview, the display list to scene painter, the scene query record and the document JSON scene, hit test and screen to world queries. See [`super`].

use super::*;

//#region ⚙️Scene
/// 👻️ Catalogue drop ghost rect shown while dragging onto the canvas.
#[derive(Clone, Debug)]
pub struct LayoutDropPreview {
    pub kind: String,
    pub x: f64,
    pub y: f64,
}

const DROP_PREVIEW_WIDTH: f64 = 200.0;

const DROP_PREVIEW_HEIGHT: f64 = 120.0;

fn append_drop_preview(scene: &mut Scene, transform: Affine, preview: &LayoutDropPreview) {
    if preview.kind == "page" {
        return;
    }
    let shape = Rect::new(preview.x, preview.y, preview.x + DROP_PREVIEW_WIDTH, preview.y + DROP_PREVIEW_HEIGHT);
    let fill = match preview.kind.as_str() {
        "rect" => Color::new([0.85, 0.88, 0.92, 0.45]),
        "text" => Color::new([0.2, 0.55, 0.9, 0.25]),
        "image" => Color::new([0.85, 0.45, 0.2, 0.25]),
        _ => Color::new([0.5, 0.5, 0.5, 0.3]),
    };
    scene.fill(FillRule::NonZero, transform, fill, None, &shape);
    scene.stroke(&Stroke::new(2.0), transform, Color::new([0.1, 0.45, 0.95, 0.85]), None, &shape);
}

pub fn display_list_to_scene(list: &DisplayList, chrome_blueprint: bool, camera: &Camera, viewport: &Viewport, drop_preview: Option<&LayoutDropPreview>) -> Scene {
    let mut scene = Scene::new();
    let transform = camera::camera_content_affine(camera, viewport);
    let page_bg = if chrome_blueprint { Color::new([0.97, 0.97, 0.98, 1.0]) } else { Color::new([1.0, 1.0, 1.0, 1.0]) };
    scene.fill(FillRule::NonZero, transform, page_bg, None, &Rect::new(0.0, 0.0, list.page_width as f64, list.page_height as f64));

    if chrome_blueprint {
        for guide in &list.guides {
            let stroke = match guide.kind.as_str() {
                "margin" => Color::new([0.75, 0.2, 0.2, 0.35]),
                "column" => Color::new([0.2, 0.45, 0.85, 0.25]),
                "baseline" => Color::new([0.5, 0.5, 0.5, 0.2]),
                _ => Color::new([0.3, 0.3, 0.3, 0.3]),
            };
            if guide.rect.height <= 0.0 {
                scene.stroke(&Stroke::new(1.0), transform, stroke, None, &Line::new(Point::new(guide.rect.x, guide.rect.y), Point::new(guide.rect.x + guide.rect.width, guide.rect.y)));
            } else {
                scene.stroke(&Stroke::new(1.0), transform, stroke, None, &Rect::new(guide.rect.x, guide.rect.y, guide.rect.x + guide.rect.width, guide.rect.y + guide.rect.height));
            }
        }
    }

    for piece in stack_pieces(list) {
        match piece {
            StackPiece::Stroke(stroke) | StackPiece::FrameStroke(stroke) => paint_display_stroke(&mut scene, transform, stroke),
            StackPiece::Rect(rect) => {
                let shape = RoundedRect::new(Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.width) as f64, (rect.y + rect.height) as f64), RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0));
                if let Some(fill) = &rect.fill {
                    scene.fill(FillRule::NonZero, transform, color_from(fill), None, &shape);
                }
                if let Some(stroke) = &rect.stroke {
                    let width = if rect.selected { 2.5 } else if rect.hovered { 1.75 } else { 1.0 };
                    scene.stroke(&Stroke::new(width), transform, color_from(stroke), None, &shape);
                } else if rect.selected && chrome_blueprint {
                    scene.stroke(&Stroke::new(2.0), transform, Color::new([0.1, 0.45, 0.95, 1.0]), None, &shape);
                } else if rect.hovered && chrome_blueprint {
                    scene.stroke(&Stroke::new(1.5), transform, Color::new([0.95, 0.72, 0.15, 1.0]), None, &shape);
                }
            }
            StackPiece::Image(image) => {
                let color = if image.placeholder { Color::new([0.92, 0.88, 0.84, 1.0]) } else { Color::new([0.85, 0.85, 0.85, 1.0]) };
                let shape = Rect::new(image.x as f64, image.y as f64, (image.x + image.width) as f64, (image.y + image.height) as f64);
                scene.fill(FillRule::NonZero, transform, color, None, &shape);
                if image.placeholder {
                    scene.stroke(&Stroke::new(1.0), transform, Color::new([0.75, 0.35, 0.2, 1.0]), None, &shape);
                }
                paint_preview_mark(&mut scene, transform, image);
            }
            StackPiece::Text(run) => {
                for glyph in &run.glyphs {
                    let skew = if glyph.italic { Affine::new([1.0, 0.0, 0.25, 1.0, 0.0, 0.0]) } else { Affine::IDENTITY };
                    scene.fill(FillRule::NonZero, transform * Affine::IDENTITY.translate(Vec2::new(glyph.x as f64, glyph.y as f64)) * skew * Affine::IDENTITY.scale((glyph.font_size / 16.0) as f64), color_from(&glyph.color), None, &Rect::new(0.0, -glyph.font_size as f64, 0.45, 0.0));
                }
            }
        }
    }

    if let Some(preview) = drop_preview {
        append_drop_preview(&mut scene, transform, preview);
    }

    scene
}

/// 🔭️ Bundled render/hit-test context for a single page query — page, camera/viewport, and
/// selection state. Groups {@link build_scene_from_document_json} and
/// {@link hit_test_document_json}'s shared arguments under `clippy::too_many_arguments`.
pub struct SceneQuery<'a> {
    pub page_id: &'a str,
    pub selected_ids: &'a [String],
    pub hovered_id: Option<&'a str>,
    pub chrome_blueprint: bool,
    pub camera: &'a Camera,
    pub viewport: &'a Viewport,
}

pub fn build_scene_from_document_json(engine: &mut LayoutEngine, json: &str, query: &SceneQuery<'_>, drop_preview: Option<&LayoutDropPreview>) -> Result<Scene, LayoutError> {
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(engine, &doc, page, query.page_id, query.selected_ids, query.hovered_id, query.chrome_blueprint);
    Ok(display_list_to_scene(&list, query.chrome_blueprint, query.camera, query.viewport, drop_preview))
}

pub fn hit_test_document_json(engine: &mut LayoutEngine, json: &str, sx: f64, sy: f64, query: &SceneQuery<'_>) -> Result<Option<String>, LayoutError> {
    let _ = engine;
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let world = camera::screen_to_world(query.camera, query.viewport, Point::new(sx, sy));
    Ok(hit_test_page_frames(&doc, page, world.x as f32, world.y as f32))
}

pub fn screen_to_world_json(camera: &Camera, viewport: &Viewport, sx: f64, sy: f64) -> String {
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    serde_json::json!({ "x": world.x, "y": world.y }).to_string()
}
//#endregion ⚙️Scene
