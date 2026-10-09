use super::*;
use crate::standards::v1::subsets::any::io::text::snapshot::{parse_dsl, BIM_EXAMPLE_TEXT};

fn demo() -> ModelSnapshot {
    parse_dsl(BIM_EXAMPLE_TEXT).expect("the committed demo parses")
}

#[test]
fn storeys_come_in_stacking_order() {
    let rows = storeys(&demo());
    assert_eq!(rows.iter().map(|row| (row.id.as_str(), row.level)).collect::<Vec<_>>(), vec![("st-ground", 0), ("st-first", 1)]);
    assert_eq!(rows[0].name, "Ground");
}

#[test]
fn a_plan_shows_the_stored_storey_or_else_the_lowest() {
    let model = demo();
    assert_eq!(plan_storey(&model, "st-first").as_deref(), Some("st-first"));
    assert_eq!(plan_storey(&model, "").as_deref(), Some("st-ground"));
    assert_eq!(plan_storey(&model, "gone").as_deref(), Some("st-ground"));
    assert_eq!(plan_storey(&ModelSnapshot::default(), ""), None);
}

#[test]
fn placed_elements_are_listed_with_their_storey_and_name() {
    let model = demo();
    let ids = element_ids(&model);
    assert_eq!(ids.len(), 4);
    assert!(ids.iter().all(|id| storey_of(&model, id) == Some("st-ground")));
    assert_eq!(element_name(&model, "w-east"), "East");
    assert_eq!(element_name(&model, "nothing"), "");
    assert_eq!(storey_of(&model, "nothing"), None);
}
