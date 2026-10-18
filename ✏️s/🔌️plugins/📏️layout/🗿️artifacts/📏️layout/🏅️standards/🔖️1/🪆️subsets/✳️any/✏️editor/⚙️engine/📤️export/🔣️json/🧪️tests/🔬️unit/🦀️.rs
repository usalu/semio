//! 🧪️ Unit laws of the layout export JSON cursors: the typed writer against an independent serde oracle and the credit bound of the dynamic validator.

use super::json_validation::JSON_INPUT_BYTES_PER_UNIT;
use super::*;
use semio_framework_io_ports::testkit::{bind, scene_content};

#[test]
fn typed_document_json_matches_serde_and_every_write_is_credit_bounded() {
    bind(&semio_s_artifact_layout_layout_model::LAYOUT_BACKGROUND_DRAWING);
    use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;

    let mut snapshot = semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    snapshot.name = "\u{1f642}\n".repeat(MAX_LAYOUT_EXPORT_STRING_BYTES / 5);
    snapshot.background_drawing =
        Some(semio_s_artifact_layout_layout_model::LayoutDrawingChild { handle: store::os_store::artifact_child::ArtifactChild::new("drawing-child".into(), semio_framework_artifact_reference::ArtifactRef::parse_uri("document!s.stdio.semio@v1/drawing").expect("child reference")), content: scene_content(&Default::default()) });
    snapshot.referenced_model = Some(store::ArtifactLink { target: semio_framework_artifact_reference::ArtifactRef::parse_uri("document!s.stdio.semio@v1/model").expect("model reference"), pin: store::LinkPin::Head, role: "model".into() });
    let expected: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&snapshot)).expect("independent document JSON oracle");
    let mut cursor = TypedJsonCursor::document();
    let mut actual = Vec::new();
    loop {
        let (bytes, done) = cursor.advance(&snapshot).expect("typed JSON step");
        assert!(bytes.len() <= JSON_OUTPUT_BYTES_PER_UNIT);
        actual.extend_from_slice(&bytes);
        if done {
            break;
        }
    }
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&actual).expect("independent bounded JSON oracle"), expected);
}

#[test]
fn dynamic_json_validation_never_scans_more_than_its_input_credit() {
    let text = format!("[{}]", std::iter::repeat_n("null", MAX_LAYOUT_EXPORT_JSON_NODES - 1).collect::<Vec<_>>().join(","));
    let mut cursor = JsonValidationCursor::new(&text, true).expect("validator");
    loop {
        let before = cursor.byte_cursor;
        let done = cursor.advance(&text).expect("bounded validation step");
        assert!(cursor.byte_cursor - before <= JSON_INPUT_BYTES_PER_UNIT);
        if done {
            break;
        }
    }
}
