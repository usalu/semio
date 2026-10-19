//! 🖥️ Layout play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared canvas chrome in `🦀️canvas.rs`, headless compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `LayoutCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use semio_s_artifact_layout_layout_model::{Frame, GridSettings, LayoutBounds, Page, PageOverride};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_action_bus::RetainedToolWireInput;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{Generation, InteractiveStage, RevisionId};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, StepContext};
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactReservedToolJob, ArtifactSnapshotCloseLease, ArtifactToolCompletion};
use semio_framework_plugin::{ArtifactReservedJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, MediaClass, MediaForm, MediaType};
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;
use semio_s_artifact_layout_layout_engine_export::LayoutExportRequest;
use semio_s_artifact_layout_layout_engine_export::LayoutExportJob;
use semio_s_artifact_layout_layout_engine_export::LayoutExportKind;
use semio_s_artifact_layout_layout_engine_export::LayoutPublicationSlot;
use semio_s_artifact_layout_layout_engine_export::output_name;
use semio_s_artifact_layout_layout_engine_export::LayoutExportPublication;
use semio_s_artifact_layout_layout_engine_export::LayoutExportPublicationKind;
use semio_s_artifact_layout_layout_engine_export::LayoutExportTurn;
use semio_s_artifact_layout_layout_engine_export::sanitize_filename;
use semio_s_artifact_layout_layout_engine_export::bounded_id;
use semio_s_artifact_layout_layout_engine_export::LAYOUT_MEDIA_EXPORT_SCHEMA;
use semio_s_artifact_layout_layout_engine_export::LAYOUT_MEDIA_EXPORT_TOOL_ID;
use semio_s_artifact_layout_layout_engine_export::LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA;
use semio_s_artifact_layout_layout_engine_json::MAX_LAYOUT_EXPORT_DECODED_ITEMS;
use semio_s_artifact_layout_layout_engine_export::MAX_LAYOUT_EXPORT_OUTPUT_BYTES;
use semio_s_artifact_layout_layout_engine_export::MAX_LAYOUT_EXPORT_RAW_BYTES;
use semio_s_artifact_layout_layout_engine_export::LAYOUT_EXPORT_TOOL_IDS;
use semio_s_artifact_layout_layout_engine_export::LAYOUT_EXPORT_PAYLOAD_SCHEMA;
use semio_s_artifact_layout_layout_engine_export::MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES;
use semio_s_artifact_layout_layout_editor_commands::LAYOUT_GRANULARITY_ELEMENT;
use semio_s_artifact_layout_layout_editor_commands::LAYOUT_INTERACTION_ELEMENTS;
use semio_s_artifact_layout_layout_editor_commands::LAYOUT_PLAY_APP_ID;
use semio_s_artifact_layout_layout_editor_commands::LayoutInteractionSnapshot;
use semio_s_artifact_layout_layout_editor_commands::layout_select_action_args;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_layout_layout_editor_commands::modes::edit;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::{blueprint, preview, blueprint::config::from_snapshot, blueprint::config::addressed};
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transient::LayoutWindowTransient;
use semio_s_artifact_layout_layout_editor_panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, preflight as preflight_panel};
use semio_s_artifact_layout_layout_editor_panels::preflight::{run_layout_preflight, PreflightIssue};
use semio_s_artifact_layout_layout_editor_panels::terminology::layout_labels;
use semio_s_artifact_layout_layout_editor_panels::terminology::LayoutLabels;
use semio_s_artifact_layout_layout_mutations_content::change_data_fields::ChangeDataFields;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_framework::kernel::Effect;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::app::{ArtifactMediaExportJobRequest, ArtifactOwnedToolJobRequest, ArtifactReservedToolInput, ArtifactReservedToolJobRequest, ArtifactToolFactoryRegistry};
#[cfg(test)]
use semio_framework_plugin::App;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use semio_framework_plugin::ContextMenuSurfaceTarget;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
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
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
use semio_framework_plugin::WindowEngagementPossible;
use semio_framework_plugin::WindowEngagementStatus;
use semio_framework_plugin::CLEAR_SELECTION_ACTION_ID;
use semio_framework_plugin::INTERACTION_HOVER_ACTION_ID;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_plugin::{AppOperationContext};
#[cfg(test)] use serde_json::{json, Value};
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_BODY_BLUEPRINT;
#[cfg(test)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_SURFACE_BLUEPRINT;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::LAYOUT_PLAY_WINDOW_BLUEPRINT;
use semio_s_artifact_layout_layout_editor_panels::catalogue::LAYOUT_PLAY_BODY_CATALOGUE;
use semio_s_artifact_layout_layout_editor_panels::document::LAYOUT_PLAY_BODY_ARTIFACT;
use semio_s_artifact_layout_layout_editor_panels::inspection::LAYOUT_PLAY_BODY_INSPECTION;
use semio_s_artifact_layout_layout_editor_panels::preflight::LAYOUT_PLAY_BODY_PREFLIGHT;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::preview::LAYOUT_PLAY_BODY_PREVIEW;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::preview::LAYOUT_PLAY_SURFACE_PREVIEW;
#[cfg(test)] use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW;

#[path = "📋️ui-values-and-context-menus/🦀️.rs"]
mod ui_values_and_context_menus;
pub use ui_values_and_context_menus::{ui_value_bool, ui_value_list, ui_value_number};
use ui_values_and_context_menus::{layout_context_menu_items, layout_internal_action};

#[path = "🧵️retained-command-factories/🦀️.rs"]
mod retained_command_factories;
use retained_command_factories::{LAYOUT_RETAINED_RAW_BYTES, LAYOUT_RETAINED_TOOL_IDS, LAYOUT_RETAINED_WORK_ITEMS, LayoutExportProofs, LayoutRetainedCommandJobFactory, LayoutRetainedProofs, LayoutWindowWork};

#[path = "🎞️reserved-import-job/🦀️.rs"]
mod reserved_import_job;
use reserved_import_job::{LAYOUT_IMPORT_TOOL_ID, LayoutImportJob, layout_build_export_tool_job};

#[path = "🪧️window-engagement-and-labels/🦀️.rs"]
mod window_engagement_and_labels;
pub use window_engagement_and_labels::layout_entity_label;
use window_engagement_and_labels::layout_window_engagement;

#[path = "🕹️layout-play-app/🦀️.rs"]
mod layout_play_app;
pub use layout_play_app::LayoutPlayApp;

#[path = "🧱️create-layout-app/🦀️.rs"]
mod layout_app_manifest;
pub use layout_app_manifest::create_layout_app;

#[path = "📤️export-tool-job/🦀️.rs"]
mod export_tool_job;
pub use export_tool_job::{LayoutExportJobFactory, LayoutExportToolJob, LayoutExportToolPayload};

#[path = "🎥️media-export-job-and-factories/🦀️.rs"]
mod media_export_job_and_factories;
pub use media_export_job_and_factories::{LayoutMediaExportJob, LayoutMediaExportJobFactory};

//#region 🔖️Interaction

//#endregion 🔖️Interaction

//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `LayoutPlayApp::Command` — the SOLE dispatch surface for layout's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different vocabularies.
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum LayoutCommand for LayoutSnapshot, LayoutMutation, NoConfig, NoConfigMutation {
        "setActivePage" as "active-page" => set_active_page::SetActivePage,
        "focusPreflightIssue" as "focus-preflight-issue" => focus_preflight_issue::FocusPreflightIssue,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasDragOver" as "canvas-drag-over" => canvas_drag_over::CanvasDragOver,
        "canvasDragLeave" as "canvas-drag-leave" => canvas_drag_leave::CanvasDragLeave,
        "setCamera" as "camera" => set_camera::SetCamera,
        "addFrame" as "add-frame" => add_frame::AddFrame,
        "addPage" as "add-page" => add_page::AddPage,
        "patchPage" as "patch-page" => patch_page::PatchPage,
        "patchFrame" as "patch-frame" => patch_frame::PatchFrame,
        "canvasDrop" as "canvas-drop" => canvas_drop::CanvasDrop,
        "exportPng" as "export-png" => export_png::ExportPng,
        "exportSvg" as "export-svg" => export_svg::ExportSvg,
        "exportPdf" as "export-pdf" => export_pdf::ExportPdf,
        "exportPackage" as "export-package" => export_package::ExportPackage,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "translateSelection" as "translate-selection" => gumball::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "patchDocument" as "patch-document" => patch_document::PatchDocument,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use semio_s_artifact_layout_layout_editor_commands::add_frame;
use semio_s_artifact_layout_layout_editor_commands::add_page;
use semio_s_artifact_layout_layout_editor_commands::canvas_drag_leave;
use semio_s_artifact_layout_layout_editor_commands::canvas_drag_over;
use semio_s_artifact_layout_layout_editor_commands::canvas_drop;
use semio_s_artifact_layout_layout_editor_commands::canvas_pointer_down;
use semio_s_artifact_layout_layout_editor_commands::canvas_pointer_move;
use semio_s_artifact_layout_layout_editor_commands::canvas_pointer_up;
use semio_s_artifact_layout_layout_editor_commands::delete_selection;
use semio_s_artifact_layout_layout_editor_commands::engagement_input;
use semio_s_artifact_layout_layout_editor_commands::engagement_submit;
use semio_s_artifact_layout_layout_editor_commands::export_package;
use semio_s_artifact_layout_layout_editor_commands::export_pdf;
use semio_s_artifact_layout_layout_editor_commands::export_png;
use semio_s_artifact_layout_layout_editor_commands::export_svg;
use semio_s_artifact_layout_layout_editor_panels::focus_preflight_issue;
use semio_s_artifact_layout_layout_editor_commands::patch_document;
use semio_s_artifact_layout_layout_editor_commands::patch_frame;
use semio_s_artifact_layout_layout_editor_commands::patch_page;
use semio_s_artifact_layout_layout_editor_commands::rotate_selection;
use semio_s_artifact_layout_layout_editor_commands::scale_selection;
use semio_s_artifact_layout_layout_editor_commands::set_active_page;
use semio_s_artifact_layout_layout_editor_commands::set_camera;
use semio_s_artifact_layout_layout_editor_commands::gumball;
//#endregion 🔖️Commands

//#region 🔖️ActionBridge
#[path = "🔀️args-bridge/🦀️.rs"]
mod args_bridge;
//#endregion 🔖️ActionBridge

//#region 🔖️LayoutPlayApp
/// 📦️ The built-in text fonts the layout package declares as package assets, for the plugin assembly's `PluginBuilder::asset`.
pub use semio_s_artifact_layout_layout_engine::scene::text_fonts::text_font_asset_declarations;

//#endregion 🔖️LayoutPlayApp

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit_tests;
#[cfg(test)]
#[path = "🧪️tests/🧪️transform-tool-transactions/🦀️.rs"]
mod transform_tool_transaction_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
#[path = "🌉️wasm/🦀️.rs"]
pub mod wasm;
//#endregion 🪢️TaxonomyMounts
