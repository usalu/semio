//! 🧬️ Forms artifact schema — every field of the artifact with its state class.

use crate::artifacts::forms::op::FormMutation;
// 🧷️ Aliased (not the bare `dsl` name): this file also needs the EXTERN `dsl` crate (kernel DSL
// value/derive surface) for `value_to_dsl`/`dsl_to_value` below — importing the artifact's own `dsl`
// submodule under the bare name would shadow that crate and break every `dsl::DslValue`/`dsl::to_dsl_value`
// reference in this file (confirmed by `cargo check`: E0425/E0433 "not found in `dsl`").
use crate::artifacts::forms::dsl as forms_dsl;
use crate::artifacts::forms::{forms_snapshot_with_state, forms_steps, FormQuestion, FormStep, FormsResultsChild, FormsSnapshot, FormsStructureChild, FORMS_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️Artifact
/// 🧬️ Full forms artifact state across the artifact, presence and config lanes. Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`forms→C:value,table`): `steps: Vec<FormStep>` is
/// replaced by the same `structure`/`results` composed-child slot pair as `FormsSnapshot` — read
/// through `crate::artifacts::forms::forms_artifact_steps`, never a bare field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.forms.forms")]
pub struct FormsArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.value")]
    pub structure: FormsStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.table")]
    pub results: FormsResultsChild,
    #[state(presence)]
    pub selected_ids: Vec<String>,
    #[state(config)]
    pub current_step_index: u32,
    #[state(config)]
    pub try_values_json: String,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub contributions_json: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for FormsArtifact {
    fn default() -> Self {
        let empty = forms_snapshot_with_state(FORMS_DOCUMENT_SCHEMA.into(), "forms".into(), "1".into(), None, Vec::new());
        Self {
            schema: empty.schema,
            id: empty.id,
            version: empty.version,
            title: empty.title,
            structure: empty.structure,
            results: empty.results,
            selected_ids: Vec::new(),
            current_step_index: 0,
            try_values_json: "{}".into(),
            locale: "en-US".into(),
            contributions_json: "[]".into(),
        }
    }
}

impl FormsArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::forms::FormsSnapshot {
        crate::artifacts::forms::FormsSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            version: self.version.clone(),
            title: self.title.clone(),
            structure: self.structure.clone(),
            results: self.results.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::forms::FormsSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            version: snapshot.version,
            title: snapshot.title,
            structure: snapshot.structure,
            results: snapshot.results,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::forms::FormsSnapshot) {
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
    can_advance, default_value_for_block as default_value_for_question, eval_playbook_expr as eval_form_expr, find_block_location as find_question_location, flatten_playbook_blocks as flatten_form_questions,
    is_block_visible as is_question_visible, is_extension_block_kind as is_extension_question_kind, step_errors, visible_blocks as visible_questions,
};

pub fn initial_try_values(spec: &FormsSnapshot, overrides: &serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
    crate::playbook::initial_values(&crate::artifacts::forms::mutations::as_playbook_spec(spec), overrides)
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
    serde_json::to_string(&default_example_spec()).expect("serialize default example document")
}

/// 📄️ The `onboarding` example, parsed once from `forms_dsl::ONBOARDING_EXAMPLE_TEXT`.
pub fn onboarding_example_spec() -> FormsSnapshot {
    forms_dsl::parse_playbook_example_dsl(forms_dsl::ONBOARDING_EXAMPLE_TEXT).unwrap_or_else(|_| empty_forms_snapshot())
}

/// 📄️ JSON re-serialization of [`onboarding_example_spec`], for the framework-generic call sites that
/// contractually require JSON text (`App::example`'s manifest `document_json`).
pub fn onboarding_example_json() -> String {
    serde_json::to_string(&onboarding_example_spec()).expect("serialize onboarding example document")
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
    Some(FormMutation::ReplaceBlock(crate::artifacts::forms::mutations::replace_block::mutation::ReplaceBlock { step_id: location.step_id, block: question }))
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
/// 🔄️ Converts a `serde_json::Value` to a `dsl::DslValue` — falls back to `Null` on an unsupported shape
/// (never occurs for the plain JSON literals every question default carries).
pub fn value_to_dsl(value: &Value) -> dsl::DslValue {
    dsl::to_dsl_value(value).unwrap_or(dsl::DslValue::Null)
}

/// 🔄️ Converts a `dsl::DslValue` back to a `serde_json::Value`.
pub fn dsl_to_value(value: &dsl::DslValue) -> Value {
    dsl::from_dsl_value(value.clone()).unwrap_or(Value::Null)
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

/// 🔤️ A `serde_json::Value` rendered as a display string — shared by the inspector's editable fields and
/// the try wizard's current-answer rendering.
pub fn json_string_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// 🔢️ A `serde_json::Value` rendered as `f64` (0.0 on a non-numeric shape).
pub fn json_f64_value(value: &Value) -> f64 {
    value.as_f64().unwrap_or(0.0)
}
//#endregion 🔖️Values

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.forms.forms` — twenty handcrafted schema leaves.
pub fn forms_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.forms.forms",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use semio_framework_plugin::ArtifactBuilder;
    use crate::artifacts::forms::{FormsDiff, FormMutation, FormsSnapshot};

    #[derive(Clone, Debug, Default)]
    pub struct FormsBuilderConstruction {
        snapshot: FormsSnapshot,
        diagnostics: Vec<dsl::Diagnostic>,
    }

    impl ArtifactBuilder for FormsBuilderConstruction {
        type Snapshot = FormsSnapshot;
        type Mutation = FormMutation;
        type Diff = FormsDiff;
        fn empty() -> Self { Self { snapshot: FormsSnapshot::default(), diagnostics: Vec::new() } }
        fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self::from_snapshot(<FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }
        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<FormsSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }
        fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot).into_parts().0;
            self.snapshot = crate::artifacts::forms::schema::mutations::apply_form_edit_mutation(&self.snapshot, &mutation);
            (self, diff)
        }
        fn absorb(mut self, diff: Self::Diff) -> Self {
            self.snapshot = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&diff, &self.snapshot);
            self
        }
        fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
            if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use semio_framework_plugin::{ArtifactAnalysis, Dialect, StandardId, SubsetId, IoConfidence, Analysis, AnalyzeSource};
    use crate::artifacts::forms::FormsSnapshot;

    #[derive(Clone, Debug, Default)]
    pub struct FormsParts {
        pub snapshot: Option<FormsSnapshot>,
    }

    pub struct FormsAnalyzerAnalysis;

    impl ArtifactAnalysis for FormsAnalyzerAnalysis {
        type Parts = FormsParts;
        const DIALECT: Dialect = Dialect { artifact_kind: "s.forms", standard: StandardId("1"), subset: SubsetId("*") };

        fn sniff(_source: &AnalyzeSource<'_>) -> IoConfidence {
            IoConfidence::Medium
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let mut parts = FormsParts::default();
            let mut diagnostics = Vec::new();
            let mut confidence = IoConfidence::High;
            for source in sources {
                match source {
                    AnalyzeSource::Text(text) => match <FormsSnapshot as store::ArtifactDsl>::parse_dsl(text) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.text", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                    AnalyzeSource::Binary(bytes) => match <FormsSnapshot as store::ArtifactPack>::decode_pack(bytes) {
                        Ok(snapshot) => parts.snapshot = Some(snapshot),
                        Err(err) => {
                            confidence = IoConfidence::Low;
                            diagnostics.push(dsl::Diagnostic::error("analyze.binary", dsl::TextSpan::at(1, 1), err.to_string()));
                        }
                    },
                }
            }
            Analysis { parts, dialect: Self::DIALECT, confidence, diagnostics }
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn locate_question_finds_a_question_anywhere_in_the_document() {
        let spec = building_component_spec();
        let steps = forms_steps(&spec);
        let first_id = steps[0].blocks[0].id.clone();
        let location = locate_question(&spec, &first_id).expect("question located");
        assert_eq!(location.step_id, steps[0].id);
    }

    #[test]
    fn update_block_operation_returns_none_for_a_missing_question() {
        let spec = empty_forms_snapshot();
        assert!(update_block_operation(&spec, "missing", |question| question.label = "x".into()).is_none());
    }

    #[test]
    fn update_block_operation_patches_the_located_question() {
        let mut spec = building_component_spec();
        let question_id = forms_steps(&spec)[0].blocks[0].id.clone();
        let operation = update_block_operation(&spec, &question_id, |question| question.label = "Renamed".into()).expect("operation");
        spec = apply_form_edit_mutation(&spec, &operation);
        assert_eq!(forms_steps(&spec)[0].blocks[0].label, "Renamed");
    }

    fn apply_form_edit_mutation(spec: &FormsSnapshot, operation: &FormMutation) -> FormsSnapshot {
        crate::artifacts::forms::op::apply_form_edit_mutation(spec, operation)
    }

    #[test]
    fn create_form_id_is_unique_per_call() {
        assert_ne!(create_form_id("q"), create_form_id("q"));
    }

    #[test]
    fn forms_play_step_tree_id_prefixes_with_step() {
        assert_eq!(forms_play_step_tree_id("s1"), "step:s1");
    }

    #[test]
    fn dsl_value_conversions_round_trip_through_json() {
        // 🩹️ A whole-number float (e.g. `6.0`) round-trips through `dsl::DslValue` as the integer-typed
        // `serde_json::Number` (`6`), which does not `==` the float-typed literal despite being numerically
        // equal — a `dsl` value-system characteristic, not something this conversion controls. Use a
        // fractional value here so the round-trip assertion is unambiguous.
        let value = json!({ "height": 6.5 });
        assert_eq!(dsl_to_value(&value_to_dsl(&value)), value);
        assert_eq!(dsl_string_value(&value_to_dsl(&json!("hello"))), "hello");
        assert_eq!(dsl_f64_value(&value_to_dsl(&json!(42.5))), 42.5);
    }

    #[test]
    fn flatten_questions_lists_every_block_across_steps() {
        let spec = onboarding_example_spec();
        assert!(!flatten_questions(&spec).is_empty());
    }

    #[test]
    fn json_value_helpers_stringify_primitives() {
        assert_eq!(json_string_value(&json!("a")), "a");
        assert_eq!(json_string_value(&json!(true)), "true");
        assert_eq!(json_string_value(&Value::Null), "");
        assert_eq!(json_f64_value(&json!(5)), 5.0);
        assert_eq!(json_f64_value(&Value::Null), 0.0);
    }

    #[test]
    fn default_and_onboarding_examples_parse_and_serialize() {
        assert!(!default_example_json().is_empty());
        assert!(!onboarding_example_json().is_empty());
    }
}
//#endregion 🧪️Tests

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec FormsBuilderFacets {
        construction: derived_construction::FormsBuilderConstruction,
        analysis: derived_analysis::FormsAnalyzerAnalysis,
        composition: super::super::io::derived_composition::FormsComposerComposition,
    }
    builder: FormsBuilder,
    analyzer: FormsAnalyzer,
    composer: FormsComposer,
);
//#endregion 🧬️DerivedArtifactFacets
