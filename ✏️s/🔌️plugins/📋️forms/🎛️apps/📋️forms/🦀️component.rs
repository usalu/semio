//! 📋️ Forms play app — the `ArtifactApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/📝️blueprint/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view
//! state in `🦀️config.rs`, shared document compute in the artifact's `🧬️schema`, this app's own IO
//! surface in the `🔖️Io` region below. This file is a routing table:
//! `handle` → `FormsCommand::dispatch`, `render` → body-key → node, plus the app-level `🔖️Contributions`
//! region — host-declared plugin contributions (`config.contributions_json`) are consumed by THREE
//! taxonomy nodes (the blueprint builder's palette, the try wizard, and the inspection panel), and every
//! helper in that region takes `&FormsConfig` (an app-only view-state type) or an app-level
//! `ProgramContributionEntry`, so per the DocumentHelpers placement rule they stay here rather than in the
//! artifact's `🧬️schema`.

use crate::apps::forms::commands::{
    add_question, add_question_option, add_step, add_vector_field, drop_question_kind, export_fixture, move_question, move_step, next_step, patch_question_options,
    patch_questions, patch_step, patch_vector_field, previous_step, remove_question, remove_question_option, remove_step, remove_vector_field, reset_try, set_active_example,
    set_contributions, set_locale, set_selection, set_spec_json, set_try_value, set_try_values, submit, update_form,
};
use crate::apps::forms::config::{FormsConfig, FormsConfigMutation};
use crate::apps::forms::presence::{FormsPresence, FormsPresenceMutation};
use crate::apps::forms::modes::blueprint;
use crate::apps::forms::modes::blueprint::windows::{builder, try_wizard as try_window};
use crate::apps::forms::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::apps::forms::terminology::{forms_play_labels, FormsLabels};
use crate::artifacts::forms::schema::{default_example_json, onboarding_example_json};
use crate::artifacts::forms::op::FormMutation;
// 🧷️ Aliased: `app_commands!` below derives `dsl::DslOps` off the EXTERN `dsl` crate — importing the
// artifact's own `dsl` submodule under the bare name would shadow it (see the identical note in the
// artifact's `🧬️schema/🦀️component.rs`).
use crate::artifacts::forms::dsl as forms_dsl;
use crate::artifacts::forms::{FormQuestion, FormsSnapshot, FORMS_DOCUMENT_SCHEMA, FORM_BUILTIN_KINDS};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ArtifactKindSpec, ArtifactApp, ArtifactView, ConfigView, Emit, Fault, IconName, Label, LocalizedLabel, MediaClass, MediaError, MediaForm,
    MediaPayload, MediaType, OsMediaCapability, UiNode,
};
use store::EngineHandles;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

//#region 🔖️Constants
pub const FORMS_PLAY_APP_ID: &str = "forms-play";
pub use builder::FORMS_PLAY_BODY_BLUEPRINT;
pub use try_window::FORMS_PLAY_BODY_TRY;
pub use catalogue_panel::FORMS_PLAY_BODY_CATALOGUE;
pub use document_panel::FORMS_PLAY_BODY_DOCUMENT;
pub use inspection_panel::FORMS_PLAY_BODY_INSPECTION;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🪟️windows/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn forms_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(FORMS_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Values
/// 🔠️ `config.try_values_json`'s parsed form — the Try wizard's in-progress answer overrides (question id
/// -> value), heterogeneous per question kind so it stays a JSON blob in `FormsConfig` rather than a
/// typed `dsl` field (see `FormsConfig`'s doc). Falls back to an empty map on malformed JSON rather than
/// erroring, matching every other "best-effort parse of a config blob" call site.
pub fn try_values_map(config: &FormsConfig) -> Map<String, Value> {
    serde_json::from_str::<Value>(&config.try_values_json).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default()
}

pub fn try_values_json_text(values: &Map<String, Value>) -> String {
    serde_json::to_string(values).unwrap_or_else(|_| "{}".into())
}

pub fn effective_try_values(spec: &FormsSnapshot, config: &FormsConfig) -> Map<String, Value> {
    crate::artifacts::forms::schema::initial_try_values(spec, &try_values_map(config))
}

/// 🌱️ Building block for every `handle()` arm that must both clear the Try wizard's answers and reset its
/// active step — was `reset_try_runtime`'s effect on the old `FormsPlayRuntime`, now two config operations
/// instead of two field writes.
pub fn reset_try_config_mutations() -> Vec<FormsConfigMutation> {
    vec![FormsConfigMutation::SetTryValues { json: "{}".into() }, FormsConfigMutation::SetStepIndex { index: 0 }]
}

/// 🔠️ Parses a command's JSON-blob payload field (`value_json`/`values_json`/…), falling back to
/// `Value::Null` on malformed or absent JSON — every one of these fields is best-effort text carried
/// across the wire, not a validated protocol.
pub fn parse_value_json(value_json: &str) -> Value {
    serde_json::from_str(value_json).unwrap_or(Value::Null)
}
//#endregion 🔖️Values

//#region 🔖️Contributions
pub use semio_framework::ProgramContributionEntry;

pub fn forms_parse_contributions(config: &FormsConfig) -> Vec<ProgramContributionEntry> {
    semio_framework::parse_contributions(&config.contributions_json)
}

pub use forms_parse_contributions as parse_contributions;

/// 🗂️ `forms.questionKind` topic payload shape, decoded from the open `TopicContribution`.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FormsQuestionKindTopicPayload {
    app_id: String,
    question_kind: String,
    label: String,
    icon_id: IconName,
    params_body_key: String,
    preview_body_key: String,
}

const FORMS_QUESTION_KIND_TOPIC: &str = "forms.questionKind";

/// 🎯️ A resolved question-kind contribution's routing fields, sourced from the open `forms.questionKind`
/// topic shape.
struct QuestionKindRoute {
    app_id: String,
    params_body_key: String,
    preview_body_key: String,
}

fn question_kind_route_from_topic(topic_contribution: &semio_framework_plugin::TopicContribution, kind: &str) -> Option<QuestionKindRoute> {
    if topic_contribution.topic != FORMS_QUESTION_KIND_TOPIC {
        return None;
    }
    let payload = topic_contribution.decode::<FormsQuestionKindTopicPayload>().ok()?;
    (payload.question_kind == kind).then(|| QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })
}

/// 🗂️ Reads the open `TopicContribution` (`"forms.questionKind"` topic) shape per entry.
pub fn find_question_kind_contribution<'a>(contributions: &'a [ProgramContributionEntry], kind: &str) -> Option<(&'a str, QuestionKindRoute)> {
    contributions.iter().find_map(|entry| {
        let route = entry.topic_contribution.as_ref().and_then(|topic_contribution| question_kind_route_from_topic(topic_contribution, kind))?;
        Some((entry.plugin_id.as_str(), route))
    })
}

fn extension_params_value(question: &FormQuestion, values: &Map<String, Value>) -> Value {
    values.get(&question.id).cloned().or_else(|| question.params.as_ref().map(crate::artifacts::forms::schema::dsl_to_value)).unwrap_or_else(|| json!({}))
}

fn extension_render_payload(question: &FormQuestion, params: &Value, surface: &str, interactive: bool) -> String {
    serde_json::to_string(&json!({
        "fixtureSlug": question.fixture_slug.clone().unwrap_or_else(|| "hexagonal-mushroom-column".into()),
        "params": params,
        "questionId": question.id,
        "controllerId": FORMS_PLAY_APP_ID,
        "surface": surface,
        "interactive": interactive,
    }))
    .unwrap_or_else(|_| "{}".into())
}

/// 🧩️ Renders a contributed (extension) question kind as a pair of external slots (params editor +
/// preview), or an "Extension unavailable" diagnostic when no contribution is registered for it. Shared
/// by the try wizard and the inspection panel's kind-specific editor fields.
pub fn render_extension_question(question: &FormQuestion, values: &Map<String, Value>, contributions: &[ProgramContributionEntry], surface: &str, interactive: bool) -> UiNode {
    let Some((plugin_id, route)) = find_question_kind_contribution(contributions, &question.kind) else {
        return semio_framework_plugin::ui_text(Label::data(format!("Extension unavailable: {}", question.kind)));
    };
    let params = extension_params_value(question, values);
    let payload = extension_render_payload(question, &params, surface, interactive);
    semio_framework_plugin::ui_stack_vertical(vec![
        semio_framework_plugin::ui_external_slot(plugin_id, route.app_id.as_str(), route.params_body_key.as_str(), &payload),
        semio_framework_plugin::ui_external_slot(plugin_id, route.app_id.as_str(), route.preview_body_key.as_str(), &payload),
    ])
}

/// 🗂️ Every kind offered by the catalogue/inspector kind selector: the built-in kinds (labeled from
/// `labels`) followed by every contributed extension kind. Shared by the blueprint builder's palette, the
/// catalogue panel, and the inspection panel's kind select.
pub fn catalogue_kinds(contributions: &[ProgramContributionEntry], labels: &FormsLabels) -> Vec<(String, String, IconName)> {
    let mut kinds: Vec<(String, String, IconName)> = FORM_BUILTIN_KINDS
        .iter()
        .map(|kind| {
            let (label, icon): (&str, &str) = match *kind {
                "text" => (labels.kind_text.as_str(), "type"),
                "longText" => (labels.kind_long_text.as_str(), "align-left"),
                "number" => (labels.kind_number.as_str(), "hash"),
                "slider" => (labels.kind_slider.as_str(), "sliders-horizontal"),
                "boolean" => (labels.kind_boolean.as_str(), "toggle-left"),
                "single" => (labels.kind_single.as_str(), "circle-dot"),
                "multi" => (labels.kind_multi.as_str(), "list-checks"),
                "date" => (labels.kind_date.as_str(), "calendar"),
                "color" => (labels.kind_color.as_str(), "palette"),
                "image" => (labels.kind_image.as_str(), "image"),
                "file" => (labels.kind_file.as_str(), "file"),
                "vector" => (labels.kind_vector.as_str(), "move-3d"),
                "note" => (labels.kind_note.as_str(), "sticky-note"),
                other => (other, "help-circle"),
            };
            (kind.to_string(), label.into(), icon.into())
        })
        .collect();
    for entry in contributions {
        let topic_kind = entry
            .topic_contribution
            .as_ref()
            .filter(|topic_contribution| topic_contribution.topic == FORMS_QUESTION_KIND_TOPIC)
            .and_then(|topic_contribution| topic_contribution.decode::<FormsQuestionKindTopicPayload>().ok())
            .map(|payload| (payload.question_kind, payload.label, payload.icon_id));
        if let Some(kind) = topic_kind {
            kinds.push(kind);
        }
    }
    kinds
}
//#endregion 🔖️Contributions

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `FormsPlayApp::Command` — the SOLE dispatch surface for forms' own behavior, assembled from the
    /// `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`, the
    /// camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — they are genuinely different vocabularies, and
    /// `setLocale`/`locale` is the row that proves it. **Row order is the binary variant ordinal:
    /// appending is safe, reordering is a wire-format break.**
    pub enum FormsCommand for FormsSnapshot, FormMutation, FormsConfig, FormsConfigMutation {
        "setSelection" as "selection" => set_selection::SetSelection,
        "setTryValue" as "try-value" => set_try_value::SetTryValue,
        "setTryValues" as "try-values" => set_try_values::SetTryValues,
        "resetTry" as "reset-try" => reset_try::ResetTry,
        "previousStep" as "previous-step" => previous_step::PreviousStep,
        "nextStep" as "next-step" => next_step::NextStep,
        "submit" as "submit" => submit::Submit,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "addStep" as "add-step" => add_step::AddStep,
        "patchStep" as "patch-step" => patch_step::PatchStep,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "moveStep" as "move-step" => move_step::MoveStep,
        "updateForm" as "update-form" => update_form::UpdateForm,
        "addQuestion" as "add-question" => add_question::AddQuestion,
        "removeQuestion" as "remove-question" => remove_question::RemoveQuestion,
        "patchQuestions" as "patch-questions" => patch_questions::PatchQuestions,
        // 🩹️ `patchQuestionOptions`..`removeVectorField` (rows 18-23) come BEFORE `moveQuestion`/
        // `dropQuestionKind` (rows 24-25) here, even though all 5 live in the same `🎮️commands/*` files —
        // this row order is the pre-migration `forms_protocol::FormsCommand`'s exact binary variant
        // ordinal and must not be re-sorted by file grouping (see the doc comment above: reordering is a
        // wire-format break no round-trip test catches, since every row still round-trips fine on its own).
        "patchQuestionOptions" as "patch-question-options" => patch_question_options::PatchQuestionOptions,
        "addQuestionOption" as "add-question-option" => add_question_option::AddQuestionOption,
        "removeQuestionOption" as "remove-question-option" => remove_question_option::RemoveQuestionOption,
        "patchVectorField" as "patch-vector-field" => patch_vector_field::PatchVectorField,
        "addVectorField" as "add-vector-field" => add_vector_field::AddVectorField,
        "removeVectorField" as "remove-vector-field" => remove_vector_field::RemoveVectorField,
        "moveQuestion" as "move-question" => move_question::MoveQuestion,
        "dropQuestionKind" as "drop-question-kind" => drop_question_kind::DropQuestionKind,
        "setSpecJson" as "spec-json" => set_spec_json::SetSpecJson,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "exportFixture" as "export-fixture" => export_fixture::ExportFixture,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.

//#endregion 🔖️Commands

//#region 🔖️Io
/// 🔌️ Forms' typed media I/O surface (`AppDefinition.io`) — the implicit `document:in`/`document:out`
/// pair (keyed by the `forms.form` document schema) plus the WORKFLOWS-END-TO-END-TYPED-PORTS
/// `dictionary:out` port: the form's currently-configured default field values (see
/// `crate::artifacts::forms::schema::initial_try_values`), re-exported as a `form.dictionary` JSON
/// object keyed by question id — the layout app's `fields:in` counterpart. Relocated from the deleted
/// artifact `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is the app's
/// own IO surface, not artifact behaviour.
pub fn forms_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: FORMS_DOCUMENT_SCHEMA.into(),
        document_media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "dictionary:out".into(),
            label: "Dictionary".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Data, form: semio_framework_plugin::MediaForm::Value },
            kind_id: Some("form.dictionary".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "form.dictionary".into(), name: "Form".into(), dimension: "data".into(), component_kind: "forms".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️FormsPlayApp
/// 🧪️ B1: unit struct — every former `FormsPlayRuntime` field now lives in `FormsConfig`, written through
/// `FormsConfigMutation`s.
#[derive(Default)]
pub struct FormsPlayApp;

impl ArtifactApp for FormsPlayApp {
    type Snapshot = FormsSnapshot;
    type Mutation = FormMutation;
    type Config = FormsConfig;
    type ConfigMutation = FormsConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = FormsPresence;
    type PresenceMutation = FormsPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = FormsCommand;

    const APP_ID: &'static str = FORMS_PLAY_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::apps::forms::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> FormsSnapshot {
        crate::artifacts::forms::schema::building_component_spec()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(forms_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale`/`setContributions` have no manifest
    /// declaration (host-pushed, not user-facing actions).
    fn command_id(command: &FormsCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &FormsCommand, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<FormMutation, FormsConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    //#region 🔖️Media
    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `document:out` replicates the trait default
    /// exactly (overriding `export_media` for `dictionary:out` forfeits the default's dispatch);
    /// `dictionary:out` re-exports the form's currently-configured default field values as a
    /// `form.dictionary` JSON object keyed by question id — no `cfg` parameter reaches this method, so
    /// this is the form's authored defaults, not a live in-progress Try-wizard session (that lives in
    /// `Self::Config`).
    fn export_media(port: &str, doc: &ArtifactView<'_, FormsSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "document:out" => {
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: FORMS_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            "dictionary:out" => {
                let values = crate::artifacts::forms::schema::initial_try_values(doc.snapshot, &Map::new());
                let json = serde_json::to_string(&Value::Object(values)).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "form.dictionary".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    fn render(body_key: &str, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> UiNode {
        let spec = doc.snapshot;
        let config = cfg.snapshot;
        let labels = forms_play_labels(config);
        match body_key {
            FORMS_PLAY_BODY_BLUEPRINT => builder::render(spec, config, labels),
            FORMS_PLAY_BODY_TRY => try_window::render(spec, config, labels),
            FORMS_PLAY_BODY_DOCUMENT => document_panel::render(spec, &config.selected_ids, labels),
            FORMS_PLAY_BODY_CATALOGUE => catalogue_panel::render(config, labels),
            FORMS_PLAY_BODY_INSPECTION => inspection_panel::render(spec, config, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️FormsPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_forms_app() -> App {
    App::from_builder(
        App::builder(FORMS_PLAY_APP_ID, LocalizedLabel::native("Forms", "Formulare"))
            .document(["semio", "forms"])
            .artifact_kind(ArtifactKindSpec {
                id: "form.dictionary".into(),
                name: "Form Dictionary".into(),
                source_format: "form.dictionary".into(),
                component_kind: "forms".into(),
                dimension: "data".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                schema: "form.dictionary".into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("forms")
            .mode_def(blueprint::definition())
            .default_mode_id(blueprint::FORMS_PLAY_MODE_BLUEPRINT)
            .window_kind_def(builder::definition())
            .window_kind_def(try_window::definition())
            .default_layout(blueprint::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .mutation("addQuestion", LocalizedLabel::native("Add Question", "Frage hinzufügen"))
            .mutation("removeQuestion", LocalizedLabel::native("Remove Question", "Frage entfernen"))
            .mutation("patchQuestions", LocalizedLabel::native("Patch Questions", "Fragen aktualisieren"))
            .mutation("patchQuestionOptions", LocalizedLabel::native("Patch Question Options", "Fragenoptionen aktualisieren"))
            .mutation("addQuestionOption", LocalizedLabel::native("Add Question Option", "Fragenoption hinzufügen"))
            .mutation("removeQuestionOption", LocalizedLabel::native("Remove Question Option", "Fragenoption entfernen"))
            .mutation("patchVectorField", LocalizedLabel::native("Patch Vector Field", "Vektorfeld aktualisieren"))
            .mutation("addVectorField", LocalizedLabel::native("Add Vector Field", "Vektorfeld hinzufügen"))
            .mutation("removeVectorField", LocalizedLabel::native("Remove Vector Field", "Vektorfeld entfernen"))
            .mutation("moveQuestion", LocalizedLabel::native("Move Question", "Frage verschieben"))
            .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
            .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
            .mutation("patchStep", LocalizedLabel::native("Patch Step", "Schritt aktualisieren"))
            .mutation("updateForm", LocalizedLabel::native("Update Form", "Formular aktualisieren"))
            .mutation("updatePlaybook", LocalizedLabel::native("Update Playbook", "Playbook aktualisieren"))
            .mutation("dropQuestionKind", LocalizedLabel::native("Drop Question Kind", "Frageart ablegen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            // 🛠️ Dev-only whole-spec import — kept out of the command palette, staged JSON form.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setSpecJson", LocalizedLabel::native("Set Spec JSON", "Spezifikations-JSON festlegen"), ActionKind::Mutation) })
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("setTryValue", LocalizedLabel::native("Set Try Value", "Testwert festlegen"))
            .view_action("setTryValues", LocalizedLabel::native("Set Try Values", "Testwerte festlegen"))
            .view_action("resetTry", LocalizedLabel::native("Reset Try", "Test zurücksetzen"))
            .view_action("previousStep", LocalizedLabel::native("Previous Step", "Vorheriger Schritt"))
            .view_action("nextStep", LocalizedLabel::native("Next Step", "Nächster Schritt"))
            .view_action("submit", LocalizedLabel::native("Submit", "Absenden"))
            .shell_action("exportFixture", LocalizedLabel::native("Export Fixture", "Fixture exportieren"))
            // 📝️ Staged argument forms for the panel-visible create/switch actions.
            .action_args("addQuestion", vec![
                ActionArgDef::select(
                    "kind",
                    LocalizedLabel::native("Kind", "Art"),
                    FORM_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
                )
                .default_value("text"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new("default", LocalizedLabel::native("Default", "Standard")),
                    ActionArgOption::new("onboarding", LocalizedLabel::native("Onboarding", "Einführung")),
                    ActionArgOption::new("building-component", LocalizedLabel::native("Building Component", "Baukomponente")),
                ]).default_value("default"),
            ])
            .action_args("setSpecJson", vec![ActionArgDef::text("json", LocalizedLabel::native("Spec JSON", "Spezifikations-JSON"))])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS) — `config_spec()`/`forms_io()`
            // are this same information's single source of truth, reused here rather than duplicated.
            .config(FormsPlayApp::config_spec())
            .io(forms_io()),
    )
    .example("default", LocalizedLabel::native("Contact", "Kontakt"), default_example_json(), "file")
    .example("onboarding", LocalizedLabel::native("Onboarding", "Einführung"), onboarding_example_json(), "user")
    .example("building-component", LocalizedLabel::native("Building Component", "Baukomponente"), forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT, "building")
    .workflow("forms", "Forms", "data")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type FormsApp = VcsArtifactApp<FormsPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn forms_app() -> FormsApp {
        new_app::<FormsPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline, and the
    /// `kind` default declared on `addQuestion` materializes host-side.
    pub fn forms_app_with_registry() -> FormsApp {
        new_app_with_registry::<FormsPlayApp>(create_forms_app)
    }

    pub fn dispatch(app: &mut FormsApp, command: FormsCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut FormsApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 🧩️ A host contribution registering `"buildingComponent"` as an extension question kind rendered
    /// by `forms-module-procedural` — shared by every test exercising the extension-question path.
    pub fn building_component_contributions() -> Vec<ProgramContributionEntry> {
        vec![ProgramContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
                "forms.questionKind",
                json!({
                    "appId": "forms-module-procedural",
                    "questionKind": "buildingComponent",
                    "label": "Building Component",
                    "iconId": "building",
                    "paramsBodyKey": "params",
                    "previewBodyKey": "preview",
                }),
            )),
        }]
    }

    /// 🧩️ A standalone `buildingComponent` question, for tests that exercise `render_extension_question`
    /// directly without going through a full document.
    pub fn building_component_question() -> FormQuestion {
        let mut question = question::question_shell("geometry".into(), "Geometry".into(), "buildingComponent".into());
        question.fixture_slug = Some("hexagonal-mushroom-column".into());
        question.params = Some(crate::artifacts::forms::schema::value_to_dsl(&json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })));
        question
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{building_component_contributions, building_component_question, forms_app, forms_app_with_registry};
    use crate::artifacts::forms::forms_steps;
    use semio_framework_plugin::testkit::meta;

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 28, "every FormsCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id for most rows, except the documented divergences copied VERBATIM from the
    /// pre-migration `forms_protocol::FormsCommand`'s own `#[dsl(key = ..)]` attributes (host-pushed
    /// `setLocale`/`setContributions`, and the shortened `selection`/`try-value`/`try-values`/
    /// `spec-json`/`active-example` keys — preserving these exactly is what makes the wire format
    /// byte-identical across the migration; see TEMPLATE.md §5.1).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = match id {
                "setLocale" => "locale".to_string(),
                "setContributions" => "contributions".to_string(),
                "setSelection" => "selection".to_string(),
                "setTryValue" => "try-value".to_string(),
                "setTryValues" => "try-values".to_string(),
                "setSpecJson" => "spec-json".to_string(),
                "setActiveExample" => "active-example".to_string(),
                _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
            };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<FormsCommand> {
        vec![
            FormsCommand::SetSelection(set_selection::SetSelection { ids: vec!["q1".into()] }),
            FormsCommand::SetTryValue(set_try_value::SetTryValue { key: "q1".into(), value_json: Some("\"Ada\"".into()), option_value: None, vector_index: None, param_key: None }),
            FormsCommand::SetTryValues(set_try_values::SetTryValues { values_json: r#"{"name":"Ada"}"#.into() }),
            FormsCommand::ResetTry(reset_try::ResetTry {}),
            FormsCommand::PreviousStep(previous_step::PreviousStep {}),
            FormsCommand::NextStep(next_step::NextStep {}),
            FormsCommand::Submit(submit::Submit {}),
            FormsCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            FormsCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
            FormsCommand::AddStep(add_step::AddStep {}),
            FormsCommand::PatchStep(patch_step::PatchStep { step_id: "s1".into(), field: "title".into(), value: "Renamed".into() }),
            FormsCommand::RemoveStep(remove_step::RemoveStep { step_id: "s1".into() }),
            FormsCommand::MoveStep(move_step::MoveStep { step_id: "s1".into(), index: 0 }),
            FormsCommand::UpdateForm(update_form::UpdateForm { title: "My Form".into() }),
            FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: Some("s1".into()) }),
            FormsCommand::RemoveQuestion(remove_question::RemoveQuestion { question_id: "q1".into() }),
            FormsCommand::PatchQuestions(patch_questions::PatchQuestions { question_ids: vec!["q1".into(), "q2".into()], field: "required".into(), value_json: "true".into(), param_key: None }),
            FormsCommand::PatchQuestionOptions(patch_question_options::PatchQuestionOptions { question_ids: vec!["q1".into()], option_value: "a".into(), field: "label".into(), value_json: "\"Option A\"".into() }),
            FormsCommand::AddQuestionOption(add_question_option::AddQuestionOption { question_id: "q1".into(), label: "New option".into() }),
            FormsCommand::RemoveQuestionOption(remove_question_option::RemoveQuestionOption { question_id: "q1".into(), option_value: "a".into() }),
            FormsCommand::PatchVectorField(patch_vector_field::PatchVectorField { question_id: "q1".into(), field_key: "x".into(), field: "value".into(), value_json: "1.0".into() }),
            FormsCommand::AddVectorField(add_vector_field::AddVectorField { question_id: "q1".into(), field_key: "w".into() }),
            FormsCommand::RemoveVectorField(remove_vector_field::RemoveVectorField { question_id: "q1".into(), field_key: "w".into() }),
            FormsCommand::MoveQuestion(move_question::MoveQuestion { question_id: "q1".into(), to_step_id: "s2".into(), target_id: Some("q2".into()), position: "before".into(), index: Some(0) }),
            FormsCommand::DropQuestionKind(drop_question_kind::DropQuestionKind { kind: "slider".into(), target_id: "step:s1".into(), drop_position: "inside".into() }),
            FormsCommand::SetSpecJson(set_spec_json::SetSpecJson { json: "{}".into() }),
            FormsCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "default".into() }),
            FormsCommand::ExportFixture(export_fixture::ExportFixture {}),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_forms_app().definition).expect("app definition json");
        for id in [builder::FORMS_PLAY_WINDOW_BLUEPRINT, try_window::FORMS_PLAY_WINDOW_TRY] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(blueprint::FORMS_PLAY_MODE_BLUEPRINT), "mode missing from the manifest");
        for body in [FORMS_PLAY_BODY_DOCUMENT, FORMS_PLAY_BODY_CATALOGUE, FORMS_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("form.dictionary"), "artifact kind missing from the manifest");
    }

    #[test]
    fn app_has_blueprint_and_try_windows_only() {
        let app = create_forms_app();
        assert_eq!(app.definition.window_kinds.len(), 2);
        assert_eq!(app.definition.window_kinds[0].id, builder::FORMS_PLAY_WINDOW_BLUEPRINT);
        assert_eq!(app.definition.window_kinds[1].id, try_window::FORMS_PLAY_WINDOW_TRY);
        assert_eq!(app.definition.modes[0].id, blueprint::FORMS_PLAY_MODE_BLUEPRINT);
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn add_question_materializes_kind_default() {
        let mut app = forms_app_with_registry();
        let steps_before = forms_steps(&app.snapshot().expect("projection")).len();
        assert!(steps_before > 0, "seeded fixture has at least one step to receive the question");
        app.dispatch_typed(FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: None }), &meta("local")).expect("add question");
        let spec = app.snapshot().expect("projection");
        assert!(crate::artifacts::forms::schema::flatten_questions(&spec).iter().any(|(_, question)| question.kind == "text"), "kind default materialized from the registry");
    }

    #[test]
    fn initial_document_seeds_building_component_fixture() {
        let app = forms_app();
        let spec = app.snapshot().expect("projection");
        assert!(!crate::artifacts::forms::schema::flatten_questions(&spec).is_empty());
        assert!(crate::artifacts::forms::schema::flatten_questions(&spec).iter().any(|(_, question)| question.kind == "buildingComponent"));
    }

    #[test]
    fn extension_question_falls_back_without_contribution() {
        let node = render_extension_question(&building_component_question(), &Map::new(), &[], "try", true);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Extension unavailable"));
    }

    #[test]
    fn extension_question_emits_external_slot_when_contribution_registered() {
        let node = render_extension_question(&building_component_question(), &Map::new(), &building_component_contributions(), "try", true);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("externalSlot"));
        assert!(json.contains("forms-module-procedural"));
    }

    /// 🗂️ The open `forms.questionKind` topic shape must resolve the extension question.
    #[test]
    fn extension_question_emits_external_slot_when_topic_contribution_registered() {
        let topic_only = vec![ProgramContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
                "forms.questionKind",
                json!({
                    "appId": "forms-module-procedural",
                    "questionKind": "buildingComponent",
                    "label": "Building Component",
                    "iconId": "building",
                    "paramsBodyKey": "params",
                    "previewBodyKey": "preview",
                }),
            )),
        }];
        let node = render_extension_question(&building_component_question(), &Map::new(), &topic_only, "try", true);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("externalSlot"));
        assert!(json.contains("forms-module-procedural"));
    }

    /// 🗂️ `catalogue_kinds` must surface topic-contributed kinds.
    #[test]
    fn catalogue_kinds_includes_topic_contributed_kinds() {
        let contributions = vec![ProgramContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
                "forms.questionKind",
                json!({
                    "appId": "forms-module-procedural",
                    "questionKind": "buildingComponent",
                    "label": "Building Component",
                    "iconId": "building",
                    "paramsBodyKey": "params",
                    "previewBodyKey": "preview",
                }),
            )),
        }];
        let labels = forms_play_labels(&FormsConfig::default());
        let kinds = catalogue_kinds(&contributions, labels);
        assert!(kinds.iter().any(|(kind, label, _)| kind == "buildingComponent" && label == "Building Component"));
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::apps::forms::testkit::render;
        let mut app = forms_app();
        assert!(render(&mut app, "forms.play.nope").contains("Unknown body"));
    }

    #[test]
    fn two_instances_converge_disjoint_edits() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<FormsPlayApp, (usize, usize)>("mem://forms-convergence", FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: None }), FormsCommand::AddStep(add_step::AddStep {}), |app| {
            let projection = app.snapshot().expect("materialize projection");
            let steps = forms_steps(&projection);
            (steps.len(), steps[0].blocks.len())
        });
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    #[test]
    fn export_media_dictionary_out_returns_default_values() {
        let app = forms_app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = <FormsPlayApp as ArtifactApp>::export_media("dictionary:out", &doc).expect("export dictionary:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "form.dictionary");
        let parsed: Value = serde_json::from_str(&json).expect("valid json dictionary");
        assert!(parsed.is_object());
    }

    #[test]
    fn export_media_document_out_round_trips_through_pack() {
        let app = forms_app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = <FormsPlayApp as ArtifactApp>::export_media("document:out", &doc).expect("export document:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, FORMS_DOCUMENT_SCHEMA);
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
        let decoded = <FormsSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
        assert_eq!(decoded, document);
    }

    #[test]
    fn forms_io_exposes_dictionary_out_port() {
        let io = FormsPlayApp::io().expect("forms declares io");
        assert!(io.ports.iter().any(|port| port.id == "dictionary:out"));
    }

    /// 🔌️ Relocated from the deleted artifact `⚙️engine`'s own `forms_io()` unit test (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — asserts the full port shape, not just
    /// presence, alongside `forms_io_exposes_dictionary_out_port` above.
    #[test]
    fn forms_io_declares_dictionary_out_port() {
        let io = forms_io();
        assert_eq!(io.document_schema, FORMS_DOCUMENT_SCHEMA);
        let dictionary_out = io.ports.iter().find(|port| port.id == "dictionary:out").expect("dictionary:out declared");
        assert_eq!(dictionary_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(dictionary_out.kind_id.as_deref(), Some("form.dictionary"));
        assert_eq!(dictionary_out.multiplicity, semio_framework::PortMultiplicity::Many);
        let all_ports = io.all_ports();
        assert!(all_ports.iter().any(|port| port.id == "document:in"));
        assert!(all_ports.iter().any(|port| port.id == "document:out"));
    }
    //#endregion 🔖️MediaPorts
}
//#endregion 🧪️Tests
