//! 🖥️ Flow play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `🎚️options/*`, panel trees in `📌️panels/*`,
//! labels in `🗣️terminology/🦀️component.rs`, view state in `🎚️config/🦀️component.rs`, plugin registration
//! and `FlowHost` bridging (below — constitutional: general, an artifact must never depend on an app, so
//! both live here rather than under `🗿️artifacts`).
//! This file is a routing table: `handle` → `FlowCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::flow::commands::{
    add_widget, connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, evaluate, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport, open_spotlight,
    patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_catalogue_sections, set_contributions, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_locale, set_lod_mode,
    set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension,
};
use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::modes::edit::windows::{compiled, main};
use crate::apps::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use crate::apps::flow::modes::generate::windows::{form, generations, preview};
use crate::apps::flow::modes::{edit, generate};
use crate::apps::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::flow::presence::{FlowPresence, FlowPresenceMutation};
use crate::apps::flow::terminology::{flow_play_labels, FlowPlayLabels};
use crate::artifacts::flow::op::FlowMutation;
use crate::artifacts::flow::{FlowSnapshot, FLOW_DOCUMENT_SCHEMA};
use flow::{dag::DagDrawLod, flow_fixture_operations, flow_host_with_session, with_process_flow_eval_session, CameraJson, FlowEvalSession, FlowHost, Widget, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppActionRegistry, ArtifactApp, ArtifactView, CommandDefinition, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DomainTopology, DraftView, Emit,
    Fault, GranularityDefinition, HierarchyProvider, HostEffect, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec,
    TopologyNode, UiNode, WindowMeasure,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::EngineHandles;

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
pub fn flow_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(FLOW_PLAY_APP_ID).action(action, args)
}

/// 🙈️ An action that exists for dispatch but never appears in the command palette.
fn flow_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
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

//#region 🔖️FlowPlayApp
/// 🧪️ Unit struct apart from `eval_session`: every former runtime field lives in [`FlowConfig`], written
/// through [`FlowConfigMutation`]s. The eval session is the one piece of state that is neither document
/// nor view — it is the off-main-thread evaluation driver, threaded into every command handler as the
/// `app_commands!` dispatch context.
#[derive(Default)]
pub struct FlowPlayApp;

impl ArtifactApp for FlowPlayApp {
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

    const APP_ID: &'static str = FLOW_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::apps::flow::config::schema::app_schema_descriptor())
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
        with_process_flow_eval_session(|session| match command {
            FlowCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, session, interaction),
            FlowCommand::FocusSelection(payload) => focus_selection::apply(payload, doc, cfg, session, interaction),
            FlowCommand::NodeGraphEdit(payload) => node_graph_edit::apply(payload, doc, cfg, session, interaction),
            FlowCommand::SpotlightCommit(payload) => spotlight_commit::apply(payload, doc, cfg, session, interaction),
            _ => command.dispatch(doc, cfg, session),
        })
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
    fn pending_effects(doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>) -> Vec<HostEffect> {
        with_process_flow_eval_session(|session| evaluate::evaluate_result(doc.snapshot, cfg.snapshot, session).effects)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>) -> UiNode {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        let labels = flow_play_labels(config);
        with_process_flow_eval_session(|session| match body_key {
            FLOW_PLAY_BODY_MAIN => main::render(fixture, config, session),
            FLOW_PLAY_BODY_COMPILED => compiled::render(fixture, config, session),
            FLOW_PLAY_BODY_GENERATIONS => generations::render(config, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::Native),
            FLOW_PLAY_BODY_GENERATE_FORM => form::render(fixture, config),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(config),
            FLOW_PLAY_BODY_DOCUMENT => document_panel::render(fixture, labels),
            FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(fixture, config, session, labels),
            FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        })
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
pub fn create_flow_app() -> App {
    App::from_builder(
        App::builder(FLOW_PLAY_APP_ID, LocalizedLabel::native("Flow", "Flow"))
            .command(CommandDefinition { in_palette: false, ..CommandDefinition::new_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
            .command(CommandDefinition { in_palette: false, ..CommandDefinition::new_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) })
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
            // 🗂️ Referenced by flow_context_menu_items — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Mutation).with_category("selection"))
            .mutation("disconnect", LocalizedLabel::native("Disconnect", "Trennen"))
            .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Anschlüsse verbinden"))
            .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation).with_category("transform"))
            .mutation("patchFlowWidgets", LocalizedLabel::native("Patch Widgets", "Widgets aktualisieren"))
            .mutation("renameFlowWidget", LocalizedLabel::native("Rename Widget", "Widget umbenennen"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .mutation("spotlightCommit", LocalizedLabel::native("Spotlight Commit", "Spotlight bestätigen"))
            // 🧩️ Dynamic extension-provided action — id resolved at runtime, kept out of the palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("runExtensionAction", LocalizedLabel::native("Run Extension Action", "Erweiterungsaktion ausführen"), ActionKind::Mutation) })
            // 👁️ Ephemeral view/config actions — mutate config, emit no document operations. Selection/
            // hover verbs (`setSelection`/`clearSelection`/`selectAll`/`selectNode`/`nodeGraphSelect`/
            // `nodeGraphHover`/`graphPointerDown`) are no longer declared here: framework-owned, injected
            // via `.interaction(...)` below (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
            .view_action("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"))
            .action_with(ActionDefinition::new_catalog("focusSelection", LocalizedLabel::native("Zoom to Selection", "Auf Auswahl zoomen"), ActionKind::View).with_category("view"))
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
            .config(FlowPlayApp::config_spec()),
    )
    .example_source(crate::examples::art_flow_demo::source())
    .workflow("flow", "Flow", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type FlowApp = VcsArtifactApp<FlowPlayApp>;

    /// 🧪️ Installs a hand-authored `flow.extension` manifest fixture (a "math" module contributing the
    /// `math.add` operator) so tests exercising the catalogue/extension surfaces have something real
    /// installed — deliberately NOT the production `flow-extension-*` crates: flow-core must not
    /// dev-depend on its own extensions (audit finding C1, see ticket
    /// `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`'s `w3-flow.md`). Each real extension crate already
    /// exhaustively tests its own manifest/operator content in its own `#[cfg(test)] mod tests` (e.g.
    /// `flow-extension-math`'s `manifest_lists_math_operators_and_schemas`); this fixture only covers
    /// what flow-core's own tests assert on (`catalogue_lists_module_operators`).
    fn install_first_party_light_flow_extensions_for_tests() {
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
            flow::install_flow_extension_manifest("flow-core-test-fixture", &manifest_json);
        });
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn flow_app() -> FlowApp {
        install_first_party_light_flow_extensions_for_tests();
        new_app::<FlowPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn flow_app_with_registry() -> FlowApp {
        install_first_party_light_flow_extensions_for_tests();
        new_app_with_registry::<FlowPlayApp>(create_flow_app)
    }

    pub fn dispatch(app: &mut FlowApp, command: FlowCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn dispatch_with_registry(app: &mut FlowApp, command: FlowCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut FlowApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub fn main_window_measures(app: &mut FlowApp) -> Vec<WindowMeasure> {
        app.window_measures().get(main::FLOW_PLAY_WINDOW_MAIN).cloned().expect("main window measures")
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is the framework's injected
    /// `interactionSelect` verb now, dispatched against the "graph" domain declared on this app —
    /// requires `flow_app_with_registry()` (a bare `flow_app()` has no declared interaction domains to
    /// select against). `node_ids`/`edge_ids` are raw widget/synapse ids, converted to the row-id-
    /// prefixed `InteractionTarget` ids the document panel tree/`interaction_topology` both use (see
    /// `flow_graph_node_target_id`/`flow_graph_edge_target_id`).
    pub fn select_graph(app: &mut FlowApp, node_ids: &[&str], edge_ids: &[&str]) {
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
    use crate::apps::flow::testkit::{dispatch, flow_app, flow_app_with_registry, FlowApp};
    use semio_framework_plugin::testkit::{assert_undo_redo_round_trip, meta};
    use semio_framework_plugin::PluginApp;

    fn context_menu_items(app: &mut FlowApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface, window_instance_id: None, point: None };
        serde_json::to_value(app.context_menu(&request)).unwrap_or(Value::Null)
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 34, "every FlowCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the one documented divergence (`setLocale` → `locale`, an
    /// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
    /// silently breaks (the record prints with no keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
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
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
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
    pub(super) fn every_command() -> Vec<FlowCommand> {
        use flow::CameraJson;
        vec![
            FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None }),
            FlowCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "n1".into() }),
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
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_flow_app().definition).expect("app definition json");
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
    #[test]
    fn graph_interaction_domain_is_declared_topology_and_scoped_to_the_main_window() {
        let definition = create_flow_app().definition;
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
    #[test]
    fn interaction_topology_registers_every_widget_and_synapse_as_a_root() {
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
    #[test]
    fn undo_restores_fixture_after_add_widget() {
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

    #[test]
    fn generate_mode_renders_three_surfaces() {
        let mut app = flow_app();
        use crate::apps::flow::testkit::render;
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATIONS).contains("addGeneration"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).contains("Add a generation"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW).contains("text-editor"));
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::apps::flow::testkit::render;
        let mut app = flow_app();
        assert!(render(&mut app, "flow.play.nope").contains("Unknown body"));
    }

    #[test]
    fn host_from_snapshot_deletes_edge_selected_by_synapse_domain() {
        let config = FlowConfig::default();
        let fixture = FlowSnapshot::default();
        let session = FlowEvalSession::new();
        let mut host = host_from_snapshot(&fixture, &config, &session);
        sync_host_selection_domains(&mut host, &[], &["s1".into()], &[]);
        assert!(host.has_selection(), "s1 must resolve through host_from_snapshot edge map");
        host.delete_selection().expect("deleteSelection");
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
    }

    #[test]
    fn two_instances_converge_on_disjoint_edits() {
        use crate::artifacts::flow::schema::widget_id;
        use semio_framework_plugin::testkit::paired_apps;
        let (mut instance_a, mut instance_b) = paired_apps::<FlowPlayApp>("mem://flow-convergence");

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
    #[test]
    fn context_menu_includes_select_all_when_empty() {
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
    fn node_selection_surface(node_ids: &[&str]) -> semio_framework_plugin::ContextMenuSurfaceTarget {
        semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![],
            selection: vec![semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: node_ids.iter().map(|id| id.to_string()).collect() }],
            text: None,
        }
    }

    #[test]
    fn context_menu_includes_hide_preview_for_selection_and_set_preview_off_mutates_scene() {
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
    #[test]
    fn context_menu_at_selects_target_and_enables_preview() {
        let mut app = flow_app_with_registry();
        let before = context_menu_items(&mut app, None).to_string();
        assert!(!before.contains(r#""id":"delete-selection""#), "preview starts without delete: {before}");
        dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: "slider".into() }));
        let after = context_menu_items(&mut app, Some(node_selection_surface(&["slider"]))).to_string();
        assert!(after.contains("setPreviewOff"), "menu keeps preview: {after}");
        assert!(after.contains(r#""ids":["slider"]"#) || after.contains("slider"), "preview args target the clicked node: {after}");
    }

    #[test]
    fn context_menu_annotates_mixed_selection_counts_and_omits_delete_without_selection() {
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

    #[test]
    fn context_menu_for_edge_hit_uses_surface_edge_selection() {
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

    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
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
