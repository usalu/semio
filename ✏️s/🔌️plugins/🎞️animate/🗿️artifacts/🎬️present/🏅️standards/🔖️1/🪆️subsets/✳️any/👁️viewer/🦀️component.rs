//! 👁️ Animate viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `AnimatePresentViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<AnimatePresentViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::present::{default_present_snapshot, PresentSnapshot, ANIMATE_DIALECT, PRESENT_DOCUMENT_SCHEMA};
use crate::viewer::animate::modes::view;
use crate::viewer::animate::modes::view::windows::tile_editor;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use semio_framework_plugin::ArtifactViewer;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*` carries
/// them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AnimateViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for AnimateViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(AnimateViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct AnimatePresentViewer;

impl ArtifactViewer for AnimatePresentViewer {
    type Snapshot = PresentSnapshot;
    type Mutation = crate::artifacts::present::PresentMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = AnimateViewCommand;

    const DIALECT: Dialect = ANIMATE_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PRESENT_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> PresentSnapshot {
        default_present_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `AnimateViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (e.g.
    /// scrub the deck's active example) is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            tile_editor::BODY_KEY => tile_editor::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_animate_present_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(ANIMATE_DIALECT)
        .document(["semio", "animate"])
        .icon_id("animate")
        .mode_def(view::definition())
        .default_mode_id(view::ANIMATE_VIEW_MODE_VIEW)
        .window_kind_def(tile_editor::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_animate_present_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_animate_present_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, ANIMATE_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<AnimatePresentViewer as ArtifactViewer>::DIALECT, ANIMATE_DIALECT);
    }
}
//#endregion 🧪️Tests
