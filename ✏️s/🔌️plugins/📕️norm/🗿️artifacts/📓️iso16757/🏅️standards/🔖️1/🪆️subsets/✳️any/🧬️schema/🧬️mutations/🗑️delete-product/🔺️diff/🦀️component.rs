//! 🔺️ `delete-product` — sparse diff construction.

use super::mutation::DeleteProduct;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub fn diff(payload: &DeleteProduct, base: &Iso16757Snapshot) -> Iso16757Diff {
    let mut catalogue = base.catalogue.clone();
    catalogue.products.retain(|product| product.id != payload.id);
    Iso16757Diff { catalogue: Some(catalogue), ..Default::default() }
}
//#endregion 🔖️Diff
