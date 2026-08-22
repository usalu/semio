//! 👁️ GIS terrain viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `GisTerrainViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<GisTerrainViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::gisterrain::schema::default_terrain_document;
use crate::artifacts::gisterrain::{GisTerrainSnapshot, GISTERRAIN_DIALECT, GIS_3D_TERRAIN_SCHEMA};
use crate::viewer::gisterrain::modes::view;
use crate::viewer::gisterrain::modes::view::windows::terrain;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GisTerrainViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for GisTerrainViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(GisTerrainViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct GisTerrainViewer;

impl ArtifactViewer for GisTerrainViewer {
    type Snapshot = GisTerrainSnapshot;
    type Mutation = crate::artifacts::gisterrain::op::GisTerrainMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = GisTerrainViewCommand;

    const DIALECT: Dialect = GISTERRAIN_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_3D_TERRAIN_SCHEMA;

    async fn initial_snapshot() -> GisTerrainSnapshot {
        default_terrain_document()
    }

    /// 👁️ Structurally read-only: the sole `GisTerrainViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no
    /// dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action
    /// (camera orbit) is a pure addition here, never a signature change.
    async fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            terrain::BODY_KEY => terrain::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_gisterrain_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(GISTERRAIN_DIALECT)
        .document(["semio", "gis", "3d"])
        .icon_id("gis3d")
        .mode_def(view::definition())
        .default_mode_id(view::GIS_TERRAIN_VIEW_MODE_VIEW)
        .window_kind_def(terrain::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn create_gisterrain_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_gisterrain_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, GISTERRAIN_DIALECT.into());
    }

    #[semio_framework_async_macros::async_test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<GisTerrainViewer as ArtifactViewer>::DIALECT, GISTERRAIN_DIALECT);
    }
}
//#endregion 🧪️Tests
