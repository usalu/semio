//! 🔺️ `change-catalog-id` — sparse diff construction.

use super::mutation::ChangeCatalogId;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCatalogId, _base: &Din4108Snapshot) -> Din4108Diff {
    Din4108Diff { catalog_id: Some(payload.new_catalog_id.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
