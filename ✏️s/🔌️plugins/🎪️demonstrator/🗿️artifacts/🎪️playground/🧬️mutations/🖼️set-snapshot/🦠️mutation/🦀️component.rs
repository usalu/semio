//! 🖼️ Playground mutation — `SetSnapshot` payload + builder + apply.
use crate::artifacts::playground::mutations::PlaygroundMutation;
use crate::artifacts::playground::PlaygroundSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🖼️ `SetSnapshot` mutation payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct SetSnapshot {
    #[dsl(block)]
    pub snapshot: PlaygroundSnapshot,
}

/// 🏗️ Builds a `SetSnapshot` mutation.
pub fn set_snapshot(snapshot: PlaygroundSnapshot) -> PlaygroundMutation {
    PlaygroundMutation::SetSnapshot { snapshot }
}

/// 🧬️ Applies a snapshot replacement.
pub fn apply(doc: &mut PlaygroundSnapshot, snapshot: &PlaygroundSnapshot) {
    *doc = snapshot.clone();
}
//#endregion 🔖️Mutation
