//! 🔺️ `introduce-product-group` — sparse diff construction.

use super::mutation::IntroduceProductGroup;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductGroupsRows};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.

pub fn diff(payload: &IntroduceProductGroup, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_groups.iter().any(|group| group.id == payload.product_group.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product group with id \"{}\" already exists.", payload.product_group.id), [payload.product_group.id.clone()]);
    }
    let ids: Vec<String> = base.catalogue.product_groups.iter().map(|item| item.id.clone()).collect();
    let clamped = matches!(payload.index, Some(index) if index > ids.len());
    let at = payload.index.filter(|index| *index <= ids.len()).unwrap_or(ids.len());
    let order = (at < ids.len()).then(|| {
        let mut order = ids.clone();
        order.insert(at, payload.product_group.id.clone());
        order
    });
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { product_groups: Some(Iso16757ProductGroupsRows { added: vec![payload.product_group.clone()], order, ..Default::default() }), ..Default::default() });
    if clamped {
        outcome.warning("mutation.clamped", format!("Insert index was out of range; appended product group \"{}\" at the end instead.", payload.product_group.id))
    } else {
        outcome
    }
}
