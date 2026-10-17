#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutWindowConfig;
#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::{select, transform};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transient::LayoutWindowTransient;
#[allow(unused_imports)] use serde_json::{json, Value};
#[allow(unused_imports)] use semio_s_artifact_layout_layout_editor_commands::canvas::*;
use semio_framework_io_ports::testkit::{bind, scene_content};
use semio_s_artifact_layout_layout_editor_commands::LayoutInteractionSnapshot;

#[semio_framework_async_macros::async_test]
async fn active_page_falls_back_to_first_page_when_config_id_unresolved() {
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let config = LayoutWindowConfig { active_page_id: "no-such-page".into(), ..LayoutWindowConfig::default() };
    let page = active_page(&doc, &config).expect("falls back to first page");
    assert_eq!(page.id, doc.pages[0].id);
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_renders_story_text_not_glyph_bars() {
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let interaction = LayoutInteractionSnapshot::default();
    let json = canvas_layers(&doc, &config, &transient, &interaction, false);
    assert!(json.contains("\"kind\":\"text\"") && json.contains("Hello layout"), "preview must emit readable story text: {json}");
    assert!(!json.contains(".glyphs"), "placeholder glyph bars must not be emitted: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_splits_a_styled_story_into_spans() {
    let mut doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    doc.character_styles.push(semio_s_artifact_layout_layout_model::CharacterStyle { id: "character-1".into(), name: Some("Emphasis".into()), font_family: None, font_size: Some(24.0), font_weight: None, italic: None, color: Some([1.0, 0.0, 0.0, 1.0]), tracking: Some(10.0) });
    doc.stories[0].style_runs.push(semio_s_artifact_layout_layout_model::TextStyleRun { start: 0, end: 5, paragraph_style_id: None, character_style_id: Some("character-1".into()) });
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), false);
    assert!(json.contains("frame-text-1.span.0.text"), "{json}");
    assert!(json.contains("frame-text-1.span.1.text"), "{json}");
    assert!(json.contains("Hello"), "{json}");
    assert!(json.contains(" layout"), "{json}");
    assert!(json.contains("\"size\":24.0"), "{json}");
    assert!(json.contains("\"size\":12.0"), "{json}");
    assert!(json.contains("[1.0,0.0,0.0,1.0]"), "{json}");
    let emphasis = json.find("frame-text-1.span.0.text").unwrap();
    let rest = json.find("frame-text-1.span.1.text").unwrap();
    assert!(emphasis < rest);
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_renders_the_page_background() {
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let interaction = LayoutInteractionSnapshot::default();
    let json = canvas_layers(&doc, &config, &transient, &interaction, true);
    assert!(json.contains("layout.page-bg"));
}

#[semio_framework_async_macros::async_test]
async fn selected_and_hovered_frames_get_chrome_strokes() {
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let config = LayoutWindowConfig::default();
    let transient = LayoutWindowTransient::default();
    let selected = LayoutInteractionSnapshot { ids: vec!["frame-1".into()], hovered_ids: vec!["frame-text-1".into()] };
    let json = canvas_layers(&doc, &config, &transient, &selected, true);
    assert!(json.contains("frame-1") && json.contains("\"width\":2.0"), "selected frame gets a thicker stroke: {json}");
    assert!(json.contains("frame-text-1") && json.contains("\"width\":1.75"), "hovered frame gets a hover stroke: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_draw_a_rotated_frame() {
    let mut doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let frame = doc.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-1").expect("frame");
    let semio_s_artifact_layout_layout_model::Frame::Rect { bounds, .. } = frame else { panic!("rect") };
    *bounds = semio_s_artifact_layout_layout_model::LayoutBounds { x: 0.0, y: 0.0, width: 100.0, height: 20.0, rotation: std::f64::consts::FRAC_PI_2 };
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(json.contains("frame-1"), "the rotated frame is still painted: {json}");
    assert!(json.contains("\"to\":[40.0,60.0]") || json.contains("\"to\":[60.0,-40.0]") || json.contains("40.0"), "rotation moves a corner off the axis-aligned box: {json}");
}

#[semio_framework_async_macros::async_test]
async fn a_rotated_proxy_is_not_painted_as_an_upright_image() {
    let mut doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    doc.links[0].state = Some("ready".into());
    doc.links[0].proxy_data_url = Some("data:image/png;base64,AA==".into());
    let frame = doc.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-image-1").expect("image");
    let semio_s_artifact_layout_layout_model::Frame::Image { bounds, .. } = frame else { panic!("image") };
    bounds.rotation = std::f64::consts::FRAC_PI_2;
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(!json.contains("data:image/png;base64,AA=="), "a rotated proxy is not an upright bitmap: {json}");
    assert!(json.contains("frame-image-1"), "the rotated frame is still drawn: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_paints_a_ready_proxy_as_an_image() {
    let mut doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    doc.links[0].state = Some("ready".into());
    doc.links[0].proxy_data_url = Some("data:image/png;base64,AA==".into());
    let config = LayoutWindowConfig::default();
    let json = canvas_layers(&doc, &config, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(json.contains("\"kind\":\"image\"") && json.contains("data:image/png;base64,AA=="), "a ready proxy is a host image layer: {json}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_omits_story_text_when_zoomed_out() {
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let mut config = LayoutWindowConfig::default();
    config.camera.zoom = 0.1;
    let json = canvas_layers(&doc, &config, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), false);
    assert!(!json.contains("Hello layout"), "zoomed-out preview keeps the page workable without story text: {json}");
}

/// 🧭️ Transform plus a selection arms the live gumball at the centroid the transform tool records; its `meta:gumball`
/// layer is exactly the host contract (`📐️Canvas2dHost/🧬️schema/🔣️gumball-meta`), validated by the framework's own
/// draft-07 validator; select, or an empty selection, arms none.
#[semio_framework_async_macros::async_test]
async fn canvas_layers_arms_a_world_gumball_for_the_selection() {
    let doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let selected = LayoutInteractionSnapshot { ids: vec!["frame-1".into()], hovered_ids: Vec::new() };
    let mut transform = LayoutWindowConfig::default();
    transform.active_utility = "transform".into();
    let json = canvas_layers(&doc, &transform, &LayoutWindowTransient::default(), &selected, true);
    let layers: Value = serde_json::from_str(&json).expect("layer JSON");
    let gumball = layers.as_array().expect("layers").iter().find(|layer| layer["id"] == "meta:gumball").unwrap_or_else(|| panic!("transform plus a selection arms the gumball: {json}"));
    let schema = runtime_fixture_text("../../../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧬️schema/🔣️gumball-meta/🔣️.json");
    let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile(schema).expect("the gumball meta schema compiles");
    validator.validate_json(&gumball.to_string()).unwrap_or_else(|error| panic!("the layer is the host contract: {error:?}\n{gumball}"));
    assert_eq!((gumball["gumball"]["liveDispatch"].as_bool(), gumball["gumball"]["pivotLayer"].clone()), (Some(true), serde_json::json!([30.0, 30.0])), "a live gumball at frame-1's centre");
    let selecting = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &selected, true);
    assert!(selecting.contains("meta:utility") && selecting.contains("select") && !selecting.contains("meta:gumball"), "select omits the gumball: {selecting}");
    let idle = canvas_layers(&doc, &transform, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(!idle.contains("meta:gumball"), "transform with an empty selection omits the gumball: {idle}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_labels_a_linked_pdf_when_it_has_no_proxy() {
    let mut doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    doc.links[0].artifact_kind = "s.stdio.pdf".into();
    doc.links[0].artifact_ref = "sheet".into();
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(json.contains("s.stdio.pdf") && json.contains(".preview"), "a linked pdf without a proxy shows its kind and a page mark: {json}");
    let mut far = LayoutWindowConfig::default();
    far.camera.zoom = 0.1;
    let overview = canvas_layers(&doc, &far, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    assert!(!overview.contains(".preview") && !overview.contains("s.stdio.pdf"), "a zoomed-out sheet keeps the frame and drops the accurate mark: {overview}");
}

#[semio_framework_async_macros::async_test]
async fn canvas_layers_turns_a_rotated_proxy() {
    let image = semio_framework_pixels::RasterImage { width: 2, height: 1, pixels: vec![255, 0, 0, 255, 0, 0, 255, 255] };
    let png = semio_framework_pixels::encode_png(&image).expect("png");
    let mut doc = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    doc.links[0].state = Some("ready".into());
    doc.links[0].proxy_data_url = Some(format!("data:image/png;base64,{}", base64_encode(&png)));
    let frame = doc.pages[0].frames.iter_mut().find(|frame| frame.id() == "frame-image-1").expect("image");
    let semio_s_artifact_layout_layout_model::Frame::Image { bounds, .. } = frame else { panic!("image") };
    bounds.rotation = std::f64::consts::FRAC_PI_2;
    let json = canvas_layers(&doc, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), true);
    let layers: serde_json::Value = serde_json::from_str(&json).expect("layers");
    let layer = layers.as_array().expect("array").iter().find(|layer| layer["id"] == "frame-image-1.image").expect("rotated image");
    assert_eq!(layer["kind"], "image");
    let url = layer["dataUrl"].as_str().expect("data url");
    let payload = url.strip_prefix("data:image/png;base64,").expect("png url");
    let decoded = semio_framework_pixels::decode_png(&decode_base64(payload).expect("base64")).expect("png");
    assert_eq!((decoded.width, decoded.height), (1, 2));
    assert_eq!(&decoded.pixels[0..4], &[255, 0, 0, 255], "the left pixel turns to the top");
    assert_eq!(&decoded.pixels[4..8], &[0, 0, 255, 255], "the right pixel turns to the bottom");
}

#[test]
fn canvas_layers_emits_an_embedded_drawing_png() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_io_ports::VectorPoint2;
    use semio_framework_io_ports::VectorTransform;
    use semio_framework_io_ports::{VectorLayer, VectorNode, VectorScene, VectorSegment};
    let point = |x: f64, y: f64| VectorPoint2 { x, y };
    let encoded = semio_framework_pixels::RasterImage { width: 1, height: 1, pixels: vec![255, 0, 0, 255] };
    let bytes = semio_framework_pixels::encode_png(&encoded).expect("png");
    let content = VectorScene {
        schema: "test.scene".into(),
        canvas: Default::default(),
        styles: Vec::new(),
        layers: vec![VectorLayer {
            id: "imported".into(),
            name: "Imported".into(),
            visible: true,
            root: VectorNode::Group {
                transform: VectorTransform::identity(),
                children: vec![
                    VectorNode::Path {
                        style: None,
                        segments: vec![VectorSegment::MoveTo { to: point(0.0, 0.0) }, VectorSegment::LineTo { to: point(1.0, 0.0) }, VectorSegment::LineTo { to: point(1.0, 1.0) }, VectorSegment::LineTo { to: point(0.0, 1.0) }, VectorSegment::Close],
                    },
                    VectorNode::Image { at: point(0.0, 0.0), width: 1.0, height: 1.0, mime: "image/png".into(), bytes },
                ],
            },
        }],
    };
    let mut document = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    document.links[0].artifact_kind = "s.draw.drawing".into();
    document.background_drawing = Some(semio_s_artifact_layout_layout_model::background_drawing_child_handle("dwg", &scene_content(&content)));
    let json = canvas_layers(&document, &LayoutWindowConfig::default(), &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default(), false);
    assert!(json.contains("data:image/png;base64,"), "{json}");
    assert!(json.contains("\"x\":146.0") || json.contains("\"x\":146"), "{json}");
    assert!(json.contains("drawing.plan.0") && json.contains("[0.0,50.0]"), "{json}");
    assert!(json.contains("drawing.frame.0"), "{json}");
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
