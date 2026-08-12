//! ↩️ `change-graph-directed` — undo reconstructed from BASE state.

use super::mutation::ChangeGraphDirected;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeGraphDirected, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: base.graph.directed })]
}
//#endregion 🔖️Inverse
