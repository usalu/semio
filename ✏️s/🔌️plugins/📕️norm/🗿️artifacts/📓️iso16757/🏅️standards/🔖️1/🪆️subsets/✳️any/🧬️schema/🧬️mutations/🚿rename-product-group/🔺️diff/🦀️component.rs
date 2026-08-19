//! 🔺️ `rename-product-group` — sparse diff construction; missing id is `mutation.target-missing`.

use super::mutation::RenameProductGroup;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &RenameProductGroup, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let Some(group) = base.catalogue.product_groups.iter().find(|group| group.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Product group \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if group.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Product group \"{}\" already has that name.", payload.id));
    }
    let mut catalogue = base.catalogue.clone();
    if let Some(group) = catalogue.product_groups.iter_mut().find(|group| group.id == payload.id) {
        group.names.preferred.text = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(Iso16757Diff { catalogue: Some(catalogue), ..Default::default() })
}
//#endregion 🔖️Diff
