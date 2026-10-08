//! 🔺️ `rename-product` — sparse diff construction; missing id is `mutation.target-missing`.

use super::mutation::RenameProduct;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductsRows, Iso16757ProductsPatch};

//#region 🔖️Diff

pub fn diff(payload: &RenameProduct, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(product) = base.catalogue.products.iter().find(|product| product.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if product.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Product \"{}\" already has that name.", payload.id));
    }
    protocol::MutationOutcome::new(Iso16757Diff {
        products: Some(Iso16757ProductsRows { modified: vec![Iso16757ProductsPatch { id: payload.id.clone(), name: Some(payload.new_name.clone()) }], ..Default::default() }),
        ..Default::default()
    })
}
