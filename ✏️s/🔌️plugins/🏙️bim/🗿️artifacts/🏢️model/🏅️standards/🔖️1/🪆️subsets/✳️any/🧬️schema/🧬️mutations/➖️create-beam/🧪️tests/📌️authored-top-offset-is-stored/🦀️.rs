//! 📌️ `create-beam` / `authored-top-offset-is-stored`: the authored top offset is stored verbatim, no derived elevation is stored with it, and a taller storey changes nothing stored.

use crate::standards::v1::subsets::any::schema::mutations::apply_model_mutation;
use crate::standards::v1::subsets::any::schema::mutations::create_beam::CreateBeam;
use crate::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight;
use crate::{Beam, ModelMutation, ModelSnapshot, Point2};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/➖️create-beam/✅️adds-below-the-storey-top/📸️snapshot/⬅️before/🔣️.json");

#[semio_framework_async_macros::async_test]
async fn the_authored_top_offset_is_stored_and_a_taller_storey_changes_nothing_stored() {
    let before: ModelSnapshot = from_json_str(BEFORE, JsonMemberPolicy::Reject).expect("before snapshot decodes");
    let created = Beam { storey: "st-ground".into(), beam_type: "bt-30x50".into(), start: Point2 { x: 0.0, y: 3.0 }, end: Point2 { x: 4.0, y: 3.0 }, top_offset: -0.2, name: "New".into() };
    let stored = apply_model_mutation(&before, &ModelMutation::CreateBeam(CreateBeam { id: "b-new".into(), beam: created.clone() })).expect("the beam is created");
    assert_eq!(stored.beams["b-new"].top_offset, -0.2);
    let wire: serde_json::Value = serde_json::from_str(&to_json_string(&stored.beams["b-new"])).expect("the beam encodes as JSON");
    let mut keys: Vec<&str> = wire.as_object().expect("a beam is an object").keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["beam_type", "end", "name", "start", "storey", "top_offset"]);
    let taller = apply_model_mutation(&stored, &ModelMutation::SetStoreyHeight(SetStoreyHeight { id: "st-ground".into(), height: 3.5 })).expect("the storey grows");
    assert_eq!(taller.beams["b-new"], created);
}
