//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::gismap::GisMapSnapshot;
use crate::artifacts::gismap::mutations::GisMapMutation;
use serde::{Deserialize, Serialize};

//#region 🔹Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: GisMapSnapshot,
}

pub fn set_snapshot(snapshot: GisMapSnapshot) -> GisMapMutation {
    GisMapMutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut GisMapSnapshot, replacement: &GisMapSnapshot) {
    *base = replacement.clone();
}
//#endregion 🔹Mutation
