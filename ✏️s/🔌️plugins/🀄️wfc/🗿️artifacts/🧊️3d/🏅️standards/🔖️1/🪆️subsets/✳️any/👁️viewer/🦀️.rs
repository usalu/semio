//! 👁️ WFC 3D viewer — the read-only `ArtifactViewer` for `s.wfc.wfc3d@1/*`: one `wfc-3d-view`
//! window showing the solved assembly. The viewer declares no mutating actions at all; its single
//! inert `Noop` command exists because the surface contract asks for a command type, not because
//! there is anything to author here.
//!
//! 🎨️ It deliberately declares NO `setActiveExample`, which is why the shell's boot announcement of
//! the active example is dropped here with an `undeclared-action` diagnostic (the same line every
//! viewer in this repo produces — see `📸️remodel`'s viewer, which says so in as many words). That is
//! the framework's gate doing its job, not a wfc fault: `ArtifactViewer` has no
//! `build_document_store_initialization_job` and `ViewerApp` forwards none, so a viewer is
//! STRUCTURALLY incapable of admitting the whole-document `Effect::LoadDocument` an example switch
//! is. Both surfaces of this dialect boot on the same example instead
//! (`initial_snapshot`), so the announcement has nothing to change anyway.

#![allow(clippy::result_large_err)]

use crate::viewer::wfc3d::modes::view;
use crate::viewer::wfc3d::modes::view::windows::preview;
use crate::{Wfc3dSnapshot, WFC3D_DIALECT, WFC3D_DOCUMENT_SCHEMA};
use crate::Wfc3dMutation;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Command
/// 👁️ The viewer's command channel — deliberately inert.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Wfc3dViewCommand {
    #[dsl(key = "noop")]
    Noop,
}

impl protocol::OpBinary for Wfc3dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Wfc3dViewer;

impl ArtifactViewer for Wfc3dViewer {
    type Snapshot = Wfc3dSnapshot;
    type Mutation = Wfc3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Wfc3dViewCommand;

    const DIALECT: Dialect = WFC3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = WFC3D_DOCUMENT_SCHEMA;

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

    fn initial_snapshot() -> Wfc3dSnapshot {
        crate::examples::two_room_corridor::snapshot()
    }

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
        render_body(body_key, doc.snapshot)
    }
}

/// 🖼️ The read-only render, as a PURE function of the document — `render`'s `ArtifactView` adapter
/// cannot be built outside the framework, so the body lives here where a test can reach it.
pub fn render_body(body_key: &str, document: &Wfc3dSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    match body_key {
        preview::WFC_3D_VIEW_BODY => preview::render(document).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_wfc3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(WFC3D_DIALECT)
        .document(["semio", "wfc", "3d"])
        .icon_id("network")
        .mode_def(view::definition())
        .default_mode_id(view::WFC_3D_VIEW_MODE_ID)
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
