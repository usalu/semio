//! 🔺️ Diff fragment yielded by `SetStock`.
use crate::artifacts::process3d::diff::Process3dDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff produced by one mutation — a sparse [`Process3dDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetStockDiff {
    pub diff: Process3dDiff,
}

impl SetStockDiff {
    pub fn from_diff(diff: Process3dDiff) -> Self { Self { diff } }
    pub fn into_process3d_diff(self) -> Process3dDiff { self.diff }
}
//#endregion 🔖️Diff
