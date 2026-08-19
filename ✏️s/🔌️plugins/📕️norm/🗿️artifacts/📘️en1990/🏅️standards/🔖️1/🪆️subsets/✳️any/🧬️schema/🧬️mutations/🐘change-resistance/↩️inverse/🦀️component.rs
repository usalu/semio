//! ↩️ `change-resistance` — undo restores BASE's `resistance_kn`; `change` is its own inverse
//! partner (per `📓️taxonomy.md`).

use super::mutation::ChangeResistance;
use crate::artifacts::en1990::{En1990Mutation, En1990Snapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeResistance, base: &En1990Snapshot) -> Vec<En1990Mutation> {
    vec![En1990Mutation::ChangeResistance(ChangeResistance { new_resistance_kn: base.resistance_kn })]
}
//#endregion 🔖️Inverse
