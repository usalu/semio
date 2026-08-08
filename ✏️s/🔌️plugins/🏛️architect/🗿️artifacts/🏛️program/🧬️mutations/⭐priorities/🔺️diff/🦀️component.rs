//! 🔺️ Diff fragment yielded by `Priorities`.
use crate::artifacts::program::diff::ProgramDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `Priorities` mutation — a sparse [`ProgramDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PrioritiesDiff {
    pub diff: ProgramDiff,
}

impl PrioritiesDiff {
    pub fn from_diff(diff: ProgramDiff) -> Self {
        Self { diff }
    }

    pub fn into_program_diff(self) -> ProgramDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
