use std::borrow::Cow;
use std::sync::{Arc, OnceLock};

use fontique::Blob;
use parley::{
    Alignment, AlignmentOptions, FontContext, FontStack, FontWeight, Layout, LayoutContext, LineHeight, PositionedLayoutItem,
    StyleProperty,
};
use infinite_cavas::camera::{self, Camera, Viewport};
use infinite_cavas::{Affine, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};

use crate::display::{
    bounds_to_display_rect, page_margin_guides, DisplayColor, DisplayGlyph, DisplayGuide, DisplayImage, DisplayList, DisplayTextRun,
};
use crate::document::{parse_layout_document, resolve_page, Frame, LayoutDocument, Page, ParagraphStyle, TextStory};

static LAYOUT_SANS: &[u8] = include_bytes!("../../infinite/cavas/rs/asset/MapLabelSans.ttf");

pub struct LayoutEngine {
    pub font_context: FontContext,
    pub layout_context: LayoutContext<[u8; 4]>,
    fonts_ready: bool,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self {
            font_context: FontContext::new(),
            layout_context: LayoutContext::new(),
            fonts_ready: false,
        }
    }

    fn ensure_fonts(&mut self) {
        if self.fonts_ready {
            return;
        }
        self.font_context
            .collection
            .register_fonts(Blob::new(Arc::new(LAYOUT_SANS.to_vec())), None);
        self.fonts_ready = true;
    }

    pub fn layout_story(&mut self, story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (Layout<[u8; 4]>, bool) {
        self.ensure_fonts();
        let mut builder = self
            .layout_context
            .ranged_builder(&mut self.font_context, &story.content, 1.0, true);
        builder.push_default(StyleProperty::FontSize(paragraph.font_size as f32));
        builder.push_default(StyleProperty::FontStack(FontStack::Source(Cow::Borrowed("Layout Sans"))));
        builder.push_default(StyleProperty::FontWeight(FontWeight::new(paragraph.font_weight as f32)));
        builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(
            (paragraph.leading / paragraph.font_size.max(1.0)) as f32,
        )));
        builder.push_default(StyleProperty::LetterSpacing(paragraph.tracking as f32));
        let mut layout = builder.build(&story.content);
        layout.break_all_lines(Some(frame_width));
        layout.align(Some(frame_width), alignment_from_str(&paragraph.alignment), AlignmentOptions::default());
        let overset = layout.height() > frame_height;
        (layout, overset)
    }
}

static ENGINE: OnceLock<std::sync::Mutex<LayoutEngine>> = OnceLock::new();

fn engine() -> &'static std::sync::Mutex<LayoutEngine> {
    ENGINE.get_or_init(|| std::sync::Mutex::new(LayoutEngine::new()))
}

fn alignment_from_str(value: &str) -> Alignment {
    match value {
        "center" | "middle" => Alignment::Middle,
        "right" => Alignment::Right,
        "justify" | "justified" => Alignment::Justified,
        _ => Alignment::Left,
    }
}

fn default_paragraph(doc: &LayoutDocument) -> ParagraphStyle {
    doc.paragraph_styles.first().cloned().unwrap_or(ParagraphStyle {
        id: "paragraph.body".into(),
        name: "Body".into(),
        font_family: "Layout Sans".into(),
        font_size: 12.0,
        font_weight: 400,
        leading: 14.4,
        tracking: 0.0,
        alignment: "left".into(),
    })
}

pub fn layout_story_in_frame(story: &TextStory, paragraph: &ParagraphStyle, frame_width: f32, frame_height: f32) -> (Layout<[u8; 4]>, bool) {
    engine().lock().expect("layout engine").layout_story(story, paragraph, frame_width, frame_height)
}

pub fn build_display_list_for_page(doc: &LayoutDocument, page: &Page, active_page_id: &str, selected_ids: &[String], hovered_id: Option<&str>, chrome_blueprint: bool) -> DisplayList {
    let resolved = resolve_page(doc, page);
    let mut rects = Vec::new();
    let mut text_runs = Vec::new();
    let mut images = Vec::new();
    let mut guides = if chrome_blueprint && page.id == active_page_id {
        page_margin_guides(page)
    } else {
        Vec::new()
    };

    if chrome_blueprint && page.id == active_page_id {
        for guide in &page.guides {
            guides.push(DisplayGuide {
                rect: guide.clone(),
                kind: "guide".into(),
            });
        }
        let col_count = page.columns.count.max(1) as f64;
        let col_width = (page.width - page.margins.left - page.margins.right - page.columns.gutter * (col_count - 1.0)) / col_count;
        for i in 0..page.columns.count {
            let x = page.margins.left + (i as f64) * (col_width + page.columns.gutter);
            guides.push(DisplayGuide {
                rect: crate::document::LayoutRect {
                    x,
                    y: page.margins.top,
                    width: col_width,
                    height: page.height - page.margins.top - page.margins.bottom,
                },
                kind: "column".into(),
            });
        }
        if doc.grid.snap_to_baseline && doc.grid.baseline_grid > 0.0 {
            let mut y = doc.grid.baseline_offset;
            while y < page.height {
                guides.push(DisplayGuide {
                    rect: crate::document::LayoutRect {
                        x: 0.0,
                        y,
                        width: page.width,
                        height: 0.0,
                    },
                    kind: "baseline".into(),
                });
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
                rects.push(bounds_to_display_rect(
                    id,
                    bounds,
                    item.inherited,
                    selected,
                    hovered,
                    *fill,
                    stroke.or(if chrome_blueprint && item.inherited {
                        Some([0.4, 0.5, 0.7, 0.8])
                    } else {
                        None
                    }),
                ));
            }
            Frame::Text { id, bounds, story_id, inset, .. } => {
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(
                        id,
                        bounds,
                        item.inherited,
                        selected,
                        hovered,
                        None,
                        Some([0.2, 0.55, 0.9, 0.9]),
                    ));
                }
                if let Some(story) = doc.stories.iter().find(|s| s.id == *story_id) {
                    let paragraph = default_paragraph(doc);
                    let frame_width = (bounds.width - inset.width - inset.x * 2.0).max(1.0) as f32;
                    let frame_height = (bounds.height - inset.height - inset.y * 2.0).max(1.0) as f32;
                    let (layout, _overset) = layout_story_in_frame(story, &paragraph, frame_width, frame_height);
                    let mut glyphs = Vec::new();
                    let base_x = (bounds.x + inset.x) as f32;
                    let base_y = (bounds.y + inset.y) as f32;
                    for line in layout.lines() {
                        for positioned in line.items() {
                            if let PositionedLayoutItem::GlyphRun(run) = positioned {
                                let font_size = paragraph.font_size as f32;
                                for glyph in run.positioned_glyphs() {
                                    glyphs.push(DisplayGlyph {
                                        glyph_id: glyph.id as u32,
                                        font_size,
                                        x: base_x + glyph.x,
                                        y: base_y + glyph.y,
                                        color: DisplayColor([0.0, 0.0, 0.0, 1.0]),
                                    });
                                }
                            }
                        }
                    }
                    text_runs.push(DisplayTextRun {
                        object_id: id.clone(),
                        glyphs,
                    });
                }
            }
            Frame::Image { id, bounds, link_id, .. } => {
                let link = doc.links.iter().find(|l| l.id == *link_id);
                let placeholder = link
                    .map(|l| l.state.as_deref() == Some("missing") || l.proxy_data_url.is_none())
                    .unwrap_or(true);
                if chrome_blueprint {
                    rects.push(bounds_to_display_rect(
                        id,
                        bounds,
                        item.inherited,
                        selected,
                        hovered,
                        None,
                        Some([0.85, 0.45, 0.2, 0.9]),
                    ));
                }
                images.push(DisplayImage {
                    object_id: id.clone(),
                    x: bounds.x as f32,
                    y: bounds.y as f32,
                    width: bounds.width as f32,
                    height: bounds.height as f32,
                    placeholder,
                });
            }
        }
    }

    DisplayList {
        page_id: page.id.clone(),
        page_width: page.width as f32,
        page_height: page.height as f32,
        rects,
        text_runs,
        images,
        guides,
    }
}

fn color_from(c: &DisplayColor) -> Color {
    Color::new(c.0)
}

/// @emoji 👻 Catalogue drop ghost rect shown while dragging onto the canvas.
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
    scene.stroke(
        &Stroke::new(2.0),
        transform,
        Color::new([0.1, 0.45, 0.95, 0.85]),
        None,
        &shape,
    );
}

pub fn display_list_to_scene(
    list: &DisplayList,
    chrome_blueprint: bool,
    camera: &Camera,
    viewport: &Viewport,
    drop_preview: Option<&LayoutDropPreview>,
) -> Scene {
    let mut scene = Scene::new();
    let transform = camera::camera_content_affine(camera, viewport);
    let page_bg = if chrome_blueprint {
        Color::new([0.97, 0.97, 0.98, 1.0])
    } else {
        Color::new([1.0, 1.0, 1.0, 1.0])
    };
    scene.fill(
        FillRule::NonZero,
        transform,
        page_bg,
        None,
        &Rect::new(0.0, 0.0, list.page_width as f64, list.page_height as f64),
    );

    if chrome_blueprint {
        for guide in &list.guides {
            let stroke = match guide.kind.as_str() {
                "margin" => Color::new([0.75, 0.2, 0.2, 0.35]),
                "column" => Color::new([0.2, 0.45, 0.85, 0.25]),
                "baseline" => Color::new([0.5, 0.5, 0.5, 0.2]),
                _ => Color::new([0.3, 0.3, 0.3, 0.3]),
            };
            if guide.rect.height <= 0.0 {
                scene.stroke(
                    &Stroke::new(1.0),
                    transform,
                    stroke,
                    None,
                    &Line::new(
                        Point::new(guide.rect.x, guide.rect.y),
                        Point::new(guide.rect.x + guide.rect.width, guide.rect.y),
                    ),
                );
            } else {
                scene.stroke(
                    &Stroke::new(1.0),
                    transform,
                    stroke,
                    None,
                    &Rect::new(guide.rect.x, guide.rect.y, guide.rect.x + guide.rect.width, guide.rect.y + guide.rect.height),
                );
            }
        }
    }

    for rect in &list.rects {
        let shape = RoundedRect::new(
            Rect::new(
                rect.x as f64,
                rect.y as f64,
                (rect.x + rect.width) as f64,
                (rect.y + rect.height) as f64,
            ),
            RoundedRectRadii::new(0.0, 0.0, 0.0, 0.0),
        );
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
            scene.stroke(
                &Stroke::new(width),
                transform,
                color_from(stroke),
                None,
                &shape,
            );
        } else if rect.selected && chrome_blueprint {
            scene.stroke(
                &Stroke::new(2.0),
                transform,
                Color::new([0.1, 0.45, 0.95, 1.0]),
                None,
                &shape,
            );
        } else if rect.hovered && chrome_blueprint {
            scene.stroke(
                &Stroke::new(1.5),
                transform,
                Color::new([0.95, 0.72, 0.15, 1.0]),
                None,
                &shape,
            );
        }
    }

    for image in &list.images {
        let color = if image.placeholder {
            Color::new([0.92, 0.88, 0.84, 1.0])
        } else {
            Color::new([0.85, 0.85, 0.85, 1.0])
        };
        let shape = Rect::new(image.x as f64, image.y as f64, (image.x + image.width) as f64, (image.y + image.height) as f64);
        scene.fill(FillRule::NonZero, transform, color, None, &shape);
        if image.placeholder {
            scene.stroke(
                &Stroke::new(1.0),
                transform,
                Color::new([0.75, 0.35, 0.2, 1.0]),
                None,
                &shape,
            );
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

pub fn build_scene_from_document_json(
    json: &str,
    page_id: &str,
    selected_ids: &[String],
    hovered_id: Option<&str>,
    chrome_blueprint: bool,
    camera: &Camera,
    viewport: &Viewport,
    drop_preview: Option<&LayoutDropPreview>,
) -> Result<Scene, String> {
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| format!("page {page_id} not found"))?;
    let list = build_display_list_for_page(&doc, page, page_id, selected_ids, hovered_id, chrome_blueprint);
    Ok(display_list_to_scene(&list, chrome_blueprint, camera, viewport, drop_preview))
}

pub fn hit_test_document_json(
    json: &str,
    page_id: &str,
    sx: f64,
    sy: f64,
    selected_ids: &[String],
    hovered_id: Option<&str>,
    camera: &Camera,
    viewport: &Viewport,
) -> Result<Option<String>, String> {
    let doc = parse_layout_document(json)?;
    let page = doc.pages.iter().find(|p| p.id == page_id).ok_or_else(|| format!("page {page_id} not found"))?;
    let list = build_display_list_for_page(&doc, page, page_id, selected_ids, hovered_id, true);
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    Ok(list.hit_test(world.x as f32, world.y as f32))
}

pub fn screen_to_world_json(camera: &Camera, viewport: &Viewport, sx: f64, sy: f64) -> String {
    let world = camera::screen_to_world(camera, viewport, Point::new(sx, sy));
    serde_json::json!({ "x": world.x, "y": world.y }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_scene_from_empty_document() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":[],"layers":[],"frames":[],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 1.0 };
        let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
        let scene = build_scene_from_document_json(json, "page-1", &[], None, true, &camera, &viewport, None).expect("scene");
        let _ = scene;
    }

    #[test]
    fn hit_test_respects_camera_zoom() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":400,"height":400,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let camera = Camera { x: 0.0, y: 0.0, zoom: 0.5 };
        let viewport = Viewport { width: 400, height: 300, dpr: 1.0 };
        let hit = hit_test_document_json(json, "page-1", 210.0, 160.0, &[], None, &camera, &viewport).expect("hit");
        assert_eq!(hit.as_deref(), Some("frame-1"));
    }

    #[test]
    fn marks_hovered_frame_rect() {
        let json = r#"{"schema":"layout.fixture","name":"t","camera":{"x":0,"y":0,"zoom":1},"previewCamera":{"x":0,"y":0,"zoom":1},"grid":{"baselineGrid":12,"baselineOffset":0,"snapToBaseline":true},"paragraphStyles":[{"id":"paragraph.body","name":"Body","fontFamily":"Layout Sans","fontSize":12,"fontWeight":400,"leading":14.4,"tracking":0,"alignment":"left"}],"characterStyles":[],"stories":[],"links":[],"parentPages":[],"spreads":[],"pages":[{"id":"page-1","name":"P","spreadId":"s","width":200,"height":200,"margins":{"top":0,"right":0,"bottom":0,"left":0},"columns":{"count":1,"gutter":0},"guides":[],"layerIds":["layer-1"],"layers":[{"id":"layer-1","name":"Content","visible":true,"locked":false,"objectIds":["frame-1"]}],"frames":[{"id":"frame-1","layerId":"layer-1","kind":"rect","bounds":{"x":10,"y":10,"w":40,"h":40,"rotation":0},"fill":[1,1,1,1]}],"overrides":[]}]}"#;
        let doc = parse_layout_document(json).expect("doc");
        let page = doc.pages.first().expect("page");
        let list = build_display_list_for_page(&doc, page, "page-1", &[], Some("frame-1"), true);
        assert!(list.rects.iter().any(|rect| rect.object_id == "frame-1" && rect.hovered));
        assert!(list.rects.iter().all(|rect| rect.object_id != "frame-1" || rect.hovered));
    }
}
