//! 🖥️ Flow play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `🎚️options/*`, panel trees in `📌️panels/*`,
//! labels in `🗣️terminology/🦀️component.rs`, view state in `🎚️config/🦀️component.rs`, shared compute in
//! the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `FlowCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::flow::commands::{catalogue, eval, extension, grid, layout, locale, lod, node_graph, selection, synapse, view, widget};
use crate::apps::flow::config::{FlowConfig, FlowConfigOperation};
use crate::apps::flow::modes::edit::windows::{compiled, main};
use crate::apps::flow::modes::generate::commands::generation;
use crate::apps::flow::modes::generate::windows::{form, generations, preview};
use crate::apps::flow::modes::{edit, generate};
use crate::apps::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::flow::terminology::{flow_play_labels, FlowPlayLabels};
use crate::artifacts::flow::{op::FlowOperation, FlowFixture, FLOW_DOCUMENT_SCHEMA};
use flow::{with_process_flow_eval_session, FlowEvalSession, Widget};
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, 
    ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppActionRegistry, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView, Emit, Fault, HostEffect, Label, LocalizedLabel,
    UiNode, WindowMeasure,
};
use store::EngineHandles;
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const FLOW_PLAY_APP_ID: &str = "flow-play";
pub use compiled::FLOW_PLAY_BODY_COMPILED;
pub use main::FLOW_PLAY_BODY_MAIN;
pub use catalogue_panel::FLOW_PLAY_BODY_CATALOGUE;
pub use document_panel::FLOW_PLAY_BODY_DOCUMENT;
pub use form::FLOW_PLAY_BODY_GENERATE_FORM;
pub use generations::FLOW_PLAY_BODY_GENERATIONS;
pub use inspection_panel::FLOW_PLAY_BODY_INSPECTOR;
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

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `FlowPlayApp::Command` — the SOLE dispatch surface for flow's own behavior, assembled from the
    /// `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`, the
    /// camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different vocabularies, and
    /// `setLocale`/`locale` is the row that proves it. **Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break.**
    pub enum FlowCommand for FlowFixture, FlowOperation, FlowConfig, FlowConfigOperation, ctx = FlowEvalSession {
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
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
        "selectAll" as "select-all" => select_all::SelectAll,
        "focusSelection" as "focus-selection" => focus_selection::FocusSelection,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "selectNode" as "select-node" => select_node::SelectNode,
        "nodeGraphSelect" as "node-graph-select" => node_graph_select::NodeGraphSelect,
        "nodeGraphHover" as "node-graph-hover" => node_graph_hover::NodeGraphHover,
        "graphPointerDown" as "graph-pointer-down" => graph_pointer_down::GraphPointerDown,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setLodMode" as "set-lod-mode" => set_lod_mode::SetLodMode,
        "setProximityDistance" as "set-proximity-distance" => set_proximity_distance::SetProximityDistance,
        "setGridVisible" as "set-grid-visible" => set_grid_visible::SetGridVisible,
        "setGridSnapEnabled" as "set-grid-snap-enabled" => set_grid_snap_enabled::SetGridSnapEnabled,
        "setGridFactor" as "set-grid-factor" => set_grid_factor::SetGridFactor,
        "clearSelection" as "clear-selection" => clear_selection::ClearSelection,
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
// payload module is imported here under its own flat name. `reorganize`/`evaluate` collide with their
// containing command-group modules and are aliased.
use catalogue::set_catalogue_sections;
use eval::{evaluate, flow_eval_resolve, flow_eval_tick};
use extension::{run_extension_action, set_contributions, toggle_extension};
use generation::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use grid::{set_grid_factor, set_grid_snap_enabled, set_grid_visible};
use locale::set_locale;
use lod::{set_lod_mode, set_proximity_distance};
use node_graph::{node_graph_edit, spotlight_commit};
use layout::reorganize;
use selection::{clear_selection, context_menu_at, delete_selection, focus_selection, graph_pointer_down, node_graph_select, select_all, select_node, set_selection};
use synapse::{connect_media_ports, disconnect};
use view::{node_graph_hover, node_graph_viewport, open_spotlight, replace_image, set_preview_off};
use widget::{add_widget, move_media_node, patch_flow_widgets, remove_widget, rename_flow_widget};
//#endregion 🔖️Commands

//#region 🔖️ContextMenu
/// 🖱️ On-demand flow node-graph context menu from surface hit-test and selection snapshot.
fn flow_context_menu_items(registry: &AppActionRegistry, fixture: &FlowFixture, config: &FlowConfig, labels: &FlowPlayLabels, is_de: bool, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, Menu};

    let hits = surface.map_or(&[][..], |target| target.hits.as_slice());
    let groups = surface.map_or(&[][..], |target| target.selection.as_slice());
    let mut nodes: Vec<String> = groups.iter().filter(|group| group.domain == "node").flat_map(|group| group.ids.iter().cloned()).collect();
    let mut edges: Vec<String> = groups.iter().filter(|group| group.domain == "edge").flat_map(|group| group.ids.iter().cloned()).collect();
    if nodes.is_empty() && edges.is_empty() {
        nodes = config.selected_node_ids.clone();
        edges = config.selected_edge_ids.clone();
    }
    let has_selection = !nodes.is_empty() || !edges.is_empty();
    let all_preview_off = !nodes.is_empty() && nodes.iter().all(|id| config.preview_off_node_ids.contains(id));
    let is_image = nodes.len() == 1
        && fixture.widgets.iter().any(|widget| match widget {
            Widget::InputImage { id, .. } => id == &nodes[0],
            _ => false,
        });
    let primary = hits.first();
    let hit_node = primary.filter(|hit| hit.domain == "node").map(|hit| hit.id.as_str());

    // 🗂️ Grouped disclosure: `add-node`/`selectAll`/`focusSelection`/`clearSelection` stay top-level
    // (the 3-5 most frequent verbs); `reorganize`/`replaceImage`/`toggle-preview` fold into taxonomy
    // groups; `delete-selection` stays a direct destructive item last — `organize_context_menu`
    // (applied automatically at the `VcsDocumentApp::context_menu` funnel) sorts the groups into
    // `RIBBON_PARENT_CATEGORIES` order and inserts the pre-destructive separator itself.
    let mut menu = Menu::of(registry);
    if hits.is_empty() {
        menu = menu
            .item(ContextMenuItemSpec { id: "add-node".into(), label: Some(labels.add_node.into()), icon: Some("plus".into()), action: Some("openSpotlight".into()), ..Default::default() })
            .action("selectAll")
            .group("transform", |m| m.action("reorganize"));
    }
    if let Some(node_id) = hit_node {
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
/// through [`FlowConfigOperation`]s. The eval session is the one piece of state that is neither document
/// nor view — it is the off-main-thread evaluation driver, threaded into every command handler as the
/// `app_commands!` dispatch context.
#[derive(Default)]
pub struct FlowPlayApp;

impl DocumentApp for FlowPlayApp {
    type Projection = FlowFixture;
    type Operation = FlowOperation;
    type Config = FlowConfig;
    type ConfigOperation = FlowConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = FlowCommand;

    const APP_ID: &'static str = FLOW_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = FLOW_DOCUMENT_SCHEMA;

    fn initial_projection() -> FlowFixture {
        FlowFixture::default()
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale`/`flowEvalTick`/`flowEvalResolve` have no
    /// manifest declaration (host-pushed/internally-chained, not user-facing actions).
    fn command_id(command: &FlowCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &FlowCommand, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<FlowOperation, FlowConfigOperation, Self::DraftOperation>, Fault> {
        with_process_flow_eval_session(|session| command.dispatch(doc, cfg, session))
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes — covers
    /// every mutation path (edits, undo/redo, example load, remote operations) in one place. Pure:
    /// recomputes the probe fresh from the fixture and the driver's persisted baseline each call.
    fn pending_effects(doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>) -> Vec<HostEffect> {
        with_process_flow_eval_session(|session| eval::evaluate_result(doc.projection, cfg.projection, session).effects)
    }

    fn render(body_key: &str, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>) -> UiNode {
        let fixture = doc.projection;
        let config = cfg.projection;
        let labels = flow_play_labels(config);
        with_process_flow_eval_session(|session| match body_key {
            FLOW_PLAY_BODY_MAIN => main::render(fixture, config, session),
            FLOW_PLAY_BODY_COMPILED => compiled::render(fixture, config, session),
            FLOW_PLAY_BODY_GENERATIONS => generations::render(config, semio_framework_plugin::locale_from_str(&config.locale), semio_framework_plugin::Terminology::Native),
            FLOW_PLAY_BODY_GENERATE_FORM => form::render(fixture, config),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => preview::render(config),
            FLOW_PLAY_BODY_DOCUMENT => document_panel::render(fixture, &config.selected_node_ids, labels),
            FLOW_PLAY_BODY_CATALOGUE => catalogue_panel::render(fixture, config, session, labels),
            FLOW_PLAY_BODY_INSPECTOR => inspection_panel::render(fixture, &config.selected_node_ids, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        })
    }

    fn window_measures(_doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        HashMap::from([(main::FLOW_PLAY_WINDOW_MAIN.to_string(), main::window_measures(config, flow_play_labels(config)))])
    }

    fn context_menu(request: &ContextMenuRequest, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let config = cfg.projection;
        let is_de = config.locale.starts_with("de");
        flow_context_menu_items(registry, doc.projection, config, flow_play_labels(config), is_de, request.surface.as_ref())
    }
}
//#endregion 🔖️FlowPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_flow_app() -> App {
    App::from_builder(
        App::builder(FLOW_PLAY_APP_ID, LocalizedLabel::native("Flow", "Flow"))
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
            .operation("addWidget", LocalizedLabel::native("Add Widget", "Widget hinzufügen"))
            .operation("removeWidget", LocalizedLabel::native("Remove Widget", "Widget entfernen"))
            // 🗂️ Referenced by flow_context_menu_items — categorized for grouped-context-menu disclosure.
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .operation("disconnect", LocalizedLabel::native("Disconnect", "Trennen"))
            .operation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Anschlüsse verbinden"))
            .operation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
            .action_with(ActionDefinition::new_catalog("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Operation).with_category("transform"))
            .operation("patchFlowWidgets", LocalizedLabel::native("Patch Widgets", "Widgets aktualisieren"))
            .operation("renameFlowWidget", LocalizedLabel::native("Rename Widget", "Widget umbenennen"))
            .operation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .operation("spotlightCommit", LocalizedLabel::native("Spotlight Commit", "Spotlight bestätigen"))
            // 🧩️ Dynamic extension-provided action — id resolved at runtime, kept out of the palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("runExtensionAction", LocalizedLabel::native("Run Extension Action", "Erweiterungsaktion ausführen"), ActionKind::Operation) })
            // 👁️ Ephemeral view/config actions — mutate config, emit no document operations.
            .view_action("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"))
            .action_with(ActionDefinition::new_catalog("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"), ActionKind::View).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("focusSelection", LocalizedLabel::native("Zoom to Selection", "Auf Auswahl zoomen"), ActionKind::View).with_category("view"))
            .action_with(flow_internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(flow_internal_action("selectNode", LocalizedLabel::native("Select Node", "Knoten auswählen"), ActionKind::View))
            .action_with(flow_internal_action("nodeGraphSelect", LocalizedLabel::native("Node Graph Select", "Knotengraph auswählen"), ActionKind::View))
            .action_with(flow_internal_action("nodeGraphHover", LocalizedLabel::native("Node Graph Hover", "Knotengraph-Hover"), ActionKind::View))
            .action_with(flow_internal_action("graphPointerDown", LocalizedLabel::native("Graph Pointer Down", "Graph-Zeiger gedrückt"), ActionKind::View))
            .action_with(flow_internal_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"), ActionKind::View))
            .action_with(flow_internal_action("setLodMode", LocalizedLabel::native("Set LOD Mode", "LOD-Modus festlegen"), ActionKind::View))
            .action_with(flow_internal_action("setProximityDistance", LocalizedLabel::native("Set Proximity Distance", "Näheabstand festlegen"), ActionKind::View))
            .action_with(flow_internal_action("setGridVisible", LocalizedLabel::native("Set Grid Visible", "Raster sichtbar"), ActionKind::View))
            .action_with(flow_internal_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"), ActionKind::View))
            .action_with(flow_internal_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"), ActionKind::View))
            .action_with(flow_internal_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"), ActionKind::View).with_category("selection"))
            .action_with(flow_internal_action("contextMenuAt", LocalizedLabel::native("Context Menu At", "Kontextmenü an Position"), ActionKind::View))
            .action_with(flow_internal_action("setPreviewOff", LocalizedLabel::native("Set Preview Off", "Vorschau deaktivieren"), ActionKind::View).with_category("view"))
            .action_with(flow_internal_action("openSpotlight", LocalizedLabel::native("Open Spotlight", "Spotlight öffnen"), ActionKind::View).with_category("create"))
            .action_with(flow_internal_action("replaceImage", LocalizedLabel::native("Replace Image", "Bild ersetzen"), ActionKind::View).with_category("actions"))
            .action_with(flow_internal_action("setCatalogueSections", LocalizedLabel::native("Set Catalogue Sections", "Katalogabschnitte festlegen"), ActionKind::View))
            .action_with(flow_internal_action("toggleAutomation", LocalizedLabel::native("Toggle Extension", "Erweiterung umschalten"), ActionKind::View))
            .action_with(flow_internal_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::View))
            .action_with(flow_internal_action("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::View))
            .action_with(flow_internal_action("selectGeneration", LocalizedLabel::native("Select Generation", "Generation auswählen"), ActionKind::View))
            .action_with(flow_internal_action("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::View))
            .action_with(flow_internal_action("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::View))
            // 📝️ Staged argument form for the panel-visible create action (module operators stay catalogue-driven).
            .action_args("addWidget", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
            ])
            .default_value("inputSlider")])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+a", "selectAll")
            .keybinding("delete,backspace", "deleteSelection")
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
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type FlowApp = VcsDocumentApp<FlowPlayApp>;

    fn install_first_party_light_flow_extensions_for_tests() {
        use std::sync::Once;
        static ONCE: Once = Once::new();
        ONCE.call_once(|| {
            for (plugin_id, manifest) in [
                ("flow-extension-core", semio_s_plugin_flow_extension_core::extension_manifest_json()),
                ("flow-extension-math", semio_s_plugin_flow_extension_math::extension_manifest_json()),
                ("flow-extension-text", semio_s_plugin_flow_extension_text::extension_manifest_json()),
                ("flow-extension-logic", semio_s_plugin_flow_extension_logic::extension_manifest_json()),
                ("flow-extension-dictionary", semio_s_plugin_flow_extension_dictionary::extension_manifest_json()),
                ("flow-extension-list", semio_s_plugin_flow_extension_list::extension_manifest_json()),
            ] {
                flow::install_flow_extension_manifest(plugin_id, &manifest);
            }
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
        assert_eq!(ids.len(), 41, "every FlowCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
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
            (
                FlowCommand::AddWidget(add_widget::AddWidget { kind: "neuron".into(), neuron_kind: Some("math.add".into()), x: None, y: None }),
                "add-widget kind=neuron neuron-kind=math.add",
                "010002086d6174682e616464066e6575726f6e02000601010600",
            ),
            (FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: None }), "set-grid-visible", "01170000"),
            (FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(true) }), "set-grid-visible pressed=true", "011700010002"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::test_support::assert_op_text_binary_equivalence(&command);
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
                    node_graph::FlowNodeGraphEditOp::SetFixture { fixture_json: "{}".into() },
                    node_graph::FlowNodeGraphEditOp::DeleteSelection,
                    node_graph::FlowNodeGraphEditOp::Connect { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() },
                ],
            }),
            FlowCommand::SpotlightCommit(spotlight_commit::SpotlightCommit { operations: vec![node_graph::FlowNodeGraphEditOp::DeleteSelection] }),
            FlowCommand::RunExtensionAction(run_extension_action::RunExtensionAction { action_id: "flow.extension.reorganize".into() }),
            FlowCommand::Evaluate(evaluate::Evaluate {}),
            FlowCommand::SelectAll(select_all::SelectAll {}),
            FlowCommand::FocusSelection(focus_selection::FocusSelection {}),
            FlowCommand::SetSelection(set_selection::SetSelection { ids: vec!["n1".into()], edge_ids: vec!["e1".into()], handle_ids: Vec::new() }),
            FlowCommand::SelectNode(select_node::SelectNode { node_id: "n1".into() }),
            FlowCommand::NodeGraphSelect(node_graph_select::NodeGraphSelect { node_ids: vec!["n1".into(), "n2".into()] }),
            FlowCommand::NodeGraphHover(node_graph_hover::NodeGraphHover {}),
            FlowCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
            FlowCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: CameraJson { x: 1.0, y: 2.0, zoom: 1.5 } }),
            FlowCommand::SetLodMode(set_lod_mode::SetLodMode { value: "micro".into() }),
            FlowCommand::SetProximityDistance(set_proximity_distance::SetProximityDistance { value: 48.0 }),
            FlowCommand::SetGridVisible(set_grid_visible::SetGridVisible { pressed: Some(true) }),
            FlowCommand::SetGridSnapEnabled(set_grid_snap_enabled::SetGridSnapEnabled { pressed: None }),
            FlowCommand::SetGridFactor(set_grid_factor::SetGridFactor { value: 10.0 }),
            FlowCommand::ClearSelection(clear_selection::ClearSelection {}),
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

    //#region 🔖️CrossCutting
    #[test]
    fn undo_restores_fixture_after_add_widget() {
        let mut app = flow_app();
        let before = app.projection().expect("projection").widgets.len();
        assert_undo_redo_round_trip(&mut app, FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) }), |app| app.projection().expect("projection").widgets.len(), before, before + 1);
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
    fn two_instances_converge_on_disjoint_edits() {
        use crate::artifacts::flow::engine::widget_id;
        use semio_framework_plugin::testkit::paired_apps;
        let (mut instance_a, mut instance_b) = paired_apps::<FlowPlayApp>("mem://flow-convergence");

        instance_a.dispatch_typed(FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: "input".into() }), &meta("actor-a")).expect("a renames slider");
        instance_b.dispatch_typed(FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(10.0), y: Some(10.0) }), &meta("actor-b")).expect("b adds a note");

        // A neutral history action always dispatches through the store, which pumps inbound operations first.
        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
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

    #[test]
    fn context_menu_includes_hide_preview_for_selection_and_set_preview_off_mutates_scene() {
        let mut app = flow_app_with_registry();
        dispatch(&mut app, FlowCommand::SetSelection(set_selection::SetSelection { ids: vec!["slider".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }));
        let menu = context_menu_items(&mut app, None).to_string();
        assert!(menu.contains("setPreviewOff"), "menu should expose preview toggle: {menu}");
        assert!(menu.contains("Hide preview") || menu.contains("eye-off"), "menu should offer hide preview: {menu}");
        assert!(menu.contains("focusSelection"), "menu should expose zoom to selection: {menu}");
        assert!(menu.contains(r#""checked":true"#), "preview checked when visible: {menu}");
        dispatch(&mut app, FlowCommand::SetPreviewOff(set_preview_off::SetPreviewOff { ids: vec!["slider".into()], value: true }));
        let after_menu = context_menu_items(&mut app, None).to_string();
        assert!(after_menu.contains("Show preview") || after_menu.contains(r#""icon":"eye""#), "menu should offer show preview: {after_menu}");
    }

    #[test]
    fn context_menu_at_selects_target_and_enables_preview() {
        let mut app = flow_app_with_registry();
        let before = context_menu_items(&mut app, None).to_string();
        assert!(!before.contains(r#""id":"delete-selection""#), "preview starts without delete: {before}");
        dispatch(&mut app, FlowCommand::ContextMenuAt(context_menu_at::ContextMenuAt { id: "slider".into() }));
        let after = context_menu_items(&mut app, None).to_string();
        assert!(after.contains("setPreviewOff"), "menu keeps preview: {after}");
        assert!(after.contains(r#""ids":["slider"]"#) || after.contains("slider"), "preview args target the clicked node: {after}");
    }

    #[test]
    fn context_menu_annotates_mixed_selection_counts_and_omits_delete_without_selection() {
        let mut app = flow_app_with_registry();
        let empty = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "main".into(), kind: "nodeGraph".into(), hits: vec![], selection: vec![], text: None })).to_string();
        assert!(!empty.contains(r#""id":"delete-selection""#), "empty must omit delete: {empty}");

        dispatch(
            &mut app,
            FlowCommand::SetSelection(set_selection::SetSelection { ids: (1..=8).map(|i| format!("n{i}")).collect(), edge_ids: (1..=13).map(|i| format!("e{i}")).collect(), handle_ids: Vec::new() }),
        );
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
    fn context_menu_for_edge_hit_uses_config_edge_selection() {
        let mut app = flow_app_with_registry();
        dispatch(&mut app, FlowCommand::SetSelection(set_selection::SetSelection { ids: Vec::new(), edge_ids: vec!["syn-1".into()], handle_ids: Vec::new() }));
        let menu = context_menu_items(
            &mut app,
            Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: "main".into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "edge".into(), id: "syn-1".into(), label: None }],
                selection: vec![],
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
        dispatch(
            &mut app,
            FlowCommand::SetSelection(set_selection::SetSelection { ids: (1..=8).map(|i| format!("n{i}")).collect(), edge_ids: (1..=13).map(|i| format!("e{i}")).collect(), handle_ids: Vec::new() }),
        );
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
