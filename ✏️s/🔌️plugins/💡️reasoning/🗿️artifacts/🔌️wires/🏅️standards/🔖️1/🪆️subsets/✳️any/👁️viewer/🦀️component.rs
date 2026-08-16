//! 👁️ Wires viewer — the read-only counterpart of the sibling editor module for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `WiresViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<WiresViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the editor module (`policyViewerPurityBreaches`).

use crate::artifacts::wires::{WiresSnapshot, MINDMAP_WIRES_SCHEMA, WIRES_DIALECT};
use crate::viewer::wires::modes::view;
use crate::viewer::wires::modes::view::windows::canvas;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way the sibling editor module's
/// `🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches anything
/// through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WiresViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for WiresViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(WiresViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct WiresViewer;

impl ArtifactViewer for WiresViewer {
    type Snapshot = WiresSnapshot;
    type Mutation = crate::artifacts::wires::WiresMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = WiresViewCommand;

    const DIALECT: Dialect = WIRES_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = MINDMAP_WIRES_SCHEMA;

    fn initial_snapshot() -> WiresSnapshot {
        crate::artifacts::wires::empty_wires_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `WiresViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (pan/zoom
    /// persisted per-viewer) is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            canvas::WIRES_VIEW_BODY_CANVAS => canvas::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_wires_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(WIRES_DIALECT)
        .document(["semio", "reasoning", "mindmap", "wires"])
        .icon_id("reasoning-wires")
        .mode_def(view::definition())
        .default_mode_id(view::WIRES_VIEW_MODE_VIEW)
        .window_kind_def(canvas::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_wires_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_wires_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, WIRES_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<WiresViewer as ArtifactViewer>::DIALECT, WIRES_DIALECT);
    }

    #[test]
    fn viewer_command_default_is_noop() {
        assert_eq!(WiresViewCommand::default(), WiresViewCommand::Noop);
    }
}
//#endregion 🧪️Tests
