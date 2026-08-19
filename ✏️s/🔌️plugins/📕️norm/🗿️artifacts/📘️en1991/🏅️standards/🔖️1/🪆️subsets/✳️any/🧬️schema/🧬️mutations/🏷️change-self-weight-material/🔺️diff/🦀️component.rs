//! 🔺️ `change-self-weight-material` — sparse diff construction.

use super::mutation::ChangeSelfWeightMaterial;
use crate::artifacts::en1991::{En1991Diff, En1991Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSelfWeightMaterial, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.self_weight_material == payload.new_self_weight_material {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Self weight material already has this value.");
    }
    protocol::MutationOutcome::new(En1991Diff { self_weight_material: Some(payload.new_self_weight_material.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
