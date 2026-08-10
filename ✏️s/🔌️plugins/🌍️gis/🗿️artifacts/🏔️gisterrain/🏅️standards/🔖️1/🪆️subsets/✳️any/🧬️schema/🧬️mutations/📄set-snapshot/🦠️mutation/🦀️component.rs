//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use serde::{Deserialize, Serialize};

//#region 🔹Mutation
/// `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: GisTerrainSnapshot,
}

pub fn set_snapshot(snapshot: GisTerrainSnapshot) -> GisTerrainMutation {
    GisTerrainMutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut GisTerrainSnapshot, replacement: &GisTerrainSnapshot) {
    *base = replacement.clone();
}
//#endregion 🔹Mutation
