//! 🔺️ `change-material-id` — sparse diff construction.

use super::mutation::ChangeMaterialId;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeMaterialId, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { material_id: Some(payload.new_material_id.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
