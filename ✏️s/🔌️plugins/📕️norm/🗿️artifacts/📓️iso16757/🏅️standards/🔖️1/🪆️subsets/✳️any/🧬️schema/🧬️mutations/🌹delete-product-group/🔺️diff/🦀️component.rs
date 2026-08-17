//! 🔺️ `delete-product-group` — sparse diff construction.

use super::mutation::DeleteProductGroup;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteProductGroup, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.catalogue.product_groups.iter().any(|group| group.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product group \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.product_groups.retain(|group| group.id != payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
//#endregion 🔖️Diff
