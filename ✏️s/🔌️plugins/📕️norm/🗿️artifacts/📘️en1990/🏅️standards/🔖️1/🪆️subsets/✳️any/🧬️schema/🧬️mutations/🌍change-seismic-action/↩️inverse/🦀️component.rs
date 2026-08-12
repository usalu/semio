//! ↩️ `change-seismic-action` — undo restores BASE's `seismic_a_ed_kn`; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use super::mutation::ChangeSeismicAction;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSeismicAction, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeSeismicAction(ChangeSeismicAction { new_seismic_a_ed_kn: base.seismic_a_ed_kn })]
}
//#endregion 🔖️Inverse
