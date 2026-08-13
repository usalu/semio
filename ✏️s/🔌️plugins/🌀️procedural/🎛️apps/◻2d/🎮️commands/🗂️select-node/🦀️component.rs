//! 🗂️ 🗂️ Procedural2d play app commands command — `select-node`.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::op::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "select-node")]
pub struct SelectNode {
    pub ids: Vec<String>}

pub fn handle(payload: &SelectNode, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural2dConfigMutation::SetSelection { ids: payload.ids.clone() }]))
}
