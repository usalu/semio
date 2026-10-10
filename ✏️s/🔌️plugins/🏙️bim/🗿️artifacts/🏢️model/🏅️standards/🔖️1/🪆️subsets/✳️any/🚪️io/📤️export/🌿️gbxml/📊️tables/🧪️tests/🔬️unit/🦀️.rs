use super::*;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};

const BOX: &str = include_str!("../../../../../../🧫️fixtures/💡️inferences/🌡️energy-envelope/🏠️box/📸️snapshot/🔣️.json");

fn table() -> String {
    let model: ModelSnapshot = from_json_str(BOX, JsonMemberPolicy::Reject).expect("the box decodes");
    let plan = registry::try_with_inference(None, &model, |inferred| super::super::plan_of(&model, inferred)).expect("infers").expect("a plan");
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
