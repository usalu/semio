//! 👁️ Process 3D viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Process3dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Process3dViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::process3d::{Process3dMutation, Process3dSnapshot, PROCESS3D_DIALECT, PROCESS_3D_SCHEMA};
use crate::viewer::process3d::modes::view;
use crate::viewer::process3d::modes::view::windows::workpiece;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Process3dViewCommand {
    Noop,
}

impl protocol::OpBinary for Process3dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Process3dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Process3dViewer;

impl ArtifactViewer for Process3dViewer {
    type Snapshot = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Process3dViewCommand;

    const DIALECT: Dialect = PROCESS3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PROCESS_3D_SCHEMA;

    fn initial_snapshot() -> Process3dSnapshot {
        crate::artifacts::process3d::schema::default_document()
    }

    /// 👁️ Structurally read-only: the sole `Process3dViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect,
    /// no dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only
    /// action (camera orbit, "jump to step") is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(
        body_key: &str,
        doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            workpiece::PROCESS3D_VIEW_BODY_MAIN => workpiece::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_process3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PROCESS3D_DIALECT)
        .document(["semio", "process", "3d"])
        .icon_id("hammer")
        .mode_def(view::definition())
        .default_mode_id(view::PROCESS3D_VIEW_MODE_VIEW)
        .window_kind_def(workpiece::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_process3d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_process3d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, PROCESS3D_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Process3dViewer as ArtifactViewer>::DIALECT, PROCESS3D_DIALECT);
    }
}
//#endregion 🧪️Tests
