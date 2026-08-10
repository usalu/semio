//! 🖼️ Lowpoly mutation — `SetSnapshot` payload + builder + apply.
use crate::artifacts::lowpoly::LowpolySnapshot;
use crate::artifacts::lowpoly::mutations::LowpolyMutation;
use serde::{Deserialize, Serialize};


//#region 🔖️Mutation
/// @emoji 🖼️ `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: LowpolySnapshot,
}

pub fn set_snapshot(snapshot: LowpolySnapshot) -> LowpolyMutation {
    LowpolyMutation::SetSnapshot { snapshot }
}

pub fn apply(projection: &mut LowpolySnapshot, replacement: &LowpolySnapshot) {
    *projection = replacement.clone();
}
//#endregion 🔖️Mutation
