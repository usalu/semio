//! 👁️ FEM 3D viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Fem3dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Fem3dViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::fem3d::Fem3dSnapshot;
use crate::viewer::fem3d::modes::view;
use crate::viewer::fem3d::modes::view::windows::model;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer,
};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fem3dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Fem3dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Fem3dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Fem3dViewer;

impl ArtifactViewer for Fem3dViewer {
    type Snapshot = Fem3dSnapshot;
    type Mutation = crate::artifacts::fem3d::op::Fem3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Fem3dViewCommand;

    const DIALECT: Dialect = crate::artifacts::fem3d::FEM3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::fem3d::FEM_3D_SCHEMA;

    /// 👁️ Real, non-empty default scene: the artifact's own bundled `default` example DSL, falling
    /// back to the empty snapshot only if that fixture ever fails to parse.
    fn initial_snapshot() -> Fem3dSnapshot {
        crate::artifacts::fem3d::dsl::parse_dsl(crate::artifacts::fem3d::dsl::FEM3D_EXAMPLE_TEXT).unwrap_or_else(|_| crate::artifacts::fem3d::schema::empty_fem3d_snapshot())
    }

    /// 👁️ Structurally read-only: the sole `Fem3dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// orbit, result-mode toggle) is a pure addition here, never a signature change.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            model::BODY_KEY => model::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
/// 🧱️ Scalar `.mode(..)`/`.window_kind(..)` builder calls throughout — matches the sibling editor
/// module's manifest style (its own `create_fem3d_app`), never a `ModeDefinition`/
/// `WindowKindDefinition` passthrough.
pub fn create_fem3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(crate::artifacts::fem3d::FEM3D_DIALECT)
        .document(["semio", "fem", "fem3d"])
        .icon_id("fem-app")
        .mode(view::FEM3D_VIEW_MODE_VIEW, semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), "eye")
        .default_mode_id(view::FEM3D_VIEW_MODE_VIEW)
        .window_kind(model::WINDOW_KIND_ID, semio_framework_plugin::LocalizedLabel::native("Model", "Modell"), model::BODY_KEY, semio_framework_plugin::SurfaceKind::World3d, "fem-model")
        .default_layout(semio_framework_plugin::create_default_layout(&[model::WINDOW_KIND_ID.into()], "row", None, None))
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fem3d_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_fem3d_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, crate::artifacts::fem3d::FEM3D_DIALECT.into());
    }

    #[test]
    fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<Fem3dViewer as ArtifactViewer>::DIALECT, crate::artifacts::fem3d::FEM3D_DIALECT);
    }

    #[test]
    fn initial_snapshot_is_the_bundled_example_not_empty() {
        let snapshot = <Fem3dViewer as ArtifactViewer>::initial_snapshot();
        assert!(!snapshot.nodes.is_empty(), "expected the bundled default example's nodes");
    }
}
//#endregion 🧪️Tests
