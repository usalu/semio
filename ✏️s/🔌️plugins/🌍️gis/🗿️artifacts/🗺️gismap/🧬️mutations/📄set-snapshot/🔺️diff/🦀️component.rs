//! Diff fragment yielded by `SetSnapshot`.
use crate::artifacts::gismap::diff::GisMapDiff;
use serde::{Deserialize, Serialize};

//#region 🔹Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SetSnapshotDiff {
    pub diff: GisMapDiff,
}

impl SetSnapshotDiff {
    pub fn from_diff(diff: GisMapDiff) -> Self { Self { diff } }
    pub fn into_gis_map_diff(self) -> GisMapDiff { self.diff }
}
//#endregion 🔹Diff
