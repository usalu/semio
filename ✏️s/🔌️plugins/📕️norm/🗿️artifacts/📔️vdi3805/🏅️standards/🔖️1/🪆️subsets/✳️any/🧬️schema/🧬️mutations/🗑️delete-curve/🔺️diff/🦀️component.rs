//! 🔺️ `delete-curve` — sparse diff construction.

use super::mutation::DeleteCurve;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteCurve, base: &Vdi3805Snapshot) -> Vdi3805Diff {
    let mut curves = base.curves.clone();
    curves.remove(&payload.id);
    Vdi3805Diff { curves: Some(curves), ..Default::default() }
}
//#endregion 🔖️Diff
