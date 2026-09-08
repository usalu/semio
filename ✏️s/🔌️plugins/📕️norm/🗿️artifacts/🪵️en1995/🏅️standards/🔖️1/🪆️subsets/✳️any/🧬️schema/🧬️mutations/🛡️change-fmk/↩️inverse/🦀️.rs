//! ↩️ `change-f-m-k` inverse — restores the pre-change `f_m_k` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_f_m_k::ChangeFMK;
use crate::mutations::En1995Mutation;
use crate::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeFMK, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeFMK(ChangeFMK { new_f_m_k: base.f_m_k })]
}
//#endregion 🔖️Inverse
