//! 🔺️ `introduce-product-group` — sparse diff construction.

use super::mutation::IntroduceProductGroup;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductGroupsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an explicit index past the end is
/// `mutation.target-missing`.

pub fn diff(payload: &IntroduceProductGroup, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_groups.iter().any(|group| group.id == payload.product_group.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product group with id \"{}\" already exists.", payload.product_group.id), [payload.product_group.id.clone()]);
    }
    let len = base.catalogue.product_groups.len();
    if let Some(index) = payload.index.filter(|index| *index > len) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {index} is past the end ({len} rows) for \"{}\".", payload.product_group.id), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(Iso16757Diff { product_groups: Some(Iso16757ProductGroupsRows::insertion(payload.index.unwrap_or(len), payload.product_group.clone())), ..Default::default() })
}
