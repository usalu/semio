//! 🔺️ `change-sfp-required-class` sparse diff construction — writes only `Din16798Diff.sfp_required_class` from the payload.

use crate::diff::Din16798Diff;
use crate::mutations::change_sfp_required_class::ChangeSfpRequiredClass;
use crate::Din16798Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeSfpRequiredClass, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.sfp_required_class == payload.new_sfp_required_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Required SFP class is already {}.", payload.new_sfp_required_class));
    }
    protocol::MutationOutcome::new(Din16798Diff { sfp_required_class: Some(payload.new_sfp_required_class), ..Default::default() })
}
//#endregion 🔖️Diff
