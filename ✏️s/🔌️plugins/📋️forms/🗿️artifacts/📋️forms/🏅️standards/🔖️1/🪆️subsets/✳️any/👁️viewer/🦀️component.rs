//! 👁️ Forms viewer — the read-only counterpart of the sibling editor surface for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `FormsViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<FormsViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::forms::op::FormMutation;
use crate::artifacts::forms::schema::building_component_spec;
use crate::artifacts::forms::{FormsSnapshot, FORMS_DIALECT, FORMS_DOCUMENT_SCHEMA};
use crate::viewer::forms::modes::view;
use crate::viewer::forms::modes::view::windows::try_wizard;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
// 🚧️ `Dialect`/`InteractionView` are only reachable through `app`, not yet in the crate-root
// re-export list (see the identical note in the sibling editor surface's root `🦀️component.rs`).
use semio_framework_plugin::app::{Dialect, InteractionView};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FormsViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for FormsViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(FormsViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct FormsViewer;

impl ArtifactViewer for FormsViewer {
    type Snapshot = FormsSnapshot;
    type Mutation = FormMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = FormsViewCommand;

    const DIALECT: Dialect = FORMS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> FormsSnapshot {
        building_component_spec()
    }

    /// 👁️ Structurally read-only: the sole `FormsViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action is a
    /// pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            try_wizard::BODY_KEY => try_wizard::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_forms_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(FORMS_DIALECT).document(["semio", "forms"]).icon_id("forms").mode_def(view::definition()).default_mode_id(view::FORMS_VIEW_MODE_VIEW).window_kind_def(try_wizard::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_forms_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_forms_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, FORMS_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<FormsViewer as ArtifactViewer>::DIALECT, FORMS_DIALECT);
    }
}
//#endregion 🧪️Tests
