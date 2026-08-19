//! ↩️ `change-v-ed-kn` inverse — restores the pre-change `v_ed_kn` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1995::mutations::change_v_ed_kn::mutation::ChangeVEdKn;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeVEdKn, base: &En1995Snapshot) -> Vec<En1995Mutation> {
    vec![En1995Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: base.v_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
