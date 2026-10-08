//! Diff for `introduce-product-class`.

use super::mutation::IntroduceProductClass;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductClassesRows};

pub fn diff(payload: &IntroduceProductClass, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_classes.iter().any(|item| item.id == payload.product_class.id) {
        return protocol::MutationOutcome::fatal(
            "mutation.duplicate-id",
            format!("An entity with id \"{}\" already exists.", payload.product_class.id),
            [payload.product_class.id.clone()],
        );
    }
    let len = base.catalogue.product_classes.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({len} rows) for \"{}\".", payload.product_class.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Iso16757Diff { product_classes: Some(Iso16757ProductClassesRows::insertion(payload.index.unwrap_or(len), payload.product_class.clone())), ..Default::default() })
}
