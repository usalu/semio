//! 🧬️ 🧬️ Procedural2d play app commands command — `enter-generate`.

use crate::apps::procedural2d::config::{Procedural2dConfig, Procedural2dConfigMutation};
use crate::artifacts::procedural2d::op::{generation_mutation_to_procedural2d, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::FlowEvalSession;
use flow::FlowFixture;
use flow::playbook::{apply_generation_mutation, generation_operations, select_generation, GenerationPlayState};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "generate")]
pub struct Generate {}

pub fn handle(_payload: &Generate, _doc: &ArtifactView<'_, Procedural2dSnapshot>, _cfg: &ConfigView<'_, Procedural2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural2dMutation, Procedural2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural2dConfigMutation::SetShowMode { value: "generate".into() }]))
}
