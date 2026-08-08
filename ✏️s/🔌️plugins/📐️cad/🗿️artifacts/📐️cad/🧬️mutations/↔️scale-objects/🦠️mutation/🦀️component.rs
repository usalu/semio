//! ↔️ CAD mutation — `ScaleObjects` payload + builder + apply.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadProjection;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// @emoji ↔️ `ScaleObjects` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScaleObjects {
    pub object_ids: Vec<String>,
    pub sx: f64,
    pub sy: f64,
    pub sz: f64,
}

pub fn scale_objects(object_ids: Vec<String>, sx: f64, sy: f64, sz: f64) -> CadMutation {
    CadMutation::ScaleObjects { object_ids, sx, sy, sz }
}

pub fn apply(projection: &mut CadProjection, object_ids: &[String], sx: f64, sy: f64, sz: f64) {
    let mutation = CadMutation::ScaleObjects { object_ids: object_ids.to_vec(), sx, sy, sz };
    let diff = <CadMutation as protocol::Mutation<CadProjection>>::diff(&mutation, projection);
    *projection = <crate::artifacts::cad::diff::CadDiff as protocol::MutationDiff<CadProjection>>::apply(&diff, projection);
}
//#endregion 🔖️Mutation
