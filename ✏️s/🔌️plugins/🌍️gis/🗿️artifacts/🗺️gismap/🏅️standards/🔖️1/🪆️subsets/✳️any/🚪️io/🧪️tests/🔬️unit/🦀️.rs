use crate::io::export::serializers::artifacts::{dwg::v_ac1018::any as dwg_out, dxf::v_r12::any as dxf_out, pdf::v1_4::any as pdf_out, png::v1_2::any as png_out, svg::v1_1::any as svg_out, txt::v_utf_8::any as txt_out};
use crate::io::import::deserializers::artifacts::{dwg::v_ac1018::any as dwg_in, dxf::v_r12::any as dxf_in, txt::v_utf_8::any as txt_in};
use crate::standards::v1::subsets::any::schema::value_to_dsl;
use crate::{gis_map_snapshot_with_derived_children, GisMapSnapshot, MapFeature};
use geo::{Area, Coord, LineString, Polygon};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn feature(id: &str, data: serde_json::Value) -> MapFeature {
    MapFeature { id: id.into(), data: value_to_dsl(&data) }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn map() -> GisMapSnapshot {
    gis_map_snapshot_with_derived_children(GisMapSnapshot {
        positions: vec![feature("p0", serde_json::json!({ "id": "p0", "lon": 5.5818, "lat": 50.603 })), feature("p1", serde_json::json!({ "id": "p1", "lon": 5.59, "lat": 50.61 }))],
        routes: vec![feature("r0", serde_json::json!({ "id": "r0", "points": [[5.5818, 50.603], [5.5825, 50.6035], [5.5901, 50.6099]] }))],
        regions: vec![feature("g0", serde_json::json!({ "id": "g0", "points": [[5.58, 50.60], [5.60, 50.60], [5.60, 50.62], [5.58, 50.62]] }))],
        ..GisMapSnapshot::default()
    })
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn chain(feature: &MapFeature) -> Vec<[f64; 2]> {
    let value = crate::standards::v1::subsets::any::schema::dsl_to_value(&feature.data);
    value["points"].as_array().expect("points").iter().map(|p| [p[0].as_f64().expect("x"), p[1].as_f64().expect("y")]).collect()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn lon_lat(feature: &MapFeature) -> [f64; 2] {
    let value = crate::standards::v1::subsets::any::schema::dsl_to_value(&feature.data);
    [value["lon"].as_f64().expect("lon"), value["lat"].as_f64().expect("lat")]
}

/// 🔮️ `geo` (third-party, test-only) measures the region before export and after import.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn geo_area(ring: &[[f64; 2]]) -> f64 {
    Polygon::new(LineString::from(ring.iter().map(|p| Coord { x: p[0], y: p[1] }).collect::<Vec<_>>()), Vec::new()).unsigned_area()
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_world_round_trip(back: &GisMapSnapshot) {
    let original = map();
    assert_eq!((back.positions.len(), back.routes.len(), back.regions.len()), (2, 1, 1));
    for (a, b) in original.positions.iter().zip(&back.positions) {
        let (a, b) = (lon_lat(a), lon_lat(b));
        assert!((a[0] - b[0]).abs() < 1e-9 && (a[1] - b[1]).abs() < 1e-9, "{a:?} vs {b:?}");
    }
    assert_eq!(chain(&back.routes[0]), chain(&original.routes[0]));
    let area = geo_area(&chain(&original.regions[0]));
    assert!((geo_area(&chain(&back.regions[0])) - area).abs() < 1e-12 * area.max(1.0), "region area survives");
}

#[test]
fn dxf_carries_every_feature_in_world_coordinates() {
    let bytes = dxf_out::serialize_bytes(&map()).expect("dxf export");
    let text = String::from_utf8(bytes.clone()).expect("dxf is text");
    assert!(text.contains("CIRCLE") && text.contains("POLYLINE") && text.contains("5.5818"), "{text}");
    assert_world_round_trip(&dxf_in::deserialize_bytes(&bytes).expect("dxf import"));
}

#[test]
fn dwg_carries_every_feature_in_world_coordinates() {
    let bytes = dwg_out::serialize_bytes(&map()).expect("dwg export");
    assert!(bytes.starts_with(b"AC10"), "a DWG version sentinel opens the file");
    assert_world_round_trip(&dwg_in::deserialize_bytes(&bytes).expect("dwg import"));
}

#[test]
fn page_formats_are_real_files_of_their_format() {
    let svg = String::from_utf8(svg_out::serialize_bytes(&map()).expect("svg")).expect("utf-8");
    assert!(svg.contains("<svg") && svg.matches("<path").count() == 4, "{svg}");
    let pdf = pdf_out::serialize_bytes(&map()).expect("pdf");
    assert!(pdf.starts_with(b"%PDF-1.4"));
    let png = semio_s_artifact_stdio_png::io::decode_png(&png_out::serialize_bytes(&map()).expect("png")).expect("decodes as png");
    assert_eq!((png.width, png.height), (256, 256));
    assert!(png.pixels.chunks(4).filter(|px| px[3] > 0).count() > 100, "markers and lines are painted");
}

#[test]
fn txt_is_the_exact_dsl_carrier() {
    let bytes = txt_out::serialize_bytes(&map()).expect("txt export");
    assert_eq!(txt_in::deserialize_bytes(&bytes).expect("txt import"), map());
}

mod geojson_io {
    use super::feature;
    use crate::io::export::serializers::artifacts::json::v_rfc8259::geojson as geojson_out;
    use crate::io::import::deserializers::artifacts::json::v_rfc8259::geojson as geojson_in;
    use crate::standards::v1::subsets::any::schema::dsl_to_value;
    use crate::{gis_map_snapshot_with_derived_children, GisMapSnapshot, MapFeature};
    use serde_json::Value;
    use std::str::FromStr;

    const VECTORS: &str = include_str!("../../../🧫️fixtures/🌍️geojson-io/🔣️.json");

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn features(value: &Value) -> Vec<MapFeature> {
        value.as_array().expect("feature array").iter().map(|item| feature(item["id"].as_str().expect("id"), item["data"].clone())).collect()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn map_of(value: &Value) -> GisMapSnapshot {
        gis_map_snapshot_with_derived_children(GisMapSnapshot { positions: features(&value["positions"]), routes: features(&value["routes"]), regions: features(&value["regions"]), ..GisMapSnapshot::default() })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn json_of(features: &[MapFeature]) -> Vec<(String, Value)> {
        features.iter().map(|feature| (feature.id.clone(), dsl_to_value(&feature.data))).collect()
    }

    #[test]
    fn language_neutral_vectors_export_and_import_exactly() {
        let vectors: Value = serde_json::from_str(VECTORS).expect("vectors are JSON");
        for case in vectors["export"].as_array().expect("export cases") {
            let written: Value = serde_json::from_slice(&geojson_out::serialize_bytes(&map_of(&case["map"])).expect("export")).expect("exported JSON");
            assert_eq!(written, case["geojson"], "{}", case["id"]);
        }
        for case in vectors["import"].as_array().expect("import cases") {
            let imported = geojson_in::deserialize_bytes(case["geojson"].to_string().as_bytes()).expect("import");
            let expected = map_of(&case["map"]);
            assert_eq!(json_of(&imported.positions), json_of(&expected.positions), "{}", case["id"]);
            assert_eq!(json_of(&imported.routes), json_of(&expected.routes), "{}", case["id"]);
            assert_eq!(json_of(&imported.regions), json_of(&expected.regions), "{}", case["id"]);
            assert_eq!(imported, expected, "{}", case["id"]);
        }
    }

    /// 🔮️ The third-party `geojson` crate reads the export of the bundled Liège example — 152 points
    /// and 149 routes — with the coordinates the map holds, and the sibling import restores the map.
    #[test]
    fn geojson_crate_reads_the_bundled_example_export() {
        let example = crate::standards::v1::subsets::any::schema::default_document();
        let bytes = geojson_out::serialize_bytes(&example).expect("the bundled example exports");
        let geojson::GeoJson::FeatureCollection(collection) = geojson::GeoJson::from_str(std::str::from_utf8(&bytes).expect("utf-8")).expect("the geojson crate reads our export") else {
            panic!("the export is a FeatureCollection")
        };
        assert_eq!(collection.features.len(), example.positions.len() + example.routes.len() + example.regions.len());
        assert_eq!((example.positions.len(), example.routes.len()), (152, 149));
        for (oracle, position) in collection.features.iter().zip(&example.positions) {
            let data = dsl_to_value(&position.data);
            let Some(geojson::Geometry { value: geojson::GeometryValue::Point { coordinates }, .. }) = &oracle.geometry else { panic!("{} is a Point", position.id) };
            assert_eq!(coordinates.as_slice(), [data["lon"].as_f64().expect("lon"), data["lat"].as_f64().expect("lat")].as_slice());
            assert_eq!(oracle.properties.as_ref().and_then(|properties| properties.get("kind")), data.get("kind"));
        }
        for (oracle, route) in collection.features[example.positions.len()..].iter().zip(&example.routes) {
            let Some(geojson::Geometry { value: geojson::GeometryValue::LineString { coordinates }, .. }) = &oracle.geometry else { panic!("{} is a LineString", route.id) };
            let expected: Vec<Vec<f64>> = serde_json::from_value(dsl_to_value(&route.data)["points"].clone()).expect("points");
            assert_eq!(coordinates.iter().map(|position| position.as_slice().to_vec()).collect::<Vec<_>>(), expected);
        }
        let back = geojson_in::deserialize_bytes(&bytes).expect("our import reads our export");
        assert_eq!((json_of(&back.positions), json_of(&back.routes), json_of(&back.regions)), (json_of(&example.positions), json_of(&example.routes), json_of(&example.regions)));
    }

    /// 🔮️ A FeatureCollection the `geojson` crate writes (a region with a hole, a 3D point) imports as
    /// map features, and the oracle's own polygon area survives our export of them.
    #[test]
    fn import_reads_what_the_geojson_crate_writes() {
        let exterior = vec![[0.0, 0.0], [4.0, 0.0], [4.0, 3.0], [0.0, 3.0], [0.0, 0.0]];
        let hole = vec![[1.0, 1.0], [1.0, 2.0], [2.0, 2.0], [2.0, 1.0], [1.0, 1.0]];
        let written = geojson::GeoJson::from(geojson::FeatureCollection {
            features: vec![
                geojson::Feature { id: Some(geojson::feature::Id::String("tower".into())), geometry: Some(geojson::Geometry::new_point([5.6, 50.6, 90.0])), properties: None, ..Default::default() },
                geojson::Feature { id: Some(geojson::feature::Id::String("yard".into())), geometry: Some(geojson::Geometry::new_polygon([exterior.clone(), hole.clone()])), properties: None, ..Default::default() },
            ],
            bbox: None,
            foreign_members: None,
        })
        .to_string_pretty()
        .expect("oracle writes");
        let imported = geojson_in::deserialize_bytes(written.as_bytes()).expect("import");
        assert_eq!(dsl_to_value(&imported.positions[0].data), serde_json::json!({ "id": "tower", "lon": 5.6, "lat": 50.6, "alt": 90.0 }));
        assert_eq!(dsl_to_value(&imported.regions[0].data)["holes"].as_array().expect("holes").len(), 1);
        let again = geojson::GeoJson::from_str(std::str::from_utf8(&geojson_out::serialize_bytes(&imported).expect("export")).expect("utf-8")).expect("oracle reads");
        let geojson::GeoJson::FeatureCollection(collection) = again else { panic!("collection") };
        let Some(geojson::Geometry { value: geojson::GeometryValue::Polygon { coordinates }, .. }) = &collection.features[1].geometry else { panic!("polygon") };
        let area = |ring: &[geojson::Position]| super::geo_area(&ring.iter().map(|position| [position[0], position[1]]).collect::<Vec<_>>());
        assert_eq!((area(&coordinates[0]), area(&coordinates[1])), (12.0, 1.0), "exterior and hole areas survive the round trip");
    }
}
