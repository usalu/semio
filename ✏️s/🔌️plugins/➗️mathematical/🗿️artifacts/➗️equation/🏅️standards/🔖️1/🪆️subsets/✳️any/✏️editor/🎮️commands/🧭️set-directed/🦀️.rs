//! 🕸️ 🕸️ Equation play app commands command — `set-directed`: the ONE intent leaf `change-graph-directed`, never a
//! whole-graph `replace-graph`.

use crate::op::EquationMutation;
use crate::standards::v1::subsets::graph::schema::mutations::change_graph_directed::ChangeGraphDirected;
use crate::{EquationGraph, EquationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-directed")]
pub struct SetDirected {
    pub directed: bool,
}

/// 🧭️ The `change-graph-directed` leaf `payload` means on `graph`, or nothing when the graph already is that way.
pub(crate) fn set_directed_leaves(payload: &SetDirected, graph: &EquationGraph) -> Vec<EquationMutation> {
    if graph.directed == payload.directed {
        return Vec::new();
    }
    vec![EquationMutation::ChangeGraphDirected(ChangeGraphDirected { new_directed: payload.directed })]
}

pub fn handle(payload: &SetDirected, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(set_directed_leaves(payload, &crate::equation_graph(doc.snapshot))))
}
