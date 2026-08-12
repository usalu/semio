//! 🔺️ `change-design-approach` sparse diff construction — writes only `En1997Diff.design_approach` from the payload.

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::change_design_approach::mutation::ChangeDesignApproach;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDesignApproach, _base: &En1997Snapshot) -> En1997Diff {
    En1997Diff { design_approach: Some(payload.new_design_approach.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
