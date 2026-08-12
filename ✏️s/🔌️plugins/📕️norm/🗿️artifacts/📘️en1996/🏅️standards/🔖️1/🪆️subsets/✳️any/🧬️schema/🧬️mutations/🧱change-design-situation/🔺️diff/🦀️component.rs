//! 🔺️ `change-design-situation` sparse diff construction — writes only `En1996Diff.design_situation` from the payload.

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::change_design_situation::mutation::ChangeDesignSituation;
use crate::artifacts::en1996::En1996Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeDesignSituation, _base: &En1996Snapshot) -> En1996Diff {
    En1996Diff { design_situation: Some(payload.new_design_situation.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
