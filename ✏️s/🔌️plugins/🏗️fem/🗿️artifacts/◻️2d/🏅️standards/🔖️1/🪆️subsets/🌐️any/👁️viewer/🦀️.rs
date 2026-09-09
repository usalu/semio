//! 👁️ Fem2d viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Fem2dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Fem2dViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::viewer::fem2d::modes::view;
use crate::viewer::fem2d::modes::view::windows::model;
use crate::{Fem2dSnapshot, FEM2D_DIALECT, FEM_2D_SCHEMA};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
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
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Fem2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Fem2dViewer;

impl ArtifactViewer for Fem2dViewer {
    type Snapshot = Fem2dSnapshot;
    type Mutation = crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Fem2dViewCommand;

    const DIALECT: Dialect = FEM2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FEM_2D_SCHEMA;

    /// 🌱️ A real, non-empty default scene: the artifact-side boot document (the bundled `fem2d`
    /// example DSL, falling back to the empty document on a parse error) the sibling editor boots on
    /// too — no editor import, this is `crate::standards::v1::subsets::any::schema`.
    fn initial_snapshot() -> Fem2dSnapshot {
        crate::standards::v1::subsets::any::schema::default_fem2d_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `Fem2dViewCommand::Noop` variant never carries a config
    /// change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no dirty
    /// scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action (camera
    /// pan, "jump to region") is a pure addition here, never a signature change.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        match body_key {
            model::BODY_KEY => model::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fem2d viewer unknown-body label admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_fem2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(FEM2D_DIALECT)
        .document(["semio", "fem", "fem2d"])
        .icon_id("fem-app")
        .mode(view::FEM2D_VIEW_MODE_VIEW, semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), "eye")
        .default_mode_id(view::FEM2D_VIEW_MODE_VIEW)
        .window_kind(model::WINDOW_KIND_ID, semio_framework_plugin::LocalizedLabel::native("Model", "Modell"), model::BODY_KEY, semio_framework_ui_contract::SurfaceKind::Canvas2d, "fem-model")
        .default_layout(semio_framework_plugin::create_default_layout(&[model::WINDOW_KIND_ID.into()], "row", None, None))
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
