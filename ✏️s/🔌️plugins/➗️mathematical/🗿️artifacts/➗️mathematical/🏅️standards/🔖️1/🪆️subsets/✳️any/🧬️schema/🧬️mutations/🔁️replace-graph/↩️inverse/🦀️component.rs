//! ↩️ `replace-graph` — undo reconstructed from BASE state (the whole prior graph).

use super::mutation::ReplaceGraph;
use crate::artifacts::mathematical::{MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub fn inverse(_payload: &ReplaceGraph, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph: crate::artifacts::mathematical::mathematical_graph(base) })]
}
//#endregion 🔖️Inverse
