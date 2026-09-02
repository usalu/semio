//! 👁️ Playground viewer — the read-only counterpart of the sibling authoring surface for this subset
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `PlaygroundViewer`
//! implements `ArtifactViewer`, never the mutating authoring trait — `ViewerApp<PlaygroundViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling authoring module (`policyViewerPurityBreaches`
//! forbids it outright, including the substring in comments).

use crate::artifacts::playground::standards::v1::subsets::any::schema::empty_playground_snapshot;
use crate::artifacts::playground::standards::v1::subsets::any::schema::mutations::PlaygroundMutation;
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use crate::artifacts::playground::{PLAYGROUND_DIALECT, PLAYGROUND_DOCUMENT_SCHEMA};
use crate::viewer::playground::modes::view;
use crate::viewer::playground::modes::view::windows::main;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiAssemblyResult, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — a real per-command payload module the way the authoring surface's
/// `🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches anything
/// through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlaygroundViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for PlaygroundViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(PlaygroundViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct PlaygroundViewer;

impl ArtifactViewer for PlaygroundViewer {
    type Snapshot = PlaygroundSnapshot;
    type Mutation = PlaygroundMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = PlaygroundViewCommand;

    const DIALECT: Dialect = PLAYGROUND_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PLAYGROUND_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> PlaygroundSnapshot {
        empty_playground_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `PlaygroundViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no
    /// dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action is
    /// a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiAssemblyResult<ComponentTree> {
        match body_key {
            main::BODY_KEY => main::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_playground_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PLAYGROUND_DIALECT)
        .document(["semio", "playground"])
        .icon_id("playground")
        .mode_def(view::definition())
        .default_mode_id(view::PLAYGROUND_VIEW_MODE_VIEW)
        .window_kind_def(main::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_playground_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_playground_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, PLAYGROUND_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<PlaygroundViewer as ArtifactViewer>::DIALECT, PLAYGROUND_DIALECT);
    }
}
//#endregion 🧪️Tests
