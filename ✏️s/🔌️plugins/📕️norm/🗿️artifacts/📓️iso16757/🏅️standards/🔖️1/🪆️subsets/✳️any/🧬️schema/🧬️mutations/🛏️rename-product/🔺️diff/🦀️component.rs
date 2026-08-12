//! 🔺️ `rename-product` — sparse diff construction; missing id is a no-op clone.

use super::mutation::RenameProduct;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &RenameProduct, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    if let Some(product) = catalogue.products.iter_mut().find(|product| product.id == payload.id) {
        product.names.preferred.text = payload.new_name.clone();
    }
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
