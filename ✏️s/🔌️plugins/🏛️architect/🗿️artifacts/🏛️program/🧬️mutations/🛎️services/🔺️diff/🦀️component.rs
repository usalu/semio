//! 🔺️ Diff fragment yielded by `Services`.
use crate::artifacts::program::diff::ProgramDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `Services` mutation — a sparse [`ProgramDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ServicesDiff {
    pub diff: ProgramDiff,
}

impl ServicesDiff {
    pub fn from_diff(diff: ProgramDiff) -> Self {
        Self { diff }
    }

    pub fn into_program_diff(self) -> ProgramDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
