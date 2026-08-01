//! 🧱️ Procedural 3D app — DocumentApp impl, render, manifest (constitutional: ui).

use flow_core::forms_bridge::flow_fixture_to_form_spec;
use flow_core::{flow_backed_node_graph_extras, FlowFixture, FlowHost, Widget};
use playbook::{
    apply_generation_operation, generation_operations, render_generation_form_body, render_generation_preview_text, render_generations_tree, select_generation, selected_generation, GenerationOperation,
    GenerationPlayState,
};
use procedural_3d::{widget_id, Procedural3dDocument, PROCEDURAL_3D_SCHEMA};
use procedural_3d_engine::{
    ensure_gumball_node, fixture_to_workflow, generation_fixture_for, generation_preview_signature, gumball_rotate_params_json, gumball_scale_params_json, gumball_translate_params_json,
    gumball_widget_number_param, gumball_widget_offset, host_from_fixture, host_from_fixture_with_driver, preview_camera_json, preview_payload_cached, preview_selection_json, refresh_all_caches,
    refresh_generation_preview, Procedural3dRuntime, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_TORUS,
};
use procedural_3d_engine::{default_projection, example_projection, evaluated_preview_payload};
use procedural_3d_op::{procedural3d_fixture_operations, Procedural3dOperation};
use semio_framework_plugin::{
    apply_world3d_sun_action, build_node_graph_scene, build_world_3d_scene, create_default_layout, create_named_layout, merge_world_selection_ids, tree_item_with_action, ui_inspector_groups_to_tree,
    ui_inspector_mixed_number, ui_inspector_readonly_field, ui_stack_vertical, ui_text, world3d_scene, world3d_sun_measures, ActionArgDef, ActionArgOption, ActionDescriptor, ActionEmit, App,
    AppLabelsOverlayExt, ArtifactKindSpec, DocumentApp, DocumentView, MeasureSelectItem, MediaClass, MediaForm, MediaType, NodeGraphScene, OsMediaCapability, PanelGroup, PanelTreeBuilder,
    SelectionSet, SurfaceKind, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, UtilityDefinition, ViewState, WindowMeasure, SET_ACTIVE_UTILITY_ACTION_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
    ui_declarative_sections_to_tree,
};
use serde_json::{json, Value};
use std::cell::RefCell;

//#region 🔖️Constants
const PROCEDURAL_3D_PLAY_APP_ID: &str = "procedural3d-play";
const PROCEDURAL_3D_PLAY_CONTROLLER_ID: &str = "procedural3d-play";
const PROCEDURAL_3D_PLAY_SURFACE_MAIN: &str = "procedural.play";
const PROCEDURAL_3D_PLAY_SURFACE_PREVIEW: &str = "procedural.play.preview";
const PROCEDURAL_3D_PLAY_BODY_MAIN: &str = "procedural.play.main";
const PROCEDURAL_3D_PLAY_BODY_PREVIEW: &str = "procedural.play.preview";
const PROCEDURAL_3D_PLAY_BODY_DOCUMENT: &str = "procedural.play.document";
const PROCEDURAL_3D_PLAY_BODY_CATALOGUE: &str = "procedural.play.catalogue";
const PROCEDURAL_3D_PLAY_BODY_INSPECTION: &str = "procedural.play.inspection";
const PROCEDURAL_3D_PLAY_WINDOW_MAIN: &str = "procedural-main";
const PROCEDURAL_3D_PLAY_WINDOW_PREVIEW: &str = "procedural-preview";
const PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS: &str = "procedural3d-generations";
const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM: &str = "procedural3d-generate-form";
const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW: &str = "procedural3d-generate-preview";
const PROCEDURAL_3D_PLAY_BODY_GENERATIONS: &str = "procedural.play.generations";
const PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM: &str = "procedural.play.generate-form";
const PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";
const PROCEDURAL_3D_PLAY_SURFACE_GENERATIONS: &str = "procedural.play.generations";
const PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW: &str = "procedural.play.generate-preview";

const WIDGET_CATALOG: &[(&str, &str)] = &[
    ("neuron", "cpu"),
    ("inputSlider", "sliders-horizontal"),
    ("inputNote", "file-text"),
    ("outputPreview", "preview"),
];

/// 🧰️ The gumball utility active when the host has not yet set `view_state.active_utility_id` (first UtilityRef).
const PROCEDURAL_3D_TRANSFORM_UTILITY_DEFAULT: &str = "move";
//#endregion 🔖️Constants

//#region 🔖️Types
/// 🧾️ Transient render/action bundle — the persisted projection (fixture + generations) with the
/// ephemeral runtime's selection, caches, and derived preview overlaid, so the pure panel/render
/// helpers keep reading a single value. Assembled per call; never serialized as the document.
struct Procedural3dPlayView {
    fixture: FlowFixture,
    runtime: Procedural3dRuntime,
    generation: GenerationPlayState,
}

/// 🧾️ Overlays the ephemeral runtime's generation selection and derived preview onto the persisted
/// generation state to build a {@link Procedural3dPlayView} for rendering.
fn play_view(projection: &Procedural3dDocument, runtime: &Procedural3dRuntime) -> Procedural3dPlayView {
    let mut generation = projection.generation.clone();
    generation.selected_generation_id = runtime.selected_generation_id.clone();
    generation.preview_text = runtime.generation_preview_text.clone();
    Procedural3dPlayView { fixture: projection.fixture.clone(), runtime: runtime.clone(), generation }
}
//#endregion 🔖️Types

//#region 🔖️DocumentHelpers
fn procedural_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: PROCEDURAL_3D_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args,
    }
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &SelectionSet) -> Vec<String> {
    args.and_then(|value| value.get("ids"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
        .filter(|ids: &Vec<String>| !ids.is_empty())
        .unwrap_or_else(|| fallback.to_vec())
}

fn generation_preview_payload(view: &Procedural3dPlayView) -> (String, String) {
    let fixture = generation_fixture_for(&view.fixture, &view.generation);
    let signature = generation_preview_signature(&fixture, &view.generation);
    if let Some(cache) = &view.runtime.generation_preview_cache {
        if cache.signature == signature {
            return (cache.meshes_json.clone(), cache.instances_json.clone());
        }
    }
    evaluated_preview_payload(&fixture, &view.runtime)
}

/// 🎚️ Level-of-detail display measure for the flow window — the migrated home of the old LOD
/// utility bar toggles (a display option, never an interactive utility). Dispatches `setLodMode` (a View action).
fn procedural3d_lod_measure(lod_mode: &str) -> WindowMeasure {
    let current = if lod_mode.is_empty() { "solid" } else { lod_mode };
    WindowMeasure::Select {
        id: "procedural3d-measure-lod".into(),
        label: Some("LOD".into()),
        value: current.into(),
        items: vec![
            MeasureSelectItem { id: "procedural3d-measure-lod-solid".into(), value: "solid".into(), label: "Solid".into() },
            MeasureSelectItem { id: "procedural3d-measure-lod-wireframe".into(), value: "wireframe".into(), label: "Wireframe".into() },
        ],
        on_change: procedural_action("setLodMode", None),
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Terminology
semio_framework_plugin::app_labels! {
    /// 🗣️ Complete UI label set for the 3D flow app; one field per label makes every locale combination compile-checked.
    struct Procedural3dLabels {
        widgets: &'static str = en: "Widgets", de: "Elemente";
        schema_prefix: &'static str = en: "Schema:", de: "Schema:";
        widgets_prefix: &'static str = en: "Widgets:", de: "Elemente:";
        no_selection: &'static str = en: "No selection", de: "Keine Auswahl";
        id_field: &'static str = en: "Id", de: "ID";
        value_field: &'static str = en: "Value", de: "Wert";
        range_field: &'static str = en: "Range", de: "Bereich";
        widget_group: &'static str = en: "Widget", de: "Element";
        generate_hint: &'static str = en: "Add a generation to edit input values.", de: "Erstelle eine Generation, um Eingabewerte zu bearbeiten.";
        preview_hint: &'static str = en: "(evaluate a generation to preview output)", de: "(Generation auswerten, um die Ausgabe in der Vorschau zu sehen)";
        catalog_neuron: &'static str = en: "Neuron", de: "Neuron";
        catalog_slider: &'static str = en: "Slider", de: "Schieberegler";
        catalog_note: &'static str = en: "Note", de: "Notiz";
        catalog_preview: &'static str = en: "Preview", de: "Vorschau";
        window_flow: &'static str = en: "Flow", de: "Workflow";
        window_preview: &'static str = en: "Preview", de: "Vorschau";
        window_generations: &'static str = en: "Generations", de: "Generationen";
        window_generate_form: &'static str = en: "Form", de: "Formular";
        window_generate_preview: &'static str = en: "Preview", de: "Vorschau";
        delete_selection: &'static str = en: "Delete selection", de: "Auswahl löschen";
    }
}

/// 🗣️ Resolves the active label set from the shell-provided locale; falls back to native English.
fn procedural3d_labels(view_state: &ViewState) -> &'static Procedural3dLabels {
    semio_framework_plugin::resolve_labels::<Procedural3dLabels>(view_state)
}

/// 🗣️ Resolves a catalogue widget kind's display label from its stable id; unknown kinds fall back to the id itself.
fn procedural3d_catalog_label(kind: &'static str, labels: &Procedural3dLabels) -> &'static str {
    match kind {
        "neuron" => labels.catalog_neuron,
        "inputSlider" => labels.catalog_slider,
        "inputNote" => labels.catalog_note,
        "outputPreview" => labels.catalog_preview,
        _ => kind,
    }
}
//#endregion 🔖️Terminology

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_procedural3d_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn procedural3d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("nodeGraphViewport", "Set Viewport", "Ansicht festlegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("nodeGraphEdit", "Edit Graph", "Graph bearbeiten"),
        ("deleteSelection", "Delete Selection", "Auswahl löschen"),
        ("removeWidget", "Remove Widget", "Element entfernen"),
        ("moveMediaNode", "Move Node", "Knoten verschieben"),
        ("addWidget", "Add Widget", "Element hinzufügen"),
        ("patchFlowWidgets", "Patch Flow Widgets", "Flow-Elemente aktualisieren"),
        ("reorganize", "Reorganize", "Neu anordnen"),
        ("translateSelection", "Translate Selection", "Auswahl verschieben"),
        ("rotateSelection", "Rotate Selection", "Auswahl drehen"),
        ("scaleSelection", "Scale Selection", "Auswahl skalieren"),
        ("addGeneration", "Add Generation", "Generation hinzufügen"),
        ("removeGeneration", "Remove Generation", "Generation entfernen"),
        ("renameGeneration", "Rename Generation", "Generation umbenennen"),
        ("updateGenerationValues", "Update Generation Values", "Generationswerte aktualisieren"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("selectNode", "Select Node", "Knoten auswählen"),
        ("nodeGraphSelect", "Node Graph Select", "Graph-Auswahl"),
        ("nodeGraphHover", "Node Graph Hover", "Graph-Hover"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("worldPointerDown", "World Pointer Down", "Welt-Zeiger gedrückt"),
        ("graphPointerDown", "Graph Pointer Down", "Graph-Zeiger gedrückt"),
        ("worldSelect", "World Select", "Welt auswählen"),
        ("worldHover", "World Hover", "Überfahren (Welt)"),
        ("setSelectionMethod", "Set Selection Method", "Auswahlmethode festlegen"),
        ("setLodMode", "Set Lod Mode", "LOD-Modus festlegen"),
        ("setShowMode", "Set Show Mode", "Anzeigemodus festlegen"),
        ("toggleSun", "Toggle Sun", "Sonne umschalten"),
        ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
        ("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen"),
        ("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("selectGeneration", "Set Generation", "Generation auswählen"),
    ];
    semio_framework_plugin::localized_label_map(is_de, ENTRIES)
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_procedural3d_app`.
fn procedural3d_utility_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("move", "Move", "Verschieben"),
        ("rotate", "Rotate", "Drehen"),
        ("scale", "Scale", "Skalieren"),
    ];
    semio_framework_plugin::localized_label_map(is_de, ENTRIES)
}
//#endregion 🔖️CommandLabels

//#region 🔖️Panels
/// 🌳️ SDK's `tree_item_with_action` plus an icon id — this crate's document/catalogue trees carry
/// icons per item, which the shared helper doesn't model directly.
fn tree_item_with_icon(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: icon_id.map(Into::into), menu: None,
    ..tree_item_with_action(id, label, None, action) }
}

fn build_document_tree(fixture: &FlowFixture, selected_node_ids: &SelectionSet, labels: &Procedural3dLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = fixture
        .widgets
        .iter()
        .map(|widget| {
            let id = widget_id(widget).to_string();
            tree_item_with_icon(
                format!("procedural-widget:{id}"),
                id.clone(),
                Some("cpu"),
                procedural_action("setSelection", Some(json!({ "ids": [id] }))),
            )
        })
        .collect();
    PanelTreeBuilder::new("procedural-play-document")
        .section("procedural-play-document.widgets", Some(labels.widgets.into()), true, items)
        .selected(selected_node_ids.iter().map(|id| format!("procedural-widget:{id}")).collect())
        .build()
}

fn build_catalogue_tree(labels: &Procedural3dLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = WIDGET_CATALOG
        .iter()
        .map(|(kind, icon)| {
            tree_item_with_icon(
                format!("procedural-play-catalogue.{kind}"),
                procedural3d_catalog_label(*kind, labels),
                Some(icon),
                procedural_action("addWidget", Some(json!({ "kind": kind }))),
            )
        })
        .collect();
    PanelTreeBuilder::new("procedural-play-catalogue")
        .section("procedural-play-catalogue.widgets", Some(labels.widgets.into()), true, items)
        .build()
}

fn build_inspector_tree(fixture: &FlowFixture, selected_node_ids: &SelectionSet, labels: &Procedural3dLabels) -> UiNode {
    let Some(selected_id) = selected_node_ids.first() else {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "procedural-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![
                ui_text(format!("{} {}", labels.schema_prefix, fixture.schema)),
            ui_text(format!("{} {}", labels.widgets_prefix, fixture.widgets.len())),
            ],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    let Some(widget) = fixture.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
        return ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "procedural-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![ui_text(labels.no_selection.to_string())],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    let mut fields = vec![ui_inspector_readonly_field(
        "procedural-play-inspector.id",
        labels.id_field,
        widget_id(widget),
    )];
    if let Widget::InputSlider { value, min, max, .. } = widget {
        let mixed = ui_inspector_mixed_number(&[*value]);
        fields.push(UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "procedural-play-inspector.value".into(),
            label: labels.value_field.into(),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode { presence: UiPresence::default(),
                id: "procedural-play-inspector.value.input".into(),
                input_kind: "number".into(),
                value: mixed.value.to_string(),
                placeholder: None,
                commit: None,
                on_change: procedural_action(
                    "patchFlowWidgets",
                    Some(json!({ "widgetIds": [selected_id], "field": "value" })),
                ),
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
        }));
        fields.push(ui_inspector_readonly_field(
            "procedural-play-inspector.range",
            labels.range_field,
            &format!("{min}..{max}"),
        ));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(),
        id: "procedural-play-inspector.widget".into(),
        label: labels.widget_group.into(),
        default_open: None,
        fields,
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Render
fn render_generate_generations(envelope: &Procedural3dPlayView) -> UiNode {
    render_generations_tree(
        PROCEDURAL_3D_PLAY_APP_ID,
        "procedural3d-play-generate",
        &envelope.generation.generations,
        envelope.generation.selected_generation_id.as_deref(),
    )
}

fn render_generate_form(envelope: &Procedural3dPlayView, labels: &Procedural3dLabels) -> UiNode {
    let spec = flow_fixture_to_form_spec(&envelope.fixture);
    let Some(generation) = selected_generation(&envelope.generation) else {
        return ui_text(labels.generate_hint);
    };
    render_generation_form_body(
        &spec,
        &generation.values,
        PROCEDURAL_3D_PLAY_APP_ID,
        "updateGenerationValues",
        &generation.id,
    )
}

fn render_generate_preview(envelope: &Procedural3dPlayView, labels: &Procedural3dLabels, active_utility: &str) -> UiNode {
    let (meshes_json, instances_json) = generation_preview_payload(envelope);
    if meshes_json == "[]" && instances_json == "[]" {
        let text = envelope
            .generation
            .preview_text
            .as_deref()
            .filter(|value| !value.is_empty())
            .unwrap_or(labels.preview_hint);
        return render_generation_preview_text(
            PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW,
            PROCEDURAL_3D_PLAY_APP_ID,
            text,
        );
    }
    build_world_3d_scene(
        PROCEDURAL_3D_PLAY_SURFACE_GENERATE_PREVIEW,
        PROCEDURAL_3D_PLAY_APP_ID,
        world3d_scene(
            preview_camera_json(&envelope.runtime),
            meshes_json,
            instances_json,
            preview_selection_json(&envelope.runtime, active_utility),
            &envelope.runtime.sun,
        ),
    )
}
//#endregion 🔖️Render

//#region 🔖️Procedural3dPlayApp
#[derive(Default)]
pub struct Procedural3dPlayApp {
    runtime: RefCell<Procedural3dRuntime>,
}

impl Procedural3dPlayApp {
    /// 🔀️ Diffs a mutated fixture into operations. Diffs against the host-normalized baseline of `before`
    /// (not the raw projection) so `FlowHost`'s own dedupe/dag-rebuild normalization does not leak
    /// spurious collection operations — only the actual mutation becomes an operation, keeping concurrent
    /// disjoint edits mergeable on the backbone. Never evaluates: `pending_effects` (called after
    /// every action's `refreshUi` pass) arms the `flowEvalTick` chain that refreshes the preview
    /// cache once the new fixture's dirty set resolves.
    fn commit_fixture(&self, before: &FlowFixture, target: &FlowFixture) -> Vec<Procedural3dOperation> {
        let baseline = host_from_fixture(before).fixture;
        procedural3d_fixture_operations(&baseline, target)
    }

    /// 🧬️ Emits generation operations for the generate-mode actions, updating ephemeral selection and
    /// preview from the post-operation state. `selectGeneration` is a view action (no operations).
    fn handle_generation(
        &self,
        action: &str,
        args: Option<&Value>,
        projection: &Procedural3dDocument,
    ) -> ActionEmit<Procedural3dOperation> {
        let spec = flow_fixture_to_form_spec(&projection.fixture);
        let mut state = projection.generation.clone();
        let mut runtime = self.runtime.borrow_mut();
        state.selected_generation_id = runtime.selected_generation_id.clone();
        if action == "selectGeneration" {
            if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                select_generation(&mut state, id);
            }
            runtime.selected_generation_id = state.selected_generation_id.clone();
            refresh_generation_preview(&mut runtime, &projection.fixture, &state);
            refresh_all_caches(&mut runtime, &projection.fixture, &state);
            return ActionEmit::default();
        }
        let Some(operations) = generation_operations(action, args, &state, &spec) else {
            return ActionEmit::default();
        };
        for operation in &operations {
            apply_generation_operation(&mut state, operation);
        }
        runtime.selected_generation_id = state.selected_generation_id.clone();
        refresh_generation_preview(&mut runtime, &projection.fixture, &state);
        refresh_all_caches(&mut runtime, &projection.fixture, &state);
        let coalesce_key = (action == "updateGenerationValues").then(|| "generation-values".to_string());
        ActionEmit {
            operations: operations.into_iter().map(Procedural3dOperation::Generation).collect(),
            coalesce_key,
            ..Default::default()
        }
    }

    /// 🧭️ Runs a gumball transform (translate/rotate/scale) as a fixture operation, splicing transform
    /// neurons via `ensure_gumball_node` and re-selecting the resulting transform widgets.
    fn gumball_transform(
        &self,
        fixture: &FlowFixture,
        args: Option<&Value>,
        operation: &str,
        apply: impl Fn(&mut FlowHost, &str) -> bool,
    ) -> ActionEmit<Procedural3dOperation> {
        let ids = mesh_selection_ids(args, &self.runtime.borrow().selected_node_ids);
        let mut host = host_from_fixture(fixture);
        let mut new_selection = Vec::new();
        let mut changed = false;
        for id in &ids {
            if let Ok(transform_id) = ensure_gumball_node(&mut host, id, operation) {
                if apply(&mut host, &transform_id) {
                    new_selection.push(transform_id);
                    changed = true;
                }
            }
        }
        if changed {
            let operations = self.commit_fixture(fixture, &host.fixture);
            self.runtime.borrow_mut().selected_node_ids = SelectionSet::from(new_selection);
            return ActionEmit::amend(operations, format!("gumball-{operation}"));
        }
        ActionEmit::default()
    }
}

impl DocumentApp for Procedural3dPlayApp {
    type Projection = Procedural3dDocument;
    type Operation = Procedural3dOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        PROCEDURAL_3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PROCEDURAL_3D_SCHEMA
    }

    fn initial_projection(&self) -> Procedural3dDocument {
        default_projection()
    }

    fn handle_action(
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, Procedural3dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> ActionEmit<Procedural3dOperation> {
        let fixture = &doc.projection.fixture;
        match action {
            // 👁️ View actions — mutate ephemeral runtime, emit no operations.
            "setSelection" | "selectNode" | "nodeGraphSelect" => {
                self.runtime.borrow_mut().selected_node_ids = SelectionSet::from(node_graph_selection_ids(args));
                ActionEmit::default()
            }
            "nodeGraphHover" => {
                if let Some(widget_id) = parse_node_graph_hover_widget_id(args) {
                    self.runtime.borrow_mut().hovered_node_id = widget_id;
                }
                ActionEmit::default()
            }
            "setHover" => {
                if args.is_none() || args.and_then(|value| value.get("objectId")).is_none() {
                    self.runtime.borrow_mut().hovered_node_id = None;
                } else {
                    self.runtime.borrow_mut().hovered_node_id = args
                        .and_then(|value| value.get("objectId"))
                        .and_then(|value| value.as_str())
                        .map(str::to_string);
                }
                ActionEmit::default()
            }
            "worldPointerDown" | "graphPointerDown" => ActionEmit::default(),
            // 🧰️ Host-owned active-utility switch — clear in-progress hover scratch, never emit operations.
            SET_ACTIVE_UTILITY_ACTION_ID => {
                self.runtime.borrow_mut().hovered_node_id = None;
                ActionEmit::default()
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value(value.clone()).ok())
                    .unwrap_or_default();
                {
                    let mut runtime = self.runtime.borrow_mut();
                    runtime.selected_node_ids = merge_world_selection_ids(&runtime.selected_node_ids, &ids, merge);
                }
                ActionEmit::default()
            }
            "worldHover" => {
                self.runtime.borrow_mut().hovered_node_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                ActionEmit::default()
            }
            "setSelectionMethod" => {
                self.runtime.borrow_mut().selection_method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle").into();
                ActionEmit::default()
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    self.runtime.borrow_mut().lod_mode = mode.into();
                }
                ActionEmit::default()
            }
            "setShowMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    self.runtime.borrow_mut().show_mode = mode.into();
                }
                ActionEmit::default()
            }
            "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                apply_world3d_sun_action(&mut self.runtime.borrow_mut().sun, action, args);
                ActionEmit::default()
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        self.runtime.borrow_mut().preview_camera = parsed;
                    }
                }
                ActionEmit::default()
            }
            // 📷️ Graph camera — ephemeral view state (never a document operation), same model as flow-play.
            "nodeGraphViewport" => {
                if let Some(camera) = args
                    .and_then(|value| value.get("viewportJson"))
                    .and_then(|value| value.as_str())
                    .and_then(|json| serde_json::from_str(json).ok())
                {
                    self.runtime.borrow_mut().camera = camera;
                }
                ActionEmit::default()
            }
            // ✏️ Operations — compute the target fixture via the host, emit fixture operations.
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let target = example_projection(example_id);
                let mut operations: Vec<Procedural3dOperation> = doc
                    .projection
                    .generation
                    .generations
                    .iter()
                    .map(|generation| Procedural3dOperation::Generation(GenerationOperation::Remove { id: generation.id.clone() }))
                    .collect();
                operations.extend(procedural3d_fixture_operations(fixture, &target.fixture));
                let camera = target.fixture.camera.clone();
                *self.runtime.borrow_mut() = Procedural3dRuntime { camera, ..Procedural3dRuntime::default() };
                ActionEmit::operations(operations)
            }
            "nodeGraphEdit" => {
                let sub_operations = args.and_then(|value| value.get("operations")).and_then(|value| value.as_array()).cloned().unwrap_or_default();
                let selected = self.runtime.borrow().selected_node_ids.clone();
                let mut host = host_from_fixture(fixture);
                let mut cleared = false;
                for operation in &sub_operations {
                    match operation.get("operation").and_then(|value| value.as_str()).unwrap_or("") {
                        "setFixture" => {
                            if let Some(new_fixture) = operation.get("fixtureJson").and_then(|value| value.as_str()).and_then(|json| serde_json::from_str::<FlowFixture>(json).ok()) {
                                host.replace_fixture(new_fixture);
                            }
                        }
                        "deleteSelection" => {
                            for id in &selected {
                                if host.remove_widget(id).is_ok() {
                                    cleared = true;
                                }
                            }
                        }
                        "connect" => {
                            let from = operation.get("sourceNodeId").and_then(|value| value.as_str());
                            let from_port = operation.get("sourcePortId").and_then(|value| value.as_str());
                            let to = operation.get("targetNodeId").and_then(|value| value.as_str());
                            let to_port = operation.get("targetPortId").and_then(|value| value.as_str());
                            if let (Some(from), Some(from_port), Some(to), Some(to_port)) = (from, from_port, to, to_port) {
                                let _ = host.connect_ports(from, from_port, to, to_port);
                            }
                        }
                        _ => {}
                    }
                }
                let operations = self.commit_fixture(fixture, &host.fixture);
                if cleared {
                    self.runtime.borrow_mut().selected_node_ids.clear();
                }
                ActionEmit::operations(operations)
            }
            "deleteSelection" => {
                let selected = self.runtime.borrow().selected_node_ids.clone();
                let mut host = host_from_fixture(fixture);
                let mut cleared = false;
                for id in &selected {
                    if host.remove_widget(id).is_ok() {
                        cleared = true;
                    }
                }
                let operations = self.commit_fixture(fixture, &host.fixture);
                if cleared {
                    self.runtime.borrow_mut().selected_node_ids.clear();
                }
                ActionEmit::operations(operations)
            }
            "removeWidget" => {
                let target_id = args
                    .and_then(|value| value.get("widgetId"))
                    .or_else(|| args.and_then(|value| value.get("id")))
                    .and_then(|value| value.as_str())
                    .map(str::to_string);
                if let Some(target_id) = target_id {
                    let mut host = host_from_fixture(fixture);
                    if host.remove_widget(&target_id).is_ok() {
                        let operations = self.commit_fixture(fixture, &host.fixture);
                        self.runtime.borrow_mut().selected_node_ids.remove_id(&target_id);
                        return ActionEmit::operations(operations);
                    }
                }
                ActionEmit::default()
            }
            "moveMediaNode" => {
                let node_id = args.and_then(|value| value.get("nodeId")).and_then(|value| value.as_str()).map(str::to_string);
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64());
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64());
                if let (Some(node_id), Some(x), Some(y)) = (node_id, x, y) {
                    let mut host = host_from_fixture(fixture);
                    if host.move_widget(&node_id, x, y).is_ok() {
                        return ActionEmit::operations(self.commit_fixture(fixture, &host.fixture));
                    }
                }
                ActionEmit::default()
            }
            "addWidget" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("inputSlider");
                let descriptor = match kind {
                    "neuron" => json!({ "kind": "neuron", "neuronKind": "math.add" }).to_string(),
                    other => json!({ "kind": other }).to_string(),
                };
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let mut host = host_from_fixture(fixture);
                if let Ok(id) = host.add_widget(&descriptor, x, y) {
                    let operations = self.commit_fixture(fixture, &host.fixture);
                    self.runtime.borrow_mut().selected_node_ids = SelectionSet::from(vec![id]);
                    return ActionEmit::operations(operations);
                }
                ActionEmit::default()
            }
            "patchFlowWidgets" => {
                let widget_ids: Vec<String> = args.and_then(|value| value.get("widgetIds")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let raw_value = args.and_then(|value| value.get("value")).and_then(|entry| entry.as_f64());
                let mut host = host_from_fixture(fixture);
                let baseline = host.fixture.clone();
                for widget in host.fixture.widgets.iter_mut() {
                    if !widget_ids.contains(&widget_id(widget).to_string()) {
                        continue;
                    }
                    if let (Widget::InputSlider { value: slider_value, .. }, Some(value)) = (widget, raw_value) {
                        if field == "value" {
                            *slider_value = value;
                        }
                    }
                }
                ActionEmit::operations(procedural3d_fixture_operations(&baseline, &host.fixture))
            }
            "reorganize" => {
                let mut host = host_from_fixture(fixture);
                if host.reorganize(r#"{"orientation":"leftRight"}"#).is_ok() {
                    return ActionEmit::operations(self.commit_fixture(fixture, &host.fixture));
                }
                ActionEmit::default()
            }
            "translateSelection" => {
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                self.gumball_transform(fixture, args, "translate", move |host, transform_id| {
                    let current = gumball_widget_offset(host, transform_id);
                    let next = [current[0] + dx, current[1] + dy, current[2] + dz];
                    host.set_neuron_params(transform_id, &gumball_translate_params_json(next)).is_ok()
                })
            }
            "rotateSelection" => {
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                self.gumball_transform(fixture, args, "rotate", move |host, transform_id| {
                    let current_angle = gumball_widget_number_param(host, transform_id, "angle", 0.0);
                    host.set_neuron_params(transform_id, &gumball_rotate_params_json([ax, ay, az], current_angle + angle)).is_ok()
                })
            }
            "scaleSelection" => {
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let uniform_factor = (sx + sy + sz) / 3.0;
                self.gumball_transform(fixture, args, "scale", move |host, transform_id| {
                    let current_factor = gumball_widget_number_param(host, transform_id, "factor", 1.0);
                    host.set_neuron_params(transform_id, &gumball_scale_params_json(current_factor * uniform_factor)).is_ok()
                })
            }
            "addGeneration" | "removeGeneration" | "selectGeneration" | "renameGeneration" | "updateGenerationValues" => {
                self.handle_generation(action, args, doc.projection)
            }
            // 🧵️ One budgeted evaluation step (see `FlowEvalDriver::tick`), off the main thread —
            // the plugin worker runs this, never the renderer. Chains itself via `DispatchAction`
            // until the fixture's dirty set is empty, then refreshes the mesh preview caches once
            // (cheap: every node hit the shared `procedural_neural_cache()` during ticking).
            "flowEvalTick" => {
                let mut runtime = self.runtime.borrow_mut();
                let mut host = host_from_fixture_with_driver(fixture, Some(&runtime.eval_driver));
                let more = runtime.eval_driver.tick(&mut host);
                if !more {
                    refresh_all_caches(&mut runtime, fixture, &doc.projection.generation);
                }
                ActionEmit {
                    effects: if more { vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }] } else { Vec::new() },
                    ..ActionEmit::default()
                }
            }
            _ => ActionEmit::default(),
        }
    }

    /// 🧵️ Arms a `flowEvalTick` chain whenever the main fixture has pending (uncomputed) nodes —
    /// covers every mutation path (edits, undo/redo, example load, remote operations) in one place.
    fn pending_effects(
        &self,
        doc: &DocumentView<'_, Procedural3dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> Vec<semio_framework_core::kernel::HostEffect> {
        let mut runtime = self.runtime.borrow_mut();
        let host = host_from_fixture_with_driver(&doc.projection.fixture, Some(&runtime.eval_driver));
        if runtime.eval_driver.sync(&host) {
            vec![semio_framework_core::kernel::HostEffect::DispatchAction { action: "flowEvalTick".into(), args: None, delay_ms: 0 }]
        } else {
            Vec::new()
        }
    }

    fn render(
        &self,
        body_key: &str,
        doc: &DocumentView<'_, Procedural3dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> UiNode {
        let envelope = play_view(doc.projection, &self.runtime.borrow());
        let host = host_from_fixture(&envelope.fixture);
        let labels = procedural3d_labels(view_state);
        let active_utility = view_state.active_utility_id.as_deref().unwrap_or(PROCEDURAL_3D_TRANSFORM_UTILITY_DEFAULT);
        match body_key {
            PROCEDURAL_3D_PLAY_BODY_MAIN => {
                let (nodes_json, edges_json) = fixture_to_workflow(&host.dag.fixture);
                let viewport_json =
                    serde_json::to_string(&envelope.runtime.camera).unwrap_or_else(|_| r#"{"x":0,"y":0,"zoom":1}"#.into());
                let selection_json = if envelope.runtime.selected_node_ids.is_empty() {
                    None
                } else {
                    serde_json::to_string(&envelope.runtime.selected_node_ids).ok()
                };
                let flow_extras = flow_backed_node_graph_extras(&envelope.fixture, &envelope.runtime.lod_mode, 0.0, true, false, ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(&envelope.runtime.eval_driver));
                let context_menu_json = serde_json::to_string(&json!([{
                    "id": "delete-selection",
                    "label": labels.delete_selection,
                    "icon": "trash",
                    "action": "nodeGraphEdit",
                    "args": { "operations": [{ "operation": "deleteSelection" }] },
                    "destructive": true,
                }]))
                .ok();
                build_node_graph_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_MAIN,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    NodeGraphScene {
                        editable: Some(true),
                        operators_json: flow_extras.operators_json,
                        capabilities_json: flow_extras.capabilities_json,
                        lod_json: flow_extras.lod_json,
                        fixture_json: flow_extras.fixture_json,
                        eval_json: flow_extras.eval_json,
                        computing_json: flow_extras.computing_json,
                        selection_json,
                        hover_json: node_graph_hover_json(&envelope.runtime),
                        context_menu_json,
                        ..NodeGraphScene::base(nodes_json, edges_json, viewport_json)
                    },
                )
            }
            PROCEDURAL_3D_PLAY_BODY_PREVIEW => {
                let (meshes_json, instances_json) = preview_payload_cached(&envelope.runtime, &envelope.fixture);
                build_world_3d_scene(
                    PROCEDURAL_3D_PLAY_SURFACE_PREVIEW,
                    PROCEDURAL_3D_PLAY_APP_ID,
                    ui_wgpu::World3dScene {
                        status_json: envelope.runtime.eval_driver.pending().then(|| r#"{"computing":true}"#.to_string()),
                        ..world3d_scene(
                            preview_camera_json(&envelope.runtime),
                            meshes_json,
                            instances_json,
                            preview_selection_json(&envelope.runtime, active_utility),
                            &envelope.runtime.sun,
                        )
                    },
                )
            }
            PROCEDURAL_3D_PLAY_BODY_GENERATIONS => render_generate_generations(&envelope),
            PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM => render_generate_form(&envelope, labels),
            PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW => render_generate_preview(&envelope, labels, active_utility),
            PROCEDURAL_3D_PLAY_BODY_DOCUMENT => {
                build_document_tree(&envelope.fixture, &envelope.runtime.selected_node_ids, labels)
            }
            PROCEDURAL_3D_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            PROCEDURAL_3D_PLAY_BODY_INSPECTION => {
                build_inspector_tree(&envelope.fixture, &envelope.runtime.selected_node_ids, labels)
            }
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_measures(
        &self,
        _doc: &DocumentView<'_, Procedural3dDocument>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        _view_state: &ViewState,
    ) -> std::collections::HashMap<String, Vec<WindowMeasure>> {
        let runtime = self.runtime.borrow();
        let measures = vec![world3d_sun_measures("procedural3d", &runtime.sun, procedural_action)];
        std::collections::HashMap::from([
            (PROCEDURAL_3D_PLAY_WINDOW_MAIN.to_string(), vec![procedural3d_lod_measure(&runtime.lod_mode)]),
            (PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.to_string(), measures.clone()),
            (PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.to_string(), measures),
        ])
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = procedural3d_labels(view_state);
        let is_de = semio_framework_plugin::is_de_locale(view_state);
        semio_framework_plugin::AppLabelsOverlay::default()
            .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_MAIN, labels.window_flow)
            .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, labels.window_preview)
            .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS, labels.window_generations)
            .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM, labels.window_generate_form)
            .window_kind_label(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW, labels.window_generate_preview)
            .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
            .mode_label("generate", if is_de { "Generieren" } else { "Generate" })
            .action_labels(procedural3d_action_labels(is_de))
            .utility_labels(procedural3d_utility_labels(is_de))
            .example_labels(semio_framework_plugin::localized_label_map(is_de, &[
                (PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column", "Sechseckige Pilzsäule"),
                (PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume", "Rechteck-Extrusionsvolumen"),
                (PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus", "Kugel mit Torus geschnitten"),
            ]))
    }
}

/// 🎯️ Parses `nodeGraphHover` args into the hovered widget id — accepts `null`, `{ nodeId }`, or a
/// `DagChannelRef` `{ widgetId, port, direction }` payload from the flow graph session.
fn parse_node_graph_hover_widget_id(args: Option<&Value>) -> Option<Option<String>> {
    let hover = args?.get("hoverJson")?;
    if hover.is_null() {
        return Some(None);
    }
    let parsed = if let Some(text) = hover.as_str() {
        serde_json::from_str::<Value>(text).unwrap_or_else(|_| Value::String(text.to_string()))
    } else {
        hover.clone()
    };
    Some(
        parsed
            .get("widgetId")
            .or_else(|| parsed.get("nodeId"))
            .and_then(|value| value.as_str())
            .map(str::to_string),
    )
}

fn node_graph_hover_json(runtime: &Procedural3dRuntime) -> Option<String> {
    runtime.hovered_node_id.as_ref().map(|id| json!({ "nodeId": id }).to_string())
}

fn node_graph_selection_ids(args: Option<&Value>) -> Vec<String> {
    if let Some(ids) = args
        .and_then(|value| value.get("nodeIds"))
        .and_then(|value| serde_json::from_value(value.clone()).ok())
    {
        return ids;
    }
    selection_ids(args)
}

/// 🎯️ `semio_framework_plugin::selection_ids`'s "ids" array plus a singular "nodeId" fallback —
/// this app's actions accept either shape depending on the caller.
fn selection_ids(args: Option<&Value>) -> Vec<String> {
    let ids = semio_framework_plugin::selection_ids(args);
    if !ids.is_empty() {
        return ids;
    }
    args.and_then(|value| value.get("nodeId"))
        .and_then(|value| value.as_str())
        .map(|id| vec![id.to_string()])
        .unwrap_or_default()
}
//#endregion 🔖️Procedural3dPlayApp

//#region 🔖️Manifest
pub fn create_procedural3d_app() -> App {
    App::from_builder(
        App::builder(PROCEDURAL_3D_PLAY_APP_ID, "Procedural 3D").document(["semio", "procedural", "3d"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.procedural".into(),
                name: "3D Procedural".into(),
                source_format: "procedural.3d".into(),
                component_kind: "procedural3d".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
                schema: "procedural.3d".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("workflow")
            .mode("edit", "Edit", "square-pen")
            .mode("generate", "Generate", "sparkles")
            .default_mode_id("edit")
            .mode_layout("generate", "procedural3d-generate")
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_MAIN,
                "Flow",
                PROCEDURAL_3D_PLAY_BODY_MAIN,
                SurfaceKind::NodeGraph,
                "flow-graph",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_PREVIEW,
                "Preview",
                PROCEDURAL_3D_PLAY_BODY_PREVIEW,
                SurfaceKind::World3d,
                "preview",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS,
                "Generations",
                PROCEDURAL_3D_PLAY_BODY_GENERATIONS,
                SurfaceKind::Canvas2d,
                "sparkles",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM,
                "Form",
                PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM,
                SurfaceKind::Canvas2d,
                "clipboard-list",
            )
            .window_kind(
                PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW,
                "Preview",
                PROCEDURAL_3D_PLAY_BODY_GENERATE_PREVIEW,
                SurfaceKind::World3d,
                "preview",
            )
            .default_layout(create_default_layout(
                &[PROCEDURAL_3D_PLAY_WINDOW_MAIN.into(), PROCEDURAL_3D_PLAY_WINDOW_PREVIEW.into()],
                "row",
                Some(&[68.0, 32.0]),
                Some(&["Flow".into(), "Preview".into()]),
            ))
            .named_layout(create_named_layout(
                "procedural3d-generate",
                "Generate",
                create_default_layout(
                    &[
                        PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS.into(),
                        PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM.into(),
                        PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW.into(),
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
                PROCEDURAL_3D_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                PROCEDURAL_3D_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                PROCEDURAL_3D_PLAY_BODY_INSPECTION,
            )
            // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
            .operation("setActiveExample", "Set Active Example")
            .operation("nodeGraphEdit", "Edit Graph")
            .operation("deleteSelection", "Delete Selection")
            .operation("removeWidget", "Remove Widget")
            .operation("moveMediaNode", "Move Node")
            .operation("addWidget", "Add Widget")
            .operation("patchFlowWidgets", "Patch Flow Widgets")
            .operation("reorganize", "Reorganize")
            .operation("translateSelection", "Translate Selection")
            .operation("rotateSelection", "Rotate Selection")
            .operation("scaleSelection", "Scale Selection")
            .operation("addGeneration", "Add Generation")
            .operation("removeGeneration", "Remove Generation")
            .operation("renameGeneration", "Rename Generation")
            .operation("updateGenerationValues", "Update Generation Values")
            // 👁️ Ephemeral view actions — selection, hover, world picking, graph camera, sun/LOD/show-mode display toggles, preview camera (emit no operations).
            .view_action("nodeGraphViewport", "Set Viewport")
            .view_action("setSelection", "Set Selection")
            .view_action("selectNode", "Select Node")
            .view_action("nodeGraphSelect", "Node Graph Select")
            .view_action("nodeGraphHover", "Node Graph Hover")
            .view_action("setHover", "Set Hover")
            .view_action("worldPointerDown", "World Pointer Down")
            .view_action("graphPointerDown", "Graph Pointer Down")
            .view_action("worldSelect", "World Select")
            .view_action("worldHover", "World Hover")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("setLodMode", "Set LOD Mode")
            .view_action("setShowMode", "Set Show Mode")
            .view_action("toggleSun", "Toggle Sun")
            .view_action("setSunAzimuth", "Set Sun Azimuth")
            .view_action("setSunElevation", "Set Sun Elevation")
            .view_action("setSunIntensity", "Set Sun Intensity")
            .view_action("setCamera", "Set Camera")
            .view_action("selectGeneration", "Select Generation")
            // 📝️ Staged argument forms for the palette-visible actions (defaults materialized host-side).
            .action_args("addWidget", vec![
                ActionArgDef::select("kind", "Kind", vec![
                    ActionArgOption::new("neuron", "Neuron"),
                    ActionArgOption::new("inputSlider", "Slider"),
                    ActionArgOption::new("inputNote", "Note"),
                    ActionArgOption::new("outputPreview", "Preview"),
                ]).default_value("inputSlider"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column"),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume"),
                    ActionArgOption::new(PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus"),
                ]).required(),
            ])
            // 🧰️ Transform gumball — an exclusive utility group scoped to the 3D preview window (active utility is host-owned).
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", "Move", "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", "Rotate", "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", "Scale", "maximize-2") })
            .window_kind_utilities(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW, vec!["move".into(), "rotate".into(), "scale".into()])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example(PROCEDURAL_EXAMPLE_HEX_COLUMN, "Hexagonal Mushroom Column", procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_HEX_COLUMN), "hexagon")
    .example(PROCEDURAL_EXAMPLE_RECT_EXTRUDE, "Rectangle Extrude Volume", procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_RECT_EXTRUDE), "box")
    .example(PROCEDURAL_EXAMPLE_SPHERE_TORUS, "Sphere Cut With Torus", procedural_3d_engine::example_document_json(PROCEDURAL_EXAMPLE_SPHERE_TORUS), "circle")
    .workflow("procedural3d", "Procedural 3D", "brep")
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use store::create_document_envelope;
    use procedural_3d_engine::empty_procedural3d_projection;
    use procedural_3d_op::{Procedural3dEnvelope, Procedural3dStore};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Procedural3dDocumentVcs {
        store: RefCell<Procedural3dStore>,
    }

    #[wasm_bindgen]
    impl Procedural3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Procedural3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Procedural3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Procedural3dStore::new(envelope)
                }
                None => Procedural3dStore::new(create_document_envelope(PROCEDURAL_3D_SCHEMA, "procedural3d", empty_procedural3d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

    fn meta(actor: &str) -> ActionMeta {
        testkit::meta(actor)
    }

    fn new_app() -> VcsDocumentApp<Procedural3dPlayApp> {
        testkit::new_app::<Procedural3dPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so default-materialization + kind discipline run.
    fn new_app_with_registry() -> VcsDocumentApp<Procedural3dPlayApp> {
        testkit::new_app_with_registry::<Procedural3dPlayApp>(create_procedural3d_app)
    }

    /// 🧵️ A `flowEvalTick` chain self-dispatches via `requestedEffects`, which only the JS renderer
    /// drains in production (see `applyHostEffects`'s `dispatchAction` branch) — a unit test has
    /// to do that draining itself. Mirrors `pending_effects`'s own arming logic so tests don't need
    /// to know whether a mutation left the driver already ticking.
    fn drain_flow_eval_ticks(app: &mut VcsDocumentApp<Procedural3dPlayApp>) {
        // 🧵️ Arms the chain if it isn't already (a no-operation if a caller already armed it — `sync`
        // correctly declines to re-arm one already scheduled, so this must not gate on its return
        // value). A "flowEvalTick" dispatched with nothing pending is a harmless, immediate no-operation
        // (`evaluate_step`'s own early-return), so always ticking at least once is safe.
        app.pending_effects(&ViewState::default());
        for _ in 0..1000 {
            let result = app.handle_action("flowEvalTick", None, &ViewState::default(), &meta("local")).expect("flowEvalTick");
            if !result.requested_effects.iter().any(|effect| matches!(effect, semio_framework_core::kernel::HostEffect::DispatchAction { action, .. } if action == "flowEvalTick")) {
                return;
            }
        }
        panic!("flowEvalTick chain did not converge within 1000 ticks");
    }

    #[test]
    fn set_active_example_arg_form_materializes_into_operations() {
        let mut app = new_app_with_registry();
        // The required `exampleId` staged arg drives an operation that rewrites the fixture.
        app.handle_action(
            "setActiveExample",
            Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("set example");
        let projection = app.projection().expect("projection");
        assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.prim3d.sphere")));
    }

    #[test]
    fn node_graph_hover_updates_preview_selection_and_graph_scene() {
        let mut app = new_app();
        app.handle_action(
            "nodeGraphHover",
            Some(&json!({ "hoverJson": { "nodeId": "extrude" } })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("node graph hover");
        let preview = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview");
        let preview_json = serde_json::to_string(&preview).expect("preview json");
        assert!(preview_json.contains(r#""hoveredId":"extrude""#) || preview_json.contains(r#""hoveredId": "extrude""#));
        let graph = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("graph");
        let graph_json = serde_json::to_string(&graph).expect("graph json");
        assert!(graph_json.contains(r#""hoverJson":"{\"nodeId\":\"extrude\"}""#) || graph_json.contains(r#""hoverJson": "{\"nodeId\":\"extrude\"}""#));
    }

    #[test]
    fn set_hover_from_world_updates_preview_and_graph_scene() {
        let mut app = new_app();
        app.handle_action("setHover", Some(&json!({ "objectId": "extrude" })), &ViewState::default(), &meta("local"))
            .expect("set hover");
        let preview = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview");
        let preview_json = serde_json::to_string(&preview).expect("preview json");
        assert!(preview_json.contains("extrude"));
        app.handle_action("setHover", None, &ViewState::default(), &meta("local")).expect("clear hover");
        let cleared = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("preview cleared");
        let cleared_json = serde_json::to_string(&cleared).expect("cleared json");
        assert!(!cleared_json.contains(r#""hoveredId":"extrude""#));
    }

    #[test]
    fn set_active_utility_switch_clears_scratch_and_emits_no_operations() {
        let mut app = new_app_with_registry();
        app.handle_action("worldHover", Some(&json!({ "id": "extrude" })), &ViewState::default(), &meta("local")).expect("hover");
        let before = app.projection().expect("projection");
        // Switching the gumball utility is the framework-injected View action: it clears scratch and emits no operations.
        let result = app
            .handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "rotate" })), &ViewState::default(), &meta("local"))
            .expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert_eq!(app.projection().expect("projection"), before, "utility switching records no history entry");
    }

    #[test]
    fn gumball_drag_coalesces_multi_tick_translate_into_one_edit() {
        let mut app = new_app();
        let before_widgets = app.projection().expect("projection").fixture.widgets.len();
        // A whole gumball drag (three ticks, same coalesce key) folds into ONE undoable edit, not one-operation-per-tick.
        for dx in [1.0, 1.0, 1.0] {
            app.handle_action(
                "translateSelection",
                Some(&json!({ "ids": ["extrude"], "dx": dx, "dy": 0.0, "dz": 0.0 })),
                &ViewState::default(),
                &meta("local"),
            )
            .expect("drag tick");
        }
        let transform_id = "extrude__gumball_translate";
        let dragged = app.projection().expect("projection");
        assert_eq!(gumball_widget_offset(&host_from_fixture(&dragged.fixture), transform_id), [3.0, 0.0, 0.0], "the three ticks accumulate on one transform node");
        // Undoing the coalesced drag reverts the whole gesture in a single step (splice + all ticks).
        app.handle_action("undo", None, &ViewState::default(), &meta("local")).expect("undo");
        let restored = app.projection().expect("projection");
        assert_eq!(restored.fixture.widgets.len(), before_widgets, "one undo removes the entire coalesced gumball edit");
        assert!(!restored.fixture.widgets.iter().any(|widget| widget_id(widget) == transform_id), "the spliced transform node is gone after a single undo");
    }

    fn slider_value(projection: &Procedural3dDocument, id: &str) -> Option<f64> {
        projection.fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id: widget_id, value, .. } if widget_id == id => Some(*value),
            _ => None,
        })
    }

    #[test]
    fn renders_node_graph_scene() {
        let mut app = new_app();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("node-graph"));
    }

    #[test]
    fn main_graph_scene_exports_flow_backed_node_graph_fields() {
        let mut app = new_app();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        let value: Value = serde_json::from_str(&json).expect("ui node json");
        let graph = value.get("nodeGraph").expect("nodeGraph");
        assert!(graph.get("fixtureJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("flow.fixture")));
        assert!(graph.get("operatorsJson").and_then(|v| v.as_str()).is_some_and(|s| s.contains("math.add") || s.contains("brep.")));
        let capabilities = graph.get("capabilitiesJson").and_then(|v| v.as_str()).unwrap_or_default();
        assert!(capabilities.contains("flow"), "missing flow engine capability: {capabilities}");
    }

    #[test]
    fn set_lod_mode_is_a_view_action_with_no_document_operations() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        app.handle_action("setLodMode", Some(&json!({ "value": "wireframe" })), &ViewState::default(), &meta("local")).expect("lod");
        assert_eq!(app.projection().expect("projection"), before, "setLodMode must not mutate the document");
    }

    #[test]
    fn sun_measures_are_exposed_on_preview_windows() {
        let mut app = new_app();
        let measures = app.window_measures(&ViewState::default());
        assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_PREVIEW));
        assert!(measures.contains_key(PROCEDURAL_3D_PLAY_WINDOW_GENERATE_PREVIEW));
        // 👁️ Sun toggling is a view action: it must not record a document operation.
        let before = app.projection().expect("projection");
        app.handle_action("toggleSun", None, &ViewState::default(), &meta("local")).expect("toggle sun");
        assert_eq!(app.projection().expect("projection"), before, "toggleSun must not mutate the document");
    }

    #[test]
    fn set_active_example_loads_sphere_fixture() {
        let mut app = new_app();
        app.handle_action(
            "setActiveExample",
            Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("set example");
        let projection = app.projection().expect("projection");
        assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.prim3d.sphere")));
    }

    #[test]
    fn sphere_cut_example_preview_renders_meshes() {
        // 🧵️ Loading the example never evaluates synchronously anymore (see `pending_effects`) —
        // draining the `flowEvalTick` chain here simulates what the JS renderer's `applyHostEffects`
        // does automatically after every refresh, so the render below sees the real evaluated
        // geometry rather than the cold-start placeholder mesh.
        let mut app = new_app();
        app.handle_action(
            "setActiveExample",
            Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("set example");
        drain_flow_eval_ticks(&mut app);
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("render");
        let parsed: ui_wgpu::UiNode = serde_json::from_str(&serde_json::to_string(&node).unwrap()).expect("preview ui json");
        match parsed {
            ui_wgpu::UiNode::ComponentScene(scene) => {
                let world = scene.world_3d.expect("world_3d payload");
                assert_ne!(world.meshes_json, "[]");
                assert_ne!(world.instances_json, "[]");
            }
            other => panic!("expected component scene, got {other:?}"),
        }
    }

    #[test]
    fn sphere_cut_example_computing_chrome_clears_once_ticks_converge() {
        let mut app = new_app();
        app.handle_action(
            "setActiveExample",
            Some(&json!({ "exampleId": PROCEDURAL_EXAMPLE_SPHERE_TORUS })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("set example");
        let main_graph = |app: &mut VcsDocumentApp<Procedural3dPlayApp>| -> ui_wgpu::NodeGraphScene {
            let node = app.render(PROCEDURAL_3D_PLAY_BODY_MAIN, None, &ViewState::default()).expect("render");
            match serde_json::from_str::<ui_wgpu::UiNode>(&serde_json::to_string(&node).unwrap()).expect("graph ui json") {
                ui_wgpu::UiNode::ComponentScene(scene) => scene.node_graph.expect("node_graph payload"),
                other => panic!("expected component scene, got {other:?}"),
            }
        };
        // 🧵️ In production, `pending_effects` runs after every `refreshUi` pass — a test driving
        // `render` directly has to call it explicitly to arm the driver the same way. Before any
        // tick runs, the graph must flag the cut node (and its downstream preview) as computing —
        // this is what drives the dag canvas's animated loading border.
        assert!(!app.pending_effects(&ViewState::default()).is_empty(), "loading the example must arm a tick chain");
        assert!(main_graph(&mut app).computing_json.is_some(), "pending nodes must be reported before the chain runs");
        drain_flow_eval_ticks(&mut app);
        assert!(main_graph(&mut app).computing_json.is_none(), "computing chrome clears once the chain converges");
        app.handle_action(
            "patchFlowWidgets",
            Some(&json!({ "widgetIds": ["slider_2"], "field": "value", "value": 4.5 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("patch slider");
        assert!(!app.pending_effects(&ViewState::default()).is_empty(), "slider mutation must re-arm evaluation");
        let computing = main_graph(&mut app).computing_json.expect("computing chrome after slider edit");
        assert!(
            computing.contains("brep_prim3d_sphere_3") || computing.contains("brep_bool_cut_5"),
            "downstream sphere/cut branch must be marked computing, got {computing}"
        );
    }

    #[test]
    fn patch_flow_widgets_edits_slider_value() {
        let mut app = new_app();
        app.handle_action(
            "patchFlowWidgets",
            Some(&json!({ "widgetIds": ["height"], "field": "value", "value": 9.5 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("patch");
        assert_eq!(slider_value(&app.projection().expect("projection"), "height"), Some(9.5));
    }

    #[test]
    fn renders_world_preview_scene() {
        let mut app = new_app();
        drain_flow_eval_ticks(&mut app);
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_PREVIEW, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("world-3d"));
        let parsed: ui_wgpu::UiNode = serde_json::from_str(&json).expect("preview ui json");
        match parsed {
            ui_wgpu::UiNode::ComponentScene(scene) => {
                assert_eq!(scene.component_kind, SurfaceKind::World3d);
                let world = scene.world_3d.expect("world_3d payload");
                assert_ne!(world.meshes_json, "[]");
                assert_ne!(world.instances_json, "[]");
            }
            other => panic!("expected component scene, got {other:?}"),
        }
    }

    #[test]
    fn add_widget_action_appends_widget() {
        let mut app = new_app();
        let before = app.projection().expect("projection").fixture.widgets.len();
        app.handle_action("addWidget", Some(&json!({ "kind": "inputNote" })), &ViewState::default(), &meta("local")).expect("add");
        assert!(app.projection().expect("projection").fixture.widgets.len() > before);
    }

    #[test]
    fn generate_mode_renders_surfaces() {
        let mut app = new_app();
        let generations = app.render(PROCEDURAL_3D_PLAY_BODY_GENERATIONS, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&generations).unwrap().contains("addGeneration"));
    }

    #[test]
    fn add_generation_records_an_undoable_generation_operation() {
        let mut app = new_app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "addGeneration",
            None,
            |app| app.projection().expect("projection").generation.generations.len(),
            0,
            1,
        );
    }

    #[test]
    fn translate_selection_persists_transform_into_flow_graph() {
        let mut app = new_app();
        let before = app.projection().expect("projection");
        assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
        app.handle_action(
            "translateSelection",
            Some(&json!({ "ids": ["extrude"], "dx": 1.0, "dy": 2.0, "dz": 3.0 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("translate");
        let projection = app.projection().expect("projection");
        let transform_id = "extrude__gumball_translate";
        let transform = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
        assert!(matches!(transform, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.translate"));
        let offset = gumball_widget_offset(&host_from_fixture(&projection.fixture), transform_id);
        assert_eq!(offset, [1.0, 2.0, 3.0]);
        let source = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == "extrude").expect("source widget");
        assert!(matches!(source, Widget::Neuron { preview, .. } if !*preview), "source preview should turn off once gumball-transformed");
        assert!(projection.fixture.synapses.iter().any(|synapse| synapse.from == transform_id && synapse.to == "column-preview"), "downstream rewired through transform node");
        assert!(!projection.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"), "old direct edge removed");

        // Re-grabbing the same transform accumulates the delta instead of creating a second node.
        app.handle_action(
            "translateSelection",
            Some(&json!({ "ids": [transform_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("translate again");
        let projection2 = app.projection().expect("projection");
        assert_eq!(projection2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
        assert_eq!(gumball_widget_offset(&host_from_fixture(&projection2.fixture), transform_id), [2.0, 2.0, 3.0]);
    }

    #[test]
    fn rotate_and_scale_selection_persist_into_flow_graph() {
        let mut app = new_app();
        app.handle_action(
            "rotateSelection",
            Some(&json!({ "ids": ["extrude"], "angle": std::f64::consts::FRAC_PI_2 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("rotate");
        let rotated = app.projection().expect("projection");
        let rotate_id = "extrude__gumball_rotate";
        assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == rotate_id && neuron_kind == "brep.xform.rotate")));
        assert_eq!(gumball_widget_number_param(&host_from_fixture(&rotated.fixture), rotate_id, "angle", 0.0), std::f64::consts::FRAC_PI_2);

        let mut scale_app = new_app();
        scale_app.handle_action(
            "scaleSelection",
            Some(&json!({ "ids": ["extrude"], "sx": 2.0, "sy": 2.0, "sz": 2.0 })),
            &ViewState::default(),
            &meta("local"),
        )
        .expect("scale");
        let scaled = scale_app.projection().expect("projection");
        let scale_id = "extrude__gumball_scale";
        assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == scale_id && neuron_kind == "brep.xform.scale")));
        assert_eq!(gumball_widget_number_param(&host_from_fixture(&scaled.fixture), scale_id, "factor", 1.0), 2.0);
    }

    #[test]
    fn undo_redo_round_trips_flow_graph_edits() {
        let mut app = new_app();
        let before = app.projection().expect("projection").fixture.widgets.len();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "addWidget",
            Some(&json!({ "kind": "inputNote" })),
            |app| app.projection().expect("projection").fixture.widgets.len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn remove_widget_action_deletes_by_id_and_supports_undo() {
        let mut app = new_app();
        assert!(app.projection().expect("projection").fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"));
        testkit::assert_undo_redo_round_trip(
            &mut app,
            "removeWidget",
            Some(&json!({ "widgetId": "sides" })),
            |app| app.projection().expect("projection").fixture.widgets.iter().any(|widget| widget_id(widget) == "sides"),
            true,
            false,
        );
    }

    #[test]
    fn two_instances_converge_disjoint_widget_moves() {
        let widgets: Vec<String> = new_app()
            .projection()
            .expect("projection")
            .fixture
            .widgets
            .iter()
            .map(|widget| widget_id(widget).to_string())
            .collect();
        assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
        let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
        testkit::assert_two_instances_converge::<Procedural3dPlayApp, (Option<f64>, Option<f64>)>(
            "mem://procedural3d-convergence",
            ("moveMediaNode", Some(&json!({ "nodeId": w0, "x": 111.0, "y": 5.0 }))),
            ("moveMediaNode", Some(&json!({ "nodeId": w1, "x": 222.0, "y": 6.0 }))),
            move |app| {
                let layout = &app.projection().expect("projection").fixture.layout;
                (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
            },
        );
    }

    #[test]
    fn procedural3d_labels_resolve_native_english_by_default() {
        let mut app = new_app();
        let node = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Widgets\""));
        assert!(json.contains("\"Slider\""));
        assert!(!json.contains("Elemente"));
    }

    #[test]
    fn procedural3d_labels_translate_catalogue_and_inspector_in_german() {
        let mut app = new_app();
        let view_state = ViewState { locale: Some("de".into()), ..ViewState::default() };
        let catalogue = app.render(PROCEDURAL_3D_PLAY_BODY_CATALOGUE, None, &view_state).expect("render");
        let catalogue_json = serde_json::to_string(&catalogue).unwrap();
        assert!(catalogue_json.contains("\"Elemente\""));
        assert!(catalogue_json.contains("Schieberegler"));
        assert!(!catalogue_json.contains("\"Widgets\""));
        let inspector = app.render(PROCEDURAL_3D_PLAY_BODY_INSPECTION, None, &view_state).expect("render");
        let inspector_json = serde_json::to_string(&inspector).unwrap();
        assert!(inspector_json.contains("Elemente:"));
    }
}
//#endregion 🧪️Tests
