//! 🧬️ 🧬️ Generation2d play app commands command — `enter-generate`.

use crate::artifacts::generation2d::op::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::config::{Generation2dConfig, Generation2dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "generate")]
pub struct Generate {}

pub fn handle(_payload: &Generate, _doc: &ArtifactView<'_, Generation2dSnapshot>, _cfg: &ConfigView<'_, Generation2dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation2dMutation, Generation2dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation2dConfigMutation::SetShowMode { value: "generate".into() }]))
}
