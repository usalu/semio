//! 🕸️ 🕸️ Equation play app commands command — `set-algorithm`: the ONE intent leaf `update-graph-algorithm`, never a
//! whole-graph `replace-graph`.

use crate::op::EquationMutation;
use crate::standards::v1::subsets::graph::schema::mutations::update_graph_algorithm::UpdateGraphAlgorithm;
use crate::{EquationGraph, EquationSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord)]
pub struct SetAlgorithm {
    pub algorithm: String,
    pub seed: Option<String>,
}

/// 🧮️ The `update-graph-algorithm` leaf `payload` means on `graph`, or nothing when the graph already runs it.
pub(crate) fn set_algorithm_leaves(payload: &SetAlgorithm, graph: &EquationGraph) -> Vec<EquationMutation> {
    if graph.algorithm == payload.algorithm && graph.algorithm_seed == payload.seed {
        return Vec::new();
    }
    vec![EquationMutation::UpdateGraphAlgorithm(UpdateGraphAlgorithm { new_algorithm: payload.algorithm.clone(), new_algorithm_seed: payload.seed.clone() })]
}

pub fn handle(payload: &SetAlgorithm, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(set_algorithm_leaves(payload, &doc.snapshot.graph.clone())))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
