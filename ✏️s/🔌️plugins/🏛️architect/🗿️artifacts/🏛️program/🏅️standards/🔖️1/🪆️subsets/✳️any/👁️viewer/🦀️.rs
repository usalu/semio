//! 👁️ Architect viewer — the read-only counterpart of the sibling editor surface for this subset
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `ArchitectViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<ArchitectViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::program::op::ProgramMutation;
use crate::artifacts::program::{sample_plugin, ProgramSnapshot, ARCHITECT_DIALECT, ARCHITECT_PROGRAM_SCHEMA};
use crate::viewer::architect::modes::view;
use crate::viewer::architect::modes::view::windows::register;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
// 🚧️ `Dialect`/`InteractionView` are only reachable through `app`, not yet in the crate-root
// re-export list (see the identical note in the sibling editor surface's root `🦀️.rs`).
use semio_framework_plugin::app::{Dialect, InteractionView};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way the sibling editor surface's
/// `🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches anything
/// through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ArchitectViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for ArchitectViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(ArchitectViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct ArchitectViewer;

impl ArtifactViewer for ArchitectViewer {
    type Snapshot = ProgramSnapshot;
    type Mutation = ProgramMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = ArchitectViewCommand;

    const DIALECT: Dialect = ARCHITECT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ARCHITECT_PROGRAM_SCHEMA;

    async fn initial_snapshot() -> ProgramSnapshot {
        sample_plugin()
    }

    /// 👁️ Structurally read-only: the sole `ArchitectViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action is a pure
    /// addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            register::ARCHITECT_VIEW_BODY_REGISTER => register::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_architect_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(ARCHITECT_DIALECT)
        .document(["semio", "architect"])
        .icon_id("architect")
        .mode_def(view::definition())
        .default_mode_id(view::ARCHITECT_VIEW_MODE_VIEW)
        .window_kind_def(register::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_architect_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_architect_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, ARCHITECT_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<ArchitectViewer as ArtifactViewer>::DIALECT, ARCHITECT_DIALECT);
    }
}
//#endregion 🧪️Tests
