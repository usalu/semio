use super::*;
use semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::text::snapshot::value_to_dsl;
use semio_s_artifact_gis_gismap::MapFeature;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn map() -> GisMapSnapshot {
    let feature = |id: &str, data: serde_json::Value| MapFeature { id: id.into(), data: value_to_dsl(&data) };
    semio_s_artifact_gis_gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: vec![feature("p0", serde_json::json!({ "id": "p0", "lon": 5.5818, "lat": 50.603 }))],
        routes: vec![feature("r0", serde_json::json!({ "id": "r0", "points": [[5.5818, 50.603], [5.5825, 50.6035], [5.5901, 50.6099]] }))],
        regions: vec![feature("g0", serde_json::json!({ "id": "g0", "points": [[5.58, 50.60], [5.60, 50.60], [5.60, 50.62], [5.58, 50.62]] }))],
        ..GisMapSnapshot::default()
    })
}

/// 🔮️ lopdf (third-party, test-only) opens the bytes as a one-page PDF 1.4.
#[test]
fn the_map_is_a_one_page_pdf_1_4() {
    let pdf = serialize_bytes(&map()).expect("pdf");
    assert!(pdf.starts_with(b"%PDF-1.4"));
    let document = lopdf::Document::load_mem(&pdf).expect("independent PDF reader opens the map");
    assert_eq!(document.version, "1.4");
    assert_eq!(document.get_pages().len(), 1);
}

#[test]
fn the_declaration_carries_the_pdf_route() {
    let declaration = declaration().expect("gismap declaration with its pdf route");
    assert!(declaration.definition().capabilities().any(|capability| capability.identity().as_str() == "s.gis.gismap.composer.pdf"));
    assert!(!semio_s_artifact_gis_gismap::definition().expect("base definition").capabilities().any(|capability| capability.identity().as_str() == "s.gis.gismap.composer.pdf"));
}
