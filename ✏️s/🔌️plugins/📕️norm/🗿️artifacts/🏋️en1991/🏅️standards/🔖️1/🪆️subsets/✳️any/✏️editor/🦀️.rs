//! ðï¸ EN 1991 play app â the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `ð®ï¸commands/*`, the two surfaces
//! in `ð­ï¸modes/âï¸edit/ðªï¸windows/*`, panel trees in `ðï¸panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::artifacts::en1991::op::En1991Mutation;
use crate::artifacts::en1991::En1991Snapshot;
use crate::config::{NormConfig, NormConfigMutation, NormHost};
use crate::editor::en1991::commands::{evaluate, selected_check, set_snapshot};
use crate::editor::en1991::modes::edit as edit_mode;
use crate::editor::en1991::modes::edit::windows::{inputs, results};
use crate::editor::en1991::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
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
pub const LABEL: &str = "EN 1991";
/// ðï¸ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "en1991";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.en1991/v1";
pub const CONFIG_SCHEMA: &str = "config.norm.en1991";
//#endregion ðï¸Constants

//#region ðï¸Commands
semio_framework_plugin::app_commands! {
    /// ð¯ï¸ `En1991PlayApp::Command` â the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_en1991_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively â both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum En1991Command for En1991Snapshot, En1991Mutation, NormConfig, NormConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
    }
}
//#endregion ðï¸Commands

//#region ðï¸En1991PlayApp
#[derive(Default)]
pub struct En1991PlayApp;


impl ArtifactEditor for En1991PlayApp {
    type Snapshot = En1991Snapshot;
    type Mutation = En1991Mutation;
    type Config = NormConfig;
    type ConfigMutation = NormConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = En1991Command;

    const DIALECT: Dialect = crate::artifacts::en1991::EN1991_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.en1991/v1";

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::app_surface::norm_artifact_store_preparation::<Self>()
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        crate::app_surface::norm_config_store_preparation::<Self>()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        En1991BoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        crate::app_surface::build_norm_tool_job::<Self>(request)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<En1991PlayApp>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.norm.en1991@1/*#editor",
        document_schema: "semio.norm.en1991/v1",
        factory: "En1991BoundedCommandJobFactory",
        factory_type: En1991BoundedCommandJobFactory,
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

    fn initial_snapshot() -> En1991Snapshot {
        En1991Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &En1991Command) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &En1991Command,
        doc: &ArtifactView<'_, En1991Snapshot>,
        cfg: &ConfigView<'_, NormConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<En1991Mutation, NormConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, En1991Snapshot>, cfg: &ConfigView<'_, NormConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let host = NormHost::<En1991Family>::from_document(doc.snapshot.clone());
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
    fn export_media(port: &str, doc: &ArtifactView<'_, En1991Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<En1991Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// ðï¸ `"model:in"`/`"document:in"` â see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, En1991Snapshot>) -> Result<Emit<En1991Mutation, NormConfigMutation, Self::DraftMutation>, MediaError> {
        crate::app_surface::import_media(port, media, |snapshot: En1991Snapshot| En1991Mutation::from_snapshot(&snapshot))
    }
    //#endregion ðï¸MediaPorts
}
//#endregion ðï¸En1991PlayApp

//#region 🧵️RetainedCommands
crate::norm_owned_tool_job_factory!(En1991BoundedCommandJobFactory, En1991PlayApp);

impl crate::app_surface::NormRetainedEditor for En1991PlayApp {
    fn dispatch_retained(command: &En1991Command, doc: &ArtifactView<'_, En1991Snapshot>, cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1991Mutation, NormConfigMutation, NoDraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct En1991Family;

impl crate::document::NormFamily for En1991Family {
    type Document = En1991Snapshot;
    type Mutation = En1991Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::En1991
    }

    fn evaluate(document: &En1991Snapshot) -> crate::document::CheckReport {
        crate::artifacts::en1991::standards::v1::subsets::any::schema::inferences::evaluate(document)
    }
}

pub type Host = NormHost<En1991Family>;
//#endregion 🧩️ComplianceFamily

//#region ðï¸Manifest
pub fn create_en1991_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::en1991::EN1991_DIALECT)
            .document(["semio", "norm", VARIANT])
            .artifact_kind(crate::artifacts::en1991::artifact_kind())
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

    /// ✏️ Adapts `create_en1991_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::new_app_with_registry` still expects (framework testkit gap,
    /// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).
    pub fn en1991_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_en1991_app(), examples: Vec::new() }
    }

    pub type NormApp = VcsArtifactApp<EditorApp<En1991PlayApp>>;


    /// ð§¬ï¸ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub fn app_with_registry() -> NormApp {
        new_app_with_registry::<EditorApp<En1991PlayApp>>(en1991_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut NormApp, command: En1991Command) -> InvocationResult {
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
        let definition = create_en1991_app();
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
    /// ð¯ï¸ One value per `En1991Command` row â the whole-command-surface laws below iterate it, so a new row
    /// that is not listed here fails `command_ids_cover_every_row`.
    fn every_command() -> Vec<En1991Command> {
        vec![
            En1991Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: En1991Snapshot::default() }),
            En1991Command::Evaluate(evaluate::Evaluate {}),
            En1991Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    fn command_ids_cover_every_row_and_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(En1991Command::command_id).collect();
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
    /// of the three payload shapes involves the per-standard `En1991Snapshot`.
    #[semio_framework_async_macros::async_test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &En1991Command| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&En1991Command::Evaluate(evaluate::Evaluate {})), "01010000");
        assert_eq!(hex(&En1991Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })), "01020001000402");
        assert_eq!(hex(&En1991Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: None })), "01020000");
    }
    //#endregion ðï¸CommandSurface

    //#region ðï¸Manifest
    #[semio_framework_async_macros::async_test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_en1991_app();
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
        let ports = create_en1991_app().io.ports;
        assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
        let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
        assert_eq!(report_out.kind_id.as_deref(), Some(crate::app_surface::artifact_kind_id(VARIANT).as_str()));
    }

    #[semio_framework_async_macros::async_test]
    fn an_unknown_body_key_falls_back_to_a_text_node() {
        let mut app = testkit::app_with_registry();
        assert!(testkit::render(&mut app, "norm.en1991.play.nope").contains("Unknown body"));
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
        testkit::dispatch(&mut app, En1991Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: En1991Snapshot::default() }));
        let host = NormHost::<En1991Family>::from_document(app.snapshot().expect("projection"));
        assert!(!host.report().checks.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    fn evaluate_recommits_the_current_projection_without_changing_it() {
        let mut app = testkit::app_with_registry();
        let before = app.snapshot().expect("projection");
        testkit::dispatch(&mut app, En1991Command::Evaluate(evaluate::Evaluate {}));
        assert_eq!(before, app.snapshot().expect("projection"));
    }

    /// ð§®ï¸ `setSelectedCheckIndex` is config-only â it must dispatch cleanly and never touch the document.
    #[semio_framework_async_macros::async_test]
    fn selected_check_index_is_a_config_only_edit() {
        let mut app = testkit::app_with_registry();
        let before = app.snapshot().expect("projection");
        let result = testkit::dispatch(&mut app, En1991Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }));
        assert!(result.mutations.is_empty(), "a config-only command must emit no document operations");
        assert_eq!(before, app.snapshot().expect("projection"), "a config-only command must never mutate the document");
    }

    /// ð§¬ï¸ Kind-discipline wrapper: the real registry enforces that View actions never emit document
    /// operations.
    #[semio_framework_async_macros::async_test]
    fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, En1991Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) }));
        assert!(result.mutations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    fn undo_redo_round_trips_through_the_wrapper() {
        let mut app = testkit::app_with_registry();
        testkit::dispatch(&mut app, En1991Command::ReplaceSnapshot(set_snapshot::ReplaceSnapshot { snapshot: En1991Snapshot::default() }));
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(app.snapshot().expect("projection"), En1991Snapshot::default());
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
