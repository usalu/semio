//! ð§±ï¸ EN 1992 play app â the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `ð®ï¸commands/*`, the two surfaces
//! in `ð­ï¸modes/âï¸edit/ðªï¸windows/*`, panel trees in `ðï¸panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::document::NormHost;
use crate::editor::en1992::commands::{apply_remedy, evaluate, insert_item, remove_item, selected_check, set_active_example, set_field, set_snapshot};
use crate::editor::en1992::modes::edit as edit_mode;
use crate::editor::en1992::modes::edit::windows::{inputs, results};
use crate::editor::en1992::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::op::En1992Mutation;
use crate::En1992Snapshot;
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
pub const LABEL: &str = "EN 1992";
/// ðï¸ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "en1992";
/// 🆔️ Retained-command / UI action controller id.
pub const CONTROLLER_ID: &str = "s.norm.en1992@1/*#editor";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.en1992/v1";
//#endregion ðï¸Constants

//#region ðï¸Commands
semio_framework_plugin::app_commands! {
    /// ð¯ï¸ `En1992PlayApp::Command` â the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_en1992_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively â both copied verbatim off the
    /// pre-migration enum, never derived from one another.
    pub enum En1992Command for En1992Snapshot, En1992Mutation, NoConfig, NoConfigMutation {
        "setSnapshot" as "set-snapshot" => set_snapshot::ReplaceSnapshot,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "setField" as "set-field" => set_field::SetField,
        "insertItem" as "insert-item" => insert_item::InsertItem,
        "removeItem" as "remove-item" => remove_item::RemoveItem,
        "applyRemedy" as "apply-remedy" => apply_remedy::ApplyRemedy,
    }
}
//#endregion ðï¸Commands

//#region ðï¸En1992PlayApp
#[derive(Default)]
pub struct En1992PlayApp;

impl ArtifactEditor for En1992PlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::compliant_office_frame::source(), crate::failing_under_reinforced::source(), crate::compliant_prestressed_beam::source(), crate::failing_prestressed_beam::source(), crate::liquid_retaining_fem_anchor::source()]
    }
    type Snapshot = En1992Snapshot;
    type Mutation = En1992Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
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

    semio_s_artifact_norm_contract::norm_exact_store_ownership!();


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
        artifact_schema: "semio.norm.en1992/v1",
        factory: "En1992BoundedCommandJobFactory",
        factory_type: En1992BoundedCommandJobFactory,
        contract: crate::app_surface::norm_bounded_contract(),
        tools: ["setSnapshot", "evaluate", "setSelectedCheckIndex", "setActiveExample", "setField", "insertItem", "removeItem", "applyRemedy"]
    }


    /// 📎️ Norm application config and presence facets are both empty; Results windows register their own config.
    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::app_surface::app_schema_descriptor())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<results::ResultsWindowConfigOwner>()
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

    semio_s_artifact_norm_contract::norm_command_from_action!(En1992Command, crate::standards::v1::subsets::any::schema::snapshot::decode_en1992_snapshot_json);

    fn handle(
        command: &En1992Command,
        doc: &ArtifactView<'_, En1992Snapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<En1992Mutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        crate::app_surface::dispatch_norm_command::<Self>(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, En1992Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let host = NormHost::<En1992Family>::from_artifact(doc.snapshot.clone());
        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot, view_state.locale, CONTROLLER_ID, &semio_framework_plugin::TreeWindows::for_body(view_state, inputs::BODY_INPUTS)),
            results::BODY_RESULTS => results::render(&host, &semio_framework_plugin::TreeWindows::for_body(view_state, results::BODY_RESULTS), view_state.locale, Some(CONTROLLER_ID)),
            document_panel::BODY_ARTIFACT => document_panel::render(&host, view_state.locale),
            catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(Self::examples(), view_state.locale, CONTROLLER_ID, &semio_framework_plugin::TreeWindows::for_body(view_state, catalogue_panel::BODY_CATALOGUE)),
            inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, crate::results_window_config::current::<results::ResultsWindowConfigOwner>(cfg).selected_check_index, view_state.locale, Some(CONTROLLER_ID)),
            _ => crate::app_surface::render_unknown_body(body_key, view_state.locale),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    //#region ðï¸MediaPorts
    /// ðï¸ `"report:out"`/`"artifact:out"` â see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"artifact:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, En1992Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<En1992Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// ðï¸ `"model:in"`/`"artifact:in"` â see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, En1992Snapshot>) -> Result<Emit<En1992Mutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        crate::app_surface::import_media(port, media, |snapshot: En1992Snapshot| En1992Mutation::from_snapshot(&En1992Snapshot::default(), &snapshot))
    }
    //#endregion ðï¸MediaPorts
}
//#endregion ðï¸En1992PlayApp

//#region 🧵️RetainedCommands
crate::norm_owned_tool_job_factory!(En1992BoundedCommandJobFactory, En1992PlayApp);

impl crate::app_surface::NormRetainedEditor for En1992PlayApp {
    type Family = En1992Family;

    type ResultsWindowConfigOwner = results::ResultsWindowConfigOwner;

    fn selected_check_window_mutation(command: &En1992Command) -> Option<crate::results_window_config::NormResultsWindowConfigMutation> {
        match command {
            En1992Command::SetSelectedCheckIndex(payload) => Some(selected_check::window_mutation(payload)),
            _ => None,
        }
    }

    fn dispatch_retained(command: &En1992Command, doc: &ArtifactView<'_, En1992Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1992Mutation, NoConfigMutation, NoDraftMutation>, Fault> {
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
            // 📝️ `setSnapshot` replaces the whole compliance document, so the shells' `{action,args}`
            // channel needs somewhere to put it: one staged text argument carrying the document's own
            // camelCase JSON — the projection the Inputs window already renders. Without it the rail
            // stages no form and the bridge refuses `norm.set-snapshot-arg-missing` (ticket 26/09/18 S10).
            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("setSnapshot", LocalizedLabel::native("Set Snapshot", "Dokument setzen"), semio_framework_plugin::ActionKind::Mutation)
                    .with_args(vec![semio_framework_plugin::ActionArgDef::text("snapshot", LocalizedLabel::native("Document JSON", "Dokument-JSON"))]),
            )
            .action_destructive("setSnapshot")
            .action_with(semio_framework_plugin::ActionDefinition::new("evaluate", LocalizedLabel::native("Evaluate", "Auswerten"), semio_framework_plugin::ActionKind::View, "hash"))
            .view_action("setSelectedCheckIndex", LocalizedLabel::native("Set Selected Check", "Ausgewählte Prüfung setzen"))
            .action_interactive_job("setSnapshot", InteractiveJobClassification::Migrated)
            .action_interactive_job("evaluate", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), semio_framework_plugin::ActionKind::Mutation, "panel-left")
                    .with_args(vec![semio_framework_plugin::ActionArgDef::text("exampleId", LocalizedLabel::native("Example", "Beispiel"))]),
            )
            .action_destructive("setActiveExample")
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            
            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("setField", LocalizedLabel::native("Set Field", "Feld setzen"), semio_framework_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_framework_plugin::ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")),
                        semio_framework_plugin::ActionArgDef::text("value", LocalizedLabel::native("Value", "Wert")),
                    ]),
            )
            .action_interactive_job("setField", InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("insertItem", LocalizedLabel::native("Insert Item", "Eintrag einfügen"), semio_framework_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_framework_plugin::ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")),
                        semio_framework_plugin::ActionArgDef::text("index", LocalizedLabel::native("Index", "Index")),
                    ]),
            )
            .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("removeItem", LocalizedLabel::native("Remove Item", "Eintrag entfernen"), semio_framework_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_framework_plugin::ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")),
                        semio_framework_plugin::ActionArgDef::text("index", LocalizedLabel::native("Index", "Index")),
                    ]),
            )
            .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
            .action_with(
                semio_framework_plugin::ActionDefinition::new_catalog("applyRemedy", LocalizedLabel::native("Apply Remedy", "Abhilfe anwenden"), semio_framework_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_framework_plugin::ActionArgDef::text("checkId", LocalizedLabel::native("Check", "Nachweis")),
                        semio_framework_plugin::ActionArgDef::text("remedyIndex", LocalizedLabel::native("Remedy", "Abhilfe")),
                    ]),
            )
            .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder` takes a bare `AppDefinition` — there is no
            // `.example(...)`/`.workflow(...)` on this builder (see the pilot's w2-cad-report.md "SDK
            // gaps" #4), so the old app-level example/workflow registration is dropped here, not
            // silently: the subset's own `📚️examples/🎬️demo-session` facet (real content, moved
            // verbatim below) is the modern role-agnostic replacement surface for this.
            .action_describe("setSnapshot", LocalizedLabel::native("Replaces the whole EN 1992 (Eurocode 2, concrete structures) compliance document with the supplied document JSON; the previous inputs are discarded.", "Ersetzt das gesamte Nachweisdokument nach EN 1992 (Eurocode 2, Betonbau) durch das übergebene Dokument-JSON; die bisherigen Eingaben werden verworfen."))
            .action_describe("evaluate", LocalizedLabel::native("Recomputes every EN 1992 (Eurocode 2, concrete structures) check from the document's inputs and refreshes the results window; the document is not changed.", "Berechnet alle Nachweise nach EN 1992 (Eurocode 2, Betonbau) aus den Eingaben des Dokuments neu und aktualisiert das Ergebnisfenster; das Dokument ändert sich nicht."))
            .action_describe("setSelectedCheckIndex", LocalizedLabel::native("Points the inspection panel at one computed check by its index in the results list; only the view changes.", "Richtet das Inspektionspanel anhand seines Index in der Ergebnisliste auf einen berechneten Nachweis aus; nur die Ansicht ändert sich."))
            .action_describe("setActiveExample", LocalizedLabel::native("Loads one of the bundled EN 1992 (Eurocode 2, concrete structures) examples into the open compliance document, replacing its inputs, by example id.", "Lädt eines der mitgelieferten Beispiele nach EN 1992 (Eurocode 2, Betonbau) in das offene Nachweisdokument und ersetzt dessen Eingaben, anhand der Beispiel-Id."))
            .build_definition()
}
//#endregion ðï¸Manifest

//#region ð§ªï¸UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion ð§ªï¸UnitTests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
