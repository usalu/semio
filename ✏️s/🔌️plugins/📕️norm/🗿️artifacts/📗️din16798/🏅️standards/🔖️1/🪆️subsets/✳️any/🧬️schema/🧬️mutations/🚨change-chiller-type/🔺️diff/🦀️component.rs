//! 🔺️ `change-chiller-type` sparse diff construction — writes only `Din16798Diff.chiller_type` from the payload.

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::change_chiller_type::mutation::ChangeChillerType;
use crate::artifacts::din16798::Din16798Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeChillerType, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
    if base.chiller_type == payload.new_chiller_type {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Chiller type is already \"{}\".", payload.new_chiller_type));
    }
    protocol::MutationOutcome::new(Din16798Diff { chiller_type: Some(payload.new_chiller_type.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
