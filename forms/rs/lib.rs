//! 📋 Forms document domain + typed VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormStep {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub questions: Vec<FormQuestion>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormQuestion {
    pub id: String,
    pub label: String,
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<FormQuestionOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FormVectorField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accept: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixture_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<FormExpr>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVectorField {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormQuestionOption {
    #[serde(alias = "id")]
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FormExpr {
    Const { value: serde_json::Value },
    Var { name: String },
    Eq { left: Box<FormExpr>, right: Box<FormExpr> },
    And { items: Vec<FormExpr> },
    Or { items: Vec<FormExpr> },
    Truthy { expr: Box<FormExpr> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormValidationError {
    pub question_id: String,
    pub message: String,
}

pub const FORM_BUILTIN_KINDS: &[&str] = &[
    "text",
    "longText",
    "number",
    "slider",
    "boolean",
    "single",
    "multi",
    "date",
    "color",
    "vector",
    "note",
    "image",
    "file",
];

pub fn is_extension_question_kind(kind: &str) -> bool {
    !FORM_BUILTIN_KINDS.contains(&kind)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormSpec {
    pub schema: String,
    pub id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub steps: Vec<FormStep>,
}

pub type FormsEnvelope = DocumentVcsEnvelope<FormSpec, FormOp>;
pub type FormsStore = DocumentVcsStore<FormSpec, FormOp>;

pub fn empty_forms_projection() -> FormSpec {
    FormSpec {
        schema: FORMS_DOCUMENT_SCHEMA.into(),
        id: "forms".into(),
        version: "1".into(),
        title: None,
        steps: vec![FormStep {
            id: "s".into(),
            title: "Inputs".into(),
            description: None,
            questions: Vec::new(),
        }],
    }
}
//#endregion 🔖Domain

//#region 🔖Runtime
pub fn flatten_form_questions(spec: &FormSpec) -> Vec<&FormQuestion> {
    spec.steps.iter().flat_map(|step| step.questions.iter()).collect()
}

pub fn find_question_location<'a>(spec: &'a FormSpec, question_id: &str) -> Option<(&'a FormStep, usize, &'a FormQuestion)> {
    for step in &spec.steps {
        if let Some(index) = step.questions.iter().position(|question| question.id == question_id) {
            return Some((step, index, &step.questions[index]));
        }
    }
    None
}

pub fn eval_form_expr(expr: &FormExpr, values: &serde_json::Map<String, serde_json::Value>) -> serde_json::Value {
    match expr {
        FormExpr::Const { value } => value.clone(),
        FormExpr::Var { name } => values.get(name).cloned().unwrap_or(serde_json::Value::Null),
        FormExpr::Eq { left, right } => {
            serde_json::Value::Bool(eval_form_expr(left, values) == eval_form_expr(right, values))
        }
        FormExpr::And { items } => serde_json::Value::Bool(items.iter().all(|item| eval_form_expr(item, values).as_bool().unwrap_or(false))),
        FormExpr::Or { items } => serde_json::Value::Bool(items.iter().any(|item| eval_form_expr(item, values).as_bool().unwrap_or(false))),
        FormExpr::Truthy { expr } => serde_json::Value::Bool(eval_form_expr(expr, values).as_bool().unwrap_or(false)),
    }
}

pub fn is_question_visible(question: &FormQuestion, values: &serde_json::Map<String, serde_json::Value>) -> bool {
    question
        .condition
        .as_ref()
        .map(|expr| eval_form_expr(expr, values).as_bool().unwrap_or(false))
        .unwrap_or(true)
}

pub fn default_value_for_question(question: &FormQuestion) -> serde_json::Value {
    match question.kind.as_str() {
        "text" | "longText" => question.default.clone().unwrap_or(serde_json::Value::String(String::new())),
        "number" | "slider" => question
            .default
            .clone()
            .or_else(|| question.min.map(|value| serde_json::json!(value)))
            .unwrap_or(serde_json::json!(0)),
        "boolean" => question.default.clone().unwrap_or(serde_json::json!(false)),
        "single" => question
            .default
            .clone()
            .or_else(|| {
                question
                    .options
                    .as_ref()
                    .and_then(|options| options.first())
                    .map(|option| serde_json::Value::String(option.value.clone()))
            })
            .unwrap_or(serde_json::Value::String(String::new())),
        "multi" => question.default.clone().unwrap_or(serde_json::json!([])),
        "date" | "color" => question.default.clone().unwrap_or(serde_json::Value::String(String::new())),
        "vector" => {
            let values: Vec<f64> = question
                .fields
                .as_ref()
                .map(|fields| fields.iter().map(|field| field.value.unwrap_or(0.0)).collect())
                .unwrap_or_default();
            serde_json::json!(values)
        }
        "note" | "image" | "file" => serde_json::Value::Null,
        _ if is_extension_question_kind(&question.kind) => question
            .params
            .clone()
            .filter(|value| value.is_object() && !value.as_object().is_none_or(|obj| obj.is_empty()))
            .unwrap_or_else(|| serde_json::json!({})),
        _ => serde_json::Value::Null,
    }
}

pub fn visible_questions<'a>(step: &'a FormStep, values: &serde_json::Map<String, serde_json::Value>) -> Vec<&'a FormQuestion> {
    step.questions
        .iter()
        .filter(|question| is_question_visible(question, values))
        .collect()
}

pub fn step_errors(step: &FormStep, values: &serde_json::Map<String, serde_json::Value>) -> Vec<FormValidationError> {
    let mut errors = Vec::new();
    for question in visible_questions(step, values) {
        if question.kind == "note" || question.kind == "image" {
            continue;
        }
        if !question.required.unwrap_or(false) {
            continue;
        }
        let value = values.get(&question.id);
        if is_extension_question_kind(&question.kind) {
            let empty = value.is_none_or(|value| {
                !value.is_object() || value.as_object().is_none_or(|obj| obj.is_empty())
            });
            if empty {
                errors.push(FormValidationError {
                    question_id: question.id.clone(),
                    message: format!("{} is required", question.label),
                });
            }
            continue;
        }
        let missing = match value {
            None | Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::String(text)) => text.is_empty(),
            Some(serde_json::Value::Array(items)) => items.is_empty(),
            _ => false,
        };
        if missing {
            errors.push(FormValidationError {
                question_id: question.id.clone(),
                message: format!("{} is required", question.label),
            });
        }
    }
    errors
}

pub fn can_advance(step: &FormStep, values: &serde_json::Map<String, serde_json::Value>) -> bool {
    step_errors(step, values).is_empty()
}

pub fn initial_try_values(spec: &FormSpec, overrides: &serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
    let mut values = serde_json::Map::new();
    for question in flatten_form_questions(spec) {
        values.insert(question.id.clone(), default_value_for_question(question));
    }
    for (key, value) in overrides {
        if values.contains_key(key) {
            values.insert(key.clone(), value.clone());
        }
    }
    values
}
//#endregion 🔖Runtime

//#region 🔖Ops
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum FormOp {
    AddStep {
        step: FormStep,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveStep {
        step_id: String,
    },
    MoveStep {
        step_id: String,
        index: usize,
    },
    AddQuestion {
        step_id: String,
        question: FormQuestion,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveQuestion {
        step_id: String,
        question_id: String,
    },
    MoveQuestion {
        question_id: String,
        from_step_id: String,
        to_step_id: String,
        index: usize,
    },
    UpdateQuestion {
        step_id: String,
        question: FormQuestion,
    },
    UpdateStep {
        step: FormStep,
    },
    UpdateForm {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FormDiff {
    #[default]
    Empty,
    AddStep {
        step: FormStep,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveStep {
        step_id: String,
    },
    MoveStep {
        step_id: String,
        index: usize,
    },
    AddQuestion {
        step_id: String,
        question: FormQuestion,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveQuestion {
        step_id: String,
        question_id: String,
    },
    MoveQuestion {
        question_id: String,
        from_step_id: String,
        to_step_id: String,
        index: usize,
    },
    UpdateQuestion {
        step_id: String,
        question: FormQuestion,
    },
    UpdateStep {
        step: FormStep,
    },
    UpdateForm {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

impl OperationDiff<FormSpec> for FormDiff {
    fn apply(&self, projection: &FormSpec) -> FormSpec {
        let op = match self {
            FormDiff::Empty => return projection.clone(),
            FormDiff::AddStep { step, index } => FormOp::AddStep {
                step: step.clone(),
                index: *index,
            },
            FormDiff::RemoveStep { step_id } => FormOp::RemoveStep {
                step_id: step_id.clone(),
            },
            FormDiff::MoveStep { step_id, index } => FormOp::MoveStep {
                step_id: step_id.clone(),
                index: *index,
            },
            FormDiff::AddQuestion { step_id, question, index } => FormOp::AddQuestion {
                step_id: step_id.clone(),
                question: question.clone(),
                index: *index,
            },
            FormDiff::RemoveQuestion { step_id, question_id } => FormOp::RemoveQuestion {
                step_id: step_id.clone(),
                question_id: question_id.clone(),
            },
            FormDiff::MoveQuestion {
                question_id,
                from_step_id,
                to_step_id,
                index,
            } => FormOp::MoveQuestion {
                question_id: question_id.clone(),
                from_step_id: from_step_id.clone(),
                to_step_id: to_step_id.clone(),
                index: *index,
            },
            FormDiff::UpdateQuestion { step_id, question } => FormOp::UpdateQuestion {
                step_id: step_id.clone(),
                question: question.clone(),
            },
            FormDiff::UpdateStep { step } => FormOp::UpdateStep { step: step.clone() },
            FormDiff::UpdateForm { title } => FormOp::UpdateForm { title: title.clone() },
        };
        apply_form_edit_op(projection, &op)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, FormDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<FormSpec> for FormOp {
    type Diff = FormDiff;

    fn diff(&self, _projection: &FormSpec) -> FormDiff {
        match self {
            FormOp::AddStep { step, index } => FormDiff::AddStep {
                step: step.clone(),
                index: *index,
            },
            FormOp::RemoveStep { step_id } => FormDiff::RemoveStep {
                step_id: step_id.clone(),
            },
            FormOp::MoveStep { step_id, index } => FormDiff::MoveStep {
                step_id: step_id.clone(),
                index: *index,
            },
            FormOp::AddQuestion { step_id, question, index } => FormDiff::AddQuestion {
                step_id: step_id.clone(),
                question: question.clone(),
                index: *index,
            },
            FormOp::RemoveQuestion { step_id, question_id } => FormDiff::RemoveQuestion {
                step_id: step_id.clone(),
                question_id: question_id.clone(),
            },
            FormOp::MoveQuestion {
                question_id,
                from_step_id,
                to_step_id,
                index,
            } => FormDiff::MoveQuestion {
                question_id: question_id.clone(),
                from_step_id: from_step_id.clone(),
                to_step_id: to_step_id.clone(),
                index: *index,
            },
            FormOp::UpdateQuestion { step_id, question } => FormDiff::UpdateQuestion {
                step_id: step_id.clone(),
                question: question.clone(),
            },
            FormOp::UpdateStep { step } => FormDiff::UpdateStep { step: step.clone() },
            FormOp::UpdateForm { title } => FormDiff::UpdateForm { title: title.clone() },
        }
    }

    fn backwards(&self, projection: &FormSpec) -> Vec<Self> {
        match self {
            FormOp::AddStep { step, .. } => vec![FormOp::RemoveStep {
                step_id: step.id.clone(),
            }],
            FormOp::RemoveStep { step_id } => projection
                .steps
                .iter()
                .find(|s| s.id == *step_id)
                .map(|step| vec![FormOp::AddStep {
                    step: step.clone(),
                    index: None,
                }])
                .unwrap_or_default(),
            FormOp::MoveStep { step_id, .. } => projection
                .steps
                .iter()
                .position(|s| s.id == *step_id)
                .map(|index| vec![FormOp::MoveStep {
                    step_id: step_id.clone(),
                    index,
                }])
                .unwrap_or_default(),
            FormOp::AddQuestion { step_id, question, index } => vec![FormOp::RemoveQuestion {
                step_id: step_id.clone(),
                question_id: question.id.clone(),
            }],
            FormOp::RemoveQuestion { step_id, question_id } => {
                for step in &projection.steps {
                    if step.id == *step_id {
                        if let Some(question) = step.questions.iter().find(|q| q.id == *question_id) {
                            return vec![FormOp::AddQuestion {
                                step_id: step_id.clone(),
                                question: question.clone(),
                                index: None,
                            }];
                        }
                    }
                }
                Vec::new()
            }
            FormOp::MoveQuestion {
                question_id,
                from_step_id,
                to_step_id,
                index,
            } => {
                for step in &projection.steps {
                    if step.id == *from_step_id {
                        if let Some(pos) = step.questions.iter().position(|q| q.id == *question_id) {
                            return vec![FormOp::MoveQuestion {
                                question_id: question_id.clone(),
                                from_step_id: to_step_id.clone(),
                                to_step_id: from_step_id.clone(),
                                index: pos,
                            }];
                        }
                    }
                }
                let _ = index;
                Vec::new()
            }
            FormOp::UpdateQuestion { step_id, question } => {
                for step in &projection.steps {
                    if step.id == *step_id {
                        if let Some(prev) = step.questions.iter().find(|q| q.id == question.id) {
                            return vec![FormOp::UpdateQuestion {
                                step_id: step_id.clone(),
                                question: prev.clone(),
                            }];
                        }
                    }
                }
                Vec::new()
            }
            FormOp::UpdateStep { step } => projection
                .steps
                .iter()
                .find(|s| s.id == step.id)
                .map(|prev| vec![FormOp::UpdateStep { step: prev.clone() }])
                .unwrap_or_default(),
            FormOp::UpdateForm { .. } => vec![FormOp::UpdateForm {
                title: projection.title.clone(),
            }],
        }
    }
}

pub fn apply_form_edit_op(spec: &FormSpec, op: &FormOp) -> FormSpec {
    let mut next = spec.clone();
    match op {
        FormOp::AddStep { step, index } => {
            let at = index.unwrap_or(next.steps.len());
            next.steps.insert(at.min(next.steps.len()), step.clone());
        }
        FormOp::RemoveStep { step_id } => {
            next.steps.retain(|step| step.id != *step_id);
        }
        FormOp::MoveStep { step_id, index } => {
            let from = next.steps.iter().position(|step| step.id == *step_id);
            if let Some(from) = from {
                let step = next.steps.remove(from);
                let at = (*index).min(next.steps.len());
                next.steps.insert(at, step);
            }
        }
        FormOp::AddQuestion { step_id, question, index } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    let at = index.unwrap_or(step.questions.len());
                    step.questions.insert(at.min(step.questions.len()), question.clone());
                }
            }
        }
        FormOp::RemoveQuestion { step_id, question_id } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    step.questions.retain(|question| question.id != *question_id);
                }
            }
        }
        FormOp::MoveQuestion {
            question_id,
            from_step_id,
            to_step_id,
            index,
        } => {
            let mut moving: Option<FormQuestion> = None;
            for step in &mut next.steps {
                if step.id == *from_step_id {
                    if let Some(pos) = step.questions.iter().position(|q| q.id == *question_id) {
                        moving = Some(step.questions.remove(pos));
                    }
                }
            }
            if let Some(question) = moving {
                for step in &mut next.steps {
                    if step.id == *to_step_id {
                        let at = (*index).min(step.questions.len());
                        step.questions.insert(at, question.clone());
                    }
                }
            }
        }
        FormOp::UpdateQuestion { step_id, question } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    for entry in &mut step.questions {
                        if entry.id == question.id {
                            *entry = question.clone();
                        }
                    }
                }
            }
        }
        FormOp::UpdateStep { step } => {
            for entry in &mut next.steps {
                if entry.id == step.id {
                    *entry = step.clone();
                }
            }
        }
        FormOp::UpdateForm { title } => {
            next.title = title.clone();
        }
    }
    next
}

//#endregion 🔖Ops

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct FormsDocumentVcs {
        store: RefCell<FormsStore>,
    }

    #[wasm_bindgen]
    impl FormsDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<FormsDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: FormsEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    FormsStore::new(envelope)
                }
                None => FormsStore::new(create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store
                .borrow_mut()
                .dispatch_json(command_json)
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .projection_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store
                .borrow()
                .envelope_json()
                .map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_document_vcs_materializes() {
        let store = FormsStore::new(create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let projection = store.projection().expect("projection");
        assert_eq!(projection.schema, FORMS_DOCUMENT_SCHEMA);
    }

    #[test]
    fn add_step_op_replays() {
        let mut store = FormsStore::new(create_document_vcs_envelope(FORMS_DOCUMENT_SCHEMA, "forms", empty_forms_projection(), None));
        let step = FormStep {
            id: "step-2".into(),
            title: "Review".into(),
            description: None,
            questions: Vec::new(),
        };
        let backwards = store.projection().expect("projection");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![FormOp::AddStep {
                    step: step.clone(),
                    index: None,
                }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 2);
        let _ = backwards;
    }

    #[test]
    fn question_fields_roundtrip() {
        let json = r#"{
            "id":"q1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
        let question: FormQuestion = serde_json::from_str(json).expect("question json");
        assert_eq!(question.min, Some(1.0));
        assert_eq!(question.unit.as_deref(), Some("people"));
        assert!(question.required.unwrap_or(false));
    }

    #[test]
    fn conditional_visibility_filters_questions() {
        let step = FormStep {
            id: "s".into(),
            title: "Step".into(),
            description: None,
            questions: vec![
                FormQuestion {
                    id: "show".into(),
                    label: "Show".into(),
                    kind: "boolean".into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(serde_json::json!(false)),
                    min: None,
                    max: None,
                    step: None,
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                },
                FormQuestion {
                    id: "team-size".into(),
                    label: "Team size".into(),
                    kind: "slider".into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(serde_json::json!(5)),
                    min: Some(1.0),
                    max: Some(50.0),
                    step: Some(1.0),
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: Some(FormExpr::Truthy {
                        expr: Box::new(FormExpr::Var { name: "show".into() }),
                    }),
                },
            ],
        };
        let mut values = serde_json::Map::new();
        values.insert("show".into(), serde_json::json!(false));
        assert_eq!(visible_questions(&step, &values).len(), 1);
        values.insert("show".into(), serde_json::json!(true));
        assert_eq!(visible_questions(&step, &values).len(), 2);
    }
}
//#endregion 🧪Tests
