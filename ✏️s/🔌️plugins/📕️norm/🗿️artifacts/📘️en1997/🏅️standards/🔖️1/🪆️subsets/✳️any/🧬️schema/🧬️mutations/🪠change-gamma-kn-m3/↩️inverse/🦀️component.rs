//! ↩️ `change-gamma-kn-m3` inverse — restores the pre-change `gamma_kn_m3` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1997::mutations::change_gamma_kn_m3::mutation::ChangeGammaKnM3;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeGammaKnM3, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeGammaKnM3(ChangeGammaKnM3 { new_gamma_kn_m3: base.gamma_kn_m3.clone() })]
}
//#endregion 🔖️Inverse
