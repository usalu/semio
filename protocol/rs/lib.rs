//! 🧩 Protocol document domain + typed VCS on `vcs`.
//!
//! A strict, ordered list of steps containing typed blocks — a Blockly-like
//! visual editor for generating code/data that is list-based, not canvas-based.
//! Block `kind`s beyond [`PROTOCOL_BUILTIN_KINDS`] are module-contributed
//! (see `Contribution::ProtocolBlockKind` in `semio-framework-core`).

use serde::{Deserialize, Serialize};
use vcs::{DocumentVcsEnvelope, DocumentVcsStore, Operation, OperationDiff};

pub const PROTOCOL_DOCUMENT_SCHEMA: &str = "protocol.program";

pub use builder_kit::{
    add_block_op, add_step_op, build_palette, build_protocol_list_scene, move_block_op, move_step_op, protocol_builder_action, remove_block_op, remove_step_op, render_protocol_builder, update_protocol_title_op, ProtocolBuilderConfig,
    ProtocolBuilderLabels, PROTOCOL_BUILDER_LABELS_EN,
};
/// 🧬 Flattens `generation_forms`/`builder_kit` onto the crate root so callers keep the flat
/// `protocol::*` import surface (mirrors how `semio-framework-plugin` flattened these before the move).
pub use generation_forms::{
    add_generation, apply_generation_op, generation_ops, handle_generation_action, initial_generation_values, invert_generation_op, remove_generation, rename_generation, render_generation_form_body, render_generation_preview_text,
    render_generations_tree, select_generation, selected_generation, selected_generation_mut, update_generation_values, FormGeneration, GenerationOp, GenerationPlayState,
};

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

pub const PROTOCOL_BUILTIN_KINDS: &[&str] = &["text", "longText", "number", "slider", "boolean", "single", "multi", "date", "color", "vector", "note", "image", "file"];

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
    ProtocolSpec { schema: PROTOCOL_DOCUMENT_SCHEMA.into(), id: "protocol".into(), version: "1".into(), title: None, steps: vec![ProtocolStep { id: "s".into(), title: "Steps".into(), description: None, blocks: Vec::new() }] }
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
        ProtocolExpr::Eq { left, right } => serde_json::Value::Bool(eval_protocol_expr(left, values) == eval_protocol_expr(right, values)),
        ProtocolExpr::And { items } => serde_json::Value::Bool(items.iter().all(|item| eval_protocol_expr(item, values).as_bool().unwrap_or(false))),
        ProtocolExpr::Or { items } => serde_json::Value::Bool(items.iter().any(|item| eval_protocol_expr(item, values).as_bool().unwrap_or(false))),
        ProtocolExpr::Truthy { expr } => serde_json::Value::Bool(eval_protocol_expr(expr, values).as_bool().unwrap_or(false)),
    }
}

pub fn is_block_visible(block: &ProtocolBlock, values: &serde_json::Map<String, serde_json::Value>) -> bool {
    block.condition.as_ref().map(|expr| eval_protocol_expr(expr, values).as_bool().unwrap_or(false)).unwrap_or(true)
}

pub fn default_value_for_block(block: &ProtocolBlock) -> serde_json::Value {
    match block.kind.as_str() {
        "text" | "longText" => block.default.clone().unwrap_or(serde_json::Value::String(String::new())),
        "number" | "slider" => block.default.clone().or_else(|| block.min.map(|value| serde_json::json!(value))).unwrap_or(serde_json::json!(0)),
        "boolean" => block.default.clone().unwrap_or(serde_json::json!(false)),
        "single" => block.default.clone().or_else(|| block.options.as_ref().and_then(|options| options.first()).map(|option| serde_json::Value::String(option.value.clone()))).unwrap_or(serde_json::Value::String(String::new())),
        "multi" => block.default.clone().unwrap_or(serde_json::json!([])),
        "date" | "color" => block.default.clone().unwrap_or(serde_json::Value::String(String::new())),
        "vector" => {
            let values: Vec<f64> = block.fields.as_ref().map(|fields| fields.iter().map(|field| field.value.unwrap_or(0.0)).collect()).unwrap_or_default();
            serde_json::json!(values)
        }
        "note" | "image" | "file" => serde_json::Value::Null,
        _ if is_extension_block_kind(&block.kind) => block.params.clone().filter(|value| value.is_object() && !value.as_object().is_none_or(|obj| obj.is_empty())).unwrap_or_else(|| serde_json::json!({})),
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
                errors.push(ProtocolValidationError { block_id: block.id.clone(), message: format!("{} is required", block.label) });
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
            errors.push(ProtocolValidationError { block_id: block.id.clone(), message: format!("{} is required", block.label) });
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
            ProtocolDiff::AddStep { step, index } => ProtocolOp::AddStep { step: step.clone(), index: *index },
            ProtocolDiff::RemoveStep { step_id } => ProtocolOp::RemoveStep { step_id: step_id.clone() },
            ProtocolDiff::MoveStep { step_id, index } => ProtocolOp::MoveStep { step_id: step_id.clone(), index: *index },
            ProtocolDiff::AddBlock { step_id, block, index } => ProtocolOp::AddBlock { step_id: step_id.clone(), block: block.clone(), index: *index },
            ProtocolDiff::RemoveBlock { step_id, block_id } => ProtocolOp::RemoveBlock { step_id: step_id.clone(), block_id: block_id.clone() },
            ProtocolDiff::MoveBlock { block_id, from_step_id, to_step_id, index } => ProtocolOp::MoveBlock { block_id: block_id.clone(), from_step_id: from_step_id.clone(), to_step_id: to_step_id.clone(), index: *index },
            ProtocolDiff::UpdateBlock { step_id, block } => ProtocolOp::UpdateBlock { step_id: step_id.clone(), block: block.clone() },
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
            ProtocolOp::AddStep { step, index } => ProtocolDiff::AddStep { step: step.clone(), index: *index },
            ProtocolOp::RemoveStep { step_id } => ProtocolDiff::RemoveStep { step_id: step_id.clone() },
            ProtocolOp::MoveStep { step_id, index } => ProtocolDiff::MoveStep { step_id: step_id.clone(), index: *index },
            ProtocolOp::AddBlock { step_id, block, index } => ProtocolDiff::AddBlock { step_id: step_id.clone(), block: block.clone(), index: *index },
            ProtocolOp::RemoveBlock { step_id, block_id } => ProtocolDiff::RemoveBlock { step_id: step_id.clone(), block_id: block_id.clone() },
            ProtocolOp::MoveBlock { block_id, from_step_id, to_step_id, index } => ProtocolDiff::MoveBlock { block_id: block_id.clone(), from_step_id: from_step_id.clone(), to_step_id: to_step_id.clone(), index: *index },
            ProtocolOp::UpdateBlock { step_id, block } => ProtocolDiff::UpdateBlock { step_id: step_id.clone(), block: block.clone() },
            ProtocolOp::UpdateStep { step } => ProtocolDiff::UpdateStep { step: step.clone() },
            ProtocolOp::UpdateProtocol { title } => ProtocolDiff::UpdateProtocol { title: title.clone() },
        }
    }

    fn backwards(&self, projection: &ProtocolSpec) -> Vec<Self> {
        match self {
            ProtocolOp::AddStep { step, .. } => vec![ProtocolOp::RemoveStep { step_id: step.id.clone() }],
            ProtocolOp::RemoveStep { step_id } => projection.steps.iter().find(|s| s.id == *step_id).map(|step| vec![ProtocolOp::AddStep { step: step.clone(), index: None }]).unwrap_or_default(),
            ProtocolOp::MoveStep { step_id, .. } => projection.steps.iter().position(|s| s.id == *step_id).map(|index| vec![ProtocolOp::MoveStep { step_id: step_id.clone(), index }]).unwrap_or_default(),
            ProtocolOp::AddBlock { step_id, block, index: _ } => vec![ProtocolOp::RemoveBlock { step_id: step_id.clone(), block_id: block.id.clone() }],
            ProtocolOp::RemoveBlock { step_id, block_id } => {
                for step in &projection.steps {
                    if step.id == *step_id {
                        if let Some(block) = step.blocks.iter().find(|b| b.id == *block_id) {
                            return vec![ProtocolOp::AddBlock { step_id: step_id.clone(), block: block.clone(), index: None }];
                        }
                    }
                }
                Vec::new()
            }
            ProtocolOp::MoveBlock { block_id, from_step_id, to_step_id, index } => {
                for step in &projection.steps {
                    if step.id == *from_step_id {
                        if let Some(pos) = step.blocks.iter().position(|b| b.id == *block_id) {
                            return vec![ProtocolOp::MoveBlock { block_id: block_id.clone(), from_step_id: to_step_id.clone(), to_step_id: from_step_id.clone(), index: pos }];
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
                            return vec![ProtocolOp::UpdateBlock { step_id: step_id.clone(), block: prev.clone() }];
                        }
                    }
                }
                Vec::new()
            }
            ProtocolOp::UpdateStep { step } => projection.steps.iter().find(|s| s.id == step.id).map(|prev| vec![ProtocolOp::UpdateStep { step: prev.clone() }]).unwrap_or_default(),
            ProtocolOp::UpdateProtocol { .. } => vec![ProtocolOp::UpdateProtocol { title: projection.title.clone() }],
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
        ProtocolOp::MoveBlock { block_id, from_step_id, to_step_id, index } => {
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

//#region 🔖GenerationForms
pub mod generation_forms {
    //! 🧬 Shared Generate-mode state, CRUD, and declarative UI helpers for answering a `ProtocolSpec` as
    //! a set of named "generations" (parameter presets) — moved here (from `semio-framework-plugin`) since
    //! it is typed end-to-end on `ProtocolSpec`/`ProtocolBlock`, i.e. protocol-domain code, not SDK code.

    use super::{default_value_for_block, flatten_protocol_blocks, is_block_visible, ProtocolBlock, ProtocolSpec};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Map, Value};
    use ui_wgpu::{
        build_text_editor_scene, ui_stack_vertical, ui_text, ActionDescriptor, TextEditorScene, UiControlNode, UiFieldNode, UiInputNode, UiNode, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode,
        UiTreeSectionNode,
    };

    //#region 🔖Types
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FormGeneration {
        pub id: String,
        pub name: String,
        pub values: Map<String, Value>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GenerationPlayState {
        #[serde(default)]
        pub generations: Vec<FormGeneration>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub selected_generation_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub preview_text: Option<String>,
    }
    //#endregion 🔖Types

    //#region 🔖Crud
    fn next_generation_id(generations: &[FormGeneration]) -> String {
        format!("generation-{}", generations.len() + 1)
    }

    fn next_generation_name(generations: &[FormGeneration]) -> String {
        format!("Generation {}", generations.len() + 1)
    }

    pub fn initial_generation_values(spec: &ProtocolSpec) -> Map<String, Value> {
        let mut values = Map::new();
        for question in flatten_protocol_blocks(spec) {
            values.insert(question.id.clone(), default_value_for_block(question));
        }
        values
    }

    pub fn add_generation(state: &mut GenerationPlayState, spec: &ProtocolSpec) -> String {
        let id = next_generation_id(&state.generations);
        let name = next_generation_name(&state.generations);
        state.generations.push(FormGeneration { id: id.clone(), name, values: initial_generation_values(spec) });
        state.selected_generation_id = Some(id.clone());
        id
    }

    pub fn remove_generation(state: &mut GenerationPlayState, generation_id: &str) {
        state.generations.retain(|entry| entry.id != generation_id);
        if state.selected_generation_id.as_deref() == Some(generation_id) {
            state.selected_generation_id = state.generations.first().map(|entry| entry.id.clone());
        }
    }

    pub fn rename_generation(state: &mut GenerationPlayState, generation_id: &str, name: &str) {
        if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
            entry.name = name.to_string();
        }
    }

    pub fn select_generation(state: &mut GenerationPlayState, generation_id: &str) {
        if state.generations.iter().any(|entry| entry.id == generation_id) {
            state.selected_generation_id = Some(generation_id.to_string());
        }
    }

    pub fn selected_generation(state: &GenerationPlayState) -> Option<&FormGeneration> {
        let selected_id = state.selected_generation_id.as_deref()?;
        state.generations.iter().find(|entry| entry.id == selected_id)
    }

    pub fn selected_generation_mut(state: &mut GenerationPlayState) -> Option<&mut FormGeneration> {
        let selected_id = state.selected_generation_id.clone()?;
        state.generations.iter_mut().find(|entry| entry.id == selected_id)
    }

    pub fn update_generation_values(state: &mut GenerationPlayState, generation_id: &str, question_id: &str, value: Value) {
        if let Some(entry) = state.generations.iter_mut().find(|entry| entry.id == generation_id) {
            entry.values.insert(question_id.to_string(), value);
        }
    }

    pub fn handle_generation_action(action: &str, args: Option<&Value>, state: &mut GenerationPlayState, spec: &ProtocolSpec, controller_id: &str) -> bool {
        match action {
            "addGeneration" => {
                add_generation(state, spec);
                true
            }
            "removeGeneration" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    remove_generation(state, id);
                }
                true
            }
            "selectGeneration" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    select_generation(state, id);
                }
                true
            }
            "renameGeneration" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str());
                let name = args.and_then(|value| value.get("name")).and_then(|value| value.as_str());
                if let (Some(id), Some(name)) = (id, name) {
                    rename_generation(state, id, name);
                }
                true
            }
            "updateGenerationValues" => {
                let generation_id = args.and_then(|value| value.get("generationId")).and_then(|value| value.as_str()).map(str::to_string).or_else(|| state.selected_generation_id.clone());
                let question_id = args.and_then(|value| value.get("questionId")).and_then(|value| value.as_str());
                let value = args.and_then(|value| value.get("value"));
                if let (Some(generation_id), Some(question_id), Some(value)) = (generation_id, question_id, value) {
                    update_generation_values(state, &generation_id, question_id, value.clone());
                }
                let _ = controller_id;
                true
            }
            _ => false,
        }
    }
    //#endregion 🔖Crud

    //#region 🔖Ops
    /// @emoji 🧬 Typed, invertible Generate-mode operation vocabulary. WS-F embeds this as a variant in
    /// `forms/module/procedural`'s own `Op` enum so generation edits flow through the document store with
    /// true inverses (replacing the in-place-mutating CRUD helpers as the document mutation surface).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum GenerationOp {
        Add { generation: FormGeneration },
        Remove { id: String },
        Rename { id: String, name: String },
        UpdateValues { id: String, question_id: String, value: Value },
    }

    /// @emoji 🎛️ Maps a Generate-mode action id to the document operations it produces, or `None` for
    /// non-document (view) actions like `selectGeneration`. Pure — reads `state`/`spec` but mutates
    /// nothing; the caller applies the returned ops through its store.
    pub fn generation_ops(action: &str, args: Option<&Value>, state: &GenerationPlayState, spec: &ProtocolSpec) -> Option<Vec<GenerationOp>> {
        let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "addGeneration" => Some(vec![GenerationOp::Add { generation: FormGeneration { id: next_generation_id(&state.generations), name: next_generation_name(&state.generations), values: initial_generation_values(spec) } }]),
            "removeGeneration" => arg_str("id").map(|id| vec![GenerationOp::Remove { id }]),
            "renameGeneration" => {
                let id = arg_str("id")?;
                let name = arg_str("name")?;
                Some(vec![GenerationOp::Rename { id, name }])
            }
            "updateGenerationValues" => {
                let id = arg_str("generationId").or_else(|| state.selected_generation_id.clone())?;
                let question_id = arg_str("questionId")?;
                let value = args.and_then(|value| value.get("value")).cloned()?;
                Some(vec![GenerationOp::UpdateValues { id, question_id, value }])
            }
            _ => None,
        }
    }

    /// @emoji ▶️ Applies a {@link GenerationOp} to `state` in place.
    pub fn apply_generation_op(state: &mut GenerationPlayState, op: &GenerationOp) {
        match op {
            GenerationOp::Add { generation } => {
                state.generations.push(generation.clone());
                state.selected_generation_id = Some(generation.id.clone());
            }
            GenerationOp::Remove { id } => remove_generation(state, id),
            GenerationOp::Rename { id, name } => rename_generation(state, id, name),
            GenerationOp::UpdateValues { id, question_id, value } => update_generation_values(state, id, question_id, value.clone()),
        }
    }

    /// @emoji ↩️ Computes the inverse of a {@link GenerationOp} from the pre-state `state`.
    pub fn invert_generation_op(state: &GenerationPlayState, op: &GenerationOp) -> Vec<GenerationOp> {
        match op {
            GenerationOp::Add { generation } => vec![GenerationOp::Remove { id: generation.id.clone() }],
            GenerationOp::Remove { id } => state.generations.iter().find(|entry| entry.id == *id).map(|entry| vec![GenerationOp::Add { generation: entry.clone() }]).unwrap_or_default(),
            GenerationOp::Rename { id, .. } => state.generations.iter().find(|entry| entry.id == *id).map(|entry| vec![GenerationOp::Rename { id: id.clone(), name: entry.name.clone() }]).unwrap_or_default(),
            GenerationOp::UpdateValues { id, question_id, .. } => state
                .generations
                .iter()
                .find(|entry| entry.id == *id)
                .map(|entry| vec![GenerationOp::UpdateValues { id: id.clone(), question_id: question_id.clone(), value: entry.values.get(question_id).cloned().unwrap_or(Value::Null) }])
                .unwrap_or_default(),
        }
    }
    //#endregion 🔖Ops

    //#region 🔖Render
    fn generation_action(controller_id: &str, action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: controller_id.into(), action: action.into(), args }
    }

    pub fn render_generations_tree(controller_id: &str, surface_prefix: &str, generations: &[FormGeneration], selected_id: Option<&str>) -> UiNode {
        let items: Vec<UiTreeItemNode> = generations
            .iter()
            .map(|generation| {
                let mut actions = vec![UiTreeItemAction { icon_id: "trash-2".into(), label: Some("Remove".into()), action: generation_action(controller_id, "removeGeneration", Some(json!({ "id": generation.id }))), reveal_on_hover: Some(true) }];
                actions.insert(
                    0,
                    UiTreeItemAction {
                        icon_id: "pencil".into(),
                        label: Some("Rename".into()),
                        action: generation_action(controller_id, "renameGeneration", Some(json!({ "id": generation.id, "name": format!("{} copy", generation.name) }))),
                        reveal_on_hover: Some(true),
                    },
                );
                UiTreeItemNode {
                    id: format!("{surface_prefix}.generation.{}", generation.id),
                    label: generation.name.clone(),
                    description: Some(format!("{} values", generation.values.len())),
                    icon_id: Some("layers".into()),
                    selected: Some(selected_id == Some(generation.id.as_str())),
                    default_open: None,
                    action: Some(generation_action(controller_id, "selectGeneration", Some(json!({ "id": generation.id })))),
                    hover_action: None,
                    unhover_action: None,
                    actions: Some(actions),
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                    loading: None,
                    waiting: None,
                }
            })
            .collect();
        let mut sections = vec![UiTreeSectionNode {
            id: format!("{surface_prefix}.generations"),
            label: Some("Generations".into()),
            default_open: Some(true),
            items: if items.is_empty() {
                vec![UiTreeItemNode {
                    id: format!("{surface_prefix}.generations.empty"),
                    label: "(no generations)".into(),
                    description: None,
                    icon_id: None,
                    selected: None,
                    default_open: None,
                    action: None,
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    is_hidden: None,
                    loading: None,
                    waiting: None,
                }]
            } else {
                items
            },
            loading: None,
            waiting: None,
        }];
        sections.push(UiTreeSectionNode {
            id: format!("{surface_prefix}.actions"),
            label: Some("Actions".into()),
            default_open: Some(true),
            items: vec![UiTreeItemNode {
                id: format!("{surface_prefix}.add-generation"),
                label: "Add Generation".into(),
                description: None,
                icon_id: Some("plus".into()),
                selected: None,
                default_open: None,
                action: Some(generation_action(controller_id, "addGeneration", None)),
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                is_hidden: None,
                loading: None,
                waiting: None,
            }],
            loading: None,
            waiting: None,
        });
        UiNode::Tree(UiTreeNode {
            sections,
            loading: None,
            waiting: None,
            selected_ids: selected_id.map(|id| vec![format!("{surface_prefix}.generation.{id}")]),
            highlighted_ids: None,
            selection_change: Some(generation_action(controller_id, "selectGeneration", None)),
            drop_action: None,
        })
    }

    fn render_question_field(question: &ProtocolBlock, values: &Map<String, Value>, controller_id: &str, patch_action: &str, generation_id: &str) -> Option<UiNode> {
        if !is_block_visible(question, values) {
            return None;
        }
        let value = values.get(&question.id).cloned().unwrap_or_else(|| default_value_for_block(question));
        let field_id = format!("generate.form.{}", question.id);
        let on_change = || {
            generation_action(
                controller_id,
                patch_action,
                Some(json!({
                    "generationId": generation_id,
                    "questionId": question.id,
                })),
            )
        };
        let child = match question.kind.as_str() {
            "text" | "longText" => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: if question.kind == "longText" { "textarea".into() } else { "text".into() },
                value: value.as_str().unwrap_or_default().to_string(),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
            }),
            "number" => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: "number".into(),
                value: value.as_f64().map(|number| number.to_string()).unwrap_or_default(),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
            }),
            "slider" => UiControlNode::Slider(UiSliderNode {
                id: format!("{field_id}.slider"),
                value: value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0)),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: on_change(),
                unit: None,
            }),
            "boolean" => UiControlNode::Toggle(UiToggleNode { id: format!("{field_id}.toggle"), icon_id: "toggle-left".into(), pressed: value.as_bool().unwrap_or(false), text: Some(question.label.clone()), on_change: on_change() }),
            "single" => {
                let items = question.options.as_ref().map(|options| options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: option.label.clone() }).collect()).unwrap_or_default();
                UiControlNode::Select(UiSelectNode { id: format!("{field_id}.select"), value: value.as_str().unwrap_or_default().to_string(), items, placeholder: question.placeholder.clone(), on_change: on_change() })
            }
            "vector" => {
                let numbers = value.as_array().cloned().unwrap_or_else(|| question.fields.as_ref().map(|fields| fields.iter().map(|field| json!(field.value.unwrap_or(0.0))).collect()).unwrap_or_default());
                let labels: Vec<String> = question
                    .fields
                    .as_ref()
                    .map(|fields| fields.iter().map(|field| field.label.clone().unwrap_or_else(|| field.key.clone())).collect())
                    .unwrap_or_else(|| numbers.iter().enumerate().map(|(index, _)| format!("Field {}", index + 1)).collect());
                let children: Vec<UiNode> = numbers
                    .iter()
                    .enumerate()
                    .map(|(index, number)| {
                        let label = labels.get(index).cloned().unwrap_or_else(|| format!("Field {}", index + 1));
                        UiNode::Field(UiFieldNode {
                            id: format!("{field_id}.vector.{index}"),
                            label,
                            child: Box::new(UiNode::Input(UiInputNode {
                                id: format!("{field_id}.vector.{index}.input"),
                                input_kind: "number".into(),
                                value: number.as_f64().map(|entry| entry.to_string()).unwrap_or_default(),
                                placeholder: None,
                                commit: None,
                                on_change: generation_action(
                                    controller_id,
                                    patch_action,
                                    Some(json!({
                                        "generationId": generation_id,
                                        "questionId": question.id,
                                        "fieldIndex": index,
                                    })),
                                ),
                                min: None,
                                max: None,
                                step: None,
                                accept: None,
                            })),
                            description: None,
                            required: None,
                            error: None,
                        })
                    })
                    .collect();
                return Some(ui_stack_vertical(children));
            }
            "note" => return Some(ui_text(question.text.clone().unwrap_or_default())),
            "image" => return Some(ui_text(question.src.clone().unwrap_or_else(|| "(no image)".into()))),
            _ => UiControlNode::Input(UiInputNode {
                id: format!("{field_id}.input"),
                input_kind: "text".into(),
                value: value.to_string(),
                placeholder: question.placeholder.clone(),
                commit: None,
                on_change: on_change(),
                min: None,
                max: None,
                step: None,
                accept: None,
            }),
        };
        Some(UiNode::Field(UiFieldNode { id: field_id, label: question.label.clone(), child: Box::new(ui_wgpu::ui_control_to_node(child)), description: None, required: None, error: None }))
    }

    pub fn render_generation_form_body(form_spec: &ProtocolSpec, values: &Map<String, Value>, controller_id: &str, patch_action: &str, generation_id: &str) -> UiNode {
        let mut children = Vec::new();
        for step in &form_spec.steps {
            if !step.blocks.is_empty() {
                children.push(ui_text(step.title.clone()));
            }
            for question in &step.blocks {
                if let Some(field) = render_question_field(question, values, controller_id, patch_action, generation_id) {
                    children.push(field);
                }
            }
        }
        if children.is_empty() {
            return ui_text("No input widgets to generate from.");
        }
        ui_stack_vertical(children)
    }

    pub fn render_generation_preview_text(surface: &str, controller_id: &str, text: &str) -> UiNode {
        build_text_editor_scene(surface, controller_id, TextEditorScene::base(text.to_string(), Some("json".into()), None))
    }
    //#endregion 🔖Render

    #[cfg(test)]
    mod generation_forms_tests {
        use super::*;
        use crate::{ProtocolBlock, ProtocolStep, PROTOCOL_DOCUMENT_SCHEMA};

        fn sample_spec() -> ProtocolSpec {
            ProtocolSpec {
                schema: PROTOCOL_DOCUMENT_SCHEMA.into(),
                id: "sample".into(),
                version: "1".into(),
                title: None,
                steps: vec![ProtocolStep {
                    id: "s".into(),
                    title: "Inputs".into(),
                    description: None,
                    blocks: vec![ProtocolBlock {
                        id: "width".into(),
                        label: "Width".into(),
                        kind: "slider".into(),
                        description: None,
                        required: None,
                        placeholder: None,
                        default: Some(json!(1.0)),
                        min: Some(0.0),
                        max: Some(10.0),
                        step: Some(0.5),
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
                    }],
                }],
            }
        }

        #[test]
        fn generation_crud_round_trip() {
            let spec = sample_spec();
            let mut state = GenerationPlayState::default();
            let id = add_generation(&mut state, &spec);
            assert_eq!(state.generations.len(), 1);
            rename_generation(&mut state, &id, "Variant A");
            update_generation_values(&mut state, &id, "width", json!(4.0));
            assert_eq!(selected_generation(&state).unwrap().name, "Variant A");
            remove_generation(&mut state, &id);
            assert!(state.generations.is_empty());
        }

        #[test]
        fn render_generations_tree_contains_add_action() {
            let json = serde_json::to_string(&render_generations_tree("flow-play", "flow-generate", &[], None)).unwrap();
            assert!(json.contains("addGeneration"));
        }
    }
}
//#endregion 🔖GenerationForms

//#region 🔖BuilderKit
pub mod builder_kit {
    //! 🧩 Shared strict-list, Blockly-like builder engine: generic step/block CRUD op-builders and
    //! [`BlockListScene`] rendering, reused by `protocol-plugin` (standalone) and `forms-plugin`
    //! (embedded Blueprint mode). Block-kind-specific property editing stays with the host app. Moved
    //! here (from `semio-framework-plugin`) since it is entirely protocol-domain code.

    use super::{ProtocolBlock, ProtocolOp, ProtocolSpec, ProtocolStep};
    use serde_json::Value;
    use ui_wgpu::{ActionDescriptor, BlockListScene, BlockPaletteEntry, SurfaceKind, UiComponentSceneNode, UiNode};

    //#region 🔖Config
    #[derive(Clone, Debug)]
    pub struct ProtocolBuilderLabels {
        pub add_step: &'static str,
        pub remove_step: &'static str,
        pub move_up: &'static str,
        pub move_down: &'static str,
        pub add_block: &'static str,
    }

    pub const PROTOCOL_BUILDER_LABELS_EN: ProtocolBuilderLabels = ProtocolBuilderLabels { add_step: "Add Step", remove_step: "Remove Step", move_up: "Move Up", move_down: "Move Down", add_block: "Add Block" };

    /// 🧩 Configures the generic strict-list builder for a host app: an action-namespace prefix
    /// (used for element/surface ids so multiple embeddings don't collide), and its labels.
    #[derive(Clone, Debug)]
    pub struct ProtocolBuilderConfig {
        pub action_namespace: &'static str,
        pub controller_id: &'static str,
        pub labels: ProtocolBuilderLabels,
    }
    //#endregion 🔖Config

    //#region 🔖OpBuilders
    pub fn add_step_op(spec: &ProtocolSpec, step_id: String) -> ProtocolOp {
        ProtocolOp::AddStep { step: ProtocolStep { id: step_id, title: format!("Step {}", spec.steps.len() + 1), description: None, blocks: Vec::new() }, index: None }
    }

    pub fn remove_step_op(step_id: &str) -> ProtocolOp {
        ProtocolOp::RemoveStep { step_id: step_id.into() }
    }

    pub fn move_step_op(step_id: &str, index: usize) -> ProtocolOp {
        ProtocolOp::MoveStep { step_id: step_id.into(), index }
    }

    pub fn add_block_op(step_id: &str, block: ProtocolBlock, index: Option<usize>) -> ProtocolOp {
        ProtocolOp::AddBlock { step_id: step_id.into(), block, index }
    }

    pub fn remove_block_op(step_id: &str, block_id: &str) -> ProtocolOp {
        ProtocolOp::RemoveBlock { step_id: step_id.into(), block_id: block_id.into() }
    }

    pub fn move_block_op(block_id: &str, from_step_id: &str, to_step_id: &str, index: usize) -> ProtocolOp {
        ProtocolOp::MoveBlock { block_id: block_id.into(), from_step_id: from_step_id.into(), to_step_id: to_step_id.into(), index }
    }

    pub fn update_protocol_title_op(title: Option<String>) -> ProtocolOp {
        ProtocolOp::UpdateProtocol { title }
    }
    //#endregion 🔖OpBuilders

    //#region 🔖Render
    pub fn protocol_builder_action(config: &ProtocolBuilderConfig, action: &str, args: Option<Value>) -> ActionDescriptor {
        ActionDescriptor { controller_id: config.controller_id.into(), action: action.into(), args }
    }

    /// 🧩 Builds the palette of insertable block kinds from a host app's built-in kinds plus any
    /// `Contribution::ProtocolBlockKind` modules already resolved by the caller into label/icon pairs.
    pub fn build_palette(builtin: &[(&str, &str, &str)], extensions: &[(String, String, String)]) -> Vec<BlockPaletteEntry> {
        let mut entries: Vec<BlockPaletteEntry> = builtin.iter().map(|(kind, label, icon_id)| BlockPaletteEntry { block_kind: (*kind).into(), label: (*label).into(), icon_id: (*icon_id).into() }).collect();
        entries.extend(extensions.iter().map(|(kind, label, icon_id)| BlockPaletteEntry { block_kind: kind.clone(), label: label.clone(), icon_id: icon_id.clone() }));
        entries
    }

    pub fn build_protocol_list_scene(spec: &ProtocolSpec, palette: &[BlockPaletteEntry], selected_id: Option<&str>) -> BlockListScene {
        BlockListScene { steps_json: serde_json::to_string(&spec.steps).unwrap_or_else(|_| "[]".into()), palette_json: serde_json::to_string(palette).unwrap_or_else(|_| "[]".into()), selected_id: selected_id.map(String::from), dragging_id: None }
    }

    /// 🧩 Renders the strict-list Blockly-like builder as a [`SurfaceKind::BlockList`] component
    /// scene, handed off to the dedicated `block-list-host.tsx` React host for drag-and-drop.
    pub fn render_protocol_builder(surface_id: &str, spec: &ProtocolSpec, palette: &[BlockPaletteEntry], selected_id: Option<&str>, config: &ProtocolBuilderConfig) -> UiNode {
        UiNode::ComponentScene(UiComponentSceneNode {
            surface_id: surface_id.into(),
            controller_id: config.controller_id.into(),
            component_kind: SurfaceKind::BlockList,
            pane_id: None,
            binding_id: None,
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            diff_view: None,
            event_feed: None,
            block_list: Some(build_protocol_list_scene(spec, palette, selected_id)),
        })
    }
    //#endregion 🔖Render

    #[cfg(test)]
    mod builder_kit_tests {
        use super::*;
        use crate::empty_protocol_projection;

        fn sample_config() -> ProtocolBuilderConfig {
            ProtocolBuilderConfig { action_namespace: "protocol-play", controller_id: "protocol-play", labels: PROTOCOL_BUILDER_LABELS_EN }
        }

        #[test]
        fn add_step_op_names_step_by_position() {
            let spec = empty_protocol_projection();
            let op = add_step_op(&spec, "step-2".into());
            assert_eq!(op, ProtocolOp::AddStep { step: ProtocolStep { id: "step-2".into(), title: "Step 2".into(), description: None, blocks: Vec::new() }, index: None });
        }

        #[test]
        fn render_protocol_builder_emits_block_list_component_scene() {
            let spec = empty_protocol_projection();
            let config = sample_config();
            let node = render_protocol_builder("surface", &spec, &[], None, &config);
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("\"componentKind\":\"block-list\""));
            assert!(json.contains("\"blockList\""));
        }
    }
}
//#endregion 🔖BuilderKit

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use std::cell::RefCell;
    use vcs::create_document_vcs_envelope;
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
                    let envelope: ProtocolEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    ProtocolStore::new(envelope)
                }
                None => ProtocolStore::new(create_document_vcs_envelope(PROTOCOL_DOCUMENT_SCHEMA, "protocol", empty_protocol_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchJson)]
        pub fn dispatch_json(&self, command_json: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_json(command_json).map_err(|e| JsValue::from_str(&e.to_string()))
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
    use vcs::{create_document_vcs_envelope, DocumentVcsCommand};

    #[test]
    fn protocol_document_vcs_materializes() {
        let store = ProtocolStore::new(create_document_vcs_envelope(PROTOCOL_DOCUMENT_SCHEMA, "protocol", empty_protocol_projection(), None));
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
        let mut store = ProtocolStore::new(create_document_vcs_envelope(PROTOCOL_DOCUMENT_SCHEMA, "protocol", empty_protocol_projection(), None));
        let step = ProtocolStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        let backwards = store.projection().expect("projection");
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![ProtocolOp::AddStep { step: step.clone(), index: None }], description: None }).expect("apply");
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
                    condition: Some(ProtocolExpr::Truthy { expr: Box::new(ProtocolExpr::Var { name: "show".into() }) }),
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
