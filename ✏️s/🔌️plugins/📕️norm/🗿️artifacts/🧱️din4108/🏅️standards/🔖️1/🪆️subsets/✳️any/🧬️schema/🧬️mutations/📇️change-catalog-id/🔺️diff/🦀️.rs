//! 🔺️ `change-catalog-id` — sparse diff construction.

use super::ChangeCatalogId;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &ChangeCatalogId, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.catalog_id == payload.new_catalog_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Catalog id already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { catalog_id: Some(payload.new_catalog_id.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
