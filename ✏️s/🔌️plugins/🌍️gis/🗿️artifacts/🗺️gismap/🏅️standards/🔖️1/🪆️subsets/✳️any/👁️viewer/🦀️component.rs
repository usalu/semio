//! 👁️ GIS map viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `GisMapViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<GisMapViewer>` (framework SDK)
//! is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::artifacts::gismap::schema::default_document;
use crate::artifacts::gismap::{GisMapSnapshot, GISMAP_DIALECT, GIS_MAP_SCHEMA};
use crate::viewer::gismap::modes::view;
use crate::viewer::gismap::modes::view::windows::map;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, UiNode, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GisMapViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for GisMapViewCommand {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    async fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(GisMapViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct GisMapViewer;

impl ArtifactViewer for GisMapViewer {
    type Snapshot = GisMapSnapshot;
    type Mutation = crate::artifacts::gismap::op::GisMapMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = GisMapViewCommand;

    const DIALECT: Dialect = GISMAP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_MAP_SCHEMA;

    async fn initial_snapshot() -> GisMapSnapshot {
        default_document()
    }

    /// 👁️ Structurally read-only: the sole `GisMapViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan) is a pure addition here, never a signature change.
    async fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &semio_framework_plugin::app::InteractionView<'_>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>) -> UiNode {
        match body_key {
            map::BODY_KEY => map::render(doc.snapshot),
            _ => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_gismap_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(GISMAP_DIALECT)
        .document(["semio", "gis", "2d"])
        .icon_id("gis2d")
        .mode_def(view::definition())
        .default_mode_id(view::GIS_MAP_VIEW_MODE_VIEW)
        .window_kind_def(map::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn create_gismap_viewer_builds_a_definition_for_the_viewer_role() {
        let def = create_gismap_viewer();
        assert_eq!(def.role, semio_framework::AppRole::Viewer);
        assert_eq!(def.dialect, GISMAP_DIALECT.into());
    }

    #[test]
    async fn viewer_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<GisMapViewer as ArtifactViewer>::DIALECT, GISMAP_DIALECT);
    }
}
//#endregion 🧪️Tests
