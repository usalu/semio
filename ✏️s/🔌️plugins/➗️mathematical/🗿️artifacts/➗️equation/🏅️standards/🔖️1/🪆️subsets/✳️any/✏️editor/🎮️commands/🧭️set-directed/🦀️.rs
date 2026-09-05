//! 🕸️ 🕸️ Equation play app commands command — `set-directed`.

use crate::artifacts::equation::op::EquationMutation;
use crate::artifacts::equation::standards::v1::subsets::graph::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::equation::EquationSnapshot;
use crate::editor::equation::config::{EquationConfig, EquationConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-directed")]
pub struct SetDirected {
    pub directed: bool,
}

pub async fn handle(payload: &SetDirected, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, EquationConfig>) -> Result<Emit<EquationMutation, EquationConfigMutation>, Fault> {
    let mut graph = crate::artifacts::equation::equation_graph(doc.snapshot);
    graph.directed = payload.directed;
    Ok(Emit::mutations(vec![EquationMutation::ReplaceGraph(ReplaceGraph { graph })]))
}
