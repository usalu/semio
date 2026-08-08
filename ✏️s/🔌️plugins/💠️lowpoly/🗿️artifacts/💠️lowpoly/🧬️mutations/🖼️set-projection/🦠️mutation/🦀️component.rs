//! 🖼️ Lowpoly mutation — `SetProjection` payload + builder + apply.
use crate::artifacts::lowpoly::LowpolyProjection;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};


//#region 🔖️Mutation
/// @emoji 🖼️ `SetProjection` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetProjection {
    #[dsl(block)]
    pub projection: LowpolyProjection,
}

pub fn set_projection(projection: LowpolyProjection) -> LowpolyMutation {
    LowpolyMutation::SetProjection { projection }
}

pub fn apply(projection: &mut LowpolyProjection, replacement: &LowpolyProjection) {
    *projection = replacement.clone();
}
//#endregion 🔖️Mutation
