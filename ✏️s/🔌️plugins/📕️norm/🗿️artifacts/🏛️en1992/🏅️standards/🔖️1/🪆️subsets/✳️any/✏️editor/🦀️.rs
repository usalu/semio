//! ð§±ï¸ EN 1992 play app â the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `ð®ï¸commands/*`, the two surfaces
//! in `ð­ï¸modes/âï¸edit/ðªï¸windows/*`, panel trees in `ðï¸panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::config::{NormConfig, NormConfigMutation, NormHost};
use crate::editor::en1992::commands::{evaluate, selected_check, set_snapshot};
use crate::editor::en1992::modes::edit as edit_mode;
use crate::editor::en1992::modes::edit::windows::{inputs, results};
use crate::editor::en1992::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::op::En1992Mutation;
use crate::En1992Snapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::{AppIo, ArtifactEditor, ArtifactView, ConfigView, DraftView, Editor, Emit, Fault, LocalizedLabel, Media, MediaError, NoDraft, NoDraftMutation};
use semio_framework_plugin::{NoPresence, NoPresenceMutation};
// 🚧️ SDK GAP: `Dialect` is not in `semio_framework_plugin`'s curated crate-root re-export list
// (only `ArtifactEditor`/`ArtifactViewer`/`Editor`/`Viewer`/`EditorApp`/`ViewerApp`/`ViewEmit` are,
// per ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET W0-F gap 1) — only reachable through `app`.
use semio_framework_plugin::app::Dialect;
use store::EngineHandles;

//#region ðï¸Constants
/// ð·ï¸ This standard's display name â the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "EN 1992";
/// ðï¸ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "en1992";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.en1992/v1";
pub const CONFIG_SCHEMA: &str = "config.norm.en1992";
//#endregion ðï¸Constants

//#region ðï¸Commands
semio_framework_plugin::app_commands! {
    /// ð¯ï¸ `En1992PlayApp::Command` â the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_en1992_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively â both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum En1992Command for En1992Snapshot, En1992Mutation, NormConfig, NormConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
    }
}
//#endregion ðï¸Commands

//#region ðï¸En1992PlayApp
#[derive(Default)]
pub struct En1992PlayApp;

impl ArtifactEditor for En1992PlayApp {
    type Snapshot = En1992Snapshot;
    type Mutation = En1992Mutation;
    type Config = NormConfig;
    type ConfigMutation = NormConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = En1992Command;

    const DIALECT: Dialect = crate::EN1992_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.en1992/v1";

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        crate::app_surface::norm_artifact_store_preparation::<Self>()
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        crate::app_surface::norm_config_store_preparation::<Self>()
    }

    fn register_tool_job_factories(registry: &mut semio_framework_plugin::ArtifactToolFactoryRegistry<'_, semio_framework_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        En1992BoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: semio_framework_plugin::ArtifactOwnedToolJobRequest<semio_framework_plugin::EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        crate::app_surface::build_norm_tool_job::<Self>(request)
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<En1992PlayApp>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.norm.en1992@1/*#editor",
        document_schema: "semio.norm.en1992/v1",
        factory: "En1992BoundedCommandJobFactory",
        factory_type: En1992BoundedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500),
        tools: ["setSnapshot", "evaluate", "setSelectedCheckIndex"]
    }

    fn config_schema() -> &'static str {
        CONFIG_SCHEMA
    }

    /// 📎️ All fifteen norm apps share NormConfig (see crate::config::schema doc) — one
    /// AppSchemaDescriptor for all fifteen, registered idempotently by whichever app binds first.
    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> En1992Snapshot {
        En1992Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &En1992Command) -> &'static str {
        command.command_id()
    }

    fn handle(
        command: &En1992Command,
        doc: &ArtifactView<'_, En1992Snapshot>,
        cfg: &ConfigView<'_, NormConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<En1992Mutation, NormConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, En1992Snapshot>, cfg: &ConfigView<'_, NormConfig>, _view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let host = NormHost::<En1992Family>::from_document(doc.snapshot.clone());
        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot),
            results::BODY_RESULTS => results::render(&host),
            document_panel::BODY_DOCUMENT => document_panel::render(&host),
            catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(),
            inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, cfg.snapshot.selected_check_index),
            _ => crate::app_surface::render_unknown_body(body_key),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    //#region ðï¸MediaPorts
    /// ðï¸ `"report:out"`/`"document:out"` â see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"document:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, En1992Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<En1992Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// ðï¸ `"model:in"`/`"document:in"` â see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, En1992Snapshot>) -> Result<Emit<En1992Mutation, NormConfigMutation, Self::DraftMutation>, MediaError> {
        crate::app_surface::import_media(port, media, |snapshot: En1992Snapshot| En1992Mutation::from_snapshot(&snapshot))
    }
    //#endregion ðï¸MediaPorts
}
//#endregion ðï¸En1992PlayApp

//#region 🧵️RetainedCommands
crate::norm_owned_tool_job_factory!(En1992BoundedCommandJobFactory, En1992PlayApp);

impl crate::app_surface::NormRetainedEditor for En1992PlayApp {
    fn dispatch_retained(command: &En1992Command, doc: &ArtifactView<'_, En1992Snapshot>, cfg: &ConfigView<'_, NormConfig>) -> Result<Emit<En1992Mutation, NormConfigMutation, NoDraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct En1992Family;

impl crate::document::NormFamily for En1992Family {
    type Document = En1992Snapshot;
    type Mutation = En1992Mutation;

    fn family_id() -> crate::document::NormFamilyId {
        crate::document::NormFamilyId::En1992
    }

    fn evaluate(document: &En1992Snapshot) -> crate::document::CheckReport {
        crate::standards::v1::subsets::any::schema::inferences::evaluate(document)
    }
}

pub type Host = NormHost<En1992Family>;
//#endregion 🧩️ComplianceFamily

//#region ðï¸Manifest
pub fn create_en1992_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::EN1992_DIALECT)
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

//#region ð§ªï¸Testkit
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion ð§ªï¸Testkit

//#region ð§ªï¸Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion ð§ªï¸Tests
