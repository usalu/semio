//! 🕸️ 🕸️ Equation play app commands command — `set-algorithm`.

use crate::op::EquationMutation;
use crate::standards::v1::subsets::graph::schema::mutations::replace_graph::ReplaceGraph;
use crate::EquationSnapshot;
use crate::editor::equation::config::{EquationConfig, EquationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
pub struct SetAlgorithm {
    pub algorithm: String,
    pub seed: Option<String>,
}

pub fn handle(payload: &SetAlgorithm, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    let mut graph = crate::equation_graph(doc.snapshot);
    graph.algorithm = payload.algorithm.clone();
    graph.algorithm_seed = payload.seed.clone();
    Ok(Emit::commit(vec![EquationMutation::ReplaceGraph(ReplaceGraph { graph })], "setAlgorithm"))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
