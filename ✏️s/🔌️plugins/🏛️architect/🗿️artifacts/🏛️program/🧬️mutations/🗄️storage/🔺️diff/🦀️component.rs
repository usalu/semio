//! 🔺️ Diff fragment yielded by `Storage`.
use crate::artifacts::program::diff::ProgramDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `Storage` mutation — a sparse [`ProgramDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StorageDiff {
    pub diff: ProgramDiff,
}

impl StorageDiff {
    pub fn from_diff(diff: ProgramDiff) -> Self {
        Self { diff }
    }

    pub fn into_program_diff(self) -> ProgramDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
