//! 👁️ Writer viewer — the read-only counterpart of the sibling `editor` module for this subset
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `WriterViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<WriterViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an
//! artifact or draft mutation. MUST NOT import anything from the sibling editor module
//! (`policyViewerPurityBreaches`).

use crate::viewer::writer::modes::view;
use crate::viewer::writer::modes::view::windows::main;
use crate::{schema, WriterSnapshot, WRITER_DIALECT, WRITER_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::{ArtifactViewer, ViewEmit, Viewer};
use semio_framework_plugin::{ArtifactView, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — a real per-command payload module the way `editor`'s `🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WriterViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for WriterViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(WriterViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct WriterViewer;

impl ArtifactViewer for WriterViewer {
    type Snapshot = WriterSnapshot;
    type Mutation = crate::op::WriterMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = WriterViewCommand;

    const DIALECT: Dialect = WRITER_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WRITER_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> WriterSnapshot {
        schema::empty_writer_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `WriterViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (e.g. a
    /// read-only outline toggle) is a pure addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            main::WRITER_VIEW_BODY_MAIN => main::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "writer viewer unknown-body text admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_writer_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(WRITER_DIALECT).document(["semio", "writer"]).icon_id("writer").mode_def(view::definition()).default_mode_id(view::WRITER_VIEW_MODE_VIEW).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
