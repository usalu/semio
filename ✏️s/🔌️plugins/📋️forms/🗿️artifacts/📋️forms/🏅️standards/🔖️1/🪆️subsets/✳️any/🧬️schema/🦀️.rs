//! 🧬️ Forms artifact schema — every field of the artifact with its state class.

use crate::op::FormMutation;
// 🧷️ Aliased (not the bare `dsl` name): this file also needs the EXTERN `dsl` crate (kernel DSL
// value/derive surface) for `value_to_dsl`/`dsl_to_value` below — importing the artifact's own `dsl`
// submodule under the bare name would shadow that crate and break every `semio_framework_value::DslValue`/`semio_framework_value::ToValue::to_value`
// reference in this file (confirmed by `cargo check`: E0425/E0433 "not found in `dsl`").
use crate::standards::v1::subsets::any::io::text::snapshot as forms_dsl;
use crate::{forms_snapshot_with_state, forms_steps, FormsResultsChild, FormsSnapshot, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use semio_framework_pack_json::{Object, Value};
use framework_schema::ArtifactSchema;

#[path = "📝️definition/🦀️.rs"]
pub mod definition;
#[path = "📨️response/🦀️.rs"]
pub mod response;
#[path = "✅️validation/🦀️.rs"]
pub mod validation;
#[path = "🧾️dictionary/🦀️.rs"]
pub mod dictionary;
pub use validation::{can_advance, step_errors};

//#region 🔖️Artifact
/// 🧬️ Form domain state and its derived composition slots.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, ArtifactSchema)]
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
    pub definition: definition::FormsDefinition,
    #[state(artifact)]
    pub responses: Vec<response::FormsResponse>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub results: FormsResultsChild,
}

impl semio_framework_value::FromValue for FormsArtifact {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <crate::FormsSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_snapshot)
    }
}

//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for FormsArtifact {
    fn default() -> Self {
        let empty = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[]);
        Self { schema: empty.schema, id: empty.id, version: empty.version, title: empty.title, definition: empty.definition, responses: empty.responses, structure: empty.structure, results: empty.results }
    }
}

impl FormsArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> FormsSnapshot {
        FormsSnapshot { schema: self.schema.clone(), id: self.id.clone(), version: self.version.clone(), title: self.title.clone(), definition: self.definition.clone(), responses: self.responses.clone(), structure: self.structure.clone(), results: self.results.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: FormsSnapshot) -> Self {
        Self { schema: snapshot.schema, id: snapshot.id, version: snapshot.version, title: snapshot.title, definition: snapshot.definition, responses: snapshot.responses, structure: snapshot.structure, results: snapshot.results }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: FormsSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.version = snapshot.version;
        self.title = snapshot.title;
        self.definition = snapshot.definition;
        self.responses = snapshot.responses;
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
    default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location, flatten_playbook_blocks as flatten_form_questions, is_block_visible as is_question_visible,
    is_extension_block_kind as is_extension_question_kind, visible_blocks as visible_questions,
};

pub fn initial_try_values(spec: &FormsSnapshot, overrides: &Object) -> Object {
    let overrides_map: crate::playbook::PlaybookValues = overrides.iter().map(|(key, value)| (key.to_string(), semio_framework_pack_json::to_dsl_value(value))).collect();
    let result = crate::playbook::initial_values(&crate::mutations::as_playbook_spec(spec), &overrides_map);
    result.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect()
}
/// 🧾️ Configured defaults retain the actual value owners and declared question order.
pub fn configured_dictionary(spec:&FormsSnapshot)->Result<dictionary::FormDictionary,semio_framework_value::ValueError>{let spec=crate::mutations::as_playbook_spec(spec);let mut entries=Vec::new();for step in &spec.steps{for block in &step.blocks{entries.push(dictionary::FormDictionaryEntry{question_id:block.id.clone(),value:crate::playbook::default_value_for_block(block)});}}let owner=semio_framework_value::DecodedValue::new(dictionary::FormDictionary{entries},<dictionary::FormDictionary as semio_framework_value::FromValue>::retire_decoded);owner.get().validate()?;Ok(owner.take())}
//#endregion 🔖️PlaybookVocabulary

//#region 🔖️DocumentHelpers
/// 🌱️ The forms app's empty document — a single "Inputs" step with no blocks yet.
pub fn empty_forms_snapshot() -> FormsSnapshot {
    forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, &[FormStep { id: "s".into(), title: "Inputs".into(), description: None, blocks: Vec::new() }])
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

/// 🎛️ The leaves that carry `before` to `after` inside `step_id` — the single seam every inspector/command question edit
/// flows through (design §17.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): one ABSOLUTE `change-block-field` per
/// field that differs, so history edits exactly that value; a changed id or kind (a re-seeded question) is the one
/// whole-question `replace-block`. Nothing when the edit changes nothing.
pub fn question_edit_mutations(step_id: &str, before: &FormQuestion, after: &FormQuestion) -> Vec<FormMutation> {
    use crate::mutations::change_block_field::mutation::{BlockField, ChangeBlockField};
    if before.id != after.id || before.kind != after.kind {
        return vec![FormMutation::ReplaceBlock(crate::mutations::replace_block::mutation::ReplaceBlock { step_id: step_id.to_string(), block: after.clone() })];
    }
    BlockField::changes(before, after).into_iter().map(|change| FormMutation::ChangeBlockField(ChangeBlockField { block_id: after.id.clone(), change })).collect()
}

/// ✏️ Locates `question_id` in `spec`, applies `mutate` to a clone, and returns the [`question_edit_mutations`] that record
/// the edit. Returns `None` if the question no longer exists.
pub fn update_block_operations(spec: &FormsSnapshot, question_id: &str, mutate: impl FnOnce(&mut FormQuestion)) -> Option<Vec<FormMutation>> {
    let location = locate_question(spec, question_id)?;
    let mut question = location.question.clone();
    mutate(&mut question);
    Some(question_edit_mutations(&location.step_id, &location.question, &question))
}
//#endregion 🔖️QuestionLocation

//#region 🔖️Ids
/// 🆔️ A platform-entropy identity for a newly created step/question/option — shared by every command that
/// creates one (`addStep`, `addQuestion`, `dropQuestionKind`, `addQuestionOption`).
pub fn create_form_id(prefix: &str) -> String {
    format!("{prefix}-{}", dsl::os_identity::time_ordered_id())
}

/// 🌳️ The document-tree node id for a step — shared by the document panel (tree item ids) and the
/// question drag/drop commands (resolving a drop target back to its owning step).
pub fn forms_play_step_tree_id(step_id: &str) -> String {
    format!("step:{step_id}")
}
//#endregion 🔖️Ids

//#region 🔖️Values
/// 🔄️ Converts a `dsl::os_pack::json::Value` to a `semio_framework_value::DslValue` — first-party, infallible.
pub fn value_to_dsl(value: &Value) -> semio_framework_value::DslValue {
    semio_framework_pack_json::to_dsl_value(value)
}

/// 🔄️ Converts a `semio_framework_value::DslValue` back to a `dsl::os_pack::json::Value` — first-party, infallible.
pub fn dsl_to_value(value: &semio_framework_value::DslValue) -> Value {
    semio_framework_pack_json::from_dsl_value(value)
}

/// 🔤️ A `semio_framework_value::DslValue` rendered as a display string — the inspector's text-field representation of a
/// question's typed default.
pub fn dsl_string_value(value: &semio_framework_value::DslValue) -> String {
    json_string_value(&dsl_to_value(value))
}

/// 🔢️ A `semio_framework_value::DslValue` rendered as `f64` — the inspector's numeric-field representation of a question's
/// typed default.
pub fn dsl_f64_value(value: &semio_framework_value::DslValue) -> f64 {
    json_f64_value(&dsl_to_value(value))
}

/// 🔤️ A `dsl::os_pack::json::Value` rendered as a display string — shared by the inspector's editable fields and
/// the try wizard's current-answer rendering.
pub fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(semio_framework_pack_json::Number::UInt(v)) => v.to_string(),
        Value::Number(semio_framework_pack_json::Number::Int(v)) => v.to_string(),
        Value::Number(semio_framework_pack_json::Number::Float(v)) => v.to_string(),
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
pub fn forms_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.forms.forms",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
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

#[cfg(test)]
#[path = "🧪️tests/🪪️document/🦀️.rs"]
mod document_contract_tests;
