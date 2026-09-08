//! 🖼️ Layout app engine — display-list construction, glyph layout, scene painting, hit-testing and
//! export (SVG/PDF/PNG/zip). Sibling topic file of `🦀️.rs` (headless compute is split across the
//! two purely because of size — see the master template's "+ sibling topic file for big engines" note).
//!
//! Relocated wholesale from the deleted artifact-tree `⚙️engine/🎬️scene` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `LayoutEngine` owns `&mut self` rendering
//! state (font/layout contexts) threaded through canvas, pointer, wasm and window-render call sites
//! across this app — the textbook D5 Behavioral case, and one of the two `*Engine` structs in this
//! ticket that IS constructed outside its own file (see the region → destination map's exception
//! clause). `parse_layout_document`/`resolve_page` (pure, artifact-level) stayed at `🧬️schema`;
//! `compose_svg_from_drawing`/`rect_path_segments`/`LayoutError` (io/codec-dispatch territory) stayed
//! at `🚪️io` — this file reaches both by qualified path, which is the normal app→artifact direction.

use crate::io::LayoutError;
use crate::standards::v1::subsets::any::schema::{parse_layout_document, resolve_page};
use crate::{Frame, LayoutBounds, LayoutRect, LayoutSnapshot, Page, ParagraphStyle, TextStory};
use infinite_canvas::camera::{self, Camera, Viewport};
use infinite_canvas::{Affine, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};
#[cfg(test)]
use serde_json::Value;
use ui_render::{FontDependencyId, FontFamilyChoice, ShapedText, TextAlignment, TextRunStyle, TextStyle, TextSystem};

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
    pub hovered: bool,
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
    vec![DisplayGuide { rect: LayoutRect { x: page.margins.left, y: page.margins.top, width: page.width - page.margins.left - page.margins.right, height: page.height - page.margins.top - page.margins.bottom }, kind: "margin".into() }]
}

pub fn bounds_to_display_rect(object_id: &str, bounds: &LayoutBounds, inherited: bool, selected: bool, hovered: bool, fill: Option<[f32; 4]>, stroke: Option<[f32; 4]>) -> DisplayRect {
    DisplayRect { object_id: object_id.into(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, fill: fill.map(DisplayColor), stroke: stroke.map(DisplayColor), inherited, selected, hovered }
}
//#endregion 🖼️Display

//#region ⚙️Scene
static LAYOUT_SANS: &[u8] = include_bytes!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🖼️assets/🔤️MapLabelSans.ttf");

pub struct LayoutEngine {
    text: TextSystem,
    font: FontDependencyId,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    /// 🏗️ Registers the embedded `LAYOUT_SANS` font under [`TextSystem`]'s
    /// request/provide-bytes seam synchronously — never `Pending`, since the bytes are already in
    /// the binary (`include_bytes!`), unlike a real host-fetched [`FontSource`].
    pub fn new() -> Self {
        let mut text = TextSystem::new();
        let font = text.request_font("layout-sans");
        text.provide_font_bytes(font, LAYOUT_SANS);
        Self { text, font }
    }

    pub fn layout_story(&mut self, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (ShapedText, bool) {
        let style = TextRunStyle {
            base: TextStyle { family: FontFamilyChoice::Custom(self.font), size_px: paragraph.font_size as f32 },
            weight: paragraph.font_weight as f32,
            line_height_relative: (paragraph.leading / paragraph.font_size.max(1.0)) as f32,
            letter_spacing_px: paragraph.tracking as f32,
            alignment: alignment_from_str(&paragraph.alignment),
        };
        let shaped = self.text.shape_paragraph(&story.content, &style, frame_width);
        let overset = shaped.height > frame_height;
        (shaped, overset)
    }
}

fn alignment_from_str(value: &str) -> TextAlignment {
    match value {
        "center" | "middle" => TextAlignment::Middle,
        "right" => TextAlignment::Right,
        "justify" | "justified" => TextAlignment::Justified,
        _ => TextAlignment::Left,
    }
}

fn default_paragraph(doc: &LayoutSnapshot) -> ParagraphStyle {
    doc.paragraph_styles.first().cloned().unwrap_or(ParagraphStyle { id: "paragraph.body".into(), name: "Body".into(), font_family: "Layout Sans".into(), font_size: 12.0, font_weight: 400, leading: 14.4, tracking: 0.0, alignment: "left".into() })
}

pub fn layout_story_in_frame(engine: &mut LayoutEngine, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (ShapedText, bool) {
    engine.layout_story(story, paragraph, frame_width, frame_height)
}

pub fn build_display_list_for_page(engine: &mut LayoutEngine, doc: &LayoutSnapshot, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool) -> DisplayList {
    let resolved = resolve_page(doc, page);
    let mut rects = Vec::new();
    let mut text_runs = Vec::new();
    let mut images = Vec::new();
    let mut guides = if chrome_blueprint && page.id == active_page_id { page_margin_guides(page) } else { Vec::new() };

    if chrome_blueprint && page.id == active_page_id {
        for guide in &page.guides {
            guides.push(DisplayGuide { rect: guide.clone(), kind: "guide".into() });
        }
        let col_count = page.columns.count.max(1) as f64;
        let col_width = (page.width - page.margins.left - page.margins.right - page.columns.gutter * (col_count - 1.0)) / col_count;
        for i in 0..page.columns.count {
            let x = page.margins.left + (i as f64) * (col_width + page.columns.gutter);
            guides.push(DisplayGuide { rect: LayoutRect { x, y: page.margins.top, width: col_width, height: page.height - page.margins.top - page.margins.bottom }, kind: "column".into() });
        }
        if doc.grid.snap_to_baseline && doc.grid.baseline_grid > 0.0 {
            let mut y = doc.grid.baseline_offset;
            while y < page.height {
                guides.push(DisplayGuide { rect: LayoutRect { x: 0.0, y, width: page.width, height: 0.0 }, kind: "baseline".into() });
                y += doc.grid.baseline_grid;
            }
        }
    }

    for item in resolved {
        if !item.frame.visible() {
            continue;
        }
        let selected = selected_ids.iter().any(|id| id == item.frame.id());
        let hovered = hovered_id.is_some_and(|id| id == item.frame.id());
        match &item.frame {
            Frame::Rect { id, bounds, fill, stroke, .. } => {
                rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, *fill, stroke.or(if chrome_blueprint && item.inherited { Some([0.4, 0.5, 0.7, 0.8]) } else { None })));
            }
            Frame::Text { id, bounds, story_id, inset, .. } => {
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.2, 0.55, 0.9, 0.9])));
                }
                if let Some(story) = doc.stories.iter().find(|s| s.id == *story_id) {
                    let paragraph = default_paragraph(doc);
                    let frame_width = (bounds.width - inset.width - inset.x * 2.0).max(1.0) as f32;
                    let frame_height = (bounds.height - inset.height - inset.y * 2.0).max(1.0) as f32;
                    let (shaped, _overset) = layout_story_in_frame(engine, story, &paragraph, frame_width, frame_height);
                    let base_x = (bounds.x + inset.x) as f32;
                    let base_y = (bounds.y + inset.y) as f32;
                    let font_size = paragraph.font_size as f32;
                    let glyphs = shaped.glyphs.iter().map(|glyph| DisplayGlyph { glyph_id: glyph.glyph_id as u32, font_size, x: base_x + glyph.x, y: base_y + glyph.y, color: DisplayColor([0.0, 0.0, 0.0, 1.0]) }).collect();
                    text_runs.push(DisplayTextRun { object_id: id.clone(), glyphs });
                }
            }
            Frame::Image { id, bounds, link_id, .. } => {
                let link = doc.links.iter().find(|l| l.id == *link_id);
                let placeholder = link.is_none_or(|l| l.state.as_deref() == Some("missing") || l.proxy_data_url.is_none());
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(id, bounds, item.inherited, selected, hovered, None, Some([0.85, 0.45, 0.2, 0.9])));
                }
                images.push(DisplayImage { object_id: id.clone(), x: bounds.x as f32, y: bounds.y as f32, width: bounds.width as f32, height: bounds.height as f32, placeholder });
            }
        }
    }

    DisplayList { page_id: page.id.clone(), page_width: page.width as f32, page_height: page.height as f32, rects, text_runs, images, guides }
}

fn color_from(c: &DisplayColor) -> Color {
    Color::new(c.0)
}

/// @emoji 👻️ Catalogue drop ghost rect shown while dragging onto the canvas.
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

    for rect in &list.rects {
        let shape = RoundedRect::new(Rect::new(rect.x as f64, rect.y as f64, (rect.x + rect.width) as f64, (rect.y + rect.height) as f64), RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0));
        if let Some(fill) = &rect.fill {
            scene.fill(FillRule::NonZero, transform, color_from(fill), None, &shape);
        }
        if let Some(stroke) = &rect.stroke {
            let width = if rect.selected {
                2.5
            } else if rect.hovered {
                1.75
            } else {
                1.0
            };
            scene.stroke(&Stroke::new(width), transform, color_from(stroke), None, &shape);
        } else if rect.selected && chrome_blueprint {
            scene.stroke(&Stroke::new(2.0), transform, Color::new([0.1, 0.45, 0.95, 1.0]), None, &shape);
        } else if rect.hovered && chrome_blueprint {
            scene.stroke(&Stroke::new(1.5), transform, Color::new([0.95, 0.72, 0.15, 1.0]), None, &shape);
        }
    }

    for image in &list.images {
        let color = if image.placeholder { Color::new([0.92, 0.88, 0.84, 1.0]) } else { Color::new([0.85, 0.85, 0.85, 1.0]) };
        let shape = Rect::new(image.x as f64, image.y as f64, (image.x + image.width) as f64, (image.y + image.height) as f64);
        scene.fill(FillRule::NonZero, transform, color, None, &shape);
        if image.placeholder {
            scene.stroke(&Stroke::new(1.0), transform, Color::new([0.75, 0.35, 0.2, 1.0]), None, &shape);
        }
    }

    for run in &list.text_runs {
        for glyph in &run.glyphs {
            scene.fill(
                FillRule::NonZero,
                transform * Affine::IDENTITY.translate(Vec2::new(glyph.x as f64, glyph.y as f64)) * Affine::IDENTITY.scale((glyph.font_size / 16.0) as f64),
                color_from(&glyph.color),
                None,
                &Rect::new(0.0, -glyph.font_size as f64, 0.45, 0.0),
            );
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
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == query.page_id).ok_or_else(|| LayoutError::PageNotFound(query.page_id.to_string()))?;
    let list = build_display_list_for_page(engine, &doc, page, query.page_id, query.selected_ids, query.hovered_id, true);
    let world = camera::screen_to_world(query.camera, query.viewport, Point::new(sx, sy));
    Ok(list.hit_test(world.x as f32, world.y as f32))
}

pub fn screen_to_world_json(camera: &Camera, viewport: &Viewport, sx: f64, sy: f64) -> String {
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    serde_json::json!({ "x": world.x, "y": world.y }).to_string()
}
//#endregion ⚙️Scene

//#region 📤️Export
#[cfg(test)]
pub use crate::editor::layout::engine::export::{export_document_pdf_headless_batch, export_document_png_headless_batch, export_document_svg_headless_batch, export_package_zip_headless_batch};
//#endregion 📤️Export

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
