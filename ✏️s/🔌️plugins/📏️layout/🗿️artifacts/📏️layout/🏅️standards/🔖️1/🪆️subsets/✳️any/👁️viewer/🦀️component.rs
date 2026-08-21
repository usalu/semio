//! 👁️ Layout viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `LayoutViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<LayoutViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{LayoutSnapshot, LAYOUT_DIALECT, LAYOUT_DOCUMENT_SCHEMA};
use crate::viewer::layout::modes::view;
use crate::viewer::layout::modes::view::windows::preview;
use semio_framework::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayoutViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for LayoutViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(LayoutViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct LayoutViewer;

impl ArtifactViewer for LayoutViewer {
    type Snapshot = LayoutSnapshot;
    type Mutation = LayoutMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = LayoutViewCommand;

    const DIALECT: Dialect = LAYOUT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LAYOUT_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> LayoutSnapshot {
        crate::artifacts::layout::schema::default_document()
    }

    /// 👁️ Structurally read-only: the sole `LayoutViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (e.g.
    /// "jump to page") is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            preview::BODY_KEY => preview::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_layout_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(LAYOUT_DIALECT).document(["semio", "layout"]).icon_id("layout").mode_def(view::definition()).default_mode_id(view::LAYOUT_VIEW_MODE_VIEW).window_kind_def(preview::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_layout_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_layout_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, LAYOUT_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<LayoutViewer as ArtifactViewer>::DIALECT, LAYOUT_DIALECT);
    }
}
//#endregion 🧪️Tests
