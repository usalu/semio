//! 🔺️ Diff fragment yielded by `Reports`.
use crate::artifacts::program::diff::ProgramDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// @emoji 🔺️ Diff produced by one `Reports` mutation — a sparse [`ProgramDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReportsDiff {
    pub diff: ProgramDiff,
}

impl ReportsDiff {
    pub fn from_diff(diff: ProgramDiff) -> Self {
        Self { diff }
    }

    pub fn into_program_diff(self) -> ProgramDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
