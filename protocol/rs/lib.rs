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
    add_block_operation, add_step_operation, build_palette, build_protocol_list_scene, move_block_operation, move_step_operation, protocol_builder_action, remove_block_operation, remove_step_operation, render_protocol_builder, update_protocol_title_operation, ProtocolBuilderConfig,
    ProtocolBuilderLabels, PROTOCOL_BUILDER_LABELS_EN,
};
/// 🧬 Flattens `generation_forms`/`builder_kit` onto the crate root so callers keep the flat
/// `protocol::*` import surface (mirrors how `semio-framework-plugin` flattened these before the move).
pub use generation_forms::{
    add_generation, apply_generation_operation, generation_operations, handle_generation_action, initial_generation_values, invert_generation_operation, remove_generation, rename_generation, render_generation_form_body, render_generation_preview_text,
    render_generations_tree, select_generation, selected_generation, selected_generation_mut, update_generation_values, FormGeneration, GenerationOperation, GenerationPlayState,
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

pub type ProtocolEnvelope = DocumentVcsEnvelope<ProtocolSpec, ProtocolOperation>;
pub type ProtocolStore = DocumentVcsStore<ProtocolSpec, ProtocolOperation>;

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

//#region 🔖Operations
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum ProtocolOperation {
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
        let operation = match self {
            ProtocolDiff::Empty => return projection.clone(),
            ProtocolDiff::AddStep { step, index } => ProtocolOperation::AddStep { step: step.clone(), index: *index },
            ProtocolDiff::RemoveStep { step_id } => ProtocolOperation::RemoveStep { step_id: step_id.clone() },
            ProtocolDiff::MoveStep { step_id, index } => ProtocolOperation::MoveStep { step_id: step_id.clone(), index: *index },
            ProtocolDiff::AddBlock { step_id, block, index } => ProtocolOperation::AddBlock { step_id: step_id.clone(), block: block.clone(), index: *index },
            ProtocolDiff::RemoveBlock { step_id, block_id } => ProtocolOperation::RemoveBlock { step_id: step_id.clone(), block_id: block_id.clone() },
            ProtocolDiff::MoveBlock { block_id, from_step_id, to_step_id, index } => ProtocolOperation::MoveBlock { block_id: block_id.clone(), from_step_id: from_step_id.clone(), to_step_id: to_step_id.clone(), index: *index },
            ProtocolDiff::UpdateBlock { step_id, block } => ProtocolOperation::UpdateBlock { step_id: step_id.clone(), block: block.clone() },
            ProtocolDiff::UpdateStep { step } => ProtocolOperation::UpdateStep { step: step.clone() },
            ProtocolDiff::UpdateProtocol { title } => ProtocolOperation::UpdateProtocol { title: title.clone() },
        };
        apply_protocol_edit_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, ProtocolDiff::Empty) {
            *self = other;
        }
    }
}

impl Operation<ProtocolSpec> for ProtocolOperation {
    type Diff = ProtocolDiff;

    fn diff(&self, _projection: &ProtocolSpec) -> ProtocolDiff {
        match self {
            ProtocolOperation::AddStep { step, index } => ProtocolDiff::AddStep { step: step.clone(), index: *index },
            ProtocolOperation::RemoveStep { step_id } => ProtocolDiff::RemoveStep { step_id: step_id.clone() },
            ProtocolOperation::MoveStep { step_id, index } => ProtocolDiff::MoveStep { step_id: step_id.clone(), index: *index },
            ProtocolOperation::AddBlock { step_id, block, index } => ProtocolDiff::AddBlock { step_id: step_id.clone(), block: block.clone(), index: *index },
            ProtocolOperation::RemoveBlock { step_id, block_id } => ProtocolDiff::RemoveBlock { step_id: step_id.clone(), block_id: block_id.clone() },
            ProtocolOperation::MoveBlock { block_id, from_step_id, to_step_id, index } => ProtocolDiff::MoveBlock { block_id: block_id.clone(), from_step_id: from_step_id.clone(), to_step_id: to_step_id.clone(), index: *index },
            ProtocolOperation::UpdateBlock { step_id, block } => ProtocolDiff::UpdateBlock { step_id: step_id.clone(), block: block.clone() },
            ProtocolOperation::UpdateStep { step } => ProtocolDiff::UpdateStep { step: step.clone() },
            ProtocolOperation::UpdateProtocol { title } => ProtocolDiff::UpdateProtocol { title: title.clone() },
        }
    }

    fn backwards(&self, projection: &ProtocolSpec) -> Vec<Self> {
        match self {
            ProtocolOperation::AddStep { step, .. } => vec![ProtocolOperation::RemoveStep { step_id: step.id.clone() }],
            ProtocolOperation::RemoveStep { step_id } => projection.steps.iter().find(|s| s.id == *step_id).map(|step| vec![ProtocolOperation::AddStep { step: step.clone(), index: None }]).unwrap_or_default(),
            ProtocolOperation::MoveStep { step_id, .. } => projection.steps.iter().position(|s| s.id == *step_id).map(|index| vec![ProtocolOperation::MoveStep { step_id: step_id.clone(), index }]).unwrap_or_default(),
            ProtocolOperation::AddBlock { step_id, block, index: _ } => vec![ProtocolOperation::RemoveBlock { step_id: step_id.clone(), block_id: block.id.clone() }],
            ProtocolOperation::RemoveBlock { step_id, block_id } => {
                for step in &projection.steps {
                    if step.id == *step_id {
                        if let Some(block) = step.blocks.iter().find(|b| b.id == *block_id) {
                            return vec![ProtocolOperation::AddBlock { step_id: step_id.clone(), block: block.clone(), index: None }];
                        }
                    }
                }
                Vec::new()
            }
            ProtocolOperation::MoveBlock { block_id, from_step_id, to_step_id, index } => {
                for step in &projection.steps {
                    if step.id == *from_step_id {
                        if let Some(pos) = step.blocks.iter().position(|b| b.id == *block_id) {
                            return vec![ProtocolOperation::MoveBlock { block_id: block_id.clone(), from_step_id: to_step_id.clone(), to_step_id: from_step_id.clone(), index: pos }];
                        }
                    }
                }
                let _ = index;
                Vec::new()
            }
            ProtocolOperation::UpdateBlock { step_id, block } => {
                for step in &projection.steps {
                    if step.id == *step_id {
                        if let Some(prev) = step.blocks.iter().find(|b| b.id == block.id) {
                            return vec![ProtocolOperation::UpdateBlock { step_id: step_id.clone(), block: prev.clone() }];
                        }
                    }
                }
                Vec::new()
            }
            ProtocolOperation::UpdateStep { step } => projection.steps.iter().find(|s| s.id == step.id).map(|prev| vec![ProtocolOperation::UpdateStep { step: prev.clone() }]).unwrap_or_default(),
            ProtocolOperation::UpdateProtocol { .. } => vec![ProtocolOperation::UpdateProtocol { title: projection.title.clone() }],
        }
    }
}

pub fn apply_protocol_edit_operation(spec: &ProtocolSpec, operation: &ProtocolOperation) -> ProtocolSpec {
    let mut next = spec.clone();
    match operation {
        ProtocolOperation::AddStep { step, index } => {
            let at = index.unwrap_or(next.steps.len());
            next.steps.insert(at.min(next.steps.len()), step.clone());
        }
        ProtocolOperation::RemoveStep { step_id } => {
            next.steps.retain(|step| step.id != *step_id);
        }
        ProtocolOperation::MoveStep { step_id, index } => {
            let from = next.steps.iter().position(|step| step.id == *step_id);
            if let Some(from) = from {
                let step = next.steps.remove(from);
                let at = (*index).min(next.steps.len());
                next.steps.insert(at, step);
            }
        }
        ProtocolOperation::AddBlock { step_id, block, index } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    let at = index.unwrap_or(step.blocks.len());
                    step.blocks.insert(at.min(step.blocks.len()), block.clone());
                }
            }
        }
        ProtocolOperation::RemoveBlock { step_id, block_id } => {
            for step in &mut next.steps {
                if step.id == *step_id {
                    step.blocks.retain(|block| block.id != *block_id);
                }
            }
        }
        ProtocolOperation::MoveBlock { block_id, from_step_id, to_step_id, index } => {
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
        ProtocolOperation::UpdateBlock { step_id, block } => {
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
        ProtocolOperation::UpdateStep { step } => {
            for entry in &mut next.steps {
                if entry.id == step.id {
                    *entry = step.clone();
                }
            }
        }
        ProtocolOperation::UpdateProtocol { title } => {
            next.title = title.clone();
        }
    }
    next
}
//#endregion 🔖Operations

//#region 🔖Dsl
/// 📜 Hand-rolled lexer, parser and printer for `ProtocolSpec`'s `.protocol` DSL and for
/// `ProtocolOperation`'s compact single-line op encoding (each op reprints the same `step`/`block`/
/// expr grammar on one line). Whitespace (including newlines) is never significant to the parser —
/// `print_dsl` inserts newlines/indentation purely for readability, `print_op` renders the identical
/// grammar with spaces only. See {@link vcs::DocumentDsl} and {@link vcs::OpText}.
mod protocol_text {
    use super::{ProtocolBlock, ProtocolBlockOption, ProtocolExpr, ProtocolSpec, ProtocolStep, ProtocolVectorField};
    use serde_json::Value;

    //#region Lexer
    #[derive(Clone, Debug, PartialEq)]
    enum Tok {
        Word(String),
        Str(String),
        LBrace,
        RBrace,
        Eof,
    }

    #[derive(Clone, Debug)]
    struct Lexed {
        tok: Tok,
        span: vcs::TextSpan,
    }

    /// 🔤 Scans `input` into tokens. A bareword `Word` runs until whitespace/`{`/`}`/`"`, so `=` is an
    /// ordinary word character — `key=value` collapses into one token (split later by
    /// {@link Parser::parse_kv_map}), and only a quoted value forces a token boundary right after `key=`.
    fn lex(input: &str) -> Result<Vec<Lexed>, vcs::TextError> {
        let chars: Vec<char> = input.chars().collect();
        let mut out = Vec::new();
        let mut i = 0usize;
        let mut line = 1u32;
        let mut col = 1u32;
        while i < chars.len() {
            match chars[i] {
                ' ' | '\t' | '\r' => {
                    i += 1;
                    col += 1;
                }
                '\n' => {
                    i += 1;
                    line += 1;
                    col = 1;
                }
                '{' => {
                    out.push(Lexed { tok: Tok::LBrace, span: vcs::TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '}' => {
                    out.push(Lexed { tok: Tok::RBrace, span: vcs::TextSpan::at(line, col) });
                    i += 1;
                    col += 1;
                }
                '"' => {
                    let (start_line, start_col) = (line, col);
                    i += 1;
                    col += 1;
                    let mut s = String::new();
                    let mut closed = false;
                    while i < chars.len() {
                        let ch = chars[i];
                        if ch == '\\' && i + 1 < chars.len() {
                            match chars[i + 1] {
                                'n' => s.push('\n'),
                                '"' => s.push('"'),
                                '\\' => s.push('\\'),
                                other => {
                                    s.push('\\');
                                    s.push(other);
                                }
                            }
                            i += 2;
                            col += 2;
                        } else if ch == '"' {
                            i += 1;
                            col += 1;
                            closed = true;
                            break;
                        } else if ch == '\n' {
                            s.push(ch);
                            i += 1;
                            line += 1;
                            col = 1;
                        } else {
                            s.push(ch);
                            i += 1;
                            col += 1;
                        }
                    }
                    if !closed {
                        return Err(vcs::TextError::new("unterminated string literal", vcs::TextSpan::at(start_line, start_col)));
                    }
                    out.push(Lexed { tok: Tok::Str(s), span: vcs::TextSpan::at(start_line, start_col) });
                }
                _ => {
                    let (start_line, start_col, start) = (line, col, i);
                    while i < chars.len() && !matches!(chars[i], ' ' | '\t' | '\r' | '\n' | '{' | '}' | '"') {
                        i += 1;
                        col += 1;
                    }
                    let word: String = chars[start..i].iter().collect();
                    out.push(Lexed { tok: Tok::Word(word), span: vcs::TextSpan::at(start_line, start_col) });
                }
            }
        }
        out.push(Lexed { tok: Tok::Eof, span: vcs::TextSpan::at(line, col) });
        Ok(out)
    }
    //#endregion Lexer

    //#region Parser
    #[derive(Clone, Debug)]
    enum FieldValue {
        Str(String),
        Word(String),
    }

    struct Parser {
        toks: Vec<Lexed>,
        pos: usize,
    }

    impl Parser {
        fn peek(&self) -> &Tok {
            &self.toks[self.pos].tok
        }

        fn span(&self) -> vcs::TextSpan {
            self.toks[self.pos].span
        }

        fn bump(&mut self) -> Tok {
            let tok = self.toks[self.pos].tok.clone();
            if self.pos + 1 < self.toks.len() {
                self.pos += 1;
            }
            tok
        }

        fn at_lbrace(&self) -> bool {
            matches!(self.peek(), Tok::LBrace)
        }

        fn at_rbrace(&self) -> bool {
            matches!(self.peek(), Tok::RBrace)
        }

        fn expect_word(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Word(w) => Ok(w),
                other => Err(vcs::TextError::expected(format!("expected a word, found {other:?}"), span, "word")),
            }
        }

        fn expect_keyword(&mut self, keyword: &str) -> Result<(), vcs::TextError> {
            let span = self.span();
            let word = self.expect_word()?;
            if word != keyword {
                return Err(vcs::TextError::expected(format!("expected '{keyword}', found '{word}'"), span, keyword.to_string()));
            }
            Ok(())
        }

        fn expect_lbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::LBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '{{', found {other:?}"), span, "{")),
            }
        }

        fn expect_rbrace(&mut self) -> Result<(), vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::RBrace => Ok(()),
                other => Err(vcs::TextError::expected(format!("expected '}}', found {other:?}"), span, "}")),
            }
        }

        /// 🗺️ Greedily reads `key=value` tokens (order-independent) until a token that isn't one — the
        /// generic header-field reader every construct (document/step/block/option/field) is built on.
        fn parse_kv_map(&mut self) -> Result<std::collections::HashMap<String, (FieldValue, vcs::TextSpan)>, vcs::TextError> {
            let mut map = std::collections::HashMap::new();
            loop {
                let word = match self.peek() {
                    Tok::Word(w) if w.contains('=') => w.clone(),
                    _ => break,
                };
                let span = self.span();
                self.bump();
                let (key, rest) = word.split_once('=').expect("word already checked to contain '='");
                let value = if rest.is_empty() {
                    FieldValue::Str(self.expect_str()?)
                } else {
                    FieldValue::Word(rest.to_string())
                };
                map.insert(key.to_string(), (value, span));
            }
            Ok(map)
        }

        fn expect_str(&mut self) -> Result<String, vcs::TextError> {
            let span = self.span();
            match self.bump() {
                Tok::Str(s) => Ok(s),
                other => Err(vcs::TextError::expected(format!("expected a quoted string, found {other:?}"), span, "string")),
            }
        }
    }

    type FieldMap = std::collections::HashMap<String, (FieldValue, vcs::TextSpan)>;

    fn kv_str(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Ok(s.clone()),
            Some((FieldValue::Word(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must be a quoted string"), *field_span, "string")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_opt_str(map: &FieldMap, key: &str) -> Option<String> {
        match map.get(key) {
            Some((FieldValue::Str(s), _)) => Some(s.clone()),
            _ => None,
        }
    }

    fn kv_word(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<String, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => Ok(w.clone()),
            Some((FieldValue::Str(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "word")),
            None => Err(vcs::TextError::new(format!("missing required field '{key}'"), span)),
        }
    }

    fn kv_opt_num(map: &FieldMap, key: &str) -> Option<f64> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) => w.parse::<f64>().ok(),
            _ => None,
        }
    }

    fn kv_opt_bool(map: &FieldMap, key: &str) -> Option<bool> {
        match map.get(key) {
            Some((FieldValue::Word(w), _)) if w == "true" => Some(true),
            Some((FieldValue::Word(w), _)) if w == "false" => Some(false),
            _ => None,
        }
    }

    fn kv_usize(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<usize, vcs::TextError> {
        let word = kv_word(map, key, span)?;
        word.parse::<usize>().map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a non-negative integer"), span, "usize"))
    }

    fn kv_opt_usize(map: &FieldMap, key: &str) -> Result<Option<usize>, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Word(w), field_span)) => w.parse::<usize>().map(Some).map_err(|_| vcs::TextError::expected(format!("field '{key}' must be a non-negative integer"), *field_span, "usize")),
            Some((FieldValue::Str(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must not be quoted"), *field_span, "usize")),
            None => Ok(None),
        }
    }

    /// 🧬 Reads a required arbitrary-JSON field (`ProtocolBlock::default`/`params`, `ProtocolExpr::Const`'s
    /// `value`): the field's quoted string content is itself compact JSON text, re-parsed with `serde_json`
    /// (already a workspace dependency — only the surrounding `.protocol`/op-line grammar is hand-rolled).
    fn kv_json(map: &FieldMap, key: &str, span: vcs::TextSpan) -> Result<Value, vcs::TextError> {
        let text = kv_str(map, key, span)?;
        serde_json::from_str(&text).map_err(|error| vcs::TextError::expected(format!("field '{key}' must be valid JSON: {error}"), span, "json"))
    }

    fn kv_opt_json(map: &FieldMap, key: &str) -> Result<Option<Value>, vcs::TextError> {
        match map.get(key) {
            Some((FieldValue::Str(text), field_span)) => serde_json::from_str(text).map(Some).map_err(|error| vcs::TextError::expected(format!("field '{key}' must be valid JSON: {error}"), *field_span, "json")),
            Some((FieldValue::Word(_), field_span)) => Err(vcs::TextError::expected(format!("field '{key}' must be a quoted JSON string"), *field_span, "json string")),
            None => Ok(None),
        }
    }

    fn parse_option(p: &mut Parser) -> Result<ProtocolBlockOption, vcs::TextError> {
        let span = p.span();
        p.expect_keyword("opt")?;
        let map = p.parse_kv_map()?;
        Ok(ProtocolBlockOption { value: kv_str(&map, "value", span)?, label: kv_str(&map, "label", span)? })
    }

    fn parse_vector_field(p: &mut Parser) -> Result<ProtocolVectorField, vcs::TextError> {
        let span = p.span();
        p.expect_keyword("field")?;
        let map = p.parse_kv_map()?;
        Ok(ProtocolVectorField { key: kv_str(&map, "key", span)?, label: kv_opt_str(&map, "label"), value: kv_opt_num(&map, "value") })
    }

    /// 🧮 Recursive-descent reader for one `ProtocolExpr` node: a keyword (`const`/`var`/`eq`/`and`/`or`/
    /// `truthy`), optional `key=value` fields, then — for the compound kinds only — a `{ }` body holding
    /// its nested expr(s) (positional for `eq`'s left/right and `truthy`'s inner expr, a list for `and`/`or`).
    fn parse_expr(p: &mut Parser) -> Result<ProtocolExpr, vcs::TextError> {
        let span = p.span();
        let keyword = p.expect_word()?;
        match keyword.as_str() {
            "const" => {
                let map = p.parse_kv_map()?;
                Ok(ProtocolExpr::Const { value: kv_json(&map, "value", span)? })
            }
            "var" => {
                let map = p.parse_kv_map()?;
                Ok(ProtocolExpr::Var { name: kv_str(&map, "name", span)? })
            }
            "eq" => {
                p.expect_lbrace()?;
                let left = parse_expr(p)?;
                let right = parse_expr(p)?;
                p.expect_rbrace()?;
                Ok(ProtocolExpr::Eq { left: Box::new(left), right: Box::new(right) })
            }
            "and" => {
                p.expect_lbrace()?;
                let mut items = Vec::new();
                while !p.at_rbrace() {
                    items.push(parse_expr(p)?);
                }
                p.expect_rbrace()?;
                Ok(ProtocolExpr::And { items })
            }
            "or" => {
                p.expect_lbrace()?;
                let mut items = Vec::new();
                while !p.at_rbrace() {
                    items.push(parse_expr(p)?);
                }
                p.expect_rbrace()?;
                Ok(ProtocolExpr::Or { items })
            }
            "truthy" => {
                p.expect_lbrace()?;
                let expr = parse_expr(p)?;
                p.expect_rbrace()?;
                Ok(ProtocolExpr::Truthy { expr: Box::new(expr) })
            }
            other => Err(vcs::TextError::expected(format!("unknown expr kind '{other}'"), span, "const|var|eq|and|or|truthy")),
        }
    }

    /// 🧱 Reads one `block kind=... id=... label=... ...` header (every optional property as a
    /// `key=value` field) followed by an optional `{ }` body holding whichever of `options`/`fields`/
    /// `condition` are present (their presence in the body — not just an empty list — is what
    /// distinguishes `Some(vec![])` from `None`).
    fn parse_block(p: &mut Parser) -> Result<ProtocolBlock, vcs::TextError> {
        let span = p.span();
        p.expect_keyword("block")?;
        let map = p.parse_kv_map()?;
        let kind = kv_word(&map, "kind", span)?;
        let id = kv_str(&map, "id", span)?;
        let label = kv_str(&map, "label", span)?;
        let description = kv_opt_str(&map, "description");
        let required = kv_opt_bool(&map, "required");
        let placeholder = kv_opt_str(&map, "placeholder");
        let default = kv_opt_json(&map, "default")?;
        let min = kv_opt_num(&map, "min");
        let max = kv_opt_num(&map, "max");
        let step = kv_opt_num(&map, "step");
        let unit = kv_opt_str(&map, "unit");
        let text = kv_opt_str(&map, "text");
        let schema = kv_opt_str(&map, "schema");
        let src = kv_opt_str(&map, "src");
        let accept = kv_opt_str(&map, "accept");
        let fixture_slug = kv_opt_str(&map, "fixtureSlug");
        let params = kv_opt_json(&map, "params")?;

        let mut options = None;
        let mut fields = None;
        let mut condition = None;
        if p.at_lbrace() {
            p.bump();
            while !p.at_rbrace() {
                let keyword = match p.peek() {
                    Tok::Word(w) => w.clone(),
                    _ => return Err(vcs::TextError::expected("expected 'options', 'fields' or 'condition'", p.span(), "options|fields|condition")),
                };
                match keyword.as_str() {
                    "options" => {
                        p.bump();
                        p.expect_lbrace()?;
                        let mut items = Vec::new();
                        while !p.at_rbrace() {
                            items.push(parse_option(p)?);
                        }
                        p.expect_rbrace()?;
                        options = Some(items);
                    }
                    "fields" => {
                        p.bump();
                        p.expect_lbrace()?;
                        let mut items = Vec::new();
                        while !p.at_rbrace() {
                            items.push(parse_vector_field(p)?);
                        }
                        p.expect_rbrace()?;
                        fields = Some(items);
                    }
                    "condition" => {
                        p.bump();
                        condition = Some(parse_expr(p)?);
                    }
                    other => return Err(vcs::TextError::expected(format!("unknown block section '{other}'"), p.span(), "options|fields|condition")),
                }
            }
            p.expect_rbrace()?;
        }
        Ok(ProtocolBlock { id, label, kind, description, required, placeholder, default, min, max, step, unit, text, options, fields, schema, src, accept, fixture_slug, params, condition })
    }

    fn parse_step(p: &mut Parser) -> Result<ProtocolStep, vcs::TextError> {
        let span = p.span();
        p.expect_keyword("step")?;
        let map = p.parse_kv_map()?;
        let id = kv_str(&map, "id", span)?;
        let title = kv_str(&map, "title", span)?;
        let description = kv_opt_str(&map, "description");
        p.expect_lbrace()?;
        let mut blocks = Vec::new();
        while !p.at_rbrace() {
            blocks.push(parse_block(p)?);
        }
        p.expect_rbrace()?;
        Ok(ProtocolStep { id, title, description, blocks })
    }

    /// 📥 Parses a full `.protocol` document: `protocol id=/schema=/version=/title=` header, then a
    /// mandatory `steps { step ... }` section (see {@link print_document} for the mirrored grammar).
    pub(super) fn parse_document(text: &str) -> Result<ProtocolSpec, vcs::TextError> {
        let toks = lex(text)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        p.expect_keyword("protocol")?;
        let map = p.parse_kv_map()?;
        let id = kv_str(&map, "id", span)?;
        let schema = kv_str(&map, "schema", span)?;
        let version = kv_str(&map, "version", span)?;
        let title = kv_opt_str(&map, "title");
        p.expect_keyword("steps")?;
        p.expect_lbrace()?;
        let mut steps = Vec::new();
        while !p.at_rbrace() {
            steps.push(parse_step(&mut p)?);
        }
        p.expect_rbrace()?;
        Ok(ProtocolSpec { schema, id, version, title, steps })
    }

    /// ⚡ Parses one op-log line: a keyword (`add-step`/`remove-step`/`move-step`/`add-block`/
    /// `remove-block`/`move-block`/`update-block`/`update-step`/`update-protocol`), its own `key=value`
    /// fields, then — for the four operations that carry a whole `step`/`block` — that nested construct.
    pub(super) fn parse_operation(line: &str) -> Result<super::ProtocolOperation, vcs::TextError> {
        use super::ProtocolOperation;
        let toks = lex(line)?;
        let mut p = Parser { toks, pos: 0 };
        let span = p.span();
        let keyword = p.expect_word()?;
        match keyword.as_str() {
            "add-step" => {
                let map = p.parse_kv_map()?;
                let index = kv_opt_usize(&map, "index")?;
                let step = parse_step(&mut p)?;
                Ok(ProtocolOperation::AddStep { step, index })
            }
            "remove-step" => {
                let map = p.parse_kv_map()?;
                Ok(ProtocolOperation::RemoveStep { step_id: kv_str(&map, "id", span)? })
            }
            "move-step" => {
                let map = p.parse_kv_map()?;
                Ok(ProtocolOperation::MoveStep { step_id: kv_str(&map, "id", span)?, index: kv_usize(&map, "index", span)? })
            }
            "add-block" => {
                let map = p.parse_kv_map()?;
                let step_id = kv_str(&map, "step", span)?;
                let index = kv_opt_usize(&map, "index")?;
                let block = parse_block(&mut p)?;
                Ok(ProtocolOperation::AddBlock { step_id, block, index })
            }
            "remove-block" => {
                let map = p.parse_kv_map()?;
                Ok(ProtocolOperation::RemoveBlock { step_id: kv_str(&map, "step", span)?, block_id: kv_str(&map, "block", span)? })
            }
            "move-block" => {
                let map = p.parse_kv_map()?;
                Ok(ProtocolOperation::MoveBlock {
                    block_id: kv_str(&map, "block", span)?,
                    from_step_id: kv_str(&map, "from", span)?,
                    to_step_id: kv_str(&map, "to", span)?,
                    index: kv_usize(&map, "index", span)?,
                })
            }
            "update-block" => {
                let map = p.parse_kv_map()?;
                let step_id = kv_str(&map, "step", span)?;
                let block = parse_block(&mut p)?;
                Ok(ProtocolOperation::UpdateBlock { step_id, block })
            }
            "update-step" => {
                let step = parse_step(&mut p)?;
                Ok(ProtocolOperation::UpdateStep { step })
            }
            "update-protocol" => {
                let map = p.parse_kv_map()?;
                Ok(ProtocolOperation::UpdateProtocol { title: kv_opt_str(&map, "title") })
            }
            other => Err(vcs::TextError::expected(format!("unknown operation '{other}'"), span, "add-step|remove-step|move-step|add-block|remove-block|move-block|update-block|update-step|update-protocol")),
        }
    }
    //#endregion Parser

    //#region Printer
    fn quote(value: &str) -> String {
        let mut out = String::with_capacity(value.len() + 2);
        out.push('"');
        for ch in value.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                _ => out.push(ch),
            }
        }
        out.push('"');
        out
    }

    fn fmt_num(value: f64) -> String {
        value.to_string()
    }

    fn quote_json(value: &Value) -> String {
        quote(&serde_json::to_string(value).unwrap_or_else(|_| "null".into()))
    }

    fn indent_str(depth: usize) -> String {
        "  ".repeat(depth)
    }

    /// 🧱 Wraps `items` (each already rendered, without its own leading indentation) in `{ }`, one per
    /// line indented at `depth + 1` when `pretty`, or space-joined on one line otherwise. The single
    /// nesting primitive every braced construct (steps/blocks/options/fields/expr bodies) is built
    /// from, so pretty-printed indentation stays correct at arbitrary nesting depth.
    fn wrap_body(items: &[String], depth: usize, pretty: bool) -> String {
        if pretty {
            let inner_pad = indent_str(depth + 1);
            let outer_pad = indent_str(depth);
            let body: String = items.iter().map(|item| format!("{inner_pad}{item}\n")).collect();
            format!("{{\n{body}{outer_pad}}}")
        } else {
            format!("{{ {} }}", items.join(" "))
        }
    }

    fn print_option(option: &ProtocolBlockOption) -> String {
        format!("opt value={} label={}", quote(&option.value), quote(&option.label))
    }

    fn print_vector_field(field: &ProtocolVectorField) -> String {
        let mut out = format!("field key={}", quote(&field.key));
        if let Some(label) = &field.label {
            out.push_str(&format!(" label={}", quote(label)));
        }
        if let Some(value) = field.value {
            out.push_str(&format!(" value={}", fmt_num(value)));
        }
        out
    }

    fn print_expr(expr: &ProtocolExpr, depth: usize, pretty: bool) -> String {
        match expr {
            ProtocolExpr::Const { value } => format!("const value={}", quote_json(value)),
            ProtocolExpr::Var { name } => format!("var name={}", quote(name)),
            ProtocolExpr::Eq { left, right } => {
                let items = vec![print_expr(left, depth + 1, pretty), print_expr(right, depth + 1, pretty)];
                format!("eq {}", wrap_body(&items, depth, pretty))
            }
            ProtocolExpr::And { items } => {
                let printed: Vec<String> = items.iter().map(|item| print_expr(item, depth + 1, pretty)).collect();
                format!("and {}", wrap_body(&printed, depth, pretty))
            }
            ProtocolExpr::Or { items } => {
                let printed: Vec<String> = items.iter().map(|item| print_expr(item, depth + 1, pretty)).collect();
                format!("or {}", wrap_body(&printed, depth, pretty))
            }
            ProtocolExpr::Truthy { expr } => {
                let items = vec![print_expr(expr, depth + 1, pretty)];
                format!("truthy {}", wrap_body(&items, depth, pretty))
            }
        }
    }

    fn print_block(block: &ProtocolBlock, depth: usize, pretty: bool) -> String {
        let mut header = format!("block kind={} id={} label={}", block.kind, quote(&block.id), quote(&block.label));
        if let Some(description) = &block.description {
            header.push_str(&format!(" description={}", quote(description)));
        }
        if let Some(required) = block.required {
            header.push_str(&format!(" required={required}"));
        }
        if let Some(placeholder) = &block.placeholder {
            header.push_str(&format!(" placeholder={}", quote(placeholder)));
        }
        if let Some(default) = &block.default {
            header.push_str(&format!(" default={}", quote_json(default)));
        }
        if let Some(min) = block.min {
            header.push_str(&format!(" min={}", fmt_num(min)));
        }
        if let Some(max) = block.max {
            header.push_str(&format!(" max={}", fmt_num(max)));
        }
        if let Some(step) = block.step {
            header.push_str(&format!(" step={}", fmt_num(step)));
        }
        if let Some(unit) = &block.unit {
            header.push_str(&format!(" unit={}", quote(unit)));
        }
        if let Some(text) = &block.text {
            header.push_str(&format!(" text={}", quote(text)));
        }
        if let Some(schema) = &block.schema {
            header.push_str(&format!(" schema={}", quote(schema)));
        }
        if let Some(src) = &block.src {
            header.push_str(&format!(" src={}", quote(src)));
        }
        if let Some(accept) = &block.accept {
            header.push_str(&format!(" accept={}", quote(accept)));
        }
        if let Some(fixture_slug) = &block.fixture_slug {
            header.push_str(&format!(" fixtureSlug={}", quote(fixture_slug)));
        }
        if let Some(params) = &block.params {
            header.push_str(&format!(" params={}", quote_json(params)));
        }

        let mut items = Vec::new();
        if let Some(options) = &block.options {
            let opts: Vec<String> = options.iter().map(print_option).collect();
            items.push(format!("options {}", wrap_body(&opts, depth + 1, pretty)));
        }
        if let Some(fields) = &block.fields {
            let flds: Vec<String> = fields.iter().map(print_vector_field).collect();
            items.push(format!("fields {}", wrap_body(&flds, depth + 1, pretty)));
        }
        if let Some(condition) = &block.condition {
            items.push(format!("condition {}", print_expr(condition, depth + 1, pretty)));
        }
        if items.is_empty() {
            header
        } else {
            format!("{header} {}", wrap_body(&items, depth, pretty))
        }
    }

    fn print_step(step: &ProtocolStep, depth: usize, pretty: bool) -> String {
        let mut header = format!("step id={} title={}", quote(&step.id), quote(&step.title));
        if let Some(description) = &step.description {
            header.push_str(&format!(" description={}", quote(description)));
        }
        let items: Vec<String> = step.blocks.iter().map(|block| print_block(block, depth + 1, pretty)).collect();
        format!("{header} {}", wrap_body(&items, depth, pretty))
    }

    /// 📤 Renders `spec` as `protocol id=/schema=/version=/title=` followed by its `steps { }` section
    /// (see {@link parse_document} for the mirrored grammar).
    pub(super) fn print_document(spec: &ProtocolSpec, pretty: bool) -> String {
        let mut header = format!("protocol id={} schema={} version={}", quote(&spec.id), quote(&spec.schema), quote(&spec.version));
        if let Some(title) = &spec.title {
            header.push_str(&format!(" title={}", quote(title)));
        }
        let steps: Vec<String> = spec.steps.iter().map(|step| print_step(step, 1, pretty)).collect();
        let steps_section = format!("steps {}", wrap_body(&steps, 0, pretty));
        if pretty {
            format!("{header}\n{steps_section}\n")
        } else {
            format!("{header} {steps_section}")
        }
    }

    /// ⚡ Renders one `ProtocolOperation` as a single line — the four operations carrying a whole
    /// `step`/`block` reuse the compact (space-joined) form of {@link print_step}/{@link print_block}.
    pub(super) fn print_operation(operation: &super::ProtocolOperation) -> String {
        use super::ProtocolOperation;
        match operation {
            ProtocolOperation::AddStep { step, index } => {
                let mut header = "add-step".to_string();
                if let Some(index) = index {
                    header.push_str(&format!(" index={index}"));
                }
                format!("{header} {}", print_step(step, 0, false))
            }
            ProtocolOperation::RemoveStep { step_id } => format!("remove-step id={}", quote(step_id)),
            ProtocolOperation::MoveStep { step_id, index } => format!("move-step id={} index={index}", quote(step_id)),
            ProtocolOperation::AddBlock { step_id, block, index } => {
                let mut header = format!("add-block step={}", quote(step_id));
                if let Some(index) = index {
                    header.push_str(&format!(" index={index}"));
                }
                format!("{header} {}", print_block(block, 0, false))
            }
            ProtocolOperation::RemoveBlock { step_id, block_id } => format!("remove-block step={} block={}", quote(step_id), quote(block_id)),
            ProtocolOperation::MoveBlock { block_id, from_step_id, to_step_id, index } => {
                format!("move-block block={} from={} to={} index={index}", quote(block_id), quote(from_step_id), quote(to_step_id))
            }
            ProtocolOperation::UpdateBlock { step_id, block } => format!("update-block step={} {}", quote(step_id), print_block(block, 0, false)),
            ProtocolOperation::UpdateStep { step } => format!("update-step {}", print_step(step, 0, false)),
            ProtocolOperation::UpdateProtocol { title } => match title {
                Some(title) => format!("update-protocol title={}", quote(title)),
                None => "update-protocol".to_string(),
            },
        }
    }
    //#endregion Printer
}

impl vcs::DocumentDsl for ProtocolSpec {
    const EXTENSION: &'static str = "protocol";

    fn parse_dsl(text: &str) -> Result<Self, vcs::TextError> {
        protocol_text::parse_document(text)
    }

    fn print_dsl(&self) -> String {
        protocol_text::print_document(self, true)
    }
}
//#endregion 🔖Dsl

//#region 🔖OpText
impl vcs::OpText for ProtocolOperation {
    fn parse_op(line: &str) -> Result<Self, vcs::TextError> {
        protocol_text::parse_operation(line)
    }

    fn print_op(&self) -> String {
        protocol_text::print_operation(self)
    }
}
//#endregion 🔖OpText

//#region 🔖GenerationForms
pub mod generation_forms {
    //! 🧬 Shared Generate-mode state, CRUD, and declarative UI helpers for answering a `ProtocolSpec` as
    //! a set of named "generations" (parameter presets) — moved here (from `semio-framework-plugin`) since
    //! it is typed end-to-end on `ProtocolSpec`/`ProtocolBlock`, i.e. protocol-domain code, not SDK code.

    use super::{default_value_for_block, flatten_protocol_blocks, is_block_visible, ProtocolBlock, ProtocolSpec};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Map, Value};
    use ui_wgpu::{
        build_text_editor_scene, ui_stack_vertical, ui_text, ActionDescriptor, TextEditorScene, UiControlNode, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSelectItem, UiSelectNode, UiSliderNode, UiToggleNode, UiTreeItemAction, UiTreeItemNode, UiTreeNode,
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

    //#region 🔖Operations
    /// @emoji 🧬 Typed, invertible Generate-mode operation vocabulary. WS-F embeds this as a variant in
    /// `forms/module/procedural`'s own `Operation` enum so generation edits flow through the document store with
    /// true inverses (replacing the in-place-mutating CRUD helpers as the document mutation surface).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum GenerationOperation {
        Add { generation: FormGeneration },
        Remove { id: String },
        Rename { id: String, name: String },
        UpdateValues { id: String, question_id: String, value: Value },
    }

    /// @emoji 🎛️ Maps a Generate-mode action id to the document operations it produces, or `None` for
    /// non-document (view) actions like `selectGeneration`. Pure — reads `state`/`spec` but mutates
    /// nothing; the caller applies the returned operations through its store.
    pub fn generation_operations(action: &str, args: Option<&Value>, state: &GenerationPlayState, spec: &ProtocolSpec) -> Option<Vec<GenerationOperation>> {
        let arg_str = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "addGeneration" => Some(vec![GenerationOperation::Add { generation: FormGeneration { id: next_generation_id(&state.generations), name: next_generation_name(&state.generations), values: initial_generation_values(spec) } }]),
            "removeGeneration" => arg_str("id").map(|id| vec![GenerationOperation::Remove { id }]),
            "renameGeneration" => {
                let id = arg_str("id")?;
                let name = arg_str("name")?;
                Some(vec![GenerationOperation::Rename { id, name }])
            }
            "updateGenerationValues" => {
                let id = arg_str("generationId").or_else(|| state.selected_generation_id.clone())?;
                let question_id = arg_str("questionId")?;
                let value = args.and_then(|value| value.get("value")).cloned()?;
                Some(vec![GenerationOperation::UpdateValues { id, question_id, value }])
            }
            _ => None,
        }
    }

    /// @emoji ▶️ Applies a {@link GenerationOperation} to `state` in place.
    pub fn apply_generation_operation(state: &mut GenerationPlayState, operation: &GenerationOperation) {
        match operation {
            GenerationOperation::Add { generation } => {
                state.generations.push(generation.clone());
                state.selected_generation_id = Some(generation.id.clone());
            }
            GenerationOperation::Remove { id } => remove_generation(state, id),
            GenerationOperation::Rename { id, name } => rename_generation(state, id, name),
            GenerationOperation::UpdateValues { id, question_id, value } => update_generation_values(state, id, question_id, value.clone()),
        }
    }

    /// @emoji ↩️ Computes the inverse of a {@link GenerationOperation} from the pre-state `state`.
    pub fn invert_generation_operation(state: &GenerationPlayState, operation: &GenerationOperation) -> Vec<GenerationOperation> {
        match operation {
            GenerationOperation::Add { generation } => vec![GenerationOperation::Remove { id: generation.id.clone() }],
            GenerationOperation::Remove { id } => state.generations.iter().find(|entry| entry.id == *id).map(|entry| vec![GenerationOperation::Add { generation: entry.clone() }]).unwrap_or_default(),
            GenerationOperation::Rename { id, .. } => state.generations.iter().find(|entry| entry.id == *id).map(|entry| vec![GenerationOperation::Rename { id: id.clone(), name: entry.name.clone() }]).unwrap_or_default(),
            GenerationOperation::UpdateValues { id, question_id, .. } => state
                .generations
                .iter()
                .find(|entry| entry.id == *id)
                .map(|entry| vec![GenerationOperation::UpdateValues { id: id.clone(), question_id: question_id.clone(), value: entry.values.get(question_id).cloned().unwrap_or(Value::Null) }])
                .unwrap_or_default(),
        }
    }
    //#endregion 🔖Operations

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
                    presence: UiPresence::selected(selected_id == Some(generation.id.as_str())),
                    default_open: None,
                    action: Some(generation_action(controller_id, "selectGeneration", Some(json!({ "id": generation.id })))),
                    hover_action: None,
                    unhover_action: None,
                    actions: Some(actions),
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    dimmed: None,
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
                    presence: UiPresence::default(),
                    default_open: None,
                    action: None,
                    hover_action: None,
                    unhover_action: None,
                    actions: None,
                    draggable: None,
                    drag_data: None,
                    items: None,
                    control: None,
                    dimmed: None,
                }]
            } else {
                items
            },
            presence: UiPresence::default(),
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
                presence: UiPresence::default(),
                default_open: None,
                action: Some(generation_action(controller_id, "addGeneration", None)),
                hover_action: None,
                unhover_action: None,
                actions: None,
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: None,
            }],
            presence: UiPresence::default(),
        });
        UiNode::Tree(UiTreeNode {
            sections,
            presence: UiPresence::default(),
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
                presence: UiPresence::default(),
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
                presence: UiPresence::default(),
            }),
            "slider" => UiControlNode::Slider(UiSliderNode {
                id: format!("{field_id}.slider"),
                value: value.as_f64().unwrap_or_else(|| question.min.unwrap_or(0.0)),
                min: question.min.unwrap_or(0.0),
                max: question.max.unwrap_or(100.0),
                step: question.step.unwrap_or(1.0),
                on_change: on_change(),
                unit: None,
                presence: UiPresence::default(),
            }),
            "boolean" => UiControlNode::Toggle(UiToggleNode { id: format!("{field_id}.toggle"), icon_id: "toggle-left".into(), text: Some(question.label.clone()), on_change: on_change(), presence: UiPresence::selected(value.as_bool().unwrap_or(false)) }),
            "single" => {
                let items = question.options.as_ref().map(|options| options.iter().map(|option| UiSelectItem { value: option.value.clone(), label: option.label.clone() }).collect()).unwrap_or_default();
                UiControlNode::Select(UiSelectNode { id: format!("{field_id}.select"), value: value.as_str().unwrap_or_default().to_string(), items, placeholder: question.placeholder.clone(), on_change: on_change(), presence: UiPresence::default() })
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
                                presence: UiPresence::default(),
                            })),
                            description: None,
                            required: None,
                            error: None,
                            presence: UiPresence::default(),
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
                presence: UiPresence::default(),
            }),
        };
        Some(UiNode::Field(UiFieldNode { id: field_id, label: question.label.clone(), child: Box::new(ui_wgpu::ui_control_to_node(child)), description: None, required: None, error: None, presence: UiPresence::default() }))
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
    //! 🧩 Shared strict-list, Blockly-like builder engine: generic step/block CRUD operation-builders and
    //! [`BlockListScene`] rendering, reused by `protocol-plugin` (standalone) and `forms-plugin`
    //! (embedded Blueprint mode). Block-kind-specific property editing stays with the host app. Moved
    //! here (from `semio-framework-plugin`) since it is entirely protocol-domain code.

    use super::{ProtocolBlock, ProtocolOperation, ProtocolSpec, ProtocolStep};
    use serde_json::Value;
    use ui_wgpu::{ActionDescriptor, BlockListScene, BlockPaletteEntry, IconName, SurfaceKind, UiComponentSceneNode, UiNode, UiPresence};

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
    pub fn add_step_operation(spec: &ProtocolSpec, step_id: String) -> ProtocolOperation {
        ProtocolOperation::AddStep { step: ProtocolStep { id: step_id, title: format!("Step {}", spec.steps.len() + 1), description: None, blocks: Vec::new() }, index: None }
    }

    pub fn remove_step_operation(step_id: &str) -> ProtocolOperation {
        ProtocolOperation::RemoveStep { step_id: step_id.into() }
    }

    pub fn move_step_operation(step_id: &str, index: usize) -> ProtocolOperation {
        ProtocolOperation::MoveStep { step_id: step_id.into(), index }
    }

    pub fn add_block_operation(step_id: &str, block: ProtocolBlock, index: Option<usize>) -> ProtocolOperation {
        ProtocolOperation::AddBlock { step_id: step_id.into(), block, index }
    }

    pub fn remove_block_operation(step_id: &str, block_id: &str) -> ProtocolOperation {
        ProtocolOperation::RemoveBlock { step_id: step_id.into(), block_id: block_id.into() }
    }

    pub fn move_block_operation(block_id: &str, from_step_id: &str, to_step_id: &str, index: usize) -> ProtocolOperation {
        ProtocolOperation::MoveBlock { block_id: block_id.into(), from_step_id: from_step_id.into(), to_step_id: to_step_id.into(), index }
    }

    pub fn update_protocol_title_operation(title: Option<String>) -> ProtocolOperation {
        ProtocolOperation::UpdateProtocol { title }
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
        entries.extend(extensions.iter().map(|(kind, label, icon_id)| BlockPaletteEntry { block_kind: kind.clone(), label: label.clone(), icon_id: IconName::from(icon_id.as_str()) }));
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
            presence: UiPresence::default(),
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
            let operation = add_step_operation(&spec, "step-2".into());
            assert_eq!(operation, ProtocolOperation::AddStep { step: ProtocolStep { id: "step-2".into(), title: "Step 2".into(), description: None, blocks: Vec::new() }, index: None });
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
        let operation = ProtocolOperation::UpdateProtocol { title: Some("Renamed".into()) };
        let next = apply_protocol_edit_operation(&spec, &operation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
        let inverse = operation.backwards(&spec);
        assert_eq!(inverse, vec![ProtocolOperation::UpdateProtocol { title: spec.title.clone() }]);
        let reverted = inverse.iter().fold(next.clone(), |current, operation| apply_protocol_edit_operation(&current, operation));
        assert_eq!(reverted.title, spec.title);
        assert_eq!(operation.diff(&spec).apply(&spec).title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn add_step_op_replays() {
        let mut store = ProtocolStore::new(create_document_vcs_envelope(PROTOCOL_DOCUMENT_SCHEMA, "protocol", empty_protocol_projection(), None));
        let step = ProtocolStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        let backwards = store.projection().expect("projection");
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![ProtocolOperation::AddStep { step: step.clone(), index: None }], description: None }).expect("apply");
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

    //#region 🔖DslAndOpText
    fn minimal_block(id: &str, kind: &str) -> ProtocolBlock {
        ProtocolBlock {
            id: id.into(),
            label: format!("Label {id}"),
            kind: kind.into(),
            description: None,
            required: None,
            placeholder: None,
            default: None,
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
        }
    }

    /// 🧱 A block with EVERY optional property populated (including nested `options`/`fields` and a
    /// deeply nested `condition` exercising every `ProtocolExpr` variant) — the DSL round-trip fixture.
    fn fully_populated_block() -> ProtocolBlock {
        ProtocolBlock {
            id: "b-full".into(),
            label: "Team Size".into(),
            kind: "slider".into(),
            description: Some("How many people?".into()),
            required: Some(true),
            placeholder: Some("Enter a number".into()),
            default: Some(serde_json::json!(5)),
            min: Some(1.0),
            max: Some(50.0),
            step: Some(1.0),
            unit: Some("people".into()),
            text: Some("Some note text\nwith a newline".into()),
            options: Some(vec![ProtocolBlockOption { value: "red".into(), label: "Red".into() }, ProtocolBlockOption { value: "blue".into(), label: "Blue".into() }]),
            fields: Some(vec![ProtocolVectorField { key: "x".into(), label: Some("X".into()), value: Some(1.5) }, ProtocolVectorField { key: "y".into(), label: None, value: None }]),
            schema: Some("solid.step".into()),
            src: Some("https://example.com/img.png".into()),
            accept: Some("image/*".into()),
            fixture_slug: Some("hexagonal-mushroom-column".into()),
            params: Some(serde_json::json!({ "height": 6.0, "nested": { "a": [1, 2, "three\"quoted"] } })),
            condition: Some(ProtocolExpr::And {
                items: vec![
                    ProtocolExpr::Truthy { expr: Box::new(ProtocolExpr::Var { name: "show-team-size".into() }) },
                    ProtocolExpr::Eq { left: Box::new(ProtocolExpr::Var { name: "mode".into() }), right: Box::new(ProtocolExpr::Const { value: serde_json::json!("advanced") }) },
                    ProtocolExpr::Or { items: vec![ProtocolExpr::Var { name: "a".into() }, ProtocolExpr::Var { name: "b".into() }] },
                ],
            }),
        }
    }

    fn sample_spec() -> ProtocolSpec {
        ProtocolSpec {
            schema: PROTOCOL_DOCUMENT_SCHEMA.into(),
            id: "recipe".into(),
            version: "1".into(),
            title: Some("Recipe".into()),
            steps: vec![
                ProtocolStep { id: "s1".into(), title: "Basics".into(), description: Some("First step".into()), blocks: vec![minimal_block("b1", "text"), fully_populated_block()] },
                ProtocolStep { id: "s2".into(), title: "Review".into(), description: None, blocks: Vec::new() },
            ],
        }
    }

    #[test]
    fn empty_protocol_projection_dsl_round_trips() {
        vcs::test_support::assert_dsl_round_trip(&empty_protocol_projection());
    }

    #[test]
    fn sample_spec_dsl_round_trips() {
        vcs::test_support::assert_dsl_round_trip(&sample_spec());
    }

    #[test]
    fn add_step_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::AddStep { step: ProtocolStep { id: "step-2".into(), title: "Review".into(), description: Some("desc".into()), blocks: vec![minimal_block("b1", "text")] }, index: Some(1) });
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::AddStep { step: ProtocolStep { id: "step-3".into(), title: "Final".into(), description: None, blocks: Vec::new() }, index: None });
    }

    #[test]
    fn remove_step_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::RemoveStep { step_id: "s1".into() });
    }

    #[test]
    fn move_step_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::MoveStep { step_id: "s1".into(), index: 2 });
    }

    #[test]
    fn add_block_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::AddBlock { step_id: "s1".into(), block: fully_populated_block(), index: Some(0) });
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::AddBlock { step_id: "s1".into(), block: minimal_block("b2", "boolean"), index: None });
    }

    #[test]
    fn remove_block_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::RemoveBlock { step_id: "s1".into(), block_id: "b1".into() });
    }

    #[test]
    fn move_block_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::MoveBlock { block_id: "b1".into(), from_step_id: "s1".into(), to_step_id: "s2".into(), index: 3 });
    }

    #[test]
    fn update_block_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::UpdateBlock { step_id: "s1".into(), block: fully_populated_block() });
    }

    #[test]
    fn update_step_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::UpdateStep { step: ProtocolStep { id: "s1".into(), title: "Basics".into(), description: Some("d".into()), blocks: vec![fully_populated_block()] } });
    }

    #[test]
    fn update_protocol_op_round_trips() {
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::UpdateProtocol { title: Some("Renamed".into()) });
        vcs::test_support::assert_op_line_round_trip(&ProtocolOperation::UpdateProtocol { title: None });
    }

    #[test]
    fn document_text_round_trips_after_applied_operations() {
        let mut store = ProtocolStore::new(create_document_vcs_envelope(PROTOCOL_DOCUMENT_SCHEMA, "protocol", empty_protocol_projection(), None));
        store
            .dispatch(DocumentVcsCommand::Apply {
                operations: vec![ProtocolOperation::AddStep { step: ProtocolStep { id: "step-2".into(), title: "Review".into(), description: None, blocks: Vec::new() }, index: None }],
                description: None,
            })
            .expect("add step");
        store
            .dispatch(DocumentVcsCommand::Apply { operations: vec![ProtocolOperation::AddBlock { step_id: "step-2".into(), block: fully_populated_block(), index: None }], description: None })
            .expect("add block");
        store.dispatch(DocumentVcsCommand::Apply { operations: vec![ProtocolOperation::UpdateProtocol { title: Some("Recipe".into()) }], description: None }).expect("update title");
        vcs::test_support::assert_document_text_round_trip(&store);
    }
    //#endregion 🔖DslAndOpText
}
//#endregion 🧪Tests
