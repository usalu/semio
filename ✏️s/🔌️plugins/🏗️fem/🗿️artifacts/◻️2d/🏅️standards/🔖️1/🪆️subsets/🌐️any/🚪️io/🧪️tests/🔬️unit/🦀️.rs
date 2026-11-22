use crate::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any::csv_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::obj::v3_0::any::obj_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::stl::v_ascii::any::stl_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;

/// 📄️ This subset's handcrafted `.semio` DSL example asset — the same bytes the shipped example
/// serves to the shell.
const EXAMPLE: &str = crate::examples::demo::PRIMARY_TEXT;

#[semio_framework_async_macros::async_test]
async fn txt_round_trips_the_example() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    let printed = dsl_text(&snapshot);
    assert_eq!(from_dsl_text(&printed).unwrap(), snapshot, "txt is not a fixed point");
}

#[semio_framework_async_macros::async_test]
async fn json_round_trips_the_example() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    assert_eq!(from_json_text(&json_text(&snapshot)).expect("json must read back"), snapshot, "json is not a lossless round trip");
}

/// 🔮️ The third-party `csv` reader (test-only) reads the node table back coordinate for coordinate.
#[semio_framework_async_macros::async_test]
async fn csv_is_the_node_table_read_by_a_third_party_reader() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    let text = csv_text(&snapshot);
    let mut reader = csv::Reader::from_reader(text.as_bytes());
    assert_eq!(reader.headers().expect("header").iter().collect::<Vec<_>>(), ["id", "x", "y"]);
    let rows: Vec<csv::StringRecord> = reader.records().map(|row| row.expect("row")).collect();
    assert_eq!(rows.len(), snapshot.nodes.len());
    for (row, node) in rows.iter().zip(snapshot.nodes.get().iter()) {
        assert_eq!(&row[0], node.id.as_str());
        let read: Vec<f64> = row.iter().skip(1).map(|value| value.parse().expect("number")).collect();
        assert_eq!(read, [node.x, node.y].to_vec(), "coordinates survive bit for bit");
    }
}

#[semio_framework_async_macros::async_test]
async fn geometry_exports_are_real() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    assert!(stl_text(&snapshot).expect("stl export must succeed").starts_with("solid"), "stl export must be real ascii stl");
    obj_text(&snapshot).expect("obj export must succeed");
}

#[semio_framework_async_macros::async_test]
async fn io_declaration_registers_every_declared_hop() {
    let declaration = super::io();
    assert_eq!(declaration.entries.len(), 7, "txt and json both ways, csv, stl and obj export only");
    let own = "s.fem.fem2d";
    let mut foreign: Vec<&str> = Vec::new();
    for entry in declaration.entries {
        assert!(entry.from.artifact_kind == own || entry.into.artifact_kind == own, "every entry must touch this subset's own dialect");
        foreign.push(if entry.from.artifact_kind == own { entry.into.artifact_kind } else { entry.from.artifact_kind });
    }
    foreign.sort_unstable();
    foreign.dedup();
    assert_eq!(foreign, vec!["framework.io.text", "s.stdio.csv", "s.stdio.json", "s.stdio.obj", "s.stdio.stl"]);
}

// #region 🔖️ExampleFixture
#[test]
fn example_fixture_parses_and_solves() {
    use pack::snapshot::ArtifactDsl;
    use semio_s_artifact_fem_2d_engine::fem2d_engine::{fem2d_solve, fem2d_solve_all, mesh_preview::fem2d_nodal_von_mises, modal_buckling::fem2d_buckling};
    const DEAD: &str = "scope:1:8:loadCase4:dead";
    let doc: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("example fixture parses");
    let counts = (doc.nodes.len(), doc.elements.len(), doc.regions.len(), doc.combinations.len());
    let result = fem2d_solve(&doc, "dead");
    let results = fem2d_solve_all(&doc);
    let nodal_von_mises = results.as_ref().ok().and_then(|results| results.get(DEAD)).map(|result| fem2d_nodal_von_mises(&doc, result));
    let buckling = fem2d_buckling(&doc, "dead");
    semio_framework_value::FromValue::retire_decoded(doc);

    assert_eq!(counts, (12, 9, 1, 1));
    assert!(result.expect("example fixture solves").checks.residual_norm < 1e-6);
    let results = results.expect("example fixture solves all");
    assert!(results.contains_key("scope:1:8:loadCase4:live"), "missing live case result");
    assert!(results.contains_key("scope:1:11:combination3:uls"), "missing uls combination result");
    assert!(results[DEAD].checks.residual_norm < 1e-6);

    let nodal_von_mises = nodal_von_mises.expect("missing dead case result").expect("nodal von mises resolves");
    assert!(!nodal_von_mises.is_empty(), "the region carries a real area load in the dead case");

    let buckling = buckling.expect("buckling resolves for the dead case's compressed column");
    assert!(buckling.factors[0].is_finite() && buckling.factors[0] > 1.0, "expected an illustrative (finite, >1) load factor: {:?}", buckling.factors);
}
// #endregion 🔖️ExampleFixture

/// 📐️ Every vector of the portable region surface corpus runs through the installed OBJ and ASCII STL exports beside the neutral kernel; the rows are handed to the owning TypeScript oracle through `SEMIO_FEM_SURFACE_RESULTS`.
/// Links: the corpus `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🧫️fixtures/🔣️.json`.
#[semio_framework_async_macros::async_test]
async fn surface_corpus_exports_obj_and_stl_beside_the_neutral_surface() {
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
    let path = std::env::var_os("SEMIO_FEM_SURFACE_RESULTS").expect("owning surface contract command must declare its output path");
    let fixture: serde_json::Value = serde_json::from_str(semio_framework_test_fixtures::repo_text!("✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/📐️surface/🧪️tests/🧫️fixtures/🔣️.json")).expect("surface corpus");
    let vectors = fixture["vectors"].as_array().expect("vectors");
    assert_eq!(vectors.len(), 9);
    let mut observed = Vec::new();
    for vector in vectors {
        let mut snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
        snapshot.regions.clear();
        let mut regions = Vec::new();
        for input in vector["regions"].as_array().expect("regions") {
            let id = input["regionId"].as_str().expect("region id").to_string();
            let outline: Vec<[f64; 2]> = serde_json::from_value(input["domain"]["outer"].clone()).expect("outer");
            let holes: Vec<Vec<[f64; 2]>> = serde_json::from_value(input["domain"]["holes"].clone()).expect("holes");
            let thickness = input["thickness"].as_f64().expect("thickness");
            let mesh_size = input["options"]["maxEdge"].as_f64().expect("max edge");
            assert_eq!(input["options"]["minAngleDeg"].as_f64().expect("min angle"), 20.0, "the installed export meshes with the declared 20 degree minimum angle");
            let domain = semio_s_fem_mesh::PlanarDomain { outer: outline.clone(), holes: holes.clone() };
            let options = semio_s_fem_mesh::MeshOpts { max_edge: mesh_size, min_angle_deg: 20.0 };
            if let Ok(triangulated) = semio_s_fem_mesh::triangulate(&domain, &options) {
                let volume = semio_s_fem_mesh_derived::split_to_tets(&semio_s_fem_mesh_derived::extrude_tri_mesh(&triangulated, thickness, 1));
                let triangles = semio_s_fem_mesh_derived::boundary_faces(&volume);
                let tetrahedra: Vec<[u32; 4]> = volume.cells.iter().map(|cell| match cell { semio_s_fem_mesh_derived::Cell::Tet4(tet) => *tet, _ => panic!("split_to_tets retained a non-tetrahedral cell") }).collect();
                regions.push(serde_json::json!({"regionId": id, "points": volume.points, "tetrahedra": tetrahedra, "triangles": triangles}));
            }
            snapshot.regions.push(FemRegion { id: id.clone(), name: id, outline, holes, thickness, material_id: String::new(), mesh_size });
        }
        observed.push(serde_json::json!({"id": vector["id"], "neutral": {"regions": regions}, "obj": obj_text(&snapshot).expect("obj export must succeed"), "stl": stl_text(&snapshot).expect("stl export must succeed")}));
    }
    let bytes = serde_json::to_vec(&observed).expect("rows");
    let path = std::path::Path::new(&path);
    std::fs::create_dir_all(path.parent().expect("parent")).expect("directory");
    std::fs::write(path, bytes).expect("results");
}
