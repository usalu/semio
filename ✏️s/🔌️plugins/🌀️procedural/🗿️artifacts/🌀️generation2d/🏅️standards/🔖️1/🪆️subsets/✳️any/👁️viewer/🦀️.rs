//! 👁️ Generation2d viewer — the read-only counterpart of the editor surface for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Generation2dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Generation2dViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::generation2d::{Generation2dSnapshot, GENERATION2D_DIALECT, GENERATION_2D_SCHEMA};
use crate::viewer::generation2d::modes::view;
use crate::viewer::generation2d::modes::view::windows::preview;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way the editor's `🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Generation2dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Generation2dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Generation2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Generation2dViewer;

impl ArtifactViewer for Generation2dViewer {
    type Snapshot = Generation2dSnapshot;
    type Mutation = crate::artifacts::generation2d::op::Generation2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Generation2dViewCommand;

    const DIALECT: Dialect = GENERATION2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;

    fn initial_snapshot() -> Generation2dSnapshot {
        crate::artifacts::generation2d::schema::default_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `Generation2dViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no
    /// dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action
    /// (pan/zoom, "jump to widget") is a pure addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let node = match body_key {
            preview::BODY_KEY => preview::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_generation2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(GENERATION2D_DIALECT)
        .document(["semio", "procedural", "2d"])
        .icon_id("generation2d")
        .mode_def(view::definition())
        .default_mode_id(view::GENERATION2D_VIEW_MODE_VIEW)
        .window_kind_def(preview::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_generation2d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_generation2d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, GENERATION2D_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Generation2dViewer as ArtifactViewer>::DIALECT, GENERATION2D_DIALECT);
    }
}
//#endregion 🧪️Tests
