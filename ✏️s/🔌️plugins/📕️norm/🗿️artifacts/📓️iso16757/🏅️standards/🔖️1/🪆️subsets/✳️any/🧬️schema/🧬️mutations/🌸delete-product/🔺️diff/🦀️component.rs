//! 🔺️ `delete-product` — sparse diff construction.

use super::mutation::DeleteProduct;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.catalogue.products.iter().any(|product| product.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let mut catalogue = base.catalogue.clone();
    catalogue.products.retain(|product| product.id != payload.id);
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
//#endregion 🔖️Diff
