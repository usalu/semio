//! 📌️ `create-column` / `authored-top-is-stored`: the authored top constraint is stored verbatim, no derived height is stored with it, and a taller storey changes nothing stored.

use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::create_column::CreateColumn;
use crate::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight;
use crate::{Column, ModelMutation, ModelSnapshot, Point2, TopConstraint};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🏛️create-column/✅️adds-to-the-storey-top/📸️snapshot/⬅️before/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_authored_top_is_stored_and_a_taller_storey_changes_nothing_stored() {
    let before: ModelSnapshot = from_json_str(BEFORE, JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let created = Column { phase: crate::Phase::New, storey: "st-ground".into(), column_type: "ct-400".into(), position: Point2 { x: 6.0, y: 0.0 }, rotation: 0.0, tilt: None, base_offset: 0.0, top: TopConstraint::StoreyTop { offset: 0.25 }, name: "New".into() };
    let stored = apply_model_mutation(&before, &ModelMutation::CreateColumn(CreateColumn { id: "c-new".into(), column: created.clone() })).expect("the column is created");
    assert_eq!(stored.columns["c-new"].top, TopConstraint::StoreyTop { offset: 0.25 });
    let wire: serde_json::Value = serde_json::from_str(&to_json_string(&stored.columns["c-new"])).expect("the column encodes as JSON");
    let mut keys: Vec<&str> = wire.as_object().expect("a column is an object").keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["base_offset", "column_type", "name", "phase", "position", "rotation", "storey", "top"]);
    let taller = apply_model_mutation(&stored, &ModelMutation::SetStoreyHeight(SetStoreyHeight { id: "st-ground".into(), height: 3.5 })).expect("the storey grows");
    assert_eq!(taller.columns["c-new"], created);
}
