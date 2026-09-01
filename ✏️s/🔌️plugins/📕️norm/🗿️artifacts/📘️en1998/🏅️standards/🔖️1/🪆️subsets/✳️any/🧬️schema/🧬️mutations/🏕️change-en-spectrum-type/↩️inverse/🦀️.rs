//! ↩️ `change-en-spectrum-type` inverse — restores the pre-change `en_spectrum_type` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_en_spectrum_type::ChangeEnSpectrumType;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeEnSpectrumType, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeEnSpectrumType(ChangeEnSpectrumType { new_en_spectrum_type: base.en_spectrum_type.clone() })]
}
//#endregion 🔖️Inverse
