//! 🔺️ `retire-product` — sparse diff construction.

use super::mutation::RetireProduct;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductsRows};

//#region 🔖️Diff

pub fn diff(payload: &RetireProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(index) = base.catalogue.products.iter().position(|product| product.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Iso16757Diff { products: Some(Iso16757ProductsRows::removal(&base.catalogue.products, index)), ..Default::default() })
}
