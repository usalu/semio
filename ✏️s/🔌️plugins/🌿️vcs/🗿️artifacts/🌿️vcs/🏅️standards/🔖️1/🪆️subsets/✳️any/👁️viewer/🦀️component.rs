//! 🌿️ VCS viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `VcsViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<VcsViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::vcs::{VcsSnapshot, VCS_DIALECT, VCS_DOCUMENT_SCHEMA};
use crate::viewer::vcs::modes::view;
use crate::viewer::vcs::modes::view::windows::history;
use semio_framework_plugin::{ArtifactView, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode};
// 🚧️ SDK GAP: `InteractionView` is only reachable through `app`, not yet in the crate-root
// re-export list (same gap the sibling editor surface's own note documents).
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactViewer, Dialect, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert, `Default`-deriving variant — real per-command payload modules the way
/// `✏️editor/🎮️commands/*` carries them would be pure ceremony for a surface that never dispatches
/// anything through `handle`. `Default` is required by `testkit::assert_viewer_never_mutates::<V>()`
/// (contract §2.5) to synthesize a representative command with no caller-supplied value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VcsViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for VcsViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(VcsViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct VcsViewer;

impl ArtifactViewer for VcsViewer {
    type Snapshot = VcsSnapshot;
    // 👁️ Decode-only per contract §2.2 — the store's op log must still decode every historical
    // mutation even though a viewer never constructs or dispatches one itself. Same artifact-level
    // type the editor uses, imported from the shared `🗿️artifacts/🌿️vcs` root, never from `editor`.
    type Mutation = crate::artifacts::vcs::VcsDemoMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = VcsViewCommand;

    const DIALECT: Dialect = VCS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;

    async fn initial_snapshot() -> VcsSnapshot {
        crate::artifacts::vcs::standards::v1::subsets::any::schema::empty_vcs_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `VcsViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (e.g. a
    /// checkpoint-tree hover/expand toggle) is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            history::BODY_KEY => history::render(doc.history),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_vcs_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(VCS_DIALECT).document(["semio", "vcs"]).icon_id("git-branch").mode_def(view::definition()).default_mode_id(view::VCS_VIEW_MODE_VIEW).window_kind_def(history::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_vcs_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_vcs_viewer();
        assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
        assert_eq!(def.dialect, VCS_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<VcsViewer as ArtifactViewer>::DIALECT, VCS_DIALECT);
    }
}
//#endregion 🧪️Tests
