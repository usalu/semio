//! 🔺️ Diff fragment yielded by `ObjectsPatch`.
use crate::artifacts::lowpoly::diff::LowpolyDiff;
use crate::artifacts::lowpoly::LowpolySnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `ObjectsPatch` mutation — a sparse [`LowpolyDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ObjectsPatchDiff {
    pub diff: LowpolyDiff,
}

impl ObjectsPatchDiff {
    pub fn from_diff(diff: LowpolyDiff) -> Self {
        Self { diff }
    }

    pub fn into_lowpoly_diff(self) -> LowpolyDiff {
        self.diff
    }
}

//#endregion 🔖️Diff
