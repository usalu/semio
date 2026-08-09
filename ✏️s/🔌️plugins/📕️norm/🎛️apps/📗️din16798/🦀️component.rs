//! 🌬️ DIN EN 16798 play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the two surfaces
//! in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, compliance compute in
//! `crate::artifacts::din16798::engine`, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::apps::din16798::commands::{evaluate, selected_check, set_snapshot};
use crate::apps::din16798::modes::edit as edit_mode;
use crate::apps::din16798::modes::edit::windows::{inputs, results};
use crate::apps::din16798::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::artifacts::din16798::engine::DinEn16798Family;
use crate::artifacts::din16798::op::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use crate::config::{NormConfig, NormConfigMutation, NormHost};
use crate::presence::{NormPresence, NormPresenceMutation};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, App, AppIo, ConfigView, DocumentApp, DocumentView, Emit, Fault, LocalizedLabel, Media, MediaError, UiNode};
use store::EngineHandles;

//#region 🔖️Constants
pub const APP_ID: &str = "norm-din-en-16798-play";
/// 🏷️ This standard's display name — the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "DIN EN 16798";
/// 🆔️ The playground/registry variant key — every body key, window id and schema is derived from it.
pub const VARIANT: &str = "din16798";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.din16798/v1";
pub const CONFIG_SCHEMA: &str = "config.norm.din16798";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Din16798PlayApp::Command` — the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_din16798_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively — both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum Din16798Command for Din16798Snapshot, Din16798Mutation, NormConfig, NormConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Din16798PlayApp
#[derive(Default)]
pub struct Din16798PlayApp;

impl DocumentApp for Din16798PlayApp {
    type Snapshot = Din16798Snapshot;
    type Mutation = Din16798Mutation;
    type Config = NormConfig;
    type ConfigMutation = NormConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NormPresence;
    type PresenceMutation = NormPresenceMutation;

    type Command = Din16798Command;

    const APP_ID: &'static str = "norm-din-en-16798-play";
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.din16798/v1";

    fn config_schema() -> &'static str {
        CONFIG_SCHEMA
    }

    fn initial_snapshot() -> Din16798Snapshot {
        Din16798Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &Din16798Command) -> &'static str {
        command.command_id()
    }

    fn handle(command: &Din16798Command, doc: &DocumentView<'_, Din16798Snapshot>, cfg: &ConfigView<'_, NormConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Din16798Mutation, NormConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Din16798Snapshot>, cfg: &ConfigView<'_, NormConfig>) -> UiNode {
        let host = NormHost::<DinEn16798Family>::from_document(doc.snapshot.clone());
        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot),
            results::BODY_RESULTS => results::render(&host),
            document_panel::BODY_DOCUMENT => document_panel::render(&host),
            catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, cfg.snapshot.selected_check_index),
            _ => crate::app_surface::render_unknown_body(body_key),
        }
    }

    //#region 🔖️MediaPorts
    /// 🎞️ `"report:out"`/`"document:out"` — see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"document:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &DocumentView<'_, Din16798Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<DinEn16798Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// 🎞️ `"model:in"`/`"document:in"` — see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, _doc: &DocumentView<'_, Din16798Snapshot>) -> Result<Emit<Din16798Mutation, NormConfigMutation, Self::DraftMutation>, MediaError> {
        crate::app_surface::import_media(port, media, |snapshot| Din16798Mutation::SetSnapshot { snapshot })
    }
    //#endregion 🔖️MediaPorts
}
//#endregion 🔖️Din16798PlayApp

//#region 🔖️Manifest
pub fn create_din16798_app() -> App {
    App::from_builder(
        App::builder(APP_ID, LocalizedLabel::data(LABEL))
            .document(["semio", "norm", VARIANT])
            .artifact_kind(crate::artifacts::din16798::artifact_kind())
            .io(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
            .mode_def(edit_mode::definition())
            .default_mode_id(crate::app_surface::MODE_EDIT)
            .window_kind_def(inputs::definition())
            .window_kind_def(results::definition())
            .default_layout(edit_mode::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("setSnapshot", LocalizedLabel::native("Set Snapshot", "Dokument setzen"))
            .view_action("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"))
            .view_action("setSelectedCheckIndex", LocalizedLabel::native("Set Selected Check", "Ausgewählte Prüfung setzen"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("default", LocalizedLabel::native("Default", "Standard"), serde_json::to_string(&Din16798Snapshot::default()).expect("default document serializes"), "file")
    .workflow(VARIANT, LABEL, "compliance")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type NormApp = VcsDocumentApp<Din16798PlayApp>;

    pub fn new_app() -> NormApp {
        sdk_new_app::<Din16798PlayApp>()
    }

    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> NormApp {
        new_app_with_registry::<Din16798PlayApp>(create_din16798_app)
    }

    pub fn dispatch(app: &mut NormApp, command: Din16798Command) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut NormApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    //#region 🔖️CommandSurface
    /// 🎯️ One value per `Din16798Command` row — the whole-command-surface laws below iterate it, so a new row
    /// that is not listed here fails `command_ids_cover_every_row`.
    fn every_command() -> Vec<Din16798Command> {
        vec![
            Din16798Command::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Din16798Snapshot::default() }),
            Din16798Command::Evaluate(evaluate::Evaluate {}),
            Din16798Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }),
        ]
    }

    #[test]
    fn command_ids_cover_every_row_and_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Din16798Command::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids, vec!["setSnapshot", "evaluate", "setSelectedCheckIndex"]);
    }

    /// 🧷️ The permanent wire guard: every row round-trips text↔binary and prints under its own declared
    /// kebab wire keyword (which is deliberately NOT the camelCase `command_id`).
    #[test]
    fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
        let keywords = ["set-snapshot", "evaluate", "selected-check"];
        for (command, keyword) in every_command().into_iter().zip(keywords) {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
            let printed = protocol::OpText::print_op(&command);
            assert!(printed.starts_with(keyword), "row {} printed {printed:?}, expected keyword {keyword}", command.command_id());
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for the rows whose shape the `app_commands!` decomposition
    /// could have silently rewritten — the fieldless `Evaluate` (was a unit variant) and both `Option`
    /// cases of `SetSelectedCheckIndex`. Hex copied verbatim from the ticket's
    /// `🧪️wire-baseline-before.txt`; these bytes are identical for all fifteen norm apps because none
    /// of the three payload shapes involves the per-standard `Din16798Snapshot`.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Din16798Command| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&Din16798Command::Evaluate(evaluate::Evaluate {})), "01010000");
        assert_eq!(hex(&Din16798Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })), "01020001000402");
        assert_eq!(hex(&Din16798Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: None })), "01020000");
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_din16798_app().definition;
        assert_eq!(definition.id, APP_ID);
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 2);
        for body_key in [document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == crate::app_surface::artifact_kind_id(VARIANT)));
    }

    /// 🔌️ Port recipe: every norm app declares `model:in`/`report:out` alongside the implicit document
    /// ports, and `report:out` is pinned to this family's already-declared artifact kind.
    #[test]
    fn declares_model_in_and_report_out_ports() {
        let ports = create_din16798_app().definition.io.ports;
        assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
        let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
        assert_eq!(report_out.kind_id.as_deref(), Some(crate::app_surface::artifact_kind_id(VARIANT).as_str()));
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, "norm.din16798.play.nope").contains("Unknown body"));
    }

    #[test]
    fn every_declared_body_key_renders() {
        let mut app = testkit::new_app();
        for body_key in [inputs::BODY_INPUTS, results::BODY_RESULTS, document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
            assert!(!testkit::render(&mut app, body_key).contains("Unknown body"), "{body_key} must render its own node");
        }
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Behavior
    #[test]
    fn set_snapshot_commits_a_host_backed_report() {
        let mut app = testkit::new_app();
        testkit::dispatch(&mut app, Din16798Command::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Din16798Snapshot::default() }));
        let host = NormHost::<DinEn16798Family>::from_document(app.snapshot().expect("projection"));
        assert!(!host.report().checks.is_empty());
    }

    #[test]
    fn evaluate_recommits_the_current_projection_without_changing_it() {
        let mut app = testkit::new_app();
        let before = app.snapshot().expect("projection");
        testkit::dispatch(&mut app, Din16798Command::Evaluate(evaluate::Evaluate {}));
        assert_eq!(before, app.snapshot().expect("projection"));
    }

    /// 🧮️ `setSelectedCheckIndex` is config-only — it must dispatch cleanly and never touch the document.
    #[test]
    fn selected_check_index_is_a_config_only_edit() {
        let mut app = testkit::new_app();
        let before = app.snapshot().expect("projection");
        let result = testkit::dispatch(&mut app, Din16798Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }));
        assert!(result.mutations.is_empty(), "a config-only command must emit no document operations");
        assert_eq!(before, app.snapshot().expect("projection"), "a config-only command must never mutate the document");
    }

    /// 🧬️ Kind-discipline wrapper: the real registry enforces that View actions never emit document
    /// operations.
    #[test]
    fn view_actions_never_emit_document_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Din16798Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) }));
        assert!(result.mutations.is_empty());
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app();
        testkit::dispatch(&mut app, Din16798Command::SetSnapshot(set_snapshot::SetSnapshot { snapshot: Din16798Snapshot::default() }));
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("projection"), Din16798Snapshot::default());
    }

    /// 🎞️ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
    #[test]
    fn report_out_exports_the_computed_check_report() {
        let mut app = testkit::new_app();
        let media = PluginApp::export_media(&mut app, "report:out").expect("export report:out");
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
        assert_eq!(schema, crate::app_surface::artifact_kind_id(VARIANT));
        let report: crate::document::CheckReport = serde_json::from_str(&json).expect("report json parses");
        assert!(!report.checks.is_empty());
    }
    //#endregion 🔖️Behavior
}
//#endregion 🧪️Tests
