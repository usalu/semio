//! ð¡ï¸ DIN 4108 play app â the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `ð®ï¸commands/*`, the two surfaces
//! in `ð­ï¸modes/âï¸edit/ðªï¸windows/*`, panel trees in `ðï¸panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::artifacts::din4108::op::Din4108Mutation;
use crate::artifacts::din4108::Din4108Snapshot;
use crate::config::{NormConfig, NormConfigMutation, NormHost};
use crate::editor::din4108::commands::{evaluate, selected_check, set_snapshot};
use crate::editor::din4108::modes::edit as edit_mode;
use crate::editor::din4108::modes::edit::windows::{inputs, results};
use crate::editor::din4108::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use semio_framework_plugin::{NoPresence, NoPresenceMutation};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{AppIo, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, LocalizedLabel, Media, MediaError, NoDraft, NoDraftMutation, UiNode};
use semio_framework_plugin::InteractiveJobClassification;
// 🚧️ SDK GAP: `Dialect` is not in `semio_framework_plugin`'s curated crate-root re-export list
// (only `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit` are,
// per ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET W0-F gap 1) — only reachable through `app`.
use semio_framework_plugin::app::Dialect;
use store::EngineHandles;

//#region ðï¸Constants
/// ð·ï¸ This standard's display name â the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "DIN 4108";
/// ðï¸ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "din4108";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.din4108/v1";
pub const CONFIG_SCHEMA: &str = "config.norm.din4108";
//#endregion ðï¸Constants

//#region ðï¸Commands
semio_framework_plugin::app_commands! {
    /// ð¯ï¸ `Din4108PlayApp::Command` â the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_din4108_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively â both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum Din4108Command for Din4108Snapshot, Din4108Mutation, NormConfig, NormConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
    }
}
//#endregion ðï¸Commands

//#region ðï¸Din4108PlayApp
#[derive(Default)]
pub struct Din4108PlayApp;


impl ArtifactEditor for Din4108PlayApp {
    type Snapshot = Din4108Snapshot;
    type Mutation = Din4108Mutation;
    type Config = NormConfig;
    type ConfigMutation = NormConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Din4108Command;

    const DIALECT: Dialect = crate::artifacts::din4108::DIN4108_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.din4108/v1";

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::app_surface::norm_artifact_store_preparation::<Self>()
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        crate::app_surface::norm_config_store_preparation::<Self>()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        Din4108BoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        crate::app_surface::build_norm_tool_job::<Self>(request)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Din4108PlayApp>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.norm.din4108@1/*#editor",
        document_schema: "semio.norm.din4108/v1",
        factory: "Din4108BoundedCommandJobFactory",
        factory_type: Din4108BoundedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        tools: ["setSnapshot", "evaluate", "setSelectedCheckIndex"]
    }




    fn config_schema() -> &'static str {
        CONFIG_SCHEMA
    }

    /// 📎️ All fifteen norm apps share NormConfig (see crate::config::schema doc) — one
    /// AppSchemaDescriptor for all fifteen, registered idempotently by whichever app binds first.
    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Din4108Snapshot {
        Din4108Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &Din4108Command) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &Din4108Command,
        doc: &ArtifactView<'_, Din4108Snapshot>,
        cfg: &ConfigView<'_, NormConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Din4108Mutation, NormConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Din4108Snapshot>, cfg: &ConfigView<'_, NormConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let host = NormHost::<Din4108Family>::from_document(doc.snapshot.clone());
        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot),
            results::BODY_RESULTS => results::render(&host),
            document_panel::BODY_DOCUMENT => document_panel::render(&host),
            catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, cfg.snapshot.selected_check_index),
            _ => crate::app_surface::render_unknown_body(body_key),
        }.map(semio_framework_plugin::built_to_component_tree)
    }

    //#region ðï¸MediaPorts
    /// ðï¸ `"report:out"`/`"document:out"` â see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"document:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, Din4108Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<Din4108Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// ðï¸ `"model:in"`/`"document:in"` â see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Din4108Snapshot>) -> Result<Emit<Din4108Mutation, NormConfigMutation, Self::DraftMutation>, MediaError> {
        let base = doc.snapshot.clone();
        crate::app_surface::import_media(port, media, move |snapshot: Din4108Snapshot| Din4108Mutation::from_snapshot(&base, &snapshot))
    }
    //#endregion ðï¸MediaPorts
}
//#endregion ðï¸Din4108PlayApp

//#region 🧵️RetainedCommands
crate::norm_owned_tool_job_factory!(Din4108BoundedCommandJobFactory, Din4108PlayApp);

impl crate::app_surface::NormRetainedEditor for Din4108PlayApp {
    fn dispatch_retained(command: &Din4108Command, doc: &ArtifactView<'_, Din4108Snapshot>, cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<Din4108Mutation, NormConfigMutation, NoDraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct Din4108Family;

impl crate::document::NormFamily for Din4108Family {
    type Document = Din4108Snapshot;
    type Mutation = Din4108Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::Din4108
    }

    fn evaluate(document: &Din4108Snapshot) -> crate::document::CheckReport {
        crate::artifacts::din4108::standards::v1::subsets::any::schema::inferences::evaluate(document)
    }
}

pub type Host = NormHost<Din4108Family>;
//#endregion 🧩️ComplianceFamily

//#region ðï¸Manifest
pub fn create_din4108_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::din4108::DIN4108_DIALECT)
            .document(["semio", "norm", VARIANT])
            .artifact_kind(crate::artifacts::din4108::artifact_kind())
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
            .action_interactive_job("setSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder` takes a bare `AppDefinition` — there is no
            // `.example(...)`/`.workflow(...)` on this builder (see the pilot's w2-cad-report.md "SDK
            // gaps" #4), so the old app-level example/workflow registration is dropped here, not
            // silently: the subset's own `📚️examples/🎬️demo-session` facet (real content, moved
            // verbatim below) is the modern role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion ðï¸Manifest

//#region ð§ªï¸Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ Adapts `create_din4108_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::new_app_with_registry` still expects (framework testkit gap,
    /// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).
    pub fn din4108_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_din4108_app(), examples: Vec::new() }
    }

    pub type NormApp = VcsArtifactApp<EditorApp<Din4108PlayApp>>;


    /// ð§¬ï¸ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> NormApp {
        new_app_with_registry::<EditorApp<Din4108PlayApp>>(din4108_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut NormApp, command: Din4108Command) -> InvocationResult {
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

    #[test]
    fn retained_command_dispositions_match_the_language_neutral_oracle() {
        crate::app_surface::retained_disposition_oracle::assert_fixture(VARIANT);
        let definition = create_din4108_app();
        let mut classified = 0usize;
        for window in definition.window_kinds.iter() {
            for id in crate::app_surface::NORM_RETAINED_TOOL_IDS {
                let action = window.actions.iter().find(|action| action.id == *id).unwrap_or_else(|| panic!("window {} must declare {id}", window.id));
                assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
                classified += 1;
            }
        }
        assert_eq!(classified, definition.window_kinds.len() * crate::app_surface::NORM_RETAINED_TOOL_IDS.len());
    }

    //#region ðï¸CommandSurface
    /// ð¯ï¸ One value per `Din4108Command` row â the whole-command-surface laws below iterate it, so a new row
    /// that is not listed here fails `command_ids_cover_every_row`.
    fn every_command() -> Vec<Din4108Command> {
        vec![
            Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() }),
            Din4108Command::Evaluate(evaluate::Evaluate {}),
            Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    fn command_ids_cover_every_row_and_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Din4108Command::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids, vec!["setSnapshot", "evaluate", "setSelectedCheckIndex"]);
    }

    /// ð§·ï¸ The permanent wire guard: every row round-trips textâbinary and prints under its own declared
    /// kebab wire keyword (which is deliberately NOT the camelCase `command_id`).
    #[semio_framework_async_macros::async_test]
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
    /// of the three payload shapes involves the per-standard `Din4108Snapshot`.
    #[semio_framework_async_macros::async_test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Din4108Command| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&Din4108Command::Evaluate(evaluate::Evaluate {})), "01010000");
        assert_eq!(hex(&Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })), "01020001000402");
        assert_eq!(hex(&Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: None })), "01020000");
    }
    //#endregion ðï¸CommandSurface

    //#region ðï¸Manifest
    #[semio_framework_async_macros::async_test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_din4108_app();
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 2);
        for body_key in [document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == crate::app_surface::artifact_kind_id(VARIANT)));
    }

    /// ðï¸ Port recipe: every norm app declares `model:in`/`report:out` alongside the implicit document
    /// ports, and `report:out` is pinned to this family's already-declared artifact kind.
    #[semio_framework_async_macros::async_test]
    fn declares_model_in_and_report_out_ports() {
        let ports = create_din4108_app().io.ports;
        assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
        let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
        assert_eq!(report_out.kind_id.as_deref(), Some(crate::app_surface::artifact_kind_id(VARIANT).as_str()));
    }

    #[semio_framework_async_macros::async_test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = testkit::app_with_registry();
        assert!(testkit::render(&mut app, "norm.din4108.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    fn every_declared_body_key_renders() {
        let mut app = testkit::app_with_registry();
        for body_key in [inputs::BODY_INPUTS, results::BODY_RESULTS, document_panel::BODY_DOCUMENT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
            assert!(!testkit::render(&mut app, body_key).contains("Unknown body"), "{body_key} must render its own node");
        }
    }
    //#endregion ðï¸Manifest

    //#region ðï¸Behavior
    #[semio_framework_async_macros::async_test]
    fn set_snapshot_commits_a_host_backed_report() {
        let mut app = testkit::app_with_registry();
        testkit::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() }));
        let host = NormHost::<Din4108Family>::from_document(app.snapshot().expect("projection"));
        assert!(!host.report().checks.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    fn host_updates_report_after_document_replace() {
        let mut host = Host::default();
        assert!(host.report().all_pass());
        let mut document = Din4108Snapshot::default();
        document.layers.clear();
        host.replace_document(document);
        assert!(!host.report().all_pass());
    }

    #[semio_framework_async_macros::async_test]
    fn evaluate_recommits_the_current_projection_without_changing_it() {
        let mut app = testkit::app_with_registry();
        let before = app.snapshot().expect("projection");
        testkit::dispatch(&mut app, Din4108Command::Evaluate(evaluate::Evaluate {}));
        assert_eq!(before, app.snapshot().expect("projection"));
    }

    /// 🧵️ The migrated route end to end: `dispatch_typed` passes the UI-dispatch classification gate,
    /// `build_norm_tool_job` hands the command to the shared owned factory, and repeated
    /// `maintenance_step` turns publish every `from_snapshot` field mutation through the exact
    /// one-item Artifact preparation authority — proving both the wiring and the LIFO drain order
    /// `norm_retained_reduce` compensates for. `app_with_registry` + a bound instance id is mandatory:
    /// a registry-less wrapper fails closed with `interactive-job.catalog-authority` once proofs exist.
    #[test]
    fn set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document() {
        let mut app = testkit::app_with_registry();
        semio_framework::io::resolve_ready(app.bind_instance_id(1));
        let mut target = Din4108Snapshot::default();
        target.layers.clear();
        assert_ne!(target, app.snapshot().expect("projection"));
        testkit::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: target.clone() }));
        let mut ticks = 0usize;
        while ticks < 5_000 && app.snapshot().expect("projection") != target {
            app.maintenance_step(1_048_576, 1_048_576).expect("maintenance step drives the pending typed operation forward");
            ticks += 1;
        }
        assert_eq!(app.snapshot().expect("projection"), target, "setSnapshot did not publish the payload document after {ticks} maintenance turns");
    }

    /// 🧵️ Three-way drift guard between the retained id list, the publication contracts and the proof
    /// catalog every norm editor forwards.
    #[test]
    fn the_proof_catalog_covers_exactly_the_shared_retained_tool_ids() {
        use semio_framework_plugin::ArtifactEditor;
        assert_eq!(crate::app_surface::NORM_RETAINED_TOOL_IDS.len(), crate::app_surface::NORM_PUBLICATION_CONTRACTS.len());
        assert_eq!(Din4108PlayApp::bounded_first_step_tool_proofs().len(), crate::app_surface::NORM_RETAINED_TOOL_IDS.len());
        assert_eq!(crate::app_surface::NORM_PUBLICATION_CONTRACTS.iter().map(|contract| contract.tool_id).collect::<Vec<_>>(), crate::app_surface::NORM_RETAINED_TOOL_IDS.to_vec());
    }

    /// ð§®ï¸ `setSelectedCheckIndex` is config-only â it must dispatch cleanly and never touch the document.
    #[semio_framework_async_macros::async_test]
    fn selected_check_index_is_a_config_only_edit() {
        let mut app = testkit::app_with_registry();
        let before = app.snapshot().expect("projection");
        let result = testkit::dispatch(&mut app, Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }));
        assert!(result.mutations.is_empty(), "a config-only command must emit no document operations");
        assert_eq!(before, app.snapshot().expect("projection"), "a config-only command must never mutate the document");
    }

    /// ð§¬ï¸ Kind-discipline wrapper: the real registry enforces that View actions never emit document
    /// operations.
    #[semio_framework_async_macros::async_test]
    fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Din4108Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) }));
        assert!(result.mutations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::app_with_registry();
        testkit::dispatch(&mut app, Din4108Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: Din4108Snapshot::default() }));
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("projection"), Din4108Snapshot::default());
    }

    /// ðï¸ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
    #[semio_framework_async_macros::async_test]
    fn report_out_exports_the_computed_check_report() {
        let mut app = testkit::app_with_registry();
        let media = semio_framework_plugin::resolve_ready(PluginApp::export_media(&mut app, "report:out")).expect("export report:out");
        let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
        assert_eq!(schema, crate::app_surface::artifact_kind_id(VARIANT));
        let report: crate::document::CheckReport = serde_json::from_str(&json).expect("report json parses");
        assert!(!report.checks.is_empty());
    }
    //#endregion ðï¸Behavior
}
//#endregion ð§ªï¸Tests
