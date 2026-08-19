//! 👁️ Fem2d viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Fem2dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Fem2dViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::fem2d::{Fem2dSnapshot, FEM2D_DIALECT, FEM_2D_SCHEMA};
use crate::viewer::fem2d::modes::view;
use crate::viewer::fem2d::modes::view::windows::model;
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fem2dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Fem2dViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Fem2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Fem2dViewer;

impl ArtifactViewer for Fem2dViewer {
    type Snapshot = Fem2dSnapshot;
    type Mutation = crate::artifacts::fem2d::op::Fem2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Fem2dViewCommand;

    const DIALECT: Dialect = FEM2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FEM_2D_SCHEMA;

    /// 🌱️ A real, non-empty default scene: the bundled `fem2d` example DSL, falling back to the empty
    /// document on a parse error (should never trigger — the fixture is asserted parseable by the
    /// sibling editor's own tests — but a viewer must never panic building its initial snapshot).
    async fn initial_snapshot() -> Fem2dSnapshot {
        use store::ArtifactDsl;
        Fem2dSnapshot::parse_dsl(crate::artifacts::fem2d::dsl::FEM2D_EXAMPLE_TEXT).unwrap_or_else(|_| crate::artifacts::fem2d::schema::empty_fem2d_snapshot())
    }

    /// 👁️ Structurally read-only: the sole `Fem2dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan, "jump to region") is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            model::BODY_KEY => model::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_fem2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(FEM2D_DIALECT)
        .document(["semio", "fem", "fem2d"])
        .icon_id("fem-app")
        .mode(view::FEM2D_VIEW_MODE_VIEW, semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), "eye")
        .default_mode_id(view::FEM2D_VIEW_MODE_VIEW)
        .window_kind(model::WINDOW_KIND_ID, semio_framework_plugin::LocalizedLabel::native("Model", "Modell"), model::BODY_KEY, semio_framework_plugin::SurfaceKind::Canvas2d, "fem-model")
        .default_layout(semio_framework_plugin::create_default_layout(&[model::WINDOW_KIND_ID.into()], "row", None, None))
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_fem2d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_fem2d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, FEM2D_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Fem2dViewer as ArtifactViewer>::DIALECT, FEM2D_DIALECT);
    }

    #[semio_framework_async_macros::async_test]
    async fn initial_snapshot_is_non_empty() {
        let snapshot = <Fem2dViewer as ArtifactViewer>::initial_snapshot();
        assert!(!snapshot.nodes.is_empty(), "expected the bundled example fixture's nodes");
    }
}
//#endregion 🧪️Tests
