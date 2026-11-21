use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_layout_layout_io::layout_snapshot_to_format;

fn fixture() -> serde_json::Value {
    let source = fixture_text!("../../../🧪️tests/🧫️fixtures/✅️validation/🔣️.json");
    serde_json::from_str::<serde_json::Value>(source).unwrap()["base"].clone()
}

fn snapshot(value: &serde_json::Value) -> LayoutSnapshot {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}

#[test]
fn cold_page_projection_writes_the_registered_standard_formats() {
    let doc = snapshot(&fixture());
    let svg = String::from_utf8(layout_snapshot_to_format(&doc, "svg").unwrap()).unwrap();
    assert!(svg.contains("<svg"));
    let png = layout_snapshot_to_format(&doc, "png").unwrap();
    assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"));
    assert_eq!(u32::from_be_bytes(png[16..20].try_into().unwrap()), doc.pages[0].width as u32);
    assert_eq!(u32::from_be_bytes(png[20..24].try_into().unwrap()), doc.pages[0].height as u32);
    let dxf = String::from_utf8(layout_snapshot_to_format(&doc, "dxf").unwrap()).unwrap();
    assert!(dxf.contains("SECTION") && dxf.contains("EOF"));
    let dwg = layout_snapshot_to_format(&doc, "dwg").unwrap();
    assert!(dwg.starts_with(b"AC1024"));
}

#[test]
fn cold_projection_requires_the_original_source_for_document_and_page_members() {
    let member = serde_json::json!({"handle":{"childId":"layout-member","target":{"artifactId":"drawing-head","dialect":{"artifactKind":"s.draw.drawing","standard":"1","subset":"*"}}}});
    for document_member in [true, false] {
        let mut value = fixture();
        if document_member {
            value["backgroundDrawing"] = member.clone();
        } else {
            value["pages"][0]["backgroundDrawing"] = member.clone();
        }
        let doc = snapshot(&value);
        for name in ["svg", "png", "dxf", "dwg"] {
            assert_eq!(layout_snapshot_to_format(&doc, name).unwrap_err(), "layout.export.member-source-required");
        }
    }
}
