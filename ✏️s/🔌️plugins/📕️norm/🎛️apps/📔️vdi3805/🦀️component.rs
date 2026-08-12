//! ð°ï¸ VDI 3805 play app â the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `ð®ï¸commands/*`, the two surfaces
//! in `ð­ï¸modes/âï¸edit/ðªï¸windows/*`, panel trees in `ðï¸panels/*`, compliance compute in
//! `crate::apps::vdi3805`, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::apps::vdi3805::commands::{evaluate, selected_check, set_snapshot};
use crate::apps::vdi3805::modes::edit as edit_mode;
use crate::apps::vdi3805::modes::edit::windows::{inputs, results};
use crate::apps::vdi3805::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::vdi3805::Vdi3805Family;
use crate::artifacts::vdi3805::op::Vdi3805Mutation;
use crate::artifacts::vdi3805::Vdi3805Snapshot;
use crate::config::{NormConfig, NormConfigMutation, NormHost};
use crate::presence::{NormPresence, NormPresenceMutation};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView, App, AppIo, ConfigView, ArtifactApp, ArtifactView, Emit, Fault, LocalizedLabel, Media, MediaError, UiNode};
use store::EngineHandles;

//#region ðï¸Constants
pub const APP_ID: &str = "norm-vdi-3805-play";
/// ð·ï¸ This standard's display name â the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "VDI 3805";
/// ðï¸ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "vdi3805";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.vdi3805/v1";
pub const CONFIG_SCHEMA: &str = "config.norm.vdi3805";
//#endregion ðï¸Constants

//#region ðï¸Commands
semio_framework_plugin::app_commands! {
    /// ð¯ï¸ `Vdi3805PlayApp::Command` â the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_vdi3805_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively â both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum Vdi3805Command for Vdi3805Snapshot, Vdi3805Mutation, NormConfig, NormConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
    }
}
//#endregion ðï¸Commands

//#region ðï¸Vdi3805PlayApp
#[derive(Default)]
pub struct Vdi3805PlayApp;

impl ArtifactApp for Vdi3805PlayApp {
    type Snapshot = Vdi3805Snapshot;
    type Mutation = Vdi3805Mutation;
    type Config = NormConfig;
    type ConfigMutation = NormConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NormPresence;
    type PresenceMutation = NormPresenceMutation;

    type Command = Vdi3805Command;

    const APP_ID: &'static str = "norm-vdi-3805-play";
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.vdi3805/v1";

    fn config_schema() -> &'static str {
        CONFIG_SCHEMA
    }

    /// 📎️ All fifteen norm apps share NormConfig (see crate::config::schema doc) — one
    /// AppSchemaDescriptor for all fifteen, registered idempotently by whichever app binds first.
    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Vdi3805Snapshot {
        Vdi3805Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &Vdi3805Command) -> &'static str {
        command.command_id()
    }

    fn handle(command: &Vdi3805Command, doc: &ArtifactView<'_, Vdi3805Snapshot>, cfg: &ConfigView<'_, NormConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Vdi3805Mutation, NormConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Vdi3805Snapshot>, cfg: &ConfigView<'_, NormConfig>) -> UiNode {
        let host = NormHost::<Vdi3805Family>::from_document(doc.snapshot.clone());
        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot),
            results::BODY_RESULTS => results::render(&host),
            document_panel::BODY_DOCUMENT => document_panel::render(&host),
            catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, cfg.snapshot.selected_check_index),
            _ => crate::app_surface::render_unknown_body(body_key),
        }
    }

    //#region ðï¸MediaPorts
    /// ðï¸ `"report:out"`/`"document:out"` â see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"document:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, Vdi3805Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<Vdi3805Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// ðï¸ `"model:in"`/`"document:in"` â see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Vdi3805Snapshot>) -> Result<Emit<Vdi3805Mutation, NormConfigMutation, Self::DraftMutation>, MediaError> {
        let base = doc.snapshot.clone();
        crate::app_surface::import_media(port, media, move |snapshot: Vdi3805Snapshot| Vdi3805Mutation::from_snapshot(&base, &snapshot))
    }
    //#endregion ðï¸MediaPorts
}
//#endregion ðï¸Vdi3805PlayApp

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct Vdi3805Family;

impl crate::document::NormFamily for Vdi3805Family {
    type Document = Vdi3805Snapshot;
    type Mutation = Vdi3805Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::Vdi3805
    }

    fn evaluate(document: &Vdi3805Snapshot) -> crate::document::CheckReport {
        crate::artifacts::vdi3805::standards::v1::subsets::any::schema::inferences::evaluate(document)
    }
}

pub type Host = crate::document::NormHost<Vdi3805Family>;
//#endregion 🧩️ComplianceFamily

//#region ðï¸Manifest
pub fn create_vdi3805_app() -> App {
    App::from_builder(
        App::builder(APP_ID, LocalizedLabel::data(LABEL))
            .document(["semio", "norm", VARIANT])
            .artifact_kind(crate::artifacts::vdi3805::artifact_kind())
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
            .view_action("setSelectedCheckIndex", LocalizedLabel::native("Set Selected Check", "AusgewÃ¤hlte PrÃ¼fung setzen"))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo"),
    )
    .example("default", LocalizedLabel::native("Default", "Standard"), serde_json::to_string(&Vdi3805Snapshot::default()).expect("default document serializes"), "file")
    .workflow(VARIANT, LABEL, "compliance")
}
//#endregion ðï¸Manifest

//#region ð§ªï¸Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type NormApp = VcsArtifactApp<Vdi3805PlayApp>;

    pub fn new_app() -> NormApp {
        sdk_new_app::<Vdi3805PlayApp>()
    }

    /// ð§¬ï¸ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> NormApp {
        new_app_with_registry::<Vdi3805PlayApp>(create_vdi3805_app)
    }

    pub fn dispatch(app: &mut NormApp, command: Vdi3805Command) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut NormApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion ð§ªï¸Testkit

//#region ð§ªï¸Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;

    //#region ðï¸CommandSurface
    /// ð¯ï¸ One value per `Vdi3805Command` row â the whole-command-surface laws below iterate it, so a new row
    /// that is not listed here fails `command_ids_cover_every_row`.
    fn every_command() -> Vec<Vdi3805Command> {
        vec![
            Vdi3805Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Vdi3805Snapshot::default() }),
            Vdi3805Command::Evaluate(evaluate::Evaluate {}),
            Vdi3805Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }),
        ]
    }

    #[test]
    fn command_ids_cover_every_row_and_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Vdi3805Command::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids, vec!["setSnapshot", "evaluate", "setSelectedCheckIndex"]);
    }

    /// ð§·ï¸ The permanent wire guard: every row round-trips textâbinary and prints under its own declared
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

    /// ð§·ï¸ Pins the exact pre-migration bytes for the rows whose shape the `app_commands!` decomposition
    /// could have silently rewritten â the fieldless `Evaluate` (was a unit variant) and both `Option`
    /// cases of `SetSelectedCheckIndex`. Hex copied verbatim from the ticket's
    /// `ð§ªï¸wire-baseline-before.txt`; these bytes are identical for all fifteen norm apps because none
    /// of the three payload shapes involves the per-standard `Vdi3805Snapshot`.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Vdi3805Command| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&Vdi3805Command::Evaluate(evaluate::Evaluate {})), "01010000");
        assert_eq!(hex(&Vdi3805Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })), "01020001000402");
        assert_eq!(hex(&Vdi3805Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: None })), "01020000");
    }
    //#endregion ðï¸CommandSurface

    //#region ðï¸Manifest
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_vdi3805_app().definition;
        assert_eq!(definition.id, APP_ID);
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 2);
        for body_key in [document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == crate::app_surface::artifact_kind_id(VARIANT)));
    }

    /// ðï¸ Port recipe: every norm app declares `model:in`/`report:out` alongside the implicit document
    /// ports, and `report:out` is pinned to this family's already-declared artifact kind.
    #[test]
    fn declares_model_in_and_report_out_ports() {
        let ports = create_vdi3805_app().definition.io.ports;
        assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
        let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
        assert_eq!(report_out.kind_id.as_deref(), Some(crate::app_surface::artifact_kind_id(VARIANT).as_str()));
    }

    #[test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, "norm.vdi3805.play.nope").contains("Unknown body"));
    }

    #[test]
    fn every_declared_body_key_renders() {
        let mut app = testkit::new_app();
        for body_key in [inputs::BODY_INPUTS, results::BODY_RESULTS, document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
            assert!(!testkit::render(&mut app, body_key).contains("Unknown body"), "{body_key} must render its own node");
        }
    }
    //#endregion ðï¸Manifest

    //#region ðï¸Behavior
    #[test]
    fn set_snapshot_commits_a_host_backed_report() {
        let mut app = testkit::new_app();
        testkit::dispatch(&mut app, Vdi3805Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Vdi3805Snapshot::default() }));
        let host = NormHost::<Vdi3805Family>::from_document(app.snapshot().expect("projection"));
        assert!(!host.report().checks.is_empty());
    }

    /// ð§©ï¸ The `NormFamily` binding lives here now (it was in the constitutional `op` crate) it
    /// names `evaluate`, so it belongs beside the compute it binds.
    #[test]
    fn norm_family_id() {
        assert_eq!(<Vdi3805Family as crate::document::NormFamily>::family_id(), crate::document::NormFamilyId::Vdi3805);
        assert_eq!(crate::document::NormFamilyId::Vdi3805.label(), "VDI 3805");
    }

    #[test]
    fn norm_host_recomputes() {
        let mut host = Host::from_document(Vdi3805Snapshot::default());
        assert!(!host.report().checks.is_empty());
        host.replace_document(Vdi3805Snapshot::default());
        assert!(host.report().all_pass());
    }

    #[test]
    fn evaluate_recommits_the_current_projection_without_changing_it() {
        let mut app = testkit::new_app();
        let before = app.snapshot().expect("projection");
        testkit::dispatch(&mut app, Vdi3805Command::Evaluate(evaluate::Evaluate {}));
        assert_eq!(before, app.snapshot().expect("projection"));
    }

    /// ð§®ï¸ `setSelectedCheckIndex` is config-only â it must dispatch cleanly and never touch the document.
    #[test]
    fn selected_check_index_is_a_config_only_edit() {
        let mut app = testkit::new_app();
        let before = app.snapshot().expect("projection");
        let result = testkit::dispatch(&mut app, Vdi3805Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }));
        assert!(result.mutations.is_empty(), "a config-only command must emit no document operations");
        assert_eq!(before, app.snapshot().expect("projection"), "a config-only command must never mutate the document");
    }

    /// ð§¬ï¸ Kind-discipline wrapper: the real registry enforces that View actions never emit document
    /// operations.
    #[test]
    fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Vdi3805Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) }));
        assert!(result.mutations.is_empty());
    }

    #[test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::new_app();
        testkit::dispatch(&mut app, Vdi3805Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Vdi3805Snapshot::default() }));
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("projection"), Vdi3805Snapshot::default());
    }

    /// ðï¸ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
    #[test]
    fn report_out_exports_the_computed_check_report() {
        let mut app = testkit::new_app();
        let media = PluginApp::export_media(&mut app, "report:out").expect("export report:out");
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
        assert_eq!(schema, crate::app_surface::artifact_kind_id(VARIANT));
        let report: crate::document::CheckReport = serde_json::from_str(&json).expect("report json parses");
        assert!(!report.checks.is_empty());
    }
    //#endregion ðï¸Behavior
}
//#endregion ð§ªï¸Tests
