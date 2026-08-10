//! 🎬️ CAD mutation — `SetSnapshot` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🎬️ `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    pub snapshot: Box<CadSnapshot>,
}

pub fn set_snapshot(scene: CadSnapshot) -> CadMutation {
    CadMutation::SetSnapshot { snapshot: Box::new(scene) }
}

pub fn apply(projection: &mut CadSnapshot, scene: &CadSnapshot) {
    *projection = scene.clone();
}
//#endregion 🔖️Mutation
