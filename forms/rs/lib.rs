//! 📋 Forms document domain + typed VCS on `vcs`.

use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const FORMS_DOCUMENT_SCHEMA: &str = "forms.form/v1";

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
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<FormQuestionOption>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormQuestionOption {
    pub id: String,
    pub label: String,
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
}
//#endregion 🧪Tests
