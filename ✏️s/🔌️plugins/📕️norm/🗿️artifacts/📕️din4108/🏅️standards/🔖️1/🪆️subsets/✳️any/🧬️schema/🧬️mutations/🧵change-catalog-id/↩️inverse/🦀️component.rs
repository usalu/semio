//! ↩️ `change-catalog-id` — undo restores BASE's `catalog_id`.

use super::mutation::ChangeCatalogId;
use crate::artifacts::din4108::{Din4108Mutation, Din4108Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeCatalogId, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    vec![Din4108Mutation::ChangeCatalogId(ChangeCatalogId { new_catalog_id: base.catalog_id.clone() })]
}
//#endregion 🔖️Inverse
