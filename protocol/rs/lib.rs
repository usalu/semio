//! 🧩 Protocol document domain + typed VCS on `vcs`.
//!
//! A strict, ordered list of steps containing typed blocks — a Blockly-like
//! visual editor for generating code/data that is list-based, not canvas-based.
//! Block `kind`s beyond [`PROTOCOL_BUILTIN_KINDS`] are module-contributed
//! (see `Contribution::ProtocolBlockKind` in `semio-framework-core`).

use vcs::{
    create_document_vcs_envelope, DocumentVcsCommand, DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff,
};
use serde::{Deserialize, Serialize};

pub const PROTOCOL_DOCUMENT_SCHEMA: &str = "protocol.program";

//#region 🔖Domain
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolStep {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub blocks: Vec<ProtocolBlock>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolBlock {
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
    pub options: Option<Vec<ProtocolBlockOption>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ProtocolVectorField>>,
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
    pub condition: Option<ProtocolExpr>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolVectorField {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolBlockOption {
    #[serde(alias = "id")]
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ProtocolExpr {
    Const { value: serde_json::Value },
    Var { name: String },
    Eq { left: Box<ProtocolExpr>, right: Box<ProtocolExpr> },
    And { items: Vec<ProtocolExpr> },
    Or { items: Vec<ProtocolExpr> },
    Truthy { expr: Box<ProtocolExpr> },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolValidationError {
    pub block_id: String,
    pub message: String,
}

pub const PROTOCOL_BUILTIN_KINDS: &[&str] = &[
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

pub fn is_extension_block_kind(kind: &str) -> bool {
    !PROTOCOL_BUILTIN_KINDS.contains(&kind)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolSpec {
    pub schema: String,
    pub id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub steps: Vec<ProtocolStep>,
}

pub type ProtocolEnvelope = DocumentVcsEnvelope<ProtocolSpec, ProtocolOp>;
pub type ProtocolStore = DocumentVcsStore<ProtocolSpec, ProtocolOp>;

pub fn empty_protocol_projection() -> ProtocolSpec {
    ProtocolSpec {
        schema: PROTOCOL_DOCUMENT_SCHEMA.into(),
        id: "protocol".into(),
        version: "1".into(),
        title: None,
        steps: vec![ProtocolStep {
            id: "s".into(),
            title: "Steps".into(),
            description: None,
            blocks: Vec::new(),
        }],
    }
}
//#endregion 🔖Domain

//#region 🔖Runtime
pub fn flatten_protocol_blocks(spec: &ProtocolSpec) -> Vec<&ProtocolBlock> {
    spec.steps.iter().flat_map(|step| step.blocks.iter()).collect()
}

pub fn find_block_location<'a>(spec: &'a ProtocolSpec, block_id: &str) -> Option<(&'a ProtocolStep, usize, &'a ProtocolBlock)> {
    for step in &spec.steps {
        if let Some(index) = step.blocks.iter().position(|block| block.id == block_id) {
            return Some((step, index, &step.blocks[index]));
        }
    }
    None
}

pub fn eval_protocol_expr(expr: &ProtocolExpr, values: &serde_json::Map<String, serde_json::Value>) -> serde_json::Value {
    match expr {
        ProtocolExpr::Const { value } => value.clone(),
        ProtocolExpr::Var { name } => values.get(name).cloned().unwrap_or(serde_json::Value::Null),
        ProtocolExpr::Eq { left, right } => {
            serde_json::Value::Bool(eval_protocol_expr(left, values) == eval_protocol_expr(right, values))
        }
        ProtocolExpr::And { items } => {
            serde_json::Value::Bool(items.iter().all(|item| eval_protocol_expr(item, values).as_bool().unwrap_or(false)))
        }
        ProtocolExpr::Or { items } => {
            serde_json::Value::Bool(items.iter().any(|item| eval_protocol_expr(item, values).as_bool().unwrap_or(false)))
        }
        ProtocolExpr::Truthy { expr } => serde_json::Value::Bool(eval_protocol_expr(expr, values).as_bool().unwrap_or(false)),
    }
}

pub fn is_block_visible(block: &ProtocolBlock, values: &serde_json::Map<String, serde_json::Value>) -> bool {
    block
        .condition
        .as_ref()
        .map(|expr| eval_protocol_expr(expr, values).as_bool().unwrap_or(false))
        .unwrap_or(true)
}

pub fn default_value_for_block(block: &ProtocolBlock) -> serde_json::Value {
    match block.kind.as_str() {
        "text" | "longText" => block.default.clone().unwrap_or(serde_json::Value::String(String::new())),
        "number" | "slider" => block
            .default
            .clone()
            .or_else(|| block.min.map(|value| serde_json::json!(value)))
            .unwrap_or(serde_json::json!(0)),
        "boolean" => block.default.clone().unwrap_or(serde_json::json!(false)),
        "single" => block
            .default
            .clone()
            .or_else(|| {
                block
                    .options
                    .as_ref()
                    .and_then(|options| options.first())
                    .map(|option| serde_json::Value::String(option.value.clone()))
            })
            .unwrap_or(serde_json::Value::String(String::new())),
        "multi" => block.default.clone().unwrap_or(serde_json::json!([])),
        "date" | "color" => block.default.clone().unwrap_or(serde_json::Value::String(String::new())),
        "vector" => {
            let values: Vec<f64> = block
                .fields
                .as_ref()
                .map(|fields| fields.iter().map(|field| field.value.unwrap_or(0.0)).collect())
                .unwrap_or_default();
            serde_json::json!(values)
        }
        "note" | "image" | "file" => serde_json::Value::Null,
        _ if is_extension_block_kind(&block.kind) => block
            .params
            .clone()
            .filter(|value| value.is_object() && !value.as_object().is_none_or(|obj| obj.is_empty()))
            .unwrap_or_else(|| serde_json::json!({})),
        _ => serde_json::Value::Null,
    }
}

pub fn visible_blocks<'a>(step: &'a ProtocolStep, values: &serde_json::Map<String, serde_json::Value>) -> Vec<&'a ProtocolBlock> {
    step.blocks.iter().filter(|block| is_block_visible(block, values)).collect()
}

pub fn step_errors(step: &ProtocolStep, values: &serde_json::Map<String, serde_json::Value>) -> Vec<ProtocolValidationError> {
    let mut errors = Vec::new();
    for block in visible_blocks(step, values) {
        if block.kind == "note" || block.kind == "image" {
            continue;
        }
        if !block.required.unwrap_or(false) {
            continue;
        }
        let value = values.get(&block.id);
        if is_extension_block_kind(&block.kind) {
            let empty = value.is_none_or(|value| !value.is_object() || value.as_object().is_none_or(|obj| obj.is_empty()));
            if empty {
                errors.push(ProtocolValidationError {
                    block_id: block.id.clone(),
                    message: format!("{} is required", block.label),
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
            errors.push(ProtocolValidationError {
                block_id: block.id.clone(),
                message: format!("{} is required", block.label),
            });
        }
    }
    errors
}

pub fn can_advance(step: &ProtocolStep, values: &serde_json::Map<String, serde_json::Value>) -> bool {
    step_errors(step, values).is_empty()
}

pub fn initial_values(spec: &ProtocolSpec, overrides: &serde_json::Map<String, serde_json::Value>) -> serde_json::Map<String, serde_json::Value> {
    let mut values = serde_json::Map::new();
    for block in flatten_protocol_blocks(spec) {
        values.insert(block.id.clone(), default_value_for_block(block));
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
pub enum ProtocolOp {
    AddStep {
        step: ProtocolStep,
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
    AddBlock {
        step_id: String,
        block: ProtocolBlock,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveBlock {
        step_id: String,
        block_id: String,
    },
    MoveBlock {
        block_id: String,
        from_step_id: String,
        to_step_id: String,
        index: usize,
    },
    UpdateBlock {
        step_id: String,
        block: ProtocolBlock,
    },
    UpdateStep {
        step: ProtocolStep,
    },
    UpdateProtocol {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ProtocolDiff {
    #[default]
    Empty,
    AddStep {
        step: ProtocolStep,
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
    AddBlock {
        step_id: String,
        block: ProtocolBlock,
        #[serde(skip_serializing_if = "Option::is_none")]
        index: Option<usize>,
    },
    RemoveBlock {
        step_id: String,
        block_id: String,
    },
    MoveBlock {
        block_id: String,
        from_step_id: String,
        to_step_id: String,
        index: usize,
    },
    UpdateBlock {
        step_id: String,
        block: ProtocolBlock,
    },
    UpdateStep {
        step: ProtocolStep,
    },
    UpdateProtocol {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

impl OperationDiff<ProtocolSpec> for ProtocolDiff {
    fn apply(&self, projection: &ProtocolSpec) -> ProtocolSpec {
        let op = match self {
            ProtocolDiff::Empty => return projection.clone(),
            ProtocolDiff::AddStep { step, index } => ProtocolOp::AddStep {
                step: step.clone(),
                index: *index,
            },
            ProtocolDiff::RemoveStep { step_id } => ProtocolOp::RemoveStep { step_id: step_id.clone() },
            ProtocolDiff::MoveStep { step_id, index } => ProtocolOp::MoveStep {
                step_id: step_id.clone(),
                index: *index,
            },
            ProtocolDiff::AddBlock { step_id, block, index } => ProtocolOp::AddBlock {
                step_id: step_id.clone(),
                block: block.clone(),
                index: *index,
            },
            ProtocolDiff::RemoveBlock { step_id, block_id } => ProtocolOp::RemoveBlock {
                step_id: step_id.clone(),
                block_id: block_id.clone(),
            },
            ProtocolDiff::MoveBlock {
                block_id,
                from_step_id,
                to_step_id,
                index,
            } => ProtocolOp::MoveBlock {
                block_id: block_id.clone(),
                from_step_id: from_step_id.clone(),
                to_step_id: to_step_id.clone(),
                index: *index,
            },
            ProtocolDiff::UpdateBlock { step_id, block } => ProtocolOp::UpdateBlock {
                step_id: step_id.clone(),
                block: block.clone(),
            },
            ProtocolDiff::UpdateStep { step } => ProtocolOp::UpdateStep { step: step.clone() },
            ProtocolDiff::UpdateProtocol { title } => ProtocolOp::UpdateProtocol { title: title.clone() },
        };
        apply_protocol_edit_op(projection, &op)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, ProtocolDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<ProtocolSpec> for ProtocolOp {
    type Diff = ProtocolDiff;

    fn diff(&self, _projection: &ProtocolSpec) -> ProtocolDiff {
        match self {
            ProtocolOp::AddStep { step, index } => ProtocolDiff::AddStep {
                step: step.clone(),
                index: *index,
            },
            ProtocolOp::RemoveStep { step_id } => ProtocolDiff::RemoveStep { step_id: step_id.clone() },
            ProtocolOp::MoveStep { step_id, index } => ProtocolDiff::MoveStep {
                step_id: step_id.clone(),
                index: *index,
            },
            ProtocolOp::AddBlock { step_id, block, index } => ProtocolDiff::AddBlock {
                step_id: step_id.clone(),
                block: block.clone(),
                index: *index,
            },
            ProtocolOp::RemoveBlock { step_id, block_id } => ProtocolDiff::RemoveBlock {
                step_id: step_id.clone(),
                block_id: block_id.clone(),
            },
            ProtocolOp::MoveBlock {
                block_id,
                from_step_id,
                to_step_id,
                index,
            } => ProtocolDiff::MoveBlock {
                block_id: block_id.clone(),
                from_step_id: from_step_id.clone(),
                to_step_id: to_step_id.clone(),
                index: *index,
            },
            ProtocolOp::UpdateBlock { step_id, block } => ProtocolDiff::UpdateBlock {
                step_id: step_id.clone(),
                block: block.clone(),
            },
            ProtocolOp::UpdateStep { step } => ProtocolDiff::UpdateStep { step: step.clone() },
            ProtocolOp::UpdateProtocol { title } => ProtocolDiff::UpdateProtocol { title: title.clone() },
        }
    }

    fn backwards(&self, projection: &ProtocolSpec) -> Vec<Self> {
        match self {
            ProtocolOp::AddStep { step, .. } => vec![ProtocolOp::RemoveStep { step_id: step.id.clone() }],
            ProtocolOp::RemoveStep { step_id } => projection
                .steps
                .iter()
                .find(|s| s.id == *step_id)
                .map(|step| {
                    vec![ProtocolOp::AddStep {
                        step: step.clone(),
                        index: None,
                    }]
                })
                .unwrap_or_default(),
            ProtocolOp::MoveStep { step_id, .. } => projection
                .steps
                .iter()
                .position(|s| s.id == *step_id)
                .map(|index| vec![ProtocolOp::MoveStep { step_id: step_id.clone(), index }])
                .unwrap_or_default(),
            ProtocolOp::AddBlock { step_id, block, index: _ } => vec![ProtocolOp::RemoveBlock {
                step_id: step_id.clone(),
                block_id: block.id.clone(),
            }],
            ProtocolOp::RemoveBlock { step_id, block_id } => {
                for step in &projection.steps {
                    if step.id == *step_id {
                        if let Some(block) = step.blocks.iter().find(|b| b.id == *block_id) {
                            return vec![ProtocolOp::AddBlock {
                                step_id: step_id.clone(),
                                block: block.clone(),
                                index: None,
                            }];
                        }
                    }
                }
                Vec::new()
            }
            ProtocolOp::MoveBlock {
                block_id,
                from_step_id,
                to_step_id,
                index,
            } => {
                for step in &projection.steps {
                    if step.id == *from_step_id {
                        if let Some(pos) = step.blocks.iter().position(|b| b.id == *block_id) {
                            return vec![ProtocolOp::MoveBlock {
                                block_id: block_id.clone(),
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
            ProtocolOp::UpdateBlock { step_id, block } => {
                for step in &projection.steps {
                    if step.id == *step_id {
                        if let Some(prev) = step.blocks.iter().find(|b| b.id == block.id) {
                            return vec![ProtocolOp::UpdateBlock {
                                step_id: step_id.clone(),
                                block: prev.clone(),
                            }];
                        }
                    }
                }
                Vec::new()
            }
            ProtocolOp::UpdateStep { step } => projection
                .steps
                .iter()
                .find(|s| s.id == step.id)
                .map(|prev| vec![ProtocolOp::UpdateStep { step: prev.clone() }])
                .unwrap_or_default(),
            ProtocolOp::UpdateProtocol { .. } => vec![ProtocolOp::UpdateProtocol {
                title: projection.title.clone(),
            }],
        }
    }
}

pub fn apply_protocol_edit_op(spec: &ProtocolSpec, op: &ProtocolOp) -> ProtocolSpec {
    let mut next = spec.clone();
    match op {
        ProtocolOp::AddStep { step, index } => {
            let at = index.unwrap_or(next.steps.len());
            next.steps.insert(at.min(next.steps.len()), step.clone());
        }
        ProtocolOp::RemoveStep { step_id } => {
            next.steps.retain(|step| step.id != *step_id);
        }
        ProtocolOp::MoveStep { step_id, index } => {
            let from = next.steps.iter().position(|step| step.id == *step_id);
            if let Some(from) = from {
                let step = next.steps.remove(from);
                let at = (*index).min(next.steps.len());
                next.steps.insert(at, step);
            }
        }
        ProtocolOp::AddBlock { step_id, block, index } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    let at = index.unwrap_or(step.blocks.len());
                    step.blocks.insert(at.min(step.blocks.len()), block.clone());
                }
            }
        }
        ProtocolOp::RemoveBlock { step_id, block_id } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    step.blocks.retain(|block| block.id != *block_id);
                }
            }
        }
        ProtocolOp::MoveBlock {
            block_id,
            from_step_id,
            to_step_id,
            index,
        } => {
            let mut moving: Option<ProtocolBlock> = None;
            for step in &mut next.steps {
                if step.id == *from_step_id {
                    if let Some(pos) = step.blocks.iter().position(|b| b.id == *block_id) {
                        moving = Some(step.blocks.remove(pos));
                    }
                }
            }
            if let Some(block) = moving {
                for step in &mut next.steps {
                    if step.id == *to_step_id {
                        let at = (*index).min(step.blocks.len());
                        step.blocks.insert(at, block.clone());
                    }
                }
            }
        }
        ProtocolOp::UpdateBlock { step_id, block } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    for entry in &mut step.blocks {
                        if entry.id == block.id {
                            *entry = block.clone();
                        }
                    }
                }
            }
        }
        ProtocolOp::UpdateStep { step } => {
            for entry in &mut next.steps {
                if entry.id == step.id {
                    *entry = step.clone();
                }
            }
        }
        ProtocolOp::UpdateProtocol { title } => {
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
    pub struct ProtocolDocumentVcs {
        store: RefCell<ProtocolStore>,
    }

    #[wasm_bindgen]
    impl ProtocolDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<ProtocolDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: ProtocolEnvelope =
                        serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    ProtocolStore::new(envelope)
                }
                None => ProtocolStore::new(create_document_vcs_envelope(
                    PROTOCOL_DOCUMENT_SCHEMA,
                    "protocol",
                    empty_protocol_projection(),
                    None,
                )),
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
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
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
    fn protocol_document_vcs_materializes() {
        let store = ProtocolStore::new(create_document_vcs_envelope(
            PROTOCOL_DOCUMENT_SCHEMA,
            "protocol",
            empty_protocol_projection(),
            None,
        ));
        let projection = store.projection().expect("projection");
        assert_eq!(projection.schema, PROTOCOL_DOCUMENT_SCHEMA);
    }

    #[test]
    fn update_protocol_op_sets_and_reverts_title() {
        let spec = empty_protocol_projection();
        let op = ProtocolOp::UpdateProtocol { title: Some("Renamed".into()) };
        let next = apply_protocol_edit_op(&spec, &op);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
        let inverse = op.backwards(&spec);
        assert_eq!(inverse, vec![ProtocolOp::UpdateProtocol { title: spec.title.clone() }]);
        let reverted = inverse.iter().fold(next.clone(), |current, op| apply_protocol_edit_op(&current, op));
        assert_eq!(reverted.title, spec.title);
        assert_eq!(op.diff(&spec).apply(&spec).title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn add_step_op_replays() {
        let mut store = ProtocolStore::new(create_document_vcs_envelope(
            PROTOCOL_DOCUMENT_SCHEMA,
            "protocol",
            empty_protocol_projection(),
            None,
        ));
        let step = ProtocolStep {
            id: "step-2".into(),
            title: "Review".into(),
            description: None,
            blocks: Vec::new(),
        };
        let backwards = store.projection().expect("projection");
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![ProtocolOp::AddStep { step: step.clone(), index: None }],
                description: None,
            })
            .expect("apply");
        assert_eq!(store.projection().expect("projection").steps.len(), 2);
        let _ = backwards;
    }

    #[test]
    fn block_fields_roundtrip() {
        let json = r#"{
            "id":"b1",
            "label":"Team size",
            "kind":"slider",
            "required":true,
            "min":1,
            "max":50,
            "step":1,
            "unit":"people",
            "condition":{"kind":"truthy","expr":{"kind":"var","name":"show-team-size"}}
        }"#;
        let block: ProtocolBlock = serde_json::from_str(json).expect("block json");
        assert_eq!(block.min, Some(1.0));
        assert_eq!(block.unit.as_deref(), Some("people"));
        assert!(block.required.unwrap_or(false));
    }

    #[test]
    fn conditional_visibility_filters_blocks() {
        let step = ProtocolStep {
            id: "s".into(),
            title: "Step".into(),
            description: None,
            blocks: vec![
                ProtocolBlock {
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
                ProtocolBlock {
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
                    condition: Some(ProtocolExpr::Truthy {
                        expr: Box::new(ProtocolExpr::Var { name: "show".into() }),
                    }),
                },
            ],
        };
        let mut values = serde_json::Map::new();
        values.insert("show".into(), serde_json::json!(false));
        assert_eq!(visible_blocks(&step, &values).len(), 1);
        values.insert("show".into(), serde_json::json!(true));
        assert_eq!(visible_blocks(&step, &values).len(), 2);
    }
}
//#endregion 🧪Tests
