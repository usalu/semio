//! ↩️ `change-v-weld-ed-kn` inverse — restores the pre-change `v_weld_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1999::mutations::change_v_weld_ed_kn::mutation::ChangeVWeldEdKn;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeVWeldEdKn, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeVWeldEdKn(ChangeVWeldEdKn { new_v_weld_ed_kn: base.v_weld_ed_kn.clone() })]
}
//#endregion 🔖️Inverse
