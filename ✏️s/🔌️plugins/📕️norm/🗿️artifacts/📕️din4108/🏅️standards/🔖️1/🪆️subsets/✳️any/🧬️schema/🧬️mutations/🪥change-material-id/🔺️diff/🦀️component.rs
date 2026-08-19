//! 🔺️ `change-material-id` — sparse diff construction.

use super::mutation::ChangeMaterialId;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeMaterialId, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.material_id == payload.new_material_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Material id already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { material_id: Some(payload.new_material_id.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
