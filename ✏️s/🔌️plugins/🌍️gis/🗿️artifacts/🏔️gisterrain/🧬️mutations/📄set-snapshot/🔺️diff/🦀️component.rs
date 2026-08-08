//! Diff fragment yielded by `SetSnapshot`.
use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use serde::{Deserialize, Serialize};

//#region 🔹Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetSnapshotDiff {
    pub diff: GisTerrainDiff,
}

impl SetSnapshotDiff {
    pub fn from_diff(diff: GisTerrainDiff) -> Self { Self { diff } }
    pub fn into_gis_terrain_diff(self) -> GisTerrainDiff { self.diff }
}
//#endregion 🔹Diff
