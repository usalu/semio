//! ↩️ `change-v-ed-kn` inverse — restores the pre-change `v_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_v_ed_kn::ChangeVEdKn;
use crate::mutations::En1997Mutation;
use crate::En1997Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeVEdKn, base: &En1997Snapshot) -> Vec<En1997Mutation> {
    vec![En1997Mutation::ChangeVEdKn(ChangeVEdKn { new_v_ed_kn: base.v_ed_kn })]
}
//#endregion 🔖️Inverse
