//! 🔄 CAD mutation — `RotateObjects` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji 🔄 `RotateObjects` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RotateObjects {
    pub object_ids: Vec<String>,
    pub ax: f64,
    pub ay: f64,
    pub az: f64,
    pub angle: f64,
}

pub fn rotate_objects(object_ids: Vec<String>, ax: f64, ay: f64, az: f64, angle: f64) -> CadMutation {
    CadMutation::RotateObjects { object_ids, ax, ay, az, angle }
}

pub fn apply(projection: &mut CadProjection, object_ids: &[String], ax: f64, ay: f64, az: f64, angle: f64) {
    let mutation = CadMutation::RotateObjects { object_ids: object_ids.to_vec(), ax, ay, az, angle };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
