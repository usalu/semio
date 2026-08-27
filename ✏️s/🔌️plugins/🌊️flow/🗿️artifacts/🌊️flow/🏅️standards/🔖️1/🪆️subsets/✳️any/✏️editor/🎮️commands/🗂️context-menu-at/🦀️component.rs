//! 🗂️ 🗂️ Flow play app commands command — `context-menu-at`.

use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct ContextMenuAt {
    pub id: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: right-clicking a node used to also
/// select it via `FlowConfigMutation::SetSelection`; selection is framework-owned `InteractionState`
/// now, only ever mutated by the framework's own injected `interactionSelect` handling, never by an app
/// command's `Emit` (mirrors note's `add-block`) — a genuine no-operation, kept only because the shared
/// `NodeGraph` canvas renderer (framework layer, unmigrated this wave) still dispatches it on right-click.
pub fn handle(_payload: &ContextMenuAt, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    Ok(Emit::default())
}
