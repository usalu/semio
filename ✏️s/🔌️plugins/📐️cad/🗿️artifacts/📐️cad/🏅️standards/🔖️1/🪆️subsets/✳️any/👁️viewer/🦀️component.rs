//! 👁️ CAD viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `CadViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<CadViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling `✏️editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::forest_play_scene;
use crate::artifacts::cad::{CadSnapshot, CAD_DIALECT, CAD_DOCUMENT_SCHEMA};
use crate::viewer::cad::modes::view;
use crate::viewer::cad::modes::view::windows::shape;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
// 🚧️ SDK GAP: see the identical note in `✏️editor/🦀️component.rs` — `ArtifactViewer`/`Viewer`/
// `ViewEmit`/`Dialect` are only reachable through `app`, not yet in the crate-root re-export list.
use semio_framework_plugin::app::{ArtifactViewer, Dialect, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CadViewCommand {
    Noop,
}

impl protocol::OpBinary for CadViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(CadViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct CadViewer;

impl ArtifactViewer for CadViewer {
    type Snapshot = CadSnapshot;
    type Mutation = crate::artifacts::cad::op::CadMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = CadViewCommand;

    const DIALECT: Dialect = CAD_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = CAD_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> CadSnapshot {
        forest_play_scene()
    }

    /// 👁️ Structurally read-only: the sole `CadViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "jump to pane") is a pure addition here, never a signature change.
    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            shape::BODY_KEY => shape::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_cad_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(CAD_DIALECT).document(["semio", "cad"]).icon_id("box").mode_def(view::definition()).default_mode_id(view::CAD_VIEW_MODE_VIEW).window_kind_def(shape::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_cad_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_cad_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, CAD_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<CadViewer as ArtifactViewer>::DIALECT, CAD_DIALECT);
    }
}
//#endregion 🧪️Tests
