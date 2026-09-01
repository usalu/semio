//! 🖥️ Flow play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `🎚️options/*`, panel trees in `📌️panels/*`,
//! labels in `🗣️terminology/🦀️component.rs`, view state in `🎚️config/🦀️component.rs`, plugin registration
//! and `FlowHost` bridging (below — constitutional: general, an artifact must never depend on an app, so
//! both live here rather than under `🗿️artifacts`).
//! This file is a routing table: `handle` → `FlowCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::artifacts::flow::op::FlowMutation;
use crate::artifacts::flow::schema::mutations::connect_widgets::mutation::ConnectWidgets;
use crate::artifacts::flow::schema::mutations::create_widget::mutation::CreateWidget;
use crate::artifacts::flow::schema::mutations::delete_widget::mutation::DeleteWidget;
use crate::artifacts::flow::schema::mutations::disconnect_widgets::mutation::DisconnectWidgets;
use crate::artifacts::flow::schema::mutations::move_widgets::mutation::MoveWidgets;
use crate::artifacts::flow::schema::mutations::reorder_synapses::mutation::ReorderSynapses;
use crate::artifacts::flow::schema::mutations::reorder_widgets::mutation::ReorderWidgets;
use crate::artifacts::flow::schema::mutations::replace_widget::mutation::ReplaceWidget;
use crate::artifacts::flow::schema::mutations::update_synapse_endpoints::mutation::UpdateSynapseEndpoints;
use crate::artifacts::flow::{flow_content_child_handle_bounded, FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
use crate::editor::flow::commands::{
    add_widget, connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, duplicate_widget_step, evaluate, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport,
    open_spotlight, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_catalogue_sections, set_contributions, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_locale,
    set_lod_mode, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension,
};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use crate::editor::flow::modes::edit::windows::{compiled, main};
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
use crate::editor::flow::modes::{edit, generate};
use crate::editor::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::flow::presence::{FlowPresence, FlowPresenceMutation};
use crate::editor::flow::terminology::{flow_play_labels, FlowPlayLabels};
use flow::{dag::DagDrawLod, flow_fixture_operations, flow_host_with_session, CameraJson, FlowEvalSession, FlowHost, Widget, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppActionRegistry, AppDefinition, ArtifactEditor, ArtifactView, CommandDefinition, ConfigView, ContextMenuItemSpec, ContextMenuRequest, Dialect,
    DomainTopology, DraftView, Editor, Effect, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod,
    SelectionMode, SelectionSpec, TopologyNode, UiNode, WindowMeasure,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use store::EngineHandles;

#[path = "🧵️retained/🦀️component.rs"]
mod retained;

//#region 🔖️Constants
pub const FLOW_PLAY_APP_ID: &str = "flow-play";
pub use catalogue_panel::FLOW_PLAY_BODY_CATALOGUE;
pub use compiled::FLOW_PLAY_BODY_COMPILED;
pub use document_panel::FLOW_PLAY_BODY_DOCUMENT;
pub use form::FLOW_PLAY_BODY_GENERATE_FORM;
pub use generations::FLOW_PLAY_BODY_GENERATIONS;
pub use inspection_panel::FLOW_PLAY_BODY_INSPECTOR;
pub use main::FLOW_PLAY_BODY_MAIN;
pub use preview::FLOW_PLAY_BODY_GENERATE_PREVIEW;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`) builds its `on_change`/item actions with.
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

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
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
const FLOW_GRAPH_NODE_TARGET_PREFIX: &str = "flow-play-document.widget.";
/// 🕹️ Same as `FLOW_GRAPH_NODE_TARGET_PREFIX`, for "edge"-granularity targets (synapses).
const FLOW_GRAPH_EDGE_TARGET_PREFIX: &str = "flow-play-document.synapse.";

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
    /// `setLocale`/`locale` is the row that proves it. **Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break.**
    ///
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `setSelection`/`clearSelection`/
    /// `selectAll`/`selectNode`/`nodeGraphSelect`/`nodeGraphHover`/`graphPointerDown` are deleted — the
    /// framework auto-injects `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/
    /// `setSelectionMode`/`setInteractionGranularity` for the declared "graph" domain instead (see
    /// `🔖️Manifest`). `deleteSelection`/`focusSelection`/`nodeGraphEdit`/`spotlightCommit` read that
    /// domain's live selection via `InteractionView` — `FlowPlayApp::handle` routes them through their
    /// own `apply` (this macro's generated `dispatch(doc, cfg, session)` has no `interaction` slot).
    pub enum FlowCommand for FlowSnapshot, FlowMutation, FlowConfig, FlowConfigMutation, ctx = FlowEvalSession {
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
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
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
        "setLocale" as "locale" => set_locale::SetLocale,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "duplicateWidgetStep" as "duplicate-widget-step" => duplicate_widget_step::DuplicateWidgetStep,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
/// 🖱️ On-demand flow node-graph context menu from surface hit-test and selection snapshot.
fn flow_context_menu_items(registry: &AppActionRegistry, fixture: &FlowSnapshot, config: &FlowConfig, labels: &FlowPlayLabels, is_de: bool, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, Menu};

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
    let is_image = nodes.len() == 1
        && fixture.to_fixture().widgets.iter().any(|widget| match widget {
            Widget::InputImage { id, .. } => id == &nodes[0],
            _ => false,
        });
    let primary = hits.first();
    let hit_node = primary.filter(|hit| hit.domain == "node").map(|hit| hit.id.as_str());

    // 🗂️ Grouped disclosure: `add-node`/`selectAll`/`focusSelection`/`clearSelection` stay top-level
    // (the 3-5 most frequent verbs); `reorganize`/`replaceImage`/`toggle-preview` fold into taxonomy
    // groups; `delete-selection` stays a direct destructive item last — `organize_context_menu`
    // (applied automatically at the `VcsArtifactApp::context_menu` funnel) sorts the groups into
    // `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive separator itself.
    let mut menu = Menu::of(registry);
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
                args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "widgetId": node_id }))),
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
                    args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "id": node_id }))),
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
                args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "ids": nodes, "value": !all_preview_off }))),
                ..Default::default()
            })
        });
        let phrase = selection_count_phrase(is_de, &[(nodes.len(), if is_de { "Knoten" } else { "node" }, if is_de { "Knoten" } else { "nodes" }), (edges.len(), if is_de { "Kante" } else { "edge" }, if is_de { "Kanten" } else { "edges" })]);
        if !phrase.is_empty() {
            menu = menu.item(ContextMenuItemSpec {
                id: "delete-selection".into(),
                label: Some(format!("{} ({phrase})", labels.delete_selection.as_str())),
                icon: Some("trash".into()),
                destructive: Some(true),
                action: Some("deleteSelection".into()),
                ..Default::default()
            });
        }
    }
    menu.build()
}
//#endregion 🔖️ContextMenu

//#region 📬️StorePreparation
const FLOW_STORE_MAX_SCENE_ITEMS: usize = 256;
const FLOW_STORE_MAX_TEXT_BYTES: usize = 16_384;
const FLOW_STORE_MAX_MUTATION_ITEMS: usize = 256;

type FlowStorePrepare<P, M> = fn(&P, M) -> Result<(P, Vec<M>, M), String>;
type FlowStoreAdmit<M> = fn(&M) -> Result<store::ArtifactStoreOneItemFootprint, String>;

struct FlowStoreOneItemPreparationFactory<P, M> {
    lane: store::HistoryLane,
    admit: FlowStoreAdmit<M>,
    prepare: FlowStorePrepare<P, M>,
}

impl<P, M> FlowStoreOneItemPreparationFactory<P, M> {
    fn new(lane: store::HistoryLane, admit: FlowStoreAdmit<M>, prepare: FlowStorePrepare<P, M>) -> Self {
        Self { lane, admit, prepare }
    }
}

struct FlowStoreOneItemPreparation<P, M> {
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepare: FlowStorePrepare<P, M>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn flow_store_edit<M>(forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("flow-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

fn flow_config_text_bytes(config: &FlowConfig) -> usize {
    config.preview_off_node_ids.iter().map(String::len).sum::<usize>()
        + config.lod_mode.len()
        + config.catalogue_sections_json.len()
        + config.automation_enabled_json.len()
        + config.contributions_json.len()
        + config.generation_json.len()
        + config.duplicate_widget_progress_json.len()
        + config.locale.len()
}

fn flow_config_mutation_text_bytes(mutation: &FlowConfigMutation) -> usize {
    match mutation {
        FlowConfigMutation::SetContributions { json }
        | FlowConfigMutation::SetCatalogueSections { sections_json: json }
        | FlowConfigMutation::SetAutomationEnabled { json }
        | FlowConfigMutation::SetGeneration { json }
        | FlowConfigMutation::SetDuplicateWidgetProgress { json } => json.len(),
        FlowConfigMutation::Snapshot { config } => flow_config_text_bytes(config),
        FlowConfigMutation::SetPreviewOff { node_ids } => node_ids.iter().map(String::len).sum(),
        FlowConfigMutation::SetLodMode { value } | FlowConfigMutation::SetLocale { value } => value.len(),
        FlowConfigMutation::SetCamera { .. }
        | FlowConfigMutation::SetProximityDistance { .. }
        | FlowConfigMutation::SetGridVisible { .. }
        | FlowConfigMutation::SetGridSnapEnabled { .. }
        | FlowConfigMutation::SetGridFactor { .. }
        | FlowConfigMutation::CancelDuplicateWidget { .. } => 0,
    }
}

fn admit_flow_config_mutation(mutation: &FlowConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let item_count = match mutation {
        FlowConfigMutation::SetPreviewOff { node_ids } => node_ids.len(),
        FlowConfigMutation::Snapshot { config } => config.preview_off_node_ids.len(),
        _ => 1,
    };
    if item_count > FLOW_STORE_MAX_SCENE_ITEMS {
        return Err("Flow config mutation exceeds its fixed retained preparation envelope".into());
    }
    let retained_bytes = flow_config_mutation_text_bytes(mutation);
    if retained_bytes > FLOW_STORE_MAX_TEXT_BYTES {
        return Err("Flow config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: item_count.max(1), retained_bytes })
}

fn prepare_flow_config(base: &FlowConfig, mutation: FlowConfigMutation) -> Result<(FlowConfig, Vec<FlowConfigMutation>, FlowConfigMutation), String> {
    admit_flow_config_mutation(&mutation)?;
    if base.preview_off_node_ids.len() > FLOW_STORE_MAX_SCENE_ITEMS || flow_config_text_bytes(base) > FLOW_STORE_MAX_TEXT_BYTES {
        return Err("Flow config base exceeds its fixed retained preparation envelope".into());
    }
    let mut post = base.clone();
    match &mutation {
        FlowConfigMutation::Snapshot { config } => post = config.clone(),
        FlowConfigMutation::SetPreviewOff { node_ids } => post.preview_off_node_ids = node_ids.clone(),
        FlowConfigMutation::SetCamera { camera } => post.camera = camera.clone(),
        FlowConfigMutation::SetLodMode { value } => post.lod_mode = value.clone(),
        FlowConfigMutation::SetProximityDistance { value } => post.proximity_distance = *value,
        FlowConfigMutation::SetGridVisible { value } => post.grid_visible = *value,
        FlowConfigMutation::SetGridSnapEnabled { value } => post.grid_snap_enabled = *value,
        FlowConfigMutation::SetGridFactor { value } => post.grid_factor = *value,
        FlowConfigMutation::SetCatalogueSections { sections_json } => post.catalogue_sections_json = sections_json.clone(),
        FlowConfigMutation::SetAutomationEnabled { json } => post.automation_enabled_json = json.clone(),
        FlowConfigMutation::SetGeneration { json } => post.generation_json = json.clone(),
        FlowConfigMutation::SetDuplicateWidgetProgress { json } => post.duplicate_widget_progress_json = json.clone(),
        FlowConfigMutation::CancelDuplicateWidget { generation } => {
            let active = serde_json::from_str::<serde_json::Value>(&post.duplicate_widget_progress_json).ok().and_then(|value| value.get("generation").and_then(serde_json::Value::as_u64));
            if active == Some(*generation) {
                post.duplicate_widget_progress_json.clear();
            }
        }
        FlowConfigMutation::SetContributions { .. } => {
            return Err("Flow contribution publication requires a post-ACK app-instance host synchronization hook".into());
        }
        FlowConfigMutation::SetLocale { value } => post.locale = value.clone(),
    }
    let inverse = FlowConfigMutation::Snapshot { config: base.clone() };
    Ok((post, vec![inverse], mutation))
}

struct FlowBoundedByteCounter {
    written: usize,
    maximum_bytes: usize,
}

impl Write for FlowBoundedByteCounter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let next = self.written.checked_add(bytes.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Flow retained byte count overflow"))?;
        if next > self.maximum_bytes {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Flow retained value exceeds its byte cap"));
        }
        self.written = next;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn flow_bounded_serialized_bytes<T: serde::Serialize>(value: &T, maximum_bytes: usize) -> Result<usize, String> {
    let mut counter = FlowBoundedByteCounter { written: 0, maximum_bytes };
    serde_json::to_writer(&mut counter, value).map_err(|error| error.to_string())?;
    Ok(counter.written)
}

fn flow_artifact_mutation_items(mutation: &FlowMutation) -> usize {
    match mutation {
        FlowMutation::MoveWidgets(payload) => payload.entries.len(),
        FlowMutation::DuplicateWidget(_) => 2,
        _ => 1,
    }
}

fn admit_flow_artifact_mutation(mutation: &FlowMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let work_items = flow_artifact_mutation_items(mutation);
    if work_items == 0 || work_items > FLOW_STORE_MAX_MUTATION_ITEMS {
        return Err("Flow artifact mutation exceeds its fixed semantic-item cap".into());
    }
    let retained_bytes = flow_bounded_serialized_bytes(mutation, FLOW_STORE_MAX_TEXT_BYTES)?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes })
}

fn flow_widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

fn prepare_flow_artifact(base: &FlowSnapshot, mutation: FlowMutation) -> Result<(FlowSnapshot, Vec<FlowMutation>, FlowMutation), String> {
    admit_flow_artifact_mutation(&mutation)?;
    let owner = base.content.local_owner::<FlowWorkingScene>().ok_or_else(|| "Flow artifact base has no exact app-instance scene owner".to_string())?;
    if owner.widgets.len() > FLOW_STORE_MAX_SCENE_ITEMS || owner.synapses.len() > FLOW_STORE_MAX_SCENE_ITEMS || owner.layout.len() > FLOW_STORE_MAX_SCENE_ITEMS {
        return Err("Flow artifact base exceeds its fixed scene-item cap".into());
    }
    flow_bounded_serialized_bytes(&*owner, FLOW_STORE_MAX_TEXT_BYTES)?;
    let mut scene = (*owner).clone();
    let inverse = match &mutation {
        FlowMutation::CreateWidget(payload) => {
            let id = flow_widget_id(&payload.widget);
            if scene.widgets.iter().any(|widget| flow_widget_id(widget) == id) || scene.widgets.len() == FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(format!("Flow create-widget rejected duplicate or capped id {id:?}"));
            }
            scene.widgets.insert(payload.index.min(scene.widgets.len()), payload.widget.clone());
            vec![FlowMutation::DeleteWidget(DeleteWidget { id: id.to_string() })]
        }
        FlowMutation::DeleteWidget(payload) => {
            let index = scene.widgets.iter().position(|widget| flow_widget_id(widget) == payload.id).ok_or_else(|| format!("Flow delete-widget target {:?} is missing", payload.id))?;
            let widget = scene.widgets[index].clone();
            let mut inverses = vec![FlowMutation::CreateWidget(CreateWidget { index, widget })];
            if let Some(layout) = scene.layout.get(&payload.id) {
                inverses.push(FlowMutation::MoveWidgets(MoveWidgets { entries: vec![flow::FlowLayoutEntry { id: payload.id.clone(), layout: Some(layout.clone()) }] }));
            }
            for (synapse_index, synapse) in scene.synapses.iter().enumerate().filter(|(_, synapse)| synapse.from == payload.id || synapse.to == payload.id) {
                inverses.push(FlowMutation::ConnectWidgets(ConnectWidgets { index: synapse_index, id: synapse.id.clone(), from: synapse.from.clone(), from_port: synapse.from_port.clone(), to: synapse.to.clone(), to_port: synapse.to_port.clone() }));
            }
            scene.widgets.remove(index);
            scene.synapses.retain(|synapse| synapse.from != payload.id && synapse.to != payload.id);
            scene.layout.remove(&payload.id);
            inverses
        }
        FlowMutation::ReorderWidgets(payload) => {
            let from = scene.widgets.iter().position(|widget| flow_widget_id(widget) == payload.id).ok_or_else(|| format!("Flow reorder-widget target {:?} is missing", payload.id))?;
            let to = payload.to_index.min(scene.widgets.len().saturating_sub(1));
            if from == to {
                return Err("Flow reorder-widget is a no-op".into());
            }
            let widget = scene.widgets.remove(from);
            scene.widgets.insert(to, widget);
            vec![FlowMutation::ReorderWidgets(ReorderWidgets { id: payload.id.clone(), to_index: from })]
        }
        FlowMutation::ReplaceWidget(payload) => {
            let current = scene.widgets.iter_mut().find(|widget| flow_widget_id(widget) == payload.id).ok_or_else(|| format!("Flow replace-widget target {:?} is missing", payload.id))?;
            if current == &payload.widget {
                return Err("Flow replace-widget is a no-op".into());
            }
            let previous = std::mem::replace(current, payload.widget.clone());
            vec![FlowMutation::ReplaceWidget(ReplaceWidget { id: payload.id.clone(), widget: previous })]
        }
        FlowMutation::ConnectWidgets(payload) => {
            if scene.synapses.len() == FLOW_STORE_MAX_SCENE_ITEMS || scene.synapses.iter().any(|synapse| synapse.id == payload.id) {
                return Err("Flow connect-widgets rejected duplicate or capped synapse".into());
            }
            if !scene.widgets.iter().any(|widget| flow_widget_id(widget) == payload.from) || !scene.widgets.iter().any(|widget| flow_widget_id(widget) == payload.to) {
                return Err("Flow connect-widgets endpoint is missing".into());
            }
            if scene.synapses.iter().any(|synapse| synapse.from == payload.from && synapse.from_port == payload.from_port && synapse.to == payload.to && synapse.to_port == payload.to_port) {
                return Err("Flow connect-widgets parallel edge is a no-op".into());
            }
            scene.synapses.insert(payload.index.min(scene.synapses.len()), flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() });
            vec![FlowMutation::DisconnectWidgets(DisconnectWidgets { id: payload.id.clone() })]
        }
        FlowMutation::DisconnectWidgets(payload) => {
            let index = scene.synapses.iter().position(|synapse| synapse.id == payload.id).ok_or_else(|| format!("Flow disconnect-widgets target {:?} is missing", payload.id))?;
            let synapse = scene.synapses.remove(index);
            vec![FlowMutation::ConnectWidgets(ConnectWidgets { index, id: synapse.id, from: synapse.from, from_port: synapse.from_port, to: synapse.to, to_port: synapse.to_port })]
        }
        FlowMutation::ReorderSynapses(payload) => {
            let from = scene.synapses.iter().position(|synapse| synapse.id == payload.id).ok_or_else(|| format!("Flow reorder-synapse target {:?} is missing", payload.id))?;
            let to = payload.to_index.min(scene.synapses.len().saturating_sub(1));
            if from == to {
                return Err("Flow reorder-synapse is a no-op".into());
            }
            let synapse = scene.synapses.remove(from);
            scene.synapses.insert(to, synapse);
            vec![FlowMutation::ReorderSynapses(ReorderSynapses { id: payload.id.clone(), to_index: from })]
        }
        FlowMutation::UpdateSynapseEndpoints(payload) => {
            if !scene.widgets.iter().any(|widget| flow_widget_id(widget) == payload.from) || !scene.widgets.iter().any(|widget| flow_widget_id(widget) == payload.to) {
                return Err("Flow update-synapse endpoint is missing".into());
            }
            let synapse = scene.synapses.iter_mut().find(|synapse| synapse.id == payload.id).ok_or_else(|| format!("Flow update-synapse target {:?} is missing", payload.id))?;
            if synapse.from == payload.from && synapse.from_port == payload.from_port && synapse.to == payload.to && synapse.to_port == payload.to_port {
                return Err("Flow update-synapse is a no-op".into());
            }
            let inverse = FlowMutation::UpdateSynapseEndpoints(UpdateSynapseEndpoints { id: payload.id.clone(), from: synapse.from.clone(), from_port: synapse.from_port.clone(), to: synapse.to.clone(), to_port: synapse.to_port.clone() });
            synapse.from = payload.from.clone();
            synapse.from_port = payload.from_port.clone();
            synapse.to = payload.to.clone();
            synapse.to_port = payload.to_port.clone();
            vec![inverse]
        }
        FlowMutation::MoveWidgets(payload) => {
            if payload.entries.is_empty() {
                return Err("Flow move-widgets has no semantic items".into());
            }
            let mut inverse_entries = Vec::with_capacity(payload.entries.len());
            for entry in &payload.entries {
                if !scene.widgets.iter().any(|widget| flow_widget_id(widget) == entry.id) {
                    return Err(format!("Flow move-widget target {:?} is missing", entry.id));
                }
                if entry.layout.as_ref().is_some_and(|layout| !layout.x.is_finite() || !layout.y.is_finite()) {
                    return Err(format!("Flow move-widget target {:?} has a non-finite position", entry.id));
                }
                inverse_entries.push(flow::FlowLayoutEntry { id: entry.id.clone(), layout: scene.layout.get(&entry.id).cloned() });
            }
            for entry in &payload.entries {
                if let Some(layout) = &entry.layout {
                    scene.layout.insert(entry.id.clone(), layout.clone());
                } else {
                    scene.layout.remove(&entry.id);
                }
            }
            vec![FlowMutation::MoveWidgets(MoveWidgets { entries: inverse_entries })]
        }
        FlowMutation::DuplicateWidget(payload) => {
            if payload.source_id == payload.new_id || scene.widgets.iter().any(|widget| flow_widget_id(widget) == payload.new_id) || scene.synapses.iter().any(|synapse| synapse.id == payload.synapse_id) {
                return Err("Flow duplicate-widget target ids are invalid or occupied".into());
            }
            if scene.widgets.len() == FLOW_STORE_MAX_SCENE_ITEMS || scene.synapses.len() == FLOW_STORE_MAX_SCENE_ITEMS {
                return Err("Flow duplicate-widget exceeds its fixed scene-item cap".into());
            }
            let source = scene.widgets.iter().find(|widget| flow_widget_id(widget) == payload.source_id).ok_or_else(|| format!("Flow duplicate-widget source {:?} is missing", payload.source_id))?;
            let copy = crate::artifacts::flow::schema::widget_with_id(source, payload.new_id.clone());
            scene.widgets.push(copy);
            scene.synapses.push(flow::SynapseSpec { id: payload.synapse_id.clone(), from: payload.source_id.clone(), from_port: payload.from_port.clone(), to: payload.new_id.clone(), to_port: payload.to_port.clone() });
            vec![FlowMutation::DisconnectWidgets(DisconnectWidgets { id: payload.synapse_id.clone() }), FlowMutation::DeleteWidget(DeleteWidget { id: payload.new_id.clone() })]
        }
    };
    let content = flow_content_child_handle_bounded(&scene.widgets, &scene.synapses, &scene.layout, FLOW_STORE_MAX_TEXT_BYTES)?;
    let post = FlowSnapshot { schema: base.schema.clone(), camera: base.camera.clone(), content };
    Ok((post, inverse, mutation))
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for FlowStoreOneItemPreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: Clone + serde::Serialize + Send + Sync + 'static,
{
    fn preflight(&self, mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != self.lane || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Flow one-item preparation rejected its lane or description envelope".into());
        }
        (self.admit)(mutation)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        if request.lane != self.lane
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(FlowStoreOneItemPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepare: self.prepare,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for FlowStoreOneItemPreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: Clone + serde::Serialize + Send + Sync + 'static,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Flow preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Flow preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = (self.prepare)(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "Flow preparation lost its Store authority".to_string())?;
        let edit = flow_store_edit(forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Flow preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

//#region 🧵️DirectStoreLaneRoutes
const FLOW_DIRECT_STORE_TOOL_IDS: &[&str] = &[
    "removeWidget",
    "deleteSelection",
    "disconnect",
    "moveMediaNode",
    "patchFlowWidgets",
    "nodeGraphViewport",
    "setLodMode",
    "setProximityDistance",
    "setGridVisible",
    "setGridSnapEnabled",
    "setGridFactor",
    "setPreviewOff",
    "setCatalogueSections",
    "toggleExtension",
    "setLocale",
];
const FLOW_DIRECT_STORE_RAW_BYTES: usize = 16_384;

fn flow_direct_store_emit(command: &FlowCommand, snapshot: &FlowSnapshot, config: &FlowConfig, _operation: &semio_framework_plugin::AppOperationContext) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let scene = snapshot.content.local_owner::<FlowWorkingScene>().ok_or_else(|| Fault::from("flow-retained-scene-owner-missing"))?;
    if scene.widgets.len() > FLOW_STORE_MAX_SCENE_ITEMS || scene.synapses.len() > FLOW_STORE_MAX_SCENE_ITEMS || scene.layout.len() > FLOW_STORE_MAX_SCENE_ITEMS {
        return Err(Fault::from("flow-retained-scene-capacity"));
    }
    Ok(match command {
        FlowCommand::NodeGraphViewport(payload) => Emit::config(vec![FlowConfigMutation::SetCamera { camera: payload.camera.clone() }]),
        FlowCommand::SetLodMode(payload) => {
            if payload.value == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(&payload.value).is_some() {
                Emit::config(vec![FlowConfigMutation::SetLodMode { value: payload.value.clone() }])
            } else {
                Emit::default()
            }
        }
        FlowCommand::SetProximityDistance(payload) => Emit::config(vec![FlowConfigMutation::SetProximityDistance { value: payload.value.max(0.0) }]),
        FlowCommand::SetGridVisible(payload) => Emit::config(vec![FlowConfigMutation::SetGridVisible { value: payload.pressed.unwrap_or(!config.grid_visible) }]),
        FlowCommand::SetGridSnapEnabled(payload) => Emit::config(vec![FlowConfigMutation::SetGridSnapEnabled { value: payload.pressed.unwrap_or(!config.grid_snap_enabled) }]),
        FlowCommand::SetGridFactor(payload) => Emit::config(vec![FlowConfigMutation::SetGridFactor { value: payload.value.clamp(0.5, 50.0) }]),
        FlowCommand::SetCatalogueSections(payload) => Emit::config(vec![FlowConfigMutation::SetCatalogueSections { sections_json: payload.sections_json.clone() }]),
        FlowCommand::ToggleExtension(payload) => {
            if config.automation_enabled_json.len() > FLOW_STORE_MAX_TEXT_BYTES || payload.id.len() > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-extension-capacity"));
            }
            let mut enabled = serde_json::from_str::<HashMap<String, bool>>(&config.automation_enabled_json).unwrap_or_default();
            if enabled.len() >= FLOW_STORE_MAX_SCENE_ITEMS && !enabled.contains_key(&payload.id) {
                return Err(Fault::from("flow-retained-extension-item-capacity"));
            }
            enabled.insert(payload.id.clone(), payload.enabled);
            let json = serde_json::to_string(&enabled).map_err(|_| Fault::from("flow-retained-extension-encode"))?;
            Emit::config(vec![FlowConfigMutation::SetAutomationEnabled { json }])
        }
        FlowCommand::SetLocale(payload) => Emit::config(vec![FlowConfigMutation::SetLocale { value: payload.value.clone() }]),
        _ => return Err(Fault::from("flow-retained-direct-route-mismatch")),
    })
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
    edge_mutations: Option<Vec<FlowMutation>>,
    node_mutations: Option<Vec<FlowMutation>>,
    artifact_mutations: Option<Vec<FlowMutation>>,
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
            edge_mutations: None,
            node_mutations: None,
            artifact_mutations: None,
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

    fn extent(&self, command: &FlowCommand, snapshot: &FlowSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>) -> Option<usize> {
        if command.command_id() != self.tool_id || !FLOW_DIRECT_STORE_TOOL_IDS.contains(&self.tool_id) {
            return None;
        }
        match command {
            FlowCommand::SetPreviewOff(payload) if payload.ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && payload.ids.iter().map(String::len).fold(0, usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES => {
                Some(payload.ids.len().saturating_mul(FLOW_STORE_MAX_SCENE_ITEMS.max(1)).max(1))
            }
            FlowCommand::SetPreviewOff(_) => None,
            FlowCommand::RemoveWidget(_) | FlowCommand::MoveMediaNode(_) => snapshot.content.local_owner::<FlowWorkingScene>().filter(|scene| scene.widgets.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|scene| scene.widgets.len().max(1)),
            FlowCommand::Disconnect(_) => snapshot.content.local_owner::<FlowWorkingScene>().filter(|scene| scene.synapses.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|scene| scene.synapses.len().max(1)),
            FlowCommand::DeleteSelection(_) => match interaction.selection.get(FLOW_INTERACTION_GRAPH) {
                Some(selection) if selection.ids.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && selection.ids.iter().map(String::len).fold(0, usize::saturating_add) <= FLOW_STORE_MAX_TEXT_BYTES => {
                    snapshot.content.local_owner::<FlowWorkingScene>().filter(|scene| scene.widgets.len() <= FLOW_STORE_MAX_SCENE_ITEMS && scene.synapses.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|scene| {
                        selection
                            .ids
                            .iter()
                            .fold(0usize, |extent, target| {
                                extent.saturating_add(if target.starts_with(FLOW_GRAPH_EDGE_TARGET_PREFIX) {
                                    scene.synapses.len().max(1)
                                } else if target.starts_with(FLOW_GRAPH_NODE_TARGET_PREFIX) {
                                    scene.widgets.len().max(1)
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
                snapshot.content.local_owner::<FlowWorkingScene>().filter(|scene| scene.widgets.len() <= FLOW_STORE_MAX_SCENE_ITEMS).map(|scene| scene.widgets.len().saturating_mul(payload.widget_ids.len().max(1)).max(1))
            }
            FlowCommand::PatchFlowWidgets(_) => None,
            _ => Some(1),
        }
    }

    fn step(
        &mut self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        config: &FlowConfig,
        _history: &semio_framework_plugin::HistoryView,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
        operation: &semio_framework_plugin::AppOperationContext,
    ) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        if self.completed || self.closing {
            return Err(Fault::from("flow-retained-direct-work-terminal"));
        }
        if matches!(command, FlowCommand::RemoveWidget(_) | FlowCommand::Disconnect(_) | FlowCommand::MoveMediaNode(_)) {
            let scene = snapshot.content.local_owner::<FlowWorkingScene>().ok_or_else(|| Fault::from("flow-retained-scene-owner-missing"))?;
            if scene.widgets.len() > FLOW_STORE_MAX_SCENE_ITEMS || scene.synapses.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            self.artifact_mutations.get_or_insert_with(Vec::new);
            let (length, matched) = match command {
                FlowCommand::RemoveWidget(payload) => (scene.widgets.len(), scene.widgets.get(self.cursor).is_some_and(|widget| flow_widget_id(widget) == payload.widget_id)),
                FlowCommand::Disconnect(payload) => (scene.synapses.len(), scene.synapses.get(self.cursor).is_some_and(|synapse| synapse.id == payload.synapse_id)),
                FlowCommand::MoveMediaNode(payload) => (scene.widgets.len(), scene.widgets.get(self.cursor).is_some_and(|widget| flow_widget_id(widget) == payload.node_id)),
                _ => unreachable!(),
            };
            if self.cursor < length {
                if matched {
                    let mutation = match command {
                        FlowCommand::RemoveWidget(payload) => Some(FlowMutation::DeleteWidget(DeleteWidget { id: payload.widget_id.clone() })),
                        FlowCommand::Disconnect(payload) => Some(FlowMutation::DisconnectWidgets(DisconnectWidgets { id: payload.synapse_id.clone() })),
                        FlowCommand::MoveMediaNode(payload) if payload.x.is_finite() && payload.y.is_finite() => {
                            let requested = flow::WidgetLayout { x: payload.x, y: payload.y };
                            (scene.layout.get(&payload.node_id) != Some(&requested)).then(|| FlowMutation::MoveWidgets(MoveWidgets { entries: vec![flow::FlowLayoutEntry { id: payload.node_id.clone(), layout: Some(requested) }] }))
                        }
                        FlowCommand::MoveMediaNode(_) => None,
                        _ => unreachable!(),
                    };
                    if let Some(mutation) = mutation {
                        self.artifact_mutations.as_mut().ok_or_else(|| Fault::from("flow-retained-direct-artifact-owner"))?.push(mutation);
                    }
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
            let mutations = self.artifact_mutations.take().ok_or_else(|| Fault::from("flow-retained-direct-artifact-owner"))?;
            self.completed = true;
            return Ok(ArtifactCommandWorkStep::Complete(if mutations.is_empty() {
                Emit::default()
            } else if let FlowCommand::MoveMediaNode(payload) = command {
                Emit::amend(mutations, format!("move-{}", payload.node_id))
            } else {
                Emit::mutations(mutations)
            }));
        }
        if matches!(command, FlowCommand::DeleteSelection(_)) {
            let selected = interaction.selection.get(FLOW_INTERACTION_GRAPH).map_or(&[][..], |selection| selection.ids.as_slice());
            if selected.len() > FLOW_STORE_MAX_MUTATION_ITEMS || selected.iter().map(String::len).sum::<usize>() > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-delete-selection-capacity"));
            }
            let scene = snapshot.content.local_owner::<FlowWorkingScene>().ok_or_else(|| Fault::from("flow-retained-scene-owner-missing"))?;
            if scene.widgets.len() > FLOW_STORE_MAX_SCENE_ITEMS || scene.synapses.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            self.edge_mutations.get_or_insert_with(Vec::new);
            self.node_mutations.get_or_insert_with(Vec::new);
            if let Some(target) = selected.get(self.cursor) {
                if let Some(id) = target.strip_prefix(FLOW_GRAPH_EDGE_TARGET_PREFIX) {
                    if let Some(synapse) = scene.synapses.get(self.scan_cursor) {
                        self.scan_cursor += 1;
                        let matched = synapse.id == id;
                        if matched {
                            self.edge_mutations.as_mut().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?.push(FlowMutation::DisconnectWidgets(DisconnectWidgets { id: id.to_string() }));
                        }
                        if matched || self.scan_cursor == scene.synapses.len() {
                            self.cursor += 1;
                            self.scan_cursor = 0;
                        }
                    } else {
                        self.cursor += 1;
                        self.scan_cursor = 0;
                    }
                } else if let Some(id) = target.strip_prefix(FLOW_GRAPH_NODE_TARGET_PREFIX) {
                    if let Some(widget) = scene.widgets.get(self.scan_cursor) {
                        self.scan_cursor += 1;
                        let matched = flow_widget_id(widget) == id;
                        if matched {
                            self.node_mutations.as_mut().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?.push(FlowMutation::DeleteWidget(DeleteWidget { id: id.to_string() }));
                        }
                        if matched || self.scan_cursor == scene.widgets.len() {
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
            let mut artifact_mutations = self.edge_mutations.take().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?;
            artifact_mutations.extend(self.node_mutations.take().ok_or_else(|| Fault::from("flow-retained-delete-selection-owner"))?);
            self.completed = true;
            return Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(artifact_mutations)));
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
            return Ok(ArtifactCommandWorkStep::Complete(Emit::config(vec![FlowConfigMutation::SetPreviewOff { node_ids }])));
        }
        if let FlowCommand::PatchFlowWidgets(payload) = command {
            let input_bytes = payload.widget_ids.iter().map(String::len).fold(payload.field.len().saturating_add(payload.value.len()), usize::saturating_add);
            if payload.widget_ids.len() > FLOW_STORE_MAX_MUTATION_ITEMS || input_bytes > FLOW_STORE_MAX_TEXT_BYTES {
                return Err(Fault::from("flow-retained-patch-widgets-capacity"));
            }
            let scene = snapshot.content.local_owner::<FlowWorkingScene>().ok_or_else(|| Fault::from("flow-retained-scene-owner-missing"))?;
            if scene.widgets.len() > FLOW_STORE_MAX_SCENE_ITEMS {
                return Err(Fault::from("flow-retained-scene-capacity"));
            }
            self.artifact_mutations.get_or_insert_with(Vec::new);
            if let Some(widget) = scene.widgets.get(self.cursor) {
                if let Some(id) = payload.widget_ids.get(self.scan_cursor) {
                    self.scan_cursor += 1;
                    let matched = id == flow_widget_id(widget);
                    if matched {
                        let mut replacement = widget.clone();
                        match (payload.field.as_str(), &mut replacement) {
                            ("value", Widget::InputSlider { value, .. }) => {
                                if let Ok(parsed) = payload.value.parse::<f64>() {
                                    *value = parsed;
                                }
                            }
                            ("text", Widget::InputNote { text, .. }) => *text = payload.value.clone(),
                            _ => {}
                        }
                        if replacement != *widget {
                            self.artifact_mutations.as_mut().ok_or_else(|| Fault::from("flow-retained-patch-widgets-owner"))?.push(FlowMutation::ReplaceWidget(ReplaceWidget { id: flow_widget_id(widget).to_string(), widget: replacement }));
                        }
                    }
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
            let mutations = self.artifact_mutations.take().ok_or_else(|| Fault::from("flow-retained-patch-widgets-owner"))?;
            self.completed = true;
            let widget_ids_separator = ",";
            return Ok(ArtifactCommandWorkStep::Complete(if mutations.is_empty() { Emit::default() } else { Emit::amend(mutations, format!("patch-{}-{}", payload.field, payload.widget_ids.join(widget_ids_separator))) }));
        }
        self.completed = true;
        flow_direct_store_emit(command, snapshot, config, operation).map(ArtifactCommandWorkStep::Complete)
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 17 {
            return Err(Fault::from("flow-retained-direct-checkpoint-capacity"));
        }
        target[0] = u8::from(self.completed);
        target[1..9].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[9..17].copy_from_slice(&(self.scan_cursor as u64).to_le_bytes());
        Ok(17)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 17 || checkpoint[0] > 1 {
            return Err(Fault::from("flow-retained-direct-checkpoint-invalid"));
        }
        if self.closing || !self.retirement.is_empty() || self.preview_off.is_some() || self.preview_next.is_some() || self.edge_mutations.is_some() || self.node_mutations.is_some() || self.artifact_mutations.is_some() {
            return Err(Fault::from("flow-retained-direct-restore-requires-empty-owner"));
        }
        self.completed = checkpoint[0] == 1;
        self.replay_target = usize::try_from(u64::from_le_bytes(checkpoint[1..9].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.replay_scan_target = usize::try_from(u64::from_le_bytes(checkpoint[9..17].try_into().map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?)).map_err(|_| Fault::from("flow-retained-direct-checkpoint-invalid"))?;
        self.cursor = 0;
        self.scan_cursor = 0;
        self.preview_off = None;
        self.preview_next = None;
        self.preview_found = false;
        self.edge_mutations = None;
        self.node_mutations = None;
        self.artifact_mutations = None;
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
        if let Some(values) = self.preview_off.take().or_else(|| self.preview_next.take()) {
            self.retirement.push(retained::Owner::Strings(values));
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(values) = self.edge_mutations.take().or_else(|| self.node_mutations.take()).or_else(|| self.artifact_mutations.take()) {
            self.retirement.push(retained::Owner::Mutations(values));
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.retirement.is_empty() && self.preview_off.is_none() && self.preview_next.is_none() && self.edge_mutations.is_none() && self.node_mutations.is_none() && self.artifact_mutations.is_none()
    }
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
        semio_framework::ToolExecutionContract::resumable(FLOW_DIRECT_STORE_RAW_BYTES, 256, FLOW_STORE_MAX_MUTATION_ITEMS, 16_384, 7_500, 1, 1)
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
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnect", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchFlowWidgets", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLodMode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setProximityDistance", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridVisible", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridSnapEnabled", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setGridFactor", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setPreviewOff", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCatalogueSections", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleExtension", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️DirectStoreLaneRoutes

//#region 🧵️HostOnlyRetainedRoutes
const FLOW_HOST_ONLY_TOOL_IDS: &[&str] = &["evaluate", "contextMenuAt", "openSpotlight", "replaceImage", "flowEvalTick", "flowEvalResolve"];
const FLOW_HOST_ONLY_RAW_BYTES: usize = 16_384;

struct FlowHostEffectPayload {
    command: FlowCommand,
    snapshot: std::sync::Arc<FlowSnapshot>,
    config: std::sync::Arc<FlowConfig>,
    history: std::sync::Arc<semio_framework_plugin::HistoryView>,
    children: std::sync::Arc<semio_framework_plugin::ChildContentView>,
    instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    completion: semio_framework_plugin::ArtifactToolCompletion<semio_framework_plugin::EditorApp<FlowPlayApp>>,
}

fn flow_scalar_command_view(command: &FlowCommand) -> Result<store::os_pack::ScalarRecordView<'_>, &'static str> {
    use store::os_pack::{ScalarRecordField as Field, ScalarRecordView};
    let ordinal = FlowCommand::TOOL_JOB_IDS.iter().position(|id| *id == command.command_id()).ok_or("Flow scalar command ordinal missing")? as u64;
    let fields = match command {
        FlowCommand::Evaluate(_) | FlowCommand::OpenSpotlight(_) | FlowCommand::FlowEvalTick(_) => [None, None, None],
        FlowCommand::ContextMenuAt(command) => [Some(Field::Text(&command.id)), None, None],
        FlowCommand::ReplaceImage(command) => [Some(Field::Text(&command.id)), None, None],
        FlowCommand::FlowEvalResolve(command) => [Some(Field::U64(command.node_hash)), Some(Field::Text(&command.output_json)), None],
        _ => return Err("Flow command does not have an admitted scalar record witness"),
    };
    Ok(ScalarRecordView { ordinal, fields })
}

fn flow_host_wire_view(payload: &FlowHostEffectPayload) -> Result<store::os_pack::ScalarRecordView<'_>, &'static str> { flow_scalar_command_view(&payload.command) }

#[cfg(test)]
#[path = "🧵️retained/🔎️wire/🧪️component.rs"]
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
                Ok(store::os_pack::ScalarRecordWireStep::Progress { .. }) => {},
                Ok(store::os_pack::ScalarRecordWireStep::Complete) => self.validated = true,
                Err(_) => return Self::fault(),
            }
            context.consume_fuel(1);
            return semio_framework_job::StepOutcome::Yield;
        }
        if !self.completed {
            let Some(payload) = self.payload.as_ref() else { return Self::fault() };
            let view = semio_framework_plugin::resolve_ready(ArtifactView::with_children(&payload.snapshot, &payload.history, (*payload.children).clone()));
            let emit = payload.instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| match &payload.command {
                    FlowCommand::Evaluate(_) => Ok(semio_framework_plugin::resolve_ready(evaluate::evaluate_result(&payload.snapshot, &payload.config, session))),
                    FlowCommand::FlowEvalTick(command) => semio_framework_plugin::resolve_ready(flow_eval_tick::handle(command, &view, &ConfigView { snapshot: &payload.config }, session)),
                    FlowCommand::FlowEvalResolve(command) => semio_framework_plugin::resolve_ready(flow_eval_resolve::handle(command, &view, &ConfigView { snapshot: &payload.config }, session)),
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

struct FlowDirectStoreJobFactoryProofs;

impl FlowDirectStoreJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.flow.flow@1/*#editor",
        document_schema: "flow.fixture",
        factory: "FlowDirectStoreJobFactory",
        factory_type: FlowDirectStoreJobFactory,
        tools: {
            "removeWidget" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "deleteSelection" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "disconnect" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "moveMediaNode" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "patchFlowWidgets" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "nodeGraphViewport" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setLodMode" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setProximityDistance" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setGridVisible" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setGridSnapEnabled" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setGridFactor" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setPreviewOff" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setCatalogueSections" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "toggleExtension" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
            "setLocale" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 256, 16_384, 7_500, 1, 1),
        }
    }
}

struct FlowHostEffectJobFactoryProofs;

impl FlowHostEffectJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.flow.flow@1/*#editor",
        document_schema: "flow.fixture",
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
    type Snapshot = FlowSnapshot;
    type Mutation = FlowMutation;
    type Config = FlowConfig;
    type ConfigMutation = FlowConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = FlowPresence;
    type PresenceMutation = FlowPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = FlowCommand;

    const DIALECT: Dialect = crate::artifacts::flow::FLOW_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(retained::artifact::preparation::PreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(retained::config::PreparationFactory))
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = FlowDirectStoreJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(FlowHostEffectJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(FlowHostEffectJobFactory::new(&controller))?;
        registry.register(FlowDirectStoreJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !FLOW_HOST_ONLY_TOOL_IDS.contains(&request.tool_id.as_str()) && !FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.retained.tool-mismatch"), "Flow command does not match its exact retained tool registration"));
        }
        if FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            let tool_id = request.command.command_id();
            let work: Box<dyn ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = Box::new(FlowDirectStoreWork::new(tool_id));
            let operation_context = semio_framework_plugin::AppOperationContext {
                app_instance_id: request.app_instance_id,
                parent_document_id: request.parent_document_id,
                operation_id: request.operation.operation.0,
                generation: request.operation.generation.0,
                canonical_base_revision: request.canonical_base_revision,
            };
            let payload = ArtifactRetainedCommandPayload::try_new_with_context(
                *request.command,
                request.snapshot,
                request.config,
                request.history,
                request.interaction_state,
                request.interaction_hover,
                request.context,
                operation_context,
                request.completion,
                FlowCommand::command_id,
                FLOW_DIRECT_STORE_RAW_BYTES,
                FLOW_STORE_MAX_MUTATION_ITEMS,
                work,
            )?;
            return Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)));
        }
        let payload = FlowHostEffectPayload {
            command: *request.command,
            snapshot: request.snapshot,
            config: request.config,
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

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::flow::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> FlowSnapshot {
        FlowSnapshot::default()
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale`/`flowEvalTick`/`flowEvalResolve` have no
    /// manifest declaration (host-pushed/internally-chained, not user-facing actions).
    fn command_id(command: &FlowCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `deleteSelection`/`focusSelection`/
    /// `nodeGraphEdit`/`spotlightCommit` read the "graph" interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg, session)`
    /// signature is framework-fixed and has no `interaction` slot) — mirrors `space`'s equivalent routing.
    fn handle(
        command: &FlowCommand,
        doc: &ArtifactView<'_, FlowSnapshot>,
        cfg: &ConfigView<'_, FlowConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<FlowMutation, FlowConfigMutation, Self::DraftMutation>, Fault> {
        if FLOW_HOST_ONLY_TOOL_IDS.contains(&command.command_id()) || FLOW_DIRECT_STORE_TOOL_IDS.contains(&command.command_id()) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.retained.legacy-dispatch"), "Flow retained routes execute only through their exact app-owned job factory"));
        }
        let mut session = FlowEvalSession::new();
        match command {
            FlowCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, &mut session, interaction),
            FlowCommand::FocusSelection(payload) => focus_selection::apply(payload, doc, cfg, &mut session, interaction),
            FlowCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, &mut session, interaction),
            FlowCommand::SpotlightCommit(payload) => spotlight_commit::apply(payload, doc, cfg, &mut session, interaction),
            _ => command.dispatch(doc, cfg, &mut session),
        }
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
    fn interaction_topology(doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>) -> InteractionTopology {
        let live = doc.snapshot.to_fixture();
        let mut ordered: Vec<TopologyNode> = live.widgets.iter().map(|widget| TopologyNode { id: flow_graph_node_target_id(crate::artifacts::flow::schema::widget_id(widget)), granularity: "node".into(), parent: None }).collect();
        ordered.extend(live.synapses.iter().map(|synapse| TopologyNode { id: flow_graph_edge_target_id(&synapse.id), granularity: "edge".into(), parent: None }));
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(FLOW_INTERACTION_GRAPH.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes — covers
    /// every mutation path (edits, undo/redo, example load, remote operations) in one place. Pure:
    /// recomputes the probe fresh from the fixture and the driver's persisted baseline each call.
    fn pending_effects(doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>) -> Vec<Effect> {
        evaluate::evaluate_result(doc.snapshot, cfg.snapshot, &mut FlowEvalSession::new()).effects
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        let labels = flow_play_labels(config);
        let mut session = FlowEvalSession::new();
        match body_key {
            FLOW_PLAY_BODY_MAIN => main::render(fixture, config, &mut session),
            FLOW_PLAY_BODY_COMPILED => compiled::render(fixture, config, &mut session),
            FLOW_PLAY_BODY_GENERATIONS => generations::render(config, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::Native),
            FLOW_PLAY_BODY_GENERATE_FORM => form::render(fixture, config),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(config),
            FLOW_PLAY_BODY_DOCUMENT => document_panel::render(fixture, labels),
            FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(fixture, config, &mut session, labels),
            FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn render_with_instance_operation_owner(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, FlowSnapshot>,
        cfg: &ConfigView<'_, FlowConfig>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        owner
            .with_mut::<FlowInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| match body_key {
                    FLOW_PLAY_BODY_MAIN => main::render(doc.snapshot, cfg.snapshot, session),
                    FLOW_PLAY_BODY_COMPILED => compiled::render(doc.snapshot, cfg.snapshot, session),
                    FLOW_PLAY_BODY_GENERATIONS => generations::render(cfg.snapshot, semio_framework_plugin::locale_from_str(&cfg.snapshot.locale), semio_framework_plugin::Terminology::Native),
                    FLOW_PLAY_BODY_GENERATE_FORM => form::render(doc.snapshot, cfg.snapshot),
                    FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(cfg.snapshot),
                    FLOW_PLAY_BODY_DOCUMENT => document_panel::render(doc.snapshot, flow_play_labels(cfg.snapshot)),
                    FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(doc.snapshot, cfg.snapshot, session, flow_play_labels(cfg.snapshot)),
                    FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(flow_play_labels(cfg.snapshot)),
                    _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
                })
            })
            .map_err(|error| semio_framework_plugin::PluginAssemblyError::new("flow.eval-session-owner", error.message))?
    }

    fn window_measures(_doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        HashMap::from([(main::FLOW_PLAY_WINDOW_MAIN.to_string(), main::window_measures(config, flow_play_labels(config)))])
    }

    fn context_menu(request: &ContextMenuRequest, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let config = cfg.snapshot;
        let is_de = config.locale.starts_with("de");
        flow_context_menu_items(registry, doc.snapshot, config, flow_play_labels(config), is_de, request.surface.as_ref())
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
pub fn apply_canvas_options(host: &mut FlowHost, config: &FlowConfig) {
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
pub fn host_from_snapshot(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession) -> FlowHost {
    let mut host = flow_host_with_session(&fixture.to_fixture(), session);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    host
}

/// ✏️ Runs a stateful `FlowHost` mutation and diffs the result back into granular `FlowMutation`s —
/// returns an empty vec when `mutate` reports "nothing changed".
pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut FlowHost) -> bool) -> Vec<FlowMutation> {
    let mut host = host_from_snapshot(snapshot, config, session);
    if !mutate(&mut host) {
        return Vec::new();
    }
    flow_fixture_operations(&snapshot.to_fixture(), &host.fixture).into_iter().filter_map(crate::artifacts::flow::schema::mutations::from_framework_mutation).collect()
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
    let json = serde_json::json!({ "nodes": nodes, "edges": edges, "handles": handles });
    host.dag.set_selection_domains_json(&json.to_string());
}

/// 🔍️ The camera that frames the given node selection (the "graph" domain's live selection, read by
/// the caller via `InteractionView` — ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), or
/// `None` when nothing is selected.
pub fn focus_selection_camera(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession, selected_node_ids: &[String]) -> Option<CameraJson> {
    if selected_node_ids.is_empty() {
        return None;
    }
    let mut host = host_from_snapshot(fixture, config, session);
    host.dag.set_viewport(1280, 800, 1.0);
    host.dag.set_selection(selected_node_ids);
    host.focus_selection_camera(1.2)
}
//#endregion 🔖️Selection

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_flow_app() -> AppDefinition {
    Editor::builder(crate::artifacts::flow::FLOW_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) })
        .document(["semio", "flow"])
        .artifact_kind(crate::artifacts::flow::artifact_kind())
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
        // 🌉️ COMPOSITE — plans create-widget then connect-widgets (ticket 26/08/16/…-COMPOSITE-MUTATIONS).
        .mutation("duplicateWidget", LocalizedLabel::native("Duplicate Widget", "Widget duplizieren"))
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(duplicate_widget::DUPLICATE_WIDGET_STEP_ACTION_ID, LocalizedLabel::native("Continue Duplicating Widget", "Widgetduplizierung fortsetzen"), ActionKind::Mutation) })
        // 🗂️ Referenced by flow_context_menu_items — categorized for grouped-context-menu disclosure.
        .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
        .mutation("disconnect", LocalizedLabel::native("Disconnect", "Trennen"))
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Anschlüsse verbinden"))
        .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
        .action_with(ActionDefinition::bounded_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
        .mutation("patchFlowWidgets", LocalizedLabel::native("Patch Widgets", "Widgets aktualisieren"))
        .mutation("renameFlowWidget", LocalizedLabel::native("Rename Widget", "Widget umbenennen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .mutation("spotlightCommit", LocalizedLabel::native("Spotlight Commit", "Spotlight bestätigen"))
        // 🧩️ Dynamic extension-provided action — id resolved at runtime, kept out of the palette.
        .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("runExtensionAction", LocalizedLabel::native("Run Extension Action", "Erweiterungsaktion ausführen"), ActionKind::Mutation) })
        // 👁️ Ephemeral view/config actions — mutate config, emit no document operations. Selection/
        // hover verbs (`setSelection`/`clearSelection`/`selectAll`/`selectNode`/`nodeGraphSelect`/
        // `nodeGraphHover`/`graphPointerDown`) are no longer declared here: framework-owned, injected
        // via `.interaction(...)` below (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
        .view_action("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"))
        .action_with(ActionDefinition::bounded_catalog("focusSelection", LocalizedLabel::native("Zoom to Selection", "Auf Auswahl zoomen"), ActionKind::View).with_category("view"))
        .action_with(flow_internal_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), ActionKind::View))
        .action_with(flow_internal_action("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View))
        .action_with(flow_internal_action("setProximityDistance", LocalizedLabel::native("Set Proximity Distance", "Näheabstand festlegen"), ActionKind::View))
        .action_with(flow_internal_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster sichtbar"), ActionKind::View))
        .action_with(flow_internal_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View))
        .action_with(flow_internal_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View))
        .action_with(flow_internal_action("contextMenuAt", LocalizedLabel::native("Context Menu At", "Kontextmenü an Position"), ActionKind::View))
        .action_with(flow_internal_action("setPreviewOff", LocalizedLabel::native("Set Preview Off", "Vorschau deaktivieren"), ActionKind::View).with_category("view"))
        .action_with(flow_internal_action("openSpotlight", LocalizedLabel::native("Open Spotlight", "Spotlight öffnen"), ActionKind::View).with_category("create"))
        .action_with(flow_internal_action("replaceImage", LocalizedLabel::native("Replace Image", "Bild ersetzen"), ActionKind::View).with_category("actions"))
        .action_with(flow_internal_action("setCatalogueSections", LocalizedLabel::native("Set Catalogue Sections", "Katalogabschnitte festlegen"), ActionKind::View))
        .action_with(flow_internal_action("toggleAutomation", LocalizedLabel::native("Toggle Extension", "Erweiterung umschalten"), ActionKind::View))
        // 📝️ Staged argument form for the panel-visible create action (module operators stay catalogue-driven).
        .action_args("addWidget", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
            ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
            ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
        ])
        .default_value("inputSlider")])
        .action_interactive_job("addWidget", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("removeWidget", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("duplicateWidget", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("deleteSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("disconnect", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("connectMediaPorts", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("moveMediaNode", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("reorganize", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("patchFlowWidgets", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("renameFlowWidget", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("nodeGraphEdit", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("spotlightCommit", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("runExtensionAction", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("setContributions", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("evaluate", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("focusSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
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
        .action_interactive_job("addGeneration", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("removeGeneration", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("selectGeneration", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("renameGeneration", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("updateGenerationValues", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
        .action_interactive_job("setLocale", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalTick", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalResolve", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("duplicateWidgetStep", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
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
        // membership that the real fixture model does not have — both hover and selection stay
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
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::meta;
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    use semio_s_plugin_stdio::artifacts::semio::{create_semio_member, SemioMembers};
    use store::ArtifactPack;

    pub type FlowApp = VcsArtifactApp<EditorApp<FlowPlayApp>, SemioMembers>;

    /// 🧪️ Testkit gap wrapper (`📓️w2-p5-flow-notes.md`): `new_app_with_registry` still takes
    /// `fn() -> App`, but `create_flow_app` now returns `AppDefinition` (contract §2.4) — mirrors the
    /// cad pilot's identical wrapper.
    async fn flow_manifest_for_testkit() -> App {
        App { definition: create_flow_app(), examples: Vec::new() }
    }

    /// 🧪️ Installs a hand-authored `flow.extension` manifest fixture (a "math" module contributing the
    /// `math.add` operator) so tests exercising the catalogue/extension surfaces have something real
    /// installed — deliberately NOT the production `flow-extension-*` crates: flow-core must not
    /// dev-depend on its own extensions (audit finding C1, see ticket
    /// `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`'s `w3-flow.md`). Each real extension crate already
    /// exhaustively tests its own manifest/operator content in its own `#[cfg(test)] mod tests` (e.g.
    /// `flow-extension-math`'s `manifest_lists_math_operators_and_schemas`); this fixture only covers
    /// what flow-core's own tests assert on (`catalogue_lists_module_operators`).
    async fn install_first_party_light_flow_extensions_for_tests() {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            let manifest = flow::FlowExtensionManifest {
                schema: "flow.extension".into(),
                id: "math".into(),
                name: "Math".into(),
                version: "0.0.0-test-fixture".into(),
                activation_events: vec!["onStartup".into()],
                contributes: flow::FlowExtensionContributes {
                    schemas: vec![],
                    operators: vec![flow::neural::OperatorInfo { id: "math.add".into(), extension: "math".into(), name: "Add".into(), abbreviation: "Add".into(), ..Default::default() }],
                    widgets: vec![],
                    commands: vec![],
                    settings: vec![],
                },
            };
            let manifest_json = serde_json::to_string(&manifest).expect("serialize test fixture manifest");
            flow::install_flow_extension_manifest("flow-core-test-fixture", &manifest_json).expect("fixture extension admission");
        });
    }

    pub(crate) async fn register_content_child(app: &mut FlowApp) {
        let snapshot = app.snapshot().await.expect("Flow parent snapshot");
        let fixture = snapshot.to_fixture();
        let content = crate::artifacts::flow::flow_content_snapshot_from_working(&fixture.widgets, &fixture.synapses, &fixture.layout).await;
        let dialect = snapshot.content.target.dialect.clone();
        let member = create_semio_member(&snapshot.content.child_id, &dialect, &content.encode_pack()).await.expect("Flow child member");
        app.register_child("content", snapshot.content.child_id, dialect, member).await.expect("register Flow content child");
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn flow_app() -> FlowApp {
        install_first_party_light_flow_extensions_for_tests();
        let mut app = VcsArtifactApp::<EditorApp<FlowPlayApp>, SemioMembers>::new(EditorApp::default()).await;
        register_content_child(&mut app).await;
        app
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn flow_app_with_registry() -> FlowApp {
        install_first_party_light_flow_extensions_for_tests();
        let definition = flow_manifest_for_testkit().await.definition;
        let registry = AppActionRegistry::from_definition(&definition).await;
        let mut app = VcsArtifactApp::<EditorApp<FlowPlayApp>, SemioMembers>::with_registry(EditorApp::default(), registry).await;
        register_content_child(&mut app).await;
        app
    }

    pub async fn dispatch(app: &mut FlowApp, command: FlowCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn dispatch_with_registry(app: &mut FlowApp, command: FlowCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut FlowApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub async fn main_window_measures(app: &mut FlowApp) -> Vec<WindowMeasure> {
        app.window_measures().get(main::FLOW_PLAY_WINDOW_MAIN).cloned().expect("main window measures")
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is the framework's injected
    /// `interactionSelect` verb now, dispatched against the "graph" domain declared on this app —
    /// requires `flow_app_with_registry()` (a bare `flow_app()` has no declared interaction domains to
    /// select against). `node_ids`/`edge_ids` are raw widget/synapse ids, converted to the row-id-
    /// prefixed `InteractionTarget` ids the document panel tree/`interaction_topology` both use (see
    /// `flow_graph_node_target_id`/`flow_graph_edge_target_id`).
    pub async fn select_graph(app: &mut FlowApp, node_ids: &[&str], edge_ids: &[&str]) {
        let mut targets: Vec<serde_json::Value> = node_ids.iter().map(|id| serde_json::json!({ "granularity": "node", "id": flow_graph_node_target_id(id) })).collect();
        targets.extend(edge_ids.iter().map(|id| serde_json::json!({ "granularity": "edge", "id": flow_graph_edge_target_id(id) })));
        let targets_json = serde_json::to_string(&targets).expect("targets json");
        app.handle_action("interactionSelect", Some(&serde_json::json!({ "domainId": FLOW_INTERACTION_GRAPH, "targets": targets_json, "merge": "replace" })), &meta("test")).expect("interactionSelect");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{dispatch, flow_app, flow_app_with_registry, FlowApp};
    use semio_framework_plugin::testkit::{assert_undo_redo_round_trip, meta};
    use semio_framework_plugin::PluginApp;

    /// ↩️ Nonadjacent severed edges regain their exact original indices and large authored content.
    #[test]
    fn delete_cascade_inverse_restores_exact_edge_order_and_label() {
        let fixture: Value = serde_json::from_str(include_str!("🧪️fixtures/↩️delete-cascade.json")).unwrap();
        let mut scene = fixture["scene"].clone();
        let label = fixture["label"]["unit"].as_str().unwrap().repeat(fixture["label"]["repetitions"].as_u64().unwrap() as usize);
        assert_eq!(label.len(), fixture["label"]["expectedBytes"].as_u64().unwrap() as usize);
        scene["widgets"][1]["label"] = Value::String(label);
        let (widgets, synapses, layout) = crate::artifacts::flow::schema::mutations::decode_flow_scene_json(&scene.to_string()).unwrap();
        let base = FlowSnapshot { content: crate::artifacts::flow::flow_content_child_handle(&widgets, &synapses, &layout), ..FlowSnapshot::default() };
        let mutation = FlowMutation::DeleteWidget(DeleteWidget { id: fixture["targetId"].as_str().unwrap().into() });
        let ordinary_inverse = crate::artifacts::flow::schema::mutations::inverse_flow_mutation(&base, &mutation);
        let (post, prepared_inverse, _) = prepare_flow_artifact(&base, mutation).unwrap();
        let forward = crate::artifacts::flow::flow_working_scene(&post);
        assert_eq!(serde_json::to_value(forward.synapses.iter().map(|edge| &edge.id).collect::<Vec<_>>()).unwrap(), fixture["expectedForwardSynapses"]);
        for inverses in [ordinary_inverse, prepared_inverse] {
            let indices: Vec<_> = inverses.iter().filter_map(|inverse| match inverse { FlowMutation::ConnectWidgets(value) => Some(value.index), _ => None }).collect();
            assert_eq!(serde_json::to_value(indices).unwrap(), fixture["expectedInverseIndices"]);
            let mut restored = post.clone();
            for inverse in inverses { crate::artifacts::flow::schema::mutations::apply_flow_mutation(&mut restored, &inverse).unwrap(); }
            assert_eq!(crate::artifacts::flow::schema::mutations::encode_flow_projection_json(&restored), crate::artifacts::flow::schema::mutations::encode_flow_projection_json(&base));
        }
    }

    /// 🗂️ Serde is the independent Rust JSON oracle for the language-agnostic Flow/Note route census.
    #[test]
    fn action_cohort_fixtures_match_the_exact_route_census() {
        let flow: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️action-cohort/🔣️component.json"))).expect("Flow action-cohort fixture must be valid JSON");
        let note: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗒️note/🧪️action-cohort/🔣️component.json"))).expect("Note action-cohort fixture must be valid JSON");
        for (fixture, owner, total, framework_owned) in [(&flow, "FlowPlayApp", 37_u64, 0_usize), (&note, "NotePlayApp", 36_u64, 0_usize)] {
            assert_eq!(fixture["owner"], owner);
            assert_eq!(fixture["routeCount"].as_u64(), Some(total));
            assert!(fixture["retainedRoutes"].as_array().is_some_and(Vec::is_empty));
            assert_eq!(fixture["frameworkOwnedRoutes"].as_array().map(Vec::len), Some(framework_owned));
            let routes: Vec<&str> = fixture["groups"].as_array().expect("groups").iter().flat_map(|group| group["routes"].as_array().expect("routes")).map(|route| route.as_str().expect("route id")).collect();
            let mut unique = routes.clone();
            unique.sort_unstable();
            unique.dedup();
            assert_eq!(routes.len() + framework_owned, total as usize);
            assert_eq!(unique.len(), routes.len());
        }
    }

    async fn context_menu_items(app: &mut FlowApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface, window_instance_id: None, point: None };
        serde_json::to_value(app.context_menu(&request)).unwrap_or(Value::Null)
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 37, "every FlowCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the one documented divergence (`setLocale` → `locale`, an
    /// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = if id == "setLocale" { "locale".to_string() } else { id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect() };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ The two rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
    /// bytes captured from the pre-merge `flow_protocol` crate. A regression here is a real format break,
    /// not a test-fixture mismatch.
    #[semio_framework_async_macros::async_test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(FlowCommand, &str, &str); 3] = [
            (FlowCommand::AddWidget(add_widget::AddWidget { kind: "neuron".into(), neuron_kind: Some("math.add".into()), x: None, y: None }), "add-widget kind=neuron neuron-kind=math.add", "010002086d6174682e616464066e6575726f6e02000601010600"),
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `SetGridVisible`'s binary
            // ordinal shifted 24 (0x18) → 18 (0x12) — seven rows ahead of it in `FlowCommand`
            // (`setSelection`/`clearSelection`/`selectAll`/`selectNode`/`nodeGraphSelect`/
            // `nodeGraphHover`/`graphPointerDown`) were deleted (framework-injected now), a real,
            // documented wire-format break (row order IS the ordinal — deleting from the middle is not
            // the safe "append only" case the row-order doc comment calls out).
            (FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: None }), "set-grid-visible", "01120000"),
            (FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(true) }), "set-grid-visible pressed=true", "011200010002"),
        ];
        for (command, text, hex) in cases {
            let encoded = protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
            assert_eq!(protocol::OpText::print_op(&command), text, "text for {command:?}");
            assert_eq!(encoded, hex, "hex for {command:?}");
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<FlowCommand> {
        use flow::CameraJson;
        vec![
            FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None }),
            FlowCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "n1".into() }),
            FlowCommand::DuplicateWidget(duplicate_widget::DuplicateWidget { widget_id: "n1".into() }),
            FlowCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            FlowCommand::Disconnect(disconnect::Disconnect { synapse_id: "s1".into() }),
            FlowCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
            FlowCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
            FlowCommand::Reorganize(reorganize::Reorganize {}),
            FlowCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["n1".into(), "n2".into()], field: "value".into(), value: "5".into() }),
            FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: "n1".into(), value: "renamed".into() }),
            FlowCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations: vec![
                    node_graph_edit::FlowNodeGraphEditOp::SetSnapshot { snapshot_json: "{}".into() },
                    node_graph_edit::FlowNodeGraphEditOp::DeleteSelection,
                    node_graph_edit::FlowNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
                ],
            }),
            FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit { operations: vec![spotlight_commit::FlowNodeGraphEditOp::DeleteSelection] }),
            FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }),
            FlowCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
            FlowCommand::Evaluate(evaluate::Evaluate {}),
            FlowCommand::FocusSelection(focus_selection::FocusSelection {}),
            FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 } }),
            FlowCommand::SetLodMode(set_lod_mode::SetLodMode { value: "micro".into() }),
            FlowCommand::SetProximityDistance(set_proximity_distance::SetProximityDistance { value: 48.0 }),
            FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(true) }),
            FlowCommand::SetGridSnapEnabled(set_grid_snap_enabled::SetGridSnapEnabled { pressed: None }),
            FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 10.0 }),
            FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: "n1".into() }),
            FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["n1".into()], value: true }),
            FlowCommand::OpenSpotlight(open_spotlight::OpenSpotlight {}),
            FlowCommand::ReplaceImage(replace_image::ReplaceImage { id: "n1".into() }),
            FlowCommand::SetCatalogueSections(set_catalogue_sections::SetCatalogueSections { sections_json: "[]".into() }),
            FlowCommand::ToggleExtension(toggle_extension::ToggleExtension { id: "auto-layout".into(), enabled: true }),
            FlowCommand::AddGeneration(add_generation::AddGeneration {}),
            FlowCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "g1".into() }),
            FlowCommand::SelectGeneration(select_generation::SelectGeneration { id: "g1".into() }),
            FlowCommand::RenameGeneration(rename_generation::RenameGeneration { id: "g1".into(), name: "Copy".into() }),
            FlowCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::Number(5.0) }),
            FlowCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            FlowCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
            FlowCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() }),
            FlowCommand::DuplicateWidgetStep(duplicate_widget_step::DuplicateWidgetStep { generation: 7, phase: "widget".into(), scan_index: 64, suffix: 2, candidate_id: "n1-copy-2".into(), ..Default::default() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_flow_app()).expect("app definition json");
        for id in [main::FLOW_PLAY_WINDOW_MAIN, compiled::FLOW_PLAY_WINDOW_COMPILED, generations::FLOW_PLAY_WINDOW_GENERATIONS, form::FLOW_PLAY_WINDOW_GENERATE_FORM, preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for id in [edit::FLOW_PLAY_MODE_EDIT, generate::FLOW_PLAY_MODE_GENERATE, generate::FLOW_PLAY_LAYOUT_GENERATE] {
            assert!(json.contains(id), "mode/layout {id} missing from the manifest");
        }
        for body in [FLOW_PLAY_BODY_DOCUMENT, FLOW_PLAY_BODY_CATALOGUE, FLOW_PLAY_BODY_INSPECTOR] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("computation.flow"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Interaction
    /// 🕹️ The "graph" domain is declared `HierarchyProvider::Topology`, scoped to the main canvas window
    /// kind, non-transitive (see the `.interaction(...)` doc comment for why), with node/edge/handle
    /// granularities and all five merges.
    #[semio_framework_async_macros::async_test]
    async fn graph_interaction_domain_is_declared_topology_and_scoped_to_the_main_window() {
        let definition = create_flow_app();
        let graph = definition.interactions.iter().find(|interaction| interaction.id == FLOW_INTERACTION_GRAPH).expect("graph interaction domain declared");
        assert!(matches!(graph.hierarchy, HierarchyProvider::Topology));
        assert!(!graph.hover.transitive, "graph's outer widget list has no real group membership to walk transitively");
        assert!(!graph.selection.transitive);
        let granularity_ids: Vec<&str> = graph.granularities.iter().map(|granularity| granularity.id.as_str()).collect();
        assert_eq!(granularity_ids, ["node", "edge", "handle"]);
        let main_window = definition.window_kinds.iter().find(|window| window.id == main::FLOW_PLAY_WINDOW_MAIN).expect("main window kind declared");
        assert!(main_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == FLOW_INTERACTION_GRAPH), "main window must reference the graph interaction domain");
    }

    /// 🌳️ `interaction_topology` registers every widget/synapse as a root at its own granularity —
    /// the same row-id-prefixed targets the document panel tree renders (see
    /// `document_panel::render`'s doc comment).
    #[semio_framework_async_macros::async_test]
    async fn interaction_topology_registers_every_widget_and_synapse_as_a_root() {
        let document = FlowSnapshot::default();
        let config = FlowConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let topology = FlowPlayApp::interaction_topology(&doc, &cfg);
        let graph = topology.domains.get(FLOW_INTERACTION_GRAPH).expect("graph domain present in topology");
        let live = document.to_fixture();
        assert_eq!(graph.ordered.len(), live.widgets.len() + live.synapses.len());
        assert!(graph.ordered.iter().all(|node| node.parent.is_none()), "every node/edge is a root — no real group membership at this level");
        assert!(graph.ordered.iter().any(|node| node.id == flow_graph_node_target_id("slider") && node.granularity == "node"));
        assert!(graph.ordered.iter().any(|node| node.id == flow_graph_edge_target_id("s1") && node.granularity == "edge"));
    }
    //#endregion 🔖️Interaction

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn undo_restores_fixture_after_add_widget() {
        let mut app = flow_app();
        let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
        assert_undo_redo_round_trip(
            &mut app,
            FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) }),
            |app| app.snapshot().expect("snapshot").to_fixture().widgets.len(),
            before,
            before + 1,
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn generate_mode_renders_three_surfaces() {
        let mut app = flow_app();
        use crate::editor::flow::testkit::render;
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATIONS).contains("addGeneration"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).contains("Add a generation"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW).contains("text-editor"));
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::flow::testkit::render;
        let mut app = flow_app();
        assert!(render(&mut app, "flow.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn host_from_snapshot_deletes_edge_selected_by_synapse_domain() {
        let config = FlowConfig::default();
        let fixture = FlowSnapshot::default();
        let session = FlowEvalSession::new();
        let mut host = host_from_snapshot(&fixture, &config, &session);
        sync_host_selection_domains(&mut host, &[], &["s1".into()], &[]);
        assert!(host.has_selection(), "s1 must resolve through host_from_snapshot edge map");
        host.delete_selection().expect("deleteSelection");
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_on_disjoint_edits() {
        use crate::artifacts::flow::schema::widget_id;
        use semio_framework_plugin::testkit::paired_apps;
        let (mut instance_a, mut instance_b) = paired_apps::<EditorApp<FlowPlayApp>>("mem://flow-convergence");

        instance_a.dispatch_typed(FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: "input".into() }), &meta("actor-a")).expect("a renames slider");
        instance_b.dispatch_typed(FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(10.0), y: Some(10.0) }), &meta("actor-b")).expect("b adds a note");

        // A neutral history action always dispatches through the store, which pumps inbound operations first.
        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.snapshot().expect("snapshot a").to_fixture();
        let projection_b = instance_b.snapshot().expect("snapshot b").to_fixture();
        assert!(projection_a.widgets.iter().any(|widget| widget_id(widget) == "input"), "A keeps its rename");
        assert!(projection_a.widgets.iter().any(|widget| matches!(widget, Widget::InputNote { .. })), "A absorbs B's note");
        assert_eq!(projection_a.widgets.len(), projection_b.widgets.len(), "both instances converge to the same widget set");
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️ContextMenu
    #[semio_framework_async_macros::async_test]
    async fn context_menu_includes_select_all_when_empty() {
        let mut app = flow_app_with_registry();
        let menu = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "main".into(), kind: "nodeGraph".into(), hits: vec![], selection: vec![], text: None }));
        let menu_json = menu.to_string();
        assert!(menu_json.contains("selectAll"), "menu should be {menu_json}");
        assert!(menu_json.contains("Select All") || menu_json.contains("select-all"), "menu should be {menu_json}");
        assert!(menu_json.contains(r#""icon":"plus""#), "add-node icon: {menu_json}");
        assert!(!menu_json.contains(r#""id":"delete-selection""#), "empty canvas must omit delete: {menu_json}");
        assert!(!menu_json.contains("setPreviewOff"), "empty canvas must omit preview: {menu_json}");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: selection is framework-owned
    /// `InteractionState` now and `ArtifactApp::context_menu` is not threaded an `InteractionView` this
    /// wave (see `flow_context_menu_items`'s doc comment) — the request's own `surface.selection` groups
    /// are the only way to feed a selection into the menu, mirroring what the real click caller carries.
    async fn node_selection_surface(node_ids: &[&str]) -> semio_framework_plugin::ContextMenuSurfaceTarget {
        semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: node_ids.iter().map(|id| id.to_string()).collect() }],
            text: None,
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_includes_hide_preview_for_selection_and_set_preview_off_mutates_scene() {
        let mut app = flow_app_with_registry();
        let menu = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).to_string();
        assert!(menu.contains("setPreviewOff"), "menu should expose preview toggle: {menu}");
        assert!(menu.contains("Hide preview") || menu.contains("eye-off"), "menu should offer hide preview: {menu}");
        assert!(menu.contains("focusSelection"), "menu should expose zoom to selection: {menu}");
        assert!(menu.contains(r#""checked":true"#), "preview checked when visible: {menu}");
        dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: true }));
        let after_menu = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).to_string();
        assert!(after_menu.contains("Show preview") || after_menu.contains(r#""icon":"eye""#), "menu should offer show preview: {after_menu}");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `contextMenuAt` no longer sets
    /// selection (a genuine no-operation now, see `context_menu_at::apply`'s doc comment) — the request's
    /// own `surface.selection` groups carry the clicked target instead, mirroring what the real caller
    /// (right-clicking a node) supplies alongside the `contextMenuAt` dispatch.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_at_selects_target_and_enables_preview() {
        let mut app = flow_app_with_registry();
        let before = context_menu_items(&mut app, None).to_string();
        assert!(!before.contains(r#""id":"delete-selection""#), "preview starts without delete: {before}");
        dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: "slider".into() }));
        let after = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).to_string();
        assert!(after.contains("setPreviewOff"), "menu keeps preview: {after}");
        assert!(after.contains(r#""ids":["slider"]"#) || after.contains("slider"), "preview args target the clicked node: {after}");
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_annotates_mixed_selection_counts_and_omits_delete_without_selection() {
        let mut app = flow_app_with_registry();
        let empty = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "main".into(), kind: "nodeGraph".into(), hits: vec![], selection: vec![], text: None })).to_string();
        assert!(!empty.contains(r#""id":"delete-selection""#), "empty must omit delete: {empty}");

        let menu = context_menu_items(
            &mut app,
            Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: "main".into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
                selection: vec![
                    semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: (1..=8).map(|i| format!("n{i}")).collect() },
                    semio_framework_plugin::ContextMenuSelectionGroup { domain: "edge".into(), ids: (1..=13).map(|i| format!("e{i}")).collect() },
                ],
                text: None,
            }),
        )
        .to_string();
        assert!(menu.contains(r#""id":"delete-selection""#), "mixed selection must expose delete: {menu}");
        assert!(menu.contains("8 nodes and 13 edges"), "count phrase missing: {menu}");
        assert!(menu.contains("deleteSelection"), "delete action missing: {menu}");
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_for_edge_hit_uses_surface_edge_selection() {
        let mut app = flow_app_with_registry();
        let menu = context_menu_items(
            &mut app,
            Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: "main".into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "edge".into(), id: "syn-1".into(), label: None }],
                selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "edge".into(), ids: vec!["syn-1".into()] }],
                text: None,
            }),
        )
        .to_string();
        assert!(menu.contains(r#""id":"delete-selection""#), "edge selection must expose delete: {menu}");
        assert!(menu.contains("1 edge") || menu.contains("1 Kante"), "edge count phrase missing: {menu}");
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        let mut app = flow_app_with_registry();
        let request = ContextMenuRequest {
            menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
            surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: "main".into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
                selection: vec![
                    semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: (1..=8).map(|i| format!("n{i}")).collect() },
                    semio_framework_plugin::ContextMenuSelectionGroup { domain: "edge".into(), ids: (1..=13).map(|i| format!("e{i}")).collect() },
                ],
                text: None,
            }),
            window_instance_id: None,
            point: None,
        };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
        let last = menu.last().expect("grouped disclosure menu should not be empty");
        let last_is_destructive_leaf = last.id == "delete-selection" && last.destructive == Some(true) && last.action.as_deref() == Some("deleteSelection");
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).is_some_and(|child| child.destructive == Some(true));
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
    }
    //#endregion 🔖️ContextMenu
}
//#endregion 🧪️Tests
