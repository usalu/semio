//! 🕸️ 🕸️ Mathematical play app commands command — `set-directed`.

use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::mathematical::MathematicalSnapshot;
use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "set-directed")]
pub struct SetDirected {
    pub directed: bool,
}

pub async fn handle(payload: &SetDirected, doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    let mut graph = crate::artifacts::mathematical::mathematical_graph(doc.snapshot);
    graph.directed = payload.directed;
    Ok(Emit::mutations(vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph })]))
}
