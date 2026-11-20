//! 📤️ Public SVG and PNG fidelity runs through the actual export job.
use super::*;

fn corpus() -> serde_json::Value { serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🎨️export-fidelity/🔣️.json")).unwrap() }
fn artifact_request(kind: LayoutExportKind) -> LayoutExportRequest { let row = corpus(); let snapshot = semio_framework_pack_json::from_json_str(&row["artifact"]["document"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(); LayoutExportRequest { kind, page_id: Some("page-1".into()), snapshot: Arc::new(snapshot), preflight_json: None, parent_document_id: "fidelity".into(), canonical_base_revision_hex: "19".repeat(32) } }
fn operation() -> Operation { Operation::new(semio_framework_job::OperationId(92), semio_framework_job::RevisionId(19), semio_framework_job::Generation(3), 17) }

#[test]
fn complete_svg_and_png_job_preserve_real_text_and_transparent_rotated_images() {
    let svg = run_layout_export_headless_batch(operation(), artifact_request(LayoutExportKind::Svg)).unwrap(); assert!(svg.data.contains('Q'), "text must export as genuine font curves"); assert!(svg.data.contains("<image")); assert!(svg.data.contains("data:image/png;base64,"));
    let png = run_layout_export_headless_batch(operation(), artifact_request(LayoutExportKind::Png)).unwrap(); let bytes = decode_base64(&png.data).unwrap(); let image = semio_framework_pixels::decode_png(&bytes).unwrap(); assert_eq!((image.width, image.height), (64, 48));
    for probe in corpus()["artifact"]["probes"].as_array().unwrap() { let x = probe["x"].as_u64().unwrap() as usize; let y = probe["y"].as_u64().unwrap() as usize; let at = (y * image.width as usize + x) * 4; for channel in 0..4 { assert!(image.pixels[at + channel].abs_diff(probe["rgba"][channel].as_u64().unwrap() as u8) <= probe["tolerance"].as_u64().unwrap() as u8); } }
}
