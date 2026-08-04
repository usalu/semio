//! 🖥️ Flow app — DocumentApp impl, render, manifest (constitutional: ui).

use flow::{FlowFixture, FLOW_DOCUMENT_SCHEMA};
use flow_core::{
    dag::{dag_lod_scale_json, DagDrawLod},
    forms_bridge::{apply_generation_values_to_fixture, flow_fixture_to_form_spec},
    CameraJson, FlowHost, Widget, FLOW_LOD_MODE_AUTOMATIC,
};
use flow_core::{flow_backed_node_graph_extras, flow_fixture_operations};
use flow_engine::{fixture_to_workflow, flow_play_neural_cache, flow_widget_descriptor, flow_widget_drag_json, seed_host_catalogue, sync_host_selection, sync_host_selection_domains, widget_id, widget_kind_label, widget_tree_label, FlowConfig};
use flow_op::{FlowConfigOperation, FlowOperation};
use flow_protocol::{FlowCommand, FlowNodeGraphEditOp};
use playbook::{handle_generation_action, render_generation_form_body, render_generation_preview_text, render_generations_tree, selected_generation};
use semio_framework_plugin::{
        build_node_graph_scene, build_text_editor_scene, create_default_layout, create_named_layout, tree_item_desc, tree_item_with_action, tree_item_with_action_draggable, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppActionRegistry, AppLabels, ArtifactKindSpec, ConfigView,
    ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView, Emit, HostEffect, Label, Locale, LocalizedLabel, MeasureSelectItem, MediaClass, MediaForm, MediaType, NodeGraphScene, NodeGraphViewport, OsMediaCapability, PanelGroup,
    PanelTreeBuilder, SurfaceKind, Terminology, TextEditorScene, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, UiTreeSectionNode, WindowMeasure, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
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

/// 🧩️ Built-in flow extensions: (id, name, actionId, actionTitle, effect).
const FLOW_EXTENSIONS: &[(&str, &str, &str, &str, &str)] =
    &[("auto-layout", "Auto Layout", "flow.extension.reorganize", "Reorganize Canvas", "reorganize"), ("auto-evaluate", "Auto Evaluate", "flow.extension.evaluate", "Evaluate Fixture", "evaluate")];
//#endregion 🔖️Constants

//#region 🔖️Locale

//#endregion 🔖️Locale

//#region 🔖️DocumentHelpers
fn flow_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(FLOW_PLAY_APP_ID).action(action, args)
}

fn apply_canvas_options(host: &mut FlowHost, config: &FlowConfig) {
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

fn host_from_fixture(fixture: &FlowFixture, config: &FlowConfig) -> FlowHost {
    let mut host = FlowHost::from_fixture_with_cache(fixture.clone(), flow_play_neural_cache());
    host.set_neuron_kind_infos_json(&flow_core::flow_neuron_kind_infos_json());
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    config.eval_driver().install_baseline_into(&mut host);
    host
}

/// 🌉️ Runs a `FlowHost` mutation over the current document fixture and diffs the result into granular
/// `FlowOperation`s. `mutate` returns `true` if it changed the fixture; a non-mutating call yields no operations.
fn host_operations(fixture: &FlowFixture, config: &FlowConfig, mutate: impl FnOnce(&mut FlowHost) -> bool) -> Vec<FlowOperation> {
    let mut host = host_from_fixture(fixture, config);
    if !mutate(&mut host) {
        return Vec::new();
    }
    flow_fixture_operations(fixture, &host.fixture)
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

/// ✏️ Patches the slider value / note text on the selected widgets in the fixture, returning the clone.
/// `raw_value` is the typed `FlowCommand::PatchFlowWidgets.value` field verbatim (a plain `&str`, not a
/// `serde_json::Value` — mirrors `dag_engine::node_patch_for_field`'s "typed command carries the raw UI
/// input string directly" convention) — numeric fields parse it themselves.
fn patched_widgets_fixture(fixture: &FlowFixture, widget_ids: &[String], field: &str, raw_value: &str) -> FlowFixture {
    let mut next = fixture.clone();
    for widget in next.widgets.iter_mut() {
        if !widget_ids.iter().any(|id| id == widget_id(widget)) {
            continue;
        }
        match (field, widget) {
            ("value", Widget::InputSlider { value, .. }) => {
                if let Ok(parsed) = raw_value.parse::<f64>() {
                    *value = parsed;
                }
            }
            ("text", Widget::InputNote { text, .. }) => *text = raw_value.into(),
            _ => {}
        }
    }
    next
}

fn focus_selection_camera(fixture: &FlowFixture, config: &FlowConfig) -> Option<CameraJson> {
    if config.selected_node_ids.is_empty() {
        return None;
    }
    let mut host = host_from_fixture(fixture, config);
    host.dag.set_viewport(1280, 800, 1.0);
    host.dag.set_selection(&config.selected_node_ids);
    host.focus_selection_camera(1.2)
}

/// 🧵️ Probes/arms the off-main-thread `flowEvalTick` chain (see `flow_core::FlowEvalDriver::sync`) and
/// persists the driver's new state via `SetEvalDriver` — shared by `FlowCommand::Evaluate` and
/// `RunExtensionAction`'s "evaluate" effect (both were the same `handle_action` "evaluate" logic
/// pre-B1, reachable from two different action ids).
fn evaluate_result(fixture: &FlowFixture, config: &FlowConfig) -> Emit<FlowOperation, FlowConfigOperation> {
    let mut driver = config.eval_driver();
    let host = host_from_fixture(fixture, config);
    let needs_tick = driver.sync(&host);
    let config_operations = vec![FlowConfigOperation::SetEvalDriver { json: serde_json::to_string(&driver).unwrap_or_default() }];
    if needs_tick {
        Emit { config_operations, effects: vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }], ..Default::default() }
    } else {
        Emit { config_operations, ..Default::default() }
    }
}

fn evaluate_generation_preview(fixture: &FlowFixture, config: &FlowConfig, values: &serde_json::Map<String, Value>) -> String {
    let fixture_json = serde_json::to_string(fixture).unwrap_or_default();
    let patched = apply_generation_values_to_fixture(&fixture_json, values);
    let patched_fixture = FlowHost::parse_fixture_json(&patched).unwrap_or_else(|_| fixture.clone());
    let mut host = FlowHost::from_fixture(patched_fixture);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    host.evaluate().unwrap_or_default()
}

/// 🧬️ Shared body for the five Generate-mode commands (`AddGeneration`/`RemoveGeneration`/
/// `SelectGeneration`/`RenameGeneration`/`UpdateGenerationValues`) — mirrors the pre-B1 shared
/// `handle_action` match arm, now dispatched from five distinct typed commands (one per declared
/// action id). Bridges into `playbook::handle_generation_action`'s still-untyped `args: Option<&Value>`
/// CRUD surface (out of scope to convert — lives in the `playbook` kernel crate); flow keeps its
/// generations config-tracked rather than document-operation-backed (unlike the sibling
/// `procedural_3d`/`procedural_2d` apps) since flow's document model is out of scope for this
/// conversion — see `flow_engine::FlowConfig::generation_json`'s doc comment.
fn handle_generation(action_id: &str, args: Option<&Value>, fixture: &FlowFixture, config: &FlowConfig) -> Emit<FlowOperation, FlowConfigOperation> {
    let spec = flow_fixture_to_form_spec(fixture);
    let mut generation = config.generation();
    if !handle_generation_action(action_id, args, &mut generation, &spec, FLOW_PLAY_APP_ID) {
        return Ok(Emit::default();
    }
    let mut config_operations = Vec::new();
    if matches!(action_id, "addGeneration" | "selectGeneration" | "updateGenerationValues") {
        match selected_generation(&generation) {
            Some(active) => {
                let preview = evaluate_generation_preview(fixture, config, &active.values.clone());
                generation.preview_text = Some(preview.clone());
                let mut driver = config.eval_driver();
                driver.set_eval_json(preview);
                config_operations.push(FlowConfigOperation::SetEvalDriver { json: serde_json::to_string(&driver).unwrap_or_default() });
            }
            None => generation.preview_text = None,
        }
    }
    config_operations.insert(0, FlowConfigOperation::SetGeneration { json: serde_json::to_string(&generation).unwrap_or_default() });
    let coalesce_key = (action_id == "updateGenerationValues").then(|| "generation-values".to_string());
    Emit { config_operations, coalesce_key, ..Default::default() }
}

/// 🎯️ Batched `NodeGraphEdit`/`SpotlightCommit` sub-operation dispatch — mirrors
/// `dag_ui::DocumentApp::handle`'s `DagNodeGraphEditOp` handling. Shared since both commands carry the
/// exact same sub-op vocabulary (the pre-B1 `"nodeGraphEdit"`/`"spotlightCommit"` actions shared one
/// `handle_action` match arm). The `deleteSelection` sub-op only clears `selected_node_ids` on success
/// (leaves `selected_edge_ids`/`selected_handle_ids` untouched) — matches the pre-B1 behavior exactly,
/// distinct from the top-level `FlowCommand::DeleteSelection`, which clears all three.
fn node_graph_edit_result(fixture: &FlowFixture, config: &FlowConfig, operations: &[FlowNodeGraphEditOp]) -> Emit<FlowOperation, FlowConfigOperation> {
    let selected = config.selected_node_ids.clone();
    let mut clear_selection = false;
    let document_operations = host_operations(fixture, config, |host| {
        let mut changed = false;
        for sub_operation in operations {
            match sub_operation {
                FlowNodeGraphEditOp::SetFixture { fixture_json } => {
                    if let Ok(parsed) = serde_json::from_str::<FlowFixture>(fixture_json) {
                        host.begin_change();
                        host.set_fixture_preserving_history(parsed);
                        changed = true;
                    }
                }
                FlowNodeGraphEditOp::DeleteSelection => {
                    sync_host_selection(host, &selected);
                    if host.delete_selection().is_ok() {
                        clear_selection = true;
                        changed = true;
                    }
                }
                FlowNodeGraphEditOp::Connect { source_node_id, source_port_id, target_node_id, target_port_id } => {
                    if host.connect_ports(source_node_id, source_port_id, target_node_id, target_port_id).is_ok() {
                        changed = true;
                    }
                }
            }
        }
        changed
    });
    let config_operations = if clear_selection { vec![FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }] } else { Vec::new() };
    Emit { document_operations, config_operations, ..Default::default() }
}

/// 🖱️ On-demand flow node-graph context menu from surface hit-test and selection snapshot.
fn flow_context_menu_items(registry: &AppActionRegistry, fixture: &FlowFixture, config: &FlowConfig, labels: &FlowPlayLabels, is_de: bool, surface: Option<&semio_framework_plugin::ContextMenuSurfaceTarget>) -> Vec<ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};

    let hits = surface.map(|target| target.hits.as_slice()).unwrap_or(&[]);
    let groups = surface.map(|target| target.selection.as_slice()).unwrap_or(&[]);
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
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the flow app; one field per label makes every locale×terminology combination compile-checked. `FlowConfig` carries no terminology axis, so `reuse_*` mirrors `native_*` throughout.
    struct FlowPlayLabels {
        widgets: native_en "Widgets", native_de "Widgets", reuse_en "Widgets", reuse_de "Widgets";
        synapses: native_en "Synapses", native_de "Synapsen", reuse_en "Synapses", reuse_de "Synapsen";
        extensions: native_en "Extensions", native_de "Erweiterungen", reuse_en "Extensions", reuse_de "Erweiterungen";
        extension_actions: native_en "Extension Actions", native_de "Erweiterungsaktionen", reuse_en "Extension Actions", reuse_de "Erweiterungsaktionen";
        sources: native_en "Sources", native_de "Quellen", reuse_en "Sources", reuse_de "Quellen";
        components: native_en "Components", native_de "Komponenten", reuse_en "Components", reuse_de "Komponenten";
        sinks: native_en "Sinks", native_de "Senken", reuse_en "Sinks", reuse_de "Senken";
        catalogue_slider: native_en "Slider", native_de "Schieberegler", reuse_en "Slider", reuse_de "Schieberegler";
        catalogue_note: native_en "Note", native_de "Notiz", reuse_en "Note", reuse_de "Notiz";
        catalogue_add: native_en "Add", native_de "Addieren", reuse_en "Add", reuse_de "Addieren";
        catalogue_and: native_en "And", native_de "Und", reuse_en "And", reuse_de "Und";
        catalogue_concat: native_en "Concat", native_de "Verketten", reuse_en "Concat", reuse_de "Verketten";
        catalogue_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        catalogue_export: native_en "Export", native_de "Exportieren", reuse_en "Export", reuse_de "Exportieren";
        extension_auto_layout: native_en "Auto Layout", native_de "Automatisches Layout", reuse_en "Auto Layout", reuse_de "Automatisches Layout";
        extension_auto_evaluate: native_en "Auto Evaluate", native_de "Automatisch Auswerten", reuse_en "Auto Evaluate", reuse_de "Automatisch Auswerten";
        extension_action_reorganize_canvas: native_en "Reorganize Canvas", native_de "Leinwand neu anordnen", reuse_en "Reorganize Canvas", reuse_de "Leinwand neu anordnen";
        extension_action_evaluate_fixture: native_en "Evaluate Fixture", native_de "Fixture auswerten", reuse_en "Evaluate Fixture", reuse_de "Fixture auswerten";
        canvas: native_en "Canvas", native_de "Leinwand", reuse_en "Canvas", reuse_de "Leinwand";
        widget: native_en "Widget", native_de "Widget", reuse_en "Widget", reuse_de "Widget";
        delete_selection: native_en "Delete selection", native_de "Auswahl löschen", reuse_en "Delete selection", reuse_de "Auswahl löschen";
        hide_preview: native_en "Hide preview", native_de "Vorschau ausblenden", reuse_en "Hide preview", reuse_de "Vorschau ausblenden";
        show_preview: native_en "Show preview", native_de "Vorschau einblenden", reuse_en "Show preview", reuse_de "Vorschau einblenden";
        add_node: native_en "Add node…", native_de "Knoten hinzufügen…", reuse_en "Add node…", reuse_de "Knoten hinzufügen…";
        reorganize: native_en "Reorganize", native_de "Neu anordnen", reuse_en "Reorganize", reuse_de "Neu anordnen";
        replace_image: native_en "Replace image…", native_de "Bild ersetzen…", reuse_en "Replace image…", reuse_de "Bild ersetzen…";
        window_main: native_en "Flow", native_de "Flow", reuse_en "Flow", reuse_de "Flow";
        window_compiled: native_en "DSL", native_de "DSL", reuse_en "DSL", reuse_de "DSL";
        window_generations: native_en "Generations", native_de "Generationen", reuse_en "Generations", reuse_de "Generationen";
        window_generate_form: native_en "Form", native_de "Formular", reuse_en "Form", reuse_de "Formular";
        window_generate_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        lod_mode: native_en "LOD Mode", native_de "LOD-Modus", reuse_en "LOD Mode", reuse_de "LOD-Modus";
        automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        proximity_distance: native_en "Proximity Distance", native_de "Näheabstand", reuse_en "Proximity Distance", reuse_de "Näheabstand";
        grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        grid_visible: native_en "Visible", native_de "Sichtbar", reuse_en "Visible", reuse_de "Sichtbar";
        grid_snap: native_en "Snap", native_de "Fang", reuse_en "Snap", reuse_de "Fang";
        grid_factor: native_en "Factor", native_de "Faktor", reuse_en "Factor", reuse_de "Faktor";
        select_all: native_en "Select All", native_de "Alles auswählen", reuse_en "Select All", reuse_de "Alles auswählen";
        zoom_to_selection: native_en "Zoom to Selection", native_de "Auf Auswahl zoomen", reuse_en "Zoom to Selection", reuse_de "Auf Auswahl zoomen";
        clear_selection: native_en "Clear Selection", native_de "Auswahl aufheben", reuse_en "Clear Selection", reuse_de "Auswahl aufheben";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
        value: native_en "Value", native_de "Wert", reuse_en "Value", reuse_de "Wert";
        text: native_en "Text", native_de "Text", reuse_en "Text", reuse_de "Text";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        none_placeholder: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        widget_not_found: native_en "Widget not found", native_de "Widget nicht gefunden", reuse_en "Widget not found", reuse_de "Widget nicht gefunden";
        generation_needed: native_en "Add a generation to edit input values.", native_de "Füge eine Generation hinzu, um Eingabewerte zu bearbeiten.", reuse_en "Add a generation to edit input values.", reuse_de "Füge eine Generation hinzu, um Eingabewerte zu bearbeiten.";
    }
}

/// 🗣️ Resolves the active label set from `cfg.locale`; falls back to native English.
fn flow_play_labels(cfg: &FlowConfig) -> &'static FlowPlayLabels {
    semio_framework_plugin::resolve_labels_for_locale::<FlowPlayLabels>(&cfg.locale)
}

/// 🗣️ Resolves a built-in extension's display name from its stable id; unknown ids fall back to the
/// extension's native English name as genuine runtime data (never authored UI copy).
fn flow_extension_label(id: &str, name: &'static str, labels: &FlowPlayLabels) -> Label {
    match id {
        "auto-layout" => labels.extension_auto_layout.into(),
        "auto-evaluate" => labels.extension_auto_evaluate.into(),
        _ => Label::data(name),
    }
}

/// 🗣️ Resolves a built-in extension action's display title from its stable action id; unknown ids
/// fall back to the action's native English title as genuine runtime data.
fn flow_extension_action_title_label(action_id: &str, title: &'static str, labels: &FlowPlayLabels) -> Label {
    match action_id {
        "flow.extension.reorganize" => labels.extension_action_reorganize_canvas.into(),
        "flow.extension.evaluate" => labels.extension_action_evaluate_fixture.into(),
        _ => Label::data(title),
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn build_document_tree(fixture: &FlowFixture, selected: &[String], labels: &FlowPlayLabels) -> UiNode {
    let widget_items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            tree_item_with_action(format!("flow-play-document.widget.{}", widget_id(widget)), Label::data(widget_tree_label(widget)), Some(widget_kind_label(widget).into()), flow_action("setSelection", Some(json!({ "ids": [widget_id(widget)] }))))
        })
        .collect();
    let synapse_items: Vec<UiTreeItemNode> =
        fixture.synapses.iter().map(|synapse| tree_item_desc(format!("flow-play-document.synapse.{}", synapse.id), Label::data(format!("{} → {}", synapse.from, synapse.to)), Some(format!("{} → {}", synapse.from_port, synapse.to_port)))).collect();
    PanelTreeBuilder::new("flow-play-document")
        .section_or_placeholder("flow-play-document.widgets", Some(labels.widgets.into()), true, widget_items, labels.none_placeholder)
        .section_or_placeholder("flow-play-document.synapses", Some(labels.synapses.into()), false, synapse_items, labels.none_placeholder)
        .selected(selected.iter().map(|id| format!("flow-play-document.widget.{id}")).collect())
        .build()
}

fn build_catalogue_tree(fixture: &FlowFixture, config: &FlowConfig, labels: &FlowPlayLabels) -> UiNode {
    let host = host_from_fixture(fixture, config);
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
                            Some(tree_item_with_action_draggable(format!("flow-play-catalogue.{id}.{kind}.{label}"), Label::data(label), Some(kind.to_string()), action, &flow_widget_drag_json(&descriptor)))
                        })
                        .collect()
                })
                .unwrap_or_default();
            Some(UiTreeSectionNode { presence: UiPresence::default(), id: format!("flow-play-catalogue.{id}"), label: Some(Label::data(title)), default_open: Some(true), items })
        })
        .collect();
    let tree_sections = if tree_sections.is_empty() { catalogue_tree_sections_fallback(labels) } else { tree_sections };
    let mut builder = PanelTreeBuilder::new("flow-play-catalogue");
    for section in tree_sections.into_iter().chain(flow_extensions_tree_sections(config, labels)) {
        builder = builder.section(section.id, section.label, section.default_open.unwrap_or(false), section.items);
    }
    builder.selected(vec![]).build()
}

/// 🧩️ Installed/enabled extension palette plus actions surfaced by active extensions.
fn flow_extensions_tree_sections(config: &FlowConfig, labels: &FlowPlayLabels) -> Vec<UiTreeSectionNode> {
    let extension_enabled = config.extension_enabled();
    let installed: Vec<UiTreeItemNode> = FLOW_EXTENSIONS
        .iter()
        .map(|(id, name, _, _, _)| {
            let enabled = extension_enabled.get(*id).copied().unwrap_or(false);
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
        .filter(|(id, ..)| extension_enabled.get(*id).copied().unwrap_or(false))
        .map(|(_, _, action_id, title, _)| {
            tree_item_with_action(format!("flow-play-extensions.action.{action_id}"), flow_extension_action_title_label(action_id, title, labels), Some((*action_id).into()), flow_action("runExtensionAction", Some(json!({ "actionId": action_id }))))
        })
        .collect();
    let mut sections = vec![UiTreeSectionNode { presence: UiPresence::default(), id: "flow-play-extensions.installed".into(), label: Some(labels.extensions.into()), default_open: Some(false), items: installed }];
    if !actions.is_empty() {
        sections.push(UiTreeSectionNode { presence: UiPresence::default(), id: "flow-play-extensions.actions".into(), label: Some(labels.extension_actions.into()), default_open: Some(false), items: actions });
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
fn flow_lod_measure(config: &FlowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: FLOW_LOD_MODE_AUTOMATIC.into(), value: FLOW_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(serde_json::from_str::<Vec<Value>>(&dag_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|lod| {
        let id = lod.get("id").and_then(|value| value.as_str())?.to_string();
        let name = lod.get("name").and_then(|value| value.as_str()).unwrap_or(&id).to_string();
        Some(MeasureSelectItem { id: id.clone(), value: id, label: name })
    }));
    WindowMeasure::Select { id: "flow-play-measures.lod".into(), label: Some(labels.lod_mode.into()), value: config.lod_mode.clone(), items, on_change: flow_action("setLodMode", Some(json!({ "value": config.lod_mode }))) }
}

fn flow_grid_measures_group(config: &FlowConfig, labels: &FlowPlayLabels) -> WindowMeasure {
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
            WindowMeasure::Toggle { id: "flow-play-measures.grid-visible".into(), icon_id: "layout-grid".into(), label: Some(labels.grid_visible.into()), pressed: config.grid_visible, text: None, on_change: flow_action("setGridVisible", None) },
            WindowMeasure::Toggle { id: "flow-play-measures.grid-snap".into(), icon_id: "magnet".into(), label: Some(labels.grid_snap.into()), pressed: config.grid_snap_enabled, text: None, on_change: flow_action("setGridSnapEnabled", None) },
            WindowMeasure::Slider {
                id: "flow-play-measures.grid-factor".into(),
                label: Some(format!("{} {:.1}", labels.grid_factor.as_str(), config.grid_factor)),
                value: config.grid_factor,
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

fn flow_window_measures(config: &FlowConfig, labels: &FlowPlayLabels) -> Vec<WindowMeasure> {
    vec![
        flow_lod_measure(config, labels),
        WindowMeasure::Slider {
            id: "flow-play-measures.proximity".into(),
            label: Some(labels.proximity_distance.into()),
            value: config.proximity_distance,
            min: 0.0,
            max: 240.0,
            step: Some(4.0),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: flow_action("setProximityDistance", None),
        },
        flow_grid_measures_group(config, labels),
    ]
}
//#endregion 🔖️WindowMeasures

fn build_inspector_tree(fixture: &FlowFixture, selected: &[String], labels: &FlowPlayLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            presence: UiPresence::default(),
            id: "flow-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
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
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.widget_not_found)],
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
            label: Label::data("inputSlider"),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "flow-play-inspector.slider-value".into(),
                label: labels.value.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "flow-play-inspector.slider-value.input".into(),
                    input_kind: "number".into(),
                    value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
                    placeholder: if mixed.uniform { None } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
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
            label: Label::data("inputNote"),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "flow-play-inspector.note-text".into(),
                label: labels.text.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "flow-play-inspector.note-text.input".into(),
                    input_kind: "text".into(),
                    value: mixed.value,
                    placeholder: mixed.placeholder.map(Label::data),
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
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "flow-play-inspector.id".into(),
                label: labels.id.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
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
fn render_main_graph(fixture: &FlowFixture, config: &FlowConfig) -> UiNode {
    let host = host_from_fixture(fixture, config);
    let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
    let viewport = NodeGraphViewport { x: config.camera.x, y: config.camera.y, zoom: config.camera.zoom };
    let fixture_json = serde_json::to_string(fixture).ok();
    let selection = config.selected_node_ids.clone();
    let driver = config.eval_driver();
    let flow_extras = flow_backed_node_graph_extras(fixture, &config.lod_mode, config.proximity_distance, config.grid_visible, config.grid_snap_enabled, config.grid_factor, Some(&driver));
    let preview_off_json = if config.preview_off_node_ids.is_empty() { None } else { serde_json::to_string(&config.preview_off_node_ids).ok() };
    build_node_graph_scene(
        FLOW_PLAY_SURFACE_MAIN,
        FLOW_PLAY_APP_ID,
        NodeGraphScene {
            editable: Some(true),
            operators: flow_extras.operators,
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json.or(fixture_json),
            eval_json: flow_extras.eval_json,
            computing_json: flow_extras.computing_json,
            selection,
            preview_off_json,
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )
}

fn render_compiled_dag(fixture: &FlowFixture, config: &FlowConfig) -> UiNode {
    let host = host_from_fixture(fixture, config);
    build_text_editor_scene(FLOW_PLAY_SURFACE_COMPILED, FLOW_PLAY_APP_ID, TextEditorScene::base(host.compiled_wire_literal(), Some("wire".into()), None))
}

fn render_generate_generations(config: &FlowConfig, locale: Locale, terminology: Terminology) -> UiNode {
    let generation = config.generation();
    render_generations_tree(FLOW_PLAY_APP_ID, "flow-play-generate", &generation.generations, generation.selected_generation_id.as_deref(), locale, terminology)
}

fn render_generate_form(fixture: &FlowFixture, config: &FlowConfig) -> UiNode {
    let spec = flow_fixture_to_form_spec(fixture);
    let generation = config.generation();
    let Some(active) = selected_generation(&generation) else {
        return ui_text(flow_play_labels(config).generation_needed);
    };
    render_generation_form_body(&spec, &active.values, FLOW_PLAY_APP_ID, "updateGenerationValues", &active.id)
}

fn render_generate_preview(config: &FlowConfig) -> UiNode {
    let generation = config.generation();
    let text = generation.preview_text.as_deref().filter(|value| !value.is_empty()).unwrap_or("(evaluate a generation to preview output)");
    render_generation_preview_text(FLOW_PLAY_SURFACE_GENERATE_PREVIEW, FLOW_PLAY_APP_ID, text)
}
//#endregion 🔖️Render

//#region 🔖️FlowPlayApp
/// 🧪️ B1: unit struct — every former `FlowPlayRuntime` field now lives in `flow_engine::FlowConfig`
/// (see `DocumentApp::Config`), written through `flow_op::FlowConfigOperation`s.
#[derive(Default)]
pub struct FlowPlayApp;

fn flow_internal_action(id: &str, label: LocalizedLabel, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

impl DocumentApp for FlowPlayApp {
    type Projection = FlowFixture;
    type Operation = FlowOperation;
    type Config = FlowConfig;
    type ConfigOperation = FlowConfigOperation;
    type Command = FlowCommand;

    fn app_id(&self) -> &str {
        FLOW_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        FLOW_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> FlowFixture {
        FlowFixture::default()
    }

    /// 🏷️ Maps each `FlowCommand` variant back to the action id it was declared under in
    /// `create_flow_app` — used by `VcsDocumentApp` for command-log labeling and the registry's
    /// View/Shell kind-discipline check. `SetLocale`/`FlowEvalTick` have no manifest declaration
    /// (host-pushed/internally-chained, not user-facing actions — matches
    /// `shooting_protocol::ShootingCommand::SetLocale`/`dag_protocol::DagCommand::SetLocale`'s
    /// equally-undeclared precedent).
    fn command_id(&self, command: &FlowCommand) -> &str {
        match command {
            FlowCommand::AddWidget { .. } => "addWidget",
            FlowCommand::RemoveWidget { .. } => "removeWidget",
            FlowCommand::DeleteSelection => "deleteSelection",
            FlowCommand::Disconnect { .. } => "disconnect",
            FlowCommand::ConnectMediaPorts { .. } => "connectMediaPorts",
            FlowCommand::MoveMediaNode { .. } => "moveMediaNode",
            FlowCommand::Reorganize => "reorganize",
            FlowCommand::PatchFlowWidgets { .. } => "patchFlowWidgets",
            FlowCommand::RenameFlowWidget { .. } => "renameFlowWidget",
            FlowCommand::NodeGraphEdit { .. } => "nodeGraphEdit",
            FlowCommand::SpotlightCommit { .. } => "spotlightCommit",
            FlowCommand::RunExtensionAction { .. } => "runExtensionAction",
            FlowCommand::Evaluate => "evaluate",
            FlowCommand::SelectAll => "selectAll",
            FlowCommand::FocusSelection => "focusSelection",
            FlowCommand::SetSelection { .. } => "setSelection",
            FlowCommand::SelectNode { .. } => "selectNode",
            FlowCommand::NodeGraphSelect { .. } => "nodeGraphSelect",
            FlowCommand::NodeGraphHover => "nodeGraphHover",
            FlowCommand::GraphPointerDown => "graphPointerDown",
            FlowCommand::NodeGraphViewport { .. } => "nodeGraphViewport",
            FlowCommand::SetLodMode { .. } => "setLodMode",
            FlowCommand::SetProximityDistance { .. } => "setProximityDistance",
            FlowCommand::SetGridVisible { .. } => "setGridVisible",
            FlowCommand::SetGridSnapEnabled { .. } => "setGridSnapEnabled",
            FlowCommand::SetGridFactor { .. } => "setGridFactor",
            FlowCommand::ClearSelection => "clearSelection",
            FlowCommand::ContextMenuAt { .. } => "contextMenuAt",
            FlowCommand::SetPreviewOff { .. } => "setPreviewOff",
            FlowCommand::OpenSpotlight => "openSpotlight",
            FlowCommand::ReplaceImage { .. } => "replaceImage",
            FlowCommand::SetCatalogueSections { .. } => "setCatalogueSections",
            FlowCommand::ToggleExtension { .. } => "toggleExtension",
            FlowCommand::AddGeneration => "addGeneration",
            FlowCommand::RemoveGeneration { .. } => "removeGeneration",
            FlowCommand::SelectGeneration { .. } => "selectGeneration",
            FlowCommand::RenameGeneration { .. } => "renameGeneration",
            FlowCommand::UpdateGenerationValues { .. } => "updateGenerationValues",
            FlowCommand::SetLocale { .. } => "setLocale",
            FlowCommand::FlowEvalTick => "flowEvalTick",
        }
    }

    fn handle(&self, command: &FlowCommand, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>) -> Result<Emit<FlowOperation, FlowConfigOperation>, Fault> {
        let fixture = doc.projection;
        let config = cfg.projection;
        match command {
            // ✏️ Operations — run the stateful `FlowHost` mutation, diff into granular operations.
            FlowCommand::AddWidget { kind, neuron_kind, x, y } => {
                let descriptor = match kind.as_str() {
                    "neuron" => json!({ "kind": "neuron", "neuronKind": neuron_kind.as_deref().unwrap_or("math.add") }).to_string(),
                    other => json!({ "kind": other }).to_string(),
                };
                let x = x.unwrap_or(120.0);
                let y = y.unwrap_or(120.0);
                let mut new_id = None;
                let operations = host_operations(fixture, config, |host| match host.add_widget(&descriptor, x, y) {
                    Ok(id) => {
                        new_id = Some(id);
                        true
                    }
                    Err(_) => false,
                });
                match new_id {
                    Some(id) => Ok(Emit { document_operations: operations, config_operations: vec![FlowConfigOperation::SetSelection { node_ids: vec![id], edge_ids: Vec::new(), handle_ids: Vec::new() }], ..Default::default() },
                    None => Ok(Emit::operations(operations),
                }
            }
            FlowCommand::RemoveWidget { widget_id: target_id } => {
                let operations = host_operations(fixture, config, |host| host.remove_widget(target_id).is_ok());
                if operations.is_empty() {
                    Ok(Emit::default()
                } else {
                    let remaining: Vec<String> = config.selected_node_ids.iter().filter(|id| *id != target_id).cloned().collect();
                    Ok(Emit {
                        document_operations: operations,
                        config_operations: vec![FlowConfigOperation::SetSelection { node_ids: remaining, edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }],
                        ..Default::default()
                    })
                }
            }
            FlowCommand::DeleteSelection => {
                let nodes = config.selected_node_ids.clone();
                let edges = config.selected_edge_ids.clone();
                let handles = config.selected_handle_ids.clone();
                let operations = host_operations(fixture, config, |host| {
                    sync_host_selection_domains(host, &nodes, &edges, &handles);
                    if !host.has_selection() {
                        return false;
                    }
                    host.delete_selection().is_ok()
                });
                if operations.is_empty() {
                    Ok(Emit::default()
                } else {
                    Emit { document_operations: operations, config_operations: vec![FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: Vec::new(), handle_ids: Vec::new() }], ..Default::default() }
                }
            }
            FlowCommand::Disconnect { synapse_id } => Ok(Emit::operations(host_operations(fixture, config, |host| host.disconnect(synapse_id).is_ok())),
            FlowCommand::ConnectMediaPorts { source_node_id, source_port_id, target_node_id, target_port_id } => {
                Ok(Emit::operations(host_operations(fixture, config, |host| host.connect_ports(source_node_id, source_port_id, target_node_id, target_port_id).is_ok()))
            }
            FlowCommand::MoveMediaNode { node_id, x, y } => {
                let operations = host_operations(fixture, config, |host| {
                    host.begin_change();
                    host.move_widget(node_id, *x, *y).is_ok()
                });
                if operations.is_empty() {
                    Ok(Emit::default()
                } else {
                    Ok(Emit::amend(operations, format!("move-{node_id}"))
                }
            }
            FlowCommand::Reorganize => Ok(Emit::operations(host_operations(fixture, config, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
            FlowCommand::PatchFlowWidgets { widget_ids, field, value } => {
                let next = patched_widgets_fixture(fixture, widget_ids, field, value);
                let operations = flow_fixture_operations(fixture, &next);
                if operations.is_empty() {
                    Ok(Emit::default()
                } else {
                    Ok(Emit::amend(operations, format!("patch-{field}-{}", widget_ids.join(",")))
                }
            }
            FlowCommand::RenameFlowWidget { old_id, value } => match renamed_fixture(fixture, old_id, value) {
                Some(next) => Ok(Emit {
                    document_operations: flow_fixture_operations(fixture, &next),
                    config_operations: vec![FlowConfigOperation::SetSelection { node_ids: vec![value.trim().to_string()], edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }],
                    ..Default::default()
                },
                None => Ok(Emit::default()),
            },
            FlowCommand::NodeGraphEdit { operations } => node_graph_edit_result(fixture, config, operations),
            FlowCommand::SpotlightCommit { operations } => node_graph_edit_result(fixture, config, operations),
            FlowCommand::RunExtensionAction { action_id } => {
                let entry = FLOW_EXTENSIONS.iter().find(|(_, _, entry_action_id, ..)| entry_action_id == action_id);
                let Some((id, _, _, _, effect)) = entry else {
                    return Ok(Emit::default();
                };
                if !config.extension_enabled().get(*id).copied().unwrap_or(false) {
                    return Ok(Emit::default();
                }
                match *effect {
                    "reorganize" => Ok(Emit::operations(host_operations(fixture, config, |host| host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok())),
                    "evaluate" => evaluate_result(fixture, config),
                    _ => Ok(Emit::default()),
                }
            }

            // 👁️ Config-only (was ephemeral `FlowPlayRuntime` state) — emit `config_operations`, never
            // document operations.
            FlowCommand::Evaluate => evaluate_result(fixture, config),
            FlowCommand::SelectAll => {
                let ids: Vec<String> = fixture.widgets.iter().map(widget_id).map(str::to_string).collect();
                Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: ids, edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }])
            }
            FlowCommand::FocusSelection => match focus_selection_camera(fixture, config) {
                Some(camera) => Ok(Emit::config(vec![FlowConfigOperation::SetCamera { camera }])),
                None => Ok(Emit::default()),
            },
            FlowCommand::SetSelection { ids, edge_ids, handle_ids } => Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: ids.clone(), edge_ids: edge_ids.clone(), handle_ids: handle_ids.clone() }])),
            FlowCommand::SelectNode { node_id } => Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: vec![node_id.clone()], edge_ids: Vec::new(), handle_ids: Vec::new() }])),
            FlowCommand::NodeGraphSelect { node_ids } => Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: node_ids.clone(), edge_ids: Vec::new(), handle_ids: Vec::new() }])),
            FlowCommand::NodeGraphHover => Ok(Emit::default()),
            FlowCommand::GraphPointerDown => Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }])),
            FlowCommand::NodeGraphViewport { camera } => Ok(Emit::config(vec![FlowConfigOperation::SetCamera { camera: camera.clone() }])),
            FlowCommand::SetLodMode { value } => {
                if value == FLOW_LOD_MODE_AUTOMATIC || DagDrawLod::from_id(value).is_some() {
                    Ok(Emit::config(vec![FlowConfigOperation::SetLodMode { value: value.clone() }])
                } else {
                    Ok(Emit::default()
                }
            }
            FlowCommand::SetProximityDistance { value } => Ok(Emit::config(vec![FlowConfigOperation::SetProximityDistance { value: value.max(0.0) }])),
            FlowCommand::SetGridVisible { pressed } => Ok(Emit::config(vec![FlowConfigOperation::SetGridVisible { value: pressed.unwrap_or(!config.grid_visible) }])),
            FlowCommand::SetGridSnapEnabled { pressed } => Ok(Emit::config(vec![FlowConfigOperation::SetGridSnapEnabled { value: pressed.unwrap_or(!config.grid_snap_enabled) }])),
            FlowCommand::SetGridFactor { value } => Ok(Emit::config(vec![FlowConfigOperation::SetGridFactor { value: value.clamp(0.5, 50.0) }])),
            FlowCommand::ClearSelection => Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: Vec::new(), edge_ids: Vec::new(), handle_ids: Vec::new() }])),
            FlowCommand::ContextMenuAt { id } => {
                if id.is_empty() {
                    Ok(Emit::default()
                } else {
                    Ok(Emit::config(vec![FlowConfigOperation::SetSelection { node_ids: vec![id.clone()], edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }])
                }
            }
            FlowCommand::SetPreviewOff { ids, value } => {
                let mut next = config.preview_off_node_ids.clone();
                if *value {
                    for id in ids {
                        if !next.contains(id) {
                            next.push(id.clone());
                        }
                    }
                } else {
                    next.retain(|id| !ids.contains(id));
                }
                Ok(Emit::config(vec![FlowConfigOperation::SetPreviewOff { node_ids: next }])
            }
            FlowCommand::OpenSpotlight => Ok(Emit::default()),
            FlowCommand::ReplaceImage { .. } => Ok(Emit::default()),
            FlowCommand::SetCatalogueSections { sections_json } => Ok(Emit::config(vec![FlowConfigOperation::SetCatalogueSections { sections_json: sections_json.clone() }])),
            FlowCommand::ToggleExtension { id, enabled } => {
                let mut map = config.extension_enabled();
                map.insert(id.clone(), *enabled);
                Ok(Emit::config(vec![FlowConfigOperation::SetExtensionEnabled { json: serde_json::to_string(&map).unwrap_or_default() }])
            }
            FlowCommand::AddGeneration => Ok(handle_generation("addGeneration", None, fixture, config),
            FlowCommand::RemoveGeneration { id } => Ok(handle_generation("removeGeneration", Some(&json!({ "id": id })), fixture, config),
            FlowCommand::SelectGeneration { id } => Ok(handle_generation("selectGeneration", Some(&json!({ "id": id })), fixture, config),
            FlowCommand::RenameGeneration { id, name } => Ok(handle_generation("renameGeneration", Some(&json!({ "id": id, "name": name })), fixture, config),
            FlowCommand::UpdateGenerationValues { generation_id, question_id, value } => {
                let value_json: Value = dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null);
                handle_generation("updateGenerationValues", Some(&json!({ "generationId": generation_id, "questionId": question_id, "value": value_json })), fixture, config)
            }
            FlowCommand::SetLocale { value } => Ok(Emit::config(vec![FlowConfigOperation::SetLocale { value: value.clone() }])),
            // 🧵️ One budgeted evaluation step (see `flow_core::FlowEvalDriver::tick`), off the main
            // thread. Chains itself via `HostEffect::DispatchAction` until the fixture's dirty set is
            // empty; persists the driver's new baseline/eval json via `SetEvalDriver` so the next
            // render/`pending_effects` call sees the converged state.
            FlowCommand::FlowEvalTick => {
                let mut driver = config.eval_driver();
                let mut host = host_from_fixture(fixture, config);
                let more = driver.tick(&mut host);
                Ok(Emit {
                    config_operations: vec![FlowConfigOperation::SetEvalDriver { json: serde_json::to_string(&driver).unwrap_or_default() }],
                    effects: if more { vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() },
                    ..Default::default()
                })
            }
        }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, example load, remote operations) in one place.
    /// Pure: recomputes the "is anything pending" probe fresh from the fixture and the driver's
    /// persisted baseline each call, never mutates anything durably.
    fn pending_effects(&self, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>) -> Vec<HostEffect> {
        let mut driver = cfg.projection.eval_driver();
        let host = host_from_fixture(doc.projection, cfg.projection);
        if driver.sync(&host) {
            vec![HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>) -> UiNode {
        let fixture = doc.projection;
        let config = cfg.projection;
        let labels = flow_play_labels(config);
        match body_key {
            FLOW_PLAY_BODY_MAIN => render_main_graph(fixture, config),
            FLOW_PLAY_BODY_COMPILED => render_compiled_dag(fixture, config),
            FLOW_PLAY_BODY_GENERATIONS => render_generate_generations(config, semio_framework_plugin::locale_from_str(&config.locale), Terminology::Native),
            FLOW_PLAY_BODY_GENERATE_FORM => render_generate_form(fixture, config),
            FLOW_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(config),
            FLOW_PLAY_BODY_DOCUMENT => build_document_tree(fixture, &config.selected_node_ids, labels),
            FLOW_PLAY_BODY_CATALOGUE => build_catalogue_tree(fixture, config, labels),
            FLOW_PLAY_BODY_INSPECTOR => build_inspector_tree(fixture, &config.selected_node_ids, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_measures(&self, _doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let labels = flow_play_labels(config);
        HashMap::from([(FLOW_PLAY_WINDOW_MAIN.to_string(), flow_window_measures(config, labels))])
    }

    fn context_menu(&self, request: &ContextMenuRequest, doc: &DocumentView<'_, FlowFixture>, cfg: &ConfigView<'_, FlowConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let config = cfg.projection;
        let labels = flow_play_labels(config);
        let is_de = config.locale.starts_with("de");
        flow_context_menu_items(registry, doc.projection, config, labels, is_de, request.surface.as_ref())
    }
}
//#endregion 🔖️FlowPlayApp

//#region 🔖️Manifest
pub fn create_flow_app() -> App {
    App::from_builder(
        App::builder(FLOW_PLAY_APP_ID, LocalizedLabel::native("Flow", "Flow")).document(["semio", "flow"])
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
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .mode("generate", LocalizedLabel::native("Generate", "Generieren"), "sparkles")
            .default_mode_id("edit")
            .window_kind(FLOW_PLAY_WINDOW_MAIN, LocalizedLabel::native("Flow", "Flow"), FLOW_PLAY_BODY_MAIN, SurfaceKind::NodeGraph, "flow-graph")
            .window_kind(FLOW_PLAY_WINDOW_COMPILED, LocalizedLabel::native("DSL", "DSL"), FLOW_PLAY_BODY_COMPILED, SurfaceKind::NodeGraph, "code")
            .window_kind(FLOW_PLAY_WINDOW_GENERATIONS, LocalizedLabel::native("Generations", "Generationen"), FLOW_PLAY_BODY_GENERATIONS, SurfaceKind::Canvas2d, "sparkles")
            .window_kind(FLOW_PLAY_WINDOW_GENERATE_FORM, LocalizedLabel::native("Form", "Formular"), FLOW_PLAY_BODY_GENERATE_FORM, SurfaceKind::Canvas2d, "clipboard-list")
            .window_kind(
                FLOW_PLAY_WINDOW_GENERATE_PREVIEW,
                LocalizedLabel::native("Preview", "Vorschau"),
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
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
                PanelGroup::Workbench,
                FLOW_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
                PanelGroup::Workbench,
                FLOW_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
                PanelGroup::Details,
                FLOW_PLAY_BODY_INSPECTOR,
            )
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
            // 👁️ Ephemeral view/config actions — mutate runtime, emit no operations.
            .view_action("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"))
            // 🗂️ Referenced by flow_context_menu_items — categorized for grouped-context-menu disclosure.
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
            .action_with(flow_internal_action("toggleExtension", LocalizedLabel::native("Toggle Extension", "Erweiterung umschalten"), ActionKind::View))
            .action_with(flow_internal_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::View))
            .action_with(flow_internal_action("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"), ActionKind::View))
            .action_with(flow_internal_action("selectGeneration", LocalizedLabel::native("Select Generation", "Generation auswählen"), ActionKind::View))
            .action_with(flow_internal_action("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"), ActionKind::View))
            .action_with(flow_internal_action("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"), ActionKind::View))
            // 📝️ Staged argument form for the panel-visible create action (module operators stay catalogue-driven).
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                    ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                ]).default_value("inputSlider"),
            ])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("mod+a", "selectAll")
            .keybinding("delete,backspace", "deleteSelection")
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS Wave 1)
            // — flow has no user-visible config defaults to expose, so `config_spec()` stays the trait
            // default `ConfigSpec::empty()`; declaring it explicitly here still keeps this app's typed
            // channel surface consistent with `shooting_ui::create_shooting_app`'s convention.
            .config(FlowPlayApp::default().config_spec()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), serde_json::to_string(&FlowFixture::default()).expect("FlowFixture::default() has no non-finite floats or non-string map keys, so serialization cannot fail"), "cylinder")
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
        PluginApp, VcsDocumentApp, ViewState,
    };

    fn render(app: &mut VcsDocumentApp<FlowPlayApp>, body_key: &str, view_state: &ViewState) -> String {
        serde_json::to_string(&app.render(body_key, None, view_state).expect("render")).unwrap()
    }

    fn context_menu_items(app: &mut VcsDocumentApp<FlowPlayApp>, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
        let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface, window_instance_id: None, point: None };
        serde_json::to_value(app.context_menu(&request)).unwrap_or(Value::Null)
    }

    fn preview_off_ids(app: &mut VcsDocumentApp<FlowPlayApp>, view_state: &ViewState) -> Value {
        let rendered: Value = serde_json::from_str(&render(app, FLOW_PLAY_BODY_MAIN, view_state)).expect("render json");
        rendered.pointer("/nodeGraph/previewOffJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
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
        let result = app.dispatch_typed(FlowCommand::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) }, &meta("local")).expect("addWidget");
        assert!(!result.operations.is_empty(), "addWidget must emit operations");
        assert_eq!(app.projection().expect("projection").widgets.len(), before + 1);
    }

    #[test]
    fn undo_restores_fixture_after_add_widget() {
        let mut app = new_app::<FlowPlayApp>();
        let before = app.projection().expect("projection").widgets.len();
        assert_undo_redo_round_trip(&mut app, FlowCommand::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) }, |app| app.projection().expect("projection").widgets.len(), before, before + 1);
    }

    #[test]
    fn selection_is_config_state_and_emits_no_document_operations() {
        let mut app = new_app::<FlowPlayApp>();
        let result = app.dispatch_typed(FlowCommand::SetSelection { ids: vec!["slider".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }, &meta("local")).expect("setSelection");
        assert!(result.operations.is_empty(), "selection must not produce document operations");
    }

    #[test]
    fn evaluate_updates_preview_state_without_operations() {
        let mut app = new_app::<FlowPlayApp>();
        let result = app.dispatch_typed(FlowCommand::Evaluate, &meta("local")).expect("evaluate");
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
        app.dispatch_typed(FlowCommand::SetLodMode { value: "bogus".into() }, &meta("local")).expect("bogus");
        app.dispatch_typed(FlowCommand::SetLodMode { value: "micro".into() }, &meta("local")).expect("micro");
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(json.contains("\\\"forcedLabel\\\":\\\"micro\\\"") || json.contains("\"forcedLabel\":\"micro\""));
    }

    #[test]
    fn toggle_extension_and_run_action_reorganizes_fixture() {
        let mut app = new_app::<FlowPlayApp>();
        let before = app.projection().expect("projection").widgets.len();
        let ignored = app.dispatch_typed(FlowCommand::RunExtensionAction { action_id: "flow.extension.reorganize".into() }, &meta("local")).expect("ignored");
        assert!(ignored.operations.is_empty(), "disabled extension action must be a no-operation");
        app.dispatch_typed(FlowCommand::ToggleExtension { id: "auto-layout".into(), enabled: true }, &meta("local")).expect("toggle");
        app.dispatch_typed(FlowCommand::RunExtensionAction { action_id: "flow.extension.reorganize".into() }, &meta("local")).expect("reorganize");
        assert_eq!(app.projection().expect("projection").widgets.len(), before, "reorganize keeps every widget");
    }

    #[test]
    fn flow_labels_resolve_native_english_and_german() {
        let mut app = new_app::<FlowPlayApp>();
        let english = render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState::default());
        assert!(english.contains("Widgets") && english.contains("Synapses"), "english labels: {english}");
        app.dispatch_typed(FlowCommand::SetLocale { value: "de-DE".into() }, &meta("local")).expect("set locale");
        let german = render(&mut app, FLOW_PLAY_BODY_DOCUMENT, &ViewState::default());
        assert!(german.contains("Synapsen"), "german labels: {german}");
    }

    #[test]
    fn default_runtime_enables_proximity_distance() {
        let mut app = new_app::<FlowPlayApp>();
        let json = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(json.contains("proximityDistance") && !json.contains(r#""proximityDistance":0"#));
    }

    #[test]
    fn window_measures_surface_lod_proximity_and_grid() {
        let mut app = new_app::<FlowPlayApp>();
        let measures = app.window_measures();
        let window_measures = measures.get(FLOW_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(window_measures.len(), 3);
        assert!(window_measures.iter().any(|measure| matches!(measure, WindowMeasure::Slider { id, .. } if id == "flow-play-measures.proximity")));
        assert!(window_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "flow-play-measures.grid")));
    }

    #[test]
    fn select_all_and_focus_selection_update_scene() {
        let mut app = new_app::<FlowPlayApp>();
        app.dispatch_typed(FlowCommand::SelectAll, &meta("local")).expect("selectAll");
        let selected = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert!(selected.contains("slider"));
        let before = selected.clone();
        app.dispatch_typed(FlowCommand::FocusSelection, &meta("local")).expect("focusSelection");
        let after = render(&mut app, FLOW_PLAY_BODY_MAIN, &ViewState::default());
        assert_ne!(before, after);
    }

    #[test]
    fn set_proximity_distance_updates_scene_lod_json() {
        let mut app = new_app::<FlowPlayApp>();
        app.dispatch_typed(FlowCommand::SetProximityDistance { value: 96.0 }, &meta("local")).expect("proximity");
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
        app.dispatch_typed(FlowCommand::SetSelection { ids: vec!["slider".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }, &meta("local")).expect("setSelection");
        let menu = context_menu_items(&mut app, None).to_string();
        assert!(menu.contains("setPreviewOff"), "menu should expose preview toggle: {menu}");
        assert!(menu.contains("Hide preview") || menu.contains("eye-off"), "menu should offer hide preview: {menu}");
        assert!(menu.contains("focusSelection"), "menu should expose zoom to selection: {menu}");
        assert!(menu.contains(r#""checked":true"#), "preview checked when visible: {menu}");
        assert!(!menu.contains(r#""id":"toggle-preview""#) || !menu.split("\"id\":\"toggle-preview\"").nth(1).unwrap_or("").split("\"id\":").next().unwrap_or("").contains("\"disabled\":true"), "preview must be enabled with selection: {menu}");
        app.dispatch_typed(FlowCommand::SetPreviewOff { ids: vec!["slider".into()], value: true }, &meta("local")).expect("setPreviewOff");
        let after_menu = context_menu_items(&mut app, None).to_string();
        let preview_off = preview_off_ids(&mut app, &ViewState::default());
        assert_eq!(preview_off, json!(["slider"]), "preview_off should land on scene: {preview_off}");
        assert!(after_menu.contains("Show preview") || after_menu.contains(r#""icon":"eye""#), "menu should offer show preview: {after_menu}");
    }

    #[test]
    fn context_menu_at_selects_target_and_enables_preview() {
        let mut app = flow_app_with_registry();
        let before = context_menu_items(&mut app, None).to_string();
        assert!(!before.contains(r#""id":"delete-selection""#), "preview starts without delete: {before}");
        app.dispatch_typed(FlowCommand::ContextMenuAt { id: "slider".into() }, &meta("local")).expect("contextMenuAt");
        let after = context_menu_items(&mut app, None).to_string();
        assert!(after.contains("setPreviewOff"), "menu keeps preview: {after}");
        assert!(after.contains(r#""ids":["slider"]"#) || after.contains("slider"), "preview args target the clicked node: {after}");
        assert!(!after.split("\"id\":\"toggle-preview\"").nth(1).unwrap_or("").contains("\"disabled\":true"), "preview enabled after contextMenuAt: {after}");
    }

    #[test]
    fn context_menu_annotates_mixed_selection_counts_and_omits_delete_without_selection() {
        let mut app = flow_app_with_registry();
        let empty = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "main".into(), kind: "nodeGraph".into(), hits: vec![], selection: vec![], text: None })).to_string();
        assert!(!empty.contains(r#""id":"delete-selection""#), "empty must omit delete: {empty}");

        app.dispatch_typed(
            FlowCommand::SetSelection { ids: vec!["n1".into(), "n2".into(), "n3".into(), "n4".into(), "n5".into(), "n6".into(), "n7".into(), "n8".into()], edge_ids: (1..=13).map(|i| format!("e{i}")).collect(), handle_ids: Vec::new() },
            &meta("local"),
        )
        .expect("setSelection");
        let menu = context_menu_items(
            &mut app,
            Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: "main".into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
                selection: vec![
                    semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec!["n1".into(), "n2".into(), "n3".into(), "n4".into(), "n5".into(), "n6".into(), "n7".into(), "n8".into()] },
                    semio_framework_plugin::ContextMenuSelectionGroup { domain: "edge".into(), ids: (1..=13).map(|i| format!("e{i}")).collect() },
                ],
                text: None,
            }),
        )
        .to_string();
        eprintln!("[DEBUG] mixed selection context menu: {menu}");
        assert!(menu.contains(r#""id":"delete-selection""#), "mixed selection must expose delete: {menu}");
        assert!(menu.contains("8 nodes and 13 edges"), "count phrase missing: {menu}");
        assert!(menu.contains("deleteSelection"), "delete action missing: {menu}");
    }

    #[test]
    fn context_menu_for_edge_hit_uses_config_edge_selection() {
        let mut app = flow_app_with_registry();
        app.dispatch_typed(FlowCommand::SetSelection { ids: Vec::new(), edge_ids: vec!["syn-1".into()], handle_ids: Vec::new() }, &meta("local")).expect("setSelection");
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
        eprintln!("[DEBUG] edge selection context menu: {menu}");
        assert!(menu.contains(r#""id":"delete-selection""#), "edge selection must expose delete: {menu}");
        assert!(menu.contains("1 edge") || menu.contains("1 Kante"), "edge count phrase missing: {menu}");
    }

    #[test]
    fn context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last() {
        let mut app = flow_app_with_registry();
        app.dispatch_typed(
            FlowCommand::SetSelection { ids: vec!["n1".into(), "n2".into(), "n3".into(), "n4".into(), "n5".into(), "n6".into(), "n7".into(), "n8".into()], edge_ids: (1..=13).map(|i| format!("e{i}")).collect(), handle_ids: Vec::new() },
            &meta("local"),
        )
        .expect("setSelection");
        let request = ContextMenuRequest {
            menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None },
            surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
                surface_id: "main".into(),
                kind: "nodeGraph".into(),
                hits: vec![semio_framework_plugin::ContextMenuHit { domain: "node".into(), id: "n1".into(), label: None }],
                selection: vec![
                    semio_framework_plugin::ContextMenuSelectionGroup { domain: "node".into(), ids: vec!["n1".into(), "n2".into(), "n3".into(), "n4".into(), "n5".into(), "n6".into(), "n7".into(), "n8".into()] },
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
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).map(|child| child.destructive == Some(true)).unwrap_or(false);
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must be last: {menu:?}");
    }

    #[test]
    fn host_from_fixture_deletes_edge_selected_by_synapse_domain() {
        let config = FlowConfig::default();
        let fixture = FlowFixture::default();
        let mut host = host_from_fixture(&fixture, &config);
        sync_host_selection_domains(&mut host, &[], &["s1".into()], &[]);
        eprintln!("[DEBUG] host_from_fixture edge selection: has={} edge_ids={:?}", host.has_selection(), host.dag.selected_edge_ids());
        assert!(host.has_selection(), "s1 must resolve through host_from_fixture edge map");
        host.delete_selection().expect("deleteSelection");
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
        eprintln!("[DEBUG] host_from_fixture after delete: synapses={:?}", host.fixture.synapses.iter().map(|synapse| synapse.id.as_str()).collect::<Vec<_>>());
    }

    #[test]
    fn delete_selection_action_removes_selected_synapses() {
        let mut app = flow_app_with_registry();
        let before = app.projection().expect("projection").synapses.len();
        app.dispatch_typed(FlowCommand::SetSelection { ids: Vec::new(), edge_ids: vec!["s1".into()], handle_ids: Vec::new() }, &meta("local")).expect("setSelection");
        let result = app.dispatch_typed(FlowCommand::DeleteSelection, &meta("local")).expect("deleteSelection");
        eprintln!("[DEBUG] deleteSelection action ops_len={}", result.operations.len());
        let after = app.projection().expect("projection");
        eprintln!("[DEBUG] deleteSelection action remaining={:?}", after.synapses.iter().map(|synapse| synapse.id.as_str()).collect::<Vec<_>>());
        assert!(!result.operations.is_empty(), "deleteSelection must emit operations for an edge");
        assert!(!after.synapses.iter().any(|synapse| synapse.id == "s1"), "synapse s1 must be removed");
        assert_eq!(after.synapses.len(), before - 1);
    }

    #[test]
    fn two_instances_converge_on_disjoint_edits() {
        let (mut instance_a, mut instance_b) = paired_apps::<FlowPlayApp>("mem://flow-convergence");

        instance_a.dispatch_typed(FlowCommand::RenameFlowWidget { old_id: "slider".into(), value: "input".into() }, &meta("actor-a")).expect("a renames slider");
        instance_b.dispatch_typed(FlowCommand::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(10.0), y: Some(10.0) }, &meta("actor-b")).expect("b adds a note");

        // A neutral history action always dispatches through the store, which pumps inbound operations first.
        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        assert!(projection_a.widgets.iter().any(|widget| widget_id(widget) == "input"), "A keeps its rename");
        assert!(projection_a.widgets.iter().any(|widget| matches!(widget, Widget::InputNote { .. })), "A absorbs B's note");
        assert_eq!(projection_a.widgets.len(), projection_b.widgets.len(), "both instances converge to the same widget set");
    }
}
//#endregion 🧪️Tests
