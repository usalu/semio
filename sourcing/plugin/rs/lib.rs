//! 🛒 Sourcing plugin — curate app: handpick and curate 3D object kinds out of a modular catalogue.

use semio_framework_plugin::{
    build_table_scene, build_world_3d_scene, table_row_json, ui_stack_vertical, ui_text,
    world3d_default_camera, world3d_scene, world3d_selection_json, ActionArgDef, ActionArgOption,
    ActionDescriptor, ActionEmit, App, Contribution, DocumentApp, DocumentView, SurfaceKind, TableCell,
    TableScene, UiInputNode, UiNode, UiNumberStepperNode, UiSelectItem, UiSelectNode, UiToggleNode,
    UiTreeItemAction, ViewState, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot,
    WindowLayoutStackNode, WindowLayoutWindowNode, WorldSunConfig, ActionDefinition, ActionKind,
};
use semio_framework_core::mesh_from_indexed;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sourcing_curate::{
    grid_placement, grid_scale, mesh_spec_for, sourcing_modules, typology_flatten, CurateDocument,
    ObjectKind, SortDirection, SourcingOp, TableSort, TypologyNode, SOURCING_CURATE_SCHEMA,
};
use std::collections::HashSet;

//#region 🔖Constants
const SOURCING_CURATE_APP_ID: &str = "sourcing-curate";
const SOURCING_CONTROLLER_ID: &str = "sourcing-curate";
const WINDOW_POOL: &str = "sourcing-pool";
const WINDOW_CURATED: &str = "sourcing-curated";
const WINDOW_PREVIEW: &str = "sourcing-preview";
const WINDOW_GRID: &str = "sourcing-grid";
const BODY_POOL: &str = "sourcing.pool";
const BODY_CURATED: &str = "sourcing.curated";
const BODY_PREVIEW: &str = "sourcing.preview";
const BODY_GRID: &str = "sourcing.grid";
const SURFACE_POOL: &str = "sourcing.pool.table";
const SURFACE_CURATED: &str = "sourcing.curated.table";
const SURFACE_PREVIEW: &str = "sourcing.preview.world";
const SURFACE_GRID: &str = "sourcing.grid.world";
const SOURCING_DRAG_MIME: &str = "application/x-semio-sourcing-object";
const GRID_CELL: f64 = 2.0;
const DEMO_STOCK_EXAMPLE_ID: &str = "demo-stock";
const EMPTY_EXAMPLE_ID: &str = "empty-curation";
const DEMO_STOCK_JSON: &str = include_str!("../../curate/example/demo-stock.curate.json");
const EMPTY_CURATION_JSON: &str = include_str!("../../curate/example/empty-curation.curate.json");
//#endregion 🔖Constants

//#region 🔖Document
fn default_document() -> CurateDocument {
    serde_json::from_str(DEMO_STOCK_JSON).unwrap_or_default()
}

fn empty_document() -> CurateDocument {
    serde_json::from_str(EMPTY_CURATION_JSON).unwrap_or_default()
}

fn sourcing_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: SOURCING_CONTROLLER_ID.into(), action: action.into(), args }
}

fn selected_ids(document: &CurateDocument) -> Vec<String> {
    document.runtime.selected_object_id.clone().into_iter().collect()
}

fn selection_json_for(document: &CurateDocument) -> String {
    json!({ "selectedIds": selected_ids(document) }).to_string()
}
//#endregion 🔖Document

//#region 🔖Contributions
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PluginContributionEntry {
    #[allow(dead_code)]
    plugin_id: String,
    contribution: Contribution,
}

/// 🧩 One module's typology + catalogue kinds, resolved either from a contributed plugin or, as a
/// standalone fallback, straight from `sourcing_curate`'s own built-in module registry.
struct ModuleCatalogue {
    module_id: String,
    label: String,
    typology: TypologyNode,
    kinds: Vec<ObjectKind>,
}

fn parse_module_catalogues(view_state: &ViewState) -> Vec<ModuleCatalogue> {
    view_state
        .contributions_json
        .as_ref()
        .and_then(|json| serde_json::from_str::<Vec<PluginContributionEntry>>(json).ok())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|entry| {
            let Contribution::SourcingModule { app_id, module_id, label, typology_json, kinds_json, .. } = entry.contribution else {
                return None;
            };
            if app_id != SOURCING_CURATE_APP_ID {
                return None;
            }
            let typology = serde_json::from_str(&typology_json).ok()?;
            let kinds = serde_json::from_str(&kinds_json).ok()?;
            Some(ModuleCatalogue { module_id, label, typology, kinds })
        })
        .collect()
}

/// 🧩 Contributed module catalogues if any plugin has contributed, else the built-in modules — so the
/// filter UI and `stockFromCatalogue` work standalone even before contributor plugins are wired up.
fn available_modules(view_state: &ViewState) -> Vec<ModuleCatalogue> {
    let contributed = parse_module_catalogues(view_state);
    if !contributed.is_empty() {
        return contributed;
    }
    sourcing_modules()
        .into_iter()
        .map(|module| ModuleCatalogue {
            module_id: module.module_id().to_string(),
            label: module.label().to_string(),
            typology: module.typology(),
            kinds: module.demo_kinds(),
        })
        .collect()
}
//#endregion 🔖Contributions

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the curate app; one field per label makes every locale combination compile-checked.
struct SourcingLabels {
    window_pool: &'static str,
    window_curated: &'static str,
    window_preview: &'static str,
    window_grid: &'static str,
    mode_curate: &'static str,
    search_placeholder: &'static str,
    all_typologies: &'static str,
    col_name: &'static str,
    col_module: &'static str,
    col_typology: &'static str,
    col_availability: &'static str,
    col_curated: &'static str,
    col_count: &'static str,
    remove: &'static str,
    no_selection: &'static str,
}

const SOURCING_LABELS_NATIVE_EN: SourcingLabels = SourcingLabels {
    window_pool: "Pool",
    window_curated: "Curated",
    window_preview: "Preview",
    window_grid: "Grid",
    mode_curate: "Curate",
    search_placeholder: "Search…",
    all_typologies: "All Typologies",
    col_name: "Name",
    col_module: "Module",
    col_typology: "Typology",
    col_availability: "Availability",
    col_curated: "Curated",
    col_count: "Count",
    remove: "Remove",
    no_selection: "No selection",
};
const SOURCING_LABELS_NATIVE_DE: SourcingLabels = SourcingLabels {
    window_pool: "Pool",
    window_curated: "Kuratiert",
    window_preview: "Vorschau",
    window_grid: "Raster",
    mode_curate: "Kuratieren",
    search_placeholder: "Suchen…",
    all_typologies: "Alle Typologien",
    col_name: "Name",
    col_module: "Modul",
    col_typology: "Typologie",
    col_availability: "Verfuegbarkeit",
    col_curated: "Kuratiert",
    col_count: "Anzahl",
    remove: "Entfernen",
    no_selection: "Keine Auswahl",
};

/// 🗣️ Resolves the active label set from the shell-provided locale; this app has no reuse-terminology mapping.
fn sourcing_labels(view_state: &ViewState) -> &'static SourcingLabels {
    let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
    if is_de {
        &SOURCING_LABELS_NATIVE_DE
    } else {
        &SOURCING_LABELS_NATIVE_EN
    }
}
//#endregion 🔖Terminology

//#region 🔖Layout
fn sourcing_window(window_kind_id: &str, title: &str) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None }
}

fn sourcing_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode { kind: "stack".into(), size, active_window_kind_id: None, children: vec![sourcing_window(window_kind_id, title)] })
}

/// 🪟 Three-column layout: pool | curated over preview | grid — mirrors `cad_quad_layout`'s pattern.
fn sourcing_three_column_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.34),
                    children: vec![sourcing_stack(WINDOW_POOL, "Pool", None)],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.33),
                    children: vec![sourcing_stack(WINDOW_CURATED, "Curated", Some(0.55)), sourcing_stack(WINDOW_PREVIEW, "Preview", Some(0.45))],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.33),
                    children: vec![sourcing_stack(WINDOW_GRID, "Grid", None)],
                }),
            ],
        }),
    }
}
//#endregion 🔖Layout

//#region 🔖Panels
fn build_filter_bar(document: &CurateDocument, modules: &[ModuleCatalogue], labels: &SourcingLabels) -> UiNode {
    let mut children = vec![UiNode::Input(UiInputNode {
        id: "sourcing-filter-query".into(),
        input_kind: "text".into(),
        value: document.filters.query.clone(),
        placeholder: Some(labels.search_placeholder.into()),
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        on_change: sourcing_action("setFilterQuery", None),
    })];
    for module in modules {
        let pressed = document.filters.module_ids.iter().any(|id| id == &module.module_id);
        children.push(UiNode::Toggle(UiToggleNode {
            id: format!("sourcing-filter-module-{}", module.module_id),
            icon_id: "layers".into(),
            pressed,
            text: Some(module.label.clone()),
            on_change: sourcing_action("setFilterModule", Some(json!({ "moduleId": module.module_id, "enabled": !pressed }))),
        }));
    }
    let mut typology_items = vec![UiSelectItem { value: String::new(), label: labels.all_typologies.into() }];
    for module in modules {
        for (path, label) in typology_flatten(&module.typology) {
            typology_items.push(UiSelectItem { value: path.join("/"), label });
        }
    }
    children.push(UiNode::Select(UiSelectNode {
        id: "sourcing-filter-typology".into(),
        value: document.filters.typology_path.join("/"),
        items: typology_items,
        placeholder: None,
        on_change: sourcing_action("setFilterTypology", None),
    }));
    children.push(UiNode::NumberStepper(UiNumberStepperNode {
        id: "sourcing-filter-min-availability".into(),
        value: document.filters.min_availability as f64,
        step: 1.0,
        uniform: true,
        on_absolute: sourcing_action("setFilterMinAvailability", None),
        on_delta: sourcing_action("setFilterMinAvailability", None),
    }));
    ui_stack_vertical(children)
}

fn pool_columns_json(labels: &SourcingLabels) -> String {
    json!([
        {"id": "name", "label": labels.col_name},
        {"id": "module", "label": labels.col_module, "sortable": true},
        {"id": "typology", "label": labels.col_typology},
        {"id": "availability", "label": labels.col_availability, "sortable": true},
        {"id": "curated", "label": labels.col_curated},
    ])
    .to_string()
}

fn build_pool_table(document: &CurateDocument, labels: &SourcingLabels) -> UiNode {
    let mut filtered = document.filtered_stock();
    if let Some(sort) = &document.filters.sort {
        filtered.sort_by(|a, b| {
            let ordering = match sort.column_id.as_str() {
                "availability" => a.availability.cmp(&b.availability),
                _ => a.name.cmp(&b.name),
            };
            if sort.direction == SortDirection::Desc {
                ordering.reverse()
            } else {
                ordering
            }
        });
    }
    let rows: Vec<Value> = filtered
        .iter()
        .map(|kind| {
            let curated_count = document.curated_count(&kind.id) as f64;
            table_row_json(
                &kind.id,
                Some(&json!({ "objectId": kind.id })),
                &[
                    ("name", TableCell::Text { value: kind.name.clone() }),
                    ("module", TableCell::Text { value: kind.module_id.clone() }),
                    ("typology", TableCell::Text { value: kind.typology_path.join(" / ") }),
                    ("availability", TableCell::Number { value: kind.availability as f64 }),
                    (
                        "curated",
                        TableCell::Stepper {
                            value: curated_count,
                            min: 0.0,
                            max: kind.availability as f64,
                            step: 1.0,
                            action: sourcing_action("curateSetCount", Some(json!({ "objectId": kind.id }))),
                        },
                    ),
                ],
            )
        })
        .collect();
    let mut scene = TableScene::base(pool_columns_json(labels), serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()));
    scene.selection_json = Some(selection_json_for(document));
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action = Some(sourcing_action("dropOnPool", None));
    scene.sort_json = document.filters.sort.as_ref().and_then(|sort| serde_json::to_string(sort).ok());
    build_table_scene(SURFACE_POOL, SOURCING_CONTROLLER_ID, scene)
}

fn build_curated_table(document: &CurateDocument, labels: &SourcingLabels) -> UiNode {
    let columns = json!([
        {"id": "name", "label": labels.col_name},
        {"id": "availability", "label": labels.col_availability},
        {"id": "count", "label": labels.col_count},
        {"id": "actions", "label": ""},
    ])
    .to_string();
    let rows: Vec<Value> = document
        .curated
        .iter()
        .filter_map(|item| {
            let kind = document.stock.iter().find(|kind| kind.id == item.object_id)?;
            Some(table_row_json(
                &kind.id,
                Some(&json!({ "objectId": kind.id })),
                &[
                    ("name", TableCell::Text { value: kind.name.clone() }),
                    ("availability", TableCell::Number { value: kind.availability as f64 }),
                    (
                        "count",
                        TableCell::Stepper {
                            value: item.count as f64,
                            min: 0.0,
                            max: kind.availability as f64,
                            step: 1.0,
                            action: sourcing_action("curateSetCount", Some(json!({ "objectId": kind.id }))),
                        },
                    ),
                    (
                        "actions",
                        TableCell::Buttons {
                            buttons: vec![UiTreeItemAction {
                                icon_id: "trash".into(),
                                label: Some(labels.remove.into()),
                                action: sourcing_action("curateRemove", Some(json!({ "objectId": kind.id }))),
                                reveal_on_hover: None,
                            }],
                        },
                    ),
                ],
            ))
        })
        .collect();
    let mut scene = TableScene::base(columns, serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()));
    scene.selection_json = Some(selection_json_for(document));
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action = Some(sourcing_action("dropOnCurated", None));
    build_table_scene(SURFACE_CURATED, SOURCING_CONTROLLER_ID, scene)
}
//#endregion 🔖Panels

//#region 🔖World3d
fn kind_mesh_json(kind: &ObjectKind) -> Value {
    let spec = mesh_spec_for(&kind.geometry);
    let mesh = mesh_from_indexed(&spec.positions, &spec.normals, &spec.indices);
    json!({ "id": kind.id, "data": mesh })
}

fn instance_json(kind: &ObjectKind, position: [f64; 3], scale: f64, selected: bool) -> Value {
    json!({
        "id": kind.id,
        "meshId": kind.id,
        "position": position,
        "rotation": [0.0, 0.0, 0.0, 1.0],
        "scale": [scale, scale, scale],
        "label": kind.name,
        "selected": selected,
        "hovered": false,
    })
}

fn render_preview(document: &CurateDocument, labels: &SourcingLabels) -> UiNode {
    let Some(kind) = document.runtime.selected_object_id.as_ref().and_then(|id| document.stock.iter().find(|kind| &kind.id == id)) else {
        return ui_text(labels.no_selection);
    };
    let meshes_json = json!([kind_mesh_json(kind)]).to_string();
    let instances_json = json!([instance_json(kind, [0.0, 0.0, 0.0], 1.0, false)]).to_string();
    let mut scene = world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    scene.fit_json = Some(json!({ "enabled": true, "padding": 0.2 }).to_string());
    build_world_3d_scene(SURFACE_PREVIEW, SOURCING_CONTROLLER_ID, scene)
}

fn render_grid(document: &CurateDocument) -> UiNode {
    let filtered = document.filtered_stock();
    let mut seen_mesh_ids = HashSet::new();
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    for (index, kind) in filtered.iter().enumerate() {
        if seen_mesh_ids.insert(kind.id.clone()) {
            meshes.push(kind_mesh_json(kind));
        }
        let (x, z) = grid_placement(filtered.len(), index, GRID_CELL);
        let scale = grid_scale(&kind.geometry, GRID_CELL * 0.8);
        let selected = document.runtime.selected_object_id.as_deref() == Some(kind.id.as_str());
        instances.push(instance_json(kind, [x, 0.0, z], scale, selected));
    }
    let mut scene = world3d_scene(
        world3d_default_camera(),
        json!(meshes).to_string(),
        json!(instances).to_string(),
        world3d_selection_json("rectangle", &selected_ids(document), None),
        &WorldSunConfig::default(),
    );
    scene.fit_json = Some(json!({ "enabled": true, "padding": 0.3 }).to_string());
    build_world_3d_scene(SURFACE_GRID, SOURCING_CONTROLLER_ID, scene)
}
//#endregion 🔖World3d

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/hidden-op declared in `create_sourcing_curate_app`'s
/// static manifest — mirrors `puzzle3d_action_labels`.
fn sourcing_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("stockFromCatalogue", "Stock From Catalogue", "Bestand aus Katalog"),
        ("setDocument", "Set Document", "Dokument festlegen"),
        ("setFilterQuery", "Set Filter Query", "Filterabfrage festlegen"),
        ("setFilterModule", "Set Filter Module", "Filtermodul festlegen"),
        ("setFilterTypology", "Set Filter Typology", "Filtertypologie festlegen"),
        ("setFilterMinAvailability", "Set Filter Min Availability", "Mindestverfuegbarkeit festlegen"),
        ("sortTable", "Sort Table", "Tabelle sortieren"),
        ("curateAdd", "Curate Add", "Kuratierung hinzufuegen"),
        ("curateSetCount", "Curate Set Count", "Kuratierte Anzahl festlegen"),
        ("curateRemove", "Curate Remove", "Kuratierung entfernen"),
        ("dropOnPool", "Drop On Pool", "Auf Pool ablegen"),
        ("dropOnCurated", "Drop On Curated", "Auf Kuratiert ablegen"),
        ("selectRow", "Select Row", "Zeile auswaehlen"),
        ("worldSelect", "World Select", "Welt auswaehlen"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}

/// 🗣️ (utility id) -> localized toolbar-button label; `create_sourcing_curate_app` declares no `.utility(...)`
/// entries, so this is empty — kept for shape-parity with `puzzle3d_utility_labels`.
fn sourcing_utility_labels(_is_de: bool) -> std::collections::HashMap<String, String> {
    std::collections::HashMap::new()
}
//#endregion 🔖CommandLabels

//#region 🔖SourcingCurateApp
pub fn create_sourcing_curate_app() -> App {
    App::from_builder(
        App::builder(SOURCING_CURATE_APP_ID, "Curate")
            .document(["semio", "sourcing", "curate"])
            .icon_id("shopping-cart")
            .mode("curate", "Curate")
            .default_mode_id("curate")
            .window_kind(WINDOW_POOL, "Pool", BODY_POOL, SurfaceKind::Table)
            .window_kind(WINDOW_CURATED, "Curated", BODY_CURATED, SurfaceKind::Table)
            .window_kind(WINDOW_PREVIEW, "Preview", BODY_PREVIEW, SurfaceKind::World3d)
            .window_kind(WINDOW_GRID, "Grid", BODY_GRID, SurfaceKind::World3d)
            .default_layout(sourcing_three_column_layout())
            // 🔧 Every curate edit — filters, sort, selection, curation counts — is persisted in the
            // `CurateDocument` (filters/sort/runtime all live in the document), so each arm emits a
            // whole-document `SetDocument` op and is declared as an Operation, never a View. The
            // filter/sort/selection/table/DnD ids are internal (kept out of the command palette).
            .operation("setActiveExample", "Set Active Example")
            .operation("stockFromCatalogue", "Stock From Catalogue")
            .action_with(hidden_op("setDocument", "Set Document"))
            .action_with(hidden_op("setFilterQuery", "Set Filter Query"))
            .action_with(hidden_op("setFilterModule", "Set Filter Module"))
            .action_with(hidden_op("setFilterTypology", "Set Filter Typology"))
            .action_with(hidden_op("setFilterMinAvailability", "Set Filter Min Availability"))
            .action_with(hidden_op("sortTable", "Sort Table"))
            .action_with(hidden_op("curateAdd", "Curate Add"))
            .action_with(hidden_op("curateSetCount", "Curate Set Count"))
            .action_with(hidden_op("curateRemove", "Curate Remove"))
            .action_with(hidden_op("dropOnPool", "Drop On Pool"))
            .action_with(hidden_op("dropOnCurated", "Drop On Curated"))
            .action_with(hidden_op("selectRow", "Select Row"))
            .action_with(hidden_op("worldSelect", "World Select"))
            // 📝 Staged argument form for the panel-visible example switch.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new(DEMO_STOCK_EXAMPLE_ID, "Demo Stock"),
                    ActionArgOption::new(EMPTY_EXAMPLE_ID, "Empty Curation"),
                ]).default_value(DEMO_STOCK_EXAMPLE_ID),
            ]),
    )
    .example(DEMO_STOCK_EXAMPLE_ID, "Demo Stock", DEMO_STOCK_JSON)
    .example(EMPTY_EXAMPLE_ID, "Empty Curation", EMPTY_CURATION_JSON)
}

/// 🙈 An internal document operation kept out of the command palette — the filter/sort/selection/DnD
/// arms that mutate the persisted `CurateDocument` but are only ever dispatched from window chrome.
fn hidden_op(id: &str, label: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new(id, label, ActionKind::Operation) }
}

#[derive(Default)]
pub struct SourcingCurateApp;

impl DocumentApp for SourcingCurateApp {
    type Projection = CurateDocument;
    type Op = SourcingOp;

    fn app_id(&self) -> &str {
        SOURCING_CURATE_APP_ID
    }

    fn document_schema(&self) -> &str {
        SOURCING_CURATE_SCHEMA
    }

    fn initial_projection(&self) -> CurateDocument {
        default_document()
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, CurateDocument>, view_state: &ViewState) -> ActionEmit<SourcingOp> {
        let mut document = doc.projection.clone();
        match action {
            "setDocument" => {
                if let Some(parsed) = args.and_then(|value| value.get("document")).and_then(|value| serde_json::from_value::<CurateDocument>(value.clone()).ok()) {
                    return ActionEmit::ops(vec![SourcingOp::SetDocument { document: parsed }]);
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let next = if example_id.is_empty() || example_id == EMPTY_EXAMPLE_ID {
                    empty_document()
                } else {
                    default_document()
                };
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document: next }]);
            }
            "stockFromCatalogue" => {
                let existing: HashSet<String> = document.stock.iter().map(|kind| kind.id.clone()).collect();
                for module in available_modules(view_state) {
                    for kind in module.kinds {
                        if !existing.contains(&kind.id) {
                            document.stock.push(kind);
                        }
                    }
                }
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
            }
            "setFilterQuery" => {
                document.filters.query = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or_default().to_string();
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
            }
            "setFilterModule" => {
                if let (Some(module_id), Some(enabled)) =
                    (args.and_then(|value| value.get("moduleId")).and_then(|value| value.as_str()), args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()))
                {
                    if enabled {
                        if !document.filters.module_ids.iter().any(|id| id == module_id) {
                            document.filters.module_ids.push(module_id.to_string());
                        }
                    } else {
                        document.filters.module_ids.retain(|id| id != module_id);
                    }
                }
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
            }
            "setFilterTypology" => {
                let path = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or_default();
                document.filters.typology_path = if path.is_empty() { Vec::new() } else { path.split('/').map(String::from).collect() };
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
            }
            "setFilterMinAvailability" => {
                let current = document.filters.min_availability as f64;
                let next = args
                    .and_then(|value| value.get("delta"))
                    .and_then(|value| value.as_f64())
                    .map(|delta| current + delta)
                    .or_else(|| args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()))
                    .unwrap_or(current);
                document.filters.min_availability = next.max(0.0) as u32;
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
            }
            "sortTable" => {
                if let Some(column_id) = args.and_then(|value| value.get("columnId")).and_then(|value| value.as_str()) {
                    let direction = args.and_then(|value| value.get("direction")).and_then(|value| value.as_str()).unwrap_or("asc");
                    document.filters.sort =
                        Some(TableSort { column_id: column_id.to_string(), direction: if direction == "desc" { SortDirection::Desc } else { SortDirection::Asc } });
                }
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
            }
            "curateAdd" | "curateSetCount" => {
                if let Some(object_id) = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()) {
                    if let Some(delta) = args.and_then(|value| value.get("delta")).and_then(|value| value.as_f64()) {
                        document.curate_delta(object_id, delta as i64);
                    } else if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                        document.curate_set(object_id, value.max(0.0) as u32);
                    } else if action == "curateAdd" {
                        document.curate_delta(object_id, 1);
                    }
                    return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
                }
            }
            "curateRemove" | "dropOnPool" => {
                if let Some(object_id) = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()) {
                    document.curate_set(object_id, 0);
                    return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
                }
            }
            "dropOnCurated" => {
                if let Some(object_id) = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()) {
                    document.curate_delta(object_id, 1);
                    return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
                }
            }
            "selectRow" => {
                document.runtime.selected_object_id = args.and_then(|value| value.get("row")).and_then(|row| row.get("id")).and_then(|value| value.as_str()).map(str::to_string);
                return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
            }
            "worldSelect" => {
                let last_id = args.and_then(|value| value.get("ids")).and_then(|value| value.as_array()).and_then(|ids| ids.last()).and_then(|value| value.as_str());
                if let Some(id) = last_id {
                    document.runtime.selected_object_id = Some(id.to_string());
                    return ActionEmit::ops(vec![SourcingOp::SetDocument { document }]);
                }
            }
            _ => {}
        }
        ActionEmit::default()
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, CurateDocument>, view_state: &ViewState) -> UiNode {
        let document = doc.projection;
        let labels = sourcing_labels(view_state);
        match body_key {
            BODY_POOL => {
                let modules = available_modules(view_state);
                ui_stack_vertical(vec![build_filter_bar(document, &modules, labels), build_pool_table(document, labels)])
            }
            BODY_CURATED => build_curated_table(document, labels),
            BODY_PREVIEW => render_preview(document, labels),
            BODY_GRID => render_grid(document),
            _ => ui_text(""),
        }
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = sourcing_labels(view_state);
        let is_de = view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"));
        semio_framework_plugin::AppLabelsOverlay {
            app_label: None,
            window_kind_labels: std::collections::HashMap::from([
                (WINDOW_POOL.to_string(), labels.window_pool.to_string()),
                (WINDOW_CURATED.to_string(), labels.window_curated.to_string()),
                (WINDOW_PREVIEW.to_string(), labels.window_preview.to_string()),
                (WINDOW_GRID.to_string(), labels.window_grid.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::from([("curate".to_string(), labels.mode_curate.to_string())]),
            action_labels: sourcing_action_labels(is_de),
            utility_labels: sourcing_utility_labels(is_de),
            example_labels: std::collections::HashMap::from([
                (DEMO_STOCK_EXAMPLE_ID.to_string(), (if is_de { "Beispielbestand" } else { "Demo Stock" }).to_string()),
                (EMPTY_EXAMPLE_ID.to_string(), (if is_de { "Leere Kuratierung" } else { "Empty Curation" }).to_string()),
            ]),
            action_arg_labels: std::collections::HashMap::new(),
            dialog_labels: std::collections::HashMap::new(),
            introduction_labels: std::collections::HashMap::new(),
        }
    }
}
//#endregion 🔖SourcingCurateApp

//#region 🔖Bundle
fn sourcing_setup() {}

semio_framework_plugin::semio_plugin! {
    id: "sourcing", label: "Sourcing", version: "0.1.0",
    setup: sourcing_setup,
    apps: [ create_sourcing_curate_app => SourcingCurateApp ],
}
//#endregion 🔖Bundle

//#region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{ActionMeta, PluginApp, VcsDocumentApp};

    fn view_state() -> ViewState {
        ViewState::default()
    }

    fn meta() -> ActionMeta {
        ActionMeta { actor: "local".into(), instance_id: 1 }
    }

    fn new_app() -> VcsDocumentApp<SourcingCurateApp> {
        VcsDocumentApp::new(SourcingCurateApp::default())
    }

    /// 🧬 A wrapper carrying the real action registry so `setActiveExample`'s default materializes and the
    /// document-mutating curate ops pass kind discipline (they are declared Operations, never Views).
    fn new_app_with_registry() -> VcsDocumentApp<SourcingCurateApp> {
        use semio_framework_plugin::app::AppActionRegistry;
        let definition = create_sourcing_curate_app().definition;
        VcsDocumentApp::with_registry(SourcingCurateApp::default(), AppActionRegistry::from_definition(&definition))
    }

    #[test]
    fn curate_and_example_actions_survive_registry_enforcement() {
        let mut app = new_app_with_registry();
        // setActiveExample with no args materializes the declared default (demo stock, non-empty).
        app.handle_action("setActiveExample", None, &view_state(), &meta()).expect("set example");
        assert!(!app.projection().expect("projection").stock.is_empty(), "demo-stock default materialized from the registry");
        // curateAdd mutates the persisted document, so as a declared Operation it emits exactly one op
        // and is NOT rejected by the View/Shell no-ops kind discipline.
        let object_id = app.projection().expect("projection").stock[0].id.clone();
        let result = app.handle_action("curateAdd", Some(&json!({ "objectId": object_id })), &view_state(), &meta()).expect("curate");
        assert_eq!(result.operations.len(), 1, "curateAdd is a document operation");
        app.handle_action("undo", None, &view_state(), &meta()).expect("undo");
    }

    #[test]
    fn initial_document_has_populated_demo_stock() {
        let app = new_app();
        let document = app.projection().expect("projection");
        assert!(!document.stock.is_empty());
    }

    #[test]
    fn pool_render_respects_query_filter() {
        let mut document = default_document();
        document.filters.query = "glulam".into();
        let node = build_pool_table(&document, &SOURCING_LABELS_NATIVE_EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Glulam"));
        assert!(!json.contains("Hollow Core"));
    }

    #[test]
    fn pool_stepper_cell_max_equals_availability() {
        let document = default_document();
        let kind = &document.stock[0];
        let node = build_pool_table(&document, &SOURCING_LABELS_NATIVE_EN);
        let json = serde_json::to_value(&node).unwrap();
        let rows_json = json.pointer("/table/rowsJson").and_then(|value| value.as_str()).unwrap();
        let rows: Vec<Value> = serde_json::from_str(rows_json).unwrap();
        let row = rows.iter().find(|row| row.get("id").and_then(|id| id.as_str()) == Some(kind.id.as_str())).unwrap();
        assert_eq!(row["curated"]["max"].as_f64().unwrap(), kind.availability as f64);
    }

    #[test]
    fn curate_add_and_remove_round_trip_through_patch_ops() {
        let mut app = new_app();
        let document = app.projection().expect("projection");
        // stock[2] isn't part of the fixture's pre-curated set, so a single add lands on count 1.
        let object_id = document.stock[2].id.clone();
        app.handle_action("curateAdd", Some(&json!({ "objectId": object_id })), &view_state(), &meta()).expect("add");
        assert_eq!(app.projection().expect("projection").curated_count(&object_id), 1);

        app.handle_action("curateRemove", Some(&json!({ "objectId": object_id })), &view_state(), &meta()).expect("remove");
        assert_eq!(app.projection().expect("projection").curated_count(&object_id), 0);
    }

    #[test]
    fn drop_on_curated_and_drop_on_pool_mirror_add_and_remove() {
        let mut app = new_app();
        let document = app.projection().expect("projection");
        // stock[2] isn't part of the fixture's pre-curated set, so a single drop lands on count 1.
        let object_id = document.stock[2].id.clone();
        app.handle_action("dropOnCurated", Some(&json!({ "objectId": object_id })), &view_state(), &meta()).expect("drop on curated");
        assert_eq!(app.projection().expect("projection").curated_count(&object_id), 1);

        app.handle_action("dropOnPool", Some(&json!({ "objectId": object_id })), &view_state(), &meta()).expect("drop on pool");
        assert_eq!(app.projection().expect("projection").curated_count(&object_id), 0);
    }

    #[test]
    fn select_row_and_world_select_update_runtime_selection() {
        let mut app = new_app();
        let document = app.projection().expect("projection");
        let object_id = document.stock[0].id.clone();
        let other_id = document.stock[1].id.clone();

        app.handle_action("selectRow", Some(&json!({ "row": { "id": object_id } })), &view_state(), &meta()).expect("select");
        assert_eq!(app.projection().expect("projection").runtime.selected_object_id.as_deref(), Some(object_id.as_str()));

        app.handle_action("worldSelect", Some(&json!({ "ids": [object_id, other_id] })), &view_state(), &meta()).expect("world select");
        assert_eq!(app.projection().expect("projection").runtime.selected_object_id.as_deref(), Some(other_id.as_str()));
    }

    #[test]
    fn grid_instance_count_matches_filtered_stock_and_normalizes_scale() {
        let mut document = default_document();
        document.filters.module_ids = vec!["slabs".into()];
        let node = render_grid(&document);
        let json = serde_json::to_value(&node).unwrap();
        let instances_json = json.pointer("/world3d/instancesJson").and_then(|value| value.as_str()).unwrap();
        let instances: Vec<Value> = serde_json::from_str(instances_json).unwrap();
        assert_eq!(instances.len(), document.filtered_stock().len());
        for instance in &instances {
            let scale = instance["scale"][0].as_f64().unwrap();
            assert!(scale > 0.0);
        }
    }

    #[test]
    fn preview_renders_selected_mesh_id() {
        let mut document = default_document();
        let object_id = document.stock[0].id.clone();
        document.runtime.selected_object_id = Some(object_id.clone());
        let node = render_preview(&document, &SOURCING_LABELS_NATIVE_EN);
        let json = serde_json::to_value(&node).unwrap();
        let meshes_json = json.pointer("/world3d/meshesJson").and_then(|value| value.as_str()).unwrap();
        let meshes: Vec<Value> = serde_json::from_str(meshes_json).unwrap();
        assert_eq!(meshes.len(), 1);
        assert_eq!(meshes[0]["id"].as_str(), Some(object_id.as_str()));
    }

    #[test]
    fn preview_shows_placeholder_without_selection() {
        let document = default_document();
        let node = render_preview(&document, &SOURCING_LABELS_NATIVE_EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("No selection"));
    }

    #[test]
    fn contributions_parse_into_module_catalogues() {
        let module = sourcing_modules().into_iter().next().unwrap();
        let contribution = Contribution::SourcingModule {
            app_id: SOURCING_CURATE_APP_ID.into(),
            module_id: module.module_id().into(),
            label: module.label().into(),
            icon_id: "beam".into(),
            typology_json: serde_json::to_string(&module.typology()).unwrap(),
            kinds_json: serde_json::to_string(&module.demo_kinds()).unwrap(),
        };
        let contributions_json = serde_json::to_string(&vec![PluginContributionEntry { plugin_id: "sourcing-module-beams".into(), contribution }]).unwrap();
        let view_state = ViewState { contributions_json: Some(contributions_json), ..ViewState::default() };
        let catalogues = parse_module_catalogues(&view_state);
        assert_eq!(catalogues.len(), 1);
        assert_eq!(catalogues[0].module_id, module.module_id());
        assert_eq!(catalogues[0].kinds.len(), module.demo_kinds().len());
    }

    #[test]
    fn available_modules_falls_back_to_built_in_modules_without_contributions() {
        let modules = available_modules(&view_state());
        assert_eq!(modules.len(), sourcing_modules().len());
    }

    #[test]
    fn stock_from_catalogue_merges_contributed_kinds_without_duplicating() {
        let mut app = new_app();
        // Reset to the empty fixture so stockFromCatalogue starts from a genuinely empty stock.
        app.handle_action("setDocument", Some(&json!({ "document": empty_document() })), &view_state(), &meta()).expect("load empty document");
        assert!(app.projection().expect("projection").stock.is_empty());

        app.handle_action("stockFromCatalogue", None, &view_state(), &meta()).expect("populate");
        let expected: usize = sourcing_modules().iter().map(|module| module.demo_kinds().len()).sum();
        assert_eq!(app.projection().expect("projection").stock.len(), expected);

        app.handle_action("stockFromCatalogue", None, &view_state(), &meta()).expect("repopulate");
        assert_eq!(app.projection().expect("projection").stock.len(), expected);
    }

    #[test]
    fn set_filter_min_availability_clamps_to_zero() {
        let mut app = new_app();
        app.handle_action("setFilterMinAvailability", Some(&json!({ "delta": -1000.0 })), &view_state(), &meta()).expect("set min availability");
        assert_eq!(app.projection().expect("projection").filters.min_availability, 0);
    }
}
//#endregion 🔖Tests
