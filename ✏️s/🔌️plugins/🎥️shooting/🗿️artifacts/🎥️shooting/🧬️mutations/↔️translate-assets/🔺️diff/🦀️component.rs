//! 🔺️ Diff fragment yielded by `TranslateAssets`.
use crate::artifacts::shooting::diff::ShootingDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Diff produced by one mutation — a sparse [`ShootingDiff`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TranslateAssetsDiff {
    pub diff: ShootingDiff,
}

impl TranslateAssetsDiff {
    pub fn from_diff(diff: ShootingDiff) -> Self {
        Self { diff }
    }

    pub fn into_shooting_diff(self) -> ShootingDiff {
        self.diff
    }
}
//#endregion 🔖️Diff
