//! 🕸️ 🕸️ Equation play app commands command — `set-directed`.

use crate::op::EquationMutation;
use crate::standards::v1::subsets::graph::schema::mutations::replace_graph::ReplaceGraph;
use crate::EquationSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-directed")]
pub struct SetDirected {
    pub directed: bool,
}

pub fn handle(payload: &SetDirected, doc: &ArtifactView<'_, EquationSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<EquationMutation, NoConfigMutation>, Fault> {
    let mut graph = crate::equation_graph(doc.snapshot);
    graph.directed = payload.directed;
    Ok(Emit::mutations(vec![EquationMutation::ReplaceGraph(ReplaceGraph { graph })]))
}
