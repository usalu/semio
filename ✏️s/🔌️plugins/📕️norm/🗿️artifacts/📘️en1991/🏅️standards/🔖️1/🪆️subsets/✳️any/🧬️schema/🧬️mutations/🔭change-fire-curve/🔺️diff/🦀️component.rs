//! 🔺️ `change-fire-curve` — sparse diff construction.

use super::mutation::ChangeFireCurve;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeFireCurve, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { fire_curve: Some(payload.new_fire_curve.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
