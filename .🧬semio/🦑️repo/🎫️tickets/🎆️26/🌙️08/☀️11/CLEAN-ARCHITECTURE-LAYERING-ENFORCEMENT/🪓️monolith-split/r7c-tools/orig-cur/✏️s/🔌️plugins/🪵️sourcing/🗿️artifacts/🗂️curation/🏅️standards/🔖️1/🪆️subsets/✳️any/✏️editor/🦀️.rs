//! 🛒️ Sourcing curation app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared
//! compute in the artifact's `🧬️schema`. This file is a routing table: `handle` → `SourcingCurationCommand::
//! dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one `definition()` per node.

use crate::standards::v1::subsets::any::schema::mutations::SourcingMutation;
use crate::{CurationSnapshot, CuratedItem, ObjectKindExtra, SOURCING_CURATION_SCHEMA};
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation, SetFilterQueryEdit, SetFilterModulesEdit, SetFilterTypologyEdit, SetFilterMinAvailabilityEdit, SetSortEdit, SetContributionsEdit};
use crate::editor::sourcing::modes::edit;
use crate::editor::sourcing::modes::edit::windows::grid::config::{self as grid_config, GridWindowConfigMutation};
use crate::editor::sourcing::modes::edit::windows::{curated, grid, pool, preview};
use crate::editor::sourcing::presence::{self, SourcingCurationPresence, SourcingCurationPresenceMutation};
use crate::editor::sourcing::terminology::sourcing_curation_labels;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (keyed off
/// `SOURCING_CURATION_SCHEMA`, `MediaType{Kit,Kit}` matching the `"catalogue.sourcing"` `ArtifactKindSpec`)
/// plus the extra `catalog:out` output port: this app's `stock` (its `"catalogue.kinds"`-shaped rows)
/// mapped into the SAME `kit.catalog` JSON shape `block_3d::puzzle3d_catalog_fragment` produces, so
/// `s/plugin/puzzle`'s `kit:in` importer can consume either producer identically without knowing which
/// one it came from (see `crate::schema::inferences::sourcing_catalog_fragment`).
pub fn sourcing_curation_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: SOURCING_CURATION_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Kit },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: "Catalog".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "catalogue.sourcing".into(), name: "Sourcing Curation".into(), dimension: "data".into(), component_kind: "catalogue".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Constants
/// 🎯️ Still used as a plain namespace tag for `ActionFactory`/`TableScene`/`WorldScene` addressing
/// below — NOT a trait const any more (contract §7.4: the real surface id is now derived from
/// `SOURCING_DIALECT` + `AppRole` via `surface_app_id`, never hand-written).
pub const SOURCING_CONTROLLER_ID: &str = "sourcing-curation";
pub const SOURCING_DRAG_MIME: &str = "application/x-semio-sourcing-object";
/// 🕹️ The framework interaction domain every sourcing table row and grid object is picked in.
pub const SOURCING_ROWS_DOMAIN: &str = "rows";
/// 🕹️ The granularity a stock kind is picked under in [`SOURCING_ROWS_DOMAIN`].
pub const SOURCING_OBJECT_GRANULARITY: &str = "object";
pub const DEMO_STOCK_EXAMPLE_ID: &str = crate::examples::demo::ID;
/// 🫙️ The shell's "no example" selection.
pub const EMPTY_EXAMPLE_ID: &str = "";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// builds its `on_change`/drop actions with.
pub fn sourcing_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(SOURCING_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Bridges semantic app actions into the retained window-measure transport.
pub fn sourcing_window_action(action: &str, args: Option<semio_framework_value::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: SOURCING_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🎯️ The table-scene action descriptor a stepper, row button or drop target dispatches to this app.
pub fn sourcing_table_action(action: &str, object_id: Option<&str>) -> ActionDescriptor {
    let args = object_id.map(|id| semio_framework_value::DslValue::object([("objectId".to_string(), semio_framework_value::DslValue::String(id.to_string()))]));
    ActionDescriptor { controller_id: SOURCING_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🧾️ One table row record for a stock kind: its id, the `{objectId}` payload a drag carries under
/// [`SOURCING_DRAG_MIME`], and `cells` keyed by column id.
pub fn sourcing_table_row(object_id: &str, cells: Vec<(&str, semio_framework_plugin::TableCell)>) -> semio_framework_value::DslValue {
    let mut entries = vec![
        ("id".to_string(), semio_framework_value::DslValue::String(object_id.to_string())),
        ("_drag".to_string(), semio_framework_value::DslValue::object([("objectId".to_string(), semio_framework_value::DslValue::String(object_id.to_string()))])),
    ];
    entries.extend(cells.into_iter().map(|(column, cell)| (column.to_string(), semio_framework_value::ToValue::to_value(&cell))));
    semio_framework_value::DslValue::object(entries)
}

/// 📊️ One sourcing table surface: `(id, label, sortable)` columns, [`sourcing_table_row`] records
/// draggable under [`SOURCING_DRAG_MIME`], and `drop_action` answering a row dropped onto it.
pub fn sourcing_table(surface_id: &str, columns: &[(&str, &str, bool)], rows: Vec<semio_framework_value::DslValue>, drop_action: &str, sort: Option<&crate::TableSort>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let columns = columns
        .iter()
        .map(|(id, label, sortable)| semio_framework_value::DslValue::object([("id".to_string(), semio_framework_value::DslValue::String(id.to_string())), ("label".to_string(), semio_framework_value::DslValue::String(label.to_string())), ("sortable".to_string(), semio_framework_value::DslValue::Bool(*sortable))]))
        .collect();
    let mut scene = semio_framework_plugin::TableScene::base(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(columns)), semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::Array(rows)));
    scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into());
    scene.drop_action_json = Some(semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&sourcing_table_action(drop_action, None))));
    scene.sort_json = sort.map(|sort| semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(sort)));
    scene.domain_id = Some(SOURCING_ROWS_DOMAIN.into());
    scene.domain_granularity_id = Some(SOURCING_OBJECT_GRANULARITY.into());
    semio_framework_plugin::scene_surface(surface_id, semio_framework_plugin::plugin_app_close_prelude::SurfaceKind::Table, &scene)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
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
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SourcingCurationApp::Command` — the SOLE dispatch surface for curation's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`) and the `dsl` wire keyword (the kebab-case `#[dsl(key = ..)]` the codec uses) —
    /// **Row order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SourcingCurationCommand for CurationSnapshot, SourcingMutation, SourcingCurationConfig, SourcingCurationConfigMutation {
        "loadDocumentJson" as "load-document-json" => load_document_json::LoadDocumentJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "stockFromCatalogue" as "stock-from-catalogue" => stock_from_catalogue::StockFromCatalogue,
        "curationAdd" as "curation-add" => curation_add::CurationAdd,
        "curationSetCount" as "curation-set-count" => curation_set_count::CurationSetCount,
        "curationRemove" as "curation-remove" => curation_remove::CurationRemove,
        "dropOnPool" as "drop-on-pool" => drop_on_pool::DropOnPool,
        "dropOnCurated" as "drop-on-curated" => drop_on_curated::DropOnCurated,
        "setFilterQuery" as "filter-query" => set_filter_query::SetFilterQuery,
        "setFilterModule" as "filter-module" => set_filter_module::SetFilterModule,
        "setFilterTypology" as "filter-typology" => set_filter_typology::SetFilterTypology,
        "setFilterMinAvailability" as "filter-min-availability" => set_filter_min_availability::SetFilterMinAvailability,
        "sortTable" as "sort-table" => sort_table::SortTable,
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "setGridInstanceDisplay" as "grid-instance-display" => set_grid_instance_display::SetGridInstanceDisplay,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use crate::editor::sourcing::commands::set_contributions;
use crate::editor::sourcing::commands::{curation_add, curation_remove, curation_set_count, drop_on_curated, drop_on_pool};
use crate::editor::sourcing::commands::{set_active_example, load_document_json, stock_from_catalogue};
use crate::editor::sourcing::commands::{set_filter_min_availability, set_filter_module, set_filter_query, set_filter_typology, set_grid_instance_display, sort_table};

/// 🎯️ Host action id + JSON args → the closed `SourcingCurationCommand` vocabulary — the production
/// bridge between the manifest's *declared* action surface (`🔖️Manifest`, camelCase arg names) and
/// the typed command channel that actually dispatches.
///
/// Chrome built by the `🎭️modes` nodes addresses this app by action id (`sourcing_action`), and
/// several of those rows carry `None` args because the host fills them at dispatch time from the
/// interaction itself (a text input's `value`, a slider's `delta`, a drop's `objectId`) — so every
/// field is read defensively and coerced, never assumed present.
///
/// Without this, `ArtifactApp::command_from_action`'s default rejects every app-owned action and the
/// pane cannot even load its own example. See `📐️cad`'s `cad_command_from_action` twin.
fn sourcing_curation_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<SourcingCurationCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_bool);
    let text_of = |key: &str| -> Option<String> {
        args.and_then(|value| value.get(key)).and_then(|value| match value {
            semio_framework_value::DslValue::String(text) => Some(text.clone()),
            semio_framework_value::DslValue::Bool(flag) => Some(flag.to_string()),
            semio_framework_value::DslValue::Number(number) => Some(match number {
                semio_framework_value::Number::UInt(number) => number.to_string(),
                semio_framework_value::Number::Int(number) => number.to_string(),
                semio_framework_value::Number::Float(number) => number.to_string(),
            }),
            _ => None,
        })
    };
    let json_field = |key: &str| -> String {
        match args.and_then(|value| value.get(key)) {
            Some(semio_framework_value::DslValue::String(text)) => text.clone(),
            Some(other) => semio_framework_pack_json::to_json_string(other),
            None => args.map(semio_framework_pack_json::to_json_string).unwrap_or_default(),
        }
    };
    let object_id = || str_field("objectId").unwrap_or_default();
    Ok(match action {
        "loadDocumentJson" => SourcingCurationCommand::LoadDocumentJson(load_document_json::LoadDocumentJson { json: json_field("json") }),
        "setActiveExample" => SourcingCurationCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
        "stockFromCatalogue" => SourcingCurationCommand::StockFromCatalogue(stock_from_catalogue::StockFromCatalogue {}),
        "curationAdd" => SourcingCurationCommand::CurationAdd(curation_add::CurationAdd { object_id: object_id() }),
        "curationSetCount" => SourcingCurationCommand::CurationSetCount(curation_set_count::CurationSetCount { object_id: object_id(), delta: f64_field("delta"), value: f64_field("value") }),
        "curationRemove" => SourcingCurationCommand::CurationRemove(curation_remove::CurationRemove { object_id: object_id() }),
        "dropOnPool" => SourcingCurationCommand::DropOnPool(drop_on_pool::DropOnPool { object_id: object_id() }),
        "dropOnCurated" => SourcingCurationCommand::DropOnCurated(drop_on_curated::DropOnCurated { object_id: object_id() }),
        "setFilterQuery" => SourcingCurationCommand::SetFilterQuery(set_filter_query::SetFilterQuery { value: text_of("value").unwrap_or_default() }),
        "setFilterModule" => SourcingCurationCommand::SetFilterModule(set_filter_module::SetFilterModule { module_id: str_field("moduleId").unwrap_or_default(), enabled: bool_field("enabled").unwrap_or(false) }),
        "setFilterTypology" => SourcingCurationCommand::SetFilterTypology(set_filter_typology::SetFilterTypology { path: str_field("path").or_else(|| text_of("value")).unwrap_or_default() }),
        "setFilterMinAvailability" => SourcingCurationCommand::SetFilterMinAvailability(set_filter_min_availability::SetFilterMinAvailability { delta: f64_field("delta"), value: f64_field("value") }),
        "sortTable" => SourcingCurationCommand::SortTable(sort_table::SortTable { column_id: str_field("columnId").unwrap_or_default(), direction: str_field("direction").unwrap_or_default() }),
        "setContributions" => SourcingCurationCommand::SetContributions(set_contributions::SetContributions { json: json_field("json") }),
        "setGridInstanceDisplay" => SourcingCurationCommand::SetGridInstanceDisplay(set_grid_instance_display::SetGridInstanceDisplay { value: str_field("value").or_else(|| text_of("value")).unwrap_or_default() }),
        other => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.unsupported"), format!("action '{other}' is not a sourcing curation command"))),
    })
}
//#endregion 🔖️Commands

//#region 🔖️SourcingCurationApp
/// 🧪️ Unit struct — every former app-struct field lives in `crate::editor::sourcing::config::
/// SourcingCurationConfig`, written through `SourcingCurationConfigMutation`s.
#[derive(Default)]
pub struct SourcingCurationApp;

#[path = "🧰️retained-commands/🦀️.rs"]
mod retained_commands;
use self::retained_commands::*;

#[path = "📬️config-store-preparation/🦀️.rs"]
mod config_store_preparation;
pub(crate) use self::config_store_preparation::*;

#[path = "🗄️artifact-store-preparation/🦀️.rs"]
mod artifact_store_preparation;
use self::artifact_store_preparation::*;

#[path = "🧩️sourcing-app/🦀️.rs"]
mod sourcing_app;
use self::sourcing_app::*;
//#endregion 🔖️SourcingCurationApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `document` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (JSON import, load-
/// example, bulk catalogue restock). Per `📓️taxonomy.md`, the former whole-snapshot-replace variant
/// is banned outright with NO replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this app (`import_media`'s
/// `"artifact:in"` above, `commands::document::{set_active_example, load_document_json,
/// stock_from_catalogue}`) builds this effect instead of an `Emit::mutations([...])`. The spr is a
/// fresh, edit-free op-log for `document` — a genesis envelope with no history to encode.
pub fn reset_document_effect(document: &CurationSnapshot) -> semio_framework::kernel::Effect {
    let pack = <CurationSnapshot as ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("curation", SOURCING_CURATION_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use self::manifest::*;

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
