//! 👁️ 👁️ Generation3d play app commands command — `set-active-utility`.

use crate::artifacts::generation3d::op::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

/// 🧰️ Host-owned active-utility switch — never emits document operations. No longer clears hover
/// itself — the framework owns `graph`'s hover exclusively now (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); a client wanting utility-switch-clears-hover
/// dispatches the injected `interactionHover` verb with empty targets alongside this one.
pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Generation3dConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
