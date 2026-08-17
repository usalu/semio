//! 🔺️ `change-design-situation` sparse diff construction — writes only `En1996Diff.design_situation` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_design_situation::mutation::ChangeDesignSituation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDesignSituation, base: &En1996Snapshot) -> protocol::MutationOutcome<En1996Diff> {
    if base.design_situation == payload.new_design_situation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Design situation already has this value.");
    }
    protocol::MutationOutcome::new(En1996Diff { design_situation: Some(payload.new_design_situation.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
