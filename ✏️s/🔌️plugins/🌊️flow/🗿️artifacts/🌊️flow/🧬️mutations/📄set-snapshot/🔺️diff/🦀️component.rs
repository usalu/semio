//! Diff fragment yielded by `SetSnapshot`.
use crate::artifacts::flow::diff::FlowDiff;
use serde::{Deserialize, Serialize};

//#region 🔹Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetSnapshotDiff {
    pub diff: FlowDiff,
}

impl SetSnapshotDiff {
    pub fn from_diff(diff: FlowDiff) -> Self {
        Self { diff }
    }
    pub fn into_flow_diff(self) -> FlowDiff {
        self.diff
    }
}
//#endregion 🔹Diff
