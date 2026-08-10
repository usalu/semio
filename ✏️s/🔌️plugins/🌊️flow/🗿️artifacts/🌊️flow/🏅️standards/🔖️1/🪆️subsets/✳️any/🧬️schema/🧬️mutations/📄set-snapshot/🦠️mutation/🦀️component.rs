//! SetSnapshot mutation payload + builder + apply.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔹Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    pub snapshot: FlowSnapshot,
}

pub fn set_snapshot(snapshot: FlowSnapshot) -> FlowMutation {
    FlowMutation::SetSnapshot { snapshot }
}

pub fn apply(base: &mut FlowSnapshot, replacement: &FlowSnapshot) {
    *base = replacement.clone();
}
//#endregion 🔹Mutation
