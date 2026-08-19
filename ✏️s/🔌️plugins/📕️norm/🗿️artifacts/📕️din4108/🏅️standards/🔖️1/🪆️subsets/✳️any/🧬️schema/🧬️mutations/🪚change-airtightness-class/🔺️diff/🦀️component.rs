//! 🔺️ `change-airtightness-class` — sparse diff construction.

use super::mutation::ChangeAirtightnessClass;
use crate::artifacts::din4108::{Din4108Diff, Din4108Snapshot};

//#region 🔖️Diff
pub async fn diff(payload: &ChangeAirtightnessClass, base: &Din4108Snapshot) -> protocol::MutationOutcome<Din4108Diff> {
    if base.airtightness_class == payload.new_airtightness_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Airtightness class already has this value.");
    }
    protocol::MutationOutcome::new(Din4108Diff { airtightness_class: Some(payload.new_airtightness_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
