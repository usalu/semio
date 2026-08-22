//! 👁️ 👁️ Procedural3d play app commands command — `set-active-utility`.

use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-utility")]
pub struct SetActiveUtility {
    pub utility_id: String,
}

/// 🧰️ Host-owned active-utility switch — never emits document operations. No longer clears hover
/// itself — the framework owns `graph`'s hover exclusively now (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); a client wanting utility-switch-clears-hover
/// dispatches the injected `interactionHover` verb with empty targets alongside this one.
pub fn handle(payload: &SetActiveUtility, _doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    Ok(Emit::config(vec![Procedural3dConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }]))
}
