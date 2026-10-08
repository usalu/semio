//! Diff for `introduce-product-index`.

use super::mutation::IntroduceProductIndex;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductIndexesRows};

pub fn diff(payload: &IntroduceProductIndex, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_indexes.iter().any(|item| item.id == payload.product_index.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_index.id),
            [payload.product_index.id.clone()],
        );
    }
    let len = base.catalogue.product_indexes.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({len} rows) for \"{}\".", payload.product_index.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Iso16757Diff { product_indexes: Some(Iso16757ProductIndexesRows::insertion(payload.index.unwrap_or(len), payload.product_index.clone())), ..Default::default() })
}
