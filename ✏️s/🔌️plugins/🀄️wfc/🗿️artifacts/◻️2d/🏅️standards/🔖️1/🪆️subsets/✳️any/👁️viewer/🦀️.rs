//! 👁️ WFC 2D viewer — the read-only counterpart of `✏️editor` for this subset. `Wfc2dViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`, so this surface can never structurally emit
//! an artifact mutation. It must not import anything from the sibling mutation-capable surface
//! (`policyViewerPurityBreaches`), which is why it renders through its OWN preview window module
//! rather than reusing the editor's.

use crate::viewer::wfc2d::modes::view;
use crate::viewer::wfc2d::modes::view::windows::preview;
use crate::{Wfc2dMutation, Wfc2dSnapshot, WFC_2D_DIALECT, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Wfc2dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Wfc2dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Wfc2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Wfc2dViewer;

impl ArtifactViewer for Wfc2dViewer {
    type Snapshot = Wfc2dSnapshot;
    type Mutation = Wfc2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Wfc2dViewCommand;

    const DIALECT: Dialect = WFC_2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_2D_DOCUMENT_SCHEMA;

    /// 👁️ The viewer boots the SAME committed example the editor does, so opening one board in both
    /// roles shows one scene (`📓️explore-build-verify-playground-pipeline.md` pitfall 5).
    fn initial_snapshot() -> Wfc2dSnapshot {
        crate::examples::two_room_corridor::document()
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
            preview::WFC_2D_VIEW_BODY => preview::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_wfc2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(WFC_2D_DIALECT)
        .document(["semio", "wfc", "2d"])
        .icon_id("network")
        .mode_def(view::definition())
        .default_mode_id(view::WFC_2D_VIEW_MODE_ID)
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
