//! ↩️ `change-silo-v-ed-kn` inverse — restores the pre-change `silo_v_ed_kn` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::mutations::change_silo_v_ed_kn::ChangeSiloVEdKn;
use crate::mutations::En1998Mutation;
use crate::En1998Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeSiloVEdKn, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeSiloVEdKn(ChangeSiloVEdKn { new_silo_v_ed_kn: base.silo_v_ed_kn })]
}
//#endregion 🔖️Inverse
