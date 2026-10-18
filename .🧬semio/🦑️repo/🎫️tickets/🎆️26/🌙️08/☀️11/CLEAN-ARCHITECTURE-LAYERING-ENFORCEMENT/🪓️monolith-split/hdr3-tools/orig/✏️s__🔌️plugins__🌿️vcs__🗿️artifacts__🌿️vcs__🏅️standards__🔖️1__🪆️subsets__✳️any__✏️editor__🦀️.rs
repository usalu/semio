//! 🖥️ VCS editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, headless compute in the artifact's `🧬️schema` (dissolved from `⚙️engine` per ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). This file is a routing table: `handle` →
//! `VcsCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use crate::editor::vcs::commands::edit as edit_command;
use crate::editor::vcs::commands::example::set_active_example;
use crate::editor::vcs::commands::{change_counter, change_notes, change_status, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, canvas_wheel, increment_counter, no_operation, rename_vcs, text_edit};
use crate::editor::vcs::config::{VcsDemoConfig, VcsDemoConfigMutation};
use crate::editor::vcs::modes::edit;
use crate::editor::vcs::modes::edit::windows::{editor, history};
use crate::editor::vcs::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::vcs::presence::{VcsDemoPresence, VcsDemoPresenceMutation};
use crate::editor::vcs::terminology::vcs_play_labels;
use crate::{op::VcsDemoMutation, VcsSnapshot, VCS_DOCUMENT_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use std::collections::BTreeSet;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants

#[path = "📋️constants-and-ui-helpers/🦀️.rs"]
mod constants_and_ui_helpers;
pub use constants_and_ui_helpers::*;

#[path = "🕹️command-surface-and-job-factories/🦀️.rs"]
mod command_surface_and_job_factories;
pub use command_surface_and_job_factories::*;

#[path = "✍️edit-command-work/🦀️.rs"]
mod edit_command_work;
pub use edit_command_work::*;

#[path = "📬️store-preparation-and-proofs/🦀️.rs"]
mod store_preparation_and_proofs;
pub use store_preparation_and_proofs::*;

#[path = "🧭️editor-and-manifest/🦀️.rs"]
mod editor_and_manifest;
pub use editor_and_manifest::*;

pub use document_panel::VCS_PLAY_BODY_ARTIFACT;
pub use editor::VCS_PLAY_BODY_EDITOR;
pub use history::VCS_PLAY_BODY_HISTORY;
pub use inspection_panel::VCS_PLAY_BODY_INSPECTION;

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub use semio_framework_plugin::ui_node_list;

//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
// 🌱️ `seed_vcs_demo_history` (test-only demo history seeding) now lives in the `🔖️UnitTests` region
// below — it must dispatch through `VcsArtifactApp`'s public surface (`dispatch_typed`/
// `handle_action`), not a raw `store::ArtifactStore`, since `ArtifactApp::seed(&mut ArtifactStore)`
// (this app's old direct-store-touch hook) no longer exists on the trait as of ticket
// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4 (`ArtifactApp::genesis() -> Vec<Self::Mutation>`
// replaced it, but `genesis` can only emit flat document mutations — it has no way to express
// `CommitCheckpoint`/`CreateAlternative`/`SwitchAlternative`, so it cannot reconstruct branching
// checkpoint history at construction time). Consequence: this demo's rich seeded history is reachable
// from tests (`context::app`/`app_with_registry` seed it explicitly) but no longer auto-populates a
// freshly constructed production instance the way `ArtifactApp::seed` used to — restoring that would
// need a framework-level hook `genesis` doesn't provide, which is out of this plugin's boundary
// (`🔌️plugin/🦀️.rs` is W1-owned).
//#endregion 🔖️DocumentHelpers

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts


//#endregion 🪢️TaxonomyMounts

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🎯️ This artifact's surface coordinate (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
/// contract §1) — `artifact_kind` matches the `"s.vcs.schema.artifact"` capability row's own
/// descriptor below (`s.vcs.vcs`), which is also this subset's `🧬️schema` id
/// (`#[artifact_schema(id = "s.vcs.vcs")]`) and the `🏅️standards/🔖️1/🪆️subsets/🔣️.json`
/// `"artifact"` field — not guessed. Lives at the ARTIFACT level (not under `editor`/`viewer`) so a
/// viewer file can read it without ever importing through the sibling editor module. `standard`/
/// `subset` mirror this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location, so the canonical
/// surface id is `s.vcs.vcs@1/*#editor` / `s.vcs.vcs@1/*#viewer` (contract §1 grammar).
pub const VCS_DIALECT: Dialect = Dialect { artifact_kind: "s.vcs.vcs", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{VCS_DIALECT};
use crate::{VCS_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the editor manifest by
/// `crate::editor::vcs::create_vcs_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: VCS_DIALECT.artifact_kind.into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("VCS Document", "VCS-Dokument"),
        source_format: VCS_DOCUMENT_SCHEMA.into(),
        component_kind: "vcs".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: VCS_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems3
