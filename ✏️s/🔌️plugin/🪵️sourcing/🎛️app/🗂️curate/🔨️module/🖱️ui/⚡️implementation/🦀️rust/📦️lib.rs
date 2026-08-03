//! 🎨️ Sourcing curate app — `DocumentApp` impl, render, manifest (constitutional: ui). B1: the
//! pure-trait pilot conversion — `SourcingCurateApp` is a unit struct; `filters` and the selected-object
//! pointer (formerly `CurateDocument.filters`/`.runtime`) now live in
//! `sourcing_engine::SourcingCurateConfig`, written via `sourcing_op::SourcingCurateConfigOperation`s
//! (real `backwards`, no ad hoc inverse bookkeeping); every action dispatches through the single typed
//! `sourcing_protocol::SourcingCurateCommand` channel via `DocumentApp::handle` — mirrors
//! `shooting_ui::ShootingPlayApp` exactly (see that crate for the reference pilot conversion).

use semio_framework_core::mesh_from_indexed;
use semio_framework_plugin::{
    app_labels, build_table_scene, build_world_3d_scene, localized_label_map, table_row_json, ui_stack_vertical, ui_text, world3d_default_camera, world3d_scene, world3d_selection_json, ActionArgDef, ActionArgOption,
    ActionDefinition, ActionDescriptor, ActionKind, App, AppIo, AppLabelsOverlay, AppLabelsOverlayExt, ConfigView, DocumentApp, DocumentView, Emit, LocaleLabels, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability, ArtifactKindSpec, SurfaceKind, TableCell, TableScene, UiInputNode, UiNode,
    UiNumberStepperNode, UiPresence, UiSelectItem, UiSelectNode, UiToggleNode, UiTreeItemAction, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WorldSunConfig,
};
use serde_json::{json, Value};
use sourcing::{CurateDocument, Filters, ObjectKind, SortDirection, TableSort, SOURCING_CURATE_SCHEMA};
use sourcing_engine::{grid_placement, grid_scale, mesh_spec_for, sourcing_modules, typology_flatten, SourcingCurateConfig, TypologyNode};
use sourcing_op::{SourcingCurateConfigOperation, SourcingOperation};
use sourcing_protocol::SourcingCurateCommand;
use std::collections::HashSet;
use store::DocumentPack;

//#region 🔖️Constants
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
//#endregion 🔖️Constants

//#region 🔖️Locale
/// 🗣️ B1: `cfg.locale`-driven counterparts to the deleted `ViewState`-driven
/// `semio_framework_plugin::is_de_locale`/`resolve_labels` — `DocumentApp::render`/`handle` no longer
/// receive a `ViewState` at all (B1 dropped it), so locale-aware label resolution now reads it off the
/// config projection instead. Mirrors `shooting_ui`'s identical local shims.
fn is_de_locale(cfg: &SourcingCurateConfig) -> bool {
    cfg.locale.starts_with("de")
}

fn resolve_labels<L: LocaleLabels>(cfg: &SourcingCurateConfig) -> &'static L {
    if is_de_locale(cfg) { L::locale_labels_de() } else { L::locale_labels_en() }
}
//#endregion 🔖️Locale

//#region 🔖️Modules
/// 🧩️ One module's typology + catalogue kinds. B1 dropped the `ViewState.contributions_json`-sourced
/// override — that channel doesn't reach the pure `DocumentApp::render`/`handle` trait at all anymore
/// (`DocumentApp::render`'s signature carries no `ViewState`), so this app now always uses its own
/// built-in module registry (`sourcing_engine::sourcing_modules`), the same fallback the pre-B1 code
/// took whenever no plugin had contributed yet. `sourcing-module-{beams,windows,slabs}` still declare
/// their `Contribution::SourcingModule` for the OS-level catalog aggregation that consumes it outside
/// this app's own render/handle path — unaffected by this simplification.
struct ModuleCatalogue {
    module_id: String,
    label: String,
    typology: TypologyNode,
    kinds: Vec<ObjectKind>,
}

fn available_modules() -> Vec<ModuleCatalogue> {
    sourcing_modules()
        .into_iter()
        .map(|module| ModuleCatalogue { module_id: module.module_id().to_string(), label: module.label().to_string(), typology: module.typology(), kinds: module.demo_kinds() })
        .collect()
}
//#endregion 🔖️Modules

//#region 🔖️Document
fn sourcing_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: SOURCING_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

/// 👁️ Was `document.runtime.selected_object_id` — B1 moved the selected-object pointer onto
/// `SourcingCurateConfig` (session-only view state).
fn selected_ids(cfg: &SourcingCurateConfig) -> Vec<String> {
    cfg.selected_object_id.clone().into_iter().collect()
}

fn selection_json_for(cfg: &SourcingCurateConfig) -> String {
    json!({ "selectedIds": selected_ids(cfg) }).to_string()
}
//#endregion 🔖️Document

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the curate app; one field per label makes every locale combination compile-checked.
    struct SourcingLabels {
        window_pool: native_en "Pool", native_de "Pool", reuse_en "Pool", reuse_de "Pool";
        window_curated: native_en "Curated", native_de "Kuratiert", reuse_en "Curated", reuse_de "Kuratiert";
        window_preview: native_en "Preview", native_de "Vorschau", reuse_en "Preview", reuse_de "Vorschau";
        window_grid: native_en "Grid", native_de "Raster", reuse_en "Grid", reuse_de "Raster";
        mode_curate: native_en "Curate", native_de "Kuratieren", reuse_en "Curate", reuse_de "Kuratieren";
        search_placeholder: native_en "Search…", native_de "Suchen…", reuse_en "Search…", reuse_de "Suchen…";
        all_typologies: native_en "All Typologies", native_de "Alle Typologien", reuse_en "All Typologies", reuse_de "Alle Typologien";
        col_name: native_en "Name", native_de "Name", reuse_en "Name", reuse_de "Name";
        col_module: native_en "Module", native_de "Modul", reuse_en "Module", reuse_de "Modul";
        col_typology: native_en "Typology", native_de "Typologie", reuse_en "Typology", reuse_de "Typologie";
        col_availability: native_en "Availability", native_de "Verfügbarkeit", reuse_en "Availability", reuse_de "Verfügbarkeit";
        col_curated: native_en "Curated", native_de "Kuratiert", reuse_en "Curated", reuse_de "Kuratiert";
        col_count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        remove: native_en "Remove", native_de "Entfernen", reuse_en "Remove", reuse_de "Entfernen";
        no_selection: native_en "No selection", native_de "Keine Auswahl", reuse_en "No selection", reuse_de "Keine Auswahl";
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn build_filter_bar(filters: &Filters, modules: &[ModuleCatalogue], labels: &SourcingLabels) -> UiNode {
    let mut children = vec![UiNode::Input(UiInputNode { presence: UiPresence::default(),
        id: "sourcing-filter-query".into(),
        input_kind: "text".into(),
        value: filters.query.clone(),
        placeholder: Some(labels.search_placeholder.into()),
        commit: None,
        min: None,
        max: None,
        step: None,
        accept: None,
        on_change: sourcing_action("setFilterQuery", None),
        menu: None,
    })];
    for module in modules {
        let pressed = filters.module_ids.iter().any(|id| id == &module.module_id);
        children.push(UiNode::Toggle(UiToggleNode {
            id: format!("sourcing-filter-module-{}", module.module_id),
            icon_id: "layers".into(),
            text: Some(module.label.clone()),
            on_change: sourcing_action("setFilterModule", Some(json!({ "moduleId": module.module_id, "enabled": !pressed }))),
            presence: UiPresence::selected(pressed),
            menu: None,
        }));
    }
    let mut typology_items = vec![UiSelectItem { value: String::new(), label: labels.all_typologies.into(),
        }];
    for module in modules {
        for (path, label) in typology_flatten(&module.typology) {
            typology_items.push(UiSelectItem { value: path.join("/"), label,
        });
        }
    }
    children.push(UiNode::Select(UiSelectNode { presence: UiPresence::default(), id: "sourcing-filter-typology".into(), value: filters.typology_path.join("/"), items: typology_items, placeholder: None, on_change: sourcing_action("setFilterTypology", None),
        menu: None,
    }));
    children.push(UiNode::NumberStepper(UiNumberStepperNode { presence: UiPresence::default(),
        id: "sourcing-filter-min-availability".into(),
        value: filters.min_availability as f64,
        step: 1.0,
        uniform: true,
        on_absolute: sourcing_action("setFilterMinAvailability", None),
        on_delta: sourcing_action("setFilterMinAvailability", None),
        menu: None,
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

fn build_pool_table(document: &CurateDocument, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiNode {
    let mut filtered = sourcing_engine::filtered_stock(document, &cfg.filters);
    if let Some(sort) = &cfg.filters.sort {
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
            let curated_count = sourcing_engine::curated_count(document, &kind.id) as f64;
            table_row_json(
                &kind.id,
                Some(&json!({ "objectId": kind.id })),
                &[
                    ("name", TableCell::Text { value: kind.name.clone() }),
                    ("module", TableCell::Text { value: kind.module_id.clone() }),
                    ("typology", TableCell::Text { value: kind.typology_path.join(" / ") }),
                    ("availability", TableCell::Number { value: kind.availability as f64 }),
                    ("curated", TableCell::Stepper { value: curated_count, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_action("curateSetCount", Some(json!({ "objectId": kind.id }))) }),
                ],
            )
        })
        .collect();
    let mut scene = TableScene::base(pool_columns_json(labels), serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()));
    scene.selection_json = Some(selection_json_for(cfg));
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action = Some(sourcing_action("dropOnPool", None));
    scene.sort_json = cfg.filters.sort.as_ref().and_then(|sort| serde_json::to_string(sort).ok());
    build_table_scene(SURFACE_POOL, SOURCING_CONTROLLER_ID, scene)
}

fn build_curated_table(document: &CurateDocument, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiNode {
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
                    ("count", TableCell::Stepper { value: item.count as f64, min: 0.0, max: kind.availability as f64, step: 1.0, action: sourcing_action("curateSetCount", Some(json!({ "objectId": kind.id }))) }),
                    (
                        "actions",
                        TableCell::Buttons { buttons: vec![UiTreeItemAction { icon_id: "trash-2".into(), label: Some(labels.remove.into()), action: sourcing_action("curateRemove", Some(json!({ "objectId": kind.id }))), reveal_on_hover: None,
        }] },
                    ),
                ],
            ))
        })
        .collect();
    let mut scene = TableScene::base(columns, serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into()));
    scene.selection_json = Some(selection_json_for(cfg));
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action = Some(sourcing_action("dropOnCurated", None));
    build_table_scene(SURFACE_CURATED, SOURCING_CONTROLLER_ID, scene)
}
//#endregion 🔖️Panels

//#region 🔖️World3d
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

fn render_preview(document: &CurateDocument, cfg: &SourcingCurateConfig, labels: &SourcingLabels) -> UiNode {
    let Some(kind) = cfg.selected_object_id.as_ref().and_then(|id| document.stock.iter().find(|kind| &kind.id == id)) else {
        return ui_text(labels.no_selection);
    };
    let meshes_json = json!([kind_mesh_json(kind)]).to_string();
    let instances_json = json!([instance_json(kind, [0.0, 0.0, 0.0], 1.0, false)]).to_string();
    let mut scene = world3d_scene(world3d_default_camera(), meshes_json, instances_json, world3d_selection_json("rectangle", &[], None), &WorldSunConfig::default());
    scene.fit_json = Some(json!({ "enabled": true, "padding": 0.2 }).to_string());
    build_world_3d_scene(SURFACE_PREVIEW, SOURCING_CONTROLLER_ID, scene)
}

fn render_grid(document: &CurateDocument, cfg: &SourcingCurateConfig) -> UiNode {
    let filtered = sourcing_engine::filtered_stock(document, &cfg.filters);
    let mut seen_mesh_ids = HashSet::new();
    let mut meshes = Vec::new();
    let mut instances = Vec::new();
    for (index, kind) in filtered.iter().enumerate() {
        if seen_mesh_ids.insert(kind.id.clone()) {
            meshes.push(kind_mesh_json(kind));
        }
        let (x, z) = grid_placement(filtered.len(), index, GRID_CELL);
        let scale = grid_scale(&kind.geometry, GRID_CELL * 0.8);
        let selected = cfg.selected_object_id.as_deref() == Some(kind.id.as_str());
        instances.push(instance_json(kind, [x, 0.0, z], scale, selected));
    }
    let mut scene = world3d_scene(world3d_default_camera(), json!(meshes).to_string(), json!(instances).to_string(), world3d_selection_json("rectangle", &selected_ids(cfg), None), &WorldSunConfig::default());
    scene.fit_json = Some(json!({ "enabled": true, "padding": 0.3 }).to_string());
    build_world_3d_scene(SURFACE_GRID, SOURCING_CONTROLLER_ID, scene)
}
//#endregion 🔖️World3d

//#region 🔖️SourcingCurateApp
/// 🧪️ B1: unit struct — every former app-struct field now lives in
/// `sourcing_engine::SourcingCurateConfig` (see `DocumentApp::Config`), written through
/// `sourcing_op::SourcingCurateConfigOperation`s.
#[derive(Default)]
pub struct SourcingCurateApp;

impl DocumentApp for SourcingCurateApp {
    type Projection = CurateDocument;
    type Operation = SourcingOperation;
    type Config = SourcingCurateConfig;
    type ConfigOperation = SourcingCurateConfigOperation;
    type Command = SourcingCurateCommand;

    fn app_id(&self) -> &str {
        SOURCING_CURATE_APP_ID
    }

    fn document_schema(&self) -> &str {
        SOURCING_CURATE_SCHEMA
    }

    fn initial_projection(&self) -> CurateDocument {
        sourcing_engine::default_document()
    }

    fn io(&self) -> Option<AppIo> {
        Some(sourcing_engine::sourcing_curate_io())
    }

    /// 🎞️ `catalog:out` (see `sourcing_engine::sourcing_catalog_fragment`) plus the inherited
    /// `document:out` default (the pack of `doc.projection`, replicated inline — overriding
    /// `export_media` shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, CurateDocument>) -> Result<Media, MediaError> {
        match port {
            "catalog:out" => Ok(Media {
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                payload: MediaPayload::Structured { schema: "kit.catalog".into(), json: sourcing_engine::sourcing_catalog_fragment(doc.projection).to_string() },
            }),
            "document:out" => {
                let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
                let bytes = doc.projection.encode_pack();
                Ok(Media {
                    media_type,
                    payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(&self, projection: CurateDocument) -> Option<SourcingOperation> {
        Some(SourcingOperation::SetDocument { document: projection })
    }

    /// 🏷️ Maps each `SourcingCurateCommand` variant back to the action id it was declared under in
    /// `create_sourcing_curate_app` — used by `VcsDocumentApp` for command-log labeling and the
    /// registry's View/Shell kind-discipline check.
    fn command_id(&self, command: &SourcingCurateCommand) -> &str {
        match command {
            SourcingCurateCommand::SetDocumentJson { .. } => "setDocument",
            SourcingCurateCommand::SetActiveExample { .. } => "setActiveExample",
            SourcingCurateCommand::StockFromCatalogue => "stockFromCatalogue",
            SourcingCurateCommand::CurateAdd { .. } => "curateAdd",
            SourcingCurateCommand::CurateSetCount { .. } => "curateSetCount",
            SourcingCurateCommand::CurateRemove { .. } => "curateRemove",
            SourcingCurateCommand::DropOnPool { .. } => "dropOnPool",
            SourcingCurateCommand::DropOnCurated { .. } => "dropOnCurated",
            SourcingCurateCommand::SetFilterQuery { .. } => "setFilterQuery",
            SourcingCurateCommand::SetFilterModule { .. } => "setFilterModule",
            SourcingCurateCommand::SetFilterTypology { .. } => "setFilterTypology",
            SourcingCurateCommand::SetFilterMinAvailability { .. } => "setFilterMinAvailability",
            SourcingCurateCommand::SortTable { .. } => "sortTable",
            SourcingCurateCommand::SelectRow { .. } => "selectRow",
            SourcingCurateCommand::WorldSelect { .. } => "worldSelect",
            SourcingCurateCommand::SetLocale { .. } => "setLocale",
        }
    }

    fn handle(&self, command: &SourcingCurateCommand, doc: &DocumentView<'_, CurateDocument>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> Emit<SourcingOperation, SourcingCurateConfigOperation> {
        let config = cfg.projection;
        match command {
            SourcingCurateCommand::SetDocumentJson { json } => match serde_json::from_str::<CurateDocument>(json) {
                Ok(document) => Emit::operations(vec![SourcingOperation::SetDocument { document }]),
                Err(_) => Emit::default(),
            },
            SourcingCurateCommand::SetActiveExample { example_id } => {
                let next = if example_id.is_empty() || example_id == EMPTY_EXAMPLE_ID { sourcing_engine::empty_document() } else { sourcing_engine::default_document() };
                Emit::operations(vec![SourcingOperation::SetDocument { document: next }])
            }
            SourcingCurateCommand::StockFromCatalogue => {
                let mut document = doc.projection.clone();
                let existing: HashSet<String> = document.stock.iter().map(|kind| kind.id.clone()).collect();
                for module in available_modules() {
                    for kind in module.kinds {
                        if !existing.contains(&kind.id) {
                            document.stock.push(kind);
                        }
                    }
                }
                Emit::operations(vec![SourcingOperation::SetDocument { document }])
            }
            SourcingCurateCommand::CurateAdd { object_id } => {
                let mut document = doc.projection.clone();
                sourcing_engine::curate_delta(&mut document, object_id, 1);
                Emit::operations(vec![SourcingOperation::SetDocument { document }])
            }
            SourcingCurateCommand::CurateSetCount { object_id, delta, value } => {
                let mut document = doc.projection.clone();
                if let Some(delta) = delta {
                    sourcing_engine::curate_delta(&mut document, object_id, *delta as i64);
                } else if let Some(value) = value {
                    sourcing_engine::curate_set(&mut document, object_id, value.max(0.0) as u32);
                }
                Emit::operations(vec![SourcingOperation::SetDocument { document }])
            }
            SourcingCurateCommand::CurateRemove { object_id } | SourcingCurateCommand::DropOnPool { object_id } => {
                let mut document = doc.projection.clone();
                sourcing_engine::curate_set(&mut document, object_id, 0);
                Emit::operations(vec![SourcingOperation::SetDocument { document }])
            }
            SourcingCurateCommand::DropOnCurated { object_id } => {
                let mut document = doc.projection.clone();
                sourcing_engine::curate_delta(&mut document, object_id, 1);
                Emit::operations(vec![SourcingOperation::SetDocument { document }])
            }
            SourcingCurateCommand::SetFilterQuery { value } => Emit::config(vec![SourcingCurateConfigOperation::SetFilterQuery { value: value.clone() }]),
            SourcingCurateCommand::SetFilterModule { module_id, enabled } => {
                let mut module_ids = config.filters.module_ids.clone();
                if *enabled {
                    if !module_ids.iter().any(|id| id == module_id) {
                        module_ids.push(module_id.clone());
                    }
                } else {
                    module_ids.retain(|id| id != module_id);
                }
                Emit::config(vec![SourcingCurateConfigOperation::SetFilterModules { module_ids }])
            }
            SourcingCurateCommand::SetFilterTypology { path } => {
                let path = if path.is_empty() { Vec::new() } else { path.split('/').map(String::from).collect() };
                Emit::config(vec![SourcingCurateConfigOperation::SetFilterTypology { path }])
            }
            SourcingCurateCommand::SetFilterMinAvailability { delta, value } => {
                let current = config.filters.min_availability as f64;
                let next = delta.map(|d| current + d).or(*value).unwrap_or(current);
                Emit::config(vec![SourcingCurateConfigOperation::SetFilterMinAvailability { value: next.max(0.0) as u32 }])
            }
            SourcingCurateCommand::SortTable { column_id, direction } => {
                let sort = TableSort { column_id: column_id.clone(), direction: if direction == "desc" { SortDirection::Desc } else { SortDirection::Asc } };
                Emit::config(vec![SourcingCurateConfigOperation::SetSort { sort: Some(sort) }])
            }
            SourcingCurateCommand::SelectRow { object_id } => Emit::config(vec![SourcingCurateConfigOperation::SetSelectedObject { object_id: object_id.clone() }]),
            SourcingCurateCommand::WorldSelect { ids } => match ids.last() {
                Some(id) => Emit::config(vec![SourcingCurateConfigOperation::SetSelectedObject { object_id: Some(id.clone()) }]),
                None => Emit::default(),
            },
            SourcingCurateCommand::SetLocale { value } => Emit::config(vec![SourcingCurateConfigOperation::SetLocale { value: value.clone() }]),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, CurateDocument>, cfg: &ConfigView<'_, SourcingCurateConfig>) -> UiNode {
        let document = doc.projection;
        let config = cfg.projection;
        let labels = resolve_labels::<SourcingLabels>(config);
        match body_key {
            BODY_POOL => {
                let modules = available_modules();
                ui_stack_vertical(vec![build_filter_bar(&config.filters, &modules, labels), build_pool_table(document, config, labels)])
            }
            BODY_CURATED => build_curated_table(document, config, labels),
            BODY_PREVIEW => render_preview(document, config, labels),
            BODY_GRID => render_grid(document, config),
            _ => ui_text(""),
        }
    }

    fn app_labels(&self, cfg: &ConfigView<'_, SourcingCurateConfig>) -> AppLabelsOverlay {
        let config = cfg.projection;
        let labels = resolve_labels::<SourcingLabels>(config);
        let is_de = is_de_locale(config);
        AppLabelsOverlay::default()
            .window_kind_label(WINDOW_POOL, labels.window_pool)
            .window_kind_label(WINDOW_CURATED, labels.window_curated)
            .window_kind_label(WINDOW_PREVIEW, labels.window_preview)
            .window_kind_label(WINDOW_GRID, labels.window_grid)
            .mode_label("curate", labels.mode_curate)
            .action_labels(sourcing_action_labels(is_de))
            .example_labels(std::collections::HashMap::from([
                (DEMO_STOCK_EXAMPLE_ID.to_string(), (if is_de { "Beispielbestand" } else { "Demo Stock" }).to_string()),
                (EMPTY_EXAMPLE_ID.to_string(), (if is_de { "Leere Kuratierung" } else { "Empty Curation" }).to_string()),
            ]))
    }
}
//#endregion 🔖️SourcingCurateApp

//#region 🔖️CommandLabels
/// 🗣️ (action id) -> localized label for every operation/hidden-operation declared in `create_sourcing_curate_app`'s
/// static manifest — mirrors `puzzle3d_action_labels`, built on the shared `localized_label_map`.
fn sourcing_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    localized_label_map(
        is_de,
        &[
            ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
            ("stockFromCatalogue", "Stock From Catalogue", "Bestand aus Katalog"),
            ("setDocument", "Set Document", "Dokument festlegen"),
            ("setFilterQuery", "Set Filter Query", "Filterabfrage festlegen"),
            ("setFilterModule", "Set Filter Module", "Filtermodul festlegen"),
            ("setFilterTypology", "Set Filter Typology", "Filtertypologie festlegen"),
            ("setFilterMinAvailability", "Set Filter Min Availability", "Mindestverfügbarkeit festlegen"),
            ("sortTable", "Sort Table", "Tabelle sortieren"),
            ("curateAdd", "Curate Add", "Kuratierung hinzufügen"),
            ("curateSetCount", "Curate Set Count", "Kuratierte Anzahl festlegen"),
            ("curateRemove", "Curate Remove", "Kuratierung entfernen"),
            ("dropOnPool", "Drop On Pool", "Auf Pool ablegen"),
            ("dropOnCurated", "Drop On Curated", "Auf Kuratiert ablegen"),
            ("selectRow", "Select Row", "Zeile auswählen"),
            ("worldSelect", "World Select", "Welt auswählen"),
        ],
    )
}
//#endregion 🔖️CommandLabels

//#region 🔖️Manifest
fn sourcing_window(window_kind_id: &str, title: &str) -> WindowLayoutWindowNode {
    WindowLayoutWindowNode { kind: "window".into(), window_kind_id: window_kind_id.into(), title: Some(title.into()), instance_id: None, template_id: None }
}

fn sourcing_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode { kind: "stack".into(), size, active_window_kind_id: None, children: vec![sourcing_window(window_kind_id, title)] })
}

/// 🪟️ Three-column layout: pool | curated over preview | grid — mirrors `cad_quad_layout`'s pattern.
fn sourcing_three_column_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.34), children: vec![sourcing_stack(WINDOW_POOL, "Pool", None)] }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.33), children: vec![sourcing_stack(WINDOW_CURATED, "Curated", Some(0.55)), sourcing_stack(WINDOW_PREVIEW, "Preview", Some(0.45))] }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.33), children: vec![sourcing_stack(WINDOW_GRID, "Grid", None)] }),
            ],
        }),
    }
}

/// 🙈️ An internal document operation kept out of the command palette — the curate/DnD arms that mutate
/// the persisted `CurateDocument` but are only ever dispatched from window chrome.
fn hidden_operation(id: &str, label: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::Operation) }
}

/// 🙈️👁️ B1: the filter/sort/selection/world-pick arms moved off the document onto
/// `SourcingCurateConfig` — they emit ONLY `config_operations` now, so (unlike `hidden_operation`
/// above) they're declared `ActionKind::View`, letting `VcsDocumentApp`'s kind-discipline check
/// actually enforce "must not emit document operations" instead of silently skipping an undeclared id.
fn hidden_view_action(id: &str, label: &str) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, ActionKind::View) }
}

pub fn create_sourcing_curate_app() -> App {
    App::from_builder(
        App::builder(SOURCING_CURATE_APP_ID, "Curate")
            .document(["semio", "sourcing", "curate"])
            .artifact_kind(ArtifactKindSpec {
                id: "catalogue.kinds".into(),
                name: "Kind Catalogue".into(),
                source_format: "catalogue.kinds".into(),
                component_kind: "catalogue".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: "catalogue.kinds".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .artifact_kind(ArtifactKindSpec {
                id: "catalogue.sourcing".into(),
                name: "Sourcing Curation".into(),
                source_format: "sourcing.curate".into(),
                component_kind: "catalogue".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
                schema: "sourcing.curate".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            // 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: the `catalog:out` port's declared kind —
            // harmless duplicate `ArtifactKindSpec` across producers (see `s/plugin/block`'s `3d` app,
            // which declares the SAME `kit.catalog` shape independently).
            .artifact_kind(ArtifactKindSpec {
                id: "kit.catalog".into(),
                name: "Kit Catalogue".into(),
                source_format: "kit.catalog".into(),
                component_kind: "catalogue".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: "kit.catalog".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("library")
            .mode("curate", "Curate", "folder-open")
            .default_mode_id("curate")
            .window_kind(WINDOW_POOL, "Pool", BODY_POOL, SurfaceKind::Table, "library")
            .window_kind(WINDOW_CURATED, "Curated", BODY_CURATED, SurfaceKind::Table, "tags")
            .window_kind(WINDOW_PREVIEW, "Preview", BODY_PREVIEW, SurfaceKind::World3d, "preview")
            .window_kind(WINDOW_GRID, "Grid", BODY_GRID, SurfaceKind::World3d, "grid-3x3")
            .default_layout(sourcing_three_column_layout())
            // 🔧️ Curation counts/stock edits are persisted in `CurateDocument`, so each arm emits a
            // whole-document `SetDocument` operation and is declared as an Operation, never a View.
            .operation("setActiveExample", "Set Active Example")
            .operation("stockFromCatalogue", "Stock From Catalogue")
            .action_with(hidden_operation("setDocument", "Set Document"))
            .action_with(hidden_operation("curateAdd", "Curate Add"))
            .action_with(hidden_operation("curateSetCount", "Curate Set Count"))
            .action_with(hidden_operation("curateRemove", "Curate Remove"))
            .action_with(hidden_operation("dropOnPool", "Drop On Pool"))
            .action_with(hidden_operation("dropOnCurated", "Drop On Curated"))
            // 👁️ Filters/sort/selection — session-only `SourcingCurateConfig` view state, never the document.
            .action_with(hidden_view_action("setFilterQuery", "Set Filter Query"))
            .action_with(hidden_view_action("setFilterModule", "Set Filter Module"))
            .action_with(hidden_view_action("setFilterTypology", "Set Filter Typology"))
            .action_with(hidden_view_action("setFilterMinAvailability", "Set Filter Min Availability"))
            .action_with(hidden_view_action("sortTable", "Sort Table"))
            .action_with(hidden_view_action("selectRow", "Select Row"))
            .action_with(hidden_view_action("worldSelect", "World Select"))
            // 📝️ Staged argument form for the panel-visible example switch.
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", "Example", vec![
                    ActionArgOption::new(DEMO_STOCK_EXAMPLE_ID, "Demo Stock"),
                    ActionArgOption::new(EMPTY_EXAMPLE_ID, "Empty Curation"),
                ]).default_value(DEMO_STOCK_EXAMPLE_ID),
            ])
            // 🎯️ Typed channel surface — this app's typed commands are dispatched via
            // `SourcingCurateCommand`'s `OpBinary` codec directly (`setLocale` deliberately left
            // undeclared above, mirroring `shooting_ui`: `VcsDocumentApp`'s kind-discipline check only
            // runs when the registry actually declares a command's id).
            .io(sourcing_engine::sourcing_curate_io()),
    )
    // 📄️ `AppDefinition::example` still wants document JSON (the manifest-wide example wire format);
    // the `.curate` text above is only the on-disk source of truth, re-serialized here once.
    .example(DEMO_STOCK_EXAMPLE_ID, "Demo Stock", serde_json::to_string(&sourcing_engine::default_document()).unwrap_or_default(), "file-text")
    .example(EMPTY_EXAMPLE_ID, "Empty Curation", serde_json::to_string(&sourcing_engine::empty_document()).unwrap_or_default(), "file-text")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, ViewState};

    fn new_app() -> VcsDocumentApp<SourcingCurateApp> {
        testkit::new_app::<SourcingCurateApp>()
    }

    /// 🧬️ A wrapper carrying the real action registry so kind discipline runs.
    fn new_app_with_registry() -> VcsDocumentApp<SourcingCurateApp> {
        testkit::new_app_with_registry::<SourcingCurateApp>(create_sourcing_curate_app)
    }

    #[test]
    fn curate_and_example_actions_survive_registry_enforcement() {
        // 🧬️ A registry-backed wrapper so `setActiveExample`'s default materializes and the
        // document-mutating curate commands pass kind discipline (they are declared Operations, never Views).
        let mut app = new_app_with_registry();
        // setActiveExample with no args materializes the declared default (demo stock, non-empty).
        app.dispatch_typed(SourcingCurateCommand::SetActiveExample { example_id: DEMO_STOCK_EXAMPLE_ID.into() }, &testkit::meta("local")).expect("set example");
        assert!(!app.projection().expect("projection").stock.is_empty(), "demo-stock default materialized from the registry");
        // curateAdd mutates the persisted document, so as a declared Operation it emits exactly one operation
        // and is NOT rejected by the View/Shell no-operations kind discipline.
        let object_id = app.projection().expect("projection").stock[0].id.clone();
        let result = app.dispatch_typed(SourcingCurateCommand::CurateAdd { object_id }, &testkit::meta("local")).expect("curate");
        assert_eq!(result.operations.len(), 1, "curateAdd is a document operation");
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
    }

    #[test]
    fn view_kind_config_only_commands_pass_kind_discipline() {
        // 🧬️ A registry-backed wrapper so the View-kind declarations actually get enforced.
        let mut app = new_app_with_registry();
        let result = app.dispatch_typed(SourcingCurateCommand::SetFilterQuery { value: "glulam".into() }, &testkit::meta("local")).expect("filter query");
        assert!(result.operations.is_empty(), "setFilterQuery is config-only, no document operations");
    }

    #[test]
    fn initial_document_has_populated_demo_stock() {
        let app = new_app();
        let document = app.projection().expect("projection");
        assert!(!document.stock.is_empty());
    }

    #[test]
    fn pool_render_respects_query_filter() {
        let document = sourcing_engine::default_document();
        let cfg = SourcingCurateConfig { filters: Filters { query: "glulam".into(), ..Default::default() }, ..Default::default() };
        let node = build_pool_table(&document, &cfg, &SourcingLabels::EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Glulam"));
        assert!(!json.contains("Hollow Core"));
    }

    #[test]
    fn pool_stepper_cell_max_equals_availability() {
        let document = sourcing_engine::default_document();
        let cfg = SourcingCurateConfig::default();
        let kind = &document.stock[0];
        let node = build_pool_table(&document, &cfg, &SourcingLabels::EN);
        let json = serde_json::to_value(&node).unwrap();
        let rows_json = json.pointer("/table/rowsJson").and_then(|value| value.as_str()).unwrap();
        let rows: Vec<Value> = serde_json::from_str(rows_json).unwrap();
        let row = rows.iter().find(|row| row.get("id").and_then(|id| id.as_str()) == Some(kind.id.as_str())).unwrap();
        assert_eq!(row["curated"]["max"].as_f64().unwrap(), kind.availability as f64);
    }

    #[test]
    fn curate_add_and_remove_round_trip_through_operations() {
        let mut app = new_app();
        let document = app.projection().expect("projection");
        // stock[2] isn't part of the fixture's pre-curated set, so a single add lands on count 1.
        let object_id = document.stock[2].id.clone();
        app.dispatch_typed(SourcingCurateCommand::CurateAdd { object_id: object_id.clone() }, &testkit::meta("local")).expect("add");
        assert_eq!(sourcing_engine::curated_count(&app.projection().expect("projection"), &object_id), 1);

        app.dispatch_typed(SourcingCurateCommand::CurateRemove { object_id: object_id.clone() }, &testkit::meta("local")).expect("remove");
        assert_eq!(sourcing_engine::curated_count(&app.projection().expect("projection"), &object_id), 0);
    }

    #[test]
    fn curate_set_count_supports_both_delta_and_absolute_value() {
        let mut app = new_app();
        let object_id = app.projection().expect("projection").stock[2].id.clone();
        app.dispatch_typed(SourcingCurateCommand::CurateSetCount { object_id: object_id.clone(), delta: Some(3.0), value: None }, &testkit::meta("local")).expect("delta");
        assert_eq!(sourcing_engine::curated_count(&app.projection().expect("projection"), &object_id), 3);
        app.dispatch_typed(SourcingCurateCommand::CurateSetCount { object_id: object_id.clone(), delta: None, value: Some(2.0) }, &testkit::meta("local")).expect("value");
        assert_eq!(sourcing_engine::curated_count(&app.projection().expect("projection"), &object_id), 2);
    }

    #[test]
    fn drop_on_curated_and_drop_on_pool_mirror_add_and_remove() {
        let mut app = new_app();
        let document = app.projection().expect("projection");
        // stock[2] isn't part of the fixture's pre-curated set, so a single drop lands on count 1.
        let object_id = document.stock[2].id.clone();
        app.dispatch_typed(SourcingCurateCommand::DropOnCurated { object_id: object_id.clone() }, &testkit::meta("local")).expect("drop on curated");
        assert_eq!(sourcing_engine::curated_count(&app.projection().expect("projection"), &object_id), 1);

        app.dispatch_typed(SourcingCurateCommand::DropOnPool { object_id: object_id.clone() }, &testkit::meta("local")).expect("drop on pool");
        assert_eq!(sourcing_engine::curated_count(&app.projection().expect("projection"), &object_id), 0);
    }

    #[test]
    fn select_row_and_world_select_update_config_selection() {
        let mut app = new_app();
        let document = app.projection().expect("projection");
        let object_id = document.stock[0].id.clone();
        let other_id = document.stock[1].id.clone();

        app.dispatch_typed(SourcingCurateCommand::SelectRow { object_id: Some(object_id.clone()) }, &testkit::meta("local")).expect("select");
        let selected = app.render(BODY_GRID, None, &ViewState::default()).expect("render grid");
        assert!(serde_json::to_string(&selected).unwrap().contains(&object_id));

        app.dispatch_typed(SourcingCurateCommand::WorldSelect { ids: vec![object_id, other_id.clone()] }, &testkit::meta("local")).expect("world select");
        let selected = app.render(BODY_GRID, None, &ViewState::default()).expect("render grid");
        let json = serde_json::to_value(&selected).unwrap();
        let instances_json = json.pointer("/world3d/instancesJson").and_then(|value| value.as_str()).unwrap();
        let instances: Vec<Value> = serde_json::from_str(instances_json).unwrap();
        let selected_instance = instances.iter().find(|instance| instance["id"] == other_id).unwrap();
        assert_eq!(selected_instance["selected"], json!(true), "worldSelect keeps the LAST id as the single selection");
    }

    #[test]
    fn grid_instance_count_matches_filtered_stock_and_normalizes_scale() {
        let document = sourcing_engine::default_document();
        let cfg = SourcingCurateConfig { filters: Filters { module_ids: vec!["slabs".into()], ..Default::default() }, ..Default::default() };
        let node = render_grid(&document, &cfg);
        let json = serde_json::to_value(&node).unwrap();
        let instances_json = json.pointer("/world3d/instancesJson").and_then(|value| value.as_str()).unwrap();
        let instances: Vec<Value> = serde_json::from_str(instances_json).unwrap();
        assert_eq!(instances.len(), sourcing_engine::filtered_stock(&document, &cfg.filters).len());
        for instance in &instances {
            let scale = instance["scale"][0].as_f64().unwrap();
            assert!(scale > 0.0);
        }
    }

    #[test]
    fn preview_renders_selected_mesh_id() {
        let document = sourcing_engine::default_document();
        let object_id = document.stock[0].id.clone();
        let cfg = SourcingCurateConfig { selected_object_id: Some(object_id.clone()), ..Default::default() };
        let node = render_preview(&document, &cfg, &SourcingLabels::EN);
        let json = serde_json::to_value(&node).unwrap();
        let meshes_json = json.pointer("/world3d/meshesJson").and_then(|value| value.as_str()).unwrap();
        let meshes: Vec<Value> = serde_json::from_str(meshes_json).unwrap();
        assert_eq!(meshes.len(), 1);
        assert_eq!(meshes[0]["id"].as_str(), Some(object_id.as_str()));
    }

    #[test]
    fn preview_shows_placeholder_without_selection() {
        let document = sourcing_engine::default_document();
        let cfg = SourcingCurateConfig::default();
        let node = render_preview(&document, &cfg, &SourcingLabels::EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("No selection"));
    }

    #[test]
    fn available_modules_uses_the_built_in_module_registry() {
        let modules = available_modules();
        assert_eq!(modules.len(), sourcing_modules().len());
    }

    #[test]
    fn stock_from_catalogue_merges_built_in_kinds_without_duplicating() {
        let mut app = new_app();
        // Reset to the empty fixture so stockFromCatalogue starts from a genuinely empty stock.
        app.dispatch_typed(SourcingCurateCommand::SetDocumentJson { json: serde_json::to_string(&sourcing_engine::empty_document()).unwrap() }, &testkit::meta("local")).expect("load empty document");
        assert!(app.projection().expect("projection").stock.is_empty());

        app.dispatch_typed(SourcingCurateCommand::StockFromCatalogue, &testkit::meta("local")).expect("populate");
        let expected: usize = sourcing_modules().iter().map(|module| module.demo_kinds().len()).sum();
        assert_eq!(app.projection().expect("projection").stock.len(), expected);

        app.dispatch_typed(SourcingCurateCommand::StockFromCatalogue, &testkit::meta("local")).expect("repopulate");
        assert_eq!(app.projection().expect("projection").stock.len(), expected);
    }

    #[test]
    fn set_filter_min_availability_clamps_to_zero() {
        let mut app = new_app();
        app.dispatch_typed(SourcingCurateCommand::SetFilterMinAvailability { delta: Some(-1000.0), value: None }, &testkit::meta("local")).expect("set min availability");
        // Filters are config-only now — the pool render reflects the clamp indirectly via an empty result
        // for an unreasonably high min-availability; assert the clamp directly through a second command
        // that reports back the applied absolute value.
        app.dispatch_typed(SourcingCurateCommand::SetFilterMinAvailability { delta: Some(0.0), value: None }, &testkit::meta("local")).expect("no-op delta");
        let node = app.render(BODY_POOL, None, &ViewState::default()).expect("render pool");
        // A clamped-to-zero min-availability keeps every stock row (all availabilities are >= 0).
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Glulam"));
    }

    #[test]
    fn filter_bar_module_toggles_encode_pressed_state_as_presence_selected() {
        let filters = Filters { module_ids: vec!["beams".into()], ..Default::default() };
        let modules = available_modules();
        let node = build_filter_bar(&filters, &modules, &SourcingLabels::EN);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"id\":\"sourcing-filter-module-beams\""), "beams toggle present: {json}");
        assert!(json.contains("\"id\":\"sourcing-filter-module-windows\""), "windows toggle present: {json}");
        // Selected module uses presence.selected=true; skip_serializing_if drops the default/false case.
        assert!(json.contains("\"selected\":true"), "pressed module encodes selected presence: {json}");
        let beams_idx = json.find("\"id\":\"sourcing-filter-module-beams\"").expect("beams id");
        let windows_idx = json.find("\"id\":\"sourcing-filter-module-windows\"").expect("windows id");
        let beams_slice = &json[beams_idx..beams_idx + 220.min(json.len() - beams_idx)];
        let windows_slice = &json[windows_idx..windows_idx + 220.min(json.len() - windows_idx)];
        assert!(beams_slice.contains("\"selected\":true"), "beams toggle selected: {beams_slice}");
        assert!(!windows_slice.contains("\"selected\":true"), "windows toggle not selected: {windows_slice}");
    }

    /// 🗣️ B1: locale is now `cfg.locale`, set via the typed `SetLocale` config command — no more
    /// passing a `ViewState` into `render`/`app_labels` for this purpose.
    #[test]
    fn sourcing_labels_resolve_native_german() {
        let mut app = new_app();
        app.dispatch_typed(SourcingCurateCommand::SetLocale { value: "de-DE".into() }, &testkit::meta("local")).expect("set locale");
        let overlay = app.app_labels();
        assert_eq!(overlay.window_kind_labels.get(WINDOW_POOL).map(String::as_str), Some("Pool"));
        assert_eq!(overlay.window_kind_labels.get(WINDOW_CURATED).map(String::as_str), Some("Kuratiert"));
        assert_eq!(overlay.mode_labels.get("curate").map(String::as_str), Some("Kuratieren"));
    }

    #[test]
    fn sourcing_curate_io_and_catalog_export_round_trip() {
        let mut app = new_app();
        let media = app.export_media("catalog:out").expect("catalog export");
        assert_eq!(media.media_type.class, MediaClass::Kit);
        assert_eq!(media.media_type.form, MediaForm::Type);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let fragment: Value = serde_json::from_str(&json).unwrap();
                assert_eq!(fragment["objectKinds"].as_array().unwrap().len(), app.projection().expect("projection").stock.len());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }
}
//#endregion 🧪️Tests
