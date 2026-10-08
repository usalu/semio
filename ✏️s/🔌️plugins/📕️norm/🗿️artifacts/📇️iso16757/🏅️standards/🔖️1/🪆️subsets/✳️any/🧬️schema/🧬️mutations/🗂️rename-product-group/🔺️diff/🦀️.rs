//! 🔺️ `rename-product-group` — sparse diff construction; missing id is `mutation.target-missing`.

use super::mutation::RenameProductGroup;
use crate::{Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ProductGroupsRows, Iso16757ProductGroupsPatch};

//#region 🔖️Diff

pub fn diff(payload: &RenameProductGroup, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(group) = base.catalogue.product_groups.iter().find(|group| group.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product group \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if group.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Product group \"{}\" already has that name.", payload.id));
    }
    protocol::MutationOutcome::new(Iso16757Diff {
        product_groups: Some(Iso16757ProductGroupsRows { modified: vec![Iso16757ProductGroupsPatch { id: payload.id.clone(), name: Some(payload.new_name.clone()) }], ..Default::default() }),
        ..Default::default()
    })
}
