//! 🎮️ Layout editor commands: the canvas, frame, page, document, export and engagement commands with the edit mode's windows, the canvas and the terminology.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

use semio_s_artifact_layout_layout_model::Frame;
use semio_s_artifact_layout_layout_mutations_content::change_data_fields::ChangeDataFields;
use semio_s_artifact_layout_layout_mutations::LayoutMutation;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework::kernel::Effect;
use semio_framework_job::{InteractiveJob, StepContext};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::app::{ArtifactMediaExportJobRequest, ArtifactOwnedToolJobRequest, ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry};
use semio_framework_plugin::ArtifactReservedJob;
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
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiFixedMap;
use semio_framework_plugin::UiText;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;
pub const LAYOUT_PLAY_APP_ID: &str = "s.layout.layout@1/*#editor";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn layout_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(LAYOUT_PLAY_APP_ID).action(action, args)
}

/// 🏷️ Admits display text into the bounded semantic label contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "layout label admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
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
pub fn layout_select_action_args(ids: &[String], merge: &str) -> DslValue {
    let targets: Vec<DslValue> = ids.iter().map(|id| semio_framework::dsl_value!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })).collect();
    semio_framework::dsl_value!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "targets": semio_framework_pack_json::to_json_string(&targets), "merge": merge, "method": "pick" })
}

/// 🐁️ Builds `interactionHover`'s JSON args for the `"pointer"` channel — `id: None` clears hover.
pub fn layout_hover_action_args(id: Option<&str>) -> DslValue {
    let targets: Vec<DslValue> = id.map(|id| vec![semio_framework::dsl_value!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })]).unwrap_or_default();
    semio_framework::dsl_value!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "channel": "pointer", "targets": semio_framework_pack_json::to_json_string(&targets) })
}

/// 🕹️ Wraps [`layout_select_action_args`] into the redispatch effect a canvas gesture's own `handle`
/// returns — `dispatch_action` intercepts the six framework interaction verbs BEFORE routing to
/// `ArtifactApp::handle`, so a plain config mutation can no longer express a selection change; the
/// app asks the host to redispatch `interactionSelect` instead (master doc: "surfaces do geometric
/// hit-testing and emit one batched `interactionSelect`").
pub fn layout_select_effect(ids: &[String], merge: &str) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(115), action: INTERACTION_SELECT_ACTION_ID.into(), args: Some(layout_select_action_args(ids, merge)), delay_ms: 0 }
}

/// 🐁️ Wraps [`layout_hover_action_args`] the same way, for `interactionHover`.
pub fn layout_hover_effect(id: Option<&str>) -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(114), action: INTERACTION_HOVER_ACTION_ID.into(), args: Some(layout_hover_action_args(id)), delay_ms: 0 }
}

/// 🕹️ Clicking empty canvas clears every domain's selection — `clearSelection` takes no `domainId`.
pub fn layout_clear_selection_effect() -> Effect {
    Effect::DispatchAction { req: semio_framework_plugin::RequestId(113), action: CLEAR_SELECTION_ACTION_ID.into(), args: None, delay_ms: 0 }
}

/// 🛍️ Page and frame tools, then every placeable 2D artifact. The third column is the native artifact kind stored on the link.
pub const NATIVE_PLACEMENTS: &[(&str, &str, &str)] = &[
    ("png", "image", "s.stdio.png"),
    ("jpg", "image", "s.stdio.jpg"),
    ("gif", "image", "s.stdio.gif"),
    ("bmp", "image", "s.stdio.bmp"),
    ("tiff", "image", "s.stdio.tiff"),
    ("pdf", "file-text", "s.stdio.pdf"),
    ("svg", "spline", "s.stdio.svg"),
    ("drawing", "pen-tool", "s.draw.drawing"),
    ("dwg", "pen-tool", "s.stdio.dwg"),
    ("dxf", "pen-tool", "s.stdio.dxf"),
    ("raster", "image", "s.raster.raster"),
    ("bitmap", "image", "s.wfc.bitmap"),
    ("cad", "pen-tool", "s.cad.cad"),
    ("map", "map", "s.gis.gismap"),
    ("fem2d", "spline", "s.fem.fem2d"),
    ("note", "file-text", "s.note.note"),
    ("writer", "type", "s.writer.writer"),
    ("forms", "file-text", "s.forms.forms"),
    ("flow", "spline", "s.flow.flow"),
    ("equation", "type", "s.mathematical.equation"),
    ("puzzle2d", "square", "s.puzzle.puzzle2d"),
    ("block2d", "square", "s.block.block2d"),
    ("generation2d", "spline", "s.procedural.generation2d"),
    ("terrain", "map", "s.gis.gisterrain"),
    ("grid2d", "image", "s.wfc.grid2d"),
    ("wfc2d", "image", "s.wfc.wfc2d"),
    ("presentation", "file", "s.animate.presentation"),
    ("sequence", "file", "s.sequence.sequence"),
    ("wires", "spline", "s.reasoning.wires"),
];

pub fn native_artifact_kind(kind: &str) -> Option<&'static str> {
    NATIVE_PLACEMENTS.iter().find(|(candidate, _, _)| *candidate == kind).map(|(_, _, artifact_kind)| *artifact_kind)
}

/// 📂 Kind, standard, subset, and document schema the shell uses to open that artifact's editor.
pub const NATIVE_OPEN: &[(&str, &str, &str, &str)] = &[
    ("s.stdio.png", "1.2", "*", "stdio.png"),
    ("s.stdio.jpg", "jfif-1.01", "*", "stdio.jpg"),
    ("s.stdio.gif", "89a", "*", "stdio.gif"),
    ("s.stdio.bmp", "v3", "*", "stdio.bmp"),
    ("s.stdio.tiff", "6.0", "*", "stdio.tiff"),
    ("s.stdio.pdf", "1.7", "*", "stdio.pdf"),
    ("s.stdio.svg", "1.1", "*", "stdio.svg"),
    ("s.draw.drawing", "1", "*", "drawing.document"),
    ("s.stdio.dwg", "ac1018", "*", "stdio.dwg"),
    ("s.stdio.dxf", "r12", "*", "stdio.dxf"),
    ("s.raster.raster", "1", "*", "raster.document"),
    ("s.wfc.bitmap", "1", "*", "s.wfc.bitmap"),
    ("s.cad.cad", "1", "*", "cad.document"),
    ("s.gis.gismap", "1", "*", "gis.map"),
    ("s.fem.fem2d", "1", "*", "fem.2d"),
    ("s.note.note", "1", "*", "note.document"),
    ("s.writer.writer", "1", "*", "writer.document"),
    ("s.forms.forms", "1", "*", "forms.form"),
    ("s.flow.flow", "1", "*", "flow.host_snapshot"),
    ("s.mathematical.equation", "1", "*", "semio.equation/v1"),
    ("s.puzzle.puzzle2d", "1", "*", "board.ports.directed.v1"),
    ("s.block.block2d", "1", "*", "block.2d"),
    ("s.procedural.generation2d", "1", "*", "generation.2d"),
    ("s.gis.gisterrain", "1", "*", "gis.terrain"),
    ("s.wfc.grid2d", "1", "*", "s.wfc.grid2d"),
    ("s.wfc.wfc2d", "1", "*", "s.wfc.wfc2d"),
    ("s.animate.presentation", "1", "*", "animate.presentation"),
    ("s.sequence.sequence", "1", "*", "sequence.sequence"),
    ("s.reasoning.wires", "1", "*", "reasoning.wires.identity.snapshot"),
];

pub fn native_open(artifact_kind: &str) -> Option<(&'static str, &'static str, &'static str)> {
    NATIVE_OPEN.iter().find(|(kind, _, _, _)| *kind == artifact_kind).map(|(_, standard, subset, schema)| (*standard, *subset, *schema))
}

//#region 🪢️TaxonomyMounts
#[path = "🖼️add-frame/🦀️.rs"]
pub mod add_frame;
#[path = "➕️add-page/🦀️.rs"]
pub mod add_page;
#[path = "../🖼️canvas/🦀️.rs"]
pub mod canvas;
#[path = "🚪️canvas-drag-leave/🦀️.rs"]
pub mod canvas_drag_leave;
#[path = "🛬️canvas-drag-over/🦀️.rs"]
pub mod canvas_drag_over;
#[path = "📥️canvas-drop/🦀️.rs"]
pub mod canvas_drop;
#[path = "👇️canvas-pointer-down/🦀️.rs"]
pub mod canvas_pointer_down;
#[path = "↔️canvas-pointer-move/🦀️.rs"]
pub mod canvas_pointer_move;
#[path = "👆️canvas-pointer-up/🦀️.rs"]
pub mod canvas_pointer_up;
#[path = "🗑️delete-selection/🦀️.rs"]
pub mod delete_selection;
#[path = "🧭️engagement-input/🦀️.rs"]
pub mod engagement_input;
#[path = "📤️engagement-submit/🦀️.rs"]
pub mod engagement_submit;
#[path = "📦️export-package/🦀️.rs"]
pub mod export_package;
#[path = "📕️export-pdf/🦀️.rs"]
pub mod export_pdf;
#[path = "🌄️export-png/🦀️.rs"]
pub mod export_png;
#[path = "🎨️export-svg/🦀️.rs"]
pub mod export_svg;
#[path = "🧭️gumball/🦀️.rs"]
pub mod gumball;
#[path = "📄️patch-document/🦀️.rs"]
pub mod patch_document;
#[path = "🩹️patch-frame/🦀️.rs"]
pub mod patch_frame;
#[path = "🧵️patch-page/🦀️.rs"]
pub mod patch_page;
#[path = "🔄️rotate-selection/🦀️.rs"]
pub mod rotate_selection;
#[path = "📏scale-selection/🦀️.rs"]
pub mod scale_selection;
#[path = "📑️set-active-page/🦀️.rs"]
pub mod set_active_page;
#[path = "📷️set-camera/🦀️.rs"]
pub mod set_camera;
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
//#endregion 🪢️TaxonomyMounts

//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod modes {
    #[path = "../🎭️modes/✏️edit/🦀️.rs"]
    pub mod edit;
}
//#endregion 🪢️TaxonomyMounts
