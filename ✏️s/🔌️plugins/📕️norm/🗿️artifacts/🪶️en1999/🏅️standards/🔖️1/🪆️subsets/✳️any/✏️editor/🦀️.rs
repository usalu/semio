//! ✨️ EN 1999 play app â the `ArtifactApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `ð®ï¸commands/*`, the two surfaces
//! in `ð­ï¸modes/âï¸edit/ðªï¸windows/*`, panel trees in `ðï¸panels/*`, compliance compute in
//! the sibling command/panel/window nodes moved here too, and everything the fifteen norm apps share verbatim (config,
//! media ports, render primitives, manifest constructors) in `crate::document::app` / `crate::document::config`.

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::NormHost;
use crate::commands::{apply_remedy, insert_item, remove_item, selected_check, set_field, set_fields, preview_remedy, clear_preview, cancel_evaluation, evaluate_units, copy_entities, duplicate_entities, paste_entities, move_entity, delete_entities};
use crate::modes::edit as edit_mode;
use crate::modes::edit::windows::{checks, diagrams, outliner, properties, viewport};
use crate::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use semio_s_artifact_norm_en1999_mutations::En1999Mutation;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_plugin::app::InteractionView;
use semio_plugin::InteractiveJobClassification;
use semio_plugin::AppIo;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::Media;
use semio_plugin::MediaError;
use semio_plugin::NoConfig;
use semio_plugin::NoConfigMutation;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::app::Dialect;
use semio_framework_2d_compute::EngineHandles;

//#region ðï¸Constants
/// 🏷️ This standard's display name â the app label, its artifact-kind name and the catalogue headline.
pub const LABEL: &str = "EN 1999";
/// 🆔️ The playground/registry variant key â every body key, window id and schema is derived from it.
pub const VARIANT: &str = "en1999";
/// 🆔️ Retained-command / UI action controller id.
pub const CONTROLLER_ID: &str = "s.norm.en1999@1/*#editor";
pub const DOCUMENT_SCHEMA: &str = "semio.norm.en1999/v1";

/// 🏷️ The scene entity kinds EN 1999 projects: one interaction granularity each.
pub const ENTITY_KIND_SCENE_KINDS: &[semio_s_plugin_norm_structural_scene::EntityKind] = &[
    semio_s_plugin_norm_structural_scene::EntityKind::Member,
    semio_s_plugin_norm_structural_scene::EntityKind::Plate,
    semio_s_plugin_norm_structural_scene::EntityKind::Shell,
    semio_s_plugin_norm_structural_scene::EntityKind::Fastener,
    semio_s_plugin_norm_structural_scene::EntityKind::Support,
    semio_s_plugin_norm_structural_scene::EntityKind::Load,
    semio_s_plugin_norm_structural_scene::EntityKind::Group,
];
//#endregion ðï¸Constants

//#region ðï¸Commands
semio_plugin::app_commands! {
    /// 🎯️ `En1999PlayApp::Command` â the SOLE dispatch surface for this app's own behavior, covering every
    /// action `create_en1999_app` declares. Row order IS the binary variant ordinal (appending is safe,
    /// reordering is a wire-format break) and each row's two literals are the camelCase manifest action
    /// id and the kebab `#[dsl(key)]` wire keyword respectively â both written out explicitly,
    /// never derived from one another.
    pub enum En1999Command for En1999Snapshot, En1999Mutation, NoConfig, NoConfigMutation {
        "setSelectedCheckIndex" as "selected-check" => selected_check::SetSelectedCheckIndex,
        "setField" as "set-field" => set_field::SetField,
        "insertItem" as "insert-item" => insert_item::InsertItem,
        "removeItem" as "remove-item" => remove_item::RemoveItem,
        "applyRemedy" as "apply-remedy" => apply_remedy::ApplyRemedy,
        "setFields" as "set-fields" => set_fields::SetFields,
        "previewRemedy" as "preview-remedy" => preview_remedy::PreviewRemedy,
        "clearPreview" as "clear-preview" => clear_preview::ClearPreview,
        "cancelEvaluation" as "cancel-evaluation" => cancel_evaluation::CancelEvaluation,
        "evaluateUnits" as "evaluate-units" => evaluate_units::EvaluateUnits,
        "viewportAct" as "viewport-act" => viewport_act::ViewportAct,
        "copyEntities" as "copy-entities" => copy_entities::CopyEntities,
        "duplicateEntities" as "duplicate-entities" => duplicate_entities::DuplicateEntities,
        "pasteEntities" as "paste-entities" => paste_entities::PasteEntities,
        "moveEntity" as "move-entity" => move_entity::MoveEntity,
        "deleteEntities" as "delete-entities" => delete_entities::DeleteEntities,
    }
}
//#endregion ðï¸Commands

//#region ðï¸En1999PlayApp
/// 🎪️ The "full showcase example" of the new-document palette: the document that populates every entity collection and runs every check family of the spec trace, so the whole code is visible in 3D from one click.
fn full_showcase_example() -> semio_plugin::ExampleSource {
    semio_plugin::ExampleSource::new("full-showcase", LocalizedLabel::native("Full showcase example", "Vollständiges Gesamtbeispiel"), semio_s_artifact_norm_en1999_io::text::snapshot::encode_en1999_dsl(&En1999Snapshot::full_showcase()), "file")
}

/// 🎪️ The "aluminium showcase" example of the new-document palette: the committed showcase DSL asset, the aluminium document of the showcase fixture: extruded and free sections, members, welded, pinned and bolted connections, fire, fatigue and patch loads.
fn aluminium_showcase_example() -> semio_plugin::ExampleSource {
    semio_plugin::ExampleSource::new("aluminium-showcase", LocalizedLabel::native("Aluminium showcase", "Aluminium-Gesamtbeispiel"), include_str!("../🖼️assets/🏠️aluminium-showcase/🏠️aluminium-showcase/🗣️.dsl.semio"), "file")
}

#[derive(Default)]
pub struct En1999PlayApp;

impl ArtifactEditor for En1999PlayApp {
    /// 📢️ Localized notices of the refusal and warning codes this norm app raises.
    fn fault_notices() -> &'static [(&'static str, semio_ui_locale::LocalizedLabel)] {
        semio_s_plugin_norm_app_surface::norm_fault_notices()
    }
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_plugin::ExampleSource> {
        vec![semio_s_artifact_norm_en1999_examples::aluminium_roof_purlin::source(), semio_s_artifact_norm_en1999_examples::noncompliant_multi_fail::source()].into_iter().chain([full_showcase_example(), aluminium_showcase_example()]).collect()
    }
    type Snapshot = En1999Snapshot;
    type Mutation = En1999Mutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::lanes::NormLanePresence;
    type PresenceMutation = crate::lanes::NormLanePresenceMutation;
    type Transient = crate::lanes::NormLaneTransient;
    type TransientMutation = crate::lanes::NormLaneTransientMutation;

    type Command = En1999Command;

    fn interaction_topology(doc: &ArtifactView<'_, En1999Snapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<semio_plugin::InteractionTopology, semio_framework_value::ValueError> {
        semio_s_plugin_norm_app_surface::live_interaction_topology::<Self>(doc.snapshot, checks::INTERACTION_DOMAIN)
    }

    const DIALECT: Dialect = semio_s_artifact_norm_en1999_model::EN1999_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.norm.en1999/v1";

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        semio_s_plugin_norm_app_surface::norm_artifact_store_preparation::<Self>()
    }

    semio_s_plugin_norm_app_surface::norm_exact_store_ownership!(live);

    fn register_tool_job_factories(registry: &mut semio_plugin::ArtifactToolFactoryRegistry<'_, semio_plugin::EditorApp<Self>>) -> Result<(), Fault> {
        En1999BoundedCommandJobFactory::register(registry)
    }

    fn build_tool_job(request: &mut Option<semio_plugin::ArtifactOwnedToolJobRequest<semio_plugin::EditorApp<Self>>>, output: &mut Option<semio_plugin::ErasedToolJob>, cx: &mut semio_plugin::__job::StepContext<'_>) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        semio_s_plugin_norm_app_surface::build_norm_tool_job::<Self>(request, output, cx)
    }

    semio_plugin::bounded_first_step_tool_proofs! {
        owner: semio_plugin::EditorApp<En1999PlayApp>,
        owner_file: "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.norm.en1999@1/*#editor",
        artifact_schema: "semio.norm.en1999/v1",
        factory: "En1999BoundedCommandJobFactory",
        factory_type: En1999BoundedCommandJobFactory,
        contract: semio_s_plugin_norm_app_surface::norm_bounded_contract(),
        tools: ["setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy", "setFields", "previewRemedy", "clearPreview", "cancelEvaluation", "evaluateUnits", "viewportAct", "copyEntities", "duplicateEntities", "pasteEntities", "moveEntity", "deleteEntities"]
    }

    /// 📎️ Norm application config and presence facets are both empty; Results windows register their own config.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_plugin_norm_app_surface::app_schema_descriptor())
    }

    fn register_window_config_owners(registry: &mut semio_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<checks::ChecksWindowConfigOwner>()
    }

    fn reference_birth()->Option<fn()->Result<En1999Snapshot,semio_plugin::Fault>>{Some(||{
        Ok(En1999Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De))
    })}

    fn io() -> Option<AppIo> {
        Some(semio_s_plugin_norm_app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
    }

    fn ephemeral(
        command: &En1999Command,
        doc: &ArtifactView<'_, En1999Snapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        presence: &semio_plugin::PresenceView<'_, Self::Presence>,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
    ) -> (Vec<Self::PresenceMutation>, Vec<Self::TransientMutation>) {
        semio_s_plugin_norm_app_surface::live_ephemeral::<Self>(command, doc, presence, transient)
    }

    fn command_id(command: &En1999Command) -> &'static str {
        command.command_id()
    }

    semio_s_plugin_norm_app_surface::norm_command_from_action!(En1999Command, live, semio_s_artifact_norm_en1999_io::text::snapshot::decode_en1999_snapshot_json);
    semio_s_plugin_norm_app_surface::norm_context_menu!();

    fn handle(
        command: &En1999Command,
        doc: &ArtifactView<'_, En1999Snapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        view_state: Option<&semio_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<En1999Mutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        semio_s_plugin_norm_app_surface::dispatch_norm_command::<Self>(command, doc, cfg, view_state)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, En1999Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &[], &semio_s_plugin_norm_app_surface::LaneFrame::default())
    }

    fn render_with_request_context(
        _owner: &semio_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, En1999Snapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &semio_plugin::ViewModel,
        transient: &semio_plugin::TransientView<'_, Self::Transient>,
        interaction: &InteractionView<'_>,
        _cx: &mut semio_plugin::StepContext<'_>,
    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &interaction.selection(&semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT)).ids, &semio_s_plugin_norm_app_surface::LaneFrame::of(transient.snapshot, interaction, &semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT))).map(Some).map_err(Into::into)
    }

    /// 🎞️ `"report:out"`/`"artifact:out"` â see `crate::app_surface::export_media`, which all fifteen apps
    /// share (overriding this method shadows the SDK default entirely, so `"artifact:out"` is
    /// re-implemented there rather than left unreachable).
    fn export_media(port: &str, doc: &ArtifactView<'_, En1999Snapshot>) -> Result<Media, MediaError> {
        semio_s_plugin_norm_app_surface::export_media::<En1999Family>(port, VARIANT, DOCUMENT_SCHEMA, doc.snapshot)
    }

    /// 🎞️ `"model:in"`/`"artifact:in"` â see `crate::app_surface::import_media`.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, En1999Snapshot>) -> Result<Emit<En1999Mutation, NoConfigMutation, Self::DraftMutation>, MediaError> {
        semio_s_plugin_norm_app_surface::import_media(port, media)
    }
}
//#endregion ðï¸En1999PlayApp

//#region 🪄️LiveRender
fn render_body(body_key: &str, doc: &ArtifactView<'_, En1999Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_plugin::ViewModel, picked: &[String], frame: &semio_s_plugin_norm_app_surface::LaneFrame) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {
    let live = semio_s_plugin_norm_app_surface::LiveView::<En1999Family>::with_lanes(doc.snapshot.clone(), semio_s_plugin_norm_results_config::current::<checks::ChecksWindowConfigOwner>(cfg).selected_check_index, picked, &frame.transient);
    let host = &live.host;
    let selection = &live.selection;
    let selected = selection.checks_selected();
    let focus = live.focus.clone();
    let worst = live.worst.as_deref();
    match body_key {
        outliner::BODY_OUTLINER => outliner::render(host, selection, view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, outliner::BODY_OUTLINER)),
        properties::BODY_PROPERTIES => properties::render(host, selection, view_state.locale, CONTROLLER_ID, &semio_plugin::TreeWindows::for_body(view_state, properties::BODY_PROPERTIES)),
        viewport::BODY_VIEWPORT => viewport::render(host.document(), host.report(), selection, frame, view_state.locale),
        diagrams::BODY_DIAGRAMS => diagrams::render(host.document(), host.report(), focus.as_deref().or(worst), &semio_plugin::TreeWindows::for_body(view_state, diagrams::BODY_DIAGRAMS), view_state.locale),
        checks::BODY_CHECKS => live.render_checks(&semio_plugin::TreeWindows::for_body(view_state, checks::BODY_CHECKS), view_state.locale, CONTROLLER_ID, checks::INTERACTION_DOMAIN),
        document_panel::BODY_ARTIFACT => document_panel::render(&host, view_state.locale),
        catalogue_panel::BODY_CATALOGUE => catalogue_panel::render(En1999PlayApp::examples(), view_state.locale, CONTROLLER_ID),
        inspection_panel::BODY_INSPECTION => inspection_panel::render(&host, selected, view_state.locale, Some(CONTROLLER_ID)),
        _ => semio_s_plugin_norm_app_surface::render_unknown_body(body_key, view_state.locale),
    }
    .map(semio_plugin::built_to_component_tree)
}
//#endregion 🪄️LiveRender

//#region 🫧️LiveEditor
fn live_entities(document: &En1999Snapshot) -> Vec<semio_s_plugin_norm_structural_scene::EntityRef> {
    viewport::scene_of(document, &<En1999Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(document)).entities.into_iter().map(|entity| entity.entity).collect()
}

fn live_handles(document: &En1999Snapshot, selection: &[semio_s_plugin_norm_structural_scene::EntityRef]) -> Vec<semio_s_plugin_norm_structural_scene::Handle> {
    viewport::handles_of(document, selection)
}

semio_s_plugin_norm_app_surface::norm_live_editor!(En1999PlayApp, En1999Command, handles = live_handles, entities = live_entities, palette = viewport::insert_specs, resolve = semio_s_artifact_norm_en1999_mutations::resolve, scene = |d, r, _| viewport::scene_of(d, r), diagrams = |d, r| diagrams::all(d, r));
semio_s_plugin_norm_app_surface::norm_live_acceptance!(En1999PlayApp, entities = live_entities);
semio_s_plugin_norm_app_surface::norm_latency_bench!(En1999PlayApp, variant = "en1999", seed = semio_s_artifact_norm_en1999_model::En1999Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), scene = viewport::scene_of, full = viewport::full_scene_of);
//#endregion 🫧️LiveEditor

//#region 🧵️RetainedCommands
semio_s_plugin_norm_app_surface::norm_owned_tool_job_factory!(En1999BoundedCommandJobFactory, En1999PlayApp);

impl semio_s_plugin_norm_app_surface::NormRetainedEditor for En1999PlayApp {
    fn inspector_rules() -> &'static semio_s_plugin_norm_app_surface::NormEditRules { &semio_s_artifact_norm_en1999_mutations::EDIT_RULES }

    fn inspector_resolve(document: &Self::Snapshot, edit: &semio_s_plugin_norm_app_surface::NormEdit) -> Result<Vec<Self::Mutation>, String> {
        semio_s_artifact_norm_en1999_mutations::resolve_edit(document, edit)
    }

    type Family = En1999Family;

    type ResultsWindowConfigOwner = checks::ChecksWindowConfigOwner;

    const TOOL_IDS: &'static [&'static str] = semio_s_plugin_norm_app_surface::NORM_EDITOR_TOOL_IDS;
    const PUBLICATION_CONTRACTS: &'static [semio_plugin::ArtifactToolPublicationContract] = semio_s_plugin_norm_app_surface::NORM_EDITOR_PUBLICATION_CONTRACTS;

    fn selected_check_window_mutation(command: &En1999Command) -> Option<semio_s_plugin_norm_results_config::NormResultsWindowConfigMutation> {
        match command {
            En1999Command::SetSelectedCheckIndex(payload) => Some(selected_check::window_mutation(payload)),
            _ => None,
        }
    }

    fn check_focus_effects(command: &En1999Command, document: &En1999Snapshot) -> Vec<semio_plugin::Effect> {
        match command {
            En1999Command::SetSelectedCheckIndex(payload) => semio_s_plugin_norm_app_surface::live_check_focus_effects::<Self>(document, payload.index, checks::INTERACTION_DOMAIN),
            _ => Vec::new(),
        }
    }

    fn view_focus_effects(command: &Self::Command, document: &Self::Snapshot, transient: &Self::Transient, interaction: &protocol::InteractionState) -> Vec<semio_plugin::Effect> {
        semio_s_plugin_norm_app_surface::live_view_focus_effects::<Self>(command, document, transient, interaction, checks::INTERACTION_DOMAIN)
    }

    fn dispatch_retained(command: &En1999Command, doc: &ArtifactView<'_, En1999Snapshot>, cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<En1999Mutation, NoConfigMutation, NoDraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }
}
//#endregion 🧵️RetainedCommands

//#region 🧩️ComplianceFamily
/// 🧩️ Headless `NormFamily` binding (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// relocated verbatim from the deleted `⚙️engine`. This is stateful/host-facing behaviour, so it
/// belongs to the app that edits the artifact, not the artifact's own `🧬️schema`.
pub struct En1999Family;

impl semio_s_plugin_norm_compliance::NormFamily for En1999Family {
    type Document = En1999Snapshot;
    type Mutation = En1999Mutation;

    fn family_id() -> semio_s_plugin_norm_compliance::NormFamilyId {
        semio_s_plugin_norm_compliance::NormFamilyId::En1999
    }

    fn evaluate(document: &En1999Snapshot) -> semio_s_plugin_norm_compliance::CheckReport {
        <En1999Family as semio_s_plugin_norm_compliance::IncrementalNormFamily>::evaluate_incrementally(document)
    }

    fn live_report(document: &En1999Snapshot) -> std::sync::Arc<semio_s_plugin_norm_compliance::CheckReport> {
        semio_s_artifact_norm_en1999_inferences::live::report(document)
    }

    fn live_progress() -> semio_s_plugin_norm_compliance::incremental_evaluation::EvalProgress {
        semio_s_artifact_norm_en1999_inferences::live::progress()
    }

    fn live_responsive(document: &En1999Snapshot, budget: std::time::Duration) -> (std::sync::Arc<semio_s_plugin_norm_compliance::CheckReport>, bool) {
        semio_s_artifact_norm_en1999_inferences::live::responsive(document, budget)
    }

    fn cancel_live() {
        semio_s_artifact_norm_en1999_inferences::live::cancel()
    }
}

pub type Host = NormHost<En1999Family>;
//#endregion 🧩️ComplianceFamily

//#region ðï¸Manifest
pub fn create_en1999_app() -> semio_plugin::AppDefinition {
    Editor::builder(semio_s_artifact_norm_en1999_model::EN1999_DIALECT)
            .document(["semio", "norm", VARIANT])
            .artifact_kind(semio_s_artifact_norm_en1999_model::artifact_kind())
            .io(semio_s_plugin_norm_app_surface::norm_io(VARIANT, DOCUMENT_SCHEMA))
            .mode_def(edit_mode::definition())
            .default_mode_id(semio_s_plugin_norm_app_surface::MODE_EDIT)
            .interaction(semio_s_plugin_norm_app_surface::viewport_interaction_definition(VARIANT, LocalizedLabel::native("EN 1999 model", "EN-1999-Modell"), ENTITY_KIND_SCENE_KINDS))
            .window_kind_def(outliner::definition())
            .window_kind_def(properties::definition())
            .window_kind_def(viewport::definition())
            .window_kind_def(diagrams::definition())
            .window_kind_def(checks::definition())
            .default_layout(edit_mode::layout())
            .named_layout(semio_s_plugin_norm_app_surface::edit_named_layout(VARIANT, "desktop"))
            .named_layout(semio_s_plugin_norm_app_surface::edit_named_layout(VARIANT, "mobile"))
            .named_layout(semio_s_plugin_norm_app_surface::edit_named_layout(VARIANT, "tablet"))
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .view_action("setSelectedCheckIndex", LocalizedLabel::native("Set Selected Check", "Ausgewählte Prüfung setzen"))
            .action_interactive_job("setSelectedCheckIndex", InteractiveJobClassification::Migrated)
            
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("setField", LocalizedLabel::native("Set Field", "Feld setzen"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")).required(),
                        semio_plugin::ActionArgDef::any("value", LocalizedLabel::native("Value", "Wert")).required(),
                    ]),
            )
            .action_interactive_job("setField", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("insertItem", LocalizedLabel::native("Insert Item", "Eintrag einfügen"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")).required(),
                        semio_plugin::ActionArgDef::index("index", LocalizedLabel::native("Index", "Index")).required(),
                        semio_plugin::ActionArgDef::any("value", LocalizedLabel::native("Value", "Wert")),
                    ]),
            )
            .action_interactive_job("insertItem", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("removeItem", LocalizedLabel::native("Remove Item", "Eintrag entfernen"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::text("path", LocalizedLabel::native("Path", "Pfad")).required(),
                        semio_plugin::ActionArgDef::index("index", LocalizedLabel::native("Index", "Index")).required(),
                    ]),
            )
            .action_interactive_job("removeItem", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("applyRemedy", LocalizedLabel::native("Apply Remedy", "Abhilfe anwenden"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::text("checkId", LocalizedLabel::native("Check", "Nachweis")).required(),
                        semio_plugin::ActionArgDef::index("remedyIndex", LocalizedLabel::native("Remedy", "Abhilfe")).required(),
                    ]),
            )
            .action_interactive_job("applyRemedy", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("setFields", LocalizedLabel::native("Set Fields", "Felder setzen"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::any("writes", LocalizedLabel::native("Writes", "Schreibvorgänge")).required(),
                        semio_plugin::ActionArgDef::any("gesture", LocalizedLabel::native("Gesture", "Geste")),
                    ]),
            )
            .action_interactive_job("setFields", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("previewRemedy", LocalizedLabel::native("Preview Remedy", "Abhilfe vorschauen"), semio_plugin::ActionKind::View)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::text("checkId", LocalizedLabel::native("Check", "Nachweis")),
                        semio_plugin::ActionArgDef::index("remedyIndex", LocalizedLabel::native("Remedy", "Abhilfe")),
                        semio_plugin::ActionArgDef::index("optionIndex", LocalizedLabel::native("Option", "Variante")),
                        semio_plugin::ActionArgDef::any("insert", LocalizedLabel::native("Insert ghost", "Einfügevorschau")),
                    ]),
            )
            .action_interactive_job("previewRemedy", InteractiveJobClassification::Migrated)
            .action_with(semio_plugin::ActionDefinition::new("clearPreview", LocalizedLabel::native("Clear Preview", "Vorschau verwerfen"), semio_plugin::ActionKind::View, "close"))
            .action_interactive_job("clearPreview", InteractiveJobClassification::Migrated)
            .action_with(semio_plugin::ActionDefinition::new("cancelEvaluation", LocalizedLabel::native("Cancel Evaluation", "Auswertung abbrechen"), semio_plugin::ActionKind::View, "close"))
            .action_interactive_job("cancelEvaluation", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("evaluateUnits", LocalizedLabel::native("Evaluate Units", "Einheiten auswerten"), semio_plugin::ActionKind::View)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::any("units", LocalizedLabel::native("Units", "Einheiten")),
                        semio_plugin::ActionArgDef::text("reason", LocalizedLabel::native("Reason", "Anlass")),
                    ]),
            )
            .action_interactive_job("evaluateUnits", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("viewportAct", LocalizedLabel::native("Viewport Action", "Viewport-Aktion"), semio_plugin::ActionKind::View)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::text("action", LocalizedLabel::native("Action", "Aktion")),
                        semio_plugin::ActionArgDef::text("text", LocalizedLabel::native("Text", "Text")),
                        semio_plugin::ActionArgDef::any("number", LocalizedLabel::native("Number", "Zahl")),
                    ]),
            )
            .action_interactive_job("viewportAct", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("copyEntities", LocalizedLabel::native("Copy Entities", "Bauteile kopieren"), semio_plugin::ActionKind::View)
                    .with_args(vec![semio_plugin::ActionArgDef::text("collection", LocalizedLabel::native("Collection", "Sammlung")), semio_plugin::ActionArgDef::any("ids", LocalizedLabel::native("Ids", "Kennungen"))]),
            )
            .action_interactive_job("copyEntities", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("duplicateEntities", LocalizedLabel::native("Duplicate Entities", "Bauteile duplizieren"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![semio_plugin::ActionArgDef::text("collection", LocalizedLabel::native("Collection", "Sammlung")), semio_plugin::ActionArgDef::any("ids", LocalizedLabel::native("Ids", "Kennungen"))]),
            )
            .action_interactive_job("duplicateEntities", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("pasteEntities", LocalizedLabel::native("Paste Entities", "Bauteile einfügen"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![semio_plugin::ActionArgDef::any("clip", LocalizedLabel::native("Clip", "Zwischenablage")), semio_plugin::ActionArgDef::any("at", LocalizedLabel::native("Position", "Position"))]),
            )
            .action_interactive_job("pasteEntities", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("moveEntity", LocalizedLabel::native("Move Entity", "Bauteil verschieben"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![
                        semio_plugin::ActionArgDef::text("path", LocalizedLabel::native("Collection", "Sammlung")),
                        semio_plugin::ActionArgDef::text("id", LocalizedLabel::native("Id", "Kennung")),
                        semio_plugin::ActionArgDef::any("to", LocalizedLabel::native("Position", "Position")),
                        semio_plugin::ActionArgDef::text("over", LocalizedLabel::native("Dropped on", "Abgelegt auf")),
                        semio_plugin::ActionArgDef::any("step", LocalizedLabel::native("Step", "Schritt")),
                    ]),
            )
            .action_interactive_job("moveEntity", InteractiveJobClassification::Migrated)
            .action_with(
                semio_plugin::ActionDefinition::new_catalog("deleteEntities", LocalizedLabel::native("Delete Entities", "Bauteile löschen"), semio_plugin::ActionKind::Mutation)
                    .with_args(vec![semio_plugin::ActionArgDef::text("collection", LocalizedLabel::native("Collection", "Sammlung")), semio_plugin::ActionArgDef::any("ids", LocalizedLabel::native("Ids", "Kennungen"))]),
            )
            .action_interactive_job("deleteEntities", InteractiveJobClassification::Migrated)
            .action_describe("copyEntities", LocalizedLabel::native("Names the rows of the selection as a clip for the clipboard; the document is not changed.", "Benennt die Zeilen der Auswahl als Ausschnitt für die Zwischenablage; das Dokument ändert sich nicht."))
            .action_describe("duplicateEntities", LocalizedLabel::native("Duplicates the selected rows with fresh ids as one undoable edit.", "Dupliziert die ausgewählten Zeilen mit neuen Kennungen als eine rückgängig machbare Änderung."))
            .action_describe("pasteEntities", LocalizedLabel::native("Pastes a clip with fresh ids as one undoable edit.", "Fügt einen Ausschnitt mit neuen Kennungen als eine rückgängig machbare Änderung ein."))
            .action_describe("moveEntity", LocalizedLabel::native("Moves a row to a position, onto another row or one step up or down as one undoable edit.", "Verschiebt eine Zeile an eine Position, auf eine andere Zeile oder um einen Schritt nach oben oder unten als eine rückgängig machbare Änderung."))
            .action_describe("deleteEntities", LocalizedLabel::native("Deletes the selected rows as one undoable edit; nothing is deleted when one of them no longer exists.", "Löscht die ausgewählten Zeilen als eine rückgängig machbare Änderung; existiert eine davon nicht mehr, wird nichts gelöscht."))
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("escape", "clearPreview")
            .action_describe("setSelectedCheckIndex", LocalizedLabel::native("Points the inspection panel at one computed check by its index in the checks list; only the view changes.", "Richtet das Inspektionspanel anhand seines Index in der Nachweisliste auf einen berechneten Nachweis aus; nur die Ansicht ändert sich."))
            .action_describe("setFields", LocalizedLabel::native("Writes several document fields as one undoable edit; a handle gesture only edits the document on release or Enter.", "Schreibt mehrere Dokumentfelder als eine rückgängig machbare Änderung; eine Griffgeste ändert das Dokument erst beim Loslassen oder mit Enter."))
            .action_describe("previewRemedy", LocalizedLabel::native("Shows what a remedy, or an inserted element at the cursor, would change as a ghost; the document is not changed.", "Zeigt als Vorschau, was eine Abhilfe oder ein am Cursor eingefügtes Element ändern würde; das Dokument ändert sich nicht."))
            .action_describe("clearPreview", LocalizedLabel::native("Discards the drag and ghost previews without a trace.", "Verwirft die Zieh- und Geistervorschau spurlos."))
            .action_describe("cancelEvaluation", LocalizedLabel::native("Cancels the evaluation in flight; the latest edit is evaluated next.", "Bricht die laufende Auswertung ab; als Nächstes wird die letzte Änderung ausgewertet."))
            .action_describe("evaluateUnits", LocalizedLabel::native("Re-runs the given check units, or all of them, now.", "Wertet die angegebenen Nachweiseinheiten oder alle sofort neu aus."))
            .action_describe("viewportAct", LocalizedLabel::native("Changes the viewport view only: colour field, parameter track, value, play or pause and speed; the document is not changed.", "Ändert nur die Viewport-Ansicht: Farbfeld, Parameterverlauf, Wert, Abspielen oder Anhalten und Geschwindigkeit; das Dokument ändert sich nicht."))
            .build_definition()
}
//#endregion ðï¸Manifest

//#region ð§ªï¸UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
#[cfg(test)]
#[path = "🧪️tests/📚️assets/🦀️.rs"]
mod asset_laws;
//#endregion ð§ªï¸UnitTests

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts

//#region 🪢️Mounts
#[path = "🏷️field-meta/🦀️.rs"]
pub mod field_meta;
#[path = "🧮️incremental/🦀️.rs"]
pub mod incremental;
#[path = "."]
pub mod commands {
    #[path = "🎮️commands/🩹apply-remedy/🦀️.rs"]
    pub mod apply_remedy;
    #[path = "🎮️commands/➕insert-item/🦀️.rs"]
    pub mod insert_item;
    #[path = "🎮️commands/➖remove-item/🦀️.rs"]
    pub mod remove_item;
    #[path = "🎮️commands/☑️selected-check/🦀️.rs"]
    pub mod selected_check;
    #[path = "🎮️commands/✏️set-field/🦀️.rs"]
    pub mod set_field;
    semio_s_plugin_norm_app_surface::norm_live_commands!(crate::En1999PlayApp);
}
#[path = "."]
pub mod config {
    #[path = "🎚️config/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod modes {
    #[path = "🎭️modes/✏️edit/🦀️.rs"]
    pub mod edit;
}
#[path = "."]
pub mod panels {
    #[path = "📌️panels/📚️catalogue/🦀️.rs"]
    pub mod catalogue;
    #[path = "📌️panels/🗿️artifact/🦀️.rs"]
    pub mod document;
    #[path = "📌️panels/🔍️inspection/🦀️.rs"]
    pub mod inspection;
}
#[path = "."]
pub mod presence {
    #[path = "👥️presence/🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "."]
pub mod lanes {
    semio_s_plugin_norm_app_surface::norm_lane_stores!(presence_extension = "norm.en1999.live.presence", transient_extension = "norm.en1999.live.transient", owner = "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️lanes");
}
//#endregion 🪢️Mounts
