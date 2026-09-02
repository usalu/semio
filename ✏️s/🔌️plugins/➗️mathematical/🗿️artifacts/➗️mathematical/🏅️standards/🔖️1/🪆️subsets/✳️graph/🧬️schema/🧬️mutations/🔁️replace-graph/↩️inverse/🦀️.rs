//! ↩️ `replace-graph` — undo reconstructed from BASE state (the whole prior graph).

use crate::artifacts::mathematical::{mathematical_graph, MathematicalMutation, MathematicalSnapshot};

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ReplaceGraph, base: &MathematicalSnapshot) -> Vec<MathematicalMutation> {
    vec![MathematicalMutation::ReplaceGraph(super::ReplaceGraph { graph: crate::artifacts::mathematical::mathematical_graph(base) })]
}
//#endregion 🔖️Inverse
