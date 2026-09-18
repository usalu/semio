//! 👁️ 2D-grid viewer — the read-only counterpart of `✏️editor` for this subset. `Grid2dViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`, so this surface can never structurally emit
//! an artifact mutation. It must not import anything from the sibling mutation-capable surface
//! (`policyViewerPurityBreaches`).

use crate::viewer::grid2d::modes::view;
use crate::viewer::grid2d::modes::view::windows::preview;
use crate::{Grid2dMutation, Grid2dSnapshot, WFC_GRID2D_DIALECT, WFC_GRID2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Grid2dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Grid2dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Grid2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Grid2dViewer;

impl ArtifactViewer for Grid2dViewer {
    type Snapshot = Grid2dSnapshot;
    type Mutation = Grid2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Grid2dViewCommand;

    const DIALECT: Dialect = WFC_GRID2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_GRID2D_DOCUMENT_SCHEMA;

    /// 👁️ The viewer boots the subset's own committed default example, so editor and viewer share
    /// one scene instead of the viewer falling back to a hardcoded empty grid.
    fn initial_snapshot() -> Grid2dSnapshot {
        crate::examples::grid2d::pipes::document()
    }

    /// 👁️ Structurally read-only: the sole `Noop` variant never carries a config change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &store::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            preview::BODY_KEY => preview::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_grid2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(WFC_GRID2D_DIALECT)
        .document(["semio", "wfc", "grid2d"])
        .icon_id("puzzle")
        .mode_def(view::definition())
        .default_mode_id(view::GRID2D_VIEW_MODE_ID)
        .window_kind_def(preview::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
