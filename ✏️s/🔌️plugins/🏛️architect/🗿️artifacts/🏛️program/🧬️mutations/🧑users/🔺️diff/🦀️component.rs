//! 🔺️ Diff fragment yielded by `Users`.
use crate::artifacts::program::diff::ProgramDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `Users` mutation — a sparse [`ProgramDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UsersDiff {
    pub diff: ProgramDiff,
}

impl UsersDiff {
    pub fn from_diff(diff: ProgramDiff) -> Self {
        Self { diff }
    }

    pub fn into_program_diff(self) -> ProgramDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
