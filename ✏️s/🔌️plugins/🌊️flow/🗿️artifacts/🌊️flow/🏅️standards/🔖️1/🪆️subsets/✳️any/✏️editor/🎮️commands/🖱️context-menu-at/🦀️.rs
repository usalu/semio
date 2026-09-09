//! 🗂️ 🗂️ Flow play app commands command — `context-menu-at`.

use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::{op::FlowMutation, FlowSnapshot};
use flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct ContextMenuAt {
    pub id: String,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: right-clicking a node used to also
/// select it via `NoConfigMutation::SetSelection`; selection is framework-owned `InteractionState`
/// now, only ever mutated by the framework's own injected `interactionSelect` handling, never by an app
/// command's `Emit` (mirrors note's `add-block`) — a genuine no-operation, kept only because the shared
/// `NodeGraph` canvas renderer (framework layer, unmigrated this wave) still dispatches it on right-click.
pub fn handle(_payload: &ContextMenuAt, _doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}
