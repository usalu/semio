//! 👁️ ISO 16757 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Iso16757Viewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Iso16757Viewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::iso16757::{ISO16757_DIALECT, ISO16757_DOCUMENT_SCHEMA};
use crate::artifacts::iso16757::Iso16757Snapshot;
use crate::viewer::iso16757::modes::view;
use crate::viewer::iso16757::modes::view::windows::report;
use semio_framework_plugin::{
    ArtifactView, ArtifactViewer, ConfigView, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer,
};
// 🚧️ SDK GAP: see the identical note in `✏️editor/🦀️component.rs` — `Dialect` is only reachable
// through `app`, not yet in the crate-root re-export list.
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::ui_text;
use semio_framework_plugin::Label;
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — a real per-command payload module the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Iso16757ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Iso16757ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Iso16757ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Iso16757Viewer;

impl ArtifactViewer for Iso16757Viewer {
    type Snapshot = Iso16757Snapshot;
    type Mutation = crate::artifacts::iso16757::op::Iso16757Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Iso16757ViewCommand;

    const DIALECT: Dialect = ISO16757_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ISO16757_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Iso16757Snapshot {
        Iso16757Snapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Iso16757ViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`. Kept as a real dispatch (not
    /// `unreachable!()`) so a future view-only action is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            report::BODY_KEY => report::render(doc.snapshot),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_iso16757_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(ISO16757_DIALECT)
        .document(["semio", "norm", "iso16757"])
        .icon_id("check-circle")
        .mode_def(view::definition())
        .default_mode_id(crate::app_surface::MODE_VIEW)
        .window_kind_def(report::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_iso16757_viewer_builds_a_definition_for_this_dialect() {
        let def = create_iso16757_viewer();
        assert_eq!(def.dialect, ISO16757_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Iso16757Viewer as ArtifactViewer>::DIALECT, ISO16757_DIALECT);
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let snapshot = Iso16757Snapshot::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let json = serde_json::to_string(&<Iso16757Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default() })).expect("json");
        assert!(json.contains("Unknown body"));
    }
}
//#endregion 🧪️Tests
