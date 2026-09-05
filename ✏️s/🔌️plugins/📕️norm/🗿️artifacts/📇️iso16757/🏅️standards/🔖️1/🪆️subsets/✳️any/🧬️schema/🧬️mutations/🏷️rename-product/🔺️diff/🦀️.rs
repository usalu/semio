//! 🔺️ `rename-product` — sparse diff construction; missing id is `mutation.target-missing`.

use super::mutation::RenameProduct;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(product) = base.catalogue.products.iter().find(|product| product.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Product \"{}\" already has that name.", payload.id));
    }
    let mut catalogue = base.catalogue.clone();
    if let Some(product) = catalogue.products.iter_mut().find(|product| product.id == payload.id) {
        product.names.preferred.text = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
//#endregion 🔖️Diff
