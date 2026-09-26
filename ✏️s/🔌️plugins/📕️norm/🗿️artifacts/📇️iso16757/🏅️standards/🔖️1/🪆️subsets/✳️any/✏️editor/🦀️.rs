//! 📇️ ISO 16757 play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the two surfaces
//! in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

use crate::document::NormHost;
use crate::editor::iso16757::commands::{apply_remedy, evaluate, insert_item, remove_item, selected_check, set_active_example, set_field, set_snapshot};
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
/// 🆔️ Retained-command / UI action controller id.
pub const CONTROLLER_ID: &str = "s.norm.iso16757@1/*#editor";
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
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "setField" as "set-field" => set_field::SetField,
        "insertItem" as "insert-item" => insert_item::InsertItem,
        "removeItem" as "remove-item" => remove_item::RemoveItem,
        "applyRemedy" as "apply-remedy" => apply_remedy::ApplyRemedy,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Iso16757PlayApp
#[derive(Default)]
pub struct Iso16757PlayApp;

impl ArtifactEditor for Iso16757PlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::demo::source(), crate::examples::broken::source()]
    }
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
        artifact_schema: "semio.norm.iso16757/v1",
        factory: "Iso16757BoundedCommandJobFactory",
        factory_type: Iso16757BoundedCommandJobFactory,
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

    fn initial_snapshot() -> Iso16757Snapshot {
        Iso16757Snapshot::default()
    }

    fn io() -> Option<AppIo> {
        Some(crate::app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn command_id(command: &Iso16757Command) -> &'static str {
        command.command_id()
    }

    semio_s_artifact_norm_contract::norm_command_from_action!(Iso16757Command, crate::standards::v1::subsets::any::schema::snapshot::decode_iso16757_snapshot_json);

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

    fn render(body_key: &str, doc: &ArtifactView<'_, Iso16757Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let host = NormHost::<Iso16757Family>::from_artifact(doc.snapshot.clone());
        match body_key {
            inputs::BODY_INPUTS => inputs::render(doc.snapshot, view_state.locale, CONTROLLER_ID, &semio_framework_plugin::TreeWindows::for_body(view_state, inputs::BODY_INPUTS)),
            results::BODY_RESULTS => results::render(&host, &semio_framework_plugin::TreeWindows::for_body(view_state, results::BODY_RESULTS), view_state.locale, Some(CONTROLLER_ID)),
            document_panel::BODY_ARTIFACT => document_panel::render(&host, view_state.locale),
            catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(Self::examples(), view_state.locale, CONTROLLER_ID),
            inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, crate::results_window_config::current::<results::ResultsWindowConfigOwner>(cfg).selected_check_index, view_state.locale, Some(CONTROLLER_ID)),
            _ => crate::app_surface::render_unknown_body(body_key, view_state.locale),
        }
        .map(semio_framework_plugin::built_to_component_tree)
    }

    //#region 🔖️MediaPorts
    /// 🎞️ `"report:out"`/`"artifact:out"` — see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"artifact:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, Iso16757Snapshot>) -> Result<Media, MediaError> {
        crate::app_surface::export_media::<Iso16757Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// 🎞️ `"model:in"`/`"artifact:in"` — see `crate::app_surface::import_media`.
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
    type Family = Iso16757Family;

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
            .action_describe("setSnapshot", LocalizedLabel::native("Replaces the whole ISO 16757 (product data for building services catalogues) compliance document with the supplied document JSON; the previous inputs are discarded.", "Ersetzt das gesamte Nachweisdokument nach ISO 16757 (Produktdaten für Kataloge der Gebäudetechnik) durch das übergebene Dokument-JSON; die bisherigen Eingaben werden verworfen."))
            .action_describe("evaluate", LocalizedLabel::native("Recomputes every ISO 16757 (product data for building services catalogues) check from the document's inputs and refreshes the results window; the document is not changed.", "Berechnet alle Nachweise nach ISO 16757 (Produktdaten für Kataloge der Gebäudetechnik) aus den Eingaben des Dokuments neu und aktualisiert das Ergebnisfenster; das Dokument ändert sich nicht."))
            .action_describe("setSelectedCheckIndex", LocalizedLabel::native("Points the inspection panel at one computed check by its index in the results list; only the view changes.", "Richtet das Inspektionspanel anhand seines Index in der Ergebnisliste auf einen berechneten Nachweis aus; nur die Ansicht ändert sich."))
            .action_describe("setActiveExample", LocalizedLabel::native("Loads one of the bundled ISO 16757 (product data for building services catalogues) examples into the open compliance document, replacing its inputs, by example id.", "Lädt eines der mitgelieferten Beispiele nach ISO 16757 (Produktdaten für Kataloge der Gebäudetechnik) in das offene Nachweisdokument und ersetzt dessen Eingaben, anhand der Beispiel-Id."))
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
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
