//! 🔺️ `change-design-approach` sparse diff construction — writes only `En1997Diff.design_approach` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_design_approach::mutation::ChangeDesignApproach;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ChangeDesignApproach, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if base.design_approach == payload.new_design_approach {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Design approach is already \"{}\".", payload.new_design_approach));
    }
    protocol::MutationOutcome::new(En1997Diff { design_approach: Some(payload.new_design_approach.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
