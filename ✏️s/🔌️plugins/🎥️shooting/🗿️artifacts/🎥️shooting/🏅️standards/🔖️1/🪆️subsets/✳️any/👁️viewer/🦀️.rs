//! 👁️ Shooting viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `ShootingViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<ShootingViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::shooting::{ShootingSnapshot, SHOOTING_DIALECT, SHOOTING_DOCUMENT_SCHEMA};
use crate::viewer::shooting::modes::view;
use crate::viewer::shooting::modes::view::windows::scene;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert, `Default`-derived variant — real per-command payload modules the way
/// `✏️editor/🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches
/// anything through `handle`. `Default` is required by `testkit::assert_viewer_never_mutates`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ShootingViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for ShootingViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(ShootingViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct ShootingViewer;

impl ArtifactViewer for ShootingViewer {
    type Snapshot = ShootingSnapshot;
    type Mutation = crate::artifacts::shooting::op::ShootingMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = ShootingViewCommand;

    const DIALECT: Dialect = SHOOTING_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = SHOOTING_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> ShootingSnapshot {
        crate::artifacts::shooting::schema::default_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `ShootingViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "jump to shot") is a pure addition here, never a signature change.
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
            scene::BODY_KEY => scene::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_shooting_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(SHOOTING_DIALECT).document(["semio", "shooting"]).icon_id("camera").mode_def(view::definition()).default_mode_id(view::SHOOTING_VIEW_MODE_VIEW).window_kind_def(scene::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_shooting_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_shooting_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, SHOOTING_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<ShootingViewer as ArtifactViewer>::DIALECT, SHOOTING_DIALECT);
    }
}
//#endregion 🧪️Tests
