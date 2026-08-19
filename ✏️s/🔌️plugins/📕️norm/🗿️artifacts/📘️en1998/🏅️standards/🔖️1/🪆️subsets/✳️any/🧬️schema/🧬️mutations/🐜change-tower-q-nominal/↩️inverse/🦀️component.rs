//! ↩️ `change-tower-q-nominal` inverse — restores the pre-change `tower_q_nominal` from BASE state; `change` is its own
//! inverse partner (per `📓️taxonomy.md`).

use crate::artifacts::en1998::mutations::change_tower_q_nominal::mutation::ChangeTowerQNominal;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeTowerQNominal, base: &En1998Snapshot) -> Vec<En1998Mutation> {
    vec![En1998Mutation::ChangeTowerQNominal(ChangeTowerQNominal { new_tower_q_nominal: base.tower_q_nominal.clone() })]
}
//#endregion 🔖️Inverse
