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


#[test]
fn layout_document_contract_json_text_pack_projection_and_identity() {
    let source = include_str!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(source).expect("language-neutral Layout document fixture");
    let document_json = serde_json::to_string(&fixture["document"]).expect("Layout document JSON");
    let snapshot: LayoutSnapshot = dsl::json::from_json_str(&document_json).expect("native Layout JSON decoder");
    let canonical: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&snapshot)).expect("native Layout canonical JSON");
    assert_eq!(canonical, fixture["document"], "native JSON codec must preserve the exact Layout document");

    let child = snapshot.background_drawing.as_ref().expect("background drawing child");
    assert_eq!(child.handle.child_id, child.handle.target.artifact_id);
    assert_eq!(child.handle.target.dialect.artifact_kind, "s.stdio.semio");
    assert_eq!(child.handle.target.dialect.standard, "v1");
    assert_eq!(child.handle.target.dialect.subset, "drawing");
    assert_eq!(store::ChildRestoreProjection::from_snapshot(&snapshot).expect("canonical Layout child projection").len(), 1);

    let artifact = crate::standards::v1::subsets::any::schema::LayoutArtifact::from_snapshot(snapshot.clone());
    assert_eq!(artifact.to_snapshot(), snapshot, "artifact/snapshot projection must preserve every Layout document field");
    semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&snapshot);
    semio_framework_os_kernel::os_store::test_support::assert_pack_round_trip(&snapshot);

    let diff_json = serde_json::to_string(&fixture["diff"]).expect("Layout diff JSON");
    let diff: LayoutDiff = dsl::json::from_json_str(&diff_json).expect("native Layout diff decoder");
    let canonical_diff: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&diff)).expect("native Layout canonical diff JSON");
    assert_eq!(canonical_diff, fixture["diff"], "native JSON codec must preserve the exact Layout diff");

    let mut foreign_parent = fixture["document"].clone();
    foreign_parent.as_object_mut().expect("Layout document object").insert("foreign".into(), serde_json::Value::Bool(true));
    assert!(dsl::json::from_json_str::<LayoutSnapshot>(&serde_json::to_string(&foreign_parent).expect("foreign parent JSON")).is_err());

    let mut foreign_grid = fixture["document"].clone();
    foreign_grid["grid"].as_object_mut().expect("Layout grid object").insert("foreign".into(), serde_json::Value::Bool(true));
    assert!(dsl::json::from_json_str::<LayoutSnapshot>(&serde_json::to_string(&foreign_grid).expect("foreign grid JSON")).is_err());

    let mut wrong_identity = snapshot.clone();
    wrong_identity.background_drawing.as_mut().expect("background drawing child").handle.target.artifact_id = "wrong".into();
    assert!(store::ChildRestoreProjection::from_snapshot(&wrong_identity).is_err(), "child projection must reject mismatched child identity");
    println!("[DEBUG] layout-document-contract json=text=pack=projection child-identities=1 unknown-fields=parent,nested");
}


#[semio_framework_async_macros::async_test]
async fn background_drawing_identity_depends_only_on_canonical_content() {
    let content = SemioDrawingSnapshot::default();
    let dwg = background_drawing_child_handle("dwg", &content);
    let svg = background_drawing_child_handle("svg", &content);
    assert_eq!(dwg.handle.child_id, svg.handle.child_id);
    assert_eq!(dwg.handle.child_id, dwg.handle.target.artifact_id);
    assert_eq!(dwg.handle.target.dialect.artifact_kind, "s.stdio.semio");
    assert_eq!(dwg.handle.target.dialect.standard, "v1");
    assert_eq!(dwg.handle.target.dialect.subset, "drawing");
    assert_eq!(dwg.content, content);
}
