//! 👁️ Lowpoly viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `LowpolyViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<LowpolyViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

use crate::artifacts::lowpoly::schema::default_snapshot;
use crate::artifacts::lowpoly::{LowpolySnapshot, LOWPOLY_DIALECT, LOWPOLY_DOCUMENT_SCHEMA};
use crate::viewer::lowpoly::modes::view;
use crate::viewer::lowpoly::modes::view::windows::model;
use semio_framework_plugin::app::{ArtifactViewer, Dialect, ViewEmit, Viewer};
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LowpolyViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for LowpolyViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(LowpolyViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct LowpolyViewer;

impl ArtifactViewer for LowpolyViewer {
    type Snapshot = LowpolySnapshot;
    type Mutation = crate::artifacts::lowpoly::op::LowpolyMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = LowpolyViewCommand;

    const DIALECT: Dialect = LOWPOLY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> LowpolySnapshot {
        default_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `LowpolyViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, "jump to object") is a pure addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            model::BODY_KEY => model::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_lowpoly_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(LOWPOLY_DIALECT).document(["semio", "lowpoly"]).icon_id("shapes").mode_def(view::definition()).default_mode_id(view::LOWPOLY_VIEW_MODE_VIEW).window_kind_def(model::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_lowpoly_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_lowpoly_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, LOWPOLY_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<LowpolyViewer as ArtifactViewer>::DIALECT, LOWPOLY_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_command_default_is_noop() {
        assert_eq!(LowpolyViewCommand::default(), LowpolyViewCommand::Noop);
    }
}
//#endregion 🧪️Tests
