//! 👁️ DIN V 18599 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Din18599Viewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Din18599Viewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::din18599::{DIN18599_DIALECT, DIN18599_DOCUMENT_SCHEMA};
use crate::artifacts::din18599::Din18599Snapshot;
use crate::viewer::din18599::modes::view;
use crate::viewer::din18599::modes::view::windows::report;
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
pub enum Din18599ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Din18599ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Din18599ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Din18599Viewer;

impl ArtifactViewer for Din18599Viewer {
    type Snapshot = Din18599Snapshot;
    type Mutation = crate::artifacts::din18599::op::Din18599Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Din18599ViewCommand;

    const DIALECT: Dialect = DIN18599_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DIN18599_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Din18599Snapshot {
        Din18599Snapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Din18599ViewCommand::Noop` variant never carries a config
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
pub fn create_din18599_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DIN18599_DIALECT)
        .document(["semio", "norm", "din18599"])
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
    fn create_din18599_viewer_builds_a_definition_for_this_dialect() {
        let def = create_din18599_viewer();
        assert_eq!(def.dialect, DIN18599_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Din18599Viewer as ArtifactViewer>::DIALECT, DIN18599_DIALECT);
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let snapshot = Din18599Snapshot::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let json = serde_json::to_string(&<Din18599Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default() })).expect("json");
        assert!(json.contains("Unknown body"));
    }
}
//#endregion 🧪️Tests
