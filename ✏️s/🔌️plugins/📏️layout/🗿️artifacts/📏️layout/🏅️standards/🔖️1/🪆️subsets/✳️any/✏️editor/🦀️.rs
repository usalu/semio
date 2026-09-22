//! 🖥️ Layout play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared canvas chrome in `🦀️canvas.rs`, headless compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `LayoutCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::layout::engine::export::LayoutExportJobFactory;
use crate::editor::layout::engine::export::LayoutMediaExportJobFactory;
use crate::editor::layout::modes::edit;
use crate::editor::layout::modes::edit::windows::{blueprint, preview};
use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use crate::editor::layout::modes::edit::windows::blueprint::transient::LayoutWindowTransient;
use crate::editor::layout::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, preflight as preflight_panel};
use crate::editor::layout::panels::preflight::{run_layout_preflight, PreflightIssue};
use crate::Frame;
use crate::editor::layout::terminology::{layout_labels, LayoutLabels};
use crate::mutations::change_data_fields::ChangeDataFields;
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework::kernel::Effect;
use semio_framework::{Dialect, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::{Checkpoint, CommitCandidate, InteractiveJob, JobFault, JobPayloadStream, RetainedJobPayload, StepContext, StepOutcome};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::app::{ArtifactMediaExportJobRequest, ArtifactOwnedToolJobRequest, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry};
use semio_framework_plugin::ArtifactReservedJob;
#[cfg(test)]
use semio_framework_plugin::App;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppActionRegistry, ArtifactEditor, ArtifactKindSpec, ArtifactView, ConfigView, ContextMenuItemSpec, ContextMenuRequest, ContextMenuSurfaceTarget, DraftView, DslValue, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec,
    WindowEngagement, WindowEngagementInput, WindowEngagementPossible, WindowEngagementStatus, CLEAR_SELECTION_ACTION_ID, INTERACTION_HOVER_ACTION_ID, INTERACTION_SELECT_ACTION_ID,
};
use semio_framework_plugin::{AppOperationContext, ArtifactToolPublicationContract, ArtifactToolPublicationLane};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::EngineHandles;

use crate::editor::layout::engine::scene::LayoutEngine;

//#region 🔖️Constants
pub const LAYOUT_PLAY_APP_ID: &str = "s.layout.layout@1/*#editor";
pub use blueprint::{LAYOUT_PLAY_BODY_BLUEPRINT, LAYOUT_PLAY_SURFACE_BLUEPRINT, LAYOUT_PLAY_WINDOW_BLUEPRINT};
pub(crate) use catalogue_panel::LAYOUT_PLAY_BODY_CATALOGUE;
pub use document_panel::LAYOUT_PLAY_BODY_ARTIFACT;
pub use inspection_panel::LAYOUT_PLAY_BODY_INSPECTION;
pub use preflight_panel::{LAYOUT_PLAY_BODY_PREFLIGHT, LAYOUT_PLAY_PREFLIGHT_TAB_ID};
pub use preview::{LAYOUT_PLAY_BODY_PREVIEW, LAYOUT_PLAY_SURFACE_PREVIEW, LAYOUT_PLAY_WINDOW_PREVIEW};

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn layout_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(LAYOUT_PLAY_APP_ID).action(action, args)
}

/// 🏷️ Admits display text into the bounded semantic label contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    value.as_ref().try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "layout label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

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

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map<const N: usize>(mut values: [(&'static str, semio_framework_plugin::UiValue); N]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    values.sort_unstable_by(|left, right| left.0.cmp(right.0));
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

//#region 🔖️Interaction
/// 🕹️ The one framework-owned interaction domain layout declares — pages/frames on the Blueprint
/// canvas plus the document tree's frame rows (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
/// Pages are never targets: canvas hit-testing only ever resolves frame ids (`DisplayList::hit_test`
/// only walks `rects`/`images`), so `"elements"` is frames-only despite the document tree also
/// listing pages/spreads/layers/etc.
pub const LAYOUT_INTERACTION_ELEMENTS: &str = "elements";
pub const LAYOUT_GRANULARITY_ELEMENT: &str = "element";

/// 🕹️ Owned snapshot of the framework `"elements"` domain — selection plus `"pointer"` hover — read
/// once per render by `LayoutPlayApp::render_with_request_context` and threaded into the Blueprint
/// canvas chrome and the inspection panel (mirrors `Gis2dInteractionSnapshot`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutInteractionSnapshot {
    pub ids: Vec<String>,
    pub hovered_ids: Vec<String>,
}

impl LayoutInteractionSnapshot {
    pub const POINTER_CHANNEL: &'static str = "pointer";

    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        Self {
            ids: interaction.selection(LAYOUT_INTERACTION_ELEMENTS).ids.clone(),
            hovered_ids: interaction.hover(LAYOUT_INTERACTION_ELEMENTS, Self::POINTER_CHANNEL).ids.clone(),
        }
    }

    pub fn hovered_id(&self) -> Option<&str> {
        self.hovered_ids.first().map(String::as_str)
    }
}

/// 🕹️ Builds `interactionSelect`'s JSON args for one merge over `ids` (all granularity `"element"`) —
/// shared by the canvas pointer commands (wrapped into a `Effect::DispatchAction`) and any
/// document-tree row whose click should select a real canvas element (wrapped into an `ActionDescriptor`).
pub fn layout_select_action_args(ids: &[String], merge: &str) -> Value {
    let targets: Vec<Value> = ids.iter().map(|id| json!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })).collect();
    json!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "targets": serde_json::to_string(&targets).unwrap_or_default(), "merge": merge, "method": "pick" })
}

/// 🐁️ Builds `interactionHover`'s JSON args for the `"pointer"` channel — `id: None` clears hover.
pub fn layout_hover_action_args(id: Option<&str>) -> Value {
    let targets: Vec<Value> = id.map(|id| vec![json!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })]).unwrap_or_default();
    json!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "channel": "pointer", "targets": serde_json::to_string(&targets).unwrap_or_default() })
}

/// 🕹️ Wraps [`layout_select_action_args`] into the redispatch effect a canvas gesture's own `handle`
/// returns — `dispatch_action` intercepts the six framework interaction verbs BEFORE routing to
/// `ArtifactApp::handle`, so a plain config mutation can no longer express a selection change; the
/// app asks the host to redispatch `interactionSelect` instead (master doc: "surfaces do geometric
/// hit-testing and emit one batched `interactionSelect`").
pub fn layout_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(115), action: INTERACTION_SELECT_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(layout_select_action_args(ids, merge))), delay_ms: 0 }
}

/// 🐁️ Wraps [`layout_hover_action_args`] the same way, for `interactionHover`.
pub fn layout_hover_effect(id: Option<&str>) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(114), action: INTERACTION_HOVER_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(layout_hover_action_args(id))), delay_ms: 0 }
}

/// 🕹️ Clicking empty canvas clears every domain's selection — `clearSelection` takes no `domainId`.
pub fn layout_clear_selection_effect() -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(113), action: CLEAR_SELECTION_ACTION_ID.into(), args: None, delay_ms: 0 }
}
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

fn layout_context_menu_item(id: &str, label: &str, icon: &str, action: &str, args: Option<Value>, destructive: bool, disabled: bool) -> ContextMenuItemSpec {
    ContextMenuItemSpec {
        id: id.into(),
        label: Some(label.into()),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
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
    menu.item(layout_context_menu_item(id, label, icon, "addFrame", Some(json!({ "kind": kind })), false, false))
}

/// 🖱️ On-demand layout context menu — canvas, document tree, and preflight surfaces each carry
/// different hit/selection shapes; `organize_context_menu` runs automatically at the host funnel.
fn layout_context_menu_items(
    registry: &AppActionRegistry,
    doc: &LayoutSnapshot,
    labels: &LayoutLabels,
    is_de: bool,
    surface: Option<&ContextMenuSurfaceTarget>,
    fallback_selected: &[String],
) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, Menu};

    let hits = surface.map_or(&[][..], |target| target.hits.as_slice());
    let authoring = layout_authoring_surface(surface);
    let preview = surface.is_some_and(|target| target.surface_id == LAYOUT_PLAY_SURFACE_PREVIEW);

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-document.page.")) {
        if let Some(page_id) = hit.id.strip_prefix("layout-document.page.") {
            return Menu::of(registry).item(layout_context_menu_item("set-active-page", labels.active_page.as_str(), "file", "setActivePage", Some(json!({ "pageId": page_id })), false, false)).build();
        }
    }

    if let Some(hit) = hits.first().filter(|hit| hit.id.starts_with("layout-document.link.")) {
        if let Some(link_id) = hit.id.strip_prefix("layout-document.link.") {
            let ids = layout_link_referencing_frame_ids(doc, link_id);
            let disabled = ids.is_empty();
            return Menu::of(registry)
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
            let issue_value = json!({
                "severity": issue.severity,
                "code": issue.code,
                "message": issue.message,
                "objectId": issue.object_id,
                "pageId": issue.page_id,
            });
            return Menu::of(registry).item(layout_context_menu_item("focus-preflight-issue", labels.preflight.as_str(), "alert-triangle", "focusPreflightIssue", Some(json!({ "issue": issue_value })), false, false)).build();
        }
    }

    let selected = layout_element_ids(surface, fallback_selected);
    let element_hit = hits.iter().find(|hit| hit.domain == LAYOUT_GRANULARITY_ELEMENT);

    if selected.is_empty() {
        if preview {
            return Menu::of(registry).action("selectAll").build();
        }
        let mut menu = Menu::of(registry);
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
        let mut items = menu.action("addPage").action("selectAll").action("paste").destructive("clearSelection").build();
        if let Some(clear) = items.iter_mut().find(|entry| entry.id == "clearSelection") {
            clear.disabled = Some(true);
        }
        return items;
    }

    let phrase = selection_count_phrase(is_de, &[(selected.len(), if is_de { "Rahmen" } else { "frame" }, if is_de { "Rahmen" } else { "frames" })]);
    let mut menu = Menu::of(registry);
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
    let mut items = menu.destructive("clearSelection").build();
    if let Some(clear) = items.iter_mut().find(|entry| entry.id == "clearSelection") {
        clear.disabled = None;
    }
    items
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
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::layout::commands::{
    add_frame, add_page, canvas_drag_leave, canvas_drag_over, canvas_drop, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, delete_selection, engagement_input, engagement_submit, export_package, export_pdf, export_png, export_svg, focus_preflight_issue,
    patch_frame, patch_page, set_active_page, set_camera,
};
//#endregion 🔖️Commands

//#region 🔖️ActionBridge
/// 🎯️ Folds the host's `{action, args}` vocabulary (camelCase keys, JSON floats, control `value`s) into
/// the snake_case `FromValue` payloads of `🎮️commands/*`, one arm per `LayoutCommand` row.
mod args_bridge {
    use super::*;
    use semio_framework_plugin::{FaultCode, FaultOrigin};

    fn snake(key: &str) -> String {
        let mut out = String::with_capacity(key.len() + 4);
        for ch in key.chars() {
            if ch.is_ascii_uppercase() {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    /// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; the `i64`/`u64` codecs
    /// decode EXACT integers only, so whole finite floats get their integer variant back (`f64` fields
    /// accept any `Number` variant, so nothing else changes).
    fn integral(value: DslValue) -> DslValue {
        match value {
            DslValue::Number(dsl::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { DslValue::Number(dsl::Number::UInt(float as u64)) } else { DslValue::Number(dsl::Number::Int(float as i64)) }
            }
            DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    /// 📝️ Prints a host control value into the `String` field the patch/engagement verbs carry
    /// (`"512"`, `"true"`, or the text itself) — the reducers re-parse per field.
    fn stringify(value: DslValue) -> DslValue {
        match value {
            DslValue::String(_) => value,
            DslValue::Null => DslValue::String(String::new()),
            other => DslValue::String(dsl::json::to_json_string(&other)),
        }
    }

    /// 🔁️ Snake-cases every key of `args`, applies `aliases` (snake_case source → destination, first
    /// present source wins and never overwrites a present destination) and seeds `defaults` for keys
    /// still absent.
    fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        if let Some(DslValue::Object(object)) = args {
            for (key, value) in object {
                put(&mut entries, &snake(key), integral(value.clone()));
            }
        }
        for (from, into) in aliases {
            if entries.iter().any(|(key, _)| key == into) {
                continue;
            }
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
                put(&mut entries, into, value);
            }
        }
        for (key, value) in defaults {
            if !entries.iter().any(|(existing, _)| existing == key) {
                entries.push(((*key).to_string(), value.clone()));
            }
        }
        DslValue::Object(entries)
    }

    /// 📷️ The canvas nests its pose under `camera`; a flat `{x, y, zoom}` payload is admitted as the
    /// pose itself.
    fn nest_camera(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if !entries.iter().any(|(key, _)| key == "camera") {
                let pose = DslValue::Object(entries.iter().filter(|(key, _)| matches!(key.as_str(), "x" | "y" | "zoom")).cloned().collect());
                entries.push(("camera".into(), pose));
            }
        }
        folded
    }

    fn with_text_value(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
                slot.1 = stringify(slot.1.clone());
            }
        }
        folded
    }

    fn decode<T: dsl::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("layout action '{action}' arguments do not decode: {error}")))
    }

    fn catalogue_args(action: &str, args: Option<&DslValue>) -> Result<DslValue, Fault> {
        use catalogue_panel::{catalogue_kind, LAYOUT_CATALOGUE_DRAG_MIME, LAYOUT_CATALOGUE_KIND_MIME_PREFIX};
        let invalid = || Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("layout action '{action}' requires one supported catalogue kind"));
        let mut folded = fold(args, &[], &[]);
        let DslValue::Object(entries) = &mut folded else { return Err(invalid()) };
        let field = |name: &str| entries.iter().find_map(|(key, value)| (key == name).then_some(value));
        let kind = if action == "canvasDragOver" {
            let Some(DslValue::Array(types)) = field("types") else { return Err(invalid()) };
            let mut has_base = false;
            let mut kind = None;
            for value in types {
                let DslValue::String(mime) = value else { return Err(invalid()) };
                has_base |= mime == LAYOUT_CATALOGUE_DRAG_MIME;
                if let Some(candidate) = mime.strip_prefix(LAYOUT_CATALOGUE_KIND_MIME_PREFIX) {
                    if kind.is_some() { return Err(invalid()); }
                    kind = Some(catalogue_kind(candidate).ok_or_else(invalid)?);
                }
            }
            if !has_base { return Err(invalid()); }
            kind.ok_or_else(invalid)?
        } else {
            let Some(DslValue::String(raw)) = field("drag_data") else { return Err(invalid()) };
            let value = dsl::json::from_json_str::<DslValue>(raw).map_err(|_| invalid())?;
            let DslValue::Object(payload) = value else { return Err(invalid()) };
            let [(key, DslValue::String(kind))] = payload.as_slice() else { return Err(invalid()) };
            if key != "kind" { return Err(invalid()); }
            catalogue_kind(kind).ok_or_else(invalid)?
        };
        entries.retain(|(key, _)| !matches!(key.as_str(), "types" | "drag_data"));
        put(entries, "kind", DslValue::String(kind.into()));
        Ok(folded)
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<LayoutCommand, Fault> {
        const PAGE: &[(&str, &str)] = &[("id", "page_id"), ("value", "page_id")];
        const TEXT: &[(&str, &str)] = &[("text", "value"), ("input", "value")];
        let text = |value: &str| DslValue::String(value.into());
        let plain = || fold(args, &[], &[]);
        Ok(match action {
            "setActivePage" => LayoutCommand::SetActivePage(decode(action, fold(args, PAGE, &[]))?),
            "focusPreflightIssue" => LayoutCommand::FocusPreflightIssue(decode(action, fold(args, &[("id", "object_id"), ("frame_id", "object_id")], &[]))?),
            "engagementInput" => LayoutCommand::EngagementInput(decode(action, with_text_value(fold(args, TEXT, &[("value", text(""))])))?),
            "canvasPointerDown" => LayoutCommand::CanvasPointerDown(decode(action, fold(args, &[("shift_key", "extend")], &[("button", DslValue::Number(dsl::Number::Int(0))), ("extend", DslValue::Bool(false))]))?),
            "canvasPointerMove" => LayoutCommand::CanvasPointerMove(decode(action, fold(args, &[], &[("samples", DslValue::Array(Vec::new()))]))?),
            "canvasPointerUp" => LayoutCommand::CanvasPointerUp(decode(action, fold(args, &[], &[("cancelled", DslValue::Bool(false))]))?),
            "canvasDragOver" => LayoutCommand::CanvasDragOver(decode(action, catalogue_args(action, args)?)?),
            "canvasDragLeave" => LayoutCommand::CanvasDragLeave(decode(action, plain())?),
            "setCamera" => LayoutCommand::SetCamera(decode(action, nest_camera(plain()))?),
            "addFrame" => LayoutCommand::AddFrame(decode(action, fold(args, &[("value", "kind")], &[("kind", text("rect"))]))?),
            "addPage" => LayoutCommand::AddPage(decode(action, plain())?),
            "patchPage" => LayoutCommand::PatchPage(decode(action, with_text_value(fold(args, &[("id", "page_id")], &[("value", text(""))])))?),
            "patchFrame" => LayoutCommand::PatchFrame(decode(action, with_text_value(fold(args, &[("id", "frame_id")], &[("value", text(""))])))?),
            "canvasDrop" => LayoutCommand::CanvasDrop(decode(action, catalogue_args(action, args)?)?),
            "exportPng" => LayoutCommand::ExportPng(decode(action, fold(args, PAGE, &[]))?),
            "exportSvg" => LayoutCommand::ExportSvg(decode(action, fold(args, PAGE, &[]))?),
            "exportPdf" => LayoutCommand::ExportPdf(decode(action, fold(args, PAGE, &[]))?),
            "exportPackage" => LayoutCommand::ExportPackage(decode(action, plain())?),
            "engagementSubmit" => LayoutCommand::EngagementSubmit(decode(action, with_text_value(fold(args, TEXT, &[("value", text(""))])))?),
            "deleteSelection" => LayoutCommand::DeleteSelection(decode(action, plain())?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the layout editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🧵️RetainedCommands
/// 🧵️ Every shell-reachable verb is a bounded first-step tool (ticket 26/09/16/LAYOUT-PLUGIN-END-TO-END):
/// the framework refuses UI dispatch of any command not classified `Migrated`, so the six content verbs
/// that stayed `BatchOnlyPendingRewrite` (`addFrame`/`addPage`/`patchPage`/`patchFrame` and the pointer
/// down/move gestures) were dead in the running app. Exports keep their own resumable factory.
const LAYOUT_RETAINED_TOOL_IDS: &[&str] = &[
    "setActivePage", "focusPreflightIssue", "engagementInput", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDragOver", "canvasDragLeave", "setCamera", "addFrame", "addPage", "patchPage", "patchFrame", "deleteSelection", "engagementSubmit", "canvasDrop",
];
const LAYOUT_RETAINED_PAYLOAD_SCHEMA: &str = "layout.layout.tool-command.v1";
const LAYOUT_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 16_384;
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
        let mut emit = match input.command {
            LayoutCommand::DeleteSelection(_) => {
                let selected = input.interaction.selection.get(LAYOUT_INTERACTION_ELEMENTS).map(|selection| selection.ids.as_slice()).unwrap_or(&[]);
                delete_selection::apply_frame_ids(&doc, selected)?
            }
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
                    let camera = infinite_canvas::camera::Camera { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom.max(0.0001) };
                    let viewport = infinite_canvas::camera::Viewport { width: payload.width.max(1.0) as u32, height: payload.height.max(1.0) as u32, dpr: 1.0 };
                    let world = infinite_canvas::camera::screen_to_world(&camera, &viewport, infinite_canvas::Point::new(payload.x, payload.y));
                    transient.drop_preview = crate::LayoutDropPreviewState { kind: payload.kind.clone(), x: world.x, y: world.y };
                    window_transient = Some(blueprint::transient::addressed(view, transient)?);
                }
            }
            LayoutCommand::CanvasDragLeave(_) | LayoutCommand::CanvasDrop(_) => {
                let view = addressed_window()?;
                transient.drop_preview = crate::LayoutDropPreviewState::default();
                window_transient = Some(blueprint::transient::addressed(view, transient)?);
                if let LayoutCommand::CanvasDrop(payload) = input.command {
                    if payload.kind == "page" {
                        let page_id = format!("page-{}", input.snapshot.pages.len() + 1);
                        config.active_page_id = page_id;
                        emit.window_config_mutations.push(blueprint::config::addressed(view, config)?);
                    }
                }
            }
            LayoutCommand::AddFrame(_) | LayoutCommand::PatchPage(_) | LayoutCommand::PatchFrame(_) | LayoutCommand::DeleteSelection(_) | LayoutCommand::CanvasPointerDown(_) | LayoutCommand::CanvasPointerMove(_) | LayoutCommand::CanvasPointerUp(_) | LayoutCommand::EngagementSubmit(_) => {}
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
    const DOCUMENT_SCHEMA: &'static str = crate::LAYOUT_DOCUMENT_SCHEMA;
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
        tools: ["setActivePage", "focusPreflightIssue", "engagementInput", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasDragOver", "canvasDragLeave", "setCamera", "addFrame", "addPage", "patchPage", "patchFrame", "deleteSelection", "engagementSubmit", "canvasDrop"]
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
    port: String,
    media: Option<Media>,
    snapshot: Option<std::sync::Arc<LayoutSnapshot>>,
    history: Option<std::sync::Arc<semio_framework_plugin::HistoryView>>,
    mutations: Vec<LayoutMutation>,
    decoded: bool,
    completed: bool,
    closing: bool,
    completion: Option<ArtifactToolCompletion<EditorApp<LayoutPlayApp>>>,
    pending_completion_rejection: Option<semio_framework_plugin::app::ArtifactToolCompletionRejection<EditorApp<LayoutPlayApp>>>,
}

fn layout_job_payload(cx: &mut StepContext<'_>, stream: JobPayloadStream, bytes: &[u8]) -> RetainedJobPayload {
    match cx.payload_from_bytes(stream, bytes) {
        Ok(payload) => payload,
        Err(rejected) => {
            drop(rejected.into_source());
            RetainedJobPayload::empty(stream)
        }
    }
}

fn layout_job_fault(cx: &mut StepContext<'_>, detail: &str) -> StepOutcome {
    let bytes = detail.as_bytes();
    let bounded = &bytes[..bytes.len().min(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)];
    StepOutcome::Fault(JobFault { detail: layout_job_payload(cx, JobPayloadStream::Fault, bounded) })
}

impl LayoutImportJob {
    fn new(request: ArtifactReservedToolJobRequest<EditorApp<LayoutPlayApp>>, port: String, media: Media) -> Self {
        Self {
            port,
            media: Some(media),
            snapshot: Some(request.snapshot),
            history: Some(request.history),
            mutations: Vec::new(),
            decoded: false,
            completed: false,
            closing: false,
            completion: Some(request.completion),
            pending_completion_rejection: None,
        }
    }

    fn decode(&mut self, cx: &mut StepContext<'_>) -> Option<StepOutcome> {
        if self.port != LAYOUT_IMPORT_PORT {
            return Some(layout_job_fault(cx, "layout import only implements fields:in"));
        }
        let decoded = {
            let (Some(media), Some(snapshot), Some(history)) = (self.media.as_ref(), self.snapshot.as_ref(), self.history.as_ref()) else {
                return Some(layout_job_fault(cx, "layout import lost its media, snapshot or history authority"));
            };
            let doc = ArtifactView::new(snapshot.as_ref(), history.as_ref());
            LayoutPlayApp::import_media(LAYOUT_IMPORT_PORT, media, &doc)
        };
        match decoded {
            Ok(emit) => {
                self.mutations = emit.artifact_mutations;
                self.decoded = true;
                None
            }
            Err(error) => Some(layout_job_fault(cx, &error.to_string())),
        }
    }
}

impl InteractiveJob for LayoutImportJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.pending_completion_rejection.is_some() {
            return layout_job_fault(cx, "layout import completion remains rejected");
        }
        if !self.decoded {
            cx.set_stage("layout-import-decode");
            if let Some(outcome) = self.decode(cx) {
                return outcome;
            }
            cx.consume_fuel(1);
            return StepOutcome::CheckpointReady(Checkpoint { state: layout_job_payload(cx, JobPayloadStream::CheckpointState, &[1]), applied_progress: 1 });
        }
        cx.set_stage("layout-import-publish");
        if !self.completed {
            let mutations = std::mem::take(&mut self.mutations);
            let Some(completion) = self.completion.as_ref() else {
                return layout_job_fault(cx, "layout import lost its completion authority");
            };
            if !completion.has_mounted_consumer() {
                return layout_job_fault(cx, "layout import completion consumer is absent");
            }
            if let Err(rejected) = completion.complete(Ok(Emit { artifact_mutations: mutations, ui_scope: semio_framework::kernel::UiDirtyScope::Full, ..Default::default() }), semio_framework_plugin::EphemeralEmit::default()) {
                let message = rejected.fault.message.clone();
                self.pending_completion_rejection = Some(rejected);
                return layout_job_fault(cx, &message);
            }
            self.completed = true;
        }
        StepOutcome::Complete(CommitCandidate { state: RetainedJobPayload::empty(JobPayloadStream::CommitState), output: RetainedJobPayload::empty(JobPayloadStream::CommitOutput) })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        match ArtifactReservedJob::close_step(self, maximum_items, maximum_bytes) {
            Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes }) => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
            Ok(semio_framework_plugin::PluginCloseStep::AwaitingInput { .. } | semio_framework_plugin::PluginCloseStep::Blocked { .. }) | Err(_) => semio_framework_job::InteractiveJobCloseStep::Blocked,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) if ArtifactReservedJob::terminal_is_empty(self) => semio_framework_job::InteractiveJobCloseStep::Complete,
            Ok(semio_framework_plugin::PluginCloseStep::Complete) => semio_framework_job::InteractiveJobCloseStep::Blocked,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        ArtifactReservedJob::terminal_is_empty(self)
    }
}

impl ArtifactReservedJob for LayoutImportJob {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if maximum_items == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(rejected) = self.pending_completion_rejection.as_mut() {
            if let Ok(emit) = rejected.emit.as_mut() {
                if let Some(step) = emit.close_child_one(maximum_items, maximum_bytes) {
                    return Ok(step);
                }
            }
            self.pending_completion_rejection = None;
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        // 🧊️ Every `LayoutMutation` variant wraps plain owned text/floats (no `Dictionary`/`Tree`
        // payload rejects a bare drop), so a popped mutation closes on drop.
        if self.mutations.pop().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.mutations.capacity() > 0 {
            self.mutations = Vec::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.media.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !self.port.is_empty() || self.port.capacity() > 0 {
            self.port = String::new();
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.history.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.snapshot.as_ref().is_some_and(|snapshot| std::sync::Arc::strong_count(snapshot) == 1) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "layout import snapshot has no mounted retained authority" });
        }
        if self.snapshot.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.completion.as_ref().is_some_and(|completion| !completion.has_mounted_consumer()) {
            return Ok(semio_framework_plugin::PluginCloseStep::Blocked { reason: "layout import completion has no mounted consumer authority" });
        }
        if self.completion.take().is_some() {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.port.is_empty()
            && self.port.capacity() == 0
            && self.media.is_none()
            && self.snapshot.is_none()
            && self.history.is_none()
            && self.mutations.is_empty()
            && self.mutations.capacity() == 0
            && self.completion.is_none()
            && self.pending_completion_rejection.is_none()
    }
}
//#endregion 🎞️ReservedImport

fn layout_build_export_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<LayoutPlayApp>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
    use crate::editor::layout::engine::export::{LayoutExportKind, LayoutExportRequest, LayoutExportToolPayload};
    let (kind, page_id) = match *request.command {
        LayoutCommand::ExportPng(payload) => (LayoutExportKind::Png, payload.page_id.or_else(|| Some(blueprint::config::from_snapshot(request.window_config.as_ref()).active_page_id))),
        LayoutCommand::ExportSvg(payload) => (LayoutExportKind::Svg, payload.page_id.or_else(|| Some(blueprint::config::from_snapshot(request.window_config.as_ref()).active_page_id))),
        // 📕️ An unaddressed PDF is the whole document (every page); PNG/SVG stay single-page.
        LayoutCommand::ExportPdf(payload) => (LayoutExportKind::Pdf, payload.page_id),
        LayoutCommand::ExportPackage(_) => (LayoutExportKind::Package, None),
        _ => return Ok(None),
    };
    if request.tool_id != kind.tool_id() {
        return Err(Fault::from("layout-export-command-tool-mismatch"));
    }
    let canonical_base_revision_hex = request.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join("");
    // 📤️ A queue of the job's OWN, never `request.output_chunks`: the mounted operation keeps a clone
    // of the request's queue and drains it at retirement (`MountedTypedCommandFullOperation::
    // retirement_step`), which emptied every download right after the host's ACK — a 0-byte `Demo.pdf`
    // (ticket 26/09/18/LAYOUT-PDF-EXPORT-END-TO-END). Same shape as puzzle's `export_fixture`.
    drop(request.output_chunks);
    let payload = LayoutExportToolPayload {
        request: LayoutExportRequest { kind, page_id, snapshot: request.snapshot, preflight_json: None, parent_document_id: request.parent_document_id, canonical_base_revision_hex },
        output_chunks: semio_framework_plugin::app::ArtifactOutputChunks::new(crate::editor::layout::engine::export::MAX_LAYOUT_EXPORT_OUTPUT_BYTES),
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
        let config = blueprint::config::current(cfg);
        let labels = layout_labels(view_state);
        let mut engine = LayoutEngine::new();
        match body_key {
            LAYOUT_PLAY_BODY_BLUEPRINT => blueprint::render(&mut engine, document, &config, transient, interaction),
            LAYOUT_PLAY_BODY_PREVIEW => preview::render(&mut engine, document, &config, transient, interaction),
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

    const DIALECT: Dialect = crate::LAYOUT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = crate::LAYOUT_DOCUMENT_SCHEMA;

    fn initial_snapshot() -> LayoutSnapshot {
        crate::standards::v1::subsets::any::schema::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(crate::editor::layout::engine::layout_io())
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
    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    /// 🧾️ Store publication authority for the `Artifact` lane — without it the host refuses every
    /// document verb at dispatch (`declares the unsupported artifact publication lane`). One retained
    /// mutation is bounded by `LAYOUT_ARTIFACT_MUTATION_MAXIMUM_BYTES` (`CreatePage` carries a whole
    /// page with its layers; `ChangeDataFields` a `fields:in` dictionary).
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("layout-artifact-retained", LAYOUT_ARTIFACT_MUTATION_MAXIMUM_BYTES))
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
            return Err(Fault::from("layout-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(LayoutWindowWork::new(tool_id));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
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
        )?;
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

    fn build_media_export_job(request: ArtifactMediaExportJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        use crate::editor::layout::engine::export::{LayoutExportJob, LayoutExportKind, LayoutExportRequest, LayoutMediaExportJob, LAYOUT_MEDIA_EXPORT_TOOL_ID};
        if request.port != "layout:out" || request.tool_id != LAYOUT_MEDIA_EXPORT_TOOL_ID {
            return Ok(None);
        }
        let page_id = request.snapshot.pages.first().map(|page| page.id.clone()).ok_or_else(|| Fault::from("layout-export-page-missing"))?;
        let canonical_base_revision_hex = request.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect::<Vec<_>>().join("");
        let export_request = LayoutExportRequest { kind: LayoutExportKind::Svg, page_id: Some(page_id), snapshot: request.snapshot, preflight_json: None, parent_document_id: request.parent_document_id, canonical_base_revision_hex };
        let job = LayoutExportJob::new(request.operation, export_request).map_err(Fault::from)?.with_snapshot_close_lease(request.snapshot_close).with_output_chunks(request.output_chunks).with_media_output_credit(request.output_credit);
        Ok(Some(ArtifactReservedToolJob::new(LayoutMediaExportJob::new(job, request.completion))))
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
        let mut emit = match command {
            LayoutCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction)?,
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
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Structured { schema: crate::LAYOUT_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "layout:out" => Err(MediaError::NotImplemented),
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `fields:in` binds the incoming `form.dictionary`
    /// values into `LayoutSnapshot::data_fields_json` — layout has no existing text-interpolation/
    /// field-binding concept for frames/stories yet, so this stores the dictionary verbatim as a new
    /// named data source (see `crate::LayoutSnapshot::data_fields_json`'s doc) rather
    /// than wiring it into rendering today.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, LayoutSnapshot>) -> Result<Emit<LayoutMutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "fields:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "fields:in only accepts a Structured (JSON object) payload".into()));
                };
                Ok(Emit::mutations(vec![LayoutMutation::ChangeDataFields(ChangeDataFields { new_json: Some(json.clone()) })]))
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
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(body_key, doc, cfg, view_state, &blueprint::transient::current(transient), &LayoutInteractionSnapshot::from_interaction(interaction))
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, LayoutSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
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
        let is_de = view_state.locale == semio_framework_plugin::Locale::De;
        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), is_de, request.surface.as_ref(), &[])
    }

    fn context_menu_with_request_context(
        request: &ContextMenuRequest,
        doc: &ArtifactView<'_, LayoutSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        let is_de = view_state.locale == semio_framework_plugin::Locale::De;
        let fallback = LayoutInteractionSnapshot::from_interaction(interaction).ids;
        layout_context_menu_items(registry, doc.snapshot, layout_labels(view_state), is_de, request.surface.as_ref(), &fallback)
    }
}
//#endregion 🔖️LayoutPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_layout_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::LAYOUT_DIALECT)
            .artifact_kind(ArtifactKindSpec {
                id: "2d.layout".into(),
                name: "Layout".into(),
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
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection") })
            .action_destructive("deleteSelection")
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
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasDrop", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            // 📇️ Per-window action scoping — the content-authoring operations only make sense on the
            // interactive Blueprint surface; the read-only Preview surface renders output and never
            // creates or edits frames/pages. Exports, camera, pointer/drag, selection and hover are
            // surface-discriminated (via `surfaceId`) or global, so they stay unscoped orphans and
            // appear on both windows.
            .window_kind_action_refs(LAYOUT_PLAY_WINDOW_BLUEPRINT, vec![
                "addFrame".into(), "addPage".into(), "patchPage".into(), "patchFrame".into(),
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
            .io(crate::editor::layout::engine::layout_io())
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
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
