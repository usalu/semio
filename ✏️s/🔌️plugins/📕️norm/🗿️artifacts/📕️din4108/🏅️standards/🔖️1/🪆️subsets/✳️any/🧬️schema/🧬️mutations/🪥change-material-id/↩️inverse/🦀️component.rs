//! ↩️ `change-material-id` — undo restores BASE's `material_id`.

use super::mutation::ChangeMaterialId;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeMaterialId, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeMaterialId(ChangeMaterialId { new_material_id: base.material_id.clone() })]
}
//#endregion 🔖️Inverse
