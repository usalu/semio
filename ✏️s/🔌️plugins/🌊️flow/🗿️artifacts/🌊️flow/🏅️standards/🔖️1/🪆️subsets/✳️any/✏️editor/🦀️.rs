//! 🖥️ Flow play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `☑️options/*`, panel trees in `📌️panels/*`,
//! labels in `🗣️terminology/🦀️.rs`, view state in `🎚️config/🦀️.rs`, plugin registration
//! and `FlowHost` bridging (below — constitutional: general, an artifact must never depend on an app, so
//! both live here rather than under `🗿️artifacts`).
//! This file is a routing table: `handle` → `FlowCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::editor::flow::commands::{
    add_widget, connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, evaluate, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport,
    open_spotlight, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_active_example, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode,
    set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension,
};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::editor::flow::modes::edit::windows::{compiled, main};
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
use crate::editor::flow::modes::{edit, generate};
use crate::editor::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::flow::presence::{FlowPresence, FlowPresenceMutation};
use crate::editor::flow::terminology::{flow_play_labels, FlowPlayLabels};
use crate::{FlowMutation, FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_artifact_flow_flow::{CameraJson, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_infinite_dag::DagDrawLod;
use semio_framework_plugin::app::{ChildEmit,ChildEmitPreparation, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_plugin::WindowMeasure;
#[cfg(test)]
use serde_json::json;
use semio_framework::kernel::UiDirtyScope;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, FlowNode, PortRef, SemioFlowSnapshot};
#[cfg(test)]
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use semio_framework_2d::compute::EngineHandles;

#[path = "🧵️retained/🦀️.rs"]
mod retained;

#[cfg(test)]
#[path = "🫧️transient/🧪️tests/🫧️transient/🦀️.rs"]
mod transient_retirement_tests;

//#region 🔖️Constants
pub const FLOW_PLAY_APP_ID: &str = "flow-play";
pub use catalogue_panel::FLOW_PLAY_BODY_CATALOGUE;
pub use compiled::FLOW_PLAY_BODY_COMPILED;
pub use document_panel::FLOW_PLAY_BODY_ARTIFACT;
pub use form::FLOW_PLAY_BODY_GENERATE_FORM;
pub use generations::FLOW_PLAY_BODY_GENERATIONS;
pub use inspection_panel::FLOW_PLAY_BODY_INSPECTOR;
pub use main::FLOW_PLAY_BODY_MAIN;
pub use preview::FLOW_PLAY_BODY_GENERATE_PREVIEW;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn flow_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(FLOW_PLAY_APP_ID).action(action, args)
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
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ The framework's one node-list admission — the crate-local copy is gone (ticket
/// 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING §8.3).
pub use semio_framework_plugin::ui_node_list;

/// 🕹️ A domain pick row: [`semio_framework_plugin::tree_item_desc`] plus the granularity that marks
/// the row a pick target of the panel tree's `.interaction_domain(..)`, so no per-row
/// `interactionSelect` argument map is ever built.
pub fn pick_item<I: AsRef<str>, L: TryInto<semio_framework_ui_contract::Label>>(
    id: I,
    label: L,
    description: Option<String>,
    granularity: &str,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut node = semio_framework_plugin::tree_item_desc(id, label, description)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.granularity = Some(semio_framework_plugin::UiText::try_from_str(granularity).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "interaction granularity admission failed"))?);
    }
    Ok(node)
}

/// 🙈️ An action that exists for dispatch but never appears in the command palette.
fn flow_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the single "graph" interaction
/// domain this app declares — node/edge/handle granularities over the node-graph canvas.
pub const FLOW_INTERACTION_GRAPH: &str = "graph";

/// 🕹️ The document panel tree's own row id prefix for "node"-granularity targets (widgets) — see
/// `document_panel::render`'s doc comment; `interaction_topology` registers the SAME ids.
pub const FLOW_GRAPH_NODE_TARGET_PREFIX: &str = "flow-play-document.widget.";
/// 🕹️ Same as `FLOW_GRAPH_NODE_TARGET_PREFIX`, for "edge"-granularity targets (synapses).
pub const FLOW_GRAPH_EDGE_TARGET_PREFIX: &str = "flow-play-document.synapse.";
/// 🕹️ The scoped prefix for transient graph handle targets published by the canvas.
pub const FLOW_GRAPH_HANDLE_TARGET_PREFIX: &str = "flow-play-document.handle.";

/// 🕹️ The "graph" domain's row id for a widget (node granularity).
pub fn flow_graph_node_target_id(widget_id: &str) -> String {
    format!("{FLOW_GRAPH_NODE_TARGET_PREFIX}{widget_id}")
}

/// 🕹️ The "graph" domain's row id for a synapse (edge granularity).
pub fn flow_graph_edge_target_id(synapse_id: &str) -> String {
    format!("{FLOW_GRAPH_EDGE_TARGET_PREFIX}{synapse_id}")
}

/// 🕹️ Splits the "graph" domain's live `InteractionTarget` ids into (widget ids, synapse ids) — the
/// reverse of `flow_graph_node_target_id`/`flow_graph_edge_target_id`, mirroring note's
/// `block_id_from_tree_row_id`. "handle" targets have no persisted document data to resolve against —
/// no live UI populates them yet (the shared `NodeGraph` canvas renderer that would is framework layer,
/// unmigrated this wave) — so they never appear in either returned list.
pub fn flow_graph_selection_domains(selected: &[String]) -> (Vec<String>, Vec<String>) {
    let nodes = selected.iter().filter_map(|id| id.strip_prefix(FLOW_GRAPH_NODE_TARGET_PREFIX).map(str::to_string)).collect();
    let edges = selected.iter().filter_map(|id| id.strip_prefix(FLOW_GRAPH_EDGE_TARGET_PREFIX).map(str::to_string)).collect();
    (nodes, edges)
}
//#endregion 🔖️Interaction

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `FlowPlayApp::Command` — the SOLE dispatch surface for flow's own behavior, assembled from the
    /// `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`, the
    /// camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different vocabularies, and
    /// is safe, reordering is a wire-format break.**
    ///
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `setSelection`/`clearSelection`/
    /// `selectAll`/`selectNode`/`nodeGraphSelect`/`nodeGraphHover`/`graphPointerDown` are deleted — the
    /// framework auto-injects `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/
    /// `setSelectionMode`/`setInteractionGranularity` for the declared "graph" domain instead (see
    /// `🔖️Manifest`). `deleteSelection`/`focusSelection`/`nodeGraphEdit`/`spotlightCommit` read that
    /// domain's live selection via `InteractionView` — `FlowPlayApp::handle` routes them through their
    /// own `apply` (this macro's generated `dispatch(doc, cfg, session)` has no `interaction` slot).
    pub enum FlowCommand for FlowSnapshot, FlowMutation, NoConfig, NoConfigMutation, ctx = FlowEvalSession {
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "duplicateWidget" as "duplicate-widget" => duplicate_widget::DuplicateWidget,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "disconnect" as "disconnect" => disconnect::Disconnect,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "patchFlowWidgets" as "patch-flow-widgets" => patch_flow_widgets::PatchFlowWidgets,
        "renameFlowWidget" as "rename-flow-widget" => rename_flow_widget::RenameFlowWidget,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "spotlightCommit" as "spotlight-commit" => spotlight_commit::SpotlightCommit,
        "runExtensionAction" as "run-extension-action" => run_extension_action::RunExtensionAction,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "focusSelection" as "focus-selection" => focus_selection::FocusSelection,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setLodMode" as "set-lod-mode" => set_lod_mode::SetLodMode,
        "setProximityDistance" as "set-proximity-distance" => set_proximity_distance::SetProximityDistance,
        "setGridVisible" as "set-grid-visible" => set_grid_visible::SetGridVisible,
        "setGridSnapEnabled" as "set-grid-snap-enabled" => set_grid_snap_enabled::SetGridSnapEnabled,
        "setGridFactor" as "set-grid-factor" => set_grid_factor::SetGridFactor,
        "contextMenuAt" as "context-menu-at" => context_menu_at::ContextMenuAt,
        "setPreviewOff" as "set-preview-off" => set_preview_off::SetPreviewOff,
        "openSpotlight" as "open-spotlight" => open_spotlight::OpenSpotlight,
        "replaceImage" as "replace-image" => replace_image::ReplaceImage,
        "setCatalogueSections" as "set-catalogue-sections" => set_catalogue_sections::SetCatalogueSections,
        "toggleExtension" as "toggle-extension" => toggle_extension::ToggleExtension,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
/// 🖱️ On-demand flow node-graph context menu from surface hit-test and selection snapshot.
fn flow_context_menu_items(registry: &AppActionRegistry, snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, labels: &FlowPlayLabels, view_state: &semio_framework_plugin::ViewModel, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{node_graph_delete_selection_spec, Menu, NodeGraphDeleteDispatch};

    let hits = surface.map_or(&[][..], |target| target.hits.as_slice());
    let groups = surface.map_or(&[][..], |target| target.selection.as_slice());
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "graph" domain's live selection
    // is framework-owned `InteractionState` now, and `ArtifactApp::context_menu` is not threaded an
    // `InteractionView` this wave — there is no config-side fallback left to read, so an empty `surface`
    // (no hit-test/selection groups carried on the request) means no selection, a real known gap rather
    // than a stale-state read.
    let nodes: Vec<String> = groups.iter().filter(|group| group.domain == "node").flat_map(|group| group.ids.iter().cloned()).collect();
    let edges: Vec<String> = groups.iter().filter(|group| group.domain == "edge").flat_map(|group| group.ids.iter().cloned()).collect();
    let has_selection = !nodes.is_empty() || !edges.is_empty();
    let all_preview_off = !nodes.is_empty() && nodes.iter().all(|id| config.preview_off_node_ids.contains(id));
    let live = snapshot.to_host_snapshot();
    let is_image = nodes.len() == 1
        && live.widgets.iter().any(|widget| match widget {
            Widget::InputImage { id, .. } => id == &nodes[0],
            _ => false,
        });
    live.retire_cold();
    let primary = hits.first();
    let hit_node = primary.filter(|hit| hit.domain == "node").map(|hit| hit.id.as_str());

    // 🗂️ Grouped disclosure: `add-node`/`selectAll`/`focusSelection`/`clearSelection` stay top-level
    // (the 3-5 most frequent verbs); `reorganize`/`replaceImage`/`toggle-preview` fold into taxonomy
    // groups; `delete-selection` stays a direct destructive item last (disabled with its reason while nothing is
    // selected) — `organize_context_menu`
    // (applied automatically at the `VcsArtifactApp::context_menu` funnel) sorts the groups into
    // `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive separator itself.
    {
        let mut menu = Menu::of(registry, view_state);
        if hits.is_empty() {
            menu = menu
                .item(ContextMenuItemSpec { id: "add-node".into(), label: Some(labels.add_node.into()), icon: Some("plus".into()), action: Some("openSpotlight".into()), ..Default::default() })
                .action("selectAll")
                .group("transform", |m| m.action("reorganize"));
        }
        if let Some(node_id) = hit_node {
            menu = menu.group("actions", |m| {
                m.item(ContextMenuItemSpec {
                    id: "duplicate-widget".into(),
                    label: Some(labels.duplicate_widget.into()),
                    icon: Some("copy".into()),
                    action: Some("duplicateWidget".into()),
                    args: Some(semio_framework_plugin::dsl_value!({ "widgetId": node_id })),
                    ..Default::default()
                })
            });
            if is_image {
                menu = menu.group("actions", |m| {
                    m.item(ContextMenuItemSpec {
                        id: "replace-image".into(),
                        label: Some(labels.replace_image.into()),
                        icon: Some("image".into()),
                        action: Some("replaceImage".into()),
                        args: Some(semio_framework_plugin::dsl_value!({ "id": node_id })),
                        ..Default::default()
                    })
                });
            }
        }
        if has_selection {
            menu = menu.action("focusSelection").action("clearSelection").group("view", |m| {
                m.item(ContextMenuItemSpec {
                    id: "toggle-preview".into(),
                    label: Some(if all_preview_off { labels.show_preview.into() } else { labels.hide_preview.into() }),
                    icon: Some(if all_preview_off { "eye".into() } else { "eye-off".into() }),
                    checked: Some(!all_preview_off),
                    action: Some("setPreviewOff".into()),
                    args: Some(semio_framework_plugin::dsl_value!({ "ids": nodes, "value": !all_preview_off })),
                    ..Default::default()
                })
            });
        }
        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::Direct)).build()
    }
}
//#endregion 🔖️ContextMenu

//#region 📏️StoreCapacity
const FLOW_STORE_MAX_SCENE_ITEMS: usize = 256;
const FLOW_STORE_MAX_TEXT_BYTES: usize = 16_384;
pub(crate) const FLOW_STORE_MAX_MUTATION_ITEMS: usize = 256;
//#endregion 📏️StoreCapacity

/// 🧮️ The composed content child this document names, read in place for a bounded scan — `None` unless it is
/// composed as `s.stdio.semio@v1/flow`. Every retained scan reads it, never the parent's working-scene owner
/// (see [`crate::flow_composed_snapshot`]).
fn flow_content_child<'a>(snapshot: &FlowSnapshot, children: &'a semio_framework_plugin::ChildContentView) -> Option<store::SnapshotReadRef<'a, SemioFlowSnapshot>> {
    let child_id = &snapshot.content.child_id;
    let dialect = children.dialect("content", child_id)?;
    if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" {
        return None;
    }
    children.typed_read::<SemioFlowSnapshot>("content", child_id).ok()
}

//#region 🧵️DirectStoreLaneRoutes
const FLOW_DIRECT_STORE_TOOL_IDS: &[&str] = &[
    "removeWidget",
    "deleteSelection",
    "disconnect",
    "patchFlowWidgets",
    "duplicateWidget",
    "focusSelection",
    "nodeGraphViewport",
    "setLodMode",
    "setProximityDistance",
    "setGridVisible",
    "setGridSnapEnabled",
    "setGridFactor",
    "setPreviewOff",
    "setCatalogueSections",
    "toggleExtension",
    "addGeneration",
    "removeGeneration",
    "selectGeneration",
    "renameGeneration",
    "updateGenerationValues",
];
const FLOW_DIRECT_STORE_RAW_BYTES: usize = 16_384;

fn flow_direct_store_emit(command: &FlowCommand, config: &FlowMainWindowConfig, view: &semio_framework_plugin::ViewModel) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
    let mut next = config.clone();
    match command {
        FlowCommand::NodeGraphViewport(payload) => {
            next.camera = semio_framework_artifact_flow_flow::CameraJson { x: payload.viewport.x, y: payload.viewport.y, zoom: payload.viewport.zoom };
        }
        FlowCommand::SetLodMode(payload) => {
            if payload.value == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(&payload.value).is_some() {
                next.lod_mode = payload.value.clone();
            }
        }
        FlowCommand::SetProximityDistance(payload) => next.proximity_distance = payload.value.max(0.0),
        FlowCommand::SetGridVisible(payload) => next.grid_visible = payload.pressed.unwrap_or(!config.grid_visible),
        FlowCommand::SetGridSnapEnabled(payload) => next.grid_snap_enabled = payload.pressed.unwrap_or(!config.grid_snap_enabled),
        FlowCommand::SetGridFactor(payload) => next.grid_factor = payload.value.clamp(0.5, 50.0),
        FlowCommand::SetCatalogueSections(payload) => next.catalogue_sections_json = payload.sections_json.clone(),
        FlowCommand::ToggleExtension(payload) => {
            if config.automation_enabled_json.len() > FLOW_STORE_MAX_TEXT_BYTES || payload.id.len() > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-extension-capacity"));
            }
            let mut enabled = serde_json::from_str::<HashMap<String, bool>>(&config.automation_enabled_json).unwrap_or_default();
            if enabled.len() >= FLOW_STORE_MAX_SCENE_ITEMS && !enabled.contains_key(&payload.id) {
                return Err(Fault::from("flow-retained-extension-item-capacity"));
            }
            enabled.insert(payload.id.clone(), payload.enabled);
            next.automation_enabled_json = serde_json::to_string(&enabled).map_err(|_| Fault::from("flow-retained-extension-encode"))?;
        }
        _ => return Err(Fault::from("flow-retained-direct-route-mismatch")),
    }
    Ok(Emit { window_config_mutations: main::config::addressed(view, &config, next)?, ..Default::default() })
}

fn duplicate_widget_id(source: &str, suffix: u64) -> String {
    if suffix == 1 { format!("{source}-copy") } else { format!("{source}-copy-{suffix}") }
}

fn duplicate_edge_id(source: &str, target: &str) -> String {
    format!("{source}-to-{target}")
}

fn evaluate_generation_preview(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, values: &crate::playbook::PlaybookValues) -> String {
    let live = snapshot.to_host_snapshot();
    let snapshot_json = semio_framework_pack_json::to_json_string(&live);
    let values: semio_framework_pack_json::Object = values.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect();
    let patched = flow::forms_bridge::apply_generation_values_to_host_snapshot(&snapshot_json, &values);
    let patched_fixture = match FlowHost::parse_host_snapshot_json(&patched) {
        Ok(parsed) => {
            live.retire_cold();
            parsed
        }
        Err(_) => live,
    };
    let mut host = FlowHost::from_host_snapshot(patched_fixture);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    let preview = host.evaluate().unwrap_or_default();
    host.retire_cold();
    preview
}

fn generation_window_transient(
    command: &FlowCommand,
    snapshot: &FlowSnapshot,
    config: &FlowMainWindowConfig,
    current: &main::transient::FlowWindowTransient,
    view: &semio_framework_plugin::ViewModel,
) -> Result<Option<semio_framework_plugin::WindowTransientMutation>, Fault> {
    let (action, args) = match command {
        FlowCommand::AddGeneration(_) => ("addGeneration", None),
        FlowCommand::RemoveGeneration(payload) => ("removeGeneration", Some(semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone()))]))),
        FlowCommand::SelectGeneration(payload) => ("selectGeneration", Some(semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone()))]))),
        FlowCommand::RenameGeneration(payload) => (
            "renameGeneration",
            Some(semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(payload.id.clone())), ("name".to_string(), semio_framework_value::DslValue::String(payload.name.clone()))])),
        ),
        FlowCommand::UpdateGenerationValues(payload) => (
            "updateGenerationValues",
            Some(semio_framework_value::DslValue::object([
                ("generationId".to_string(), payload.generation_id.clone().map(semio_framework_value::DslValue::String).unwrap_or(semio_framework_value::DslValue::Null)),
                ("questionId".to_string(), semio_framework_value::DslValue::String(payload.question_id.clone())),
                ("value".to_string(), payload.value.clone()),
            ])),
        ),
        _ => return Ok(None),
    };
    let live = snapshot.to_host_snapshot();
    let spec = flow::forms_bridge::flow_host_snapshot_to_form_spec(&live);
    live.retire_cold();
    let mut generation_owner = current.generation().map_err(|error| Fault::from(error.into_message()))?;
    let generation = generation_owner.as_mut();
    if !crate::playbook::handle_generation_action(action, args.as_ref(), generation, &spec, FLOW_PLAY_APP_ID) {
        return Ok(None);
    }
    if matches!(command, FlowCommand::AddGeneration(_) | FlowCommand::SelectGeneration(_) | FlowCommand::UpdateGenerationValues(_)) {
        match crate::playbook::selected_generation(&generation) {
            Some(active) => generation.preview_text = Some(evaluate_generation_preview(snapshot, config, &active.values)),
            None => generation.preview_text = None,
        }
    }
    let mut transient = current.clone();
    transient.generation_json = semio_framework_pack_json::to_json_string(generation);
    main::transient::addressed(view, transient).map(Some)
}

struct FlowDirectStoreWork {
    tool_id: &'static str,
    cursor: usize,
    scan_cursor: usize,
    replay_target: usize,
    replay_scan_target: usize,
    preview_off: Option<Vec<String>>,
    preview_next: Option<Vec<String>>,
    preview_found: bool,
    edge_ids: Option<Vec<String>>,
    node_ids: Option<Vec<String>>,
    duplicate_phase: u8,
    duplicate_source: Option<usize>,
    duplicate_suffix: u64,
    completed: bool,
    closing: bool,
    retirement: retained::Retirement,
}

impl FlowDirectStoreWork {
    fn new(tool_id: &'static str) -> Self {
        Self {
            tool_id,
            cursor: 0,
            scan_cursor: 0,
            replay_target: 0,
            replay_scan_target: 0,
            preview_off: None,
            preview_next: None,
            preview_found: false,
            edge_ids: None,
            node_ids: None,
            duplicate_phase: 0,
            duplicate_source: None,
            duplicate_suffix: 1,
            completed: false,
            closing: false,
            retirement: retained::Retirement::default(),
        }
    }

    fn replaying(&self) -> bool {
        self.cursor < self.replay_target || (self.cursor == self.replay_target && self.scan_cursor <= self.replay_scan_target)
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowDirectStoreWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if command.command_id() != self.tool_id || !FLOW_DIRECT_STORE_TOOL_IDS.contains(&self.tool_id) {
            return None;
        }
        match command {
            FlowCommand::SetPreviewOff(payload) if payload.ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && payload.ids.iter().map(String::len).fold(0, usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES => {
                Some(payload.ids.len().saturating_mul(FLOW_STORE_MAX_SCENE_ITEMS.max(1)).max(1))
            }
            FlowCommand::SetPreviewOff(_) => None,
            FlowCommand::RemoveWidget(_) => flow_content_child(snapshot, &context?.children).filter(|child| child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| child.nodes.len().max(1)),
            FlowCommand::Disconnect(_) => flow_content_child(snapshot, &context?.children).filter(|child| child.edges.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| child.edges.len().max(1)),
            FlowCommand::DeleteSelection(_) => match interaction.selection.get(FLOW_INTERACTION_GRAPH) {
                Some(selection) if selection.ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && selection.ids.iter().map(String::len).fold(0, usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES => {
                    flow_content_child(snapshot, &context?.children).filter(|child| child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS && child.edges.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| {
                        selection
                            .ids
                            .iter()
                            .fold(0usize, |extent, target| {
                                extent.saturating_add(if target.starts_with(FLOW_GRAPH_EDGE_TARGET_PREFIX) {
                                    child.edges.len().max(1)
                                } else if target.starts_with(FLOW_GRAPH_NODE_TARGET_PREFIX) {
                                    child.nodes.len().max(1)
                                } else {
                                    1
                                })
                            })
                            .max(1)
                    })
                }
                Some(_) => None,
                None => Some(1),
            },
            FlowCommand::PatchFlowWidgets(payload)
                if payload.widget_ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && payload.widget_ids.iter().map(String::len).fold(payload.field.len().saturating_add(payload.value.len()), usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES =>
            {
                flow_content_child(snapshot, &context?.children).filter(|child| child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|child| child.nodes.len().saturating_mul(payload.widget_ids.len().max(1)).max(1))
            }
            FlowCommand::PatchFlowWidgets(_) => None,
            FlowCommand::DuplicateWidget(payload) if !payload.widget_id.is_empty() && payload.widget_id.len() <= 256 => {
                let child = flow_content_child(snapshot, &context?.children)?;
                (child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS && child.edges.len() <= FLOW_STORE_MAX_SCENE_ITEMS)
                    .then_some(child.nodes.len().saturating_add(1).saturating_mul(child.nodes.len().saturating_add(child.edges.len()).saturating_add(2)).max(1))
            }
            FlowCommand::DuplicateWidget(_) => None,
            _ => Some(1),
        }
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
        let context = context.ok_or_else(|| Fault::from("flow-window-context-required"))?;
        let view = context.view_state.as_ref().ok_or_else(|| Fault::from("flow-window-view-required"))?;
        let config = main::config::from_snapshot(context.window_config.as_ref());
        if self.completed || self.closing {
            return Err(Fault::from("flow-retained-direct-work-terminal"));
        }
        if let FlowCommand::DuplicateWidget(payload) = command {
            if payload.widget_id.is_empty() || payload.widget_id.len() > 256 {
                return Err(Fault::from("flow-duplicate-widget-id-capacity"));
            }
            let window = context.window_transient.as_ref();
            if window.is_some_and(|window| view.window_id.as_deref() != Some(window.window_id())) {
                return Err(Fault::from("flow-duplicate-window-transient-stale"));
            }
            let child_id = &snapshot.content.child_id;
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-duplicate-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-duplicate-child-capacity"));
            }
            if self.duplicate_phase == 0 {
                if let Some(node) = child.nodes.get(self.cursor) {
                    if node.id == payload.widget_id {
                        self.duplicate_source = Some(self.cursor);
                        self.duplicate_phase = 1;
                        self.scan_cursor = 0;
                    } else {
                        self.cursor += 1;
                    }
                    return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-source", preview: br#"{"en":"Finding source widget","de":"Quell-Widget wird gesucht"}"# });
                }
                self.completed = true;
                return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("duplicateWidget found no widget \"{}\"", payload.widget_id)));
            }
            let source_index = self.duplicate_source.ok_or_else(|| Fault::from("flow-duplicate-source-owner"))?;
            let source = child.nodes.get(source_index).filter(|node| node.id == payload.widget_id).ok_or_else(|| Fault::from("flow-duplicate-source-stale"))?;
            let target_id = duplicate_widget_id(&payload.widget_id, self.duplicate_suffix);
            if target_id.len() > 256 {
                return Err(Fault::from("flow-duplicate-target-id-capacity"));
            }
            if self.duplicate_phase == 1 {
                if let Some(node) = child.nodes.get(self.scan_cursor) {
                    self.scan_cursor += 1;
                    if node.id == target_id {
                        self.duplicate_suffix = self.duplicate_suffix.checked_add(1).ok_or_else(|| Fault::from("flow-duplicate-suffix-overflow"))?;
                        self.scan_cursor = 0;
                    }
                    return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-node-collision", preview: br#"{"en":"Checking node identity","de":"Knotenidentitaet wird geprueft"}"# });
                }
                self.duplicate_phase = 2;
                self.scan_cursor = 0;
                return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-edge-start", preview: br#"{"en":"Checking edge identity","de":"Kantenidentitaet wird geprueft"}"# });
            }
            let edge_id = duplicate_edge_id(&payload.widget_id, &target_id);
            if edge_id.len() > 512 {
                return Err(Fault::from("flow-duplicate-edge-id-capacity"));
            }
            if let Some(edge) = child.edges.get(self.scan_cursor) {
                self.scan_cursor += 1;
                if edge.id == edge_id {
                    self.duplicate_suffix = self.duplicate_suffix.checked_add(1).ok_or_else(|| Fault::from("flow-duplicate-suffix-overflow"))?;
                    self.duplicate_phase = 1;
                    self.scan_cursor = 0;
                }
                return Ok(ArtifactCommandWorkStep::Progress { stage: "flow-duplicate-edge-collision", preview: br#"{"en":"Checking edge identity","de":"Kantenidentitaet wird geprueft"}"# });
            }
            let mut node = source.clone();
            node.id = target_id.clone();
            let edge = FlowEdge { id: edge_id, from: PortRef { node: payload.widget_id.clone(), port: String::new() }, to: PortRef { node: target_id, port: String::new() }, kind: "data".into() };
            let emit = Emit {
                child_preparations: std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot, _>("content", child_id, vec![SemioFlowMutation::InsertNode(insert_node::InsertNode::new(node)), SemioFlowMutation::InsertEdge(insert_edge::InsertEdge::new(edge))])]),
                ..Default::default()
            };
            self.completed = true;
            let Some(window) = window else { return Ok(ArtifactCommandWorkStep::Complete(emit)) };
            let mut transient = main::transient::from_snapshot(Some(window));
            transient.duplicate_widget_progress_json.clear();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![main::transient::addressed(view, transient)?] },
            });
        }
        if let FlowCommand::FocusSelection(_) = command {
            let (nodes, _) = flow_graph_selection_domains(interaction.selection.get(FLOW_INTERACTION_GRAPH).map_or(&[][..], |selection| selection.ids.as_slice()));
            let mut next = config.clone();
            let session = FlowEvalSession::new();
            let composed = crate::flow_composed_snapshot(snapshot, &context.children)?;
            let camera = focus_selection_camera(&composed, &config, &session, &nodes);
            session.retire_cold();
            if let Some(camera) = camera {
                next.camera = camera;
            }
            self.completed = true;
            return Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: main::config::addressed(view, &config, next)?, ..Default::default() }));
        }
        if matches!(
            command,
            FlowCommand::AddGeneration(_)
                | FlowCommand::RemoveGeneration(_)
                | FlowCommand::SelectGeneration(_)
                | FlowCommand::RenameGeneration(_)
                | FlowCommand::UpdateGenerationValues(_)
        ) {
            let window = context.window_transient.as_ref().ok_or_else(|| Fault::from("flow-generation-window-transient-required"))?;
            if view.window_id.as_deref() != Some(window.window_id()) {
                return Err(Fault::from("flow-generation-window-transient-stale"));
            }
            let current = main::transient::from_snapshot(Some(window));
            let composed = crate::flow_composed_snapshot(snapshot, &context.children)?;
            let mutation = generation_window_transient(command, &composed, &config, &current, view)?;
            self.completed = true;
            let emit = Emit::default();
            let ephemeral = semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: mutation.into_iter().collect() };
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral });
        }
        if matches!(command, FlowCommand::RemoveWidget(_) | FlowCommand::Disconnect(_)) {
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            let (length, matched, target, found) = match command {
                FlowCommand::RemoveWidget(payload) => (child.nodes.len(), child.nodes.get(self.cursor).is_some_and(|node| node.id == payload.widget_id), &payload.widget_id, self.node_ids.get_or_insert_with(Vec::new)),
                FlowCommand::Disconnect(payload) => (child.edges.len(), child.edges.get(self.cursor).is_some_and(|edge| edge.id == payload.synapse_id), &payload.synapse_id, self.edge_ids.get_or_insert_with(Vec::new)),
                _ => return Err(Fault::from("flow-retained-direct-route-mismatch")),
            };
            if self.cursor < length {
                if matched {
                    found.push(target.clone());
                    self.cursor = length;
                } else {
                    self.cursor += 1;
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-direct-artifact-scan-replay", preview: br#"{"en":"Restoring artifact scan","de":"Artefaktsuche wird wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-direct-artifact-scan", preview: br#"{"en":"Scanning one artifact item","de":"Ein Artefaktelement wird geprueft"}"# }
                });
            }
            let (node_ids, edge_ids) = (self.node_ids.take().unwrap_or_default(), self.edge_ids.take().unwrap_or_default());
            self.completed = true;
            if node_ids.is_empty() && edge_ids.is_empty() {
                return Err(match command {
                    FlowCommand::RemoveWidget(payload) => semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("removeWidget found no widget \"{}\"", payload.widget_id)),
                    FlowCommand::Disconnect(payload) => semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("disconnect found no synapse \"{}\"", payload.synapse_id)),
                    _ => Fault::from("flow-retained-direct-route-mismatch"),
                });
            }
            return Ok(ArtifactCommandWorkStep::Complete(flow_content_leaves_emit(&snapshot.content.child_id, flow_removal_leaves(&child, &node_ids, &edge_ids))));
        }
        if matches!(command, FlowCommand::DeleteSelection(_)) {
            let selected = interaction.selection.get(FLOW_INTERACTION_GRAPH).map_or(&[][..], |selection| selection.ids.as_slice());
            if selected.len() > FLOW_STORE_MAX_MUTATION_ITEMS || selected.iter().map(String::len).sum::<usize>() > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-delete-selection-capacity"));
            }
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            self.edge_ids.get_or_insert_with(Vec::new);
            self.node_ids.get_or_insert_with(Vec::new);
            if let Some(target) = selected.get(self.cursor) {
                if let Some(id) = target.strip_prefix(FLOW_GRAPH_EDGE_TARGET_PREFIX) {
                    if let Some(edge) = child.edges.get(self.scan_cursor) {
                        self.scan_cursor += 1;
                        let matched = edge.id == id;
                        if matched {
                            self.edge_ids.as_mut().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?.push(id.to_string());
                        }
                        if matched || self.scan_cursor == child.edges.len() {
                            self.cursor += 1;
                            self.scan_cursor = 0;
                        }
                    } else {
                        self.cursor += 1;
                        self.scan_cursor = 0;
                    }
                } else if let Some(id) = target.strip_prefix(FLOW_GRAPH_NODE_TARGET_PREFIX) {
                    if let Some(node) = child.nodes.get(self.scan_cursor) {
                        self.scan_cursor += 1;
                        let matched = node.id == id;
                        if matched {
                            self.node_ids.as_mut().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?.push(id.to_string());
                        }
                        if matched || self.scan_cursor == child.nodes.len() {
                            self.cursor += 1;
                            self.scan_cursor = 0;
                        }
                    } else {
                        self.cursor += 1;
                        self.scan_cursor = 0;
                    }
                } else {
                    self.cursor += 1;
                    self.scan_cursor = 0;
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-delete-selection-replay", preview: br#"{"en":"Restoring selected deletion","de":"Auswahlloeschung wird wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-delete-selection", preview: br#"{"en":"Preparing selected deletion","de":"Auswahlloeschung wird vorbereitet"}"# }
                });
            }
            let edge_ids = self.edge_ids.take().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?;
            let node_ids = self.node_ids.take().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?;
            self.completed = true;
            if node_ids.is_empty() && edge_ids.is_empty() {
                return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.delete-selection-empty"), "deleteSelection needs at least one selected widget or synapse"));
            }
            return Ok(ArtifactCommandWorkStep::Complete(flow_content_leaves_emit(&snapshot.content.child_id, flow_removal_leaves(&child, &node_ids, &edge_ids))));
        }
        if let FlowCommand::SetPreviewOff(payload) = command {
            if self.preview_off.is_none() {
                if config.preview_off_node_ids.len() > FLOW_STORE_MAX_SCENE_ITEMS || config.preview_off_node_ids.iter().map(String::len).sum::<usize>() > FLOW_STORE_MAX_TEXT_BYTES {
                    return Err(Fault::from("flow-retained-preview-off-capacity"));
                }
                self.preview_off = Some(config.preview_off_node_ids.clone());
            }
            if let Some(id) = payload.ids.get(self.cursor) {
                self.preview_next.get_or_insert_with(Vec::new);
                let source_len = self.preview_off.as_ref().ok_or_else(|| Fault::from("flow-retained-preview-off-owner"))?.len();
                if let Some(existing) = self.preview_off.as_ref().and_then(|source| source.get(self.scan_cursor)).cloned() {
                    let matched = existing == *id;
                    self.preview_found |= matched;
                    if payload.value || !matched {
                        self.preview_next.as_mut().ok_or_else(|| Fault::from("flow-retained-preview-off-next-owner"))?.push(existing);
                    }
                    self.scan_cursor += 1;
                }
                if self.scan_cursor == source_len {
                    if payload.value && !self.preview_found {
                        let next = self.preview_next.as_mut().ok_or_else(|| Fault::from("flow-retained-preview-off-next-owner"))?;
                        if next.len() == FLOW_STORE_MAX_SCENE_ITEMS {
                            return Err(Fault::from("flow-retained-preview-off-item-capacity"));
                        }
                        next.push(id.clone());
                    }
                    self.preview_off = self.preview_next.take();
                    self.preview_found = false;
                    self.scan_cursor = 0;
                    self.cursor += 1;
                } else {
                    if self.scan_cursor > source_len {
                        return Err(Fault::from("flow-retained-preview-off-cursor"));
                    }
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-preview-off-replay", preview: br#"{"en":"Restoring preview visibility","de":"Vorschau-Sichtbarkeit wird wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-preview-off", preview: br#"{"en":"Updating preview visibility","de":"Vorschau-Sichtbarkeit wird aktualisiert"}"# }
                });
            }
            let node_ids = self.preview_off.take().ok_or_else(|| Fault::from("flow-retained-preview-off-owner"))?;
            self.completed = true;
            let mut next = config.clone();
            next.preview_off_node_ids = node_ids;
            return Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: main::config::addressed(view, &config, next)?, ..Default::default() }));
        }
        if let FlowCommand::PatchFlowWidgets(payload) = command {
            let input_bytes = payload.widget_ids.iter().map(String::len).fold(payload.field.len().saturating_add(payload.value.len()), usize::saturating_add);
            if payload.widget_ids.len() > FLOW_STORE_MAX_MUTATION_ITEMS || input_bytes > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-patch-widgets-capacity"));
            }
            let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
            if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            if let Some(node) = child.nodes.get(self.cursor) {
                if let Some(id) = payload.widget_ids.get(self.scan_cursor) {
                    self.scan_cursor += 1;
                    let matched = *id == node.id;
                    if matched || self.scan_cursor == payload.widget_ids.len() {
                        self.cursor += 1;
                        self.scan_cursor = 0;
                    }
                } else {
                    self.cursor += 1;
                    self.scan_cursor = 0;
                }
                return Ok(if self.replaying() {
                    ArtifactCommandWorkStep::Replay { stage: "flow-patch-widgets-replay", preview: br#"{"en":"Restoring widget patches","de":"Widget-Aktualisierungen werden wiederhergestellt"}"# }
                } else {
                    ArtifactCommandWorkStep::Progress { stage: "flow-patch-widgets", preview: br#"{"en":"Preparing widget patches","de":"Widget-Aktualisierungen werden vorbereitet"}"# }
                });
            }
            self.completed = true;
            let leaves = patch_flow_widgets::patch_flow_widgets_leaves(&child, payload);
            return Ok(ArtifactCommandWorkStep::Complete(patch_flow_widgets::widget_leaves_emit(&snapshot.content.child_id, leaves)));
        }
        self.completed = true;
        flow_direct_store_emit(command, &config, view).map(ArtifactCommandWorkStep::Complete)
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 34 {
            return Err(Fault::from("flow-retained-direct-checkpoint-capacity"));
        }
        target[0] = u8::from(self.completed);
        target[1..9].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[9..17].copy_from_slice(&(self.scan_cursor as u64).to_le_bytes());
        target[17] = self.duplicate_phase;
        target[18..26].copy_from_slice(&(self.duplicate_source.unwrap_or(usize::MAX) as u64).to_le_bytes());
        target[26..34].copy_from_slice(&self.duplicate_suffix.to_le_bytes());
        Ok(34)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 34 || checkpoint[0] > 1 || checkpoint[17] > 2 {
            return Err(Fault::from("flow-retained-direct-checkpoint-invalid"));
        }
        if self.closing || !self.retirement.is_empty() || self.preview_off.is_some() || self.preview_next.is_some() || self.edge_ids.is_some() || self.node_ids.is_some() {
            return Err(Fault::from("flow-retained-direct-restore-requires-empty-owner"));
        }
        self.completed = checkpoint[0] == 1;
        self.replay_target = usize::try_from(u64::from_le_bytes(checkpoint[1..9].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.replay_scan_target = usize::try_from(u64::from_le_bytes(checkpoint[9..17].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.cursor = if self.tool_id == "duplicateWidget" { self.replay_target } else { 0 };
        self.scan_cursor = if self.tool_id == "duplicateWidget" { self.replay_scan_target } else { 0 };
        self.duplicate_phase = checkpoint[17];
        let duplicate_source = usize::try_from(u64::from_le_bytes(checkpoint[18..26].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.duplicate_source = (duplicate_source != usize::MAX).then_some(duplicate_source);
        self.duplicate_suffix = u64::from_le_bytes(checkpoint[26..34].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?);
        self.preview_off = None;
        self.preview_next = None;
        self.preview_found = false;
        self.edge_ids = None;
        self.node_ids = None;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if !self.retirement.is_empty() {
            return self.retirement.step(maximum_items, maximum_bytes);
        }
        if maximum_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(values) = self.preview_off.take().or_else(|| self.preview_next.take()).or_else(|| self.edge_ids.take()).or_else(|| self.node_ids.take()) {
            self.retirement.push(retained::Owner::Strings(values));
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.retirement.is_empty() && self.preview_off.is_none() && self.preview_next.is_none() && self.edge_ids.is_none() && self.node_ids.is_none()
    }
}

/// 🧾️ The ONE execution contract the direct-Store route declares — returned by the factory and
/// proved by `FlowDirectStoreJobFactoryProofs`, so `validate_tool_job_rows`' `registration.contract
/// == row.contract` join can never drift from a hand-copied literal.
fn flow_direct_store_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(FLOW_DIRECT_STORE_RAW_BYTES, 256, FLOW_STORE_MAX_MUTATION_ITEMS as u64, 16_384, 7_500, 1, 1)
}

struct FlowDirectStoreJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowDirectStoreJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_DIRECT_STORE_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowDirectStoreJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        flow_direct_store_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FLOW_DIRECT_STORE_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("Flow direct Store job rejects oversized wire owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowDirectStoreJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_DIRECT_STORE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnect", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchFlowWidgets", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "duplicateWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child, semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "focusSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setProximityDistance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setPreviewOff", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCatalogueSections", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleExtension", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "selectGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameGeneration", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "updateGenerationValues", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
    ];
}
//#endregion 🧵️DirectStoreLaneRoutes

//#region 🧵️ChildGroupRetainedRoute
const FLOW_CHILD_GROUP_TOOL_IDS: &[&str] = &["addWidget", "moveMediaNode", "nodeGraphEdit", "spotlightCommit"];
const FLOW_CHILD_GROUP_RAW_BYTES: usize = 16_384;

struct FlowChildGroupWork {
    tool_id: &'static str,
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    output: Option<Emit<FlowMutation, NoConfigMutation, NoDraftMutation>>,
    completed: bool,
    closing: bool,
}

impl FlowChildGroupWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner: Some(instance_owner), output: None, completed: false, closing: false }
    }

    fn has_vector_allocation<T>(owner:&Vec<T>)->bool{std::mem::size_of::<T>()!=0&&owner.capacity()!=0}

    fn accept_output(&mut self,emit:Emit<FlowMutation,NoConfigMutation,NoDraftMutation>,child_id:&str)->Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>,Fault>{
        assert!(self.output.is_none(),"one Flow work unit retains at most one original output");
        self.output=Some(emit);
        let emit=self.output.as_ref().expect("original output retained before validation");
        let maximum=FLOW_STORE_MAX_MUTATION_ITEMS+1;
        let ready=emit.child_emits.first().is_some_and(|child|child.slot=="content"&&child.child_id==child_id&&!child.ops.is_empty()&&child.ops.len()<=maximum&&child.labels.len()==child.ops.len());
        let preparing=emit.child_preparations.front().is_some_and(|source|source.matches_source::<SemioFlowMutation>("content",child_id,maximum));
        let exact=(ready&&emit.child_preparations.is_empty())||(preparing&&emit.child_emits.is_empty());
        if emit.child_emits.len()+emit.child_preparations.len()>1
            || ((!emit.child_emits.is_empty()||!emit.child_preparations.is_empty())&&!exact)
            || (emit.transaction.is_some()&&!exact)
            || !emit.artifact_mutations.is_empty()||!emit.config_mutations.is_empty()||!emit.draft_mutations.is_empty()
            || !emit.effects.is_empty()||!emit.events.is_empty()
        {return Err(Fault::from("flow-retained-child-group-output-contract"));}
        self.completed=true;
        Ok(ArtifactCommandWorkStep::Complete(self.output.take().expect("validated original output transferred once")))
    }

    fn admitted_child<'a>(
        command: &FlowCommand,
        snapshot: &'a FlowSnapshot,
        context: Option<&'a semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<store::SnapshotReadRef<'a, SemioFlowSnapshot>> {
        if !matches!(command, FlowCommand::AddWidget(_) | FlowCommand::MoveMediaNode(_) | FlowCommand::NodeGraphEdit(_) | FlowCommand::SpotlightCommit(_)) {
            return None;
        }
        flow_content_child(snapshot, &context?.children)
    }

    fn payload_admitted(command: &FlowCommand) -> bool {
        match command {
            FlowCommand::AddWidget(payload) => {
                let text_bytes = payload.kind.len().checked_add(payload.neuron_kind.as_ref().map_or(0, String::len));
                text_bytes.is_some_and(|bytes| bytes <= FLOW_CHILD_GROUP_RAW_BYTES) && payload.x.unwrap_or(120.0).is_finite() && payload.y.unwrap_or(120.0).is_finite()
            }
            FlowCommand::MoveMediaNode(payload) => !payload.node_id.is_empty() && payload.node_id.len() <= FLOW_STORE_MAX_TEXT_BYTES && payload.x.is_finite() && payload.y.is_finite(),
            FlowCommand::NodeGraphEdit(payload) => payload.operations.len() <= FLOW_STORE_MAX_MUTATION_ITEMS,
            FlowCommand::SpotlightCommit(payload) => payload.operations.len() <= FLOW_STORE_MAX_MUTATION_ITEMS,
            _ => false,
        }
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowChildGroupWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if self.closing || self.completed || command.command_id() != self.tool_id || !FLOW_CHILD_GROUP_TOOL_IDS.contains(&self.tool_id) || !Self::payload_admitted(command) {
            return None;
        }
        let child = Self::admitted_child(command, snapshot, context)?;
        (child.nodes.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && child.edges.len() <= FLOW_STORE_MAX_MUTATION_ITEMS).then_some(1)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config, history, interaction: _interaction, hover: _hover, context, operation } = *input;
        if self.closing || self.completed || self.output.is_some() {
            return Err(Fault::from("flow-retained-child-group-terminal"));
        }
        if command.command_id() != self.tool_id || !Self::payload_admitted(command) {
            return Err(Fault::from("flow-retained-child-group-payload-capacity"));
        }
        let _child = Self::admitted_child(command, snapshot, context).ok_or_else(|| Fault::from("flow-retained-child-group-authority"))?;
        let context = context.ok_or_else(|| Fault::from("flow-retained-child-group-context"))?;
        let view = ArtifactView::with_children(snapshot, history, (*context.children).clone()).bound_to_operation(operation.clone());
        let window_config = main::config::from_snapshot(context.window_config.as_ref());
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-retained-child-group-instance-owner"))?;
        let emit = instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| match command {
                FlowCommand::AddWidget(payload) => add_widget::handle(payload, &view, &ConfigView { snapshot: config, window: context.window_config.as_ref() }, session),
                FlowCommand::MoveMediaNode(payload) => move_media_node::handle(payload, &view, &ConfigView { snapshot: config, window: context.window_config.as_ref() }, session),
                FlowCommand::NodeGraphEdit(payload) => node_graph_edit::node_graph_edit_result(&view, &window_config, session, &payload.operations),
                FlowCommand::SpotlightCommit(payload) => spotlight_commit::node_graph_edit_result(&view, &window_config, session, &payload.operations),
                _ => Err(Fault::from("flow-retained-child-group-route-mismatch")),
            })?
        })?;
        self.accept_output(emit, &snapshot.content.child_id)
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep{
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_plugin::app::PluginCloseStep;
        if !self.closing{return InteractiveJobCloseStep::Blocked;}
        if maximum_items==0||maximum_bytes==0{return InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0};}
        if let Some(emit)=self.output.as_mut(){
            if let Some(step)=emit.close_child_one(1,maximum_bytes){return match step{
                PluginCloseStep::Pending{released_items,released_bytes}=>InteractiveJobCloseStep::Pending{released_items,released_bytes},
                PluginCloseStep::Complete=>InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0},
                _=>InteractiveJobCloseStep::Blocked,
            };}
            if let Some(transaction)=emit.transaction.as_mut(){
                for text in [&mut transaction.id,&mut transaction.tool]{
                    let bytes=text.capacity();if bytes==0{continue;}
                    if bytes>maximum_bytes{return InteractiveJobCloseStep::Pending{released_items:0,released_bytes:0};}
                    *text=String::new();return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:bytes};
                }
                emit.transaction=None;return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};
            }
            if Self::has_vector_allocation(&emit.artifact_mutations)||Self::has_vector_allocation(&emit.config_mutations)||Self::has_vector_allocation(&emit.window_config_mutations)||Self::has_vector_allocation(&emit.draft_mutations)
                ||Self::has_vector_allocation(&emit.effects)||Self::has_vector_allocation(&emit.events)||Self::has_vector_allocation(&emit.extension_invocations)||Self::has_vector_allocation(&emit.interaction_writes)||Self::has_vector_allocation(&emit.tasks)||matches!(emit.ui_scope,semio_framework::kernel::UiDirtyScope::Partial{..})
            {return InteractiveJobCloseStep::Blocked;}
            self.output=None;return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};
        }
        if self.instance_owner.take().is_some(){return InteractiveJobCloseStep::Pending{released_items:1,released_bytes:0};}
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none() && self.output.is_none()
    }
}

struct FlowChildGroupJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowChildGroupJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_CHILD_GROUP_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowChildGroupJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(FLOW_CHILD_GROUP_RAW_BYTES, 256, 1, 16_384, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FLOW_CHILD_GROUP_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Flow child-group job rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowChildGroupJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_CHILD_GROUP_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "spotlightCommit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    ];
}
//#endregion 🧵️ChildGroupRetainedRoute

//#region 🧵️HostOnlyRetainedRoutes
const FLOW_HOST_ONLY_TOOL_IDS: &[&str] = &["evaluate", "contextMenuAt", "openSpotlight", "replaceImage", "flowEvalTick", "flowEvalResolve"];
const FLOW_HOST_ONLY_RAW_BYTES: usize = 16_384;

struct FlowHostEffectPayload {
    command: FlowCommand,
    snapshot: Arc<FlowSnapshot>,
    config: Arc<FlowMainWindowConfig>,
    history: Arc<semio_framework_plugin::HistoryView>,
    children: Arc<semio_framework_plugin::ChildContentView>,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<FlowPlayApp>>,
}

fn flow_scalar_command_view(command: &FlowCommand) -> Result<store::os_pack::ScalarRecordView<'_>, &'static str> {
    use store::os_pack::{ScalarRecordField as Field, ScalarRecordView};
    let ordinal = FlowCommand::TOOL_JOB_IDS.iter().position(|id| *id == command.command_id()).ok_or("Flow scalar command ordinal missing")? as u64;
    let fields = match command {
        FlowCommand::Evaluate(_) | FlowCommand::OpenSpotlight(_) => [None, None, None],
        FlowCommand::ContextMenuAt(command) => [Some(Field::Text(&command.id)), None, None],
        FlowCommand::ReplaceImage(command) => [Some(Field::Text(&command.id)), None, None],
        // 📏️ Three slots, at most two of them text (`🎒️pack/🔎️scalar-witness`) — which is exactly what
        // an ADDRESSED evaluation hop and its answer need, and the reason the answer carries no
        // `extensionId`/`ok`/`fault_*`.
        FlowCommand::FlowEvalTick(command) => [Some(Field::Text(&command.window_id)), Some(Field::Text(&command.window_kind_id)), None],
        FlowCommand::FlowEvalResolve(command) => [Some(Field::Text(&command.window_id)), Some(Field::U64(command.node_hash)), Some(Field::Text(&command.output_json))],
        _ => return Err("Flow command does not have an admitted scalar record witness"),
    };
    Ok(ScalarRecordView { ordinal, fields })
}

fn flow_host_wire_view(payload: &FlowHostEffectPayload) -> Result<store::os_pack::ScalarRecordView<'_>, &'static str> {
    flow_scalar_command_view(&payload.command)
}

#[cfg(test)]
#[path = "🧵️retained/🔎️wire/🧪️tests/🔎️wire/🦀️.rs"]
mod scalar_host_wire_tests;

struct FlowHostEffectJob {
    payload: Option<Arc<FlowHostEffectPayload>>,
    input: Option<semio_framework::action_bus::RetainedToolWireInput>,
    decoder: Option<store::os_pack::ScalarRecordWireWitness<FlowHostEffectPayload>>,
    page: usize,
    byte: usize,
    validated: bool,
    completed: bool,
    closing: bool,
}

impl FlowHostEffectJob {
    fn fault() -> semio_framework_job::StepOutcome {
        semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) })
    }
}

impl semio_framework_job::InteractiveJob for FlowHostEffectJob {
    fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if context.is_cancelled() {
            return semio_framework_job::StepOutcome::Cancelled;
        }
        if context.should_yield() || context.fuel_remaining() == 0 {
            return semio_framework_job::StepOutcome::Yield;
        }
        if !self.validated {
            let Some(input) = self.input.as_ref() else { return Self::fault() };
            let page = input.page(self.page);
            if page.is_some_and(|page| self.byte == page.len()) {
                self.page += 1;
                self.byte = 0;
                context.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            let Some(decoder) = self.decoder.as_mut() else { return Self::fault() };
            match decoder.advance(page.and_then(|page| page.get(self.byte)).copied()) {
                Ok(store::os_pack::ScalarRecordWireStep::Consumed { .. }) => self.byte += 1,
                Ok(store::os_pack::ScalarRecordWireStep::Progress { .. }) => {}
                Ok(store::os_pack::ScalarRecordWireStep::Complete) => self.validated = true,
                Err(_) => return Self::fault(),
            }
            context.consume_fuel(1);
            return semio_framework_job::StepOutcome::Yield;
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return Self::fault() };
            let view = ArtifactView::with_children(payload.snapshot.as_ref(), &payload.history, (*payload.children).clone());
            let emit = payload.instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| match &payload.command {
                    FlowCommand::Evaluate(_) => crate::flow_composed_snapshot(&payload.snapshot, &payload.children).map(|composed| evaluate::evaluate_result(&composed, &payload.config, session, main::FLOW_PLAY_WINDOW_MAIN, main::FLOW_PLAY_WINDOW_MAIN)),
                    FlowCommand::FlowEvalTick(command) => crate::flow_composed_snapshot(&payload.snapshot, &payload.children).map(|composed| flow_eval_tick::tick_result(&composed, &payload.config, session, &command.window_id, &command.window_kind_id)),
                    FlowCommand::FlowEvalResolve(command) => flow_eval_resolve::handle(command, &view, &ConfigView { snapshot: &NoConfig {}, window: None }, session),
                    FlowCommand::ContextMenuAt(_) | FlowCommand::OpenSpotlight(_) | FlowCommand::ReplaceImage(_) => Ok(Emit::default()),
                    _ => Err(Fault::from("flow-host-effect-route-mismatch")),
                })?
            });
            if payload.completion.complete(emit, semio_framework_plugin::EphemeralEmit::default()).is_err() {
                return Self::fault();
            }
            self.completed = true;
            context.consume_fuel(1);
        }
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(input) = self.input.as_mut() {
            input.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if let Some(input) = self.input.as_mut() {
            let step = input.close_step(1, maximum_bytes.min(FLOW_HOST_ONLY_RAW_BYTES));
            if input.terminal_is_empty() {
                self.input = None;
            }
            return match step {
                semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                step => step,
            };
        }
        if let Some(decoder) = self.decoder.as_mut() {
            decoder.begin_close();
            let root = decoder.take_root();
            assert!(self.payload.as_ref().zip(root.as_ref()).is_some_and(|(payload, root)| Arc::ptr_eq(payload, root)));
            drop(root);
            assert!(decoder.terminal_is_empty());
            self.decoder = None;
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.payload.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.payload.is_none() && self.input.is_none() && self.decoder.is_none()
    }
}

struct FlowHostEffectJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowHostEffectJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_HOST_ONLY_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowHostEffectJobFactory {
    type Payload = FlowHostEffectPayload;
    type Job = FlowHostEffectJob;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(FLOW_HOST_ONLY_RAW_BYTES, 256, 1, 16_384, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(FlowHostEffectJob { payload: Some(Arc::new(payload)), input: None, decoder: None, page: 0, byte: 0, validated: true, completed: false, closing: false })
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if checkpoint.is_some() || input.declared_bytes() > FLOW_HOST_ONLY_RAW_BYTES {
            return Err((semio_framework::ToolJobFactoryError::new("Flow host-only job rejects checkpoint or oversized wire owner"), input, checkpoint));
        }
        let mut job = match self.create_job(operation, payload) {
            Ok(job) => job,
            Err(error) => return Err((error, input, None)),
        };
        job.input = Some(input);
        job.decoder = Some(store::os_pack::ScalarRecordWireWitness::new(job.payload.as_ref().unwrap().clone(), flow_host_wire_view));
        job.validated = false;
        Ok(job)
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowHostEffectJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_HOST_ONLY_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "evaluate", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "contextMenuAt", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "openSpotlight", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "replaceImage", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "flowEvalTick", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "flowEvalResolve", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
    ];
}
//#endregion 🧵️HostOnlyRetainedRoutes

//#region 🧵️GraphOperationRetainedRoute
/// 🕸️ The whole-graph routes: each runs one `FlowHost` operation (or the rename's pure snapshot
/// rewrite) against the composed scene and publishes ONE snapshot group on its content child. Retained rather
/// than batch-dispatched, because a `BatchOnlyPendingRewrite` classification on these six is what
/// faulted the entire app at construction with `interactive-job.catalog-authority` on every host
/// that instantiates the flow editor (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const FLOW_GRAPH_OPERATION_TOOL_IDS: &[&str] = &["connectMediaPorts", "reorganize", "renameFlowWidget", "runExtensionAction", "setActiveExample"];
pub(crate) const FLOW_GRAPH_OPERATION_RAW_BYTES: usize = 16_384;
/// 🧮️ The ONE capacity this route declares — its survey walks at most every widget of an admitted
/// scene, plus the single apply step (`📓️work-capacity-2026-09-10.md`).
const FLOW_GRAPH_OPERATION_CAPACITY: semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity =
    semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(FLOW_STORE_MAX_SCENE_ITEMS + 1);

/// 🕸️ The widget ids one graph operation references, resolved by the survey walk before the host
/// runs — `[source, target]` for `connectMediaPorts`, `[old, trimmed new]` for `renameFlowWidget`,
/// none for the whole-graph routes.
fn flow_graph_operation_references(command: &FlowCommand) -> [Option<&str>; 2] {
    match command {
        FlowCommand::ConnectMediaPorts(payload) => [Some(payload.source_node_id.as_str()), Some(payload.target_node_id.as_str())],
        FlowCommand::RenameFlowWidget(payload) => [Some(payload.old_id.as_str()), Some(payload.value.trim())],
        _ => [None, None],
    }
}

/// 🧾️ Every graph-operation payload bound this route admits, in one place — a payload over any of
/// them refuses at `extent` (`None`) instead of reaching the host.
fn flow_graph_operation_payload_admitted(command: &FlowCommand) -> bool {
    let text_bytes = flow_graph_operation_references(command).iter().flatten().map(|reference| reference.len()).fold(0, usize::saturating_add);
    if text_bytes > FLOW_STORE_MAX_TEXT_BYTES {
        return false;
    }
    match command {
        FlowCommand::RunExtensionAction(payload) => payload.action_id.len() <= FLOW_STORE_MAX_TEXT_BYTES,
        FlowCommand::ConnectMediaPorts(payload) => payload.source_port_id.len().saturating_add(payload.target_port_id.len()) <= FLOW_STORE_MAX_TEXT_BYTES,
        FlowCommand::SetActiveExample(payload) => payload.example_id.len() <= FLOW_STORE_MAX_TEXT_BYTES,
        FlowCommand::Reorganize(_) | FlowCommand::RenameFlowWidget(_) => true,
        _ => false,
    }
}

struct FlowGraphOperationWork {
    tool_id: &'static str,
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    cursor: usize,
    replay_target: usize,
    resolved: [bool; 2],
    completed: bool,
    closing: bool,
}

impl FlowGraphOperationWork {
    fn new(tool_id: &'static str, instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, instance_owner: Some(instance_owner), cursor: 0, replay_target: 0, resolved: [false, false], completed: false, closing: false }
    }

    fn replaying(&self) -> bool {
        self.cursor < self.replay_target
    }

    fn apply(&self, command: &FlowCommand, composed: &FlowSnapshot, config: &FlowMainWindowConfig, _interaction: &protocol::InteractionState) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-retained-graph-instance-owner"))?;
        let resolved = self.resolved;
        instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| match command {
                FlowCommand::ConnectMediaPorts(payload) if resolved == [true, true] => connect_media_ports::connect_edit(payload, composed, config, session),
                FlowCommand::ConnectMediaPorts(payload) => Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("connectMediaPorts found no widget \"{}\"", if resolved[0] { &payload.target_node_id } else { &payload.source_node_id }))),
                FlowCommand::Reorganize(_) => reorganize::reorganize_edit(composed, config, session),
                FlowCommand::RenameFlowWidget(payload) if resolved[0] && !resolved[1] && !payload.value.trim().is_empty() => rename_flow_widget::rename_edit(payload, composed),
                FlowCommand::RenameFlowWidget(payload) if !resolved[0] => Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("mutation.target-missing"), format!("renameFlowWidget found no widget \"{}\"", payload.old_id))),
                FlowCommand::RenameFlowWidget(payload) => Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.widget-id-unavailable"), format!("renameFlowWidget cannot rename \"{}\" to \"{}\": the id is empty or taken", payload.old_id, payload.value.trim()))),
                FlowCommand::RunExtensionAction(payload) => run_extension_action::extension_action_result(payload, composed, config, session),
                FlowCommand::SetActiveExample(payload) => set_active_example::set_active_example_edit(payload),
                _ => Err(Fault::from("flow-retained-graph-route-mismatch")),
            })?
        })
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowGraphOperationWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if self.closing || self.completed || command.command_id() != self.tool_id || !FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&self.tool_id) || !flow_graph_operation_payload_admitted(command) {
            return None;
        }
        let child = flow_content_child(snapshot, &context?.children)?;
        if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
            return None;
        }
        FLOW_GRAPH_OPERATION_CAPACITY.rows_for_items(child.nodes.len().saturating_add(1))
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
        if self.completed || self.closing {
            return Err(Fault::from("flow-retained-graph-work-terminal"));
        }
        if command.command_id() != self.tool_id || !flow_graph_operation_payload_admitted(command) {
            return Err(Fault::from("flow-retained-graph-payload-capacity"));
        }
        let context = context.ok_or_else(|| Fault::from("flow-retained-graph-context-required"))?;
        let config = main::config::from_snapshot(context.window_config.as_ref());
        let child = flow_content_child(snapshot, &context.children).ok_or_else(|| Fault::from("flow-retained-content-child-required"))?;
        if child.nodes.len() > FLOW_STORE_MAX_SCENE_ITEMS || child.edges.len() > FLOW_STORE_MAX_SCENE_ITEMS {
            return Err(Fault::from("flow-retained-scene-capacity"));
        }
        if let Some(node) = child.nodes.get(self.cursor) {
            for (slot, reference) in flow_graph_operation_references(command).into_iter().enumerate() {
                if reference.is_some_and(|reference| reference == node.id) {
                    self.resolved[slot] = true;
                }
            }
            self.cursor += 1;
            return Ok(if self.replaying() {
                ArtifactCommandWorkStep::Replay { stage: "flow-graph-survey-replay", preview: br#"{"en":"Restoring graph survey","de":"Graph-Erhebung wird wiederhergestellt"}"# }
            } else {
                ArtifactCommandWorkStep::Progress { stage: "flow-graph-survey", preview: br#"{"en":"Surveying the graph","de":"Graph wird erhoben"}"# }
            });
        }
        self.completed = true;
        let composed = crate::flow_composed_snapshot(snapshot, &context.children)?;
        self.apply(command, &composed, &config, interaction).map(ArtifactCommandWorkStep::Complete)
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 9 {
            return Err(Fault::from("flow-retained-graph-checkpoint-capacity"));
        }
        target[0] = u8::from(self.completed);
        target[1..9].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        Ok(9)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 9 || checkpoint[0] > 1 {
            return Err(Fault::from("flow-retained-graph-checkpoint-invalid"));
        }
        self.completed = checkpoint[0] == 1;
        self.replay_target = usize::try_from(u64::from_le_bytes(checkpoint[1..9].try_into().map_err(|_| Fault::from("flow-retained-graph-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-graph-checkpoint-invalid"))?;
        self.cursor = 0;
        self.resolved = [false, false];
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.instance_owner.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none()
    }
}

struct FlowGraphOperationJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowGraphOperationJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_GRAPH_OPERATION_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

fn flow_graph_operation_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::resumable(FLOW_GRAPH_OPERATION_RAW_BYTES, 256, FLOW_GRAPH_OPERATION_CAPACITY.work_items() as u64, 16_384, 7_500, 1, 1)
}

impl semio_framework::ToolJobFactory for FlowGraphOperationJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        flow_graph_operation_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FLOW_GRAPH_OPERATION_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Flow graph operation job rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowGraphOperationJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_GRAPH_OPERATION_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "connectMediaPorts", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameFlowWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "runExtensionAction", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    ];
}

struct FlowGraphOperationJobFactoryProofs;

impl FlowGraphOperationJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowGraphOperationJobFactory",
        factory_type: FlowGraphOperationJobFactory,
        contract: flow_graph_operation_contract(),
        tools: ["connectMediaPorts", "reorganize", "renameFlowWidget", "runExtensionAction", "setActiveExample"]
    }
}
//#endregion 🧵️GraphOperationRetainedRoute

struct FlowDirectStoreJobFactoryProofs;

impl FlowDirectStoreJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowDirectStoreJobFactory",
        factory_type: FlowDirectStoreJobFactory,
        contract: flow_direct_store_contract(),
        tools: [
            "removeWidget",
            "deleteSelection",
            "disconnect",
            "patchFlowWidgets",
            "duplicateWidget",
            "focusSelection",
            "nodeGraphViewport",
            "setLodMode",
            "setProximityDistance",
            "setGridVisible",
            "setGridSnapEnabled",
            "setGridFactor",
            "setPreviewOff",
            "setCatalogueSections",
            "toggleExtension",
            "addGeneration",
            "removeGeneration",
            "selectGeneration",
            "renameGeneration",
            "updateGenerationValues",
        ]
    }
}

struct FlowHostEffectJobFactoryProofs;

impl FlowHostEffectJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowHostEffectJobFactory",
        factory_type: FlowHostEffectJobFactory,
        tools: {
            "evaluate" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "contextMenuAt" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "openSpotlight" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "replaceImage" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "flowEvalTick" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "flowEvalResolve" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
        }
    }
}

struct FlowChildGroupJobFactoryProofs;

impl FlowChildGroupJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowChildGroupJobFactory",
        factory_type: FlowChildGroupJobFactory,
        tools: {
            "addWidget" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "moveMediaNode" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "nodeGraphEdit" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
            "spotlightCommit" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
        }
    }
}

//#region 🧩️ContributionsRoute
/// 🧩️ The host→guest contributions route — the flow twin of generation2d's: the shell's `flow.extension`
/// closure is what makes this app's extension operators exist at all (`install_builtin_flow_extensions`
/// installs none).
const FLOW_CONTRIBUTIONS_TOOL_IDS: &[&str] = &["setContributions"];
const FLOW_CONTRIBUTIONS_PAYLOAD_SCHEMA: &str = "flow.contributions-command.v1";
/// 📐️ The real wire ceiling of one contributions push: the paged command ingress's assembled-command bound,
/// exactly as generation2d/generation3d declare it — the unscoped nine-extension closure (293 642 characters,
/// `flow-extension-brep` alone 190 656) crosses whole.
const FLOW_CONTRIBUTIONS_RAW_BYTES: usize = semio_framework::kernel::COMMAND_MAXIMUM_BYTES;

fn flow_contributions_contract() -> semio_framework::ToolExecutionContract {
    semio_framework::ToolExecutionContract::bounded_first_step(FLOW_CONTRIBUTIONS_RAW_BYTES, 256, 1, 16_384, 7_500)
}

/// 🧩️ Installs one contributions page against the app instance's retained evaluation session.
struct FlowContributionsWork {
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    completed: bool,
    closing: bool,
}

impl FlowContributionsWork {
    fn new(instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { instance_owner: Some(instance_owner), completed: false, closing: false }
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowContributionsWork {
    fn tool_id(&self) -> &'static str {
        "setContributions"
    }

    fn extent(
        &self,
        command: &FlowCommand,
        _snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        (!self.closing && !self.completed && matches!(command, FlowCommand::SetContributions(payload) if payload.json.len() <= FLOW_CONTRIBUTIONS_RAW_BYTES)).then_some(1)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        if self.closing || self.completed {
            return Err(Fault::from("flow-contributions-work-terminal"));
        }
        let FlowCommand::SetContributions(payload) = input.command else {
            return Err(Fault::from("flow-contributions-route-rejected"));
        };
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-contributions-instance-owner"))?;
        instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| owner.with_session(|session| set_contributions::install(payload, session))?)?;
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(Emit::default()))
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if self.instance_owner.take().is_some() {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.instance_owner.is_none()
    }
}

struct FlowContributionsJobFactory {
    keys: Vec<semio_framework::ToolFactoryKey>,
}

impl FlowContributionsJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FLOW_CONTRIBUTIONS_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FlowContributionsJobFactory {
    type Payload = ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<FlowPlayApp>>;
    type Job = ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<FlowPlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FLOW_CONTRIBUTIONS_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        flow_contributions_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FLOW_CONTRIBUTIONS_RAW_BYTES || checkpoint.is_some() {
            return Err((semio_framework::ToolJobFactoryError::new("Flow contributions command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FlowContributionsJobFactory {
    type Owner = semio_framework_plugin::EditorApp<FlowPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FLOW_CONTRIBUTIONS_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] =
        &[semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] }];
}

struct FlowContributionsJobFactoryProofs;

impl FlowContributionsJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        artifact_schema: "s.flow.flow",
        factory: "FlowContributionsJobFactory",
        factory_type: FlowContributionsJobFactory,
        contract: flow_contributions_contract(),
        tools: ["setContributions"]
    }
}
//#endregion 🧩️ContributionsRoute

//#region 🔖️FlowPlayApp
struct FlowInstanceOperationOwner {
    eval_session: Option<FlowEvalSession>,
    closing: bool,
}

impl FlowInstanceOperationOwner {
    fn new() -> Self {
        Self { eval_session: Some(FlowEvalSession::new()), closing: false }
    }

    fn with_session<R>(&mut self, body: impl FnOnce(&mut FlowEvalSession) -> R) -> Result<R, Fault> {
        if self.closing {
            return Err(Fault::from("flow-eval-session-closing"));
        }
        self.eval_session.as_mut().map(body).ok_or_else(|| Fault::from("flow-eval-session-owner-missing"))
    }
}

impl semio_framework_plugin::ArtifactInstanceOperationOwner for FlowInstanceOperationOwner {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn maintenance_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        let Some(session) = self.eval_session.as_mut() else { return Ok(semio_framework_plugin::PluginCloseStep::Complete) };
        let step = session.close_step(maximum_items, maximum_bytes);
        if session.terminal_is_empty() {
            self.eval_session = None;
        }
        Ok(match step {
            semio_framework_job::InteractiveJobCloseStep::Blocked => semio_framework_plugin::PluginCloseStep::Blocked { reason: "Flow evaluation session awaits its exact close grant" },
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes },
            semio_framework_job::InteractiveJobCloseStep::Complete => semio_framework_plugin::PluginCloseStep::Complete,
        })
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        self.closing = true;
        if let Some(session) = self.eval_session.as_mut() {
            session.begin_close();
        }
        self.maintenance_step(maximum_items, maximum_bytes)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.eval_session.is_none()
    }
}

/// 🧪️ Stateless app definition; the evaluation session is retained by the exact app-instance owner.
#[derive(Default)]
pub struct FlowPlayApp;

impl ArtifactEditor for FlowPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source()]
    }
    /// 🧩️ Composes `s.stdio.semio@v1/*` children, so every bundle of this surface opens them through the same roster.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    /// 🛍️ Publishes the whole registered flow operator catalogue once per app instance on the reserved
    /// `framework.section.catalogue` retained surface — never on the node-graph scene, whose fixed
    /// `UI_FIXED_BYTES` admission it exceeds threefold with the real `brep`/`math` sets installed
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
    fn app_catalogue_json() -> String {
        flow::flow_app_catalogue_json()
    }

    type Snapshot = FlowSnapshot;
    type Mutation = FlowMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = FlowPresence;
    type PresenceMutation = FlowPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = FlowCommand;

    const DIALECT: Dialect = crate::FLOW_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<store::ChildRestoreProjection<'_>, Fault> {
        store::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.child-projection"), error.to_string()))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::retirement::store_owners())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(crate::editor::flow::presence::retirement::FlowPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(crate::editor::flow::presence::retirement::FlowPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(crate::editor::flow::presence::retirement::store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<main::config::FlowMainWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        main::transient::register(registry)
    }

    /// 🧭️ The leaf that names a node-drag release (design §19.1): a release that also drew a wire lands its `insert-edge`
    /// before the relative `drag-nodes`, and the history row is still labelled by the drag.
    fn tool_intent_kinds(tool: &str) -> &'static [&'static str] {
        match tool.strip_prefix(crate::editor::flow::modes::edit::tools::drag::FLOW_EDITOR_APP_ID).and_then(|verb| verb.strip_prefix('#')) {
            Some(crate::editor::flow::commands::node_graph_edit::NODE_GRAPH_EDIT_VERB | "moveMediaNode") => &["drag-nodes"],
            _ => &[],
        }
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = FlowDirectStoreJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(FlowHostEffectJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(FlowChildGroupJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(FlowGraphOperationJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(FlowContributionsJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(FlowChildGroupJobFactory::new(&controller))?;
        registry.register(FlowHostEffectJobFactory::new(&controller))?;
        registry.register(FlowGraphOperationJobFactory::new(&controller))?;
        registry.register(FlowContributionsJobFactory::new(&controller))?;
        registry.register(FlowDirectStoreJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !FLOW_CHILD_GROUP_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_HOST_ONLY_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "Flow command does not match its exact retained tool registration"));
        }
        if FLOW_CHILD_GROUP_TOOL_IDS.contains(&request.tool_id.as_str())
            || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&request.tool_id.as_str())
            || FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str())
            || FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            let tool_id = request.command.command_id();
            let work: Box<dyn ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = if FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id) {
                Box::new(FlowContributionsWork::new(request.instance_operation_owner))
            } else if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
                Box::new(FlowChildGroupWork::new(tool_id, request.instance_operation_owner))
            } else if FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id) {
                Box::new(FlowGraphOperationWork::new(tool_id, request.instance_operation_owner))
            } else {
                Box::new(FlowDirectStoreWork::new(tool_id))
            };
            let operation_context = semio_framework_plugin::AppOperationContext {
                app_instance_id: request.app_instance_id,
                parent_document_id: request.parent_document_id,
                operation_id: request.operation.operation.0,
                generation: request.operation.generation.0,
                canonical_base_revision: request.canonical_base_revision,
                authoring_seed: request.authoring_seed.clone(),
            };
            let payload = ArtifactRetainedCommandPayload::try_new(
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
                FlowCommand::command_id,
                if FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id) {
                    FLOW_CONTRIBUTIONS_RAW_BYTES
                } else if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
                    FLOW_CHILD_GROUP_RAW_BYTES
                } else if FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id) {
                    FLOW_GRAPH_OPERATION_RAW_BYTES
                } else {
                    FLOW_DIRECT_STORE_RAW_BYTES
                },
                if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) || FLOW_CONTRIBUTIONS_TOOL_IDS.contains(&tool_id) {
                    1
                } else if FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id) {
                    FLOW_GRAPH_OPERATION_CAPACITY.work_items()
                } else {
                    FLOW_STORE_MAX_MUTATION_ITEMS
                },
                work,
            )?;
            return Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        let payload = FlowHostEffectPayload {
            command: *request.command,
            snapshot: request.snapshot,
            config: Arc::new(main::config::from_snapshot(request.window_config.as_ref())),
            history: request.history,
            children: request.context.children.clone(),
            instance_owner: request.instance_operation_owner,
            completion: request.completion,
        };
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(FlowInstanceOperationOwner::new())
    }

    fn initial_snapshot() -> FlowSnapshot {
        FlowSnapshot::default()
    }

    /// 🌱️ Derives the `content` child at boot and on every archive load, so a live shell composes the
    /// child that every verb edits and every window reads — see [`crate::flow_genesis_content_pack`] and
    /// [`crate::flow_composed_snapshot`].
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::flow_genesis_content_pack(snapshot, slot, child_id)
    
})())
}

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// manifest declaration (host-pushed/internally-chained, not user-facing actions).
    fn command_id(command: &FlowCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `FlowCommand`, the bridge every app owes the shell's
    /// `{action, args}` wire. Without it the trait's default refuses EVERY id, and the one caller that
    /// cannot avoid this channel is the host's own re-arm: `Effect::dispatchAction` is delivered through
    /// `handle_action`, so flow's `flowEvalTick` chain died on its first hop with
    /// `action 'flowEvalTick' is not a framework-reserved action` and the node graph never evaluated
    /// (16 refusal lines per boot, measured on :6016, ticket 26/09/18 slice B3c). Mirrors
    /// `Generation2dPlayApp::command_from_action`, which is why that sibling's identical eval chain runs.
    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.cloned().unwrap_or(semio_framework_value::DslValue::Null);
        let str_arg = |keys: &[&str]| -> Option<String> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_str()).map(str::to_string)) };
        let f64_arg = |keys: &[&str]| -> Option<f64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_f64())) };
        let u64_arg = |keys: &[&str]| -> Option<u64> { keys.iter().find_map(|key| args.get(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().map(|number| number as u64)))) };
        let bool_arg = |keys: &[&str]| -> Option<bool> { keys.iter().find_map(|key| args.get(key).and_then(semio_framework_value::DslValue::as_bool)) };
        let string_list = |keys: &[&str]| -> Vec<String> {
            keys.iter()
                .find_map(|key| args.get(key).and_then(semio_framework_value::DslValue::as_array))
                .map_or_else(Vec::new, |items| items.iter().filter_map(|item| item.as_str().map(str::to_string)).collect())
        };
        match action {
            "addWidget" => Ok(FlowCommand::AddWidget(add_widget::AddWidget {
                kind: str_arg(&["kind"]).unwrap_or_else(|| "inputSlider".into()),
                neuron_kind: str_arg(&["neuronKind", "neuron_kind"]),
                x: f64_arg(&["x"]),
                y: f64_arg(&["y"]),
                label: str_arg(&["label"]),
                action: str_arg(&["action"]),
                format: str_arg(&["format"]),
            })),
            "removeWidget" => Ok(FlowCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "duplicateWidget" => Ok(FlowCommand::DuplicateWidget(duplicate_widget::DuplicateWidget { widget_id: str_arg(&["widgetId", "widget_id", "id"]).unwrap_or_default() })),
            "deleteSelection" => Ok(FlowCommand::DeleteSelection(delete_selection::DeleteSelection {})),
            "disconnect" => Ok(FlowCommand::Disconnect(disconnect::Disconnect { synapse_id: str_arg(&["synapseId", "synapse_id", "id"]).unwrap_or_default() })),
            "connectMediaPorts" => Ok(FlowCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts {
                source_node_id: str_arg(&["sourceNodeId", "source_node_id"]).unwrap_or_default(),
                source_port_id: str_arg(&["sourcePortId", "source_port_id"]).unwrap_or_default(),
                target_node_id: str_arg(&["targetNodeId", "target_node_id"]).unwrap_or_default(),
                target_port_id: str_arg(&["targetPortId", "target_port_id"]).unwrap_or_default(),
            })),
            "moveMediaNode" => Ok(FlowCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: str_arg(&["nodeId", "node_id", "id"]).unwrap_or_default(), x: f64_arg(&["x"]).unwrap_or(0.0), y: f64_arg(&["y"]).unwrap_or(0.0) })),
            "reorganize" => Ok(FlowCommand::Reorganize(reorganize::Reorganize {})),
            "patchFlowWidgets" => Ok(FlowCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets {
                widget_ids: string_list(&["widgetIds", "widget_ids", "ids"]),
                field: str_arg(&["field"]).unwrap_or_default(),
                value: str_arg(&["value"]).unwrap_or_default(),
            })),
            "renameFlowWidget" => Ok(FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: str_arg(&["oldId", "old_id", "id"]).unwrap_or_default(), value: str_arg(&["value", "name"]).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(FlowCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations: node_graph_edit::operations_from_action(&args)? })),
            "spotlightCommit" => Ok(FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit { operations: node_graph_edit::operations_from_action(&args)? })),
            "runExtensionAction" => Ok(FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: str_arg(&["actionId", "action_id", "id"]).unwrap_or_default() })),
            "setActiveExample" => Ok(FlowCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_arg(&["exampleId", "example_id"]).unwrap_or_default() })),
            "evaluate" => Ok(FlowCommand::Evaluate(evaluate::Evaluate {})),
            "focusSelection" => Ok(FlowCommand::FocusSelection(focus_selection::FocusSelection {})),
            "nodeGraphViewport" => Ok(FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport {
                viewport: match args.get("viewport").cloned() {
                    None => semio_framework_os_kernel::Viewport2d::default(),
                    Some(value) => semio_framework_value::FromValue::from_value(value).map_err(|error| Fault::from(format!("invalid nodeGraphViewport viewport: {error}")))?,
                },
            })),
            "setLodMode" => Ok(FlowCommand::SetLodMode(set_lod_mode::SetLodMode { value: str_arg(&["value", "mode"]).unwrap_or_default() })),
            "setProximityDistance" => Ok(FlowCommand::SetProximityDistance(set_proximity_distance::SetProximityDistance { value: f64_arg(&["value", "distance"]).unwrap_or_default() })),
            "setGridVisible" => Ok(FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: bool_arg(&["pressed", "value", "visible"]) })),
            "setGridSnapEnabled" => Ok(FlowCommand::SetGridSnapEnabled(set_grid_snap_enabled::SetGridSnapEnabled { pressed: bool_arg(&["pressed", "value", "enabled"]) })),
            "setGridFactor" => Ok(FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: f64_arg(&["value", "factor"]).unwrap_or_default() })),
            "contextMenuAt" => Ok(FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: str_arg(&["id"]).unwrap_or_default() })),
            "setPreviewOff" => Ok(FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: string_list(&["ids", "widgetIds", "widget_ids"]), value: bool_arg(&["value", "off"]).unwrap_or(false) })),
            "openSpotlight" => Ok(FlowCommand::OpenSpotlight(open_spotlight::OpenSpotlight {})),
            "replaceImage" => Ok(FlowCommand::ReplaceImage(replace_image::ReplaceImage { id: str_arg(&["id", "widgetId", "widget_id"]).unwrap_or_default() })),
            "setCatalogueSections" => Ok(FlowCommand::SetCatalogueSections(set_catalogue_sections::SetCatalogueSections { sections_json: str_arg(&["sectionsJson", "sections_json"]).or_else(|| args.get("sections").map(semio_framework_pack_json::to_json_string)).unwrap_or_else(|| "[]".into()) })),
            "toggleExtension" => Ok(FlowCommand::ToggleExtension(toggle_extension::ToggleExtension { id: str_arg(&["id", "extensionId", "extension_id"]).unwrap_or_default(), enabled: bool_arg(&["enabled", "value"]).unwrap_or(false) })),
            "addGeneration" => Ok(FlowCommand::AddGeneration(add_generation::AddGeneration {})),
            "removeGeneration" => Ok(FlowCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "selectGeneration" => Ok(FlowCommand::SelectGeneration(select_generation::SelectGeneration { id: str_arg(&["id"]).unwrap_or_default() })),
            "renameGeneration" => Ok(FlowCommand::RenameGeneration(rename_generation::RenameGeneration { id: str_arg(&["id"]).unwrap_or_default(), name: str_arg(&["name", "value"]).unwrap_or_default() })),
            "updateGenerationValues" => Ok(FlowCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues {
                generation_id: str_arg(&["generationId", "generation_id"]),
                question_id: str_arg(&["questionId", "question_id"]).unwrap_or_default(),
                value: args.get("value").cloned().unwrap_or(semio_framework_value::DslValue::Null),
            })),
            // 🪟️ Both hops are ADDRESSED: `windowId` rides on the tick's own args and is echoed back
            // onto the answer by `reactor::extension_response_args`, so a hop discharges the latch of
            // the window it evaluated instead of whichever window happened to be current.
            "flowEvalTick" => Ok(FlowCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_else(|| main::FLOW_PLAY_WINDOW_MAIN.into()),
                window_kind_id: str_arg(&["windowKindId", "window_kind_id"]).unwrap_or_else(|| main::FLOW_PLAY_WINDOW_MAIN.into()),
            })),
            "flowEvalResolve" => Ok(FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve {
                window_id: str_arg(&["windowId", "window_id"]).unwrap_or_else(|| main::FLOW_PLAY_WINDOW_MAIN.into()),
                node_hash: u64_arg(&["nodeHash", "node_hash"]).unwrap_or_default(),
                output_json: str_arg(&["outputJson", "output_json"]).unwrap_or_default(),
            })),
            "setContributions" => Ok(FlowCommand::SetContributions(set_contributions::SetContributions {
                json: str_arg(&["json"]).unwrap_or_default(),
                page: u64_arg(&["page"]).unwrap_or_default(),
                page_count: u64_arg(&["pageCount", "page_count"]).unwrap_or(1),
            })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `deleteSelection`/`focusSelection`/
    /// `nodeGraphEdit`/`spotlightCommit` read the "graph" interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg, session)`
    /// signature is framework-fixed and has no `interaction` slot) — mirrors `space`'s equivalent routing.
    fn handle(
        command: &FlowCommand,
        doc: &ArtifactView<'_, FlowSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<FlowMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        if FLOW_CHILD_GROUP_TOOL_IDS.contains(&command.command_id())
            || FLOW_HOST_ONLY_TOOL_IDS.contains(&command.command_id())
            || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&command.command_id())
            || FLOW_DIRECT_STORE_TOOL_IDS.contains(&command.command_id())
        {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.retained.legacy-dispatch"), "Flow retained routes execute only through their exact app-owned job factory"));
        }
        let mut session = FlowEvalSession::new();
        let emitted = match command {
            FlowCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, &mut session, interaction),
            FlowCommand::FocusSelection(payload) => focus_selection::apply(payload, doc, cfg, &mut session, interaction),
            _ => command.dispatch(doc, cfg, &mut session),
        };
        // 🧹️ A throwaway evaluation session refuses a bare drop; it is closed, never dropped.
        session.retire_cold();
        emitted
    }

    /// 🕹️ `graph`'s `HierarchyProvider::Topology`: every widget/synapse is registered at its own
    /// granularity, every one a root — the outer widget list has no real parent/child membership (a
    /// `Widget::Cluster`'s own `tree` is a private, self-contained nested sub-graph, not exposed at this
    /// domain), so this deliberately does NOT declare transitive hover/selection (see `🔖️Manifest`'s
    /// `.interaction(...)` doc comment for why that diverges from the ticket's headline example).
    /// `Topology` (rather than `Flat`) is still the right choice purely for the pruning it buys:
    /// `validate_state` drops stale ids of a domain it has membership info for, and `Flat` domains are
    /// skipped entirely (see the design doc's `HierarchyProvider::Flat` note). "handle" targets have no
    /// persisted document data to register — see `flow_graph_selection_domains`'s doc comment.
    fn interaction_topology(doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        let mut domains = std::collections::BTreeMap::new();
        let Ok(composed) = crate::flow_composed_snapshot(doc.snapshot, &doc.children) else {
            domains.insert(FLOW_INTERACTION_GRAPH.to_string(), DomainTopology { ordered: Vec::new() });
            return InteractionTopology { domains };
        };
        let live = composed.to_host_snapshot();
        let mut ordered: Vec<TopologyNode> = live.widgets.iter().map(|widget| TopologyNode { id: flow_graph_node_target_id(crate::schema::widget_id(widget)), granularity: "node".into(), parent: None }).collect();
        ordered.extend(live.synapses.iter().map(|synapse| TopologyNode { id: flow_graph_edge_target_id(&synapse.id), granularity: "edge".into(), parent: None }));
        live.retire_cold();
        domains.insert(FLOW_INTERACTION_GRAPH.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    
})())
}

    /// 🧵️ Arms a `flowEvalTick` chain for every ATTACHED main window whose evaluation the snapshot
    /// still owes — covers every mutation path (edits, undo/redo, example load, remote operations) in
    /// one place.
    ///
    /// 🔒️ The probe runs against the app instance's RETAINED session, never a throwaway one. A
    /// throwaway session has an empty tick latch by construction, so every host refresh answered
    /// "nothing owes a hop yet" and minted another one for the identical snapshot: 2015 ×
    /// `transient read registry is busy or exhausted` in ~3 s and the shell down with it, measured on
    /// :6016 (ticket 26/09/18 §5.3). Latches of windows that have left the roster are dropped here,
    /// because a detached window's latch can never be discharged by a hop.
    fn pending_effects(owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        let windows: Vec<(String, String)> = view
            .map(|view| view.window_instances.iter().filter(|window| window.window_kind_id == main::FLOW_PLAY_WINDOW_MAIN).map(|window| (window.id.clone(), window.window_kind_id.clone())).collect())
            .unwrap_or_default();
        if windows.is_empty() {
            return Vec::new();
        }
        let config = main::config::current(cfg);
        let Ok(composed) = crate::flow_composed_snapshot(doc.snapshot, &doc.children) else { return Vec::new() };
        owner
            .with_mut::<FlowInstanceOperationOwner, _>(|instance| {
                instance.with_session(|session| {
                    let live: Vec<&str> = windows.iter().map(|(id, _)| id.as_str()).collect();
                    session.retain_window_tick_latches(&live);
                    windows.iter().flat_map(|(window_id, window_kind_id)| evaluate::evaluate_result(&composed, &config, session, window_id, window_kind_id).effects).collect::<Vec<Effect>>()
                })
            })
            .unwrap_or_default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("flow.content-unavailable", fault.message))?;
        let snapshot = &composed;
        let config = main::config::current(cfg);
        let transient = main::transient::FlowWindowTransient::default();
        let labels = flow_play_labels(view_state);
        let mut session = FlowEvalSession::new();
        let rendered = match body_key {
            FLOW_PLAY_BODY_MAIN => main::render(snapshot, &config, &mut session, &[]).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_COMPILED => compiled::render(snapshot, &config, &mut session).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATIONS => generations::render(&transient, view_state.locale, view_state.terminology, &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_GENERATIONS))
                .map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATE_FORM => form::render(snapshot, &config, &transient, labels).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(&transient).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_ARTIFACT => document_panel::render(snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_ARTIFACT)).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(snapshot, &config, &mut session, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_CATALOGUE)).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_INSPECTOR)).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        };
        // 🧹️ A throwaway evaluation session refuses a bare drop; it is closed, never dropped.
        session.retire_cold();
        rendered
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, FlowSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = main::config::current(cfg);
        let transient = main::transient::current(transient);
        let (graph_selection, _) = flow_graph_selection_domains(&interaction.selection(FLOW_INTERACTION_GRAPH).ids);
        let composed = crate::flow_composed_snapshot(doc.snapshot, &doc.children).map_err(|fault| semio_framework_plugin::PluginAssemblyError::new("flow.content-unavailable", fault.message))?;
        owner
            .with_mut::<FlowInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| match body_key {
                    FLOW_PLAY_BODY_MAIN => main::render(&composed, &config, session, &graph_selection).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_COMPILED => compiled::render(&composed, &config, session).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_GENERATIONS => {
                        generations::render(&transient, view_state.locale, view_state.terminology, &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_GENERATIONS))
                            .map(semio_framework_plugin::built_to_component_tree)
                    }
                    FLOW_PLAY_BODY_GENERATE_FORM => form::render(&composed, &config, &transient, flow_play_labels(view_state)).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(&transient).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_ARTIFACT => document_panel::render(&composed, flow_play_labels(view_state), &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_ARTIFACT)).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_CATALOGUE => {
                        catalogue_panel::render(&composed, &config, session, flow_play_labels(view_state), &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_CATALOGUE)).map(semio_framework_plugin::built_to_component_tree)
                    }
                    FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(flow_play_labels(view_state), &semio_framework_plugin::TreeWindows::for_body(view_state, FLOW_PLAY_BODY_INSPECTOR)).map(semio_framework_plugin::built_to_component_tree),
                    _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
                })
            })
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("flow.eval-session-owner", error.message))?
    }

    fn window_measures(_doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.clone() else { return HashMap::new() };
        let is_main = view_state.window_instances.iter().any(|window| window.id == window_id && window.window_kind_id == main::FLOW_PLAY_WINDOW_MAIN);
        if !is_main {
            return HashMap::new();
        }
        let config = main::config::current(cfg);
        HashMap::from([(window_id, main::window_measures(&config, flow_play_labels(view_state)))])
    }

    fn context_menu(request: &ContextMenuRequest, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let config = main::config::current(cfg);
        let Ok(composed) = crate::flow_composed_snapshot(doc.snapshot, &doc.children) else { return Vec::new() };
        flow_context_menu_items(registry, &composed, &config, flow_play_labels(view_state), view_state, request.surface.as_ref())
    }
}
//#endregion 🔖️FlowPlayApp

//#region 🔖️Host
pub fn seed_host_catalogue(host: &mut FlowHost, extra_sections_json: &str) {
    let mut sections = flow::flow_catalogue_sections();
    if let Ok(extra) = serde_json::from_str::<Vec<flow::CatalogueSection>>(extra_sections_json) {
        sections.extend(extra);
    }
    host.set_host_catalogue_json(&serde_json::to_string(&sections).unwrap_or_else(|_| "[]".into()));
}

/// 🎚️ Pushes the view-state canvas options (LOD mode, proximity distance, grid) onto a freshly built host.
pub fn apply_canvas_options(host: &mut FlowHost, config: &FlowMainWindowConfig) {
    if config.lod_mode != FLOW_LOD_MODE_AUTOMATIC && DagDrawLod::from_id(&config.lod_mode).is_some() {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(&config.lod_mode);
    } else {
        host.dag.set_automatic_lod(true);
    }
    host.dag.set_proximity_distance(config.proximity_distance);
    host.set_grid_visible(config.grid_visible);
    host.set_grid_snap_enabled(config.grid_snap_enabled);
    let _ = host.set_grid_factor(config.grid_factor);
}

/// 🏗️ Rebuilds the stateful `FlowHost` from the document projection + view config + eval session — the
/// single entry point every command handler and every window renderer goes through.
pub fn host_from_snapshot(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> FlowHost {
    let live = snapshot.to_host_snapshot();
    let mut host = flow_host_with_session(&live, session);
    live.retire_cold();
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    host
}

/// 🏠️ Runs `body` against a host rebuilt by [`host_from_snapshot`], then retires that host — the ONE
/// shape every render, probe and evaluation step that only needs a host for the length of a call must
/// use. A `FlowHost` owns a `FlowHostSnapshot` whose `layout: OrderedMap<WidgetLayout>` aborts the
/// guest on a bare drop, so a host is closed, never dropped (the twin of `FlowHost::with_host_snapshot`).
pub fn with_host_from_snapshot<R>(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, body: impl FnOnce(&mut FlowHost) -> R) -> R {
    let mut host = host_from_snapshot(snapshot, config, session);
    let result = body(&mut host);
    host.retire_cold();
    result
}

/// 📸️ Runs `body` against the live host projection of `snapshot`, then retires that projection — a
/// `FlowHostSnapshot` owns an `OrderedMap` layout root and widget payloads that refuse a bare drop.
pub fn with_live_host_snapshot<R>(snapshot: &FlowSnapshot, body: impl FnOnce(&semio_framework_artifact_flow_flow::FlowHostSnapshot) -> R) -> R {
    let live = snapshot.to_host_snapshot();
    let result = body(&live);
    live.retire_cold();
    result
}

/// 🪆️ The content child `composed` names, as the exact scene its working-scene owner carries.
pub fn flow_composed_content(composed: &FlowSnapshot) -> Result<SemioFlowSnapshot, Fault> {
    let scene = composed.content.local_owner::<FlowWorkingScene>().ok_or_else(|| Fault::from("flow-edit-scene-owner-missing"))?;
    Ok(crate::flow_content_snapshot_from_working(&scene.widgets, &scene.synapses, &scene.layout))
}

/// 📮️ `leaves` as ONE group on the content child `child_id`; nothing when there are none.
pub fn flow_content_leaves_emit(child_id: &str, leaves: Vec<SemioFlowMutation>) -> Emit<FlowMutation, NoConfigMutation> {
    match leaves.is_empty() {
        true => Emit::default(),
        false => Emit { child_preparations: std::collections::VecDeque::from([ChildEmitPreparation::of::<SemioFlowSnapshot, _>("content", child_id, leaves)]), ui_scope: UiDirtyScope::Full, ..Default::default() },
    }
}

/// 🗑️ The child leaves that delete `node_ids` and `edge_ids` from `child`: every named edge and every edge touching a named
/// node first (child order, so no edge ever dangles), then every named node.
pub fn flow_removal_leaves(child: &SemioFlowSnapshot, node_ids: &[String], edge_ids: &[String]) -> Vec<SemioFlowMutation> {
    let mut edit = crate::editor::flow::edit_rules::ContentEdit::new(child.clone());
    edit.remove(node_ids, edge_ids);
    edit.leaves
}
//#endregion 🔖️Host

//#region 🔖️Selection
pub fn sync_host_selection(host: &mut FlowHost, selected: &[String]) {
    sync_host_selection_domains(host, selected, &[], &[]);
}

pub fn sync_host_selection_domains(host: &mut FlowHost, nodes: &[String], edges: &[String], handles: &[String]) {
    if nodes.is_empty() && edges.is_empty() && handles.is_empty() {
        let _ = host.dag.cancel_area_select();
        return;
    }
    host.dag.set_selection_domains(&infinite_canvas::board::schema::dag_input::DagSelectionDomains {nodes:nodes.to_vec(),edges:edges.to_vec(),handles:handles.to_vec()});
}

/// 🔍️ The camera that frames the given node selection (the "graph" domain's live selection, read by
/// the caller via `InteractionView` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), or
/// `None` when nothing is selected.
pub fn focus_selection_camera(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, selected_node_ids: &[String]) -> Option<CameraJson> {
    if selected_node_ids.is_empty() {
        return None;
    }
    with_host_from_snapshot(snapshot, config, session, |host| {
        host.dag.set_viewport(1280, 800, 1.0);
        host.dag.set_selection(selected_node_ids);
        host.focus_selection_camera(1.2)
    })
}
//#endregion 🔖️Selection

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_flow_app() -> AppDefinition {
    Editor::builder(crate::FLOW_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) })
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung auflösen"), "runtime", ActionKind::View) })
        .command(CommandDefinition {
            in_palette: false,
            ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
            ])
        })
        .document(["semio", "flow"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("flow")
        .mode_def(edit::definition())
        .mode_def(generate::definition())
        .default_mode_id(edit::FLOW_PLAY_MODE_EDIT)
        .window_kind_def(main::definition())
        .window_kind_def(compiled::definition())
        .window_kind_def(generations::definition())
        .window_kind_def(form::definition())
        .window_kind_def(preview::definition())
        .default_layout(edit::layout())
        .named_layout(generate::layout())
        .panel_tab_def(document_panel::definition())
        .panel_tab_def(catalogue_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
        .mutation("addWidget", LocalizedLabel::native("Add Widget", "Widget hinzufügen"))
        .mutation("removeWidget", LocalizedLabel::native("Remove Widget", "Widget entfernen"))
        .action_destructive("removeWidget")
        // 🌉️ COMPOSITE — plans create-widget then connect-widgets (ticket 26/08/16/…-COMPOSITE-MUTATIONS).
        .mutation("duplicateWidget", LocalizedLabel::native("Duplicate Widget", "Widget duplizieren"))
        // 🗂️ Referenced by flow_context_menu_items — categorized for grouped-context-menu disclosure.
        .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
        .action_destructive("deleteSelection")
        .mutation("disconnect", LocalizedLabel::native("Disconnect", "Trennen"))
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Anschlüsse verbinden"))
        .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
        .action_with(ActionDefinition::new("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "rotate-cw").with_category("transform"))
        .mutation("patchFlowWidgets", LocalizedLabel::native("Patch Widgets", "Widgets aktualisieren"))
        .mutation("renameFlowWidget", LocalizedLabel::native("Rename Widget", "Widget umbenennen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .mutation("spotlightCommit", LocalizedLabel::native("Spotlight Commit", "Spotlight bestätigen"))
        // 🧩️ Dynamic extension-provided action — id resolved at runtime, kept out of the palette.
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("runExtensionAction", LocalizedLabel::native("Run Extension Action", "Erweiterungsaktion ausführen"), ActionKind::Mutation) })
        .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Beispiel setzen"))
        // 👁️ Ephemeral view/config actions — mutate config, emit no document operations. Selection/
        // hover verbs (`setSelection`/`clearSelection`/`selectAll`/`selectNode`/`nodeGraphSelect`/
        // `nodeGraphHover`/`graphPointerDown`) are no longer declared here: framework-owned, injected
        // via `.interaction(...)` below (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        .action_with(ActionDefinition::new("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"), ActionKind::View, "hash"))
        .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Zoom to Selection", "Auf Auswahl zoomen"), ActionKind::View).with_category("view"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), ActionKind::View, "camera") })
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View, "layers") })
        .action_with(flow_internal_action("setProximityDistance", LocalizedLabel::native("Set Proximity Distance", "Näheabstand festlegen"), ActionKind::View))
        .action_with(flow_internal_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster sichtbar"), ActionKind::View))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View, "grid-3x3") })
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View, "grid-3x3") })
        .action_with(flow_internal_action("contextMenuAt", LocalizedLabel::native("Context Menu At", "Kontextmenü an Position"), ActionKind::View))
        .action_with(flow_internal_action("setPreviewOff", LocalizedLabel::native("Set Preview Off", "Vorschau deaktivieren"), ActionKind::View).with_category("view"))
        .action_with(flow_internal_action("openSpotlight", LocalizedLabel::native("Open Spotlight", "Spotlight öffnen"), ActionKind::View).with_category("create"))
        .action_with(flow_internal_action("replaceImage", LocalizedLabel::native("Replace Image", "Bild ersetzen"), ActionKind::View).with_category("actions"))
        .action_with(flow_internal_action("setCatalogueSections", LocalizedLabel::native("Set Catalogue Sections", "Katalogabschnitte festlegen"), ActionKind::View))
        .action_with(flow_internal_action("toggleExtension", LocalizedLabel::native("Toggle Extension", "Erweiterung umschalten"), ActionKind::View))
        // 📝️ Staged argument form for the panel-visible create action (module operators stay catalogue-driven).
        .action_args("addWidget", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
            ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
            ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
        ])
        .default_value(&"inputSlider")])
        .action_args("removeWidget", vec![ActionArgDef::text("widgetId", LocalizedLabel::native("Widget Id", "Widget-ID"))])
        .action_args("duplicateWidget", vec![ActionArgDef::text("widgetId", LocalizedLabel::native("Widget Id", "Widget-ID"))])
        .action_args("disconnect", vec![ActionArgDef::text("synapseId", LocalizedLabel::native("Synapse Id", "Synapsen-ID"))])
        .action_args(
            "connectMediaPorts",
            vec![
                ActionArgDef::text("sourceNodeId", LocalizedLabel::native("Source Widget Id", "Quell-Widget-ID")),
                ActionArgDef::text("sourcePortId", LocalizedLabel::native("Source Port", "Quellanschluss")),
                ActionArgDef::text("targetNodeId", LocalizedLabel::native("Target Widget Id", "Ziel-Widget-ID")),
                ActionArgDef::text("targetPortId", LocalizedLabel::native("Target Port", "Zielanschluss")),
            ],
        )
        .action_args("moveMediaNode", vec![ActionArgDef::text("nodeId", LocalizedLabel::native("Widget Id", "Widget-ID")), ActionArgDef::number("x", LocalizedLabel::native("X", "X")), ActionArgDef::number("y", LocalizedLabel::native("Y", "Y"))])
        .action_args(
            "patchFlowWidgets",
            vec![
                ActionArgDef::text_list("widgetIds", LocalizedLabel::native("Widget Ids", "Widget-IDs")),
                ActionArgDef::select("field", LocalizedLabel::native("Field", "Feld"), vec![ActionArgOption::new("value", LocalizedLabel::native("Slider Value", "Reglerwert")), ActionArgOption::new("text", LocalizedLabel::native("Note Text", "Notiztext"))]),
                ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")),
            ],
        )
        .action_args("renameFlowWidget", vec![ActionArgDef::text("oldId", LocalizedLabel::native("Widget Id", "Widget-ID")), ActionArgDef::text("value", LocalizedLabel::native("New Id", "Neue ID"))])
        .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![ActionArgOption::new(crate::examples::demo::ID, LocalizedLabel::native("Demo", "Demo"))])])
        .action_interactive_job("addWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("removeWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("duplicateWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("disconnect", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("connectMediaPorts", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("moveMediaNode", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("patchFlowWidgets", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("renameFlowWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("spotlightCommit", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("runExtensionAction", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_destructive("setActiveExample")
        .action_interactive_job("evaluate", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("focusSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setLodMode", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setProximityDistance", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setGridVisible", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setGridSnapEnabled", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setGridFactor", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("contextMenuAt", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setPreviewOff", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("openSpotlight", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("replaceImage", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setCatalogueSections", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("toggleExtension", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("addGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("removeGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_destructive("removeGeneration")
        .action_interactive_job("selectGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("renameGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("updateGenerationValues", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalTick", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalResolve", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("setContributions", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        // 🕹️ `mod+a`/`escape` are no longer declared here — the framework auto-injects `selectAll`/
        // `clearSelection` (with these SAME keys) for every app with at least one `.interaction(...)`
        // domain, see `interaction_action_definitions`.
        .keybinding("delete,backspace", "deleteSelection")
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "graph" domain — node/
        // edge/handle granularities over the node-graph canvas. `HierarchyProvider::Topology` purely
        // for `validate_state`'s pruning of deleted widget/synapse ids (see
        // `FlowPlayApp::interaction_topology`'s doc comment) — the outer widget list has no real
        // parent/child membership to walk (a `Widget::Cluster`'s own nested `tree` is a private,
        // self-contained sub-graph, never exposed as top-level "graph" members), so — DIVERGING from
        // this ticket's headline "flow" example, which describes transitive hover from group-node
        // membership that the real snapshot model does not have — both hover and selection stay
        // non-transitive here; a future wave adding real group-node containment to the top-level
        // widget list should flip both flags. Multi-select via Pick (document tree rows; the node-
        // graph canvas's own marquee/click wiring is a separate, framework-layer, unmigrated-this-wave
        // renderer — see `flow_graph_selection_domains`'s doc comment) and Rectangle, all five merges.
        .interaction(InteractionDefinition {
            id: FLOW_INTERACTION_GRAPH.into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "spline".into() },
                GranularityDefinition { id: "handle".into(), label: LocalizedLabel::native("Handle", "Anfasser"), icon_id: "move".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                transitive: false,
                broadcast: true,
            },
        })
        .window_kind_interactions(main::FLOW_PLAY_WINDOW_MAIN, vec![InteractionRef::new(FLOW_INTERACTION_GRAPH)])
        // 🎯️ Flow has no user-visible config defaults to expose, so `config_spec()` stays the trait
        // default `ConfigSpec::empty()`; declaring it explicitly keeps the typed channel surface
        // consistent with the sibling apps' convention.
        .config(FlowPlayApp::config_spec())
        // 🚧️ `.example_source(crate::examples::art_flow_demo::source())` and `.workflow("flow",
        // "Flow", "graph")` are DROPPED here, not ported: `EditorBuilder` has no such methods
        // (contract §2.4's `App { definition, examples }` split — `.editor::<E>(def:
        // AppDefinition)` only takes the definition, so `App.examples` has no carrier through this
        // builder). See `📓️w2-p5-flow-notes.md` "SDK gaps" for the framework-level finding.
        .action_describe("addWidget", LocalizedLabel::native("Adds a new widget of the given kind (an input such as a slider or note, or an operator, optionally a specific neuron kind) to the flow canvas at x, y.", "Fügt der Flow-Fläche an x, y ein neues Widget der angegebenen Art hinzu (eine Eingabe wie Schieberegler oder Notiz oder einen Operator, optional eine bestimmte Neuronart)."))
        .action_describe("removeWidget", LocalizedLabel::native("Removes one widget by id from the flow graph together with every synapse attached to it.", "Entfernt ein Widget anhand seiner Id samt aller angeschlossenen Synapsen aus dem Flow-Graphen."))
        .action_describe("duplicateWidget", LocalizedLabel::native("Copies one widget by id into a new widget next to it, connected from the original, as one undoable edit; an unknown id is refused.", "Kopiert ein Widget anhand seiner Id in ein neues, vom Original aus verbundenes Widget daneben, als eine rückgängig machbare Änderung; eine unbekannte Id wird abgelehnt."))
        .action_describe("deleteSelection", LocalizedLabel::native("Deletes every selected widget and synapse from the flow graph.", "Löscht alle ausgewählten Widgets und Synapsen aus dem Flow-Graphen."))
        .action_describe("disconnect", LocalizedLabel::native("Removes one synapse (a connection between two ports) by id from the flow graph.", "Entfernt eine Synapse (eine Verbindung zwischen zwei Ports) anhand ihrer Id aus dem Flow-Graphen."))
        .action_describe("connectMediaPorts", LocalizedLabel::native("Connects an output port of one widget to an input port of another with a new synapse; an unknown widget or an incompatible connection is refused.", "Verbindet einen Ausgangsport eines Widgets mit einem Eingangsport eines anderen durch eine neue Synapse; ein unbekanntes Widget oder eine unverträgliche Verbindung wird abgelehnt."))
        .action_describe("moveMediaNode", LocalizedLabel::native("Moves one widget to the canvas position x, y; consecutive moves of the same widget merge into one edit.", "Verschiebt ein Widget an die Position x, y der Fläche; aufeinanderfolgende Verschiebungen desselben Widgets werden zu einer Änderung zusammengefasst."))
        .action_describe("reorganize", LocalizedLabel::native("Lays out every widget of the flow graph automatically from left to right, overwriting their manual positions.", "Ordnet alle Widgets des Flow-Graphen automatisch von links nach rechts an und überschreibt ihre manuellen Positionen."))
        .action_describe("patchFlowWidgets", LocalizedLabel::native("Sets one field on several widgets at once: a slider's value or a note's text.", "Setzt ein Feld auf mehreren Widgets zugleich: den Wert eines Schiebereglers oder den Text einer Notiz."))
        .action_describe("renameFlowWidget", LocalizedLabel::native("Renames a widget's id from the old id to a new one and updates every synapse that referenced it; a taken or empty id changes nothing.", "Benennt die Id eines Widgets von der alten in eine neue um und aktualisiert alle Synapsen, die darauf verweisen; eine vergebene oder leere Id ändert nichts."))
        .action_describe("runExtensionAction", LocalizedLabel::native("Runs one automation of an installed flow extension, such as auto-layout (Reorganize) or auto-evaluate (Evaluate), when that automation is enabled.", "Führt eine Automatisierung einer installierten Flow-Erweiterung aus, etwa automatisches Anordnen (Neu anordnen) oder automatisches Auswerten (Auswerten), sofern sie aktiviert ist."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole flow graph with the bundled demo graph, or with an empty graph for an empty id; other ids change nothing.", "Ersetzt den gesamten Flow-Graphen durch den mitgelieferten Demo-Graphen, bei leerer Id durch einen leeren Graphen; andere Ids ändern nichts."))
        .action_describe("evaluate", LocalizedLabel::native("Evaluates every widget whose result is not computed yet and shows the results on the canvas; the graph itself is not changed.", "Wertet alle Widgets aus, deren Ergebnis noch nicht berechnet ist, und zeigt die Ergebnisse auf der Fläche; der Graph selbst ändert sich nicht."))
        .action_describe("focusSelection", LocalizedLabel::native("Zooms the main window onto the selected widgets; only the view changes.", "Zoomt das Hauptfenster auf die ausgewählten Widgets; nur die Ansicht ändert sich."))
        .action_describe("addGeneration", LocalizedLabel::native("Adds a new parameter generation, a named set of input values, to the generate mode's list; generations live in the window, not in the document.", "Fügt der Liste des Generieren-Modus eine neue Parametergeneration hinzu, einen benannten Satz von Eingabewerten; Generationen liegen im Fenster, nicht im Dokument."))
        .action_describe("removeGeneration", LocalizedLabel::native("Removes one parameter generation by id from the generate mode's list; the flow graph is not changed.", "Entfernt eine Parametergeneration anhand ihrer Id aus der Liste des Generieren-Modus; der Flow-Graph ändert sich nicht."))
        .action_describe("renameGeneration", LocalizedLabel::native("Renames one parameter generation of the generate mode's list.", "Benennt eine Parametergeneration in der Liste des Generieren-Modus um."))
        .action_describe("updateGenerationValues", LocalizedLabel::native("Sets the value one input question takes in a parameter generation (the selected one when no id is given) and re-evaluates its preview.", "Setzt den Wert, den eine Eingabefrage in einer Parametergeneration annimmt (ohne Id in der ausgewählten), und wertet ihre Vorschau neu aus."))
        .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
        .action_audience("spotlightCommit", semio_framework_plugin::CapabilityAudience::Input)
        .action_destructive("reorganize")
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

//#region 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️interactive-job/🦀️.rs"]
mod interactive_job_tests;

#[cfg(test)]
#[path = "🧪️tests/⚖️declared-verbs/🦀️.rs"]
mod declared_verb_laws;
//#endregion 🧪️Tests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts

#[cfg(test)]
#[path="🧪️tests/📨️child-preparation/🦀️.rs"]
mod child_preparation_owner_tests;
