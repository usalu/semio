//! 🎬️ CAD mutation — `SetScene` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🎬️ `SetScene` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetScene {
    pub scene: Box<CadProjection>,
}

pub fn set_scene(scene: CadProjection) -> CadMutation {
    CadMutation::SetScene { scene: Box::new(scene) }
}

pub fn apply(projection: &mut CadProjection, scene: &CadProjection) {
    *projection = scene.clone();
}
//#endregion 🔖️Mutation
