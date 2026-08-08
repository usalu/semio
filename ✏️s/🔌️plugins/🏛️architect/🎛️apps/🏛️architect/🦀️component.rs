//! 🏛️ Architect play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the five window
//! surfaces in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, view state in
//! `🦀️config.rs`, presentation factories in `🦀️chrome.rs`, the register bridge in `🦀️catalog.rs`, and
//! all document-side compute in `crate::artifacts::program::engine`.

use crate::apps::architect::catalog::{analysis_kind_picker_options, parse_entity_id, parse_entity_id_from_args, parse_register_id, report_kind_picker_options, REGISTER_IDS};
use crate::apps::architect::commands::adjacency::{set_adjacency_field, set_adjacency_filter, set_adjacency_kind};
use crate::apps::architect::commands::analysis::{run_analysis, run_report, run_validation};
use crate::apps::architect::commands::element::{add_element, remove_element};
use crate::apps::architect::commands::exchange::{export_program, export_registers_csv, import_program, import_program_request, import_registers_csv};
use crate::apps::architect::commands::graph::{node_graph_edit, node_graph_viewport};
use crate::apps::architect::commands::register::{add_register_item, patch_register_item, remove_register_item, select_register};
use crate::apps::architect::commands::search::query;
use crate::apps::architect::commands::selection::set_selection;
use crate::apps::architect::commands::template::apply;
use crate::apps::architect::config::{ArchitectConfig, ArchitectConfigMutation};
use crate::apps::architect::modes::edit as edit_mode;
use crate::apps::architect::modes::edit::windows::{adjacency as adjacency_window, graph as graph_window, register as register_window, report as report_window, trace as trace_window};
use crate::apps::architect::modes::{report as report_mode, review as review_mode};
use crate::apps::architect::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::artifacts::program::op::ProgramMutation;
use crate::artifacts::program::{empty_plugin, sample_plugin, Program, ARCHITECT_PROGRAM_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, UiNode};
use store::EngineHandles;
use serde_json::Value;

//#region 🔖️Constants
pub const ARCHITECT_APP_ID: &str = "architect";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🪟️windows/*`) builds its item/on-change actions with.
pub fn architect_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(ARCHITECT_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ B1: `ArchitectPlayApp::Command` — the sole typed dispatch surface, one row per action declared
    /// on `create_architect_app`'s `AppBuilder`. Row order IS the binary variant ordinal: appending is
    /// safe, reordering is a wire-format break. Each row's first literal is the camelCase manifest action
    /// id (`command_id()`); the second is the kebab `#[dsl(key)]` wire keyword the codec uses — both are
    /// copied verbatim off the pre-migration `ArchitectCommand` enum, never derived from one another.
    ///
    /// JSON blob arguments (patches, CSV, DSL payloads, node-graph edit lists, viewport JSON) stay
    /// `String`-typed and are parsed inside each handler — mirrors `gis2d`'s `positions_json`/`camera_json`
    /// convention for the same reason (their shapes have no `dsl::DslField` binding of their own).
    pub enum ArchitectCommand for Program, ProgramMutation, ArchitectConfig, ArchitectConfigMutation {
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "selectRegister" as "select-register" => select_register::SelectRegister,
        "addRegisterItem" as "add-register-item" => add_register_item::AddRegisterItem,
        "removeRegisterItem" as "remove-register-item" => remove_register_item::RemoveRegisterItem,
        "patchRegisterItem" as "patch-register-item" => patch_register_item::PatchRegisterItem,
        "setAdjacencyField" as "set-adjacency-field" => set_adjacency_field::SetAdjacencyField,
        "applyTemplate" as "apply-template" => apply::ApplyTemplate,
        "exportRegistersCsv" as "export-registers-csv" => export_registers_csv::ExportRegistersCsv,
        "importRegistersCsv" as "import-registers-csv" => import_registers_csv::ImportRegistersCsv,
        "addElement" as "add-element" => add_element::AddElement,
        "removeElement" as "remove-element" => remove_element::RemoveElement,
        "runValidation" as "run-validation" => run_validation::RunValidation,
        "runAnalysis" as "run-analysis" => run_analysis::RunAnalysis,
        "runReport" as "run-report" => run_report::RunReport,
        "exportProgram" as "export-program" => export_program::ExportProgram,
        "importProgramRequest" as "import-program-request" => import_program_request::ImportProgramRequest,
        "importProgram" as "import-program" => import_program::ImportProgram,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setAdjacencyKind" as "set-adjacency-kind" => set_adjacency_kind::SetAdjacencyKind,
        "search" as "search" => query::Search,
        "setAdjacencyFilter" as "set-adjacency-filter" => set_adjacency_filter::SetAdjacencyFilter,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ArchitectPlayApp
/// 🧪️ B1: unit struct — every former `RefCell<ArchitectPlayRuntime>` field now lives in
/// `crate::apps::architect::config::ArchitectConfig`, written through `ArchitectConfigMutation`s.
#[derive(Default)]
pub struct ArchitectPlayApp;

impl DocumentApp for ArchitectPlayApp {
    type Projection = Program;
    type Mutation = ProgramMutation;
    type Config = ArchitectConfig;
    type ConfigMutation = ArchitectConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;

    type Command = ArchitectCommand;

    const APP_ID: &'static str = ARCHITECT_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = ARCHITECT_PROGRAM_SCHEMA;

    fn initial_projection() -> Program {
        sample_plugin()
    }

    fn initial_config() -> ArchitectConfig {
        ArchitectConfig { active_register: "elements".into(), ..ArchitectConfig::default() }
    }

    fn command_id(command: &ArchitectCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `ArchitectCommand` — React/wgpu still speak the
    /// stringly `{action,args}` wire; this is the typed-command bridge until those call sites send
    /// `OpBinary` bytes directly (mirrors `gis2d`'s `command_from_action`).
    fn command_from_action(action: &str, args: Option<&Value>) -> Result<ArchitectCommand, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_bool);
        match action {
            "setSelection" => Ok(ArchitectCommand::SetSelection(set_selection::SetSelection {
                ids: args.and_then(|value| value.get("ids")).and_then(Value::as_array).map(|ids| ids.iter().filter_map(|value| value.as_str().map(str::to_string)).collect()).unwrap_or_default(),
            })),
            "selectRegister" => Ok(ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: parse_register_id(args).unwrap_or_default() })),
            "addRegisterItem" => Ok(ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                name: str_field("name").unwrap_or_else(|| "New Item".into()),
                template_id: str_field("templateId"),
            })),
            "removeRegisterItem" => Ok(ArchitectCommand::RemoveRegisterItem(remove_register_item::RemoveRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
            })),
            "patchRegisterItem" => Ok(ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem {
                register_id: parse_register_id(args).unwrap_or_default(),
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                patch_json: args.and_then(|value| value.get("patch")).map_or_else(|| "null".into(), Value::to_string),
            })),
            "setAdjacencyField" => Ok(ArchitectCommand::SetAdjacencyField(set_adjacency_field::SetAdjacencyField {
                entity_id: parse_entity_id_from_args(args, "entityId").map(|id| id.0).unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value_json: args.and_then(|value| value.get("value")).map_or_else(|| "null".into(), Value::to_string),
            })),
            "applyTemplate" => Ok(ArchitectCommand::ApplyTemplate(apply::ApplyTemplate { template_id: parse_entity_id_from_args(args, "templateId").map(|id| id.0).unwrap_or_default() })),
            "exportRegistersCsv" => Ok(ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {})),
            "importRegistersCsv" => Ok(ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv {
                csv: str_field("csv").unwrap_or_default(),
                strategy: str_field("strategy").unwrap_or_else(|| "upsert".into()),
            })),
            "addElement" => Ok(ArchitectCommand::AddElement(add_element::AddElement { name: str_field("name").unwrap_or_else(|| "New Room".into()) })),
            "removeElement" => Ok(ArchitectCommand::RemoveElement(remove_element::RemoveElement { element_id: str_field("elementId").or_else(|| str_field("id")).unwrap_or_default() })),
            "runValidation" => Ok(ArchitectCommand::RunValidation(run_validation::RunValidation {})),
            "runAnalysis" => Ok(ArchitectCommand::RunAnalysis(run_analysis::RunAnalysis { analysis_kind: str_field("analysisKind").unwrap_or_else(|| "gap".into()) })),
            "runReport" => Ok(ArchitectCommand::RunReport(run_report::RunReport { report_kind: str_field("reportKind").unwrap_or_else(|| "executiveSummary".into()) })),
            "exportProgram" => Ok(ArchitectCommand::ExportProgram(export_program::ExportProgram {})),
            "importProgramRequest" => Ok(ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {})),
            "importProgram" => Ok(ArchitectCommand::ImportProgram(import_program::ImportProgram { payload: str_field("payload").or_else(|| str_field("dsl")).unwrap_or_default() })),
            "nodeGraphEdit" => Ok(ArchitectCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit {
                operations_json: args.and_then(|value| value.get("operations")).map_or_else(|| "[]".into(), Value::to_string),
            })),
            "nodeGraphViewport" => Ok(ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: str_field("viewportJson").unwrap_or_default() })),
            "setAdjacencyKind" => Ok(ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind {
                element_a_id: parse_entity_id(args, "elementAId").map(|id| id.0).unwrap_or_default(),
                element_b_id: parse_entity_id(args, "elementBId").map(|id| id.0).unwrap_or_default(),
                kind: str_field("kind"),
                cycle: bool_field("cycle").unwrap_or(false),
            })),
            "search" => Ok(ArchitectCommand::Search(query::Search { query: str_field("query").unwrap_or_default() })),
            "setAdjacencyFilter" => Ok(ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: str_field("kind") })),
            other => Err(Fault::from(format!("architect: unhandled action id {other}"))),
        }
    }

    fn handle(command: &ArchitectCommand, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<ProgramMutation, ArchitectConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Program>, cfg: &ConfigView<'_, ArchitectConfig>) -> UiNode {
        let program = doc.projection;
        let config = cfg.projection;
        match body_key {
            adjacency_window::ARCHITECT_BODY_ADJACENCY => adjacency_window::render(program, config),
            graph_window::ARCHITECT_BODY_GRAPH => graph_window::render(program, config),
            register_window::ARCHITECT_BODY_REGISTER => register_window::render(program, config),
            report_window::ARCHITECT_BODY_REPORT => report_window::render(config),
            trace_window::ARCHITECT_BODY_TRACE => trace_window::render(program, config),
            document_panel::ARCHITECT_BODY_DOCUMENT => document_panel::render(program, config),
            catalogue_panel::ARCHITECT_BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::ARCHITECT_BODY_INSPECTION => inspection_panel::render(program, config),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️ArchitectPlayApp

//#region 🔖️Manifest
pub fn create_architect_app() -> App {
    App::from_builder(
        App::builder(ARCHITECT_APP_ID, LocalizedLabel::native("Architect", "Architekt"))
            .document(["semio", "architect"])
            .icon_id("architect")
            .mode_def(edit_mode::definition())
            .mode_def(review_mode::definition())
            .mode_def(report_mode::definition())
            .default_mode_id(edit_mode::ARCHITECT_MODE_EDIT)
            .window_kind_def(adjacency_window::definition())
            .window_kind_def(graph_window::definition())
            .window_kind_def(register_window::definition())
            .window_kind_def(report_window::definition())
            .window_kind_def(trace_window::definition())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("setAdjacencyKind", LocalizedLabel::native("Set Adjacency Kind", "Adjazenzart festlegen"))
            .mutation("addRegisterItem", LocalizedLabel::native("Add Register Item", "Registereintrag hinzufügen"))
            .mutation("removeRegisterItem", LocalizedLabel::native("Remove Register Item", "Registereintrag entfernen"))
            .mutation("patchRegisterItem", LocalizedLabel::native("Patch Register Item", "Registereintrag patchen"))
            .mutation("importProgram", LocalizedLabel::native("Import Program", "Programm importieren"))
            .mutation("importRegistersCsv", LocalizedLabel::native("Import Registers CSV", "Register CSV importieren"))
            .mutation("applyTemplate", LocalizedLabel::native("Apply Template", "Vorlage anwenden"))
            .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            .view_action("selectRegister", LocalizedLabel::native("Select Register", "Register wählen"))
            .view_action("addElement", LocalizedLabel::native("Add Element", "Element hinzufügen"))
            .view_action("removeElement", LocalizedLabel::native("Remove Element", "Element entfernen"))
            .view_action("setAdjacencyField", LocalizedLabel::native("Set Adjacency Field", "Adjazenzfeld setzen"))
            .view_action("runValidation", LocalizedLabel::native("Run Validation", "Validierung ausführen"))
            .view_action("runAnalysis", LocalizedLabel::native("Run Analysis", "Analyse ausführen"))
            .view_action("runReport", LocalizedLabel::native("Run Report", "Bericht erzeugen"))
            .view_action("search", LocalizedLabel::native("Search", "Suchen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .shell_action("exportProgram", LocalizedLabel::native("Export Program", "Programm exportieren"))
            .shell_action("exportRegistersCsv", LocalizedLabel::native("Export Registers CSV", "Register CSV exportieren"))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setAdjacencyFilter", LocalizedLabel::native("Set Adjacency Filter", "Adjazenzfilter setzen"), ActionKind::View) })
            .action_args("selectRegister", vec![ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect())])
            .action_args(
                "addRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("name", LocalizedLabel::native("Name", "Name")),
                    ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID")),
                ],
            )
            .action_args(
                "removeRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                ],
            )
            .action_args(
                "patchRegisterItem",
                vec![
                    ActionArgDef::select("registerId", LocalizedLabel::native("Register", "Register"), REGISTER_IDS.iter().map(|register| ActionArgOption::new(*register, LocalizedLabel::data(*register))).collect()),
                    ActionArgDef::text("entityId", LocalizedLabel::native("Entity Id", "Entitäts-ID")),
                    ActionArgDef::text("patch", LocalizedLabel::native("Patch JSON", "Patch-JSON")),
                ],
            )
            .action_args("applyTemplate", vec![ActionArgDef::text("templateId", LocalizedLabel::native("Template Id", "Vorlagen-ID"))])
            .action_args(
                "importRegistersCsv",
                vec![
                    ActionArgDef::text("csv", LocalizedLabel::native("CSV", "CSV")),
                    ActionArgDef::select(
                        "strategy",
                        LocalizedLabel::native("Strategy", "Strategie"),
                        vec![
                            ActionArgOption::new("upsert", LocalizedLabel::native("Upsert", "Upsert")),
                            ActionArgOption::new("replace", LocalizedLabel::native("Replace", "Ersetzen")),
                            ActionArgOption::new("skipDuplicates", LocalizedLabel::native("Skip Duplicates", "Duplikate überspringen")),
                        ],
                    ),
                ],
            )
            .action_args(
                "setAdjacencyKind",
                vec![ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    vec![
                        ActionArgOption::new("required", LocalizedLabel::native("Required", "Erforderlich")),
                        ActionArgOption::new("preferred", LocalizedLabel::native("Preferred", "Bevorzugt")),
                        ActionArgOption::new("optional", LocalizedLabel::native("Optional", "Optional")),
                        ActionArgOption::new("prohibited", LocalizedLabel::native("Prohibited", "Verboten")),
                    ],
                )],
            )
            .action_args("runAnalysis", vec![ActionArgDef::select("analysisKind", LocalizedLabel::native("Analysis", "Analyse"), analysis_kind_picker_options())])
            .action_args("runReport", vec![ActionArgDef::select("reportKind", LocalizedLabel::native("Report", "Bericht"), report_kind_picker_options())])
            .action_args("search", vec![ActionArgDef::text("query", LocalizedLabel::native("Query", "Suchanfrage"))])
            .action_args("importProgram", vec![ActionArgDef::text("payload", LocalizedLabel::native("Program DSL", "Programm-DSL"))])
            .default_layout(edit_mode::layout()),
    )
    .example("sample", LocalizedLabel::native("Sample Clinic", "Beispielklinik"), serde_json::to_string(&sample_plugin()).expect("sample_plugin is a static hand-built fixture with no non-finite floats or non-UTF8 keys"), "cylinder")
    .example("empty", LocalizedLabel::native("Empty Program", "Leeres Programm"), serde_json::to_string(&empty_plugin()).expect("empty_plugin is a static hand-built fixture with no non-finite floats or non-UTF8 keys"), "file")
    .workflow("architect", "Architect", "data")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{HistoryView, InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type ArchitectApp = VcsDocumentApp<ArchitectPlayApp>;

    pub fn new_app() -> ArchitectApp {
        sdk_new_app::<ArchitectPlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> ArchitectApp {
        new_app_with_registry::<ArchitectPlayApp>(create_architect_app)
    }

    pub fn dispatch(app: &mut ArchitectApp, command: ArchitectCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut ArchitectApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 🔀️ Drives a typed `ArchitectCommand` straight through `handle` against a bare
    /// `ArchitectPlayApp` — mirrors `cad`'s `drive`/`drive_with_config` harness.
    pub fn drive(command: &ArchitectCommand, program: &Program) -> Emit<ProgramMutation, ArchitectConfigMutation> {
        drive_with_config(command, program, &ArchitectPlayApp.initial_config())
    }

    pub fn drive_with_config(command: &ArchitectCommand, program: &Program, config: &ArchitectConfig) -> Emit<ProgramMutation, ArchitectConfigMutation> {
        let history = HistoryView::empty();
        let doc = DocumentView { projection: program, history: &history };
        let cfg = ConfigView { projection: config };
        ArchitectPlayApp.handle(command, &doc, &cfg).expect("handle")
    }

    /// 🧮️ Folds an `Emit`'s `config_mutations` onto a base `ArchitectConfig` — mirrors what
    /// `VcsDocumentApp`'s config store does when it dispatches them.
    pub fn config_after(emit: &Emit<ProgramMutation, ArchitectConfigMutation>, base: &ArchitectConfig) -> ArchitectConfig {
        use protocol::Mutation;
        let mut next = base.clone();
        for operation in &emit.config_mutations {
            next = operation.diff(&next);
        }
        next
    }

    pub fn render_direct(body_key: &str, program: &Program, config: &ArchitectConfig) -> UiNode {
        let history = HistoryView::empty();
        ArchitectPlayApp.render(body_key, &DocumentView { projection: program, history: &history }, &ConfigView { projection: config })
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::architect::catalog::{analysis_kind_from_str, register_entities};
    use crate::apps::architect::testkit;
    use crate::artifacts::program::engine::exchange::export_registers_csv;
    use crate::artifacts::program::registers::{AdjacencyKind, AnalysisKind};
    use protocol::CollectionMutation;
    use semio_framework_plugin::PluginApp;
    use serde_json::json;

    //#region 🔖️CommandSurface
    /// 🎯️ One value per `app_commands!` row — the fixture behind the wire laws below.
    fn every_command() -> Vec<ArchitectCommand> {
        vec![
            ArchitectCommand::SetSelection(set_selection::SetSelection { ids: vec!["a".into(), "b".into()] }),
            ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "risks".into() }),
            ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: None }),
            ArchitectCommand::RemoveRegisterItem(remove_register_item::RemoveRegisterItem { register_id: "elements".into(), entity_id: "e1".into() }),
            ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem { register_id: "elements".into(), entity_id: "e1".into(), patch_json: "{\"name\":\"X\"}".into() }),
            ArchitectCommand::SetAdjacencyField(set_adjacency_field::SetAdjacencyField { entity_id: "a1".into(), field: "kind".into(), value_json: "\"required\"".into() }),
            ArchitectCommand::ApplyTemplate(apply::ApplyTemplate { template_id: "t1".into() }),
            ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {}),
            ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv: "a,b".into(), strategy: "upsert".into() }),
            ArchitectCommand::AddElement(add_element::AddElement { name: "Room".into() }),
            ArchitectCommand::RemoveElement(remove_element::RemoveElement { element_id: "e1".into() }),
            ArchitectCommand::RunValidation(run_validation::RunValidation {}),
            ArchitectCommand::RunAnalysis(run_analysis::RunAnalysis { analysis_kind: "gap".into() }),
            ArchitectCommand::RunReport(run_report::RunReport { report_kind: "executiveSummary".into() }),
            ArchitectCommand::ExportProgram(export_program::ExportProgram {}),
            ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {}),
            ArchitectCommand::ImportProgram(import_program::ImportProgram { payload: "text".into() }),
            ArchitectCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
            ArchitectCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: "{}".into() }),
            ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: None, cycle: true }),
            ArchitectCommand::Search(query::Search { query: "hall".into() }),
            ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None }),
        ]
    }

    #[test]
    fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(ArchitectCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 22, "every ArchitectCommand row must be covered by every_command()");
    }

    #[test]
    fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
            let printed = protocol::OpText::print_op(&command);
            let keyword = printed.split_whitespace().next().unwrap_or_default().to_string();
            assert!(keyword.contains('-') || keyword == "search", "row {} printed a non-kebab keyword {printed:?}", command.command_id());
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for every row whose `Option`/`bool` fields make the
    /// `None`/`Some` cases distinct on the wire — copied verbatim out of the ticket's
    /// `🧪️wire-baseline-before.txt`, captured from the pre-migration hand-written `ArchitectCommand`.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &ArchitectCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&ArchitectCommand::SetSelection(set_selection::SetSelection { ids: Vec::new() })), "01000001000c00");
        assert_eq!(hex(&ArchitectCommand::SetSelection(set_selection::SetSelection { ids: vec!["a".into(), "b".into()] })), "0100020161016201000c0206000601");
        assert_eq!(
            hex(&ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: None })),
            "01020204526f6f6d08656c656d656e747302000601010600"
        );
        assert_eq!(
            hex(&ArchitectCommand::AddRegisterItem(add_register_item::AddRegisterItem { register_id: "elements".into(), name: "Room".into(), template_id: Some("t1".into()) })),
            "01020304526f6f6d08656c656d656e747302743103000601010600020602"
        );
        assert_eq!(hex(&ArchitectCommand::ExportRegistersCsv(export_registers_csv::ExportRegistersCsv {})), "01070000");
        assert_eq!(hex(&ArchitectCommand::RunValidation(run_validation::RunValidation {})), "010b0000");
        assert_eq!(hex(&ArchitectCommand::ExportProgram(export_program::ExportProgram {})), "010e0000");
        assert_eq!(hex(&ArchitectCommand::ImportProgramRequest(import_program_request::ImportProgramRequest {})), "010f0000");
        assert_eq!(
            hex(&ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: None, cycle: true })),
            "01130201610162030006000106010302"
        );
        assert_eq!(
            hex(&ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: "a".into(), element_b_id: "b".into(), kind: Some("required".into()), cycle: false })),
            "01130301610162087265717569726564040006000106010206020301"
        );
        assert_eq!(hex(&ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: None })), "01150000");
        assert_eq!(hex(&ArchitectCommand::SetAdjacencyFilter(set_adjacency_filter::SetAdjacencyFilter { kind: Some("required".into()) })), "01150108726571756972656401000600");
    }

    /// 🎯️ Every app-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`.
    #[test]
    fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<ArchitectPlayApp>(create_architect_app);
        assert!(ArchitectPlayApp.command_from_action("notARealAction", None).is_err());
    }

    /// 🎯️ Spot-check a representative sample of action ids round-tripping into the expected typed
    /// `ArchitectCommand` variant.
    #[test]
    fn command_from_action_bridges_declared_actions() {
        let app = ArchitectPlayApp;
        assert!(matches!(app.command_from_action("runValidation", None), Ok(ArchitectCommand::RunValidation(_))));
        assert!(matches!(app.command_from_action("search", Some(&json!({ "query": "hall" }))), Ok(ArchitectCommand::Search(query::Search { query })) if query == "hall"));
        assert!(matches!(
            app.command_from_action("selectRegister", Some(&json!({ "registerId": "risks" }))),
            Ok(ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id })) if register_id == "risks"
        ));
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_architect_app().definition;
        assert_eq!(definition.modes.len(), 3);
        assert_eq!(definition.window_kinds.len(), 5);
        for body_key in [document_panel::ARCHITECT_BODY_DOCUMENT, catalogue_panel::ARCHITECT_BODY_CATALOGUE, inspection_panel::ARCHITECT_BODY_INSPECTION] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        for window in [
            adjacency_window::ARCHITECT_WINDOW_ADJACENCY,
            graph_window::ARCHITECT_WINDOW_GRAPH,
            register_window::ARCHITECT_WINDOW_REGISTER,
            report_window::ARCHITECT_WINDOW_REPORT,
            trace_window::ARCHITECT_WINDOW_TRACE,
        ] {
            assert!(definition.window_kinds.iter().any(|kind| kind.id == window), "window kind {window} is stitched into the manifest");
        }
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, "architect.nope").contains("Unknown body"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Behavior
    #[test]
    fn adjacency_matrix_renders_triangle_strip() {
        let program = sample_plugin();
        let json = serde_json::to_string(&testkit::render_direct(adjacency_window::ARCHITECT_BODY_ADJACENCY, &program, &ArchitectPlayApp.initial_config())).expect("json");
        assert!(json.contains('▲'));
        assert!(json.contains("Reception"));
    }

    #[test]
    fn graph_body_emits_node_graph_scene() {
        let program = sample_plugin();
        let json = serde_json::to_string(&testkit::render_direct(graph_window::ARCHITECT_BODY_GRAPH, &program, &ArchitectPlayApp.initial_config())).expect("json");
        assert!(json.contains("node-graph"));
    }

    #[test]
    fn set_adjacency_kind_cycles_required_to_preferred() {
        let program = sample_plugin();
        let adjacency = program.adjacencies.first().expect("adjacency");
        let emit = testkit::drive(
            &ArchitectCommand::SetAdjacencyKind(set_adjacency_kind::SetAdjacencyKind { element_a_id: adjacency.element_a_id.0.clone(), element_b_id: adjacency.element_b_id.0.clone(), kind: None, cycle: true }),
            &program,
        );
        assert!(matches!(
            emit.document_mutations.first(),
            Some(ProgramMutation::SetAdjacency { adjacency: updated }) if updated.kind == AdjacencyKind::Preferred
        ));
    }

    #[test]
    fn run_validation_populates_last_result_json() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp.initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::RunValidation(run_validation::RunValidation {}), &program, &initial);
        assert!(!testkit::config_after(&emit, &initial).last_result_json.is_empty());
    }

    #[test]
    fn search_finds_sample_elements() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp.initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::Search(query::Search { query: "Reception".into() }), &program, &initial);
        let config = testkit::config_after(&emit, &initial);
        assert!(!config.selected_ids.is_empty());
        assert!(!config.search_history_json.is_empty());
    }

    #[test]
    fn select_register_switches_active_register() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp.initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::SelectRegister(select_register::SelectRegister { register_id: "stakeholders".into() }), &program, &initial);
        assert_eq!(testkit::config_after(&emit, &initial).active_register, "stakeholders");
        assert!(!register_entities(&program, "stakeholders").is_empty());
    }

    #[test]
    fn patch_register_item_updates_element_name() {
        let program = sample_plugin();
        let element_id = program.elements[0].header.id.clone();
        let emit = testkit::drive(
            &ArchitectCommand::PatchRegisterItem(patch_register_item::PatchRegisterItem { register_id: "elements".into(), entity_id: element_id.0, patch_json: json!({ "name": "Updated Reception" }).to_string() }),
            &program,
        );
        assert!(matches!(
            emit.document_mutations.first(),
            Some(ProgramMutation::Elements(CollectionMutation::Patch { patch, .. })) if patch.name.as_deref() == Some("Updated Reception")
        ));
    }

    #[test]
    fn formatted_report_renders_section_headings() {
        let program = sample_plugin();
        let initial = ArchitectPlayApp.initial_config();
        let emit = testkit::drive_with_config(&ArchitectCommand::RunReport(run_report::RunReport { report_kind: "executiveSummary".into() }), &program, &initial);
        let config = testkit::config_after(&emit, &initial);
        let json = serde_json::to_string(&testkit::render_direct(report_window::ARCHITECT_BODY_REPORT, &program, &config)).expect("json");
        assert!(json.contains("Overview"));
        assert!(json.contains("architect-report.section"));
    }

    #[test]
    fn analysis_kind_picker_maps_all_variants() {
        let options = analysis_kind_picker_options();
        assert_eq!(options.len(), 20);
        for option in &options {
            let kind = analysis_kind_from_str(&option.value);
            assert!(!format!("{kind:?}").is_empty(), "missing mapping for {}", option.value);
        }
        assert_eq!(analysis_kind_from_str("relationshipAnalysis"), AnalysisKind::RelationshipAnalysis);
    }

    #[test]
    fn import_registers_csv_action_sets_plugin() {
        let program = sample_plugin();
        let csv = export_registers_csv(&program).expect("export csv");
        let emit = testkit::drive(&ArchitectCommand::ImportRegistersCsv(import_registers_csv::ImportRegistersCsv { csv, strategy: "upsert".into() }), &program);
        assert!(matches!(emit.document_mutations.first(), Some(ProgramMutation::SetProgram { .. })));
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app();
        let before = app.projection().expect("projection").elements.len();
        testkit::dispatch(&mut app, ArchitectCommand::AddElement(add_element::AddElement { name: "Ward".into() }));
        assert_eq!(app.projection().expect("projection").elements.len(), before + 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").elements.len(), before);
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").elements.len(), before + 1);
    }

    /// 🧬️ Kind-discipline wrapper: the real registry enforces View actions never emit document
    /// operations. Exercising it here (rather than only the plain `new_app()`) is the reason
    /// `testkit::app_with_registry` exists.
    #[test]
    fn view_actions_never_emit_document_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, ArchitectCommand::SetSelection(set_selection::SetSelection { ids: vec!["e1".into()] }));
        assert!(result.mutations.is_empty(), "setSelection is a view action and must never reach document operations under kind discipline");
    }
    //#endregion 🔖️Behavior
}
//#endregion 🧪️Tests
