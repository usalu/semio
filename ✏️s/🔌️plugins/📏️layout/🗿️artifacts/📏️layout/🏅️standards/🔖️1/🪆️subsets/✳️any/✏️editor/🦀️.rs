//! 🖥️ Layout play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared canvas chrome in `🦀️canvas.rs`, headless compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `LayoutCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use semio_s_artifact_layout_layout_model::{Frame, GridSettings, LayoutBounds, Page, PageOverride};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework::action_bus::RetainedToolWireInput;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::{Generation, InteractiveStage, RevisionId};
use semio_framework_job_worker_authority::{BatchDriveConfig,BatchJobParams};
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, Operation, StepContext};
use semio_framework_plugin::app::{ArtifactDownloadOutput, ArtifactMediaExportCompletion, ArtifactMediaExportCredit, ArtifactMediaExportResult, ArtifactOutputChunks, ArtifactReservedToolJob, ArtifactSnapshotCloseLease, ArtifactToolCompletion};
use semio_framework_plugin::{ArtifactReservedJob, ArtifactToolPublicationContract, ArtifactToolPublicationLane, EditorApp, EphemeralEmit, Fault, MediaClass, MediaForm, MediaType};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioRgba;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawNode, PathSegment};
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
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::{blueprint, preview};
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::transient::LayoutWindowTransient;
use semio_s_artifact_layout_layout_editor_panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, preflight as preflight_panel};
use semio_s_artifact_layout_layout_editor_panels::preflight::{run_layout_preflight, PreflightIssue};
use semio_s_artifact_layout_layout_model::Frame;
use semio_s_artifact_layout_layout_editor_panels::terminology::layout_labels;
use semio_s_artifact_layout_layout_editor_panels::terminology::LayoutLabels;
use semio_s_artifact_layout_layout_mutations_content::change_data_fields::ChangeDataFields;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework::kernel::Effect;
use {semio_framework_artifact_reference::Dialect,semio_framework::InteractiveJobClassification,semio_framework::ToolExecutionContract,semio_framework::ToolFactoryKey,semio_framework::ToolJobFactoryError};
use semio_framework_job::{InteractiveJob, StepContext};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::app::{ArtifactMediaExportJobRequest, ArtifactOwnedToolJobRequest, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry};
use semio_framework_plugin::ArtifactReservedJob;
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
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
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
use semio_framework_plugin::{AppOperationContext, ArtifactToolPublicationContract, ArtifactToolPublicationLane};
#[cfg(test)] use serde_json::{json, Value};
use std::collections::HashMap;
use semio_framework_2d::compute::EngineHandles;


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


/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}


//#region 🔖️Interaction


//#endregion 🔖️Interaction

//#region 🔖️ContextMenu
fn layout_element_ids(surface: Option<&ContextMenuSurfaceTarget>, fallback: &[String]) -> Vec<String> {
    let groups = surface.map_or(&[][..], |target| target.selection.as_slice());
    let mut ids: Vec<String> = groups.iter().filter(|group| group.domain == LAYOUT_GRANULARITY_ELEMENT).flat_map(|group| group.ids.iter().cloned()).collect();
    if ids.is_empty() {
        ids = fallback.to_vec();
    }
    ids
}

fn layout_preflight_issue_for_row(doc: &LayoutSnapshot, labels: &LayoutLabels, row_id: &str) -> Option<PreflightIssue> {
    run_layout_preflight(doc, labels).into_iter().find(|issue| {
        let object = issue.object_id.clone().unwrap_or_else(|| issue.message.clone());
        format!("layout-preflight.{}.{}", issue.code, object) == row_id
    })
}

fn layout_frame_kind<'a>(doc: &'a LayoutSnapshot, frame_id: &str) -> Option<&'a str> {
    doc.pages.iter().flat_map(|page| page.frames.iter()).find(|frame| frame.id() == frame_id).map(|frame| frame.kind_str())
}

fn layout_link_referencing_frame_ids(doc: &LayoutSnapshot, link_id: &str) -> Vec<String> {
    doc.pages
        .iter()
        .flat_map(|page| page.frames.iter())
        .filter(|frame| matches!(frame, Frame::Image { link_id: id, .. } if id == link_id))
        .map(|frame| frame.id().to_string())
        .collect()
}

fn layout_authoring_surface(surface: Option<&ContextMenuSurfaceTarget>) -> bool {
    surface.map_or(true, |target| target.surface_id != LAYOUT_PLAY_SURFACE_PREVIEW)
}

fn layout_context_menu_item(id: &str, label: &str, icon: &str, action: &str, args: Option<DslValue>, destructive: bool, disabled: bool) -> ContextMenuItemSpec {
    ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args,
        destructive: destructive.then_some(true),
        disabled: disabled.then_some(true),
        ..Default::default()
    }
}

fn layout_add_frame_items<'a>(menu: semio_framework_plugin::Menu<'a>, labels: &'a LayoutLabels, kind: &'a str) -> semio_framework_plugin::Menu<'a> {
    let (id, label, icon) = match kind {
        "text" => ("add-text-frame", labels.kind_text.as_str(), "type"),
        "image" => ("add-image-frame", labels.kind_image.as_str(), "image"),
        _ => ("add-rect-frame", labels.kind_rect.as_str(), "square"),
    };
    menu.item(layout_context_menu_item(id, label, icon, "addFrame", Some(semio_framework::dsl_value!({ "kind": kind })), false, false))
}

/// 🖱️ On-demand layout context menu — canvas, document tree, and preflight surfaces each carry
/// different hit/selection shapes; `organize_context_menu` runs automatically at the host funnel.
fn layout_context_menu_items(
    registry: &AppActionRegistry,
    doc: &LayoutSnapshot,
    labels: &LayoutLabels,
    view_state: &semio_framework_plugin::ViewModel,
    surface: Option<&ContextMenuSurfaceTarget>,
    fallback_selected: &[String],
) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, Menu, SelectionKind};
    let is_de = view_state.locale == semio_framework_ui_locale::Locale::De;

    let hits = surface.map_or(&[][..], |target| target.hits.as_slice());
    let authoring = layout_authoring_surface(surface);
    let preview = surface.is_some_and(|target| target.surface_id == LAYOUT_PLAY_SURFACE_PREVIEW);

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-document.page.")) {
        if let Some(page_id) = hit.id.strip_prefix("layout-document.page.") {
            return Menu::of(registry, view_state).item(layout_context_menu_item("set-active-page", labels.active_page.as_str(), "file", "setActivePage", Some(semio_framework::dsl_value!({ "pageId": page_id })), false, false)).build();
        }
    }

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-document.link.")) {
        if let Some(link_id) = hit.id.strip_prefix("layout-document.link.") {
            let ids = layout_link_referencing_frame_ids(doc, link_id);
            let disabled = ids.is_empty();
            return Menu::of(registry, view_state)
                .item(layout_context_menu_item(
                    "select-link-frames",
                    if is_de { "Verknüpfte Rahmen auswählen" } else { "Select linked frames" },
                    "image",
                    INTERACTION_SELECT_ACTION_ID,
                    Some(layout_select_action_args(&ids, "replace")),
                    false,
                    disabled,
                ))
                .build();
        }
    }

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-preflight.")) {
        if let Some(issue) = layout_preflight_issue_for_row(doc, labels, &hit.id) {
            let issue_value = semio_framework::dsl_value!({
                "severity": issue.severity,
                "code": issue.code,
                "message": issue.message,
                "objectId": issue.object_id,
                "pageId": issue.page_id,
            });
            return Menu::of(registry, view_state).item(layout_context_menu_item("focus-preflight-issue", labels.preflight.as_str(), "alert-triangle", "focusPreflightIssue", Some(semio_framework::dsl_value!({ "issue": issue_value })), false, false)).build();
        }
    }

    let selected = layout_element_ids(surface, fallback_selected);
    let element_hit = hits.iter().find(|hit| hit.domain == LAYOUT_GRANULARITY_ELEMENT);

    if selected.is_empty() {
        if preview {
            return Menu::of(registry, view_state).action("selectAll").build();
        }
        let mut menu = Menu::of(registry, view_state);
        if let Some(hit) = element_hit {
            menu = menu.item(layout_context_menu_item(
                "select-hit",
                if is_de { "Auswählen" } else { "Select" },
                "mouse-pointer",
                INTERACTION_SELECT_ACTION_ID,
                Some(layout_select_action_args(std::slice::from_ref(&hit.id), "replace")),
                false,
                false,
            ));
        }
        menu = menu.group("create", |group| layout_add_frame_items(layout_add_frame_items(layout_add_frame_items(group, labels, "rect"), labels, "text"), labels, "image"));
        return menu.action("addPage").action("selectAll").action("paste").destructive("clearSelection").disabled_because(&semio_framework_plugin::nothing_selected()).build();
    }

    let phrase = selection_count_phrase(view_state.locale, &[(selected.len(), SelectionKind::Frame)]).unwrap_or_default();
    let mut menu = Menu::of(registry, view_state);
    if let Some(hit) = element_hit.filter(|hit| !selected.contains(&hit.id)) {
        menu = menu.item(layout_context_menu_item(
            "select-hit",
            if is_de { "Auswählen" } else { "Select" },
            "mouse-pointer",
            INTERACTION_SELECT_ACTION_ID,
            Some(layout_select_action_args(std::slice::from_ref(&hit.id), "replace")),
            false,
            false,
        ));
    }
    if selected.len() == 1 {
        if let Some(kind) = layout_frame_kind(doc, &selected[0]) {
            menu = layout_add_frame_items(menu.group("create", |group| group), labels, kind);
        }
    }
    menu = menu.action("copy").action("cut").action("paste");
    if authoring {
        menu = menu.item(layout_context_menu_item(
            "delete-selection",
            &format!("{} ({phrase})", if is_de { "Löschen" } else { "Delete" }),
            "trash",
            "deleteSelection",
            None,
            true,
            false,
        ));
    }
    menu.destructive("clearSelection").build()
}
//#endregion 🔖️ContextMenu

/// 🙈️ An internal (non-palette) action declaration — the pointer/inspector/DnD/engagement-bound
/// vocabulary dispatched by the canvas and panels, never surfaced as a standalone palette command.
fn layout_internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}
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
/// 🎯️ Folds the host's `{action, args}` vocabulary (camelCase keys, JSON floats, control `value`s) into
/// the snake_case `FromValue` payloads of `🎮️commands/*`, one arm per `LayoutCommand` row.
//#endregion 🔖️ActionBridge

//#region 🧵️RetainedCommands
/// 🧵️ Every shell-reachable verb is a bounded first-step tool (ticket 26/09/16/LAYOUT-PLUGIN-END-TO-END):
/// the framework refuses UI dispatch of any command not classified `Migrated`, so the six content verbs
/// that stayed `BatchOnlyPendingRewrite` (`addFrame`/`addPage`/`patchPage`/`patchFrame` and the pointer
/// down/move gestures) were dead in the running app. Exports keep their own resumable factory.
const LAYOUT_RETAINED_TOOL_IDS: &[&str] = &[
    "setActivePage", "focusPreflightIssue", "engagementInput", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDragOver", "canvasDragLeave", "setCamera", "addFrame", "addPage", "patchPage", "patchFrame", "deleteSelection", "engagementSubmit", "canvasDrop", "translateSelection", "rotateSelection", "scaleSelection", "patchDocument",
];
const LAYOUT_RETAINED_PAYLOAD_SCHEMA: &str = "layout.layout.tool-command.v1";
const LAYOUT_RETAINED_RAW_BYTES: usize = 8_192;
const LAYOUT_RETAINED_WORK_ITEMS: usize = 1;

fn layout_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(LAYOUT_RETAINED_RAW_BYTES, 64, 1, 16_384, 7_500)
}

struct LayoutWindowWork {
    tool_id: &'static str,
    completed: bool,
}

impl LayoutWindowWork {
    fn new(tool_id: &'static str) -> Self { Self { tool_id, completed: false } }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<EditorApp<LayoutPlayApp>> for LayoutWindowWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LayoutPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(
        &self,
        command: &LayoutCommand,
        _snapshot: &LayoutSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<LayoutPlayApp>>>,
    ) -> Option<usize> {
        let _ = context;
        (!self.completed && command.command_id() == self.tool_id).then_some(1)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LayoutPlayApp>>,
    _cx: &mut StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<EditorApp<LayoutPlayApp>>, Fault> {
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || input.command.command_id() != self.tool_id {
            return Err(Fault::from("layout-window-work-terminal"));
        }
        // 🪟️ Window-lane verbs need the addressed Blueprint window; the document-lane verbs
        // (`addFrame`/`patchPage`/…) and the host-only pointer gestures dispatch without one.
        let window_config_snapshot = input.context.and_then(|context| context.window_config.as_ref());
        let window = input.context.and_then(|context| context.view_state.as_ref());
        let addressed_window = || window.ok_or_else(|| Fault::from("layout-window-view-required"));
        let mut config = blueprint::config::from_snapshot(window_config_snapshot);
        let mut transient = blueprint::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let config_view = ConfigView { snapshot: input.config, window: window_config_snapshot };
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let selected = input.interaction.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.as_slice()).unwrap_or(&[]);
        let gumball_verb = match input.command {
            LayoutCommand::TranslateSelection(payload) => Some(payload.verb()),
            LayoutCommand::RotateSelection(payload) => Some(payload.verb()),
            LayoutCommand::ScaleSelection(payload) => Some(payload.verb()),
            _ => None,
        };
        let mut transform_tool = None;
        let mut emit = match (input.command, gumball_verb) {
            (_, Some(verb)) => {
                let notice = window.map(|view| layout_labels(view).frames_locked.as_str());
                let dispatch = gumball::transform(verb, &doc, &config_view, selected, transient.transform_tool.as_deref(), notice);
                transform_tool = dispatch.tool;
                dispatch.emit
            }
            (LayoutCommand::DeleteSelection(_), None) => delete_selection::apply_frame_ids(&doc, selected)?,
            _ => input.command.dispatch(&doc, &config_view)?,
        };
        let mut window_config = None;
        let mut window_transient = None;
        match input.command {
            LayoutCommand::SetActivePage(payload) => {
                config.active_page_id = payload.page_id.clone();
                window_config = Some(blueprint::config::addressed(addressed_window()?, config)?);
            }
            LayoutCommand::FocusPreflightIssue(payload) => {
                if let Some(page_id) = &payload.page_id {
                    config.active_page_id = page_id.clone();
                    window_config = Some(blueprint::config::addressed(addressed_window()?, config)?);
                }
            }
            LayoutCommand::SetCamera(payload) => {
                config.camera = payload.camera.clone();
                window_config = Some(blueprint::config::addressed(addressed_window()?, config)?);
            }
            LayoutCommand::EngagementInput(payload) => {
                transient.engagement_input = payload.value.clone();
                window_transient = Some(blueprint::transient::addressed(addressed_window()?, transient)?);
            }
            LayoutCommand::AddPage(_) => {
                if let Some(view) = window {
                    config.active_page_id = format!("page-{}", input.snapshot.pages.len() + 1);
                    window_config = Some(blueprint::config::addressed(view, config)?);
                }
            }
            LayoutCommand::CanvasDragOver(payload) => {
                let view = addressed_window()?;
                if view.window_instances.iter().any(|window| view.window_id.as_deref() == Some(&window.id) && window.window_kind_id == LAYOUT_PLAY_WINDOW_BLUEPRINT) {
                    let camera = semio_framework_canvas::camera::Camera { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom.max(0.0001) };
                    let viewport = semio_framework_canvas::camera::Viewport { width: payload.width.max(1.0) as u32, height: payload.height.max(1.0) as u32, dpr: 1.0 };
                    let world = semio_framework_canvas::camera::screen_to_world(&camera, &viewport, semio_framework_canvas::Point::new(payload.x, payload.y));
                    transient.drop_preview = semio_s_artifact_layout_layout_model::LayoutDropPreviewState { kind: payload.kind.clone(), x: world.x, y: world.y };
                    window_transient = Some(blueprint::transient::addressed(view, transient)?);
                }
            }
            LayoutCommand::CanvasDragLeave(_) | LayoutCommand::CanvasDrop(_) => {
                let view = addressed_window()?;
                transient.drop_preview = semio_s_artifact_layout_layout_model::LayoutDropPreviewState::default();
                window_transient = Some(blueprint::transient::addressed(view, transient)?);
                if let LayoutCommand::CanvasDrop(payload) = input.command {
                    if payload.kind == "page" {
                        let page_id = format!("page-{}", input.snapshot.pages.len() + 1);
                        config.active_page_id = page_id;
                        emit.window_config_mutations.push(blueprint::config::addressed(view, config)?);
                    }
                }
            }
            LayoutCommand::TranslateSelection(_) | LayoutCommand::RotateSelection(_) | LayoutCommand::ScaleSelection(_) => {
                if let Some(tool) = transform_tool {
                    transient.transform_tool = tool;
                    window_transient = Some(blueprint::transient::addressed(addressed_window()?, transient)?);
                }
            }
            LayoutCommand::AddFrame(_) | LayoutCommand::PatchPage(_) | LayoutCommand::PatchFrame(_) | LayoutCommand::PatchDocument(_) | LayoutCommand::DeleteSelection(_) | LayoutCommand::CanvasPointerDown(_) | LayoutCommand::CanvasPointerMove(_) | LayoutCommand::CanvasPointerUp(_) | LayoutCommand::EngagementSubmit(_) => {}
            _ => return Err(Fault::from("layout-window-work-route-rejected")),
        }
        if let Some(mutation) = window_config { emit.window_config_mutations.push(mutation); }
        self.completed = true;
        match window_transient {
            Some(mutation) => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![mutation] },
            }),
            None => Ok(ArtifactCommandWorkStep::Complete(emit)),
        }
    }
}

struct LayoutRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl LayoutRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: LAYOUT_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for LayoutRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<LayoutPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<LayoutPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        layout_retained_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > LAYOUT_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("LAYOUT retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutRetainedCommandJobFactory {
    type Owner = EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LAYOUT_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setActivePage", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "focusPreflightIssue", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasDragOver", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "canvasDragLeave", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "addFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addPage", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig] },
        ArtifactToolPublicationContract { tool_id: "patchPage", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "canvasDrop", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowConfig, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] },
        ArtifactToolPublicationContract { tool_id: "patchDocument", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
}
//#region 🧾️ProofCatalogs
struct LayoutRetainedProofs;
impl LayoutRetainedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<LayoutPlayApp>,
        owner_file: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.layout.layout@1/*#editor",
        artifact_schema: "layout.layout",
        factory: "LayoutRetainedCommandJobFactory",
        factory_type: LayoutRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
        tools: ["setActivePage", "focusPreflightIssue", "engagementInput", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDragOver", "canvasDragLeave", "setCamera", "addFrame", "addPage", "patchPage", "patchFrame", "deleteSelection", "engagementSubmit", "canvasDrop", "translateSelection", "rotateSelection", "scaleSelection", "patchDocument"]
    }
}

struct LayoutExportProofs;
impl LayoutExportProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<LayoutPlayApp>,
        owner_file: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.layout.layout@1/*#editor",
        artifact_schema: "layout.layout",
        factory: "LayoutExportJobFactory",
        factory_type: LayoutExportJobFactory,
        tools: {
            "exportPng" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportSvg" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportPdf" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
            "exportPackage" => ToolExecutionContract::resumable(4_096, 131_072, 1, 33_554_432, 2_000, 64, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs
//#endregion 🧵️RetainedCommands

//#region 🎞️ReservedImport
/// 🎞️ The framework registers the reserved `import-media` factory for every app but never a concrete
/// job, so `VcsArtifactApp::import_media` fails closed with `interactive-job.missing-reserved-builder`
/// until the app hands one back from [`ArtifactEditor::build_reserved_tool_job`]. Layout's one inbound
/// port is `fields:in`.
const LAYOUT_IMPORT_TOOL_ID: &str = "import-media";
const LAYOUT_IMPORT_PORT: &str = "fields:in";

/// 🎞️ Layout's ONE concrete resumable importer. Two bounded steps: decode the `fields:in`
/// `form.dictionary` through [`LayoutPlayApp::import_media`] — the single decoding authority, shared
/// with every non-interactive caller — then publish its mutations through the completion authority.
struct LayoutImportJob {
    port: Option<String>,
    media: Option<Media>,
    snapshot: Option<std::sync::Arc<LayoutSnapshot>>,
    history: Option<std::sync::Arc<semio_framework_plugin::HistoryView>>,
    mutations: Option<Vec<LayoutMutation>>,
    active: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
    checkpoint_publication: semio_framework_job_publication::RetainedJobPublication,
    checkpoint_delivered: bool,
    fault_publication: semio_framework_job_publication::RetainedJobPublication,
    fault_detail: Option<Vec<u8>>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<LayoutPlayApp>>>,
}

type LayoutJobTurn<'a> = Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError>;

impl LayoutImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>, port: String, media: Media) -> Self {
        Self {
            port: Some(port),
            media: Some(media),
            snapshot: Some(request.snapshot),
            history: Some(request.history),
            mutations: None,
            active: None,
            checkpoint_publication: semio_framework_job_publication::RetainedJobPublication::new(),
            checkpoint_delivered: false,
            fault_publication: semio_framework_job_publication::RetainedJobPublication::new(),
            fault_detail: None,
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
        }
    }

    fn fault<'a>(&'a mut self, cx: &mut StepContext<'_>, detail: &str) -> LayoutJobTurn<'a> {
        if self.fault_detail.is_none() {
            let bytes = detail.as_bytes();
            self.fault_detail = Some(bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)].to_vec());
        }
        let bytes = self.fault_detail.as_deref().unwrap_or_default();
        self.fault_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Fault, bytes, cx)
    }

    fn decode(&mut self) -> Result<(), &'static str> {
        if self.port.as_deref() != Some(LAYOUT_IMPORT_PORT) {
            return Err("layout import only implements fields:in");
        }
        let decoded = {
            let (Some(media), Some(snapshot), Some(history)) = (self.media.as_ref(), self.snapshot.as_ref(), self.history.as_ref()) else {
                return Err("layout import lost its media, snapshot or history authority");
            };
            let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
            LayoutPlayApp::import_media(LAYOUT_IMPORT_PORT, media, &doc)
        };
        match decoded {
            Ok(emit) => {
                self.mutations = Some(emit.artifact_mutations);
                self.decoded = true;
                Ok(())
            }
            Err(_) => Err("layout import media decode refused"),
        }
    }

    fn nested(mut demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        demand.depth = demand.depth.checked_add(1).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "layout import close depth overflow"))?;
        Ok(demand)
    }

    fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        if let Some(active) = self.active.as_ref() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_box_demands(active, body)?);
        }
        if let Some(rejected) = self.pending_completion_rejection.as_ref() {
            if let Ok(emit) = rejected.emit.as_ref() {
                if let Some(demand) = emit.child_close_demands(body)? {
                    return Ok(demand);
                }
            }
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if self.mutations.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.mutations)?);
        }
        if self.media.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.media)?);
        }
        if self.port.is_some() {
            return Self::nested(store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.port)?);
        }
        if self.history.is_some() || self.completion.is_some() {
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        if self.snapshot.is_some() {
            return Ok(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::shared_retirement_birth_bytes::<LayoutSnapshot>(), depth: 2, ..Default::default() });
        }
        if !self.fault_publication.terminal_is_empty() {
            return Self::nested(self.fault_publication.retirement_demands()?);
        }
        if !self.checkpoint_publication.terminal_is_empty() {
            return Self::nested(self.checkpoint_publication.retirement_demands()?);
        }
        Ok(Default::default())
    }

    fn close_original(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let unit = || Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let demand = self.close_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(grant).map_err(|fault| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, fault.message))? {
                    return Ok(RetainedCloneStep::Progress(step.progress().unwrap_or_default()));
                }
            }
            self.pending_completion_rejection = None;
            return unit();
        }
        if self.mutations.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.mutations, &mut self.active, child);
        }
        if self.media.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.media, &mut self.active, child);
        }
        if self.port.is_some() {
            return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.port, &mut self.active, child);
        }
        if self.history.take().is_some() || self.completion.take().is_some() {
            return unit();
        }
        if let Some(snapshot) = self.snapshot.take() {
            return match semio_framework_value::admit_shared_retirement(snapshot, child, true) {
                Ok((owner, progress)) => {
                    self.active = Some(owner);
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, snapshot)) => {
                    self.snapshot = Some(snapshot);
                    Err(error)
                }
            };
        }
        if !self.fault_publication.terminal_is_empty() {
            return self.fault_publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if !self.checkpoint_publication.terminal_is_empty() {
            return self.checkpoint_publication.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl InteractiveJob for LayoutImportJob {
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> LayoutJobTurn<'a> {
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx);
        }
        if self.fault_detail.is_some() {
            return self.fault(cx, "");
        }
        if self.pending_completion_rejection.is_some() {
            return self.fault(cx, "layout import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("layout-import-decode");
            if let Err(detail) = self.decode() {
                return self.fault(cx, detail);
            }
            cx.consume_fuel(1);
        }
        if !self.checkpoint_delivered {
            let outcome = self.checkpoint_publication.advance_from_source(semio_framework_job_publication::JobPublicationKind::Checkpoint { applied_progress: 1 }, &[1], cx)?;
            if outcome.is_some() {
                self.checkpoint_delivered = true;
            }
            return Ok(outcome);
        }
        cx.set_stage("layout-import-publish");
        if !self.completed {
            let mutations = self.mutations.take().unwrap_or_default();
            let Some(completion) = self.completion.as_ref() else {
                return self.fault(cx, "layout import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                return self.fault(cx, "layout import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return self.fault(cx, &message);
            }
            self.completed = true;
        }
        semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Complete => descriptor.complete(None, None),
            semio_framework_job::JobOutcomeKind::CheckpointReady { .. } => self.checkpoint_publication.borrow_outcome(descriptor),
            semio_framework_job::JobOutcomeKind::Fault => self.fault_publication.borrow_outcome(descriptor),
            semio_framework_job::JobOutcomeKind::PreviewReady => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout import publishes no preview")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        self.closing = true;
        match self.close_original(grant) {
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress)) => semio_framework_job::InteractiveJobCloseStep::Pending { progress },
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress)) => semio_framework_job::InteractiveJobCloseStep::Complete { progress },
            Err(error) => semio_framework_job::InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        }
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(maximum_copy_bytes)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.close_demands(0)?.depth) }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_none()
            && self.media.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.mutations.is_none()
            && self.active.is_none()
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
            && self.fault_publication.terminal_is_empty()
            && self.checkpoint_publication.terminal_is_empty()
    }
}

impl ArtifactReservedJob for LayoutImportJob {}
//#endregion 🎞️ReservedImport

fn layout_build_export_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<LayoutPlayApp>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    use semio_s_artifact_layout_layout_engine_export::{LayoutExportKind, LayoutExportRequest};
    let (kind, page_id) = match *request.command {
        LayoutCommand::ExportPng(payload) => (LayoutExportKind::Png, payload.page_id.or_else(|| Some(blueprint::config::from_snapshot(request.window_config.as_ref()).active_page_id))),
        LayoutCommand::ExportSvg(payload) => (LayoutExportKind::Svg, payload.page_id.or_else(|| Some(blueprint::config::from_snapshot(request.window_config.as_ref()).active_page_id))),
        // 📕️ An unaddressed PDF is the whole document (every page); PNG/SVG stay single-page.
        LayoutCommand::ExportPdf(payload) => (LayoutExportKind::Pdf, payload.page_id),
        LayoutCommand::ExportPackage(_) => (LayoutExportKind::Package, None),
        _ => return Ok(None),
    };
    if request.tool_id != kind.tool_id() {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "layout export command does not match its exact registered tool"));
    }
    let canonical_base_revision_hex = request.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join("");
    // 📤️ A queue of the job's OWN, never `request.output_chunks`: the mounted operation keeps a clone
    // of the request's queue and drains it at retirement (`MountedTypedCommandFullOperation::
    // retirement_step`), which emptied every download right after the host's ACK — a 0-byte `Demo.pdf`
    // (ticket 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END). Same shape as puzzle's `export_snapshot`.
    drop(request.output_chunks);
    let payload = LayoutExportToolPayload {
        request: LayoutExportRequest { kind, page_id, snapshot: request.snapshot, preflight_json: None, parent_document_id: request.parent_document_id, canonical_base_revision_hex },
        output_chunks: semio_framework_plugin::app::ArtifactOutputChunks::new(semio_s_artifact_layout_layout_engine_export::MAX_LAYOUT_EXPORT_OUTPUT_BYTES),
        completion: Some(request.completion),
    };
    Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
}

//#region 🔖️WindowEngagement
fn layout_window_engagement(config: &LayoutWindowConfig, transient: &LayoutWindowTransient, label: &str, labels: &LayoutLabels) -> WindowEngagement {
    WindowEngagement {
        session_active: Some(false),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some(format!("layout-engagement-{label}")),
            value: Some(transient.engagement_input.clone()),
            placeholder: Some("undo, redo, export png".into()),
            disabled: None,
            on_change: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "engagementInput".into(), args: None }),
            on_submit: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "engagementSubmit".into(), args: None }),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: format!("layout-status-{label}"), text: format!("{} {}", labels.page.as_str(), config.active_page_id) }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "layout.eng.undo".into(), label: labels.undo.into(), detail: None, action: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "undo".into(), args: None }) },
            WindowEngagementPossible { id: "layout.eng.redo".into(), label: labels.redo.into(), detail: None, action: Some(ActionDescriptor { controller_id: LAYOUT_PLAY_APP_ID.into(), action: "redo".into(), args: None }) },
        ]),
    }
}
//#endregion 🔖️WindowEngagement

//#region 🪧️EntityLabels
/// 🪧️ What a history-edit reference chip reads for the layout entity `id` (gap N3): a frame (`kinds` names `frame`, or
/// none is declared) by its kind word — a text frame quoting the start of its story, an image frame its linked file, any
/// other frame its ordinal among the frames of its kind on its page when there are several — then its page name. Pages,
/// layers and styles name themselves through the framework's generic walk (their `name`); `None` for every other id.
pub fn layout_entity_label(document: &LayoutSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    if !(kinds.is_empty() || kinds.iter().any(|kind| kind == "frame")) {
        return None;
    }
    let pages = document.pages.iter().map(|page| (page.name.as_str(), page.frames.as_slice())).chain(document.parent_pages.iter().map(|page| (page.name.as_str(), page.frames.as_slice())));
    let (page, frames) = pages.into_iter().find(|(_, frames)| frames.iter().any(|frame| frame.id() == id))?;
    let frame = frames.iter().find(|frame| frame.id() == id)?;
    let quoted = match frame {
        Frame::Text { story_id, .. } => document.stories.iter().find(|story| &story.id == story_id).and_then(|story| layout_label_excerpt(&story.content)),
        Frame::Image { link_id, .. } => document.links.iter().find(|link| &link.id == link_id).and_then(|link| link.path.rsplit(['/', '\\']).next().and_then(layout_label_excerpt)),
        Frame::Rect { .. } => None,
    };
    let siblings: Vec<&Frame> = frames.iter().filter(|other| other.kind_str() == frame.kind_str()).collect();
    let ordinal = (siblings.len() > 1).then(|| siblings.iter().position(|other| other.id() == id).map_or(1, |index| index + 1));
    let (kind, page) = (frame.kind_str().to_string(), page.to_string());
    Some(LocalizedLabel::from_fn(move |terminology, locale| {
        let labels = <LayoutLabels as semio_framework_ui_locale::AppLabels>::labels(locale, terminology);
        let word = match kind.as_str() {
            "text" => labels.kind_text,
            "image" => labels.kind_image,
            _ => labels.kind_rect,
        }
        .as_str();
        let (open, close) = match locale {
            semio_framework_ui_locale::Locale::De => ("\u{201e}", "\u{201c}"),
            semio_framework_ui_locale::Locale::En => ("\u{201c}", "\u{201d}"),
        };
        match (&quoted, ordinal) {
            (Some(quoted), _) => format!("{word} {open}{quoted}{close} \u{b7} {page}"),
            (None, Some(ordinal)) => format!("{word} {ordinal} \u{b7} {page}"),
            (None, None) => format!("{word} \u{b7} {page}"),
        }
    }))
}

/// ✂️ The first non-blank line of `content`, trimmed and cut to [`LAYOUT_LABEL_EXCERPT_CHARS`] characters with an ellipsis.
fn layout_label_excerpt(content: &str) -> Option<String> {
    let line = content.lines().map(str::trim).find(|line| !line.is_empty())?;
    Some(if line.chars().count() > LAYOUT_LABEL_EXCERPT_CHARS { format!("{}\u{2026}", line.chars().take(LAYOUT_LABEL_EXCERPT_CHARS).collect::<String>()) } else { line.to_string() })
}

/// 📏️ The longest story or file excerpt a frame chip quotes.
const LAYOUT_LABEL_EXCERPT_CHARS: usize = 24;
//#endregion 🪧️EntityLabels

//#region 🔖️LayoutPlayApp
/// 🧪️ Stateless app shell; exact window owners hold persisted view preferences and ephemeral input.
#[derive(Default)]
pub struct LayoutPlayApp;

impl LayoutPlayApp {
    fn render_body(
        body_key: &str,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &LayoutWindowTransient,
        interaction: &LayoutInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let document = doc.snapshot;
        let mut config = blueprint::config::current(cfg);
        if body_key == LAYOUT_PLAY_BODY_BLUEPRINT {
            if let Some(utility) = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()) {
                config.active_utility = utility.to_string();
            }
        }
        let labels = layout_labels(view_state);
        match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => match transient.transform_tool.as_deref() {
                Some(tool) => blueprint::render(&blueprint::transform::layout_transform_tool_preview(document, tool), &config, transient, interaction),
                None => blueprint::render(document, &config, transient, interaction),
            },
            LAYOUT_PLAY_BODY_PREVIEW => preview::render(document, &config, transient, interaction),
            LAYOUT_PLAY_BODY_ARTIFACT => document_panel::render(document, &config, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LAYOUT_PLAY_BODY_ARTIFACT)),
            LAYOUT_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LAYOUT_PLAY_BODY_CATALOGUE)),
            LAYOUT_PLAY_BODY_INSPECTION => inspection_panel::render(document, &config, interaction, labels),
            LAYOUT_PLAY_BODY_PREFLIGHT => preflight_panel::render(document, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LAYOUT_PLAY_BODY_PREFLIGHT)),
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "layout error text admission failed")),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }
}

impl ArtifactEditor for LayoutPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_layout_layout_examples::demo::source()]
    }
    type Snapshot = LayoutSnapshot;
    type Mutation = LayoutMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = semio_framework_plugin::NoPresence;
    type PresenceMutation = semio_framework_plugin::NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = LayoutCommand;

    const DIALECT: Dialect = semio_s_artifact_layout_layout_model::LAYOUT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> LayoutSnapshot {
        semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(semio_s_artifact_layout_layout_engine::layout_io())
    }

    /// 🏷️ Supplied wholesale by `app_commands!`'s generated `command_id()`.
    fn command_id(command: &LayoutCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Host-action bridge into the closed `LayoutCommand` enum (ticket
    /// 26/09/16/LAYOUT-PLUGIN-END-TO-END). The React/wgpu shells still speak `{action, args}` with
    /// camelCase argument keys; every `🎮️commands/*` payload derives `FromValue` over its own
    /// snake_case field names, so this boundary folds the keys and decodes — the default trait impl
    /// refuses every app action outright, which left every panel/canvas verb dead in the shell.
    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        args_bridge::command_from_action(action, args)
    }

    /// 📨️ Every host event ends the window's open transform-tool gesture with zero trace under the reason the tool
    /// records: a blur `blur`, a lost pointer capture `captureLost`, a utility switch or a closing window `retired`, an
    /// opened history edit `frozen` and a remote edit `baseMoved` — the window's typed `translateSelection{phase: "abort"}`.
    fn host_event(event: &semio_framework_plugin::HostEvent) -> Option<Self::Command> {
        use semio_framework_plugin::HostEvent;
        use semio_framework_tool_machine::ToolAbortReason;
        let reason = match event {
            HostEvent::WindowBlurred { .. } => ToolAbortReason::Blur,
            HostEvent::PointerCaptureLost { .. } => ToolAbortReason::CaptureLost,
            HostEvent::UtilityChanged { .. } | HostEvent::Retiring { .. } => ToolAbortReason::Retired,
            HostEvent::TimeTravelFrozen { .. } => ToolAbortReason::Frozen,
            HostEvent::BaseMoved { .. } => ToolAbortReason::BaseMoved,
        };
        Some(LayoutCommand::TranslateSelection(gumball::TranslateSelection { ids: Vec::new(), dx: 0.0, dy: 0.0, phase: Some("abort".into()), reason: Some(reason.as_str().into()) }))
    }

    /// 🪧️ A history-edit reference chip names a frame by its kind, content and page ([`layout_entity_label`]).
    fn entity_label(snapshot: &LayoutSnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        layout_entity_label(snapshot, kinds, id)
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🧾️ Store publication authority for the `Artifact` lane — without it the host refuses every
    /// document verb at dispatch (`declares the unsupported artifact publication lane`). The paged document-lane
    /// preparation clones the snapshot page by page, so `CreatePage` (a whole page with its layers) and
    /// `ChangeDataFields` (a `fields:in` dictionary) are bounded only by their own encoded mutation footprint
    /// (`ArtifactStoreOneItemFootprint::for_leaf`).
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        blueprint::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        blueprint::transient::register(registry)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        LayoutRetainedProofs::bounded_first_step_tool_proofs().into_iter().chain(LayoutExportProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(LayoutRetainedCommandJobFactory::new(&controller))?;
        registry.register(LayoutExportJobFactory::new(&controller))?;
        registry.register(LayoutMediaExportJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !LAYOUT_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return layout_build_export_tool_job(request);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "layout retained command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(LayoutWindowWork::new(tool_id));
        let operation_context = AppOperationContext { retained: request.retained,
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            LayoutCommand::command_id,
            LAYOUT_RETAINED_RAW_BYTES,
            LAYOUT_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🎞️ `import-media` is the only reserved route layout owns: every inbound `fields:in` delivery is
    /// routed through this builder (`dispatch_import_media` → `build_artifact_reserved_media_job`);
    /// `copy`/`cut`/`paste` stay on the framework's own reserved factories.
    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if request.tool_id.as_str() != LAYOUT_IMPORT_TOOL_ID {
            return Ok(None);
        }
        if !request.raw_wire.is_empty() {
            return Err(Fault::from("layout import-media admits a decoded media value, never a wire payload"));
        }
        let ArtifactReservedToolInput::Media { port, media } = &request.input else {
            return Err(Fault::from("layout import-media requires media input"));
        };
        let (port, media) = (port.clone(), media.clone());
        Ok(Some(ArtifactReservedToolJob::new(LayoutImportJob::new(request, port, media))))
    }

    fn build_media_export_job(request:&mut Option<ArtifactMediaExportJobRequest<EditorApp<Self>>>,body:&mut Option<Box<dyn semio_framework_plugin::ArtifactReservedJob+Send>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use semio_s_artifact_layout_layout_engine_export::LAYOUT_MEDIA_EXPORT_TOOL_ID;
        if request.as_ref().is_some_and(|input|input.port!="layout:out"||input.tool_id!=LAYOUT_MEDIA_EXPORT_TOOL_ID){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout producer requires its exact registered output port").into())}
        LayoutMediaExportJob::admit_original(request,body,cx)
    }

    fn handle(
        command: &LayoutCommand,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<LayoutMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let gumball_verb = match command {
            LayoutCommand::TranslateSelection(payload) => Some(payload.verb()),
            LayoutCommand::RotateSelection(payload) => Some(payload.verb()),
            LayoutCommand::ScaleSelection(payload) => Some(payload.verb()),
            _ => None,
        };
        let mut emit = match (command, gumball_verb) {
            (_, Some(verb)) => gumball::transform(verb, doc, cfg, &interaction.selection(LAYOUT_INTERACTION_ELEMENTS).ids, None, _view_state.map(|view| layout_labels(view).frames_locked.as_str())).emit,
            (LayoutCommand::DeleteSelection(payload), None) => delete_selection::apply(payload, doc, cfg, interaction)?,
            _ => command.dispatch(doc, cfg)?,
        };
        if let (LayoutCommand::AddPage(_), Some(view)) = (command, _view_state) {
            let mut config = blueprint::config::current(cfg);
            config.active_page_id = format!("page-{}", doc.snapshot.pages.len() + 1);
            emit.window_config_mutations.push(blueprint::config::addressed(view, config)?);
        }
        Ok(emit)
    }

    //#region 🔖️Media
    /// 🎞️ `document:out` is the bounded document-pack batch recipe. Interactive `layout:out`
    /// is intercepted by the exact registered media-export factory before this reducer is reached;
    /// direct reducer reachability therefore fails closed instead of completing an export inline.
    fn export_media(port: &str, doc: &ArtifactView<'_, LayoutSnapshot>) -> Result<Media, MediaError> {
        match port {
            "artifact:out" => {
                let bytes = pack::snapshot::ArtifactPack::encode_pack(doc.snapshot);
                Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "layout:out" => Err(MediaError::NotImplemented),
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `fields:in` binds the incoming `form.dictionary`
    /// values into `LayoutSnapshot::data_fields` — layout has no existing text-interpolation/
    /// field-binding concept for frames/stories yet, so this stores the dictionary verbatim as a new
    /// named data source (see `crate::LayoutSnapshot::data_fields`'s doc) rather
    /// than wiring it into rendering today.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, LayoutSnapshot>) -> Result<Emit<LayoutMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "fields:in" => {
                let MediaPayload::Intrinsic {schema,value}= &media.payload else{return Err(MediaError::Payload(port.to_string(),"fields:in requires an intrinsic form dictionary".into()))};
                if schema!="form.dictionary"{return Err(MediaError::Payload(port.to_string(),"fields:in requires the declared form.dictionary schema".into()))}
                let mut callback=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut callback);
                let dictionary=semio_framework_value_dictionary::QuestionDictionary::from_intrinsic_controlled(value,&mut control).map_err(|error|MediaError::Payload(port.to_string(),error.into_message()))?;
                Ok(Emit::mutations(vec![LayoutMutation::ChangeDataFields(ChangeDataFields {new_fields:Some(dictionary)})]))
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    fn render(body_key: &str, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &LayoutWindowTransient::default(), &LayoutInteractionSnapshot::default())
    }

    fn window_engagements(_doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let config = blueprint::config::current(cfg);
        let labels = layout_labels(view_state);
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        HashMap::from([(window_id, layout_window_engagement(&config, &LayoutWindowTransient::default(), "window", labels))])
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        Self::render_body(body_key, doc, cfg, view_state, &blueprint::transient::current(transient), &LayoutInteractionSnapshot::from_interaction(interaction))

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let mut engagements = Self::window_engagements(doc, cfg, view_state);
        let Some(window_id) = view_state.window_id.as_ref() else { return HashMap::new() };
        if let Some(engagement) = engagements.get_mut(window_id) {
            *engagement = layout_window_engagement(&blueprint::config::current(cfg), &blueprint::transient::current(transient), "window", layout_labels(view_state));
        }
        engagements
    }

    fn context_menu(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), view_state, request.surface.as_ref(), &[])
    }

    fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        let fallback = LayoutInteractionSnapshot::from_interaction(interaction).ids;
        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), view_state, request.surface.as_ref(), &fallback)
    }
}
//#endregion 🔖️LayoutPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_layout_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(semio_s_artifact_layout_layout_model::LAYOUT_DIALECT)
            .artifact_kind(ArtifactKindSpec {
                id: "2d.layout".into(),
                label: LocalizedLabel::native("Layout", "Layout"),
                source_format: "layout.layout".into(),
                component_kind: "layout".into(),
                dimension: "2d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                schema: "layout.layout".into(),
                export_formats: vec![],
                import_formats: vec![],
                export_stdio_kinds: vec!["stdio.svg".into(), "stdio.png".into()],
                import_stdio_kinds: vec!["stdio.svg".into(), "stdio.png".into()],
            })
            .document(["semio", "layout"])
            .icon_id("layout")
            .mode_def(edit::definition())
            .default_mode_id(edit::LAYOUT_PLAY_MODE_EDIT)
            .window_kind_def(blueprint::definition())
            .window_kind_def(preview::definition())
            .utility(blueprint::select::definition())
            .utility(blueprint::transform::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(preflight_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            // ✏️ Palette-visible content commands — dispatched as VCS operations with a true inverse.
            .mutation("addFrame", LocalizedLabel::native("Add Frame", "Rahmen hinzufügen"))
            .mutation("addPage", LocalizedLabel::native("Add Page", "Seite hinzufügen"))
            .action_args("addFrame", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("rect", LocalizedLabel::native("Rectangle", "Rechteck")),
                    ActionArgOption::new("text", LocalizedLabel::native("Text Frame", "Textrahmen")),
                    ActionArgOption::new("image", LocalizedLabel::native("Image Frame", "Bildrahmen")),
                ]).default_value(&"rect"),
                ActionArgDef::number("x", LocalizedLabel::native("X", "X")),
                ActionArgDef::number("y", LocalizedLabel::native("Y", "Y")),
            ])
            // 🐚️ Palette-visible shell exports — round-trip through the host.
            .shell_action("exportPng", LocalizedLabel::native("Export Png", "Png exportieren"))
            .shell_action("exportSvg", LocalizedLabel::native("Export Svg", "Svg exportieren"))
            .shell_action("exportPdf", LocalizedLabel::native("Export Pdf", "Pdf exportieren"))
            .shell_action("exportPackage", LocalizedLabel::native("Export Package", "Paket exportieren"))
            // 🔧️ Internal document operations — inspector/DnD-bound, not palette commands.
            .action_with(layout_internal_action("patchPage", LocalizedLabel::native("Patch Page", "Seite aktualisieren"), ActionKind::Mutation))
            .action_with(layout_internal_action("patchFrame", LocalizedLabel::native("Patch Frame", "Rahmen aktualisieren"), ActionKind::Mutation))
            .action_with(layout_internal_action("patchDocument", LocalizedLabel::native("Patch Document", "Dokument aktualisieren"), ActionKind::Mutation))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection") })
            .action_destructive("deleteSelection")
            .action_destructive("exportPng")
            .action_destructive("exportSvg")
            .action_destructive("exportPdf")
            .action_destructive("exportPackage")
            .action_with(layout_internal_action("canvasDrop", LocalizedLabel::native("Canvas Drop", "Ablegen auf Leinwand"), ActionKind::Mutation))
            // 👁️ Ephemeral view state — active page, drop ghost, pointer, camera, engagement draft.
            // Selection/hover are framework-owned now (domain "elements") — no app-declared verbs;
            // `interactionSelect`/`interactionHover`/`clearSelection` auto-inject below.
            .action_with(layout_internal_action("setActivePage", LocalizedLabel::native("Set Active Page", "Aktive Seite festlegen"), ActionKind::View))
            .action_with(layout_internal_action("focusPreflightIssue", LocalizedLabel::native("Focus Preflight Issue", "Preflight-Problem fokussieren"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), ActionKind::View, "mouse-pointer") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegen"), ActionKind::View, "mouse-pointer") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), ActionKind::View, "mouse-pointer") })
            .action_with(layout_internal_action("canvasDragOver", LocalizedLabel::native("Canvas Drag Over", "Ziehen über Leinwand"), ActionKind::View))
            .action_with(layout_internal_action("canvasDragLeave", LocalizedLabel::native("Canvas Drag Leave", "Ziehen verlässt Leinwand"), ActionKind::View))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            // 🐚️ Engagement submit — routes typed export intents through the host, emits only shell effects.
            .action_with(layout_internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Shell))
            // 💬️ Agent-facing descriptions (ticket 26/09/18 slice M5a) — EN first, DE second.
            .action_describe("addPage", LocalizedLabel::native("Appends a new page to the layout document.", "Fügt dem Layoutdokument eine neue Seite hinzu."))
            .action_use_when("addPage", vec!["add a page".into(), "new page in the layout".into()])
            .action_describe("addFrame", LocalizedLabel::native("Places a new frame on a page — the box that holds an image, a text block or a nested artifact.", "Platziert einen neuen Rahmen auf einer Seite — den Kasten für Bild, Textblock oder eingebettetes Artefakt."))
            .action_use_when("addFrame", vec!["add a frame".into(), "place an image box".into(), "add a text frame".into()])
            .action_describe("patchPage", LocalizedLabel::native("Sets one named property of one page — its size, orientation, margins or name.", "Setzt eine benannte Eigenschaft einer Seite — Größe, Ausrichtung, Ränder oder Name."))
            .action_use_when("patchPage", vec!["make the page A4".into(), "change the page margins".into()])
            .action_describe("patchFrame", LocalizedLabel::native("Sets one named property of one frame — its position, size, content binding or style.", "Setzt eine benannte Eigenschaft eines Rahmens — Position, Größe, Inhaltsbindung oder Stil."))
            .action_use_when("patchFrame", vec!["move a frame".into(), "resize this frame".into()])
            .action_describe("patchDocument", LocalizedLabel::native("Sets one named property of the layout document — its name, print target, or data fields.", "Setzt eine benannte Eigenschaft des Layoutdokuments — Name, Druckziel oder Datenfelder."))
            .action_use_when("patchDocument", vec!["rename the layout".into(), "set the print target".into()])
            .action_describe("deleteSelection", LocalizedLabel::native("Removes the selected pages or frames from the layout.", "Entfernt die ausgewählten Seiten oder Rahmen aus dem Layout."))
            .action_describe("exportPdf", LocalizedLabel::native("Renders the whole layout to a downloadable PDF.", "Rendert das gesamte Layout in eine herunterladbare PDF-Datei."))
            .action_use_when("exportPdf", vec!["export the document as pdf".into(), "print this layout to pdf".into()])
            .action_describe("exportPng", LocalizedLabel::native("Renders the layout to downloadable PNG raster images.", "Rendert das Layout in herunterladbare PNG-Rasterbilder."))
            .action_describe("exportSvg", LocalizedLabel::native("Renders the layout to a downloadable SVG vector document.", "Rendert das Layout in ein herunterladbares SVG-Vektordokument."))
            .action_describe("exportPackage", LocalizedLabel::native("Bundles the layout and every asset it references into one downloadable package.", "Bündelt das Layout und alle referenzierten Assets in ein herunterladbares Paket."))
            .action_describe("setActivePage", LocalizedLabel::native("Brings one page of the layout into view.", "Holt eine Seite des Layouts in die Ansicht."))
            .action_describe("focusPreflightIssue", LocalizedLabel::native("Scrolls to and highlights one reported preflight issue.", "Springt zu einem gemeldeten Preflight-Problem und hebt es hervor."))
            // 🖱️ Raw input plumbing — the canvas feeds these, agents never do.
            .action_audience("canvasDrop", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasDragOver", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasDragLeave", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_interactive_job("setActivePage", InteractiveJobClassification::Migrated)
            .action_interactive_job("focusPreflightIssue", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasDragOver", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasDragLeave", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportPng", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportSvg", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportPdf", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportPackage", InteractiveJobClassification::Migrated)
            .action_interactive_job("addFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("addPage", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchPage", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchFrame", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchDocument", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasDrop", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_with(layout_internal_action("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation))
            .action_with(layout_internal_action("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation))
            .action_with(layout_internal_action("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation))
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_describe("translateSelection", LocalizedLabel::native("Moves the selected frames by a delta in page units.", "Verschiebt die ausgewählten Rahmen um einen Versatz in Seiteneinheiten."))
            .action_use_when("translateSelection", vec!["move the selected frames".into(), "shift this frame to the right".into()])
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the selected frames by an angle in radians about the centroid of their centres.", "Dreht die ausgewählten Rahmen um einen Winkel im Bogenmaß um den Schwerpunkt ihrer Mittelpunkte."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the selected frames by a factor per axis about the centroid of their centres.", "Skaliert die ausgewählten Rahmen um einen Faktor je Achse um den Schwerpunkt ihrer Mittelpunkte."))
            // 📇️ Per-window action scoping — the content-authoring operations only make sense on the
            // interactive Blueprint surface; the read-only Preview surface renders output and never
            // creates or edits frames/pages. Exports, camera, pointer/drag, selection and hover are
            // surface-discriminated (via `surfaceId`) or global, so they stay unscoped orphans and
            // appear on both windows.
            .window_kind_action_refs(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![
                "addFrame".into(), "addPage".into(), "patchPage".into(), "patchFrame".into(), "patchDocument".into(), "translateSelection".into(), "rotateSelection".into(), "scaleSelection".into(),
            ])
            // 🕹️ Domain "elements": frames on the Blueprint canvas (pages are never targets — canvas
            // hit-testing only ever resolves frame ids). Flat: layout has no real parent/child
            // structure among frames. `Invertive` merge is the toggle-on-shift-click old
            // `CanvasPointerDown.extend` used to hand-roll; `Replace` is a plain click.
            .interaction(InteractionDefinition {
                id: LAYOUT_INTERACTION_ELEMENTS.into(),
                label: LocalizedLabel::native("Elements", "Elemente"),
                granularities: vec![GranularityDefinition { id: LAYOUT_GRANULARITY_ELEMENT.into(), label: LocalizedLabel::native("Element", "Element"), icon_id: "square".into() }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Invertive],
                    transitive: false,
                    broadcast: true,
                },
            })
            .window_kind_interactions(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![InteractionRef::new(LAYOUT_INTERACTION_ELEMENTS)])
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS) — `config_spec()`/`layout_io()`
            // are this same information's single source of truth, reused here rather than duplicated.
            .io(semio_s_artifact_layout_layout_engine::layout_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old `"sample"`/`"cylinder"`
            // app-level example registration and the no-op `.workflow("layout", …)` call are dropped
            // here (not silently: reported in this packet's migration notes). The subset's own
            // `📚️examples/🎬️demo` facet (`crate::examples::...`, real content,
            // pre-existing) is the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
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

pub struct LayoutExportToolPayload {
    pub request: LayoutExportRequest,
    pub output_chunks: ArtifactOutputChunks,
    pub completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
}

pub struct LayoutExportToolJob {
    inner: Option<LayoutExportJob>,
    pending_operation: Option<Operation>,
    pending_request: Option<LayoutExportRequest>,
    pending_output_chunks: Option<ArtifactOutputChunks>,
    completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
    kind: LayoutExportKind,
    name: String,
    raw_input: Option<RetainedToolWireInput>,
    raw_bytes: Vec<u8>,
    raw_page_cursor: usize,
    raw_validated: bool,
    completed: bool,
    slot: LayoutPublicationSlot,
    pending_download: Option<ArtifactDownloadOutput>,
    rejected_download: Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,
}

impl LayoutExportToolJob {
    fn materialize_decoded_job(&mut self,source_identity:(semio_framework_job::OperationId,semio_framework_job::Generation)) -> bool {
        if self.inner.is_some() {
            return true;
        }
        let (Some(operation), Some(request), Some(output_chunks)) = (self.pending_operation.take(), self.pending_request.take(), self.pending_output_chunks.take()) else {
            return false;
        };
        self.name = output_name(&request);
        self.inner = Some(LayoutExportJob::from_admitted(operation,source_identity,request,output_chunks));
        self.inner.is_some()
    }

    /// 🔐️ The retained wire is the command's `OpBinary` encoding (ticket
    /// 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END) — the JSON-text state machine that used to sit here
    /// faulted at the first byte of every real export, which the shell only showed as an empty
    /// `typed-operation failed:`. Verb and page target must match the reducer-built request.
    fn decoded_wire_command_matches(&self) -> bool {
        let Ok(command) = <crate::LayoutCommand as protocol::OpBinary>::decode_op(&self.raw_bytes) else { return false };
        if command.command_id() != self.kind.tool_id() {
            return false;
        }
        let page_id = match &command {
            crate::LayoutCommand::ExportPng(payload) => payload.page_id.clone(),
            crate::LayoutCommand::ExportSvg(payload) => payload.page_id.clone(),
            crate::LayoutCommand::ExportPdf(payload) => payload.page_id.clone(),
            _ => None,
        };
        match self.kind {
            LayoutExportKind::Package => page_id.is_none(),
            _ => page_id.as_ref().is_none_or(|page_id| self.pending_request.as_ref().and_then(|request| request.page_id.as_ref()) == Some(page_id)),
        }
    }
}

impl LayoutExportToolJob {
    fn fault<'a>(&'a mut self, context: &mut StepContext<'_>, detail: &str) -> LayoutExportTurn<'a> {
        self.slot.publication = Some(LayoutExportJob::fault_publication(detail));
        self.slot.advance(context)
    }

    fn checkpoint<'a>(&'a mut self, context: &mut StepContext<'_>, applied_progress: u64) -> LayoutExportTurn<'a> {
        let cursor = (self.raw_page_cursor as u64).to_le_bytes();
        self.slot.publication = LayoutExportPublication::new(LayoutExportPublicationKind::Checkpoint { applied_progress }, &cursor).ok();
        self.slot.advance(context)
    }

    fn close_demand(&self)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        let one=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
        if let Some(owner)=self.rejected_download.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,0)}
        if self.pending_download.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_download)}
        if let Some(inner)=self.inner.as_ref(){return if inner.terminal_is_empty(){Ok(one)}else{inner.close_demand()}}
        if self.pending_request.is_some(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<LayoutExportJob>(),..one})}
        if self.raw_bytes.capacity()!=0{return Ok(semio_framework_value::RetirementDemand{release_bytes:self.raw_bytes.capacity(),..one})}
        if let Some(input)=self.raw_input.as_ref(){return Ok(semio_framework_value::RetirementDemand{copy_bytes:input.next_close_copy_byte_demand()?,capacity_bytes:input.next_close_capacity_byte_demand(0)?,release_bytes:input.next_close_release_byte_demand()?,depth:input.next_close_depth_demand()?.max(1)})}
        if self.name.capacity()!=0{return Ok(semio_framework_value::RetirementDemand{release_bytes:self.name.capacity(),..one})}
        if !self.slot.terminal_is_empty(){return self.slot.retirement_demands()}
        if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion)}
        Ok(Default::default())
    }

}

impl InteractiveJob for LayoutExportToolJob {
    fn step<'a>(&'a mut self, context: &mut StepContext<'_>) -> LayoutExportTurn<'a> {
        if !self.slot.settle(context)? {
            return Ok(None);
        }
        if self.slot.publication.is_some() {
            return self.slot.advance(context);
        }
        if !self.raw_validated {
            if context.is_cancelled() {
                return JobOutcomeBorrow::admit_cancelled(context).map_err(Into::into);
            }
            if context.should_yield() || context.fuel_remaining() == 0 {
                return JobOutcomeBorrow::admit_yield(context).map_err(Into::into);
            }
            context.set_stage("layout-export-retained-wire-decode");
            let Some(input) = self.raw_input.as_ref() else {
                return self.fault(context, "layout-export-wire-input-missing");
            };
            if let Some(page) = input.page(self.raw_page_cursor) {
                self.raw_bytes.extend_from_slice(page);
                self.raw_page_cursor = self.raw_page_cursor.saturating_add(1);
                context.consume_fuel(1);
                let progress = self.raw_bytes.len() as u64;
                return self.checkpoint(context, progress);
            }
            if !self.decoded_wire_command_matches() {
                return self.fault(context, "layout-export-wire-command-mismatch");
            }
            if !self.materialize_decoded_job((context.operation(),context.generation())) {
                return self.fault(context, "layout-export-wire-job-refused");
            }
            self.raw_validated = true;
            context.consume_fuel(1);
            let progress = self.raw_bytes.len().saturating_add(1) as u64;
            return self.checkpoint(context, progress);
        }
        let Some(inner) = self.inner.as_mut() else {
            self.slot.publication = Some(LayoutExportJob::fault_publication("layout-export-inner-missing"));
            return self.slot.advance(context);
        };
        if inner.commit_ready() && !self.completed && self.completion.is_some() {
            // 📤️ The download OWNS the sealed chunk queue from here: `ArtifactOutputChunks` clones
            // share one queue, and the job's own close (`LayoutExportCloseStage::OutputChunks`) pops it —
            // which drained every export to a 0-byte `Demo.pdf` before the host could take a single
            // chunk (ticket 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END). The job keeps an empty stand-in.
            let chunks = inner.output_chunks.take().expect("layout publication owns its admitted output");
            let download = ArtifactDownloadOutput::new(format!("{}.{}", sanitize_filename(&self.name), self.kind.extension()), self.kind.mime_type(), self.kind.binary().then(|| "base64".into()), chunks);
            if let Some(completion) = &self.completion {
                if let Err(rejected) = completion.complete_download(download, EphemeralEmit::<EditorApp<LayoutPlayApp>>::default()) {
                    if let Ok(download) = rejected.download {
                        self.pending_download = Some(download);
                    }
                    self.slot.publication = Some(LayoutExportJob::fault_publication("layout-export-download-rejected"));
                    return self.slot.advance(context);
                }
            }
            self.completed = true;
        }
        inner.step(context)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            _ if self.slot.delivered => self.slot.borrow_outcome(descriptor),
            _ => self.inner.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout export outcome lost its job"))?.borrow_outcome(descriptor),
        }
    }

    fn begin_close(&mut self) {
        if let Some(input) = self.raw_input.as_mut() {
            input.begin_close();
        }
        if let Some(inner) = self.inner.as_mut() {
            inner.begin_close();
        }
    }

    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_value::{RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};self.begin_close();
        if let Some(inner)=self.inner.as_mut(){if !inner.terminal_is_empty(){return inner.close_step(cx).map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()})}}
        if self.inner.is_none()&&self.pending_request.is_none(){if let Some(input)=self.raw_input.as_mut(){let step=input.close_step(cx)?;if input.terminal_is_empty(){self.raw_input.take();}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}}
        let source_identity=(cx.operation(),cx.generation());let demand=self.close_demand()?;let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{
            let item=RetainedCloneProgress{copied_items:1,..Default::default()};
            if self.rejected_download.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.rejected_download,grant)}
            if self.pending_download.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_download,&mut self.rejected_download,grant)}
            if self.inner.is_some(){self.inner.take();return Ok(RetainedCloneStep::Progress(item))}
            if self.pending_request.is_some(){if self.pending_operation.is_none()||self.pending_output_chunks.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout pending producer lost one of its paired original inputs"))}let operation=self.pending_operation.take().unwrap();let request=self.pending_request.take().unwrap();let chunks=self.pending_output_chunks.take().unwrap();let mut inner=LayoutExportJob::from_admitted(operation,source_identity,request,chunks);inner.begin_close();self.inner=Some(inner);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_bytes:size_of::<LayoutExportJob>(),..item}))}
            if self.raw_bytes.capacity()!=0{let bytes=std::mem::take(&mut self.raw_bytes).capacity();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:bytes,..item}))}
            if self.name.capacity()!=0{let bytes=std::mem::take(&mut self.name).capacity();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:bytes,..item}))}
            if !self.slot.terminal_is_empty(){return self.slot.close_step(grant)}
            if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.rejected_download,grant)}
            Ok(RetainedCloneStep::Complete(Default::default()))
        })?;Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
    }

    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,_body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand()?.depth)}

    fn terminal_is_empty(&self) -> bool {
        self.rejected_download.is_none() && self.pending_download.is_none() && self.slot.terminal_is_empty() && self.name.capacity()==0 && self.completion.is_none() && self.raw_input.is_none() && self.raw_bytes.capacity()==0 && self.inner.is_none() && self.pending_operation.is_none() && self.pending_request.is_none() && self.pending_output_chunks.is_none()
    }
}

pub struct LayoutExportJobFactory {
    keys: Vec<ToolFactoryKey>,
}

pub struct LayoutMediaExportJob {
 inner:Option<LayoutExportJob>,completion:Option<ArtifactMediaExportCompletion>,active:Option<Box<dyn store::ErasedSnapshotRetirement>>,
 metadata:std::mem::ManuallyDrop<Option<([Vec<u8>;2],Option<(String,String)>)>>,metadata_retirement:std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<([Vec<u8>;2],Option<(String,String)>)>>>,
 pending_result:std::mem::ManuallyDrop<Option<Result<ArtifactMediaExportResult,semio_framework::MediaError>>>,result_published:bool,
 slot:LayoutPublicationSlot,completed:bool,
}

impl LayoutMediaExportJob{
 pub fn admit_original(request:&mut Option<semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<LayoutPlayApp>>>,body:&mut Option<Box<dyn ArtifactReservedJob+Send>>,cx:&mut StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
  use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
  let _=cx.original_source_parent()?;if cx.should_yield()||cx.is_cancelled(){return Ok(false)}let grant=cx.retained_grant();
  if body.is_none(){let input=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout producer lost original request"))?;let page=input.snapshot.pages.first().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"layout media requires one actual page"))?;
   if !bounded_id(&page.id)||!bounded_id(&input.parent_document_id){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"layout media requires its exact bounded original authority").into())}let bytes=size_of::<Self>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<bytes{return Ok(false)}
   let pointer=std::ptr::NonNull::new(unsafe{std::alloc::alloc(std::alloc::Layout::new::<Self>())}.cast::<Self>()).ok_or_else(||ValueError::literal(ValueRefusalKind::AllocationFailed,"layout media original body allocation refused"))?;
   let owner=Self{inner:None,completion:None,active:None,metadata:std::mem::ManuallyDrop::new(Some((std::array::from_fn(|_|Vec::new()),None))),metadata_retirement:std::mem::ManuallyDrop::new(None),pending_result:std::mem::ManuallyDrop::new(None),result_published:false,slot:LayoutPublicationSlot::default(),completed:false};*body=Some(unsafe{pointer.as_ptr().write(owner);Box::from_raw(pointer.as_ptr())});cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
  }
  let owner=(&mut**body.as_mut().unwrap()as &mut dyn std::any::Any).downcast_mut::<Self>().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout media requires its exact concrete body"))?;
  if owner.inner.is_some()&&request.is_none(){return Ok(true)}if owner.inner.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"layout original request cannot overwrite its prepared body").into())}
  let input=request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout media lost original source before admission"))?;let page=input.snapshot.pages.first().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"layout captured page changed"))?;let fields=&mut owner.metadata.as_mut().unwrap().0;
  for index in 0..2{let bytes=if index==0{page.id.len()}else{64};let output=&mut fields[index];if output.capacity()<bytes{if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(false)}output.try_reserve_exact(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"layout original authority allocation refused"))?;cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:output.capacity(),..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
   if output.len()<bytes{let count=(bytes-output.len()).min(grant.maximum_copy_bytes);if grant.maximum_items==0||grant.maximum_depth==0||count==0{return Ok(false)}let start=output.len();if index==0{output.extend_from_slice(&page.id.as_bytes()[start..start+count])}else{const HEX:&[u8;16]=b"0123456789abcdef";for offset in start..start+count{let value=input.canonical_base_revision[offset/2];output.push(HEX[usize::from(if offset%2==0{value>>4}else{value&15})])}}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;cx.consume_fuel(1);return Ok(false)}
  }
  let bytes=size_of::<LayoutExportJob>()+size_of::<semio_framework_plugin::ArtifactMediaExportJobRequest<EditorApp<LayoutPlayApp>>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}
  let input=request.take().unwrap();let fields=&mut owner.metadata.as_mut().unwrap().0;let page_id=String::from_utf8(std::mem::take(&mut fields[0])).expect("layout retained page bytes preserve original UTF-8");let revision=String::from_utf8(std::mem::take(&mut fields[1])).expect("layout original revision is hexadecimal");
  let export=LayoutExportRequest{kind:LayoutExportKind::Svg,page_id:Some(page_id),snapshot:input.snapshot,preflight_json:None,parent_document_id:input.parent_document_id,canonical_base_revision_hex:revision};
  owner.inner=Some(LayoutExportJob::from_admitted(input.operation,input.source_identity,export,input.output_chunks).with_snapshot_close_lease(input.snapshot_close).with_media_output_credit(input.output_credit));owner.completion=Some(input.completion);owner.metadata.as_mut().unwrap().1=Some((input.tool_id,input.port));cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(false)
 }
 fn close_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
  let one=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
  if let Some(inner)=self.inner.as_ref(){return if inner.terminal_is_empty(){Ok(one)}else{Ok(semio_framework_value::RetirementDemand{copy_bytes:inner.next_close_copy_byte_demand()?,capacity_bytes:inner.next_close_capacity_byte_demand(body)?,release_bytes:inner.next_close_release_byte_demand()?,depth:inner.next_close_depth_demand()?.max(1)})}}
  if let Some(active)=self.active.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body)}
  if self.pending_result.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.pending_result)}
  if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.completion)}
  if !self.slot.terminal_is_empty(){return self.slot.retirement_demands()}
  if let Some(owner)=self.metadata_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(one)}else{owner.selected_demand(body)}}
  Ok(if self.metadata.is_some(){one}else{Default::default()})
 }
 fn close_turn(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
  let item=semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.inner.as_ref().is_some_and(InteractiveJob::terminal_is_empty){self.inner.take();return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}
  if self.active.is_some(){return store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.active,grant)}
  if self.pending_result.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.pending_result,&mut self.active,grant)}
  if self.completion.is_some(){return store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.completion,&mut self.active,grant)}
  if !self.slot.terminal_is_empty(){return self.slot.close_step(grant)}
  if let Some(owner)=self.metadata_retirement.as_mut(){if owner.terminal_is_empty(){self.metadata_retirement.take();return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}return owner.step(grant)}
  if let Some(metadata)=self.metadata.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new(metadata){Ok(owner)=>*self.metadata_retirement=Some(owner),Err((error,metadata))=>{*self.metadata=Some(metadata);return Err(error)}}return Ok(semio_framework_value::RetainedCloneStep::Progress(item))}
  Ok(semio_framework_value::RetainedCloneStep::Complete(Default::default()))
 }
}

impl InteractiveJob for LayoutMediaExportJob{
 fn step<'a>(&'a mut self,cx:&mut StepContext<'_>)->LayoutExportTurn<'a>{
  if !self.slot.settle(cx)?{return Ok(None)}if self.slot.publication.is_some(){return self.slot.advance(cx)}
  if self.pending_result.is_some()||self.result_published{self.completed=self.completion.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout media lost its original completion receiver"))?.complete_original(&mut self.pending_result,&mut self.result_published,cx)?;return Ok(None)}
  let inner=self.inner.as_mut().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout media cannot drive an unadmitted body"))?;
  if inner.commit_ready()&&!self.completed{
   if let Some(credit)=inner.media_output_credit.as_ref(){credit.credit(LAYOUT_MEDIA_EXPORT_SCHEMA.len())?}
   let chunks=inner.output_chunks.take().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout media lost supplied output roots"))?;
   let media=ArtifactMediaExportResult::structured(MediaType{class:MediaClass::TwoD,form:MediaForm::Vector},LAYOUT_MEDIA_EXPORT_SCHEMA,"application/json",chunks)?;
   *self.pending_result=Some(Ok(media));return Ok(None);
  }inner.step(cx)
 }
 fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,semio_framework_value::ValueError>{match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),_ if self.slot.delivered=>self.slot.borrow_outcome(descriptor),_=>self.inner.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"layout outcome lost its original body"))?.borrow_outcome(descriptor)}}
 fn begin_close(&mut self){if let Some(inner)=self.inner.as_mut(){inner.begin_close()}}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
  self.begin_close();if let Some(inner)=self.inner.as_mut(){if !inner.terminal_is_empty(){return inner.close_step(cx).map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()})}}
  let demand=self.close_demand(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.close_turn(grant))?;Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(0)?.copy_bytes)}
 fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(body)?.capacity_bytes)}
 fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(0)?.release_bytes)}
 fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.close_demand(0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.inner.is_none()&&self.completion.is_none()&&self.active.is_none()&&self.slot.terminal_is_empty()&&self.metadata.is_none()&&self.metadata_retirement.is_none()&&self.pending_result.is_none()}
}

impl ArtifactReservedJob for LayoutMediaExportJob {}

pub struct LayoutMediaExportJobFactory {
    keys: [ToolFactoryKey; 1],
}

impl LayoutMediaExportJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: [ToolFactoryKey::new(controller_id, LAYOUT_MEDIA_EXPORT_TOOL_ID)] }
    }
}

impl ToolJobFactory for LayoutMediaExportJobFactory {
    type Payload = ArtifactReservedToolJob;
    type Job = ArtifactReservedToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_MEDIA_EXPORT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_LAYOUT_EXPORT_RAW_BYTES, MAX_LAYOUT_EXPORT_DECODED_ITEMS, 1, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, 2_000, 64, 1)
    }

    fn create_job(&mut self, _operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(payload)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutMediaExportJobFactory {
    type Owner = EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = &[LAYOUT_MEDIA_EXPORT_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: LAYOUT_MEDIA_EXPORT_TOOL_ID, lanes: &[ArtifactToolPublicationLane::HostOnly] }];
}

impl LayoutExportJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: LAYOUT_EXPORT_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for LayoutExportJobFactory {
    type Payload = LayoutExportToolPayload;
    type Job = LayoutExportToolJob;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LAYOUT_EXPORT_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        ToolExecutionContract::resumable(MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES, MAX_LAYOUT_EXPORT_DECODED_ITEMS, 1, MAX_LAYOUT_EXPORT_OUTPUT_BYTES, 2_000, 64, 1)
    }

    fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        let kind = payload.request.kind;
        let name = output_name(&payload.request);
        let inner = LayoutExportJob::from_admitted(operation,(operation.operation,operation.generation),payload.request,payload.output_chunks);
        Ok(LayoutExportToolJob {
            inner: Some(inner),
            rejected_download:None,
            pending_download: None,
            slot: LayoutPublicationSlot::default(),
            pending_operation: None,
            pending_request: None,
            pending_output_chunks: None,
            completion: payload.completion,
            kind,
            name,
            raw_input: None,
            raw_bytes: Vec::new(),
            raw_page_cursor: 0,
            raw_validated: true,
            completed: false,
        })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: Operation,
        payload: Self::Payload,
        input: RetainedToolWireInput,
        checkpoint: Option<RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, RetainedToolWireInput, Option<RetainedToolWireInput>)> {
        if checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("layout export retained ingress does not accept an unvalidated checkpoint owner"), input, checkpoint));
        }
        let declared_bytes = input.declared_bytes();
        if declared_bytes > MAX_LAYOUT_EXPORT_COMMAND_RAW_BYTES {
            return Err((ToolJobFactoryError::new("layout export retained ingress exceeds its command byte envelope"), input, None));
        }
        let kind = payload.request.kind;
        let mut job = LayoutExportToolJob {
            inner: None,
            rejected_download:None,
            pending_download: None,
            slot: LayoutPublicationSlot::default(),
            pending_operation: Some(operation),
            pending_request: Some(payload.request),
            pending_output_chunks: Some(payload.output_chunks),
            completion: payload.completion,
            kind,
            name: String::new(),
            raw_input: None,
            raw_bytes: Vec::new(),
            raw_page_cursor: 0,
            raw_validated: false,
            completed: false,
        };
        if job.raw_bytes.try_reserve_exact(declared_bytes).is_err() {
            return Err((ToolJobFactoryError::new("layout export retained decoder capacity was not admitted"), input, None));
        }
        job.raw_input = Some(input);
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LayoutExportJobFactory {
    type Owner = EditorApp<LayoutPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LAYOUT_EXPORT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_layout_layout_model::LAYOUT_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "exportPng", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportSvg", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportPdf", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "exportPackage", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ];
}
