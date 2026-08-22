//! 📋️ Forms editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
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

use crate::artifacts::forms::op::FormMutation;
use crate::artifacts::forms::{forms_steps, FormQuestion, FormsSnapshot, FORMS_DOCUMENT_SCHEMA, FORM_BUILTIN_KINDS};
use crate::editor::forms::commands::{
    add_question, add_question_option, add_step, add_vector_field, drop_question_kind, export_fixture, move_question, move_step, next_step, patch_question_options, patch_questions, patch_step, patch_vector_field, previous_step, remove_question,
    remove_question_option, remove_step, remove_vector_field, reset_try, set_active_example, set_contributions, set_locale, set_spec_json, set_try_value, set_try_values, submit, update_form,
};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::modes::blueprint;
use crate::editor::forms::modes::blueprint::windows::{builder, try_wizard as try_window};
use crate::editor::forms::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::forms::presence::{FormsPresence, FormsPresenceMutation};
use crate::editor::forms::terminology::{forms_play_labels, FormsLabels};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppDefinition, ArtifactEditor, ArtifactKindSpec, ArtifactView, CommandDefinition, ConfigView, DomainTopology, DraftView, Editor, Emit, Fault, GranularityDefinition,
    HierarchyProvider, HoverSpec, IconName, InteractionDefinition, InteractionRef, InteractionTopology, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability,
    SelectionMethod, SelectionMode, SelectionSpec, TopologyNode, UiNode,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use store::EngineHandles;

//#region 🔖️Constants
pub const FORMS_PLAY_APP_ID: &str = "forms-play";
pub use builder::FORMS_PLAY_BODY_BLUEPRINT;
pub use catalogue_panel::FORMS_PLAY_BODY_CATALOGUE;
pub use document_panel::FORMS_PLAY_BODY_DOCUMENT;
pub use inspection_panel::FORMS_PLAY_BODY_INSPECTION;
pub use try_window::FORMS_PLAY_BODY_TRY;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🪟️windows/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub async fn forms_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(FORMS_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "fields" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own step/question nesting
/// (steps are the "section" granularity, questions are the "field" granularity), transitive (selecting/
/// hovering a step covers its questions).
pub const FORMS_INTERACTION_FIELDS: &str = "fields";
pub const FORMS_INTERACTION_GRANULARITY_FIELD: &str = "field";
pub const FORMS_INTERACTION_GRANULARITY_SECTION: &str = "section";

/// 🌳️ `fields` domain topology from the document's own step/question nesting — step ids are the SAME
/// row-id-prefixed ids the document panel tree renders (`forms_play_step_tree_id`), question ids are
/// raw question ids (matching both the document panel tree's item ids and every question-editing
/// command's own id vocabulary), so `validate_state` prunes deleted steps/questions and range/
/// transitive selection walk the real document structure.
async fn forms_fields_topology(spec: &FormsSnapshot) -> DomainTopology {
    let mut ordered = Vec::new();
    for step in forms_steps(spec) {
        let step_id = crate::artifacts::forms::schema::forms_play_step_tree_id(&step.id);
        ordered.push(TopologyNode { id: step_id.clone(), granularity: FORMS_INTERACTION_GRANULARITY_SECTION.into(), parent: None });
        for question in step.blocks {
            ordered.push(TopologyNode { id: question.id, granularity: FORMS_INTERACTION_GRANULARITY_FIELD.into(), parent: Some(step_id.clone()) });
        }
    }
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

//#region 🔖️Values
/// 🔠️ `config.try_values_json`'s parsed form — the Try wizard's in-progress answer overrides (question id
/// -> value), heterogeneous per question kind so it stays a JSON blob in `FormsConfig` rather than a
/// typed `dsl` field (see `FormsConfig`'s doc). Falls back to an empty map on malformed JSON rather than
/// erroring, matching every other "best-effort parse of a config blob" call site.
pub async fn try_values_map(config: &FormsConfig) -> Map<String, Value> {
    serde_json::from_str::<Value>(&config.try_values_json).ok().and_then(|value| value.as_object().cloned()).unwrap_or_default()
}

pub async fn try_values_json_text(values: &Map<String, Value>) -> String {
    serde_json::to_string(values).unwrap_or_else(|_| "{}".into())
}

pub async fn effective_try_values(spec: &FormsSnapshot, config: &FormsConfig) -> Map<String, Value> {
    crate::artifacts::forms::schema::initial_try_values(spec, &try_values_map(config))
}

/// 🌱️ Building block for every `handle()` arm that must both clear the Try wizard's answers and reset its
/// active step — was `reset_try_runtime`'s effect on the old `FormsPlayRuntime`, now two config operations
/// instead of two field writes.
pub async fn reset_try_config_mutations() -> Vec<FormsConfigMutation> {
    vec![FormsConfigMutation::SetTryValues { json: "{}".into() }, FormsConfigMutation::SetStepIndex { index: 0 }]
}

/// 🔠️ Parses a command's JSON-blob payload field (`value_json`/`values_json`/…), falling back to
/// `Value::Null` on malformed or absent JSON — every one of these fields is best-effort text carried
/// across the wire, not a validated protocol.
pub async fn parse_value_json(value_json: &str) -> Value {
    serde_json::from_str(value_json).unwrap_or(Value::Null)
}
//#endregion 🔖️Values

//#region 🔖️Contributions
pub use semio_framework::ProgramContributionEntry;

pub async fn forms_parse_contributions(config: &FormsConfig) -> Vec<ProgramContributionEntry> {
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

async fn question_kind_route_from_topic(topic_contribution: &semio_framework_plugin::TopicContribution, kind: &str) -> Option<QuestionKindRoute> {
    if topic_contribution.topic != FORMS_QUESTION_KIND_TOPIC {
        return None;
    }
    let payload = topic_contribution.decode::<FormsQuestionKindTopicPayload>().ok()?;
    (payload.question_kind == kind).then(|| QuestionKindRoute { app_id: payload.app_id, params_body_key: payload.params_body_key, preview_body_key: payload.preview_body_key })
}

/// 🗂️ Reads the open `TopicContribution` (`"forms.questionKind"` topic) shape per entry.
async fn find_question_kind_contribution<'a>(contributions: &'a [ProgramContributionEntry], kind: &str) -> Option<(&'a str, QuestionKindRoute)> {
    contributions.iter().find_map(|entry| {
        let route = entry.topic_contribution.as_ref().and_then(|topic_contribution| question_kind_route_from_topic(topic_contribution, kind))?;
        Some((entry.plugin_id.as_str(), route))
    })
}

async fn extension_params_value(question: &FormQuestion, values: &Map<String, Value>) -> Value {
    values.get(&question.id).cloned().or_else(|| question.params.as_ref().map(crate::artifacts::forms::schema::dsl_to_value)).unwrap_or_else(|| json!({}))
}

async fn extension_render_payload(question: &FormQuestion, params: &Value, surface: &str, interactive: bool) -> String {
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
pub async fn render_extension_question(question: &FormQuestion, values: &Map<String, Value>, contributions: &[ProgramContributionEntry], surface: &str, interactive: bool) -> UiNode {
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
pub async fn catalogue_kinds(contributions: &[ProgramContributionEntry], labels: &FormsLabels) -> Vec<(String, String, IconName)> {
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
pub async fn forms_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: FORMS_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "dictionary:out".into(),
            label: "Dictionary".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
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

impl ArtifactEditor for FormsPlayApp {
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

    const DIALECT: Dialect = crate::artifacts::forms::FORMS_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::forms::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> FormsSnapshot {
        crate::artifacts::forms::schema::building_component_spec()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(forms_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale`/`setContributions` have no manifest
    /// declaration (host-pushed, not user-facing actions).
    async fn command_id(command: &FormsCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(
        command: &FormsCommand,
        doc: &ArtifactView<'_, FormsSnapshot>,
        cfg: &ConfigView<'_, FormsConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<FormMutation, FormsConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ `fields` domain: `HierarchyProvider::Topology` from the document's own step/question nesting —
    /// see `forms_fields_topology`'s doc comment.
    async fn interaction_topology(doc: &ArtifactView<'_, FormsSnapshot>, _cfg: &ConfigView<'_, FormsConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(FORMS_INTERACTION_FIELDS.to_string(), forms_fields_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    //#region 🔖️Media
    /// 🎞️ WORKFLOWS-END-TO-END-TYPED-PORTS port recipe: `document:out` replicates the trait default
    /// exactly (overriding `export_media` for `dictionary:out` forfeits the default's dispatch);
    /// `dictionary:out` re-exports the form's currently-configured default field values as a
    /// `form.dictionary` JSON object keyed by question id — no `cfg` parameter reaches this method, so
    /// this is the form's authored defaults, not a live in-progress Try-wizard session (that lives in
    /// `Self::Config`).
    async fn export_media(port: &str, doc: &ArtifactView<'_, FormsSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "document:out" => {
                let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
                Ok(semio_framework_plugin::Media {
                    media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                    payload: MediaPayload::Structured { schema: FORMS_DOCUMENT_SCHEMA.into(), json: store::pack_rt::pack_value_to_base64(&bytes) },
                })
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

    async fn render(body_key: &str, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> UiNode {
        let spec = doc.snapshot;
        let config = cfg.snapshot;
        let labels = forms_play_labels(config);
        match body_key {
            FORMS_PLAY_BODY_BLUEPRINT => builder::render(spec, config, labels),
            FORMS_PLAY_BODY_TRY => try_window::render(spec, config, labels),
            FORMS_PLAY_BODY_DOCUMENT => document_panel::render(spec, labels),
            FORMS_PLAY_BODY_CATALOGUE => catalogue_panel::render(config, labels),
            FORMS_PLAY_BODY_INSPECTION => inspection_panel::render(spec),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️FormsPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
///
/// 🚧️ SDK GAP (contract §2.4, w0-f-report Gap 4): `EditorBuilder` has no `.example(...)`/`.workflow(...)`
/// methods — the three example registrations (`default`/`onboarding`/`building-component`) and the
/// `"forms"` workflow tag the pre-migration `App`-based manifest carried are dropped here, not ported.
/// The subset's own `📚️examples/🎬️demo` facet is the likely intended replacement mechanism; flagged
/// for the coordinator, not fixed locally.
pub fn create_forms_app() -> AppDefinition {
    Editor::builder(crate::artifacts::forms::FORMS_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
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
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog("setSpecJson", LocalizedLabel::native("Set Spec JSON", "Spezifikations-JSON festlegen"), ActionKind::Mutation) })
            .view_action("setTryValue", LocalizedLabel::native("Set Try Value", "Testwert festlegen"))
            .action_interactive_job("setTryValue", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
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
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "fields" interaction
            // domain — two granularities ("field" default, "section"), `HierarchyProvider::Topology`
            // from the document's own step/question nesting (`forms_fields_topology`/
            // `FormsPlayApp::interaction_topology`), both hover and selection transitive (selecting/
            // hovering a step covers its questions). Selection is pick-only (no canvas marquee surface
            // exists for this domain today); the framework auto-injects interactionSelect/
            // interactionHover/clearSelection/selectAll/setSelectionMode/setInteractionGranularity.
            .interaction(InteractionDefinition {
                id: FORMS_INTERACTION_FIELDS.into(),
                label: LocalizedLabel::native("Fields", "Felder"),
                granularities: vec![
                    GranularityDefinition { id: FORMS_INTERACTION_GRANULARITY_FIELD.into(), label: LocalizedLabel::native("Field", "Feld"), icon_id: "help-circle".into() },
                    GranularityDefinition { id: FORMS_INTERACTION_GRANULARITY_SECTION.into(), label: LocalizedLabel::native("Section", "Abschnitt"), icon_id: "list-tree".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: true,
                    broadcast: true,
                },
            })
            .window_kind_interactions(builder::FORMS_PLAY_WINDOW_BLUEPRINT, vec![InteractionRef::new(FORMS_INTERACTION_FIELDS)])
            // 🎯️ Typed channel surface (WORKFLOWS-END-TO-END-TYPED-PORTS) — `config_spec()`/`forms_io()`
            // are this same information's single source of truth, reused here rather than duplicated.
            .config(FormsPlayApp::config_spec())
            .io(forms_io())
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type FormsApp = VcsArtifactApp<EditorApp<FormsPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn forms_app() -> FormsApp {
        new_app::<EditorApp<FormsPlayApp>>()
    }

    /// 🚧️ SDK GAP (w0-f-report Gap 3): `new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
    /// still take `fn() -> App` (the pre-migration manifest wrapper), unchanged for this ticket —
    /// `create_forms_app` now returns `AppDefinition`, so wrap it in a throwaway `App` (empty examples)
    /// rather than widen the framework testkit signature.
    async fn forms_manifest_for_testkit() -> App {
        App { definition: create_forms_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline, and the
    /// `kind` default declared on `addQuestion` materializes host-side.
    pub async fn forms_app_with_registry() -> FormsApp {
        new_app_with_registry::<EditorApp<FormsPlayApp>>(forms_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut FormsApp, command: FormsCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut FormsApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 🧩️ A host contribution registering `"buildingComponent"` as an extension question kind rendered
    /// by `forms-module-procedural` — shared by every test exercising the extension-question path.
    pub async fn building_component_contributions() -> Vec<ProgramContributionEntry> {
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
    pub async fn building_component_question() -> FormQuestion {
        let mut question = crate::editor::forms::commands::add_question::question_shell("geometry".into(), "Geometry".into(), "buildingComponent".into());
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
    use crate::artifacts::forms::forms_steps;
    use crate::editor::forms::testkit::{building_component_contributions, building_component_question, forms_app, forms_app_with_registry};
    use semio_framework_plugin::testkit::meta;

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 27, "every FormsCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id for most rows, except the documented divergences copied VERBATIM from the
    /// pre-migration `forms_protocol::FormsCommand`'s own `#[dsl(key = ..)]` attributes (host-pushed
    /// `setLocale`/`setContributions`, and the shortened `try-value`/`try-values`/
    /// `spec-json`/`active-example` keys — preserving these exactly is what makes the wire format
    /// byte-identical across the migration; see TEMPLATE.md §5.1).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = match id {
                "setLocale" => "locale".to_string(),
                "setContributions" => "contributions".to_string(),
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
    pub(super) async fn every_command() -> Vec<FormsCommand> {
        vec![
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
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_forms_app()).expect("app definition json");
        for id in [builder::FORMS_PLAY_WINDOW_BLUEPRINT, try_window::FORMS_PLAY_WINDOW_TRY] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(blueprint::FORMS_PLAY_MODE_BLUEPRINT), "mode missing from the manifest");
        for body in [FORMS_PLAY_BODY_DOCUMENT, FORMS_PLAY_BODY_CATALOGUE, FORMS_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("form.dictionary"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn app_has_blueprint_and_try_windows_only() {
        let definition = create_forms_app();
        assert_eq!(definition.window_kinds.len(), 2);
        assert_eq!(definition.window_kinds[0].id, builder::FORMS_PLAY_WINDOW_BLUEPRINT);
        assert_eq!(definition.window_kinds[1].id, try_window::FORMS_PLAY_WINDOW_TRY);
        assert_eq!(definition.modes[0].id, blueprint::FORMS_PLAY_MODE_BLUEPRINT);
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️Interaction
    /// 🕹️ The `fields` domain is declared `HierarchyProvider::Topology`, transitive on both hover and
    /// selection, and scoped to the blueprint (builder) window kind.
    #[semio_framework_async_macros::async_test]
    async fn fields_interaction_domain_is_declared_topology_and_transitive_on_the_blueprint_window() {
        let definition = create_forms_app();
        let fields = definition.interactions.iter().find(|interaction| interaction.id == FORMS_INTERACTION_FIELDS).expect("fields interaction domain declared");
        assert!(matches!(fields.hierarchy, HierarchyProvider::Topology));
        assert!(fields.hover.transitive, "fields hover must be transitive so a hovered step covers its questions");
        assert!(fields.selection.transitive, "fields selection must be transitive so a selected step covers its questions");
        let builder_window = definition.window_kinds.iter().find(|window| window.id == builder::FORMS_PLAY_WINDOW_BLUEPRINT).expect("blueprint window kind declared");
        assert!(builder_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == FORMS_INTERACTION_FIELDS), "blueprint window must reference the fields interaction domain");
    }

    /// 🌳️ `interaction_topology` walks the document's own step/question nesting into `TopologyNode.parent`
    /// links — a step has no parent, every question's parent is its owning step's row id.
    #[semio_framework_async_macros::async_test]
    async fn interaction_topology_walks_step_nesting_into_parent_links() {
        let document = crate::artifacts::forms::schema::building_component_spec();
        let config = FormsConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let topology = FormsPlayApp::interaction_topology(&doc, &cfg);
        let fields = topology.domains.get(FORMS_INTERACTION_FIELDS).expect("fields domain present in topology");
        let steps = forms_steps(&document);
        let question_count: usize = steps.iter().map(|step| step.blocks.len()).sum();
        assert!(!steps.is_empty() && question_count > 0, "the building-component fixture must have steps and questions to make this assertion meaningful");
        assert_eq!(fields.ordered.len(), steps.len() + question_count, "topology must cover every step and every question");
    }

    /// 🌱️ A document with a step but no questions still contributes its (parent-less) section node —
    /// only the field-granularity nodes are absent.
    #[semio_framework_async_macros::async_test]
    async fn interaction_topology_has_a_section_node_and_no_field_nodes_for_a_document_with_no_questions() {
        let document = crate::artifacts::forms::schema::empty_forms_snapshot();
        let config = FormsConfig::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let cfg = ConfigView { snapshot: &config };
        let topology = FormsPlayApp::interaction_topology(&doc, &cfg);
        let fields = topology.domains.get(FORMS_INTERACTION_FIELDS).expect("fields domain present in topology");
        assert!(!fields.ordered.is_empty(), "the empty document's own single step still contributes a section node");
        assert!(fields.ordered.iter().all(|node| node.granularity == FORMS_INTERACTION_GRANULARITY_SECTION), "an empty document has sections but no fields");
    }
    //#endregion 🔖️Interaction

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn add_question_materializes_kind_default() {
        let mut app = forms_app_with_registry();
        let steps_before = forms_steps(&app.snapshot().expect("projection")).len();
        assert!(steps_before > 0, "seeded fixture has at least one step to receive the question");
        app.dispatch_typed(FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: None }), &meta("local")).expect("add question");
        let spec = app.snapshot().expect("projection");
        assert!(crate::artifacts::forms::schema::flatten_questions(&spec).iter().any(|(_, question)| question.kind == "text"), "kind default materialized from the registry");
    }

    #[semio_framework_async_macros::async_test]
    async fn initial_document_seeds_building_component_fixture() {
        let app = forms_app();
        let spec = app.snapshot().expect("projection");
        assert!(!crate::artifacts::forms::schema::flatten_questions(&spec).is_empty());
        assert!(crate::artifacts::forms::schema::flatten_questions(&spec).iter().any(|(_, question)| question.kind == "buildingComponent"));
    }

    #[semio_framework_async_macros::async_test]
    async fn extension_question_falls_back_without_contribution() {
        let node = render_extension_question(&building_component_question(), &Map::new(), &[], "try", true);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Extension unavailable"));
    }

    #[semio_framework_async_macros::async_test]
    async fn extension_question_emits_external_slot_when_contribution_registered() {
        let node = render_extension_question(&building_component_question(), &Map::new(), &building_component_contributions(), "try", true);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("externalSlot"));
        assert!(json.contains("forms-module-procedural"));
    }

    /// 🗂️ The open `forms.questionKind` topic shape must resolve the extension question.
    #[semio_framework_async_macros::async_test]
    async fn extension_question_emits_external_slot_when_topic_contribution_registered() {
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
    #[semio_framework_async_macros::async_test]
    async fn catalogue_kinds_includes_topic_contributed_kinds() {
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

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::forms::testkit::render;
        let mut app = forms_app();
        assert!(render(&mut app, "forms.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_edits() {
        semio_framework_plugin::testkit::assert_two_instances_converge::<semio_framework_plugin::EditorApp<FormsPlayApp>, (usize, usize)>(
            "mem://forms-convergence",
            FormsCommand::AddQuestion(add_question::AddQuestion { kind: "text".into(), step_id: None }),
            FormsCommand::AddStep(add_step::AddStep {}),
            |app| {
                let projection = app.snapshot().expect("materialize projection");
                let steps = forms_steps(&projection);
                (steps.len(), steps[0].blocks.len())
            },
        );
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    #[semio_framework_async_macros::async_test]
    async fn export_media_dictionary_out_returns_default_values() {
        let app = forms_app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = semio_framework_plugin::resolve_ready(<FormsPlayApp as ArtifactEditor>::export_media("dictionary:out", &doc)).expect("export dictionary:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "form.dictionary");
        let parsed: Value = serde_json::from_str(&json).expect("valid json dictionary");
        assert!(parsed.is_object());
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_document_out_round_trips_through_pack() {
        let app = forms_app();
        let document = app.snapshot().expect("projection");
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = semio_framework_plugin::resolve_ready(<FormsPlayApp as ArtifactEditor>::export_media("document:out", &doc)).expect("export document:out");
        let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
        assert_eq!(schema, FORMS_DOCUMENT_SCHEMA);
        let bytes = store::pack_rt::pack_value_from_base64(&json).expect("decode base64 pack");
        let decoded = <FormsSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode pack");
        assert_eq!(decoded, document);
    }

    #[semio_framework_async_macros::async_test]
    async fn forms_io_exposes_dictionary_out_port() {
        let io = FormsPlayApp::io().expect("forms declares io");
        assert!(io.ports.iter().any(|port| port.id == "dictionary:out"));
    }

    /// 🔌️ Relocated from the deleted artifact `⚙️engine`'s own `forms_io()` unit test (ticket
    /// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — asserts the full port shape, not just
    /// presence, alongside `forms_io_exposes_dictionary_out_port` above.
    #[semio_framework_async_macros::async_test]
    async fn forms_io_declares_dictionary_out_port() {
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
