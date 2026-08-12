//! 🔺️ `change-en-spectrum-type` sparse diff construction — writes only `En1998Diff.en_spectrum_type` from the payload.

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::change_en_spectrum_type::mutation::ChangeEnSpectrumType;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeEnSpectrumType, _base: &En1998Snapshot) -> En1998Diff {
    En1998Diff { en_spectrum_type: Some(payload.new_en_spectrum_type.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
