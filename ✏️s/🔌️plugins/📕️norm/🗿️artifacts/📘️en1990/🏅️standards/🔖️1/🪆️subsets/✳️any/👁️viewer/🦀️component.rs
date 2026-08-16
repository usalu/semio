//! 👁️ EN 1990 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `En1990Viewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<En1990Viewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::en1990::{EN1990_DIALECT, EN1990_DOCUMENT_SCHEMA};
use crate::artifacts::en1990::En1990Snapshot;
use crate::viewer::en1990::modes::view;
use crate::viewer::en1990::modes::view::windows::report;
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
pub enum En1990ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for En1990ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(En1990ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct En1990Viewer;

impl ArtifactViewer for En1990Viewer {
    type Snapshot = En1990Snapshot;
    type Mutation = crate::artifacts::en1990::op::En1990Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = En1990ViewCommand;

    const DIALECT: Dialect = EN1990_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = EN1990_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> En1990Snapshot {
        En1990Snapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `En1990ViewCommand::Noop` variant never carries a config
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
pub fn create_en1990_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(EN1990_DIALECT)
        .document(["semio", "norm", "en1990"])
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
    fn create_en1990_viewer_builds_a_definition_for_this_dialect() {
        let def = create_en1990_viewer();
        assert_eq!(def.dialect, EN1990_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<En1990Viewer as ArtifactViewer>::DIALECT, EN1990_DIALECT);
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let snapshot = En1990Snapshot::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let json = serde_json::to_string(&<En1990Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default() })).expect("json");
        assert!(json.contains("Unknown body"));
    }
}
//#endregion 🧪️Tests
