//! 🧬️ Forms artifact schema — every field of the artifact with its state class.

use crate::op::FormMutation;
// 🧷️ Aliased (not the bare `dsl` name): this file also needs the EXTERN `dsl` crate (kernel DSL
// value/derive surface) for `value_to_dsl`/`dsl_to_value` below — importing the artifact's own `dsl`
// submodule under the bare name would shadow that crate and break every `dsl::DslValue`/`dsl::to_dsl_value`
// reference in this file (confirmed by `cargo check`: E0425/E0433 "not found in `dsl`").
use crate::document_dsl as forms_dsl;
use crate::{forms_snapshot_with_state, forms_steps, FormsResultsChild, FormsSnapshot, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use dsl::os_pack::json::{Object, Value};
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ forms document artifact state. Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): `steps: Vec<FormStep>` is
/// replaced by the same `structure`/`results` composed-child slot pair as `FormsSnapshot` — read
/// through `crate::forms_artifact_steps`, never a bare field.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[value(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: FormsResultsChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for FormsArtifact {
    fn default() -> Self {
        let empty = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, Vec::new());
        Self { schema: empty.schema, id: empty.id, version: empty.version, title: empty.title, structure: empty.structure, results: empty.results }
    }
}

impl FormsArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> FormsSnapshot {
        FormsSnapshot { schema: self.schema.clone(), id: self.id.clone(), version: self.version.clone(), title: self.title.clone(), structure: self.structure.clone(), results: self.results.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: FormsSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, version: snapshot.version, title: snapshot.title, structure: snapshot.structure, results: snapshot.results, ..Self::default() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: FormsSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.version = snapshot.version;
        self.title = snapshot.title;
        self.structure = snapshot.structure;
        self.results = snapshot.results;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️PlaybookVocabulary
/// 🌿️ Pure compute over the shared `playbook` kernel crate's step/block domain, re-exported here under
/// forms' historical names (relocated from the deleted `⚙️engine`, ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub use crate::playbook::{
    can_advance, default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location, flatten_playbook_blocks as flatten_form_questions, is_block_visible as is_question_visible,
    is_extension_block_kind as is_extension_question_kind, step_errors, visible_blocks as visible_questions,
};

pub fn initial_try_values(spec: &FormsSnapshot, overrides: &Object) -> Object {
    let overrides_map: std::collections::HashMap<String, dsl::DslValue> = overrides.iter().map(|(key, value)| (key.to_string(), dsl::os_pack::json::to_dsl_value(value))).collect();
    let result = crate::playbook::initial_values(&crate::mutations::as_playbook_spec(spec), &overrides_map);
    result.into_iter().map(|(key, value)| (key, dsl::os_pack::json::from_dsl_value(&value))).collect()
}
//#endregion 🔖️PlaybookVocabulary

//#region 🔖️DocumentHelpers
/// 🌱️ The forms app's empty document — a single "Inputs" step with no blocks yet.
pub fn empty_forms_snapshot() -> FormsSnapshot {
    forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, vec![FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() }])
}

/// 🌱️ The forms app's default document — the building-component fixture, seeded from its derive-
/// generated `.forms` DSL text.
pub fn building_component_spec() -> FormsSnapshot {
    forms_dsl::parse_playbook_example_dsl(forms_dsl::BUILDING_COMPONENT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_snapshot())
}

/// 📄️ The `default` (Contact) example, parsed once from `forms_dsl::DEFAULT_EXAMPLE_TEXT` — the source of truth
/// for every "default" example call site (`setActiveExample`, `App::example`). Loaded through
/// `parse_playbook_example_dsl` (ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM), not `parse_dsl`
/// — see that function's own doc comment for why.
pub fn default_example_spec() -> FormsSnapshot {
    forms_dsl::parse_playbook_example_dsl(forms_dsl::DEFAULT_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_snapshot())
}

/// 📄️ JSON re-serialization of [`default_example_spec`], for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn default_example_json() -> String {
    dsl::os_pack::json::to_json_string(&default_example_spec())
}

/// 📄️ The `onboarding` example, parsed once from `forms_dsl::ONBOARDING_EXAMPLE_TEXT`.
pub fn onboarding_example_spec() -> FormsSnapshot {
    forms_dsl::parse_playbook_example_dsl(forms_dsl::ONBOARDING_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_snapshot())
}

/// 📄️ JSON re-serialization of [`onboarding_example_spec`], for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn onboarding_example_json() -> String {
    dsl::os_pack::json::to_json_string(&onboarding_example_spec())
}

/// 🔠️ Every `(step title, question)` pair in document order — the empty-inspector diagnostic and every
/// command test's "did the edit land" assertion share this flattening.
pub fn flatten_questions(spec: &FormsSnapshot) -> Vec<(String, FormQuestion)> {
    let mut pairs = Vec::new();
    for step in forms_steps(spec) {
        for question in step.blocks {
            pairs.push((step.title.clone(), question));
        }
    }
    pairs
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️QuestionLocation
pub struct QuestionLocation {
    pub step_id: String,
    pub question: FormQuestion,
}

/// 🔎️ Locates a question by id anywhere in the document — the single lookup every question-editing
/// command (`❓️question`, `🔘️option`, `📐️vector`) and the inspection panel share.
pub fn locate_question(spec: &FormsSnapshot, question_id: &str) -> Option<QuestionLocation> {
    for step in forms_steps(spec) {
        if let Some(question) = step.blocks.into_iter().find(|question| question.id == question_id) {
            return Some(QuestionLocation { step_id: step.id, question });
        }
    }
    None
}

/// ✏️ Locates `question_id` in `spec`, applies `mutate` to a clone, and returns the `replace-block`
/// operation that records the edit — the single seam every inspector/command patch flows through.
/// Returns `None` if the question no longer exists.
pub fn update_block_operation(spec: &FormsSnapshot, question_id: &str, mutate: impl FnOnce(&mut FormQuestion)) -> Option<FormMutation> {
    let location = locate_question(spec, question_id)?;
    let mut question = location.question;
    mutate(&mut question);
    Some(FormMutation::ReplaceBlock(crate::mutations::replace_block::mutation::ReplaceBlock { step_id: location.step_id, block: question }))
}
//#endregion 🔖️QuestionLocation

//#region 🔖️Ids
/// 🆔️ A process-unique id for a newly created step/question/option — shared by every command that
/// creates one (`addStep`, `addQuestion`, `dropQuestionKind`, `addQuestionOption`).
pub fn create_form_id(prefix: &str) -> String {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let next = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    format!("{prefix}-{next}")
}

/// 🌳️ The document-tree node id for a step — shared by the document panel (tree item ids) and the
/// question drag/drop commands (resolving a drop target back to its owning step).
pub fn forms_play_step_tree_id(step_id: &str) -> String {
    format!("step:{step_id}")
}
//#endregion 🔖️Ids

//#region 🔖️Values
/// 🔄️ Converts a `dsl::os_pack::json::Value` to a `dsl::DslValue` — first-party, infallible.
pub fn value_to_dsl(value: &Value) -> dsl::DslValue {
    dsl::os_pack::json::to_dsl_value(value)
}

/// 🔄️ Converts a `dsl::DslValue` back to a `dsl::os_pack::json::Value` — first-party, infallible.
pub fn dsl_to_value(value: &dsl::DslValue) -> Value {
    dsl::os_pack::json::from_dsl_value(value)
}

/// 🔤️ A `dsl::DslValue` rendered as a display string — the inspector's text-field representation of a
/// question's typed default.
pub fn dsl_string_value(value: &dsl::DslValue) -> String {
    json_string_value(&dsl_to_value(value))
}

/// 🔢️ A `dsl::DslValue` rendered as `f64` — the inspector's numeric-field representation of a question's
/// typed default.
pub fn dsl_f64_value(value: &dsl::DslValue) -> f64 {
    json_f64_value(&dsl_to_value(value))
}

/// 🔤️ A `dsl::os_pack::json::Value` rendered as a display string — shared by the inspector's editable fields and
/// the try wizard's current-answer rendering.
pub fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(dsl::os_pack::json::Number::UInt(v)) => v.to_string(),
        Value::Number(dsl::os_pack::json::Number::Int(v)) => v.to_string(),
        Value::Number(dsl::os_pack::json::Number::Float(v)) => v.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// 🔢️ A `dsl::os_pack::json::Value` rendered as `f64` (0.0 on a non-numeric shape).
pub fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}
//#endregion 🔖️Values

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.forms.forms` — twenty handcrafted schema leaves.
pub fn forms_artifact_schema_descriptor() -> framework_schema::ArtifactSchemaDescriptor {
    framework_schema::ArtifactSchemaDescriptor {
        id: "s.forms.forms",
        artifact: framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️Construction
/// 🏗️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM: the old hand-rolled
/// `FormsBuilderConstruction` (`empty`/`from_snapshot`/`from_text`/`from_binary`/`mutate`/`absorb`/
/// `build`) did nothing beyond the ordinary `Mutation`/`MutationDiff` algebra — a trivial subset,
/// per the SDK's own `SnapshotBuilder<S, M>` (W1-C task 3).
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<FormsSnapshot, FormMutation>;
//#endregion 🏗️Construction

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🔁️Re-exports
pub use crate::FormQuestion;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::FormStep;
//#endregion 🔁️Re-exports
