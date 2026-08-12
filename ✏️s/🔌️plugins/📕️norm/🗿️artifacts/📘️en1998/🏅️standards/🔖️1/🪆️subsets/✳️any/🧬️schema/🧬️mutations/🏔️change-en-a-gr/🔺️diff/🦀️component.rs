//! 🔺️ `change-en-a-gr` sparse diff construction — writes only `En1998Diff.en_a_gr` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_en_a_gr::mutation::ChangeEnAGr;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnAGr, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { en_a_gr: Some(payload.new_en_a_gr.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
