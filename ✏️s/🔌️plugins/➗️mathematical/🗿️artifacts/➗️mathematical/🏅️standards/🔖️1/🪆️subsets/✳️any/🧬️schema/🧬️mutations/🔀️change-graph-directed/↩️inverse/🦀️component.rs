//! ↩️ `change-graph-directed` — undo reconstructed from BASE state.

use super::mutation::ChangeGraphDirected;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &ChangeGraphDirected, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: crate::artifacts::mathematical::mathematical_graph(base).directed })]
}
//#endregion 🔖️Inverse
