//! 👁️ Equation viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `EquationViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<EquationViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an
//! artifact or draft mutation. MUST NOT import anything from the sibling editor module (the purity
//! check forbids it outright).
//!
//! `Config`/`Presence`/`Transient` are the framework's `NoConfig`/`NoPresence`/`NoTransient` — this
//! viewer needs no persisted per-session state to render (no camera, no locale): the Geometry
//! window's table has nothing view-dependent to remember between renders.

use crate::viewer::equation::modes::view;
use crate::viewer::equation::modes::view::windows::geometry;
use crate::{EquationSnapshot, EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ArtifactViewer, ConfigView, Dialect, Fault, Label, NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation, ViewEmit, Viewer};
use store::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EquationViewCommand {
    #[default]
    Noop,
}

impl protocol::OpBinary for EquationViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(EquationViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct EquationViewer;

impl ArtifactViewer for EquationViewer {
    /// 🧩️ Same composed-child roster as the editor — the contract is declared on BOTH surfaces.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = EquationSnapshot;
    type Mutation = crate::op::EquationMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EquationViewCommand;

    const DIALECT: Dialect = EQUATION_DIALECT;
    /// 🧬️ The crate's one loaded-parent child projection (`crate::equation_child_restore_projection`).
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
        crate::equation_child_restore_projection(snapshot)
    }
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    /// 🔐️ The document-store owner catalogue, identical to the sibling editor's: a viewer owns the
    /// very same `EquationSnapshot` envelope and must allocate and retire it the same way. Read-only
    /// says nothing about ownership.
    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    /// 🧹️ The bounded disposer `VcsArtifactApp`'s close ladder drives for the `document-store` lane.
    ///
    /// 🐛️ Left at the trait default (`None`) this surface can NEVER close: every close of a
    /// `ViewerApp<EquationViewer>` faults `interactive-job.close-owned-disposer-missing` ("app owner
    /// did not provide the required bounded disposer for document-store") and the store then reaches
    /// `Drop` without its terminal-empty witness — which on a `panic = "abort"` wasm32 guest is an
    /// `unreachable` that kills the whole instance. The codec resolver constructs every app of the
    /// bundle to read its schema and closes the ones it does not return, so a viewer missing this
    /// disposer faults `codec.*` calls that never touch the viewer at all (the exact defect
    /// `🗒️note`'s viewer documents for itself, ticket 26/09/18 slices TC3c §5f / TC3d §1 / TC3e).
    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    /// 🧹️ The remaining three lanes `VcsArtifactApp`'s close ladder drives. This viewer declares
    /// `NoConfig`/`NoPresence`/`NoTransient`, so each is the framework's own empty owner — but the
    /// ladder still demands a disposer per lane, and the trait default `None` faults the close the
    /// same way `document-store` did. Mirrors `🗒️note`'s viewer, which carries the identical set.
    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
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

    /// 🌱️ The derivable `notation`/`results`/`computed` members — see
    /// `crate::genesis_equation_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
        crate::genesis_equation_child_pack(snapshot, slot, child_id)
    }

    fn initial_snapshot() -> EquationSnapshot {
        EquationSnapshot::default()
    }

    /// 👁️ Structurally read-only: the sole `EquationViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect,
    /// no dirty scope. Kept as a real dispatch (not `unreachable!()`) so a future view-only action
    /// is a pure addition here, never a signature change.
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
        let node = match body_key {
            geometry::BODY_KEY => geometry::render(doc.snapshot),
            _ => return semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_equation_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(EQUATION_DIALECT)
        .document(["semio", "equation"])
        .icon_id("math-app")
        .mode_def(view::definition())
        .default_mode_id(view::MATH_VIEW_MODE_VIEW)
        .window_kind_def(geometry::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
