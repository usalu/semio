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
    remove_question_option, remove_step, remove_vector_field, reset_try, set_active_example, set_contributions, set_locale, set_spec_json, set_try_value, set_try_value_step, set_try_values, submit, update_form,
};
use crate::editor::forms::config::{FormsConfig, FormsConfigMutation};
use crate::editor::forms::modes::blueprint;
use crate::editor::forms::modes::blueprint::windows::{builder, try_wizard as try_window};
use crate::editor::forms::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::forms::presence::{FormsPresence, FormsPresenceMutation};
use crate::editor::forms::terminology::{forms_play_labels, FormsLabels};
use dsl::os_pack::json::{object, Object, Value};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppDefinition, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract,
    ArtifactToolPublicationLane, ArtifactView, CommandDefinition, ConfigView, DomainTopology, DraftView, Editor, EditorApp, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, IconName, InteractionDefinition, InteractionRef,
    InteractionTopology, InteractiveJobClassification, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec,
    TopologyNode, UiNode,
};
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
pub fn forms_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(FORMS_PLAY_APP_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}


/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
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
/// 🔠️ Materializes the independently stored answer leaves for rendering and validation.
pub async fn try_values_map(config: &FormsConfig) -> Object {
    config
        .try_values
        .iter_chunks()
        .into_iter()
        .map(|(key, chunks)| {
            let raw = chunks.iter().fold(String::new(), |mut raw, chunk| {
                raw.push_str(&chunk);
                raw
            });
            (key, dsl::os_pack::json::parse(&raw).unwrap_or(Value::Null))
        })
        .collect()
}

pub async fn effective_try_values(spec: &FormsSnapshot, config: &FormsConfig) -> Object {
    crate::artifacts::forms::schema::initial_try_values(spec, &try_values_map(config))
}

/// 🌱️ Building block for every `handle()` arm that must both clear the Try wizard's answers and reset its
/// active step — was `reset_try_runtime`'s effect on the old `FormsPlayRuntime`, now two config operations
/// instead of two field writes.
pub async fn reset_try_config_mutations() -> Vec<FormsConfigMutation> {
    vec![FormsConfigMutation::ClearTryValues, FormsConfigMutation::SetStepIndex { index: 0 }]
}

/// 🔠️ Parses a command's JSON-blob payload field (`value_json`/`values_json`/…), falling back to
/// `Value::Null` on malformed or absent JSON — every one of these fields is best-effort text carried
/// across the wire, not a validated protocol.
pub async fn parse_value_json(value_json: &str) -> Value {
    dsl::os_pack::json::parse(value_json).unwrap_or(Value::Null)
}
//#endregion 🔖️Values

//#region 🔖️Contributions
pub use semio_framework::ProgramContributionEntry;

pub async fn forms_parse_contributions(config: &FormsConfig) -> Vec<ProgramContributionEntry> {
    semio_framework::parse_contributions(&config.contributions_json)
}

pub use forms_parse_contributions as parse_contributions;

/// 🗂️ `forms.questionKind` topic payload shape, decoded from the open `TopicContribution`.
#[derive(Clone, Debug, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
struct FormsQuestionKindTopicPayload {
    app_id: String,
    question_kind: String,
    label: String,
    icon_id: String,
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

async fn extension_params_value(question: &FormQuestion, values: &Object) -> Value {
    values.get(&question.id).cloned().or_else(|| question.params.as_ref().map(crate::artifacts::forms::schema::dsl_to_value)).unwrap_or_else(|| Value::Object(Object::new()))
}

async fn extension_render_payload(question: &FormQuestion, params: &Value, surface: &str, interactive: bool) -> String {
    let payload = object([
        ("fixtureSlug".to_string(), Value::from(question.fixture_slug.clone().unwrap_or_else(|| "hexagonal-mushroom-column".into()))),
        ("params".to_string(), params.clone()),
        ("questionId".to_string(), Value::from(question.id.clone())),
        ("controllerId".to_string(), Value::from(FORMS_PLAY_APP_ID)),
        ("surface".to_string(), Value::from(surface)),
        ("interactive".to_string(), Value::from(interactive)),
    ]);
    dsl::os_pack::json::to_string(&payload)
}

/// 🧩️ Renders a contributed (extension) question kind as a pair of external slots (params editor +
/// preview), or an "Extension unavailable" diagnostic when no contribution is registered for it. Shared
/// by the try wizard and the inspection panel's kind-specific editor fields.
pub async fn render_extension_question(question: &FormQuestion, values: &Object, contributions: &[ProgramContributionEntry], surface: &str, interactive: bool) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
            .map(|payload| (payload.question_kind, payload.label, IconName::from(payload.icon_id.as_str())));
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
        "setTryValueStep" as "try-value-step" => set_try_value_step::SetTryValueStep,
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

//#region 🧵️RetainedCommands
/// 🧾️ Every Forms tool id in `FormsCommand` declaration order, kept exact against the generated
/// command catalog, proof rows, and publication contracts by the exhaustiveness law below.
const FORMS_RETAINED_TOOL_IDS: &[&str] = &[
    "setTryValue",
    "setTryValues",
    "resetTry",
    "previousStep",
    "nextStep",
    "submit",
    "setLocale",
    "setContributions",
    "addStep",
    "patchStep",
    "removeStep",
    "moveStep",
    "updateForm",
    "addQuestion",
    "removeQuestion",
    "patchQuestions",
    "patchQuestionOptions",
    "addQuestionOption",
    "removeQuestionOption",
    "patchVectorField",
    "addVectorField",
    "removeVectorField",
    "moveQuestion",
    "dropQuestionKind",
    "setSpecJson",
    "setActiveExample",
    "exportFixture",
    "setTryValueStep",
];
const FORMS_RETAINED_PAYLOAD_SCHEMA: &str = "forms.tool-command.v1";
const FORMS_RETAINED_RAW_BYTES: usize = 16_384;
const FORMS_STORE_MUTATION_MAXIMUM_BYTES: usize = store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;

fn forms_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(FORMS_RETAINED_RAW_BYTES, 64, 64, 4_096, 7_500)
}

fn forms_bounded_extent(_command: &FormsCommand, _snapshot: &FormsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

fn forms_retained_reduce(
    command: &FormsCommand,
    snapshot: &FormsSnapshot,
    config: &FormsConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<FormMutation, FormsConfigMutation, NoDraftMutation>, Fault> {
    if !FORMS_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return Err(Fault::from("forms-command-retained-route-rejected"));
    }
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    command.dispatch(&doc, &cfg)
}

struct FormsBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl FormsBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: FORMS_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for FormsBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<FormsPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<FormsPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        FORMS_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        forms_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > FORMS_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Forms retained command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for FormsBoundedCommandJobFactory {
    type Owner = EditorApp<FormsPlayApp>;
    const TOOL_IDS: &'static [&'static str] = FORMS_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = FORMS_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "setTryValue", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setTryValues", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "resetTry", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "previousStep", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "nextStep", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "submit", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "patchStep", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "updateForm", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addQuestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "removeQuestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "patchQuestions", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "patchQuestionOptions", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addQuestionOption", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeQuestionOption", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "patchVectorField", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "addVectorField", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "removeVectorField", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "moveQuestion", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "dropQuestionKind", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setSpecJson", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
        ArtifactToolPublicationContract { tool_id: "exportFixture", lanes: &[ArtifactToolPublicationLane::HostOnly] },
        ArtifactToolPublicationContract { tool_id: "setTryValueStep", lanes: &[ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
fn forms_next_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

fn forms_store_mutation_retained_bytes<M: protocol::OpBinary>(mutation: &M) -> Result<usize, String> {
    protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "forms-store-mutation-encode-failed".to_string())
}

fn admit_forms_store_mutation<M: protocol::OpBinary>(mutation: &M) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = forms_store_mutation_retained_bytes(mutation)?;
    if retained_bytes > FORMS_STORE_MUTATION_MAXIMUM_BYTES {
        return Err("forms-store-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_forms_store_mutation<P, M>(base: &P, mutation: M) -> Result<(P, Vec<M>, M), String>
where
    M: protocol::Mutation<P> + protocol::OpBinary,
{
    admit_forms_store_mutation(&mutation)?;
    let inverse = protocol::Mutation::inverse(&mutation, base);
    if inverse.iter().any(|step| admit_forms_store_mutation(step).is_err()) {
        return Err("forms-store-inverse-mutation-envelope".into());
    }
    let diff = protocol::Mutation::diff(&mutation, base).into_parts().0;
    let post = protocol::MutationDiff::apply(&diff, base).map_err(|_| "forms-store-diff-apply-failed".to_string())?;
    Ok((post, inverse, mutation))
}

struct FormsStorePreparationFactory<P, M> {
    prefix: &'static str,
    marker: std::marker::PhantomData<fn() -> (P, M)>,
}

impl<P, M> FormsStorePreparationFactory<P, M> {
    fn new(prefix: &'static str) -> Self {
        Self { prefix, marker: std::marker::PhantomData }
    }
}

struct FormsStorePreparation<P, M> {
    prefix: &'static str,
    base: Option<store::SnapshotRead<P>>,
    mutation: Option<M>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<P, M>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl<P, M> store::ArtifactStoreOneItemPreparationFactory<P, M> for FormsStorePreparationFactory<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::OpBinary + Send + 'static,
{
    fn preflight(&self, mutation: &M, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("forms-store-lane-or-description-envelope".into());
        }
        admit_forms_store_mutation(mutation)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<P, M>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<P, M>>, store::ArtifactStoreOneItemPreparationRequest<P, M>> {
        let retained_bytes = forms_store_mutation_retained_bytes(&request.mutation).unwrap_or(FORMS_STORE_MUTATION_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > FORMS_STORE_MUTATION_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(FormsStorePreparation {
            prefix: self.prefix,
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl<P, M> store::ArtifactStoreOneItemPreparation<P, M> for FormsStorePreparation<P, M>
where
    P: Clone + Send + Sync + 'static,
    M: protocol::Mutation<P> + protocol::OpBinary + Send + 'static,
{
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "forms-store-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "forms-store-mutation-owner-missing".to_string())?;
        let (post, inverse, forward) = prepare_forms_store_mutation(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "forms-store-authority-missing".to_string())?;
        let edit = forms_next_edit(self.prefix, forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("forms-store-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

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

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(FormsStorePreparationFactory::<FormsSnapshot, FormMutation>::new("forms-artifact-retained")))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(FormsStorePreparationFactory::<FormsConfig, FormsConfigMutation>::new("forms-config-retained")))
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(FormsBoundedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !FORMS_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("forms-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work = Box::new(BoundedArtifactCommandWork::new(tool_id, forms_retained_reduce, forms_bounded_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            FormsCommand::command_id,
            FORMS_RETAINED_RAW_BYTES,
            1,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<FormsPlayApp>,
        owner_file: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.forms.forms@1/*#editor",
        document_schema: "forms.form",
        factory: "FormsBoundedCommandJobFactory",
        factory_type: FormsBoundedCommandJobFactory,
        tools: {
            "setTryValue" => forms_bounded_contract(),
            "setTryValues" => forms_bounded_contract(),
            "resetTry" => forms_bounded_contract(),
            "previousStep" => forms_bounded_contract(),
            "nextStep" => forms_bounded_contract(),
            "submit" => forms_bounded_contract(),
            "setLocale" => forms_bounded_contract(),
            "setContributions" => forms_bounded_contract(),
            "addStep" => forms_bounded_contract(),
            "patchStep" => forms_bounded_contract(),
            "removeStep" => forms_bounded_contract(),
            "moveStep" => forms_bounded_contract(),
            "updateForm" => forms_bounded_contract(),
            "addQuestion" => forms_bounded_contract(),
            "removeQuestion" => forms_bounded_contract(),
            "patchQuestions" => forms_bounded_contract(),
            "patchQuestionOptions" => forms_bounded_contract(),
            "addQuestionOption" => forms_bounded_contract(),
            "removeQuestionOption" => forms_bounded_contract(),
            "patchVectorField" => forms_bounded_contract(),
            "addVectorField" => forms_bounded_contract(),
            "removeVectorField" => forms_bounded_contract(),
            "moveQuestion" => forms_bounded_contract(),
            "dropQuestionKind" => forms_bounded_contract(),
            "setSpecJson" => forms_bounded_contract(),
            "setActiveExample" => forms_bounded_contract(),
            "exportFixture" => forms_bounded_contract(),
            "setTryValueStep" => forms_bounded_contract(),
        }
    }

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
                let values = crate::artifacts::forms::schema::initial_try_values(doc.snapshot, &Object::new());
                let json = dsl::os_pack::json::to_string(&Value::Object(values));
                Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "form.dictionary".into(), json } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    async fn render(body_key: &str, doc: &ArtifactView<'_, FormsSnapshot>, cfg: &ConfigView<'_, FormsConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
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
            .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setLocale", LocalizedLabel::native("Set Locale", "Gebietsschema festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("value", LocalizedLabel::native("Locale", "Gebietsschema"))]) })
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
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(set_try_value::SET_TRY_VALUE_STEP_ACTION_ID, LocalizedLabel::native("Set Try Value Step", "Testwert-Schritt festlegen"), ActionKind::View) })
            .action_interactive_job("setTryValue", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job(set_try_value::SET_TRY_VALUE_STEP_ACTION_ID, semio_framework_plugin::InteractiveJobClassification::Migrated)
            .view_action("setTryValues", LocalizedLabel::native("Set Try Values", "Testwerte festlegen"))
            .view_action("resetTry", LocalizedLabel::native("Reset Try", "Test zurücksetzen"))
            .view_action("previousStep", LocalizedLabel::native("Previous Step", "Vorheriger Schritt"))
            .view_action("nextStep", LocalizedLabel::native("Next Step", "Nächster Schritt"))
            .view_action("submit", LocalizedLabel::native("Submit", "Absenden"))
            .shell_action("exportFixture", LocalizedLabel::native("Export Fixture", "Fixture exportieren"))
            .action_interactive_job("setTryValues", InteractiveJobClassification::Migrated)
            .action_interactive_job("resetTry", InteractiveJobClassification::Migrated)
            .action_interactive_job("previousStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("nextStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("submit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", InteractiveJobClassification::Migrated)
            .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
            .action_interactive_job("addStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveStep", InteractiveJobClassification::Migrated)
            .action_interactive_job("updateForm", InteractiveJobClassification::Migrated)
            .action_interactive_job("addQuestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeQuestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchQuestions", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchQuestionOptions", InteractiveJobClassification::Migrated)
            .action_interactive_job("addQuestionOption", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeQuestionOption", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchVectorField", InteractiveJobClassification::Migrated)
            .action_interactive_job("addVectorField", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeVectorField", InteractiveJobClassification::Migrated)
            .action_interactive_job("moveQuestion", InteractiveJobClassification::Migrated)
            .action_interactive_job("dropQuestionKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSpecJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("exportFixture", InteractiveJobClassification::Migrated)
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
        dsl::os_pack::json::to_json_string(&app.render(body_key, None, &ViewModel::default()).expect("render"))
    }

    /// 🧩️ A host contribution registering `"buildingComponent"` as an extension question kind rendered
    /// by `forms-module-procedural` — shared by every test exercising the extension-question path.
    pub async fn building_component_contributions() -> Vec<ProgramContributionEntry> {
        vec![ProgramContributionEntry {
            plugin_id: "forms-module-procedural".into(),
            topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
                "forms.questionKind",
                semio_framework_os_kernel::DslValue::object([
                    ("appId".to_string(), semio_framework_os_kernel::DslValue::String("forms-module-procedural".to_string())),
                    ("questionKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                    ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                    ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
                    ("paramsBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("params".to_string())),
                    ("previewBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("preview".to_string())),
                ]),
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
        assert_eq!(ids.len(), 28, "every FormsCommand row must be covered by every_command()");
    }

    /// ⚖️ Every generated Forms command has one concrete retained-factory key, proof row, and exact
    /// nonempty publication contract in the same declaration order.
    #[test]
    fn retained_route_dispositions_are_exact_and_exhaustive() {
        use semio_framework::{ToolCancellationPolicy, ToolExecutionShape, ToolJobFactory};
        use semio_framework_plugin::ArtifactOwnedToolJobFactory;

        assert_eq!(FormsCommand::TOOL_JOB_IDS, FORMS_RETAINED_TOOL_IDS);
        assert_eq!(<FormsPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), FORMS_RETAINED_TOOL_IDS.len());
        assert_eq!(FormsBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), FORMS_RETAINED_TOOL_IDS.len());
        assert_eq!(forms_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
        assert_eq!(forms_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);

        let factory = FormsBoundedCommandJobFactory::new("s.forms.forms@1/*#editor");
        let factory_ids: Vec<&str> = factory.keys().iter().map(|key| key.tool_id.as_str()).collect();
        assert_eq!(factory_ids, FORMS_RETAINED_TOOL_IDS);
        for (tool_id, contract) in FORMS_RETAINED_TOOL_IDS.iter().zip(FormsBoundedCommandJobFactory::PUBLICATION_CONTRACTS) {
            assert_eq!(*tool_id, contract.tool_id);
            assert!(!contract.lanes.is_empty(), "tool {tool_id} must declare a publication lane");
            assert!(!contract.lanes.contains(&ArtifactToolPublicationLane::HostOnly) || contract.lanes.len() == 1, "HostOnly must be exclusive for tool {tool_id}");
        }
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
            FormsCommand::SetTryValue(set_try_value::SetTryValue { key: "q1".into(), value_json: Some("\"Ada\"".into()), ..Default::default() }),
            FormsCommand::SetTryValues(set_try_values::SetTryValues { values_json: r#"{"name":"Ada"}"#.into(), ..Default::default() }),
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
            FormsCommand::SetTryValueStep(set_try_value_step::SetTryValueStep { app_id: "1".into(), document_id: "document".into(), operation_id: "1".into(), generation: 1, cursor: 64, target_index: 128, base_revision: "0".repeat(64) }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = dsl::os_pack::json::to_json_string(&create_forms_app());
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
        let node = render_extension_question(&building_component_question(), &Object::new(), &[], "try", true);
        let json = dsl::os_pack::json::to_json_string(&node);
        assert!(json.contains("Extension unavailable"));
    }

    #[semio_framework_async_macros::async_test]
    async fn extension_question_emits_external_slot_when_contribution_registered() {
        let node = render_extension_question(&building_component_question(), &Object::new(), &building_component_contributions(), "try", true);
        let json = dsl::os_pack::json::to_json_string(&node);
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
                semio_framework_os_kernel::DslValue::object([
                    ("appId".to_string(), semio_framework_os_kernel::DslValue::String("forms-module-procedural".to_string())),
                    ("questionKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                    ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                    ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
                    ("paramsBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("params".to_string())),
                    ("previewBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("preview".to_string())),
                ]),
            )),
        }];
        let node = render_extension_question(&building_component_question(), &Object::new(), &topic_only, "try", true);
        let json = dsl::os_pack::json::to_json_string(&node);
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
                semio_framework_os_kernel::DslValue::object([
                    ("appId".to_string(), semio_framework_os_kernel::DslValue::String("forms-module-procedural".to_string())),
                    ("questionKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                    ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                    ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
                    ("paramsBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("params".to_string())),
                    ("previewBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("preview".to_string())),
                ]),
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
        let parsed: Value = dsl::os_pack::json::parse(&json).expect("valid json dictionary");
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
