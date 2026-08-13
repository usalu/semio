//! 🕸️ 🕸️ Mathematical play app commands command — `set-directed`.

use crate::apps::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::mathematical::{MathematicalCamera, MathematicalEdge, MathematicalNode, MathematicalSnapshot};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-directed")]
pub struct SetDirected {
    pub directed: bool,
}

pub fn handle(payload: &SetDirected, doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    let mut graph = crate::artifacts::mathematical::mathematical_graph(doc.snapshot);
    graph.directed = payload.directed;
    Ok(Emit::mutations(vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph })]))
}
