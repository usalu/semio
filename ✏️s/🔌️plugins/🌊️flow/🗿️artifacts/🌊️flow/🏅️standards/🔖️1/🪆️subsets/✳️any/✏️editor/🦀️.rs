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
    open_spotlight, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode,
    set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension,
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
use crate::op::FlowMutation;
#[cfg(test)]
use crate::schema::mutations::connect_widgets::ConnectWidgets;
#[cfg(test)]
use crate::schema::mutations::create_widget::CreateWidget;
use crate::schema::mutations::delete_widget::DeleteWidget;
use crate::schema::mutations::disconnect_widgets::DisconnectWidgets;
use crate::schema::mutations::move_widgets::MoveWidgets;
#[cfg(test)]
use crate::schema::mutations::reorder_synapses::ReorderSynapses;
#[cfg(test)]
use crate::schema::mutations::reorder_widgets::ReorderWidgets;
use crate::schema::mutations::replace_widget::ReplaceWidget;
#[cfg(test)]
use crate::schema::mutations::update_synapse_endpoints::UpdateSynapseEndpoints;
use crate::{FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_artifact_flow_flow::{flow_fixture_operations, CameraJson, Widget};
use semio_framework_artifact_infinite_dag::DagDrawLod;
use semio_framework_plugin::app::{ChildEmit, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppActionRegistry, AppDefinition, ArtifactEditor, ArtifactView, CommandDefinition, ConfigView, ContextMenuItemSpec, ContextMenuRequest, Dialect, DomainTopology, DraftView, Editor,
    Effect, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
    TopologyNode, WindowMeasure,
};
use serde_json::json;
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{FlowEdge, PortRef, SemioFlowSnapshot};
#[cfg(test)]
use serde_json::Value;
use std::collections::HashMap;
#[cfg(test)]
use std::io::Write;
use std::sync::Arc;
use store::EngineHandles;

#[path = "🧵️retained/🦀️.rs"]
mod retained;

#[cfg(test)]
#[path = "🫧️transient/🧪️tests/🫧️transient/🦀️.rs"]
mod transient_retirement_tests;

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
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
/// 🖱️ On-demand flow node-graph context menu from surface hit-test and selection snapshot.
fn flow_context_menu_items(registry: &AppActionRegistry, fixture: &FlowSnapshot, config: &FlowMainWindowConfig, labels: &FlowPlayLabels, is_de: bool, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {
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
    let live = fixture.to_fixture();
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
    // groups; `delete-selection` stays a direct destructive item last — `organize_context_menu`
    // (applied automatically at the `VcsArtifactApp::context_menu` funnel) sorts the groups into
    // `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive separator itself.
    {
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
    authority: Option<Arc<store::ArtifactStoreOneItemLiveAuthority>>,
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

#[cfg(test)]
struct FlowBoundedByteCounter {
    written: usize,
    maximum_bytes: usize,
}

#[cfg(test)]
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

#[cfg(test)]
fn flow_bounded_serialized_bytes<T: dsl::ToValue>(value: &T, maximum_bytes: usize) -> Result<usize, String> {
    let mut counter = FlowBoundedByteCounter { written: 0, maximum_bytes };
    let json: Value = dsl::ToValue::to_value(value).into();
    serde_json::to_writer(&mut counter, &json).map_err(|error| error.to_string())?;
    Ok(counter.written)
}

#[cfg(test)]
fn flow_artifact_mutation_items(mutation: &FlowMutation) -> usize {
    match mutation {
        FlowMutation::MoveWidgets(payload) => payload.entries.len(),
        FlowMutation::DuplicateWidget(_) => 2,
        _ => 1,
    }
}

#[cfg(test)]
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

#[cfg(test)]
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
                inverses.push(FlowMutation::MoveWidgets(MoveWidgets { entries: vec![semio_framework_artifact_flow_flow::FlowLayoutEntry { id: payload.id.clone(), layout: Some(layout.clone()) }] }));
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
            scene.synapses.insert(
                payload.index.min(scene.synapses.len()),
                semio_framework_artifact_flow_flow::SynapseSpec { id: payload.id.clone(), from: payload.from.clone(), from_port: payload.from_port.clone(), to: payload.to.clone(), to_port: payload.to_port.clone() },
            );
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
                inverse_entries.push(semio_framework_artifact_flow_flow::FlowLayoutEntry { id: entry.id.clone(), layout: scene.layout.get(&entry.id).cloned() });
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
            let copy = crate::schema::widget_with_id(source, payload.new_id.clone());
            scene.widgets.push(copy);
            scene.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: payload.synapse_id.clone(), from: payload.source_id.clone(), from_port: payload.from_port.clone(), to: payload.new_id.clone(), to_port: payload.to_port.clone() });
            vec![FlowMutation::DisconnectWidgets(DisconnectWidgets { id: payload.synapse_id.clone() }), FlowMutation::DeleteWidget(DeleteWidget { id: payload.new_id.clone() })]
        }
    };
    let content = flow_content_child_handle_bounded(&scene.widgets, &scene.synapses, &scene.layout, FLOW_STORE_MAX_TEXT_BYTES)?;
    let post = FlowSnapshot { schema: base.schema.clone(), content };
    Ok((post, inverse, mutation))
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for FlowStoreOneItemPreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: Clone + dsl::ToValue + Send + Sync + 'static,
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
    M: Clone + dsl::ToValue + Send + Sync + 'static,
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
        let prepared = authority.prepare_one_item(edit, Arc::new(post))?;
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
        FlowCommand::NodeGraphViewport(payload) => next.camera = payload.camera.clone(),
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
    Ok(Emit { window_config_mutations: vec![main::config::addressed(view, next)?], ..Default::default() })
}

fn duplicate_widget_id(source: &str, suffix: u64) -> String {
    if suffix == 1 { format!("{source}-copy") } else { format!("{source}-copy-{suffix}") }
}

fn duplicate_edge_id(source: &str, target: &str) -> String {
    format!("{source}-to-{target}")
}

fn evaluate_generation_preview(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, values: &crate::playbook::PlaybookValues) -> String {
    let live = fixture.to_fixture();
    let fixture_json = dsl::json::to_json_string(&live);
    let values: dsl::json::Object = values.iter().map(|(key, value)| (key.clone(), dsl::json::from_dsl_value(value))).collect();
    let patched = flow::forms_bridge::apply_generation_values_to_fixture(&fixture_json, &values);
    let patched_fixture = match FlowHost::parse_fixture_json(&patched) {
        Ok(parsed) => {
            live.retire_cold();
            parsed
        }
        Err(_) => live,
    };
    let mut host = FlowHost::from_fixture(patched_fixture);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    host.evaluate().unwrap_or_default()
}

fn generation_window_transient(
    command: &FlowCommand,
    fixture: &FlowSnapshot,
    config: &FlowMainWindowConfig,
    current: &main::transient::FlowWindowTransient,
    view: &semio_framework_plugin::ViewModel,
) -> Result<Option<semio_framework_plugin::WindowTransientMutation>, Fault> {
    let (action, args) = match command {
        FlowCommand::AddGeneration(_) => ("addGeneration", None),
        FlowCommand::RemoveGeneration(payload) => ("removeGeneration", Some(dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone()))]))),
        FlowCommand::SelectGeneration(payload) => ("selectGeneration", Some(dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone()))]))),
        FlowCommand::RenameGeneration(payload) => (
            "renameGeneration",
            Some(dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone())), ("name".to_string(), dsl::DslValue::String(payload.name.clone()))])),
        ),
        FlowCommand::UpdateGenerationValues(payload) => (
            "updateGenerationValues",
            Some(dsl::DslValue::object([
                ("generationId".to_string(), payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null)),
                ("questionId".to_string(), dsl::DslValue::String(payload.question_id.clone())),
                ("value".to_string(), payload.value.clone()),
            ])),
        ),
        _ => return Ok(None),
    };
    let live = fixture.to_fixture();
    let spec = flow::forms_bridge::flow_fixture_to_form_spec(&live);
    live.retire_cold();
    let mut generation = current.generation();
    if !crate::playbook::handle_generation_action(action, args.as_ref(), &mut generation, &spec, FLOW_PLAY_APP_ID) {
        return Ok(None);
    }
    if matches!(command, FlowCommand::AddGeneration(_) | FlowCommand::SelectGeneration(_) | FlowCommand::UpdateGenerationValues(_)) {
        match crate::playbook::selected_generation(&generation) {
            Some(active) => generation.preview_text = Some(evaluate_generation_preview(fixture, config, &active.values)),
            None => generation.preview_text = None,
        }
    }
    let mut transient = current.clone();
    transient.generation_json = serde_json::to_string(&generation).map_err(|_| Fault::from("flow-generation-transient-encode"))?;
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
    edge_mutations: Option<Vec<FlowMutation>>,
    node_mutations: Option<Vec<FlowMutation>>,
    artifact_mutations: Option<Vec<FlowMutation>>,
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
            edge_mutations: None,
            node_mutations: None,
            artifact_mutations: None,
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
            FlowCommand::DuplicateWidget(payload) if !payload.widget_id.is_empty() && payload.widget_id.len() <= 256 => {
                let context = context?;
                let child_id = &snapshot.content.child_id;
                let dialect = context.children.dialect("content", child_id)?;
                if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" { return None; }
                let child = context.children.typed_read::<SemioFlowSnapshot>("content", child_id).ok()?;
                (child.nodes.len() <= FLOW_STORE_MAX_SCENE_ITEMS && child.edges.len() <= FLOW_STORE_MAX_SCENE_ITEMS)
                    .then_some(child.nodes.len().saturating_add(1).saturating_mul(child.nodes.len().saturating_add(child.edges.len()).saturating_add(2)).max(1))
            }
            FlowCommand::DuplicateWidget(_) => None,
            _ => Some(1),
        }
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
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
            let window = context.window_transient.as_ref().ok_or_else(|| Fault::from("flow-duplicate-window-transient-required"))?;
            if view.window_id.as_deref() != Some(window.window_id()) {
                return Err(Fault::from("flow-duplicate-window-transient-stale"));
            }
            let child_id = &snapshot.content.child_id;
            let dialect = context.children.dialect("content", child_id).ok_or_else(|| Fault::from("flow-duplicate-child-dialect-required"))?;
            if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" {
                return Err(Fault::from("flow-duplicate-child-dialect-mismatch"));
            }
            let child = context.children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
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
                let mut transient = main::transient::from_snapshot(Some(window));
                transient.duplicate_widget_progress_json.clear();
                return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                    emit: Emit::default(),
                    ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![main::transient::addressed(view, transient)?] },
                });
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
                child_emits: vec![ChildEmit::of::<SemioFlowSnapshot, _>("content", child_id, &[SemioFlowMutation::InsertNode(insert_node::InsertNode::new(node)), SemioFlowMutation::InsertEdge(insert_edge::InsertEdge::new(edge))])],
                coalesce_key: Some(format!("duplicateWidget:{}", payload.widget_id)),
                ..Default::default()
            };
            let mut transient = main::transient::from_snapshot(Some(window));
            transient.duplicate_widget_progress_json.clear();
            self.completed = true;
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: vec![main::transient::addressed(view, transient)?] },
            });
        }
        if let FlowCommand::FocusSelection(_) = command {
            let (nodes, _) = flow_graph_selection_domains(interaction.selection.get(FLOW_INTERACTION_GRAPH).map_or(&[][..], |selection| selection.ids.as_slice()));
            let mut next = config.clone();
            if let Some(camera) = focus_selection_camera(snapshot, &config, &FlowEvalSession::new(), &nodes) {
                next.camera = camera;
            }
            self.completed = true;
            return Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![main::config::addressed(view, next)?], ..Default::default() }));
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
            let mutation = generation_window_transient(command, snapshot, &config, &current, view)?;
            self.completed = true;
            let emit = Emit { coalesce_key: matches!(command, FlowCommand::UpdateGenerationValues(_)).then(|| "generation-values".to_string()), ..Default::default() };
            let ephemeral = semio_framework_plugin::EphemeralEmit { presence: Vec::new(), transient: Vec::new(), window_transient: mutation.into_iter().collect() };
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral });
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
                            let requested = semio_framework_artifact_flow_flow::WidgetLayout { x: payload.x, y: payload.y };
                            (scene.layout.get(&payload.node_id) != Some(&requested))
                                .then(|| FlowMutation::MoveWidgets(MoveWidgets { entries: vec![semio_framework_artifact_flow_flow::FlowLayoutEntry { id: payload.node_id.clone(), layout: Some(requested) }] }))
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
            let mut next = config.clone();
            next.preview_off_node_ids = node_ids;
            return Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: vec![main::config::addressed(view, next)?], ..Default::default() }));
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
        if self.closing || !self.retirement.is_empty() || self.preview_off.is_some() || self.preview_next.is_some() || self.edge_mutations.is_some() || self.node_mutations.is_some() || self.artifact_mutations.is_some() {
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
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnect", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveMediaNode", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchFlowWidgets", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
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
const FLOW_CHILD_GROUP_TOOL_IDS: &[&str] = &["addWidget"];
const FLOW_CHILD_GROUP_RAW_BYTES: usize = 16_384;

struct FlowChildGroupWork {
    instance_owner: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    completed: bool,
    closing: bool,
}

impl FlowChildGroupWork {
    fn new(instance_owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { instance_owner: Some(instance_owner), completed: false, closing: false }
    }

    fn admitted_child<'a>(
        command: &'a FlowCommand,
        snapshot: &'a FlowSnapshot,
        context: Option<&'a semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<(&'a add_widget::AddWidget, store::SnapshotReadRef<'a, semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot>)> {
        let FlowCommand::AddWidget(payload) = command else { return None };
        let context = context?;
        let child_id = &snapshot.content.child_id;
        let dialect = context.children.dialect("content", child_id)?;
        if dialect.artifact_kind != "s.stdio.semio" || dialect.standard != "v1" || dialect.subset != "flow" {
            return None;
        }
        let child = context.children.typed_read::<semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot>("content", child_id).ok()?;
        Some((payload, child))
    }
}

impl ArtifactCommandWork<semio_framework_plugin::EditorApp<FlowPlayApp>> for FlowChildGroupWork {
    fn tool_id(&self) -> &'static str {
        "addWidget"
    }

    fn extent(
        &self,
        command: &FlowCommand,
        snapshot: &FlowSnapshot,
        _interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if self.closing || self.completed || command.command_id() != "addWidget" {
            return None;
        }
        let (payload, child) = Self::admitted_child(command, snapshot, context)?;
        let text_bytes = payload.kind.len().checked_add(payload.neuron_kind.as_ref().map_or(0, String::len))?;
        let x = payload.x.unwrap_or(120.0);
        let y = payload.y.unwrap_or(120.0);
        (text_bytes <= FLOW_CHILD_GROUP_RAW_BYTES && x.is_finite() && y.is_finite() && child.nodes.len() <= FLOW_STORE_MAX_MUTATION_ITEMS && child.edges.len() <= FLOW_STORE_MAX_MUTATION_ITEMS).then_some(1)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, history, interaction: _interaction, hover: _hover, context, operation: _operation } = *input;
        if self.closing || self.completed {
            return Err(Fault::from("flow-retained-add-widget-terminal"));
        }
        let (payload, _) = Self::admitted_child(command, snapshot, context).ok_or_else(|| Fault::from("flow-retained-add-widget-child-authority"))?;
        let context = context.ok_or_else(|| Fault::from("flow-retained-add-widget-context"))?;
        let view = ArtifactView::with_children(snapshot, history, (*context.children).clone());
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-retained-add-widget-instance-owner"))?;
        let emit = instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| owner.with_session(|session| add_widget::handle(payload, &view, &ConfigView { snapshot: config, window: None }, session))?)?;
        let exact_child = emit.child_emits.first().filter(|child| child.slot == "content" && child.child_id == snapshot.content.child_id && child.ops.len() == 1 && child.labels.len() == 1);
        if exact_child.is_none()
            || emit.child_emits.len() != 1
            || !emit.artifact_mutations.is_empty()
            || !emit.config_mutations.is_empty()
            || !emit.draft_mutations.is_empty()
            || emit.description.is_some()
            || emit.coalesce_key.is_some()
            || !emit.effects.is_empty()
            || !emit.events.is_empty()
        {
            return Err(Fault::from("flow-retained-add-widget-output-contract"));
        }
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(emit))
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
            return Err((semio_framework::ToolJobFactoryError::new("Flow addWidget job rejects oversized wire or checkpoint owner"), input, checkpoint));
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
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] =
        &[semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] }];
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
        FlowCommand::Evaluate(_) | FlowCommand::OpenSpotlight(_) | FlowCommand::FlowEvalTick(_) => [None, None, None],
        FlowCommand::ContextMenuAt(command) => [Some(Field::Text(&command.id)), None, None],
        FlowCommand::ReplaceImage(command) => [Some(Field::Text(&command.id)), None, None],
        FlowCommand::FlowEvalResolve(command) => [Some(Field::U64(command.node_hash)), Some(Field::Text(&command.output_json)), None],
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
                    FlowCommand::Evaluate(_) => Ok(evaluate::evaluate_result(&payload.snapshot, &payload.config, session)),
                    FlowCommand::FlowEvalTick(_) => Ok(flow_eval_tick::tick_result(&payload.snapshot, &payload.config, session)),
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
/// 🕸️ The whole-graph routes: each runs one `FlowHost` operation (or the rename's pure fixture
/// rewrite) against the live scene and publishes ONE batched artifact-lane edit. Retained rather
/// than batch-dispatched, because a `BatchOnlyPendingRewrite` classification on these six is what
/// faulted the entire app at construction with `interactive-job.catalog-authority` on every host
/// that instantiates the flow editor (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const FLOW_GRAPH_OPERATION_TOOL_IDS: &[&str] = &["connectMediaPorts", "reorganize", "renameFlowWidget", "nodeGraphEdit", "spotlightCommit", "runExtensionAction"];
const FLOW_GRAPH_OPERATION_RAW_BYTES: usize = 16_384;
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
        FlowCommand::NodeGraphEdit(payload) => payload.operations.len() <= FLOW_STORE_MAX_MUTATION_ITEMS,
        FlowCommand::SpotlightCommit(payload) => payload.operations.len() <= FLOW_STORE_MAX_MUTATION_ITEMS,
        FlowCommand::RunExtensionAction(payload) => payload.action_id.len() <= FLOW_STORE_MAX_TEXT_BYTES,
        FlowCommand::ConnectMediaPorts(payload) => payload.source_port_id.len().saturating_add(payload.target_port_id.len()) <= FLOW_STORE_MAX_TEXT_BYTES,
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

    fn apply(&self, command: &FlowCommand, snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, interaction: &protocol::InteractionState) -> Result<Emit<FlowMutation, NoConfigMutation>, Fault> {
        let instance_owner = self.instance_owner.as_ref().ok_or_else(|| Fault::from("flow-retained-graph-instance-owner"))?;
        let (nodes, _edges) = flow_graph_selection_domains(interaction.selection.get(FLOW_INTERACTION_GRAPH).map_or(&[][..], |selection| selection.ids.as_slice()));
        let resolved = self.resolved;
        instance_owner.with_mut::<FlowInstanceOperationOwner, _>(|owner| {
            owner.with_session(|session| match command {
                FlowCommand::ConnectMediaPorts(payload) if resolved == [true, true] => Ok(Emit::mutations(connect_media_ports::connect_operations(payload, snapshot, config, session))),
                FlowCommand::ConnectMediaPorts(_) => Ok(Emit::default()),
                FlowCommand::Reorganize(_) => Ok(Emit::mutations(reorganize::reorganize_operations(snapshot, config, session))),
                FlowCommand::RenameFlowWidget(payload) if resolved[0] && !resolved[1] => Ok(Emit::mutations(rename_flow_widget::rename_operations(payload, snapshot))),
                FlowCommand::RenameFlowWidget(_) => Ok(Emit::default()),
                FlowCommand::NodeGraphEdit(payload) => Ok(node_graph_edit::node_graph_edit_result(snapshot, config, session, &payload.operations, &nodes)),
                FlowCommand::SpotlightCommit(payload) => Ok(spotlight_commit::node_graph_edit_result(snapshot, config, session, &payload.operations, &nodes)),
                FlowCommand::RunExtensionAction(payload) => Ok(run_extension_action::extension_action_result(payload, snapshot, config, session)),
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
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<FlowPlayApp>>>,
    ) -> Option<usize> {
        if self.closing || self.completed || command.command_id() != self.tool_id || !FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&self.tool_id) || !flow_graph_operation_payload_admitted(command) {
            return None;
        }
        let scene = snapshot.content.local_owner::<FlowWorkingScene>()?;
        if scene.widgets.len() > FLOW_STORE_MAX_SCENE_ITEMS || scene.synapses.len() > FLOW_STORE_MAX_SCENE_ITEMS {
            return None;
        }
        FLOW_GRAPH_OPERATION_CAPACITY.rows_for_items(scene.widgets.len().saturating_add(1))
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<FlowPlayApp>>) -> Result<ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<FlowPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
        if self.completed || self.closing {
            return Err(Fault::from("flow-retained-graph-work-terminal"));
        }
        if command.command_id() != self.tool_id || !flow_graph_operation_payload_admitted(command) {
            return Err(Fault::from("flow-retained-graph-payload-capacity"));
        }
        let context = context.ok_or_else(|| Fault::from("flow-retained-graph-context-required"))?;
        let config = main::config::from_snapshot(context.window_config.as_ref());
        let scene = snapshot.content.local_owner::<FlowWorkingScene>().ok_or_else(|| Fault::from("flow-retained-scene-owner-missing"))?;
        if scene.widgets.len() > FLOW_STORE_MAX_SCENE_ITEMS || scene.synapses.len() > FLOW_STORE_MAX_SCENE_ITEMS {
            return Err(Fault::from("flow-retained-scene-capacity"));
        }
        if let Some(widget) = scene.widgets.get(self.cursor) {
            let id = flow_widget_id(widget);
            for (slot, reference) in flow_graph_operation_references(command).into_iter().enumerate() {
                if reference.is_some_and(|reference| reference == id) {
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
        self.apply(command, snapshot, &config, interaction).map(ArtifactCommandWorkStep::Complete)
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
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "connectMediaPorts", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "reorganize", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "renameFlowWidget", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "nodeGraphEdit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "spotlightCommit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "runExtensionAction", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
    ];
}

struct FlowGraphOperationJobFactoryProofs;

impl FlowGraphOperationJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        document_schema: "flow.fixture",
        factory: "FlowGraphOperationJobFactory",
        factory_type: FlowGraphOperationJobFactory,
        contract: flow_graph_operation_contract(),
        tools: ["connectMediaPorts", "reorganize", "renameFlowWidget", "nodeGraphEdit", "spotlightCommit", "runExtensionAction"]
    }
}
//#endregion 🧵️GraphOperationRetainedRoute

struct FlowDirectStoreJobFactoryProofs;

impl FlowDirectStoreJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        document_schema: "flow.fixture",
        factory: "FlowDirectStoreJobFactory",
        factory_type: FlowDirectStoreJobFactory,
        contract: flow_direct_store_contract(),
        tools: [
            "removeWidget",
            "deleteSelection",
            "disconnect",
            "moveMediaNode",
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

struct FlowChildGroupJobFactoryProofs;

impl FlowChildGroupJobFactoryProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FlowPlayApp>,
        owner_file: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.flow.flow@1/*#editor",
        document_schema: "flow.fixture",
        factory: "FlowChildGroupJobFactory",
        factory_type: FlowChildGroupJobFactory,
        tools: {
            "addWidget" => semio_framework::ToolExecutionContract::resumable(16_384, 256, 1, 16_384, 7_500, 1, 1),
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

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(Arc::new(retained::artifact::preparation::PreparationFactory))
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::retirement::store_owners())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::no_config_store_owners())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
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

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = FlowDirectStoreJobFactoryProofs::bounded_first_step_tool_proofs();
        proofs.extend(FlowHostEffectJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(FlowChildGroupJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs.extend(FlowGraphOperationJobFactoryProofs::bounded_first_step_tool_proofs());
        proofs
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(FlowChildGroupJobFactory::new(&controller))?;
        registry.register(FlowHostEffectJobFactory::new(&controller))?;
        registry.register(FlowGraphOperationJobFactory::new(&controller))?;
        registry.register(FlowDirectStoreJobFactory::new(&controller))
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !FLOW_CHILD_GROUP_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_HOST_ONLY_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&request.tool_id.as_str())
            && !FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str())
        {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.retained.tool-mismatch"), "Flow command does not match its exact retained tool registration"));
        }
        if FLOW_CHILD_GROUP_TOOL_IDS.contains(&request.tool_id.as_str()) || FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&request.tool_id.as_str()) || FLOW_DIRECT_STORE_TOOL_IDS.contains(&request.tool_id.as_str()) {
            let tool_id = request.command.command_id();
            let work: Box<dyn ArtifactCommandWork<semio_framework_plugin::EditorApp<Self>>> = if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
                Box::new(FlowChildGroupWork::new(request.instance_operation_owner))
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
                if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
                    FLOW_CHILD_GROUP_RAW_BYTES
                } else if FLOW_GRAPH_OPERATION_TOOL_IDS.contains(&tool_id) {
                    FLOW_GRAPH_OPERATION_RAW_BYTES
                } else {
                    FLOW_DIRECT_STORE_RAW_BYTES
                },
                if FLOW_CHILD_GROUP_TOOL_IDS.contains(&tool_id) {
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

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
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
    fn interaction_topology(doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> InteractionTopology {
        let live = doc.snapshot.to_fixture();
        let mut ordered: Vec<TopologyNode> = live.widgets.iter().map(|widget| TopologyNode { id: flow_graph_node_target_id(crate::schema::widget_id(widget)), granularity: "node".into(), parent: None }).collect();
        ordered.extend(live.synapses.iter().map(|synapse| TopologyNode { id: flow_graph_edge_target_id(&synapse.id), granularity: "edge".into(), parent: None }));
        live.retire_cold();
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(FLOW_INTERACTION_GRAPH.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes — covers
    /// every mutation path (edits, undo/redo, example load, remote operations) in one place. Pure:
    /// recomputes the probe fresh from the fixture and the driver's persisted baseline each call.
    fn pending_effects(doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, _view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        evaluate::evaluate_result(doc.snapshot, &main::config::current(cfg), &mut FlowEvalSession::new()).effects
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let fixture = doc.snapshot;
        let config = main::config::current(cfg);
        let transient = main::transient::FlowWindowTransient::default();
        let labels = flow_play_labels(view_state);
        let mut session = FlowEvalSession::new();
        match body_key {
            FLOW_PLAY_BODY_MAIN => main::render(fixture, &config, &mut session).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_COMPILED => compiled::render(fixture, &config, &mut session).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATIONS => generations::render(&transient, view_state.locale, view_state.terminology).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATE_FORM => form::render(fixture, &config, &transient, labels).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(&transient).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_DOCUMENT => document_panel::render(fixture, labels).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(fixture, &config, &mut session, labels).map(semio_framework_plugin::built_to_component_tree),
            FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(labels).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn render_with_request_context(
        owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, FlowSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        _interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = main::config::current(cfg);
        let transient = main::transient::current(transient);
        owner
            .with_mut::<FlowInstanceOperationOwner, _>(|owner| {
                owner.with_session(|session| match body_key {
                    FLOW_PLAY_BODY_MAIN => main::render(doc.snapshot, &config, session).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_COMPILED => compiled::render(doc.snapshot, &config, session).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_GENERATIONS => generations::render(&transient, view_state.locale, view_state.terminology).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_GENERATE_FORM => form::render(doc.snapshot, &config, &transient, flow_play_labels(view_state)).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(&transient).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_DOCUMENT => document_panel::render(doc.snapshot, flow_play_labels(view_state)).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(doc.snapshot, &config, session, flow_play_labels(view_state)).map(semio_framework_plugin::built_to_component_tree),
                    FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(flow_play_labels(view_state)).map(semio_framework_plugin::built_to_component_tree),
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
        let is_de = view_state.locale == semio_framework_plugin::Locale::De;
        flow_context_menu_items(registry, doc.snapshot, &config, flow_play_labels(view_state), is_de, request.surface.as_ref())
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
pub fn host_from_snapshot(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession) -> FlowHost {
    let live = fixture.to_fixture();
    let mut host = flow_host_with_session(&live, session);
    live.retire_cold();
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    host
}

/// ✏️ Runs a stateful `FlowHost` mutation and diffs the result back into granular `FlowMutation`s —
/// returns an empty vec when `mutate` reports "nothing changed".
pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut FlowHost) -> bool) -> Vec<FlowMutation> {
    let mut host = host_from_snapshot(snapshot, config, session);
    if !mutate(&mut host) {
        host.retire_cold();
        return Vec::new();
    }
    let live = snapshot.to_fixture();
    let operations = flow_fixture_operations(&live, &host.fixture).unwrap_or_default();
    live.retire_cold();
    host.retire_cold();
    operations.into_iter().filter_map(crate::schema::mutations::from_framework_mutation).collect()
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
pub fn focus_selection_camera(fixture: &FlowSnapshot, config: &FlowMainWindowConfig, session: &FlowEvalSession, selected_node_ids: &[String]) -> Option<CameraJson> {
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
    Editor::builder(crate::FLOW_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) })
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung auflösen"), "runtime", ActionKind::View) })
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
        // 🌉️ COMPOSITE — plans create-widget then connect-widgets (ticket 26/08/16/…-COMPOSITE-MUTATIONS).
        .mutation("duplicateWidget", LocalizedLabel::native("Duplicate Widget", "Widget duplizieren"))
        // 🗂️ Referenced by flow_context_menu_items — categorized for grouped-context-menu disclosure.
        .action_with(ActionDefinition::bounded_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
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
        .action_interactive_job("selectGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("renameGeneration", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("updateGenerationValues", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalTick", semio_framework_plugin::InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalResolve", semio_framework_plugin::InteractiveJobClassification::Migrated)
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
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️interactive-job/🦀️.rs"]
mod interactive_job_tests;
//#endregion 🧪️Tests

#[cfg(test)]
use crate::flow_content_child_handle_bounded;
