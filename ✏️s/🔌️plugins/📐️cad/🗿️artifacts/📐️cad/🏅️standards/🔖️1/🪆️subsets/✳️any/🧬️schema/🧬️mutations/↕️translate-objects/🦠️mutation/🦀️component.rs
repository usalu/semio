//! ↕️ CAD mutation — `TranslateObjects` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ↕️ `TranslateObjects` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslateObjects {
    pub object_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

pub fn translate_objects(object_ids: Vec<String>, dx: f64, dy: f64, dz: f64) -> CadMutation {
    CadMutation::TranslateObjects { object_ids, dx, dy, dz }
}

pub fn apply(projection: &mut CadSnapshot, object_ids: &[String], dx: f64, dy: f64, dz: f64) {
    let mutation = CadMutation::TranslateObjects { object_ids: object_ids.to_vec(), dx, dy, dz };
    let diff = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
