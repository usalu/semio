//! ð¢ï¸ DIN V 18599 play app â the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `ð®ï¸commands/*`, the two surfaces
//! in `ð­ï¸modes/âï¸edit/ðªï¸windows/*`, panel trees in `ðï¸panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::document::NormHost;
use crate::editor::din18599::commands::{evaluate, selected_check, set_snapshot};
use crate::editor::din18599::modes::edit as edit_mode;
use crate::editor::din18599::modes::edit::windows::{inputs, results};
use crate::editor::din18599::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::op::Din18599Mutation;
use crate::Din18599Snapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::{AppIo, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, LocalizedLabel, Media, MediaError, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation};
use semio_framework_plugin::{NoPresence, NoPresenceMutation};
// 🚧️ SDK GAP: `Dialect` is not in `semio_framework_plugin`'s curated crate-root re-export list
// (only `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit` are,
// per ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET W0-F gap 1) — only reachable through `app`.
use semio_framework_plugin::app::Dialect;
use store::EngineHandles;

//#region ðï¸Constants
/// ð·ï¸ This standard's display name â the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "DIN V 18599";
/// ðï¸ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "din18599";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.din18599/v1";
//#endregion ðï¸Constants

//#region ðï¸Commands
semio_framework_plugin::app_commands! {
    /// ð¯ï¸ `Din18599PlayApp::Command` â the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_din18599_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively â both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum Din18599Command for Din18599Snapshot, Din18599Mutation, NoConfig, NoConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
    }
}
//#endregion ðï¸Commands

//#region ðï¸Din18599PlayApp
#[derive(Default)]
pub struct Din18599PlayApp;

impl ArtifactEditor for Din18599PlayApp {
    type Snapshot = Din18599Snapshot;
    type Mutation = Din18599Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Din18599Command;

    const DIALECT: Dialect = crate::DIN18599_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.din18599/v1";

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::app_surface::norm_artifact_store_preparation::<Self>()
    }

    semio_s_artifact_norm_contract::norm_exact_store_ownership!();


    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        Din18599BoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        crate::app_surface::build_norm_tool_job::<Self>(request)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Din18599PlayApp>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.norm.din18599@1/*#editor",
        document_schema: "semio.norm.din18599/v1",
        factory: "Din18599BoundedCommandJobFactory",
        factory_type: Din18599BoundedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        tools: ["setSnapshot", "evaluate", "setSelectedCheckIndex"]
    }


    /// 📎️ Norm application config and presence facets are both empty; Results windows register their own config.
    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::app_surface::app_schema_descriptor())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<results::ResultsWindowConfigOwner>()
    }

    fn initial_snapshot() -> Din18599Snapshot {
        Din18599Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &Din18599Command) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &Din18599Command,
        doc: &ArtifactView<'_, Din18599Snapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Din18599Mutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        crate::app_surface::dispatch_norm_command::<Self>(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Din18599Snapshot>, cfg: &ConfigView<'_, NoConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let host = NormHost::<DinV18599Family>::from_document(doc.snapshot.clone());
        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot),
            results::BODY_RESULTS => results::render(&host),
            document_panel::BODY_DOCUMENT => document_panel::render(&host),
            catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, crate::results_window_config::current::<results::ResultsWindowConfigOwner>(cfg).selected_check_index),
            _ => crate::app_surface::render_unknown_body(body_key),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    //#region ðï¸MediaPorts
    /// ðï¸ `"report:out"`/`"document:out"` â see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"document:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, Din18599Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<DinV18599Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// ðï¸ `"model:in"`/`"document:in"` â see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, Din18599Snapshot>) -> Result<Emit<Din18599Mutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        crate::app_surface::import_media(port, media, |snapshot: Din18599Snapshot| Din18599Mutation::from_snapshot(&snapshot))
    }
    //#endregion ðï¸MediaPorts
}
//#endregion ðï¸Din18599PlayApp

//#region 🧵️RetainedCommands
crate::norm_owned_tool_job_factory!(Din18599BoundedCommandJobFactory, Din18599PlayApp);

impl crate::app_surface::NormRetainedEditor for Din18599PlayApp {
    type ResultsWindowConfigOwner = results::ResultsWindowConfigOwner;

    fn selected_check_window_mutation(command: &Din18599Command) -> Option<crate::results_window_config::NormResultsWindowConfigMutation> {
        match command {
            Din18599Command::SetSelectedCheckIndex(payload) => Some(selected_check::window_mutation(payload)),
            _ => None,
        }
    }

    fn dispatch_retained(command: &Din18599Command, doc: &ArtifactView<'_, Din18599Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Din18599Mutation, NoConfigMutation, NoDraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct DinV18599Family;

impl crate::document::NormFamily for DinV18599Family {
    type Document = Din18599Snapshot;
    type Mutation = Din18599Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::DinV18599
    }

    fn evaluate(document: &Din18599Snapshot) -> crate::document::CheckReport {
        crate::standards::v1::subsets::any::schema::inferences::evaluate(document)
    }
}

pub type Host = NormHost<DinV18599Family>;
//#endregion 🧩️ComplianceFamily

//#region ðï¸Manifest
pub fn create_din18599_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::DIN18599_DIALECT)
            .document(["semio", "norm", VARIANT])
            .artifact_kind(crate::artifact_kind())
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
            .action_with(semio_framework_plugin::ActionDefinition::new("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"), semio_framework_plugin::ActionKind::View, "hash"))
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

//#region ð§ªï¸UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion ð§ªï¸UnitTests

