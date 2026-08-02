//! 🖥️ Flow app — DocumentApp impl, render, manifest (constitutional: ui).

use flow::{FlowFixture, FLOW_DOCUMENT_SCHEMA};
use flow_core::{
    dag::{dag_lod_scale_json, DagDrawLod},
    forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
    CameraJson, FlowEvalDriver, FlowHost, Widget, FLOW_LOD_MODE_AUTOMATIC,
};
use flow_core::{flow_backed_node_graph_extras, flow_fixture_operations};
use flow_engine::{flow_play_neural_cache, flow_widget_descriptor, flow_widget_drag_json, fixture_to_workflow, seed_host_catalogue, sync_host_selection, sync_host_selection_domains, widget_id, widget_kind_label, widget_tree_label};
use flow_op::FlowOperation;
use playbook::{handle_generation_action, render_generation_form_body, render_generation_preview_text, render_generations_tree, selected_generation, GenerationPlayState};
use semio_framework_plugin::{
    build_node_graph_scene, build_text_editor_scene, create_default_layout, create_named_layout, is_de_locale, localized_label_map, resolve_labels, tree_item_desc, tree_item_with_action, tree_item_with_action_draggable,
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionEmit, ActionKind,
    App, AppLabelsOverlay, AppLabelsOverlayExt, AppActionRegistry, ContextMenuRequest, ContextMenuItemSpec, DocumentApp, DocumentView, HostEffect, NodeGraphScene, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, PanelTreeBuilder, ArtifactKindSpec, SurfaceKind, TextEditorScene, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence,
    UiTreeItemNode, UiTreeSectionNode, ViewState, WindowMeasure, MeasureSelectItem, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
    FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
const FLOW_PLAY_APP_ID: &str = "flow-play";
const FLOW_PLAY_SURFACE_MAIN: &str = "flow.play.main";
const FLOW_PLAY_SURFACE_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_BODY_MAIN: &str = "flow.play.main";
const FLOW_PLAY_BODY_COMPILED: &str = "flow.play.compiled-dag";
const FLOW_PLAY_BODY_DOCUMENT: &str = "flow.play.document";
const FLOW_PLAY_BODY_CATALOGUE: &str = "flow.play.catalogue";
const FLOW_PLAY_BODY_INSPECTOR: &str = "flow.play.inspection";
const FLOW_PLAY_WINDOW_MAIN: &str = "flow-main";
const FLOW_PLAY_WINDOW_COMPILED: &str = "flow-compiled-dag";
const FLOW_PLAY_WINDOW_GENERATIONS: &str = "flow-generations";
const FLOW_PLAY_WINDOW_GENERATE_FORM: &str = "flow-generate-form";
const FLOW_PLAY_WINDOW_GENERATE_PREVIEW: &str = "flow-generate-preview";
const FLOW_PLAY_BODY_GENERATIONS: &str = "flow.play.generations";
const FLOW_PLAY_BODY_GENERATE_FORM: &str = "flow.play.generate-form";
const FLOW_PLAY_BODY_GENERATE_PREVIEW: &str = "flow.play.generate-preview";
const FLOW_PLAY_SURFACE_GENERATE_PREVIEW: &str = "flow.play.generate-preview";
const FLOW_DEFAULT_PROXIMITY_DISTANCE: f64 = 48.0;
const FLOW_DEFAULT_GRID_FACTOR: f64 = 10.0;

/// 🧩️ Built-in flow extensions: (id, name, actionId, actionTitle, effect).
const FLOW_EXTENSIONS: &[(&str, &str, &str, &str, &str)] =
    &[("auto-layout", "Auto Layout", "flow.extension.reorganize", "Reorganize Canvas", "reorganize"), ("auto-evaluate", "Auto Evaluate", "flow.extension.evaluate", "Evaluate Fixture", "evaluate")];
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🎛️ Ephemeral view/config state — selection, camera, live eval preview, LOD/catalogue/extension
/// config, and the generate-mode exploration surface — lives in the app struct, never the document,
/// so panning, selecting, and previewing never pollute undo history.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct FlowPlayRuntime {
    selected_node_ids: Vec<String>,
    #[serde(default)]
    selected_edge_ids: Vec<String>,
    #[serde(default)]
    selected_handle_ids: Vec<String>,
    #[serde(default)]
    preview_off_node_ids: Vec<String>,
    camera: CameraJson,
    /// 🧵️ Off-main-thread evaluation state — see `FlowEvalDriver`. Explicit "evaluate" arms it; the
    /// "auto-evaluate" extension additionally re-arms it after every mutation (see `pending_effects`).
    eval_driver: FlowEvalDriver,
    lod_mode: String,
    #[serde(default = "default_proximity_distance")]
    proximity_distance: f64,
    #[serde(default = "default_grid_visible")]
    grid_visible: bool,
    grid_snap_enabled: bool,
    #[serde(default = "default_grid_factor")]
    grid_factor: f64,
    catalogue_sections_json: String,
    extension_enabled: HashMap<String, bool>,
    generation: GenerationPlayState,
}

fn default_proximity_distance() -> f64 {
    FLOW_DEFAULT_PROXIMITY_DISTANCE
}

fn default_grid_factor() -> f64 {
    FLOW_DEFAULT_GRID_FACTOR
}

fn default_grid_visible() -> bool {
    true
}

fn default_flow_lod_mode() -> String {
    FLOW_LOD_MODE_AUTOMATIC.into()
}

fn default_catalogue_sections_json() -> String {
    "[]".into()
}

impl Default for FlowPlayRuntime {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            selected_edge_ids: Vec::new(),
            selected_handle_ids: Vec::new(),
            preview_off_node_ids: Vec::new(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            eval_driver: FlowEvalDriver::default(),
            lod_mode: default_flow_lod_mode(),
            proximity_distance: default_proximity_distance(),
            grid_visible: default_grid_visible(),
            grid_snap_enabled: false,
            grid_factor: default_grid_factor(),
            catalogue_sections_json: default_catalogue_sections_json(),
            extension_enabled: HashMap::new(),
            generation: GenerationPlayState::default(),
        }
    }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
fn flow_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: FLOW_PLAY_APP_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

fn apply_canvas_options(host: &mut FlowHost, runtime: &FlowPlayRuntime) {
    if runtime.lod_mode != FLOW_LOD_MODE_AUTOMATIC && DagDrawLod::from_id(&runtime.lod_mode).is_some() {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(&runtime.lod_mode);
    } else {
        host.dag.set_automatic_lod(true);
    }
    host.dag.set_proximity_distance(runtime.proximity_distance);
    host.set_grid_visible(runtime.grid_visible);
    host.set_grid_snap_enabled(runtime.grid_snap_enabled);
    let _ = host.set_grid_factor(runtime.grid_factor);
}

fn host_from_fixture(fixture: &FlowFixture, runtime: &FlowPlayRuntime) -> FlowHost {
    let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), flow_play_neural_cache());
    host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
    seed_host_catalogue(&mut host, &runtime.catalogue_sections_json);
    apply_canvas_options(&mut host, runtime);
    runtime.eval_driver.install_baseline_into(&mut host);
    host
}

/// 🌉️ Runs a `FlowHost` mutation over the current document fixture and diffs the result into granular
/// `FlowOperation`s. `mutate` returns `true` if it changed the fixture; a non-mutating call yields no operations.
fn host_operations(fixture: &FlowFixture, runtime: &FlowPlayRuntime, mutate: impl FnOnce(&mut FlowHost) -> bool) -> Vec<FlowOperation> {
    let mut host = host_from_fixture(fixture, runtime);
    if !mutate(&mut host) {
        return Vec::new();
    }
    flow_fixture_operations(fixture, &host.fixture)
}

/// 🖱️ On-demand flow node-graph context menu from surface hit-test and selection snapshot.
fn flow_context_menu_items(
    registry: &semio_framework_plugin::AppActionRegistry,
    fixture: &FlowFixture,
    runtime: &FlowPlayRuntime,
    labels: &FlowPlayLabels,
    is_de: bool,
    surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>,
) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};

    let hits = surface.map(|target| target.hits.as_slice()).unwrap_or(&[]);
    let groups = surface.map(|target| target.selection.as_slice()).unwrap_or(&[]);
    let mut nodes: Vec<String> = groups.iter().filter(|group| group.domain == "node").flat_map(|group| group.ids.iter().cloned()).collect();
    let mut edges: Vec<String> = groups.iter().filter(|group| group.domain == "edge").flat_map(|group| group.ids.iter().cloned()).collect();
    if nodes.is_empty() && edges.is_empty() {
        nodes = runtime.selected_node_ids.clone();
        edges = runtime.selected_edge_ids.clone();
    }
    let has_selection = !nodes.is_empty() || !edges.is_empty();
    let all_preview_off = !nodes.is_empty() && nodes.iter().all(|id| runtime.preview_off_node_ids.contains(id));
    let is_image = nodes.len() == 1
        && fixture.widgets.iter().any(|widget| match widget {
            Widget::InputImage { id, .. } => id == &nodes[0],
            _ => false,
        });
    let primary = hits.first();
    let hit_node = primary.filter(|hit| hit.domain == "node").map(|hit| hit.id.as_str());

    let mut menu = Menu::of(registry);
    if hits.is_empty() {
        menu = menu
            .item(ContextMenuItemSpec {
                id: "add-node".into(),
                label: Some(labels.add_node.into()),
                icon: Some("plus".into()),
                action: Some("openSpotlight".into()),
                ..Default::default()
            })
            .action("selectAll")
            .action("reorganize");
    }
    if let Some(node_id) = hit_node {
        if is_image {
            menu = menu.item(ContextMenuItemSpec {
                id: "replace-image".into(),
                label: Some(labels.replace_image.into()),
                icon: Some("image".into()),
                action: Some("replaceImage".into()),
                args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "id": node_id }))),
                ..Default::default()
            });
        }
    }
    if has_selection {
        menu = menu.separator().item(ContextMenuItemSpec {
            id: "toggle-preview".into(),
            label: Some(if all_preview_off { labels.show_preview.into() } else { labels.hide_preview.into() }),
            icon: Some(if all_preview_off { "eye".into() } else { "eye-off".into() }),
            checked: Some(!all_preview_off),
            action: Some("setPreviewOff".into()),
            args: semio_framework_plugin::optional_json_to_dsl(Some(json!({ "ids": nodes, "value": !all_preview_off }))),
            ..Default::default()
        });
        menu = menu.action("focusSelection").action("clearSelection");
        let phrase = selection_count_phrase(
            is_de,
            &[
                (nodes.len(), if is_de { "Knoten" } else { "node" }, if is_de { "Knoten" } else { "nodes" }),
                (edges.len(), if is_de { "Kante" } else { "edge" }, if is_de { "Kanten" } else { "edges" }),
            ],
        );
        if !phrase.is_empty() {
            menu = menu.separator().item(ContextMenuItemSpec {
                id: "delete-selection".into(),
                label: Some(format!("{} ({phrase})", labels.delete_selection)),
                icon: Some("trash".into()),
                destructive: Some(true),
                action: Some("deleteSelection".into()),
                ..Default::default()
            });
        }
    }
    menu.build()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the flow app; one field per label makes every locale combination compile-checked.
    struct FlowPlayLabels {
        widgets: &'static str = en: "Widgets", de: "Widgets";
        synapses: &'static str = en: "Synapses", de: "Synapsen";
        extensions: &'static str = en: "Extensions", de: "Erweiterungen";
        extension_actions: &'static str = en: "Extension Actions", de: "Erweiterungsaktionen";
        sources: &'static str = en: "Sources", de: "Quellen";
        components: &'static str = en: "Components", de: "Komponenten";
        sinks: &'static str = en: "Sinks", de: "Senken";
        catalogue_slider: &'static str = en: "Slider", de: "Schieberegler";
        catalogue_note: &'static str = en: "Note", de: "Notiz";
        catalogue_add: &'static str = en: "Add", de: "Addieren";
        catalogue_and: &'static str = en: "And", de: "Und";
        catalogue_concat: &'static str = en: "Concat", de: "Verketten";
        catalogue_preview: &'static str = en: "Preview", de: "Vorschau";
        catalogue_export: &'static str = en: "Export", de: "Exportieren";
        extension_auto_layout: &'static str = en: "Auto Layout", de: "Automatisches Layout";
        extension_auto_evaluate: &'static str = en: "Auto Evaluate", de: "Automatisch Auswerten";
        extension_action_reorganize_canvas: &'static str = en: "Reorganize Canvas", de: "Leinwand neu anordnen";
        extension_action_evaluate_fixture: &'static str = en: "Evaluate Fixture", de: "Fixture auswerten";
        canvas: &'static str = en: "Canvas", de: "Leinwand";
        widget: &'static str = en: "Widget", de: "Widget";
        delete_selection: &'static str = en: "Delete selection", de: "Auswahl löschen";
        hide_preview: &'static str = en: "Hide preview", de: "Vorschau ausblenden";
        show_preview: &'static str = en: "Show preview", de: "Vorschau einblenden";
        add_node: &'static str = en: "Add node…", de: "Knoten hinzufügen…";
        reorganize: &'static str = en: "Reorganize", de: "Neu anordnen";
        replace_image: &'static str = en: "Replace image…", de: "Bild ersetzen…";
        window_main: &'static str = en: "Flow", de: "Flow";
        window_compiled: &'static str = en: "DSL", de: "DSL";
        window_generations: &'static str = en: "Generations", de: "Generationen";
        window_generate_form: &'static str = en: "Form", de: "Formular";
        window_generate_preview: &'static str = en: "Preview", de: "Vorschau";
        lod_mode: &'static str = en: "LOD Mode", de: "LOD-Modus";
        automatic: &'static str = en: "Automatic", de: "Automatisch";
        proximity_distance: &'static str = en: "Proximity Distance", de: "Näheabstand";
        grid: &'static str = en: "Grid", de: "Raster";
        grid_visible: &'static str = en: "Visible", de: "Sichtbar";
        grid_snap: &'static str = en: "Snap", de: "Fang";
        grid_factor: &'static str = en: "Factor", de: "Faktor";
        select_all: &'static str = en: "Select All", de: "Alles auswählen";
        zoom_to_selection: &'static str = en: "Zoom to Selection", de: "Auf Auswahl zoomen";
        clear_selection: &'static str = en: "Clear Selection", de: "Auswahl aufheben";
        no_selection: &'static str = en: "No selection", de: "Keine Auswahl";
        value: &'static str = en: "Value", de: "Wert";
        text: &'static str = en: "Text", de: "Text";
        kind: &'static str = en: "Kind", de: "Art";
        id: &'static str = en: "Id", de: "Id";
    }
}

/// 🗣️ Resolves a built-in extension's display name from its stable id; unknown ids fall back to the extension's native English name.
fn flow_extension_label(id: &str, name: &'static str, labels: &FlowPlayLabels) -> &'static str {
    match id {
        "auto-layout" => labels.extension_auto_layout,
        "auto-evaluate" => labels.extension_auto_evaluate,
        _ => name,
    }
}

/// 🗣️ Resolves a built-in extension action's display title from its stable action id; unknown ids fall back to the action's native English title.
fn flow_extension_action_title_label(action_id: &str, title: &'static str, labels: &FlowPlayLabels) -> &'static str {
    match action_id {
        "flow.extension.reorganize" => labels.extension_action_reorganize_canvas,
        "flow.extension.evaluate" => labels.extension_action_evaluate_fixture,
        _ => title,
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_flow_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn flow_action_labels(is_de: bool) -> HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("addWidget", "Add Widget", "Widget hinzufügen"),
        ("removeWidget", "Remove Widget", "Widget entfernen"),
        ("deleteSelection", "Delete Selection", "Auswahl löschen"),
        ("disconnect", "Disconnect", "Trennen"),
        ("connectMediaPorts", "Connect Ports", "Anschlüsse verbinden"),
        ("moveMediaNode", "Move Node", "Knoten verschieben"),
        ("reorganize", "Reorganize", "Neu anordnen"),
        ("patchFlowWidgets", "Patch Widgets", "Widgets aktualisieren"),
        ("renameFlowWidget", "Rename Widget", "Widget umbenennen"),
        ("nodeGraphEdit", "Node Graph Edit", "Knotengraph bearbeiten"),
        ("spotlightCommit", "Spotlight Commit", "Spotlight bestätigen"),
        ("runExtensionAction", "Run Extension Action", "Erweiterungsaktion ausführen"),
        ("evaluate", "Evaluate", "Auswerten"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("selectNode", "Select Node", "Knoten auswählen"),
        ("nodeGraphSelect", "Node Graph Select", "Knotengraph auswählen"),
        ("nodeGraphHover", "Node Graph Hover", "Knotengraph-Hover"),
        ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
        ("nodeGraphViewport", "Node Graph Viewport", "Knotengraph-Ansicht"),
        ("setLodMode", "Set LOD Mode", "LOD-Modus festlegen"),
        ("setProximityDistance", "Set Proximity Distance", "Näheabstand festlegen"),
        ("setGridVisible", "Set Grid Visible", "Raster sichtbar"),
        ("setGridSnapEnabled", "Set Grid Snap Enabled", "Rasterfang aktivieren"),
        ("setGridFactor", "Set Grid Factor", "Rasterfaktor festlegen"),
        ("selectAll", "Select All", "Alles auswählen"),
        ("focusSelection", "Zoom to Selection", "Auf Auswahl zoomen"),
        ("clearSelection", "Clear Selection", "Auswahl aufheben"),
        ("setCatalogueSections", "Set Catalogue Sections", "Katalogabschnitte festlegen"),
        ("toggleExtension", "Toggle Extension", "Erweiterung umschalten"),
        ("addGeneration", "Add Generation", "Generation hinzufügen"),
        ("removeGeneration", "Remove Generation", "Generation entfernen"),
        ("selectGeneration", "Select Generation", "Generation auswählen"),
        ("renameGeneration", "Rename Generation", "Generation umbenennen"),
        ("updateGenerationValues", "Update Generation Values", "Generationswerte aktualisieren"),
    ];
    localized_label_map(is_de, ENTRIES)
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
fn build_document_tree(fixture: &FlowFixture, selected: &[String], labels: &FlowPlayLabels) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| tree_item_with_action(format!("flow-play-document.widget.{}", widget_id(widget)), widget_tree_label(widget), Some(widget_kind_label(widget).into()), flow_action("setSelection", Some(json!({ "ids": [widget_id(widget)] })))))
        .collect();
    let synapse_items: Vec<UiTreeItemNode> =
        fixture.synapses.iter().map(|synapse| tree_item_desc(format!("flow-play-document.synapse.{}", synapse.id), format!("{} → {}", synapse.from, synapse.to), Some(format!("{} → {}", synapse.from_port, synapse.to_port)))).collect();
    PanelTreeBuilder::new("flow-play-document")
        .section_or_placeholder("flow-play-document.widgets", Some(labels.widgets.into()), true, widget_items, "(none)")
        .section_or_placeholder("flow-play-document.synapses", Some(labels.synapses.into()), false, synapse_items, "(none)")
        .selected(selected.iter().map(|id| format!("flow-play-document.widget.{id}")).collect())
        .build()
}

fn build_catalogue_tree(fixture: &FlowFixture, runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_fixture(fixture, runtime);
    let sections: Vec<Value> = host.catalogue_json().ok().and_then(|raw| serde_json::from_str(&raw).ok()).unwrap_or_default();
    let tree_sections: Vec<UiTreeSectionNode> = sections
        .iter()
        .filter_map(|section| {
            let id = section.get("id")?.as_str()?.to_string();
            let title = section.get("title").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
            let items: Vec<UiTreeItemNode> = section
                .get("items")
                .and_then(|value| value.as_array())
                .map(|entries| {
                    entries
                        .iter()
                        .filter_map(|entry| {
                            let kind = entry.get("kind")?.as_str()?;
                            let label = entry.get("name").or_else(|| entry.get("abbreviation")).and_then(|value| value.as_str()).unwrap_or(kind);
                            let descriptor = if kind == "neuron" { flow_widget_descriptor("neuron", entry.get("neuronKind").and_then(|value| value.as_str())) } else { flow_widget_descriptor(kind, None) };
                            let action = flow_action("addWidget", Some(descriptor.clone()));
                            Some(tree_item_with_action_draggable(format!("flow-play-catalogue.{id}.{kind}.{label}"), label, Some(kind.to_string()), action, &flow_widget_drag_json(&descriptor)))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Some(UiTreeSectionNode { presence: UiPresence::default(), id: format!("flow-play-catalogue.{id}"), label: Some(title), default_open: Some(true), items,
        })
        })
        .collect();
    let tree_sections = if tree_sections.is_empty() { catalogue_tree_sections_fallback(labels) } else { tree_sections };
    let mut builder = PanelTreeBuilder::new("flow-play-catalogue");
    for section in tree_sections.into_iter().chain(flow_extensions_tree_sections(runtime, labels)) {
        builder = builder.section(section.id, section.label, section.default_open.unwrap_or(false), section.items);
    }
    builder.selected(vec![]).build()
}

/// 🧩️ Installed/enabled extension palette plus actions surfaced by active extensions.
fn flow_extensions_tree_sections(runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> Vec<UiTreeSectionNode> {
    let installed: Vec<UiTreeItemNode> = FLOW_EXTENSIONS
        .iter()
        .map(|(id, name, _, _, _)| {
            let enabled = runtime.extension_enabled.get(*id).copied().unwrap_or(false);
            tree_item_with_action(
                format!("flow-play-extensions.{id}"),
                flow_extension_label(id, name, labels),
                Some(if enabled { "enabled".into() } else { "disabled".into() }),
                flow_action("toggleExtension", Some(json!({ "id": id, "enabled": !enabled }))),
            )
        })
        .collect();
    let actions: Vec<UiTreeItemNode> = FLOW_EXTENSIONS
        .iter()
        .filter(|(id, ..)| runtime.extension_enabled.get(*id).copied().unwrap_or(false))
        .map(|(_, _, action_id, title, _)| {
            tree_item_with_action(format!("flow-play-extensions.action.{action_id}"), flow_extension_action_title_label(action_id, title, labels), Some((*action_id).into()), flow_action("runExtensionAction", Some(json!({ "actionId": action_id }))))
        })
        .collect();
    let mut sections = vec![UiTreeSectionNode { presence: UiPresence::default(), id: "flow-play-extensions.installed".into(), label: Some(labels.extensions.into()), default_open: Some(false), items: installed,
        }];
    if !actions.is_empty() {
        sections.push(UiTreeSectionNode { presence: UiPresence::default(), id: "flow-play-extensions.actions".into(), label: Some(labels.extension_actions.into()), default_open: Some(false), items: actions,
        });
    }
    sections
}

fn catalogue_tree_sections_fallback(labels: &FlowPlayLabels) -> Vec<UiTreeSectionNode> {
    let sources = [("inputSlider", labels.catalogue_slider), ("inputNote", labels.catalogue_note)];
    let components = [("math.add", labels.catalogue_add), ("logic.and", labels.catalogue_and), ("text.concat", labels.catalogue_concat)];
    let sinks = [("outputPreview", labels.catalogue_preview), ("outputExport", labels.catalogue_export)];
    vec![
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-catalogue.sources".into(),
            label: Some(labels.sources.into()),
            default_open: Some(true),
            items: sources
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor(kind, None);
                    tree_item_with_action_draggable(format!("flow-play-catalogue.source.{kind}"), *label, Some((*kind).into()), flow_action("addWidget", Some(descriptor.clone())), &flow_widget_drag_json(&descriptor))
                })
                .collect(),
            },
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-catalogue.components".into(),
            label: Some(labels.components.into()),
            default_open: Some(true),
            items: components
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor("neuron", Some(kind));
                    tree_item_with_action_draggable(format!("flow-play-catalogue.component.{kind}"), *label, Some((*kind).into()), flow_action("addWidget", Some(descriptor.clone())), &flow_widget_drag_json(&descriptor))
                })
                .collect(),
            },
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-catalogue.sinks".into(),
            label: Some(labels.sinks.into()),
            default_open: Some(false),
            items: sinks
                .iter()
                .map(|(kind, label)| {
                    let descriptor = flow_widget_descriptor(kind, None);
                    tree_item_with_action_draggable(format!("flow-play-catalogue.sink.{kind}"), *label, Some((*kind).into()), flow_action("addWidget", Some(descriptor.clone())), &flow_widget_drag_json(&descriptor))
                })
                .collect(),
            },
    ]
}

//#region 🔖️WindowMeasures
fn window_instance_ids(view_state: &ViewState, kind_id: &str) -> Vec<String> {
    let ids: Vec<String> = view_state.window_instances.iter().filter(|instance| instance.window_kind_id == kind_id).map(|instance| instance.id.clone()).collect();
    if ids.is_empty() {
        vec![kind_id.to_string()]
    } else {
        ids
    }
}

fn flow_lod_measure(runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: FLOW_LOD_MODE_AUTOMATIC.into(), value: FLOW_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
        let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
        let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select {
        id: "flow-play-measures.lod".into(),
        label: Some(labels.lod_mode.into()),
        value: runtime.lod_mode.clone(),
        items,
        on_change: flow_action("setLodMode", Some(json!({ "value": runtime.lod_mode }))),
    }
}

fn flow_grid_measures_group(runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: "flow-play-measures.grid".into(),
        label: labels.grid.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle {
                id: "flow-play-measures.grid-visible".into(),
                icon_id: "layout-grid".into(),
                label: Some(labels.grid_visible.into()),
                pressed: runtime.grid_visible,
                text: None,
                on_change: flow_action("setGridVisible", None),
            },
            WindowMeasure::Toggle {
                id: "flow-play-measures.grid-snap".into(),
                icon_id: "magnet".into(),
                label: Some(labels.grid_snap.into()),
                pressed: runtime.grid_snap_enabled,
                text: None,
                on_change: flow_action("setGridSnapEnabled", None),
            },
            WindowMeasure::Slider {
                id: "flow-play-measures.grid-factor".into(),
                label: Some(format!("{} {:.1}", labels.grid_factor, runtime.grid_factor)),
                value: runtime.grid_factor,
                min: 0.5,
                max: 50.0,
                step: Some(0.5),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: flow_action("setGridFactor", None),
            },
        ],
    }
}

fn flow_window_measures(runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> Vec<WindowMeasure> {
    vec![
        flow_lod_measure(runtime, labels),
        WindowMeasure::Slider {
            id: "flow-play-measures.proximity".into(),
            label: Some(labels.proximity_distance.into()),
            value: runtime.proximity_distance,
            min: 0.0,
            max: 240.0,
            step: Some(4.0),
            ready: None,
            loading: None, waiting: None,
            disabled: None,
            reveal: None,
            on_change: flow_action("setProximityDistance", None),
        },
        flow_grid_measures_group(runtime, labels),
    ]
}
//#endregion 🔖️WindowMeasures

//#region 🔖️Actions
fn flow_internal_action(id: &str, label: &str, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}
fn focus_selection_camera(fixture: &FlowFixture, runtime: &FlowPlayRuntime) -> Option<CameraJson> {
    if runtime.selected_node_ids.is_empty() {
        return None;
    }
    let mut host = host_from_fixture(fixture, runtime);
    host.dag.set_viewport(1280, 800, 1.0);
    host.dag.set_selection(&runtime.selected_node_ids);
    host.focus_selection_camera(1.2)
}
//#endregion 🔖️Actions

fn build_inspector_tree(fixture: &FlowFixture, selected: &[String], _runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.no_selection)],
            menu: None,
        }]);
    }
    let widgets: Vec<&Widget> = selected.iter().filter_map(|id| fixture.widgets.iter().find(|widget| widget_id(widget) == id)).collect();
    if widgets.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-inspector.missing".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text("Widget not found")],
            menu: None,
        }]);
    }
    let widget_ids: Vec<String> = widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if widgets.iter().all(|widget| matches!(widget, Widget::InputSlider { .. })) {
        let mixed = ui_inspector_mixed_number(
            &widgets
                .iter()
                .map(|widget| match widget {
                    Widget::InputSlider { value, .. } => *value,
                    _ => 0.0,
                })
                .collect::<Vec<_>>(),
        );
        groups.push(UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "flow-play-inspector.kind.inputSlider".into(),
            label: "inputSlider".into(),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "flow-play-inspector.slider-value".into(),
                label: labels.value.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: "flow-play-inspector.slider-value.input".into(),
                    input_kind: "number".into(),
                    value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
                    placeholder: if mixed.uniform { None } else { Some(UI_INSPECTOR_MIXED_PLACEHOLDER.into()) },
                    commit: None,
                    on_change: flow_action("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "value" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            })],
        });
    }
    if widgets.iter().all(|widget| matches!(widget, Widget::InputNote { .. })) {
        let mixed = ui_inspector_mixed_text(
            &widgets
                .iter()
                .map(|widget| match widget {
                    Widget::InputNote { text, .. } => text.clone(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>(),
        );
        groups.push(UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "flow-play-inspector.kind.inputNote".into(),
            label: "inputNote".into(),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "flow-play-inspector.note-text".into(),
                label: labels.text.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: "flow-play-inspector.note-text.input".into(),
                    input_kind: "text".into(),
                    value: mixed.value,
                    placeholder: mixed.placeholder,
                    commit: Some("blur".into()),
                    on_change: flow_action("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "text" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            })],
        });
    }
    let kind_mixed = ui_inspector_mixed_text(&widgets.iter().map(|widget| widget_kind_label(widget).to_string()).collect::<Vec<_>>());
    let mut base_fields = vec![ui_inspector_readonly_field("flow-play-inspector.kind", labels.kind, if kind_mixed.placeholder.is_none() { widget_kind_label(widgets[0]).to_string() } else { "—".into() })];
    if widget_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {presence: UiPresence::default(),
                id: "flow-play-inspector.id".into(),
                label: labels.id.into(),
                child: Box::new(UiNode::Input(UiInputNode {presence: UiPresence::default(),
                    id: "flow-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: widget_ids[0].clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: flow_action("renameFlowWidget", Some(json!({ "oldId": widget_ids[0] }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        );
    }
    groups.push(UiInspectorFieldGroup { presence: UiPresence::default(), id: "flow-play-inspector.base".into(), label: labels.widget.into(), default_open: None, fields: base_fields });
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_main_graph(fixture: &FlowFixture, runtime: &FlowPlayRuntime, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_fixture(fixture, runtime);
    let (nodes_json, edges_json) = fixture_to_workflow(&host.dag.fixture);
    let viewport_json = serde_json::to_string(&runtime.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
    let fixture_json = serde_json::to_string(fixture).ok();
    let selection_json = if runtime.selected_node_ids.is_empty() { None } else { serde_json::to_string(&runtime.selected_node_ids).ok() };
    let flow_extras = flow_backed_node_graph_extras(
        fixture,
        &runtime.lod_mode,
        runtime.proximity_distance,
        runtime.grid_visible,
        runtime.grid_snap_enabled,
        runtime.grid_factor,
        Some(&runtime.eval_driver),
    );
    let preview_off_json = if runtime.preview_off_node_ids.is_empty() {
        None
    } else {
        serde_json::to_string(&runtime.preview_off_node_ids).ok()
    };
    build_node_graph_scene(
        FLOW_PLAY_SURFACE_MAIN,
        FLOW_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators_json: flow_extras.operators_json,
            find_items_json: None,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json.or(fixture_json),
            eval_json: flow_extras.eval_json,
            computing_json: flow_extras.computing_json,
            selection_json,
            preview_off_json,
            ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
        },
    )
}

fn render_compiled_dag(fixture: &FlowFixture, runtime: &FlowPlayRuntime) -> UiNode {
    let host = host_from_fixture(fixture, runtime);
    build_text_editor_scene(FLOW_PLAY_SURFACE_COMPILED, FLOW_PLAY_APP_ID, TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None))
}

fn evaluate_generation_preview(fixture: &FlowFixture, runtime: &FlowPlayRuntime, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    seed_host_catalogue(&mut host, &runtime.catalogue_sections_json);
    host.evaluate().unwrap_or_default()
}

/// 👁️ Re-evaluates the selected generation into the runtime preview text (ephemeral view state — never
/// a document operation).
fn refresh_generation_preview(fixture: &FlowFixture, runtime: &mut FlowPlayRuntime) {
    let Some(generation) = selected_generation(&runtime.generation) else {
        runtime.generation.preview_text = None;
        return;
    };
    let preview = evaluate_generation_preview(fixture, runtime, &generation.values.clone());
    runtime.generation.preview_text = Some(preview.clone());
    runtime.eval_driver.set_eval_json(preview);
}

fn render_generate_generations(runtime: &FlowPlayRuntime) -> UiNode {
    render_generations_tree(FLOW_PLAY_APP_ID, "flow-play-generate", &runtime.generation.generations, runtime.generation.selected_generation_id.as_deref())
}

fn render_generate_form(fixture: &FlowFixture, runtime: &FlowPlayRuntime) -> UiNode {
    let spec = flow_fixture_to_form_spec(fixture);
    let Some(generation) = selected_generation(&runtime.generation) else {
        return ui_text("Add a generation to edit input values.");
    };
    render_generation_form_body(&spec, &generation.values, FLOW_PLAY_APP_ID, "updateGenerationValues", &generation.id)
}

fn render_generate_preview(runtime: &FlowPlayRuntime) -> UiNode {
    let text = runtime.generation.preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or("(evaluate a generation to preview output)");
    render_generation_preview_text(FLOW_PLAY_SURFACE_GENERATE_PREVIEW, FLOW_PLAY_APP_ID, text)
}
//#endregion 🔖️Render

//#region 🔖️FlowPlayApp
use std::cell::RefCell;

pub struct FlowPlayApp {
    runtime: RefCell<FlowPlayRuntime>,
}

impl Default for FlowPlayApp {
    fn default() -> Self {
        Self { runtime: RefCell::new(FlowPlayRuntime::default()) }
    }
}

impl FlowPlayApp {
    /// 👁️ Parses the many selection-arg shapes (`ids`/`nodeIds` arrays or a single `nodeId`) into ids.
    fn parse_selection(args: Option<&Value>) -> Vec<String> {
        args.and_then(|value| value.get("ids").or_else(|| value.get("nodeIds")))
            .and_then(|value| if value.is_array() { serde_json::from_value(value.clone()).ok() } else { value.as_str().map(|id| vec![id.to_string()]) })
            .or_else(|| args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(|id| vec![id.to_string()]))
            .unwrap_or_default()
    }

    /// ✏️ Renames a widget id (rewiring synapses and layout) purely in the fixture; `None` if the target
    /// id is blank, unchanged, or already taken.
    fn renamed_fixture(fixture: &FlowFixture, old_id: &str, new_id: &str) -> Option<FlowFixture> {
        let trimmed = new_id.trim();
        if trimmed.is_empty() || trimmed == old_id || fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
            return None;
        }
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
            if widget_id(widget) == old_id {
                match widget {
                    Widget::Neuron { id, .. }
                    | Widget::InputSlider { id, .. }
                    | Widget::InputNote { id, .. }
                    | Widget::InputImage { id, .. }
                    | Widget::Variable { id, .. }
                    | Widget::OutputPreview { id, .. }
                    | Widget::OutputAction { id, .. }
                    | Widget::OutputExport { id, .. }
                    | Widget::Cluster { id, .. } => *id = trimmed.to_string(),
                }
            }
        }
        for synapse in next.synapses.iter_mut() {
            if synapse.from == old_id {
                synapse.from = trimmed.into();
            }
            if synapse.to == old_id {
                synapse.to = trimmed.into();
            }
        }
        if let Some(layout) = next.layout.remove(old_id) {
            next.layout.insert(trimmed.into(), layout);
        }
        Some(next)
    }

    /// ✏️ Patches slider values / note text on the selected widgets in the fixture, returning the clone.
    fn patched_widgets_fixture(fixture: &FlowFixture, widget_ids: &[String], field: &str, raw_value: Option<&Value>) -> FlowFixture {
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
            if !widget_ids.iter().any(|id| id == widget_id(widget)) {
                continue;
            }
            match (field, widget) {
                ("value", Widget::InputSlider { value, .. }) => {
                    if let Some(v) = raw_value.and_then(|value| value.as_f64()) {
                        *value = v;
                    }
                }
                ("text", Widget::InputNote { text, .. }) => {
                    if let Some(v) = raw_value.and_then(|value| value.as_str()) {
                        *text = v.into();
                    }
                }
                _ => {}
            }
        }
        next
    }
}

impl DocumentApp for FlowPlayApp {
    type Projection = FlowFixture;
    type Operation = FlowOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        FLOW_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> FlowFixture {
        FlowFixture::default()
    }

    fn handle_action(&self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, FlowFixture>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, _view_state: &ViewState) -> ActionEmit<FlowOperation> {
        let fixture = doc.projection;
        let mut runtime = self.runtime.borrow_mut();
        match action {
            // 👁️ View/config actions — mutate runtime, emit no operations (never pollute undo).
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                runtime.selected_node_ids = Self::parse_selection(args);
                runtime.selected_edge_ids = args
                    .and_then(|value| value.get("edgeIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                runtime.selected_handle_ids = args
                    .and_then(|value| value.get("handleIds"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                ActionEmit::default()
            }
            "nodeGraphHover" => ActionEmit::default(),
            "graphPointerDown" => {
                runtime.selected_node_ids.clear();
                ActionEmit::default()
            }
            "nodeGraphViewport" => {
                if let Some(viewport_json) = args.and_then(|value| value.get("viewportJson")).and_then(|value| value.as_str()) {
                    if let Ok(camera) = serde_json::from_str::<CameraJson>(viewport_json) {
                        runtime.camera = camera;
                    }
                }
                ActionEmit::default()
            }
            // 🧵️ Arms the off-main-thread `flowEvalTick` chain (see `FlowEvalDriver`) instead of
            // evaluating synchronously; `pending_effects` keeps it going after every subsequent
            // mutation only while the "auto-evaluate" extension is on, but this explicit action
            // always kicks at least one run to completion regardless of that toggle.
            "evaluate" => {
                let host = host_from_fixture(fixture, &*runtime);
                if runtime.eval_driver.sync(&host) {
                    return ActionEmit { effects: vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..ActionEmit::default() };
                }
                ActionEmit::default()
            }
            "flowEvalTick" => {
                let mut host = host_from_fixture(fixture, &*runtime);
                let more = runtime.eval_driver.tick(&mut host);
                ActionEmit {
                    effects: if more { vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() },
                    ..ActionEmit::default()
                }
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
                    if mode == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(mode).is_some() {
                        runtime.lod_mode = mode.into();
                    }
                }
                ActionEmit::default()
            }
            "setProximityDistance" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    runtime.proximity_distance = value.max(0.0);
                }
                ActionEmit::default()
            }
            "setGridVisible" => {
                let pressed = args.and_then(|value| value.get("pressed").or_else(|| value.get("enabled"))).and_then(|value| value.as_bool());
                runtime.grid_visible = pressed.unwrap_or(!runtime.grid_visible);
                ActionEmit::default()
            }
            "setGridSnapEnabled" => {
                let pressed = args.and_then(|value| value.get("pressed").or_else(|| value.get("enabled"))).and_then(|value| value.as_bool());
                runtime.grid_snap_enabled = pressed.unwrap_or(!runtime.grid_snap_enabled);
                ActionEmit::default()
            }
            "setGridFactor" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    runtime.grid_factor = value.clamp(0.5, 50.0);
                }
                ActionEmit::default()
            }
            "selectAll" => {
                runtime.selected_node_ids = fixture.widgets.iter().map(widget_id).map(str::to_string).collect();
                ActionEmit::default()
            }
            "clearSelection" => {
                runtime.selected_node_ids.clear();
                runtime.selected_edge_ids.clear();
                runtime.selected_handle_ids.clear();
                ActionEmit::default()
            }
            "contextMenuAt" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                if let Some(id) = id {
                    if !id.is_empty() {
                        runtime.selected_node_ids = vec![id.to_string()];
                    }
                }
                ActionEmit::default()
            }
            "setPreviewOff" => {
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_else(|| runtime.selected_node_ids.clone());
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_bool()).unwrap_or(true);
                if value {
                    for id in ids {
                        if !runtime.preview_off_node_ids.contains(&id) {
                            runtime.preview_off_node_ids.push(id);
                        }
                    }
                } else {
                    runtime.preview_off_node_ids.retain(|id| !ids.contains(id));
                }
                ActionEmit::default()
            }
            "openSpotlight" => ActionEmit::default(),
            "replaceImage" => ActionEmit::default(),
            "focusSelection" => {
                if let Some(camera) = focus_selection_camera(fixture, &*runtime) {
                    runtime.camera = camera;
                }
                ActionEmit::default()
            }
            "setCatalogueSections" => {
                if let Some(sections) = args.and_then(|value| value.get("sections")) {
                    runtime.catalogue_sections_json = sections.to_string();
                }
                ActionEmit::default()
            }
            "toggleExtension" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool());
                if let (Some(id), Some(enabled)) = (id, enabled) {
                    runtime.extension_enabled.insert(id.into(), enabled);
                }
                ActionEmit::default()
            }
            "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                let spec = flow_fixture_to_form_spec(fixture);
                let mut generation = runtime.generation.clone();
                if handle_generation_action(action, args, &mut generation, &spec, FLOW_PLAY_APP_ID) {
                    runtime.generation = generation;
                    if matches!(action, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
                        refresh_generation_preview(fixture, &mut *runtime);
                    }
                }
                ActionEmit::default()
            }
            // ✏️ Operation actions — run the stateful `FlowHost` mutation, diff into granular operations.
            "addWidget" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => {
                        let neuron_kind = args.and_then(|value| value.get("neuronKind")).and_then(|value| value.as_str()).unwrap_or("math.add");
                        json!({ "kind": "neuron", "neuronKind": neuron_kind }).to_string()
                    }
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let mut new_id = None;
                let operations = host_operations(fixture, &*runtime, |host| match host.add_widget(&descriptor, x, y) {
                    Ok(id) => {
                        new_id = Some(id);
                        true
                    }
                    Err(_) => false,
                });
                if let Some(id) = new_id {
                    runtime.selected_node_ids = vec![id];
                }
                ActionEmit::operations(operations)
            }
            "removeWidget" => {
                let widget_id = args.and_then(|value| value.get("widgetId")).or_else(|| args.and_then(|value| value.get("id"))).and_then(|value| value.as_str()).map(str::to_string);
                let Some(widget_id) = widget_id else {
                    return ActionEmit::default();
                };
                let operations = host_operations(fixture, &*runtime, |host| host.remove_widget(&widget_id).is_ok());
                if !operations.is_empty() {
                    runtime.selected_node_ids.retain(|id| id != &widget_id);
                }
                ActionEmit::operations(operations)
            }
            "deleteSelection" => {
                let nodes = runtime.selected_node_ids.clone();
                let edges = runtime.selected_edge_ids.clone();
                let handles = runtime.selected_handle_ids.clone();
                let operations = host_operations(fixture, &*runtime, |host| {
                    sync_host_selection_domains(host, &nodes, &edges, &handles);
                    if !host.has_selection() {
                        return false;
                    }
                    host.delete_selection().is_ok()
                });
                if !operations.is_empty() {
                    runtime.selected_node_ids.clear();
                    runtime.selected_edge_ids.clear();
                    runtime.selected_handle_ids.clear();
                }
                ActionEmit::operations(operations)
            }
            "disconnect" => {
                let synapse_id = args.and_then(|value| value.get("synapseId")).or_else(|| args.and_then(|value| value.get("edgeId"))).and_then(|value| value.as_str()).map(str::to_string);
                let Some(synapse_id) = synapse_id else {
                    return ActionEmit::default();
                };
                ActionEmit::operations(host_operations(fixture, &*runtime, |host| host.disconnect(&synapse_id).is_ok()))
            }
            "connectMediaPorts" => {
                let from = args.and_then(|value| value.get("sourceNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let from_port = args.and_then(|value| value.get("sourcePortId")).and_then(|value| value.as_str()).map(str::to_string);
                let to = args.and_then(|value| value.get("targetNodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let to_port = args.and_then(|value| value.get("targetPortId")).and_then(|value| value.as_str()).map(str::to_string);
                let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) else {
                    return ActionEmit::default();
                };
                ActionEmit::operations(host_operations(fixture, &*runtime, |host| host.connect_ports(&from, &from_port, &to, &to_port).is_ok()))
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) else {
                    return ActionEmit::default();
                };
                let operations = host_operations(fixture, &*runtime, |host| {
                    host.begin_change();
                    host.move_widget(&node_id, x, y).is_ok()
                });
                if operations.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::amend(operations, format!("move-{node_id}"))
            }
            "reorganize" => ActionEmit::operations(host_operations(fixture, &*runtime, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
            "patchFlowWidgets" => {
                let widget_ids: Vec<String> = args.and_then(|value| value.get("widgetIds")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let raw_value = args.and_then(|value| value.get("value")).cloned();
                let next = Self::patched_widgets_fixture(fixture, &widget_ids, &field, raw_value.as_ref());
                let operations = flow_fixture_operations(fixture, &next);
                if operations.is_empty() {
                    return ActionEmit::default();
                }
                ActionEmit::amend(operations, format!("patch-{field}-{}", widget_ids.join(",")))
            }
            "renameFlowWidget" => {
                let old_id = args.and_then(|value| value.get("oldId")).and_then(|value| value.as_str());
                let new_id = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
                let (Some(old_id), Some(new_id)) = (old_id, new_id) else {
                    return ActionEmit::default();
                };
                let Some(next) = Self::renamed_fixture(fixture, old_id, new_id) else {
                    return ActionEmit::default();
                };
                runtime.selected_node_ids = vec![new_id.trim().into()];
                ActionEmit::operations(flow_fixture_operations(fixture, &next))
            }
            "nodeGraphEdit" | "spotlightCommit" => {
                let raw_operations = args.and_then(|value| value.get("operations")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let selected = runtime.selected_node_ids.clone();
                let mut clear_selection = false;
                let operations = host_operations(fixture, &*runtime, |host| {
                    let mut changed = false;
                    for operation in &raw_operations {
                        match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                            "setFixture" => {
                                if let Some(fixture_json) = operation.get("fixtureJson").and_then(|value| value.as_str()) {
                                    if let Ok(parsed) = serde_json::from_str::<FlowFixture>(fixture_json) {
                                        host.begin_change();
                                        host.set_fixture_preserving_history(parsed);
                                        changed = true;
                                    }
                                }
                            }
                            "deleteSelection" => {
                                sync_host_selection(host, &selected);
                                if host.delete_selection().is_ok() {
                                    clear_selection = true;
                                    changed = true;
                                }
                            }
                            "connect" => {
                                let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                                let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                                let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                                let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                                if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                    if host.connect_ports(from, from_port, to, to_port).is_ok() {
                                        changed = true;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    changed
                });
                if clear_selection {
                    runtime.selected_node_ids.clear();
                }
                ActionEmit::operations(operations)
            }
            "runExtensionAction" => {
                let action_id = args.and_then(|value| value.get("actionId")).and_then(|value| value.as_str());
                let Some(action_id) = action_id else {
                    return ActionEmit::default();
                };
                let entry = FLOW_EXTENSIONS.iter().find(|(_, _, entry_action_id, ..)| *entry_action_id == action_id);
                let Some((id, _, _, _, effect)) = entry else {
                    return ActionEmit::default();
                };
                if !runtime.extension_enabled.get(*id).copied().unwrap_or(false) {
                    return ActionEmit::default();
                }
                match *effect {
                    "reorganize" => ActionEmit::operations(host_operations(fixture, &*runtime, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
                    "evaluate" => {
                        let host = host_from_fixture(fixture, &*runtime);
                        if runtime.eval_driver.sync(&host) {
                            return ActionEmit { effects: vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..ActionEmit::default() };
                        }
                        ActionEmit::default()
                    }
                    _ => ActionEmit::default(),
                }
            }
            _ => ActionEmit::default(),
        }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, example load, remote operations) in one place.
    fn pending_effects(&self, doc: &DocumentView<'_, FlowFixture>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, _view_state: &ViewState) -> Vec<HostEffect> {
        let mut runtime = self.runtime.borrow_mut();
        let host = host_from_fixture(doc.projection, &*runtime);
        if runtime.eval_driver.sync(&host) {
            vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, FlowFixture>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, view_state: &ViewState) -> UiNode {
        let fixture = doc.projection;
        let labels = resolve_labels::<FlowPlayLabels>(view_state);
        let runtime = self.runtime.borrow();
        match body_key {
            FLOW_PLAY_BODY_MAIN => render_main_graph(fixture, &runtime, labels),
            FLOW_PLAY_BODY_COMPILED => render_compiled_dag(fixture, &runtime),
            FLOW_PLAY_BODY_GENERATIONS => render_generate_generations(&runtime),
            FLOW_PLAY_BODY_GENERATE_FORM => render_generate_form(fixture, &runtime),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&runtime),
            FLOW_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &runtime.selected_node_ids, labels),
            FLOW_PLAY_BODY_CATALOGUE => build_catalogue_tree(fixture, &runtime, labels),
            FLOW_PLAY_BODY_INSPECTOR => build_inspector_tree(fixture, &runtime.selected_node_ids, &runtime, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> AppLabelsOverlay {
        let labels = resolve_labels::<FlowPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        AppLabelsOverlay::default()
            .window_kind_label(FLOW_PLAY_WINDOW_MAIN, labels.window_main)
            .window_kind_label(FLOW_PLAY_WINDOW_COMPILED, labels.window_compiled)
            .window_kind_label(FLOW_PLAY_WINDOW_GENERATIONS, labels.window_generations)
            .window_kind_label(FLOW_PLAY_WINDOW_GENERATE_FORM, labels.window_generate_form)
            .window_kind_label(FLOW_PLAY_WINDOW_GENERATE_PREVIEW, labels.window_generate_preview)
            .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
            .mode_label("generate", if is_de { "Generieren" } else { "Generate" })
            .action_labels(flow_action_labels(is_de))
            .example_labels(HashMap::from([("demo".to_string(), "Demo".to_string())]))
    }

    fn window_measures(&self, _doc: &DocumentView<'_, FlowFixture>, _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = resolve_labels::<FlowPlayLabels>(view_state);
        let runtime = self.runtime.borrow();
        window_instance_ids(view_state, FLOW_PLAY_WINDOW_MAIN)
            .into_iter()
            .map(|window_id| (window_id, flow_window_measures(&*runtime, labels)))
            .collect()
    }

    fn context_menu(
        &self,
        request: &ContextMenuRequest,
        doc: &DocumentView<'_, FlowFixture>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        let labels = resolve_labels::<FlowPlayLabels>(view_state);
        let is_de = is_de_locale(view_state);
        let runtime = self.runtime.borrow();
        flow_context_menu_items(registry, doc.projection, &runtime, labels, is_de, request.surface.as_ref())
    }
}
//#endregion 🔖️FlowPlayApp

//#region 🔖️Manifest
pub fn create_flow_app() -> App {
    App::from_builder(
        App::builder(FLOW_PLAY_APP_ID, "Flow").document(["semio", "flow"])
            .artifact_kind(ArtifactKindSpec {
                id: "computation.flow".into(),
                name: "Flow".into(),
                source_format: "flow.document".into(),
                component_kind: "flow".into(),
                dimension: "graph".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Flow },
                schema: "flow.document".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("flow")
            .mode("edit", "Edit", "square-pen")
            .mode("generate", "Generate", "sparkles")
            .default_mode_id("edit")
            .window_kind(FLOW_PLAY_WINDOW_MAIN, "Flow", FLOW_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "flow-graph")
            .window_kind(FLOW_PLAY_WINDOW_COMPILED, "DSL", FLOW_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph, "code")
            .window_kind(FLOW_PLAY_WINDOW_GENERATIONS, "Generations", FLOW_PLAY_BODY_GENERATIONS, SurfaceKind::Canvas2d, "sparkles")
            .window_kind(FLOW_PLAY_WINDOW_GENERATE_FORM, "Form", FLOW_PLAY_BODY_GENERATE_FORM, SurfaceKind::Canvas2d, "clipboard-list")
            .window_kind(
                FLOW_PLAY_WINDOW_GENERATE_PREVIEW,
                "Preview",
                FLOW_PLAY_BODY_GENERATE_PREVIEW,
                SurfaceKind::Canvas2d,
                "eye",
            )
            .default_layout(create_default_layout(
                &[FLOW_PLAY_WINDOW_MAIN.into(), FLOW_PLAY_WINDOW_COMPILED.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Flow".into(), "DSL".into()]),
            ))
            .named_layout(create_named_layout(
                "flow-generate",
                "Generate",
                create_default_layout(
                    &[
                        FLOW_PLAY_WINDOW_GENERATIONS.into(),
                        FLOW_PLAY_WINDOW_GENERATE_FORM.into(),
                        FLOW_PLAY_WINDOW_GENERATE_PREVIEW.into(),
                    ],
                    "row",
                    Some(&[22.0, 43.0, 35.0]),
                    Some(&["Generations".into(), "Form".into(), "Preview".into()]),
                ),
                "builtin",
                Some("sparkles".into()),
                None,
            ))
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                FLOW_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                FLOW_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                FLOW_PLAY_BODY_INSPECTOR,
            )
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .operation("addWidget", "Add Widget")
            .operation("removeWidget", "Remove Widget")
            .operation("deleteSelection", "Delete Selection")
            .operation("disconnect", "Disconnect")
            .operation("connectMediaPorts", "Connect Ports")
            .operation("moveMediaNode", "Move Node")
            .operation("reorganize", "Reorganize")
            .operation("patchFlowWidgets", "Patch Widgets")
            .operation("renameFlowWidget", "Rename Widget")
            .operation("nodeGraphEdit", "Node Graph Edit")
            .operation("spotlightCommit", "Spotlight Commit")
            // 🧩️ Dynamic extension-provided action — id resolved at runtime, kept out of the palette.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("runExtensionAction", "Run Extension Action", ActionKind::Operation) })
            // 👁️ Ephemeral view/config actions — mutate runtime, emit no operations.
            .view_action("evaluate", "Evaluate")
            .view_action("selectAll", "Select All")
            .view_action("focusSelection", "Zoom to Selection")
            .action_with(flow_internal_action("setSelection", "Set Selection", ActionKind::View))
            .action_with(flow_internal_action("selectNode", "Select Node", ActionKind::View))
            .action_with(flow_internal_action("nodeGraphSelect", "Node Graph Select", ActionKind::View))
            .action_with(flow_internal_action("nodeGraphHover", "Node Graph Hover", ActionKind::View))
            .action_with(flow_internal_action("graphPointerDown", "Graph Pointer Down", ActionKind::View))
            .action_with(flow_internal_action("nodeGraphViewport", "Node Graph Viewport", ActionKind::View))
            .action_with(flow_internal_action("setLodMode", "Set LOD Mode", ActionKind::View))
            .action_with(flow_internal_action("setProximityDistance", "Set Proximity Distance", ActionKind::View))
            .action_with(flow_internal_action("setGridVisible", "Set Grid Visible", ActionKind::View))
            .action_with(flow_internal_action("setGridSnapEnabled", "Set Grid Snap Enabled", ActionKind::View))
            .action_with(flow_internal_action("setGridFactor", "Set Grid Factor", ActionKind::View))
            .action_with(flow_internal_action("clearSelection", "Clear Selection", ActionKind::View))
            .action_with(flow_internal_action("contextMenuAt", "Context Menu At", ActionKind::View))
            .action_with(flow_internal_action("setPreviewOff", "Set Preview Off", ActionKind::View))
            .action_with(flow_internal_action("openSpotlight", "Open Spotlight", ActionKind::View))
            .action_with(flow_internal_action("replaceImage", "Replace Image", ActionKind::View))
            .action_with(flow_internal_action("setCatalogueSections", "Set Catalogue Sections", ActionKind::View))
            .action_with(flow_internal_action("toggleExtension", "Toggle Extension", ActionKind::View))
            .action_with(flow_internal_action("addGeneration", "Add Generation", ActionKind::View))
            .action_with(flow_internal_action("removeGeneration", "Remove Generation", ActionKind::View))
            .action_with(flow_internal_action("selectGeneration", "Select Generation", ActionKind::View))
            .action_with(flow_internal_action("renameGeneration", "Rename Generation", ActionKind::View))
            .action_with(flow_internal_action("updateGenerationValues", "Update Generation Values", ActionKind::View))
            // 📝️ Staged argument form for the panel-visible create action (module operators stay catalogue-driven).
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("inputSlider", "Slider"),
                    ActionArgOption::new("inputNote", "Note"),
                ]).default_value("inputSlider"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+a", "selectAll")
            .keybinding("delete,backspace", "deleteSelection"),
    )
    .example("demo", "Demo", serde_json::to_string(&FlowFixture::default()).expect("FlowFixture::default() has no non-finite floats or non-string map keys, so serialization cannot fail"), "flask-conical")
    .workflow("flow", "Flow", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use flow_core::FlowFixture;
    use flow_engine::FLOW_WIDGET_DRAG_MIME;
    use semio_framework_plugin::{
        testkit::{assert_undo_redo_round_trip, meta, new_app, new_app_with_registry, paired_apps},
        PluginApp, VcsDocumentApp,
    };

    fn render(app: &mut VcsDocumentApp<FlowPlayApp>, body_key: &str, view_state: &ViewState) -> String {
        serde_json::to_string(&app.render(body_key, None, view_state).expect("render")).unwrap()
    }

    fn context_menu_items(app: &mut VcsDocumentApp<FlowPlayApp>, view_state: &ViewState, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
        let request = semio_framework_plugin::ContextMenuRequest {
            menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
            surface,
            window_instance_id: None,
            point: None,
        };
        serde_json::to_value(app.context_menu(&request, view_state)).unwrap_or(Value::Null)
    }

    fn preview_off_ids(app: &mut VcsDocumentApp<FlowPlayApp>, view_state: &ViewState) -> Value {
        let rendered: Value = serde_json::from_str(&render(app, FLOW_PLAY_BODY_MAIN, view_state)).expect("render json");
        rendered
            .pointer("/nodeGraph/previewOffJson")
            .and_then(Value::as_str)
            .and_then(|raw| serde_json::from_str(raw).ok())
            .unwrap_or(Value::Null)
    }

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app::<FlowPlayApp>();
        assert!(render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default()).contains("node-graph"));
    }

    #[test]
    fn renders_compiled_wire_editor() {
        let mut app = new_app::<FlowPlayApp>();
        assert!(render(&mut app, FLOW_PLAY_BODY_COMPILED, &ViewState::default()).contains("text-editor"));
    }

    #[test]
    fn default_fixture_has_widgets() {
        assert!(!FlowFixture::default().widgets.is_empty());
    }

    #[test]
    fn document_lists_widgets() {
        let mut app = new_app::<FlowPlayApp>();
        assert!(render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState::default()).contains("flow-play-document.widgets"));
    }

    #[test]
    fn catalogue_lists_module_operators() {
        let mut app = new_app::<FlowPlayApp>();
        let json = render(&mut app, FLOW_PLAY_BODY_CATALOGUE, &ViewState::default());
        assert!(json.contains("flow-play-catalogue.math"), "expected math module section: {json}");
        assert!(json.contains("math.add"), "expected math.add operator: {json}");
    }

    #[test]
    fn catalogue_items_export_flow_widget_drag_payload() {
        let mut app = new_app::<FlowPlayApp>();
        let json = render(&mut app, FLOW_PLAY_BODY_CATALOGUE, &ViewState::default());
        assert!(json.contains(FLOW_WIDGET_DRAG_MIME), "missing drag mime: {json}");
        assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
    }

    #[test]
    fn add_widget_emits_ops_and_grows_the_document() {
        let mut app = new_app::<FlowPlayApp>();
        let before = app.projection().expect("projection").widgets.len();
        let result = app.handle_action("addWidget", Some(&json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 })), &ViewState::default(), &meta("local")).expect("addWidget");
        assert!(!result.operations.is_empty(), "addWidget must emit operations");
        assert_eq!(app.projection().expect("projection").widgets.len(), before + 1);
    }

    #[test]
    fn undo_restores_fixture_after_add_widget() {
        let mut app = new_app::<FlowPlayApp>();
        let before = app.projection().expect("projection").widgets.len();
        assert_undo_redo_round_trip(&mut app, "addWidget", Some(&json!({ "kind": "inputNote", "x": 40.0, "y": 40.0 })), |app| app.projection().expect("projection").widgets.len(), before, before + 1);
    }

    #[test]
    fn selection_is_view_state_and_emits_no_operations() {
        let mut app = new_app::<FlowPlayApp>();
        let result = app.handle_action("setSelection", Some(&json!({ "ids": ["slider"] })), &ViewState::default(), &meta("local")).expect("setSelection");
        assert!(result.operations.is_empty(), "selection must not produce document operations");
    }

    #[test]
    fn evaluate_updates_preview_state_without_operations() {
        let mut app = new_app::<FlowPlayApp>();
        let result = app.handle_action("evaluate", None, &ViewState::default(), &meta("local")).expect("evaluate");
        assert!(result.operations.is_empty(), "evaluate is a view action");
    }

    #[test]
    fn generate_mode_renders_three_surfaces() {
        let mut app = new_app::<FlowPlayApp>();
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATIONS, &ViewState::default()).contains("addGeneration"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_FORM, &ViewState::default()).contains("Add a generation"));
        assert!(render(&mut app, FLOW_PLAY_BODY_GENERATE_PREVIEW, &ViewState::default()).contains("text-editor"));
    }

    #[test]
    fn set_lod_mode_rejects_unknown_and_accepts_known() {
        let mut app = new_app::<FlowPlayApp>();
        app.handle_action("setLodMode", Some(&json!({ "mode": "bogus" })), &ViewState::default(), &meta("local")).expect("bogus");
        app.handle_action("setLodMode", Some(&json!({ "mode": "micro" })), &ViewState::default(), &meta("local")).expect("micro");
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(json.contains("\\\"forcedLabel\\\":\\\"micro\\\"") || json.contains("\"forcedLabel\":\"micro\""));
    }

    #[test]
    fn toggle_extension_and_run_action_reorganizes_fixture() {
        let mut app = new_app::<FlowPlayApp>();
        let before = app.projection().expect("projection").widgets.len();
        let ignored = app.handle_action("runExtensionAction", Some(&json!({ "actionId": "flow.extension.reorganize" })), &ViewState::default(), &meta("local")).expect("ignored");
        assert!(ignored.operations.is_empty(), "disabled extension action must be a no-operation");
        app.handle_action("toggleExtension", Some(&json!({ "id": "auto-layout", "enabled": true })), &ViewState::default(), &meta("local")).expect("toggle");
        app.handle_action("runExtensionAction", Some(&json!({ "actionId": "flow.extension.reorganize" })), &ViewState::default(), &meta("local")).expect("reorganize");
        assert_eq!(app.projection().expect("projection").widgets.len(), before, "reorganize keeps every widget");
    }

    #[test]
    fn flow_labels_resolve_native_english_and_german() {
        let mut app = new_app::<FlowPlayApp>();
        let english = render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState::default());
        assert!(english.contains("Widgets") && english.contains("Synapses"), "english labels: {english}");
        let german = render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState { locale: Some("de".into()), ..ViewState::default() });
        assert!(german.contains("Synapsen"), "german labels: {german}");
    }

    /// 🤝️ Definitional merge proof: two instances on one backbone make DISJOINT edits (one renames a
    /// widget, the other adds a widget); after exchanging operations both converge — impossible under
    /// whole-fixture `setDocument` snapshots, which would clobber one side.
    #[test]
    fn default_runtime_enables_proximity_distance() {
        let mut app = new_app::<FlowPlayApp>();
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(json.contains("proximityDistance") && !json.contains(r#""proximityDistance":0"#));
    }

    #[test]
    fn window_measures_surface_lod_proximity_and_grid() {
        let mut app = new_app::<FlowPlayApp>();
        let measures = app.window_measures(&ViewState::default());
        let window_measures = measures.get(FLOW_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(window_measures.len(), 3);
        assert!(window_measures.iter().any(|measure| matches!(measure, WindowMeasure::Slider { id, .. } if id == "flow-play-measures.proximity")));
        assert!(window_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "flow-play-measures.grid")));
    }

    #[test]
    fn select_all_and_focus_selection_update_scene() {
        let mut app = new_app::<FlowPlayApp>();
        app.handle_action("selectAll", None, &ViewState::default(), &meta("local")).expect("selectAll");
        let selected = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(selected.contains("slider"));
        let before = selected.clone();
        app.handle_action("focusSelection", None, &ViewState::default(), &meta("local")).expect("focusSelection");
        let after = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert_ne!(before, after);
    }

    #[test]
    fn set_proximity_distance_updates_scene_lod_json() {
        let mut app = new_app::<FlowPlayApp>();
        app.handle_action("setProximityDistance", Some(&json!({ "value": 96.0 })), &ViewState::default(), &meta("local")).expect("proximity");
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(json.contains("96"));
    }

    #[test]
    fn empty_inspector_no_longer_shows_canvas_settings() {
        let mut app = new_app::<FlowPlayApp>();
        let json = render(&mut app, FLOW_PLAY_BODY_INSPECTOR, &ViewState::default());
        assert!(!json.contains("flow-play-inspector.lod-mode"));
        assert!(json.contains("flow-play-inspector.empty"));
    }

    fn flow_app_with_registry() -> VcsDocumentApp<FlowPlayApp> {
        new_app_with_registry::<FlowPlayApp>(create_flow_app)
    }

    #[test]
    fn context_menu_includes_select_all_when_empty() {
        let mut app = flow_app_with_registry();
        let menu = context_menu_items(&mut app, &ViewState::default(), Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![],
            selection: vec![],
            text: None,
        }));
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
        app.handle_action("setSelection", Some(&json!({ "ids": ["slider"] })), &ViewState::default(), &meta("local")).expect("setSelection");
        let menu = context_menu_items(&mut app, &ViewState::default(), None).to_string();
        assert!(menu.contains("setPreviewOff"), "menu should expose preview toggle: {menu}");
        assert!(menu.contains("Hide preview") || menu.contains("eye-off"), "menu should offer hide preview: {menu}");
        assert!(menu.contains("focusSelection"), "menu should expose zoom to selection: {menu}");
        assert!(menu.contains(r#""checked":true"#), "preview checked when visible: {menu}");
        assert!(!menu.contains(r#""id":"toggle-preview""#) || !menu
            .split("\"id\":\"toggle-preview\"")
            .nth(1)
            .unwrap_or("")
            .split("\"id\":")
            .next()
            .unwrap_or("")
            .contains("\"disabled\":true"), "preview must be enabled with selection: {menu}");
        app.handle_action("setPreviewOff", Some(&json!({ "ids": ["slider"], "value": true })), &ViewState::default(), &meta("local")).expect("setPreviewOff");
        let after_menu = context_menu_items(&mut app, &ViewState::default(), None).to_string();
        let preview_off = preview_off_ids(&mut app, &ViewState::default());
        assert_eq!(preview_off, json!(["slider"]), "preview_off should land on scene: {preview_off}");
        assert!(after_menu.contains("Show preview") || after_menu.contains(r#""icon":"eye""#), "menu should offer show preview: {after_menu}");
    }

    #[test]
    fn context_menu_at_selects_target_and_enables_preview() {
        let mut app = flow_app_with_registry();
        let before = context_menu_items(&mut app, &ViewState::default(), None).to_string();
        assert!(!before.contains(r#""id":"delete-selection""#), "preview starts without delete: {before}");
        app.handle_action("contextMenuAt", Some(&json!({ "id": "slider" })), &ViewState::default(), &meta("local")).expect("contextMenuAt");
        let after = context_menu_items(&mut app, &ViewState::default(), None).to_string();
        assert!(after.contains("setPreviewOff"), "menu keeps preview: {after}");
        assert!(after.contains(r#""ids":["slider"]"#) || after.contains("slider"), "preview args target the clicked node: {after}");
        assert!(!after.split("\"id\":\"toggle-preview\"")
            .nth(1)
            .unwrap_or("")
            .contains("\"disabled\":true"), "preview enabled after contextMenuAt: {after}");
    }

    #[test]
    fn context_menu_annotates_mixed_selection_counts_and_omits_delete_without_selection() {
        let mut app = flow_app_with_registry();
        let empty = context_menu_items(&mut app, &ViewState::default(), Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![],
            selection: vec![],
            text: None,
        })).to_string();
        assert!(!empty.contains(r#""id":"delete-selection""#), "empty must omit delete: {empty}");

        app.handle_action(
            "setSelection",
            Some(&json!({
                "ids": ["n1", "n2", "n3", "n4", "n5", "n6", "n7", "n8"],
                "edgeIds": ["e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "e10", "e11", "e12", "e13"]
            })),
            &ViewState::default(),
            &meta("local"),
        ).expect("setSelection");
        let menu = context_menu_items(&mut app, &ViewState::default(), Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
            selection: vec![
                semio_framework_plugin::ContextMenuSelectionGroup {
                    domain: "node".into(),
                    ids: vec!["n1".into(), "n2".into(), "n3".into(), "n4".into(), "n5".into(), "n6".into(), "n7".into(), "n8".into()],
                },
                semio_framework_plugin::ContextMenuSelectionGroup {
                    domain: "edge".into(),
                    ids: (1..=13).map(|i| format!("e{i}")).collect(),
                },
            ],
            text: None,
        })).to_string();
        eprintln!("[DEBUG] mixed selection context menu: {menu}");
        assert!(menu.contains(r#""id":"delete-selection""#), "mixed selection must expose delete: {menu}");
        assert!(menu.contains("8 nodes and 13 edges"), "count phrase missing: {menu}");
        assert!(menu.contains("deleteSelection"), "delete action missing: {menu}");
    }

    #[test]
    fn context_menu_for_edge_hit_uses_runtime_edge_selection() {
        let mut app = flow_app_with_registry();
        app.handle_action(
            "setSelection",
            Some(&json!({ "ids": [], "edgeIds": ["syn-1"] })),
            &ViewState::default(),
            &meta("local"),
        ).expect("setSelection");
        let menu = context_menu_items(&mut app, &ViewState::default(), Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "main".into(),
            kind: "nodeGraph".into(),
            hits: vec![semio_framework_plugin::ContextMenuHit { domain: "edge".into(), id: "syn-1".into(), label: None }],
            selection: vec![],
            text: None,
        })).to_string();
        eprintln!("[DEBUG] edge selection context menu: {menu}");
        assert!(menu.contains(r#""id":"delete-selection""#), "edge selection must expose delete: {menu}");
        assert!(menu.contains("1 edge") || menu.contains("1 Kante"), "edge count phrase missing: {menu}");
    }

    #[test]
    fn host_from_fixture_deletes_edge_selected_by_synapse_domain() {
        let runtime = FlowPlayRuntime::default();
        let fixture = FlowFixture::default();
        let mut host = host_from_fixture(&fixture, &runtime);
        sync_host_selection_domains(&mut host, &[], &["s1".into()], &[]);
        eprintln!(
            "[DEBUG] host_from_fixture edge selection: has={} edge_ids={:?}",
            host.has_selection(),
            host.dag.selected_edge_ids()
        );
        assert!(host.has_selection(), "s1 must resolve through host_from_fixture edge map");
        host.delete_selection().expect("deleteSelection");
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
        eprintln!(
            "[DEBUG] host_from_fixture after delete: synapses={:?}",
            host.fixture.synapses.iter().map(|synapse| synapse.id.as_str()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn delete_selection_action_removes_selected_synapses() {
        let mut app = flow_app_with_registry();
        let before = app.projection().expect("projection").synapses.len();
        app.handle_action(
            "setSelection",
            Some(&json!({ "ids": [], "edgeIds": ["s1"] })),
            &ViewState::default(),
            &meta("local"),
        ).expect("setSelection");
        let result = app.handle_action("deleteSelection", None, &ViewState::default(), &meta("local")).expect("deleteSelection");
        eprintln!("[DEBUG] deleteSelection action ops_len={}", result.operations.len());
        let after = app.projection().expect("projection");
        eprintln!(
            "[DEBUG] deleteSelection action remaining={:?}",
            after.synapses.iter().map(|synapse| synapse.id.as_str()).collect::<Vec<_>>()
        );
        assert!(!result.operations.is_empty(), "deleteSelection must emit operations for an edge");
        assert!(!after.synapses.iter().any(|synapse| synapse.id == "s1"), "synapse s1 must be removed");
        assert_eq!(after.synapses.len(), before - 1);
    }




    #[test]
    fn two_instances_converge_on_disjoint_edits() {
        let (mut instance_a, mut instance_b) = paired_apps::<FlowPlayApp>("mem://flow-convergence");

        instance_a.handle_action("renameFlowWidget", Some(&json!({ "oldId": "slider", "value": "input" })), &ViewState::default(), &meta("actor-a")).expect("a renames slider");
        instance_b.handle_action("addWidget", Some(&json!({ "kind": "inputNote", "x": 10.0, "y": 10.0 })), &ViewState::default(), &meta("actor-b")).expect("b adds a note");

        // A neutral history action always dispatches through the store, which pumps inbound operations first.
        instance_a.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &ViewState::default(), &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        assert!(projection_a.widgets.iter().any(|widget| widget_id(widget) == "input"), "A keeps its rename");
        assert!(projection_a.widgets.iter().any(|widget| matches!(widget, Widget::InputNote { .. })), "A absorbs B's note");
        assert_eq!(projection_a.widgets.len(), projection_b.widgets.len(), "both instances converge to the same widget set");
    }
}
//#endregion 🧪️Tests
