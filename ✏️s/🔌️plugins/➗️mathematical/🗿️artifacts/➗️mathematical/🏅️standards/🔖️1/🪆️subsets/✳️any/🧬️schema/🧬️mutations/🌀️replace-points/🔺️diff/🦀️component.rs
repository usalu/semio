//! 🔺️ `replace-points` — sparse diff construction.

use super::mutation::ReplacePoints;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalGeometry, MathematicalSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePoints, _base: &MathematicalSnapshot) -> MathematicalDiff {
    MathematicalDiff { geometry: Some(MathematicalGeometry { points: payload.points.clone() }), ..Default::default() }
}
//#endregion 🔖️Diff
