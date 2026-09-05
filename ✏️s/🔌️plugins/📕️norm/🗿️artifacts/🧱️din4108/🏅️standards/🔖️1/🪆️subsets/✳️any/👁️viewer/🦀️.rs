//! 👁️ DIN 4108 viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Din4108Viewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Din4108Viewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::din4108::Din4108Snapshot;
use crate::artifacts::din4108::{DIN4108_DIALECT, DIN4108_DOCUMENT_SCHEMA};
use crate::viewer::din4108::modes::view;
use crate::viewer::din4108::modes::view::windows::report;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
// 🚧️ SDK GAP: see the identical note in `✏️editor/🦀️.rs` — `Dialect` is only reachable
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
pub enum Din4108ViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Din4108ViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Din4108ViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Din4108Viewer;

impl ArtifactViewer for Din4108Viewer {
    type Snapshot = Din4108Snapshot;
    type Mutation = crate::artifacts::din4108::op::Din4108Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Din4108ViewCommand;

    const DIALECT: Dialect = DIN4108_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = DIN4108_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> Din4108Snapshot {
        Din4108Snapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `Din4108ViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit`. Kept as a real dispatch (not
    /// `unreachable!()`) so a future view-only action is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            report::BODY_KEY => report::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_din4108_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(DIN4108_DIALECT)
        .document(["semio", "norm", "din4108"])
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

    #[semio_framework_async_macros::async_test]
    fn create_din4108_viewer_builds_a_definition_for_this_dialect() {
        let def = create_din4108_viewer();
        assert_eq!(def.dialect, DIN4108_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Din4108Viewer as ArtifactViewer>::DIALECT, DIN4108_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let snapshot = Din4108Snapshot::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let json = serde_json::to_string(&<Din4108Viewer as ArtifactViewer>::render("nope", &doc, &ConfigView { snapshot: &NoConfig::default() })).expect("json");
        assert!(json.contains("Unknown body"));
    }
}
//#endregion 🧪️Tests
