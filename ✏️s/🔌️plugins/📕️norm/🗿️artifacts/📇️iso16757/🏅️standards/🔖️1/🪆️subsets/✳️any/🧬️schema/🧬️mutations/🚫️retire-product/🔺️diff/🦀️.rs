//! 🔺️ `retire-product` — sparse diff construction.

use super::mutation::RetireProduct;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductsRows};

//#region 🔖️Diff

pub fn diff(payload: &RetireProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if !base.catalogue.products.iter().any(|product| product.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Iso16757Diff { products: Some(Iso16757ProductsRows { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
