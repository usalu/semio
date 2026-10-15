//! 👁️ Process 3D viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Process3dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Process3dViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact or draft
//! mutation. MUST NOT import anything from the sibling `editor` module (`policyViewerPurityBreaches`).

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
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
            #[path = "."]
            pub mod workpiece {
                #[path = "🎭️modes/👁️view/🪟️windows/🪚️workpiece/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
//#endregion 🧩️Modules

use crate::modes::view;
use crate::modes::view::windows::workpiece;
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_s_artifact_process_process3d_model::{snapshot::Process3dSnapshot, snapshot::PROCESS3D_DIALECT, snapshot::PROCESS_3D_SCHEMA};
use semio_framework_plugin::app::InteractionView;
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
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Command
/// 👁️ The viewer declares no actions (no utilities, no mutations), so its typed command channel has
/// exactly one inert variant — real per-command payload modules the way `✏️editor/🎮️commands/*`
/// carries them would be pure ceremony for a surface that never dispatches anything through `handle`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Process3dViewCommand {
    Noop,
}

semio_framework_value::artifact_retire_leaf!(Process3dViewCommand);

impl protocol::OpBinary for Process3dViewCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(Process3dViewCommand::Noop)
    }
}
//#endregion 🔖️Command

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct Process3dViewer;

impl ArtifactViewer for Process3dViewer {
    /// 🧩️ The same member roster the editor declares — see `Process3dPlayApp`'s `Members`.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = Process3dViewCommand;

    const DIALECT: Dialect = PROCESS3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PROCESS_3D_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_s_artifact_process_process3d_model::snapshot::process3d_child_restore_projection(snapshot)
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(snapshot, slot, child_id)
    
})())
}

    fn initial_snapshot() -> Process3dSnapshot {
        semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document()
    }

    /// 👁️ Structurally read-only: the sole `Process3dViewCommand::Noop` variant never carries a
    /// config change, so this always returns the empty `ViewEmit` — no config mutation, no effect,
    /// no dirty scope. Kept as a real dispatch (not an `unreachable!()`) so a future view-only
    /// action (camera orbit, "jump to step") is a pure addition here, never a signature change.
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
            workpiece::PROCESS3D_VIEW_BODY_MAIN => workpiece::render(doc.snapshot).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_process3d_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(PROCESS3D_DIALECT)
        .document(["semio", "process", "3d"])
        .icon_id("hammer")
        .mode_def(view::definition())
        .default_mode_id(view::PROCESS3D_VIEW_MODE_VIEW)
        .window_kind_def(workpiece::definition())
        .default_layout(view::layout())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
