//! 🔺️ `change-sfp-required-class` sparse diff construction — writes only `Din16798Diff.sfp_required_class` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_sfp_required_class::mutation::ChangeSfpRequiredClass;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeSfpRequiredClass, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.sfp_required_class == payload.new_sfp_required_class {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Required SFP class is already {}.", payload.new_sfp_required_class));
    }
    protocol::MutationOutcome::new(Din16798Diff { sfp_required_class: Some(payload.new_sfp_required_class.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
