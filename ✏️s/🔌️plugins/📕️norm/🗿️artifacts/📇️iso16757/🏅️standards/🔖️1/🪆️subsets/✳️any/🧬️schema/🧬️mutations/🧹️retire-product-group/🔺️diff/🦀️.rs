//! 🔺️ `retire-product-group` — sparse diff construction.

use super::mutation::RetireProductGroup;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductGroupsRows};

//#region 🔖️Diff

pub fn diff(payload: &RetireProductGroup, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(index) = base.catalogue.product_groups.iter().position(|group| group.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product group \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Iso16757Diff { product_groups: Some(Iso16757ProductGroupsRows::removal(&base.catalogue.product_groups, index)), ..Default::default() })
}
