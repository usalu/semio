//! 👁️ S Home viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `HomeViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<HomeViewer>` (framework SDK) is
//! the sole runtime adapter, so this file can never structurally emit an artifact or draft mutation.
//! MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::{SHomeSnapshot, HOME_DIALECT, S_HOME_DOCUMENT_SCHEMA};
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use crate::viewer::home::modes::view;
use crate::viewer::home::modes::view::windows::main;
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Fault, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, PluginAssemblyError, UiAssemblyResult, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The read-only Home surface accepts no command: its directory projection is written only by the
/// editor's sealed-page lane (`applyDirectoryEventPage`), which the viewer renders but never advances.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum HomeViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for HomeViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(HomeViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct HomeViewer;

impl ArtifactViewer for HomeViewer {
    type Snapshot = SHomeSnapshot;
    type Mutation = crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
    // 📇️ Shared with the editor (`crate::editor::home::config::HomeConfig`), not `NoConfig` — the
    // viewer renders the SAME hub-directory-fed table (`crate::home_space_rows`) and must therefore
    // read the SAME folded `directory_json`. `assert_viewer_never_mutates` only asserts the ARTIFACT/
    // draft store never advances (contract §2.5) — the config lane is fair game for both surfaces, and
    // a viewer emitting a `ConfigMutation` is not a document mutation.
    type Config = HomeConfig;
    type ConfigMutation = HomeConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = HomeViewCommand;

    const DIALECT: Dialect = HOME_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = S_HOME_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> SHomeSnapshot {
        SHomeSnapshot::default()
    }

    /// 🪪️ Same app-schema descriptor as the editor (contract requires both surfaces sharing a dialect
    /// to also share a config schema, since it is registered per-document-schema, not per-role).
    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::home::config::schema::app_schema_descriptor())
    }

    /// 👁️ Structurally read-only: the sole `HomeViewCommand::Noop` answers the empty emit — no config
    /// change, no effect, never a document edit.
    fn handle(_command: &Self::Command, _doc: &ArtifactView<'_, Self::Snapshot>, _cfg: &ConfigView<'_, Self::Config>, _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    /// 👁️ Renders the SAME overview table the editor's main window does, read-only: no create/delete/
    /// rename/share affordances, fed by `cfg.snapshot.directory()` — never the artifact document itself.
    fn render(body_key: &str, _doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        let root = match body_key {
            main::S_HOME_VIEW_BODY => {
                let directory = cfg.snapshot.directory().map_err(|_| PluginAssemblyError::new("s.home.directory-projection-malformed", "Home directory projection is invalid"))?;
                main::render(&directory, view_state)?
            }
            _ => {
                semio_framework_plugin::built_text_node(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))).map_err(|_| PluginAssemblyError::new("s.home.viewer.render.unknown-body", "unknown body key text admission failed"))?
            }
        };
        Ok(ComponentTree { root })
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub async fn create_home_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(HOME_DIALECT).document(["semio", "s", "home"]).icon_id("home").mode_def(view::definition()).default_mode_id(view::S_HOME_VIEW_MODE).window_kind_def(main::definition()).default_layout(view::layout()).build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
