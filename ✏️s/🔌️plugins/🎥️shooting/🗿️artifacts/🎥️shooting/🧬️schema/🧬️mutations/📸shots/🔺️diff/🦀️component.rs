//! 🔺️ Diff fragment yielded by `Shots`.
use crate::artifacts::shooting::diff::ShootingDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff produced by one mutation — a sparse [`ShootingDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ShotsDiff {
    pub diff: ShootingDiff,
}

impl ShotsDiff {
    pub fn from_diff(diff: ShootingDiff) -> Self {
        Self { diff }
    }

    pub fn into_shooting_diff(self) -> ShootingDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
