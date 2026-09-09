use super::*;

fn rect_frame(id: &str, visible: Option<bool>) -> Frame {
    Frame::Rect { id: id.into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 10.0, height: 10.0, rotation: 0.0 }, locked: None, visible, fill: None, stroke: None }
}

#[semio_framework_async_macros::async_test]
async fn frame_helpers_report_id_bounds_kind_and_visibility() {
    let rect = rect_frame("frame-1", Some(false));
    assert_eq!(rect.id(), "frame-1");
    assert_eq!(rect.kind_str(), "rect");
    assert!(!rect.visible());

    let default_visible = rect_frame("frame-2", None);
    assert!(default_visible.visible());
    assert_eq!(default_visible.bounds().width, 10.0);

    let text = Frame::Text {
        id: "frame-3".into(),
        layer_id: "layer-1".into(),
        bounds: LayoutBounds { x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0 },
        locked: None,
        visible: None,
        story_id: "story-1".into(),
        thread_next: None,
        columns: 1,
        inset: LayoutRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 },
        wrap_mode: "box".into(),
    };
    assert_eq!(text.kind_str(), "text");

    let image = Frame::Image { id: "frame-4".into(), layer_id: "layer-1".into(), bounds: LayoutBounds { x: 0.0, y: 0.0, width: 1.0, height: 1.0, rotation: 0.0 }, locked: None, visible: Some(true), link_id: "link-1".into() };
    assert_eq!(image.kind_str(), "image");
    assert!(image.visible());
}

/// 🗂️ The manifest-facing `ArtifactKindSpec.schema` matches the store envelope schema for layout
/// (unlike e.g. flow, layout uses the same string for both).
#[semio_framework_async_macros::async_test]
async fn artifact_kind_uses_the_fixture_schema() {
    assert_eq!(artifact_kind().schema, LAYOUT_DOCUMENT_SCHEMA);
    assert_eq!(artifact_kind().source_format, LAYOUT_DOCUMENT_SCHEMA);
}
