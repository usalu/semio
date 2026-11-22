use crate::standards::v1::subsets::any::io::export::serializers::artifacts::{gltf::v2_0::any as gltf_out, las::v1_0::any as las_out, obj::v3_0::any as obj_out, ply::v1_0::any as ply_out, stl::v_ascii::any as stl_out, txt::v_utf_8::any as txt_out};
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any as txt_in;
use crate::GisTerrainSnapshot;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElevationFixture {
    exaggeration: f64,
    vertex_count: usize,
    triangle_count: usize,
    elevation_range_meters: [f32; 2],
    south_west_meters: [f32; 3],
    north_east_meters: [f32; 3],
}

fn elevation_fixture() -> ElevationFixture {
    serde_json::from_str(include_str!("../🧫️fixtures/🏔️liege-elevation/🔣️.json")).expect("neutral elevation fixture")
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn terrain() -> GisTerrainSnapshot {
    let terrain = crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document();
    assert_eq!(terrain.exaggeration, elevation_fixture().exaggeration);
    terrain
}

/// 🔮️ The third-party `gltf` reader (test-only) loads the export and reads its surface back.
#[test]
fn gltf_export_is_read_by_the_gltf_crate() {
    let expected = elevation_fixture();
    let bytes = gltf_out::serialize_bytes(&terrain()).expect("gltf export");
    let (document, buffers, _) = gltf::import_slice(&bytes).expect("the gltf crate imports the export");
    let primitive = document.meshes().next().expect("one mesh").primitives().next().expect("one primitive");
    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    let positions: Vec<[f32; 3]> = reader.read_positions().expect("positions").collect();
    let indices: Vec<u32> = reader.read_indices().expect("indices").into_u32().collect();
    assert_eq!((positions.len(), indices.len()), (expected.vertex_count, 3 * expected.triangle_count));
    let range = positions.iter().fold([f32::INFINITY, f32::NEG_INFINITY], |[min, max], vertex| {
        [min.min(vertex[2]), max.max(vertex[2])]
    });
    assert_eq!(range, expected.elevation_range_meters);
    for expected in [expected.south_west_meters, expected.north_east_meters] {
        assert!(positions.iter().any(|position| position.iter().zip(expected).all(|(actual, expected)| (actual - expected).abs() < 0.01)), "{expected:?}");
    }
    for face in indices.chunks_exact(3) {
        let [a, b, c] = [positions[face[0] as usize], positions[face[1] as usize], positions[face[2] as usize]];
        assert!((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) > 0.0);
    }
}

#[test]
fn mesh_formats_carry_the_sampled_surface() {
    let expected = elevation_fixture();
    let stl = String::from_utf8(stl_out::serialize_bytes(&terrain()).expect("stl")).expect("ascii stl");
    assert_eq!(stl.matches("facet normal").count(), expected.triangle_count);
    let obj = String::from_utf8(obj_out::serialize_bytes(&terrain()).expect("obj")).expect("obj text");
    assert_eq!(obj.lines().filter(|line| line.starts_with("f ")).count(), expected.triangle_count);
    let ply = ply_out::serialize_bytes(&terrain()).expect("ply");
    assert!(ply.starts_with(b"ply\n"));
    let las = semio_s_artifact_stdio_las::standards::v1_0::subsets::any::io::decode_las(&las_out::serialize_bytes(&terrain()).expect("las")).expect("decodes as las");
    assert_eq!(las.points.len(), expected.vertex_count);
}

#[test]
fn mesh_formats_require_an_authored_elevation_source() {
    let map = crate::schema::imported_map::ImportedMap::from_json(r#"{"routes":[{"id":"river","points":[[13.3,52.5],[13.4,52.5]]}]}"#)
        .expect("an imported map in another region");
    let snapshot = GisTerrainSnapshot { imported_map: Some(map), elevation_source: None, ..terrain() };
    for export in [gltf_out::serialize_bytes, stl_out::serialize_bytes, obj_out::serialize_bytes, ply_out::serialize_bytes, las_out::serialize_bytes] {
        let error = export(&snapshot).expect_err("no authored elevation source");
        assert!(error.to_string().contains("This terrain has no elevation source"));
    }
}

#[test]
fn changing_map_overlays_preserves_authored_terrain_coordinates() {
    let map = crate::schema::imported_map::ImportedMap::from_json(r#"{"routes":[{"id":"trail","points":[[5.58,50.60],[5.59,50.61]]}]}"#)
        .expect("Liège route overlay");
    let snapshot = GisTerrainSnapshot { imported_map: Some(map), ..terrain() };
    let projected = crate::gis_terrain_mesh_from_snapshot(&snapshot).expect("terrain with a route overlay");
    let demo = crate::gis_terrain_mesh_from_snapshot(&terrain()).expect("Liège demo");
    assert_eq!(projected, demo);
}

#[test]
fn clearing_map_features_preserves_the_authored_elevation_geometry() {
    let base = terrain();
    let empty = GisTerrainSnapshot {
        imported_map: Some(crate::schema::imported_map::ImportedMap::default()),
        ..base.clone()
    };
    assert_eq!(gltf_out::serialize_bytes(&base).unwrap(), gltf_out::serialize_bytes(&empty).unwrap());
}

#[test]
fn explicitly_authored_elevations_export_independently_of_map_coverage() {
    let import = crate::schema::imported_map::ImportedMap::from_json(r#"{"positions":[{"id":"far","lon":5.5818,"lat":50.603}]}"#)
        .expect("a point far from the authored grid");
    let mut rows_millimetres = vec![vec![30_000; 4]; 3];
    rows_millimetres[1][2] = 70_000;
    let snapshot = GisTerrainSnapshot {
        imported_map: Some(import),
        elevation_source: Some(crate::schema::snapshot::ElevationSource { missing_samples: None, bounds: [13.3, 52.5, 13.4, 52.6], rows_millimetres }),
        ..terrain()
    };
    let output = crate::gis_terrain_mesh_from_snapshot(&snapshot).expect("authored elevations");
    let points = &output.meshes[0].primitives[0].positions;
    assert_eq!(points.iter().filter(|point| point.z == 45.0).count(), points.len() - 1);
    assert!(points.iter().any(|point| point.z == 105.0));
}

#[test]
fn txt_is_the_exact_dsl_carrier() {
    let bytes = txt_out::serialize_bytes(&terrain()).expect("txt export");
    assert_eq!(txt_in::deserialize_bytes(&bytes).expect("txt import"), terrain());
}
