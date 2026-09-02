//! ↩️ `change-graph-directed` — undo reconstructed from BASE state.

use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangeGraphDirected, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::ChangeGraphDirected(super::ChangeGraphDirected { new_directed: crate::artifacts::mathematical::mathematical_graph(base).directed })]
}
//#endregion 🔖️Inverse
