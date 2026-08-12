//! 🔺️ `insert-point` — sparse diff construction.

use super::mutation::InsertPoint;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalPoint, MathematicalSnapshot};

//#region 🔖️Diff
/// 🔺️ Ascending-insert-clamped: an out-of-range `index` lands at the end rather than panicking.
pub fn diff(payload: &InsertPoint, base: &MathematicalSnapshot) -> MathematicalDiff {
    let mut geometry = base.geometry.clone();
    let index = payload.index.min(geometry.points.len());
    geometry.points.insert(index, MathematicalPoint { x: payload.x, y: payload.y });
    MathematicalDiff { geometry: Some(geometry), ..Default::default() }
}
//#endregion 🔖️Diff
