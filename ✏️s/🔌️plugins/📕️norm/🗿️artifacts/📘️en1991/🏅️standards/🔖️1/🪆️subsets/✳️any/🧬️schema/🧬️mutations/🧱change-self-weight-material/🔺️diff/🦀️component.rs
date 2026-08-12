//! 🔺️ `change-self-weight-material` — sparse diff construction.

use super::mutation::ChangeSelfWeightMaterial;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeSelfWeightMaterial, _base: &En1991Snapshot) -> En1991Diff {
    En1991Diff { self_weight_material: Some(payload.new_self_weight_material.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
