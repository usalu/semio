//! 🔺️ Diff fragment yielded by `SetSnapshot`.
use crate::artifacts::playground::diff::PlaygroundDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff produced by one `SetSnapshot` mutation — a sparse [`PlaygroundDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetSnapshotDiff {
    pub diff: PlaygroundDiff,
}

impl SetSnapshotDiff {
    /// 🏗️ Wraps a sparse field delta.
    pub fn from_diff(diff: PlaygroundDiff) -> Self {
        Self { diff }
    }

    /// 📤 Unwraps the sparse field delta.
    pub fn into_playground_diff(self) -> PlaygroundDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
