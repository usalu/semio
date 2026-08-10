//! 🔺️ Diff fragment yielded by `AddPaintLayer`.
use crate::artifacts::lowpoly::diff::LowpolyDiff;
use crate::artifacts::lowpoly::LowpolySnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `AddPaintLayer` mutation — a sparse [`LowpolyDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AddPaintLayerDiff {
    pub diff: LowpolyDiff,
}

impl AddPaintLayerDiff {
    pub fn from_diff(diff: LowpolyDiff) -> Self {
        Self { diff }
    }

    pub fn into_lowpoly_diff(self) -> LowpolyDiff {
        self.diff
    }
}

//#endregion 🔖️Diff
