//! 👁️ Generation2d viewer — the read-only counterpart of the editor surface for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Generation2dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Generation2dViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling editor module (`policyViewerPurityBreaches`).
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use crate::modes::view;
use crate::modes::view::windows::preview;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_s_artifact_procedural_2d_model::GENERATION2D_DIALECT;
use semio_s_artifact_procedural_2d_model::GENERATION_2D_SCHEMA;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way the editor's `🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, semio_framework_value::RetireOwned)]
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
    /// 📣️ The localized notices of the viewer's refusal codes (design §20.12): the document-level ones.
    fn fault_notices() -> &'static [(&'static str, semio_framework_ui_locale::LocalizedLabel)] {
        semio_s_artifact_procedural_2d_model::generation2d_document_fault_notices()
    }

    /// 🧩️ The loaded-parent child projection every archive load and maintenance swap asks for before a
    /// decoded document may replace the store. `Generation2dSnapshot` declares no child slot, so the
    /// projection is honestly empty; without it every replacement faulted with `viewer did not declare
    /// a loaded-parent child projection`.
    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new(semio_s_artifact_procedural_2d_model::GENERATION2D_CHILD_PROJECTION), error.to_string()))
    }

    type Snapshot = Generation2dSnapshot;
    type Mutation = semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Generation2dViewCommand;

    const DIALECT: Dialect = GENERATION2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = GENERATION_2D_SCHEMA;

    fn initial_snapshot() -> Generation2dSnapshot {
        semio_s_artifact_procedural_2d_io_text::snapshot::default_snapshot()
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

#[path = "."]
pub mod presence {
    #[path = "👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod config {
    #[path = "🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod modes {
    #[path = "."]
    pub mod view {
        #[path = "🎭️modes/👁️view/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod windows {
            #[path = "🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs"]
            pub mod preview;
        }
    }
}
