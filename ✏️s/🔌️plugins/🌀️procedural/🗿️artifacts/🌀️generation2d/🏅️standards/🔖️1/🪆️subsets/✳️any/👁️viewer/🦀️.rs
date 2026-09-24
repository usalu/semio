//! 👁️ Generation2d viewer — the read-only counterpart of the editor surface for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Generation2dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Generation2dViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).

use crate::viewer::generation2d::modes::view;
use crate::viewer::generation2d::modes::view::windows::preview;
use crate::{Generation2dSnapshot, GENERATION2D_DIALECT, GENERATION_2D_SCHEMA};
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way the editor's `🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Generation2dViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for Generation2dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Generation2dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Generation2dViewer;

impl ArtifactViewer for Generation2dViewer {
    /// 🧩️ The loaded-parent child projection every archive load and maintenance swap asks for before a
    /// decoded document may replace the store. `Generation2dSnapshot` declares no child slot, so the
    /// projection is honestly empty; without it every replacement faulted with `viewer did not declare
    /// a loaded-parent child projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("generation2d.child-projection"), error.to_string()))
    }

    type Snapshot = Generation2dSnapshot;
    type Mutation = crate::standards::v1::subsets::any::schema::mutations::text::Generation2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Generation2dViewCommand;

    const DIALECT: Dialect = GENERATION2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;

    /// 🔐️ The artifact's own document-store owner catalogue, identical to the sibling editor's: the snapshot holds
    /// retained `OrderedMap` roots that only this catalogue retires explicitly, and the framework's generic bounded
    /// owners dropped them plainly (`ordered-map root must be explicitly retired before drop`, S15 viewer matrix).
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_document_store_owners())
    }

    fn initial_snapshot() -> Generation2dSnapshot {
        crate::standards::v1::subsets::any::schema::default_snapshot()
    }

    /// 👁️ Structurally read-only: the sole `Generation2dViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect, no
    /// dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only action
    /// (pan/zoom, "jump to widget") is a pure addition here, never a signature change.
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
        let node = match body_key {
            preview::BODY_KEY => preview::render(doc.snapshot),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.unknown-body", "fixed UI unknown-body admission failed")),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_generation2d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(GENERATION2D_DIALECT)
        .document(["semio", "procedural", "2d"])
        .icon_id("generation2d")
        .mode_def(view::definition())
        .default_mode_id(view::GENERATION2D_VIEW_MODE_VIEW)
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
