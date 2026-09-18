//! 👁️ Bitmap viewer — the read-only counterpart of `✏️editor` for this subset. `BitmapViewer`
//! implements `ArtifactViewer`, never `ArtifactEditor`, so this file can never structurally emit an
//! artifact mutation. It imports nothing from the sibling mutation-capable surface: the shared
//! canvas layer builder it renders through lives at the ARTIFACT root, which both surfaces may read.

use crate::viewer::bitmap::modes::view;
use crate::viewer::bitmap::modes::view::windows::{input, output};
use crate::{BitmapMutation, BitmapSnapshot, WFC_BITMAP_DIALECT, WFC_BITMAP_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};

//#region 🔖️Command
/// 👁️ The viewer declares no actions, so its typed command channel has exactly one inert variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BitmapViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for BitmapViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(BitmapViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct BitmapViewer;

impl ArtifactViewer for BitmapViewer {
    type Snapshot = BitmapSnapshot;
    type Mutation = BitmapMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = BitmapViewCommand;

    const DIALECT: Dialect = WFC_BITMAP_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC_BITMAP_DOCUMENT_SCHEMA;

    /// 🗃️♻️ The same owned-store declaration the editor carries: the instance close ladder walks one
    /// store lane per stage and faults the whole close with `interactive-job.close-owned-disposer-missing`
    /// (or, one layer down, `artifact store has no owner-supplied bounded disposer`) the moment a lane
    /// answers `None`. A viewer never edits these stores, but it still owns and must release them.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 👁️ The viewer boots the same committed example the editor does, so opening one document in
    /// either surface shows the same scene rather than an empty default.
    fn initial_snapshot() -> BitmapSnapshot {
        crate::examples::rooms_16::snapshot()
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
            input::BODY_KEY => input::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            output::BODY_KEY => output::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_bitmap_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(WFC_BITMAP_DIALECT)
        .document(["semio", "wfc", "bitmap"])
        .icon_id("image")
        .mode_def(view::definition())
        .default_mode_id(view::WFC_BITMAP_MODE_VIEW)
        .window_kind_def(input::definition())
        .window_kind_def(output::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
