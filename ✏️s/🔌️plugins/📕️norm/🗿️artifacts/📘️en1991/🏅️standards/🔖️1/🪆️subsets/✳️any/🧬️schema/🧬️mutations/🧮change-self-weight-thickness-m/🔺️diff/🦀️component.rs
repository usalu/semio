//! 🔺️ `change-self-weight-thickness-m` — sparse diff construction.

use super::mutation::ChangeSelfWeightThicknessM;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSelfWeightThicknessM, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { self_weight_thickness_m: Some(payload.new_self_weight_thickness_m.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
