//! 📇️ ISO 16757 play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the two surfaces
//! in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::document::NormHost;
use crate::editor::iso16757::commands::{evaluate, selected_check, set_snapshot};
use crate::editor::iso16757::modes::edit as edit_mode;
use crate::editor::iso16757::modes::edit::windows::{inputs, results};
use crate::editor::iso16757::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::op::Iso16757Mutation;
use crate::Iso16757Snapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::{AppIo, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, LocalizedLabel, Media, MediaError, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation};
use semio_framework_plugin::{NoPresence, NoPresenceMutation};
// 🚧️ SDK GAP: `Dialect` is not in `semio_framework_plugin`'s curated crate-root re-export list
// (only `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit` are,
// per ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET W0-F gap 1) — only reachable through `app`.
use semio_framework_plugin::app::Dialect;
use store::EngineHandles;

//#region 🔖️Constants
/// 🏷️ This standard's display name — the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "ISO 16757";
/// 🆔️ The playground/registry variant key — every body key, window id and schema is derived from it.
pub const VARIANT: &str = "iso16757";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.iso16757/v1";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Iso16757PlayApp::Command` — the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_iso16757_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively — both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum Iso16757Command for Iso16757Snapshot, Iso16757Mutation, NoConfig, NoConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Iso16757PlayApp
#[derive(Default)]
pub struct Iso16757PlayApp;

impl ArtifactEditor for Iso16757PlayApp {
    type Snapshot = Iso16757Snapshot;
    type Mutation = Iso16757Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Iso16757Command;

    const DIALECT: Dialect = crate::ISO16757_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.iso16757/v1";

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::app_surface::norm_artifact_store_preparation::<Self>()
    }

    semio_s_artifact_norm_contract::norm_exact_store_ownership!();


    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        Iso16757BoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        crate::app_surface::build_norm_tool_job::<Self>(request)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Iso16757PlayApp>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.norm.iso16757@1/*#editor",
        document_schema: "semio.norm.iso16757/v1",
        factory: "Iso16757BoundedCommandJobFactory",
        factory_type: Iso16757BoundedCommandJobFactory,
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

    fn initial_snapshot() -> Iso16757Snapshot {
        Iso16757Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &Iso16757Command) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &Iso16757Command,
        doc: &ArtifactView<'_, Iso16757Snapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Iso16757Mutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        crate::app_surface::dispatch_norm_command::<Self>(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Iso16757Snapshot>, cfg: &ConfigView<'_, NoConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let host = NormHost::<Iso16757Family>::from_document(doc.snapshot.clone());
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

    //#region 🔖️MediaPorts
    /// 🎞️ `"report:out"`/`"document:out"` — see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"document:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, Iso16757Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<Iso16757Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// 🎞️ `"model:in"`/`"document:in"` — see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Iso16757Snapshot>) -> Result<Emit<Iso16757Mutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        let base = doc.snapshot.clone();
        crate::app_surface::import_media(port, media, move |snapshot: Iso16757Snapshot| Iso16757Mutation::from_snapshot(&base, &snapshot))
    }
    //#endregion 🔖️MediaPorts
}
//#endregion 🔖️Iso16757PlayApp

//#region 🧵️RetainedCommands
crate::norm_owned_tool_job_factory!(Iso16757BoundedCommandJobFactory, Iso16757PlayApp);

impl crate::app_surface::NormRetainedEditor for Iso16757PlayApp {
    type ResultsWindowConfigOwner = results::ResultsWindowConfigOwner;

    fn selected_check_window_mutation(command: &Iso16757Command) -> Option<crate::results_window_config::NormResultsWindowConfigMutation> {
        match command {
            Iso16757Command::SetSelectedCheckIndex(payload) => Some(selected_check::window_mutation(payload)),
            _ => None,
        }
    }

    fn dispatch_retained(command: &Iso16757Command, doc: &ArtifactView<'_, Iso16757Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<Iso16757Mutation, NoConfigMutation, NoDraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct Iso16757Family;

impl crate::document::NormFamily for Iso16757Family {
    type Document = Iso16757Snapshot;
    type Mutation = Iso16757Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::Iso16757
    }

    fn evaluate(document: &Iso16757Snapshot) -> crate::document::CheckReport {
        crate::standards::v1::subsets::any::schema::inferences::evaluate(document)
    }
}

pub type Host = NormHost<Iso16757Family>;
//#endregion 🧩️ComplianceFamily

//#region 🔖️Manifest
pub fn create_iso16757_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::ISO16757_DIALECT)
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
            .view_action("setSelectedCheckIndex", LocalizedLabel::native("Set Selected Check", "Ausgewählte Prüfung setzen"))
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
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

