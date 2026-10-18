//! 🎞️ Animate editor — the `ArtifactEditor` impl (dispatch-only, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET), the aggregated command enum and the manifest
//! stitch. B1: the pure-trait pivot — `AnimatePresentationPlayApp` is a unit struct; every former
//! `AnimatePresentationPlayRuntime` field (selection, engagement draft) now lives in
//! `crate::editor::animate::config::PresentationConfig`, written via `PresentationConfigMutation`s (real
//! `backwards`, no ad hoc `InverseAction`); every action dispatches through the single typed
//! `PresentationCommand` channel via `ArtifactEditor::handle`. MUST NOT be imported from the sibling
//! `👁️viewer` module (`policyViewerPurityBreaches`).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/🖊️main/🪟️windows/🖼️tile-editor`, panel trees in `📌️panels/*`, labels in
//! `🦀️terminology.rs`, view state in `🦀️config.rs`, pure document helpers in
//! `crate::schema`, and stateful behaviour (the Manim-class animation core + the
//! headless video renderer, ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) in this app's
//! own `⚙️engine`. This file is a routing table: `handle` → `PresentationCommand::dispatch`, `render` →
//! body-key → node, `🔖️Io`/`🔌️Registration` regions below, and a `🔖️Manifest` region that calls one
//! `definition()` per node.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use crate::editor::animate::commands::{
    add_tile, canvas_pointer_down, clear_tiles, copy_prompt, delete_selection, delete_tile, engagement_input, engagement_submit, export_video_from_deck, no_operation, patch_tile_crops, rename_tiles, reset_grid, seed_grid, set_active_example,
    set_frame, set_source,
};
use crate::editor::animate::config::{PresentationConfig, PresentationConfigMutation};
use crate::editor::animate::modes::main;
use crate::editor::animate::modes::main::windows::tile_editor;
use crate::editor::animate::panels::{artifact, catalogue, inspection};
use crate::editor::animate::terminology::animate_presentation_labels;
use crate::editor::animate::support::*;
use crate::mutations::create_tile::CreateTile;
use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
use crate::{default_presentation_snapshot, FigureTileDraft, PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, StepContext};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::app::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion};
use semio_framework_plugin::ArtifactReservedJob;
// 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
// `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
// `.workflow(...)` on this builder (see `🔖️Manifest` below for what got dropped, not silently).
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ComponentTree;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants
pub use artifact::PRESENTATION_PLAY_BODY_ARTIFACT;
pub use catalogue::PRESENTATION_PLAY_BODY_CATALOGUE;
pub use inspection::PRESENTATION_PLAY_BODY_DETAILS;
pub use tile_editor::PRESENTATION_PLAY_BODY_MAIN;
//#endregion 🔖️Constants

//#region 🔖️Interaction
//#endregion 🔖️Interaction

#[path = "🔌️presentation-io-and-frame-helpers/🦀️.rs"]
mod presentation_io_and_frame_helpers;
pub use presentation_io_and_frame_helpers::{
    next_frame_tile_crop, next_frame_tile_id, presentation_io,
};
use presentation_io_and_frame_helpers::frame_media_name;

#[path = "🧵️retained-command-tool-jobs/🦀️.rs"]
mod retained_command_tool_jobs;
pub use retained_command_tool_jobs::{
    ANIMATE_PRESENTATION_CONFIG_STEP_BYTES, ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES,
    ANIMATE_PRESENTATION_RETAINED_PUBLICATION_CONTRACTS, ANIMATE_PRESENTATION_RETAINED_TOOL_IDS,
};
use retained_command_tool_jobs::{
    animate_presentation_retained_contract, animate_presentation_retained_extent,
    animate_presentation_retained_reduce, AnimatePresentationRetainedCommandJobFactory,
    ANIMATE_PRESENTATION_CONFIG_BASE_BYTES, ANIMATE_PRESENTATION_RETAINED_RAW_BYTES,
    ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS,
};

#[path = "📬️config-store-preparation/🦀️.rs"]
mod config_store_preparation;
pub use config_store_preparation::{
    AnimatePresentationConfigByteCounter, AnimatePresentationConfigPreparation,
    AnimatePresentationConfigPreparationFactory,
};

#[path = "📥️reserved-presentation-import/🦀️.rs"]
mod reserved_presentation_import;
use reserved_presentation_import::{PresentationImportJob, PRESENTATION_IMPORT_TOOL_ID};

#[path = "🎛️play-app-editor/🦀️.rs"]
mod play_app_editor;
pub use play_app_editor::AnimatePresentationPlayApp;

#[path = "🧾️manifest-and-app-constructor/🦀️.rs"]
mod manifest_and_app_constructor;
pub use manifest_and_app_constructor::create_animate_presentation_app;

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `AnimatePresentationPlayApp::Command` — the SOLE dispatch surface for animate presentation's own
    /// behavior, assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest
    /// action id (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire
    /// keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies:
    /// `"resetGrid" as "reset-grid"` is the row that proves it. The app owner is carried by the
    /// qualified command address rather than repeated in this local id. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum PresentationCommand for PresentationSnapshot, PresentationMutation, PresentationConfig, PresentationConfigMutation, ctx = PresentationDispatchCtx {
        "seedGrid" as "seed-grid" => seed_grid::SeedGrid,
        "addTile" as "add-tile" => add_tile::AddTile,
        "deleteTile" as "delete-tile" => delete_tile::DeleteTile,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "renameTiles" as "rename-tiles" => rename_tiles::RenameTiles,
        "patchTileCrops" as "patch-tile-crops" => patch_tile_crops::PatchTileCrops,
        "setSource" as "set-source" => set_source::SetSource,
        "setFrame" as "set-frame" => set_frame::SetFrame,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "clearTiles" as "clear-tiles" => clear_tiles::ClearTiles,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "resetGrid" as "reset-grid" => reset_grid::ResetGrid,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "noMutation" as "no-op" => no_operation::NoOperation,
        "copyPrompt" as "copy-prompt" => copy_prompt::CopyPrompt,
        "exportVideoFromDeck" as "export-video-from-deck" => export_video_from_deck::ExportVideoFromDeck,
    }
}
//#endregion 🔖️Commands

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts

//#endregion 🪢️TaxonomyMounts

//#region 🧩️ArtifactItems12
pub mod artifact_items_12 {
use crate::{PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::presentation::create_animate_presentation_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: PRESENTATION_DOCUMENT_SCHEMA.into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Animate Presentation", "Animierte Präsentation"),
        source_format: PRESENTATION_DOCUMENT_SCHEMA.into(),
        component_kind: "panel".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Presentation, form: MediaForm::Deck },
        schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.pdf".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.pdf".into()],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems12
