//! 🔺️ `create-product-group` — sparse diff construction.

use super::mutation::CreateProductGroup;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
/// 🔺️ A duplicate `id` is `mutation.duplicate-id`; an out-of-range explicit index clamps to the
/// end with `mutation.clamped`.
pub async fn diff(payload: &CreateProductGroup, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.product_groups.iter().any(|group| group.id == payload.product_group.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A product group with id \"{}\" already exists.", payload.product_group.id), [payload.product_group.id.clone()]);
    }
    let mut catalogue = base.catalogue.clone();
    let clamped = matches!(payload.index, Some(index) if index > catalogue.product_groups.len());
    match payload.index {
        Some(index) if index <= catalogue.product_groups.len() => catalogue.product_groups.insert(index, payload.product_group.clone()),
        _ => catalogue.product_groups.push(payload.product_group.clone()),
    }
    let outcome = protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() });
    if clamped {
        outcome.warn("mutation.clamped", format!("Insert index was out of range; appended product group \"{}\" at the end instead.", payload.product_group.id))
    } else {
        outcome
    }
}
//#endregion 🔖️Diff
