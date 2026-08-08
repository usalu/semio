//! 🔺️ Diff fragment yielded by mutation.
use crate::artifacts::cad::diff::CadDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one mutation — a sparse [`CadDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TranslateObjectsDiff {
    pub diff: CadDiff,
}

impl TranslateObjectsDiff {
    pub fn from_diff(diff: CadDiff) -> Self {
        Self { diff }
    }

    pub fn into_cad_diff(self) -> CadDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
