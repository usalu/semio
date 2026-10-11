//! 👁️ GIS terrain viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `GisTerrainViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<GisTerrainViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::standards::v1::subsets::any::io::text::snapshot::default_terrain_document;
use crate::viewer::gisterrain::modes::view;
use crate::viewer::gisterrain::modes::view::windows::terrain;
use crate::{GisTerrainSnapshot, GISTERRAIN_DIALECT, GIS_3D_TERRAIN_SCHEMA};
use {semio_framework_plugin::ArtifactView,semio_framework_plugin::ArtifactViewer,semio_framework_plugin::ConfigView,semio_framework_artifact_reference::Dialect,semio_framework_plugin::Fault,semio_framework_plugin::NoConfig,semio_framework_plugin::NoConfigMutation,semio_framework_plugin::NoPresence,semio_framework_plugin::NoPresenceMutation,semio_framework_plugin::NoTransient,semio_framework_plugin::NoTransientMutation,semio_framework_plugin::ViewEmit,semio_framework_plugin::Viewer};
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum GisTerrainViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for GisTerrainViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(GisTerrainViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct GisTerrainViewer;

impl ArtifactViewer for GisTerrainViewer {
    type Snapshot = GisTerrainSnapshot;
    type Mutation = crate::standards::v1::subsets::any::schema::mutations::GisTerrainMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = GisTerrainViewCommand;

    const DIALECT: Dialect = GISTERRAIN_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GIS_3D_TERRAIN_SCHEMA;

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn initial_snapshot() -> GisTerrainSnapshot {
        default_terrain_document()
    }

    /// 👁️ Structurally read-only: the sole `GisTerrainViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no
    /// dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action
    /// (camera orbit) is a pure addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            terrain::BODY_KEY => terrain::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("Unknown body: {body_key}"))),
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
