use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::{ConstructionRow, OpeningRow, Plan, SpaceRow, SurfaceRow};
#[allow(unused_imports)]
use std::collections::BTreeMap;
use semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::tables::table_json;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_test_fixtures::fixture_text;

static BOX: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../../🧪️tests/🧫️fixtures/💡️inferences/🌡️energy-envelope/🏠️box/📸️snapshot/🔣️.json"));

fn table() -> String {
    let model: ModelSnapshot = from_json_str((*BOX), JsonMemberPolicy::Reject).expect("the box decodes");
    let plan = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::try_with_inference(None, &model, |inferred| semio_s_artifact_bim_model_gbxml::standards::v1::subsets::any::io::export::gbxml::plan_of(&model, inferred)).expect("infers").expect("a plan");
    table_json(&plan, &model)
}

#[test]
fn the_table_keys_every_space_and_surface_by_its_cad_id() {
    let table = table();
    assert!(table.contains("\"spaces\":{\"sp\":{"), "{table}");
    assert!(table.contains("\"surfaces\":{\"sp/c1\":{") && table.contains("\"sp/w1\":{"), "{table}");
    assert!(table.contains("\"o-win\":{\"kind\":\"FixedWindow\"") && table.contains("\"o-door\":{\"kind\":\"NonSlidingDoor\""), "{table}");
}

#[test]
fn the_roof_without_data_has_no_construction_or_u_value_and_the_totals_add_the_types() {
    let table = table();
    let roof = &table[table.find("\"sp/c1\":{").expect("the roof")..];
    let roof = &roof[..roof.find("\"openings\"").expect("its openings")];
    assert!(!roof.contains("u_value") && !roof.contains("construction"), "{roof}");
    assert!(table.contains("\"by_type\":{") && table.contains("\"Roof\":9.99"), "{table}");
}

#[test]
fn the_table_is_deterministic() {
    assert_eq!(table(), table());
}
