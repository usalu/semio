//! ↩️ `change-fire-curve` — undo restores BASE's fire curve.

use super::mutation::ChangeFireCurve;
use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeFireCurve, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    vec![En1991Mutation::ChangeFireCurve(ChangeFireCurve { new_fire_curve: base.fire_curve.clone() })]
}
//#endregion 🔖️Inverse
