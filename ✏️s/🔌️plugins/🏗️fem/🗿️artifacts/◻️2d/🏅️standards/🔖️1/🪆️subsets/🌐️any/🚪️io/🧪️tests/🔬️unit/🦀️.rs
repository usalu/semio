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
    for (row, node) in rows.iter().zip(&snapshot.nodes) {
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
    let doc: semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot = semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("example fixture parses");
    assert_eq!(doc.nodes.len(), 12);
    assert_eq!(doc.elements.len(), 9);
    assert_eq!(doc.regions.len(), 1);
    assert_eq!(doc.combinations.len(), 1);

    let result = semio_s_artifact_fem_2d_engine::fem2d_engine::fem2d_solve(&doc, "dead").expect("example fixture solves");
    assert!(result.checks.residual_norm < 1e-6);

    let results = semio_s_artifact_fem_2d_engine::fem2d_engine::fem2d_solve_all(&doc).expect("example fixture solves all");
    assert!(results.contains_key("dead"), "missing dead case result");
    assert!(results.contains_key("live"), "missing live case result");
    assert!(results.contains_key("uls"), "missing uls combination result");
    assert!(results.get("dead").unwrap().checks.residual_norm < 1e-6);

    let nodal_von_mises = semio_s_artifact_fem_2d_engine::fem2d_engine::mesh_preview::fem2d_nodal_von_mises(&doc, "dead").expect("nodal von mises resolves");
    assert!(!nodal_von_mises.is_empty(), "the region carries a real area load in the dead case");

    let buckling = semio_s_artifact_fem_2d_engine::fem2d_engine::modal_buckling::fem2d_buckling(&doc, "dead").expect("buckling resolves for the dead case's compressed column");
    assert!(buckling.factors[0].is_finite() && buckling.factors[0] > 1.0, "expected an illustrative (finite, >1) load factor: {:?}", buckling.factors);
}
// #endregion 🔖️ExampleFixture
