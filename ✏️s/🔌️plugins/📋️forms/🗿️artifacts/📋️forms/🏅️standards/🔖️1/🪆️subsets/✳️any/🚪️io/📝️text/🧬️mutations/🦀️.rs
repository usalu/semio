//! ⚡️ Forms artifact — hand-rolled `OpText`/`OpBinary` for `FormMutation`. `#[derive(dsl::Mutations)]`
//! only generates `Mutation`/`SemanticMutation` (see `../../🧬️mutations/🦀️.rs`'s `🔖️FormMutation` region)
//! — the wire-text/wire-binary codecs stay handcrafted here, one keyword per semantic verb, grammar
//! `keyword key1=value1 key2=value2 ...`. `🦀️.rs`'s `op` shim re-exports this module's `*`
//! (constants only — the trait impls below attach directly to `FormMutation`).

pub use crate::mutations::FormMutation;

use crate::mutations::{
    change_form_title::mutation::ChangeFormTitle, change_step_description::mutation::ChangeStepDescription, create_block::mutation::CreateBlock, create_step::mutation::CreateStep, delete_block::mutation::DeleteBlock,
    delete_step::mutation::DeleteStep, move_block_to_step::mutation::MoveBlockToStep, rename_step::mutation::RenameStep, reorder_step::mutation::ReorderStep, replace_block::mutation::ReplaceBlock,
    change_block_field::mutation::{BlockField, ChangeBlockField},
};
use crate::{FormQuestion, FormStep};
use crate::mutations::{commit_response::mutation::CommitResponse, discard_response::mutation::DiscardResponse};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's text form stays space-free and tokenizable by [`tokenize_args`].
fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_str(s)?))
    }
}
fn enc_usize(v: usize) -> String {
    v.to_string()
}
fn dec_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
fn enc_opt_usize(v: &Option<usize>) -> String {
    match v {
        Some(x) => enc_usize(*x),
        None => "-".to_string(),
    }
}
fn dec_opt_usize(s: &str) -> Result<Option<usize>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_usize(s)?))
    }
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because step titles/block labels/JSON payloads may contain spaces.
fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️StructCodec
/// 🌳️ Whole-`FormStep`/`FormQuestion` text form — a quoted JSON string (both already derive
/// `ToValue`/`FromValue`) rather than a second handcrafted step/block grammar; `enc_str`/
/// `dec_str`'s backslash/quote escaping round-trips it byte-for-byte.
pub(crate) fn enc_step(step: &FormStep) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(step))
}
pub(crate) fn dec_step(s: &str) -> Result<FormStep, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.into_message())
}
pub(crate) fn enc_block(block: &FormQuestion) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(block))
}
pub(crate) fn dec_block(s: &str) -> Result<FormQuestion, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.into_message())
}
fn dec_change(s: &str) -> Result<BlockField, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.into_message())
}
//#endregion 🔖️StructCodec

//#region 🔖️OpText
fn print_forms_mutation(mutation: &FormMutation) -> String {
    match mutation {
        FormMutation::CreateStep(p) => format!("create-step step={} index={}", enc_step(&p.step), enc_opt_usize(&p.index)),
        FormMutation::DeleteStep(p) => format!("delete-step id={}", enc_str(&p.id)),
        FormMutation::ReorderStep(p) => format!("reorder-step id={} to-index={}", enc_str(&p.id), enc_usize(p.to_index)),
        FormMutation::RenameStep(p) => format!("rename-step id={} new-title={}", enc_str(&p.id), enc_str(&p.new_title)),
        FormMutation::ChangeStepDescription(p) => format!("change-step-description id={} new-description={}", enc_str(&p.id), enc_opt_str(&p.new_description)),
        FormMutation::CreateBlock(p) => format!("create-block step-id={} block={} index={}", enc_str(&p.step_id), enc_block(&p.block), enc_opt_usize(&p.index)),
        FormMutation::DeleteBlock(p) => format!("delete-block step-id={} id={}", enc_str(&p.step_id), enc_str(&p.id)),
        FormMutation::MoveBlockToStep(p) => format!("move-block-to-step step-id={} block-id={} to-step-id={} index={}", enc_str(&p.step_id), enc_str(&p.block_id), enc_str(&p.to_step_id), enc_usize(p.index)),
        FormMutation::ReplaceBlock(p) => format!("replace-block step-id={} block={}", enc_str(&p.step_id), enc_block(&p.block)),
        FormMutation::ChangeBlockField(p) => format!("change-block-field block-id={} change={}", enc_str(&p.block_id), enc_str(&semio_framework_pack_json::to_json_string(&p.change))),
        FormMutation::CommitResponse(p) => format!("commit-response response={} index={}", enc_str(&semio_framework_pack_json::to_json_string(&p.response)), enc_opt_usize(&p.index)),
        FormMutation::DiscardResponse(p) => format!("discard-response id={}", enc_str(&p.id)),
        FormMutation::ChangeFormTitle(p) => format!("change-form-title new-title={}", enc_opt_str(&p.new_title)),
    }
}

fn parse_forms_mutation(line: &str) -> Result<FormMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("forms mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "create-step" => Ok(FormMutation::CreateStep(CreateStep { step: dec_step(&arg("step")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "delete-step" => Ok(FormMutation::DeleteStep(DeleteStep { id: dec_str(&arg("id")?)? })),
        "reorder-step" => Ok(FormMutation::ReorderStep(ReorderStep { id: dec_str(&arg("id")?)?, to_index: dec_usize(&arg("to-index")?)? })),
        "rename-step" => Ok(FormMutation::RenameStep(RenameStep { id: dec_str(&arg("id")?)?, new_title: dec_str(&arg("new-title")?)? })),
        "change-step-description" => Ok(FormMutation::ChangeStepDescription(ChangeStepDescription { id: dec_str(&arg("id")?)?, new_description: dec_opt_str(&arg("new-description")?)? })),
        "create-block" => Ok(FormMutation::CreateBlock(CreateBlock { step_id: dec_str(&arg("step-id")?)?, block: dec_block(&arg("block")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "delete-block" => Ok(FormMutation::DeleteBlock(DeleteBlock { step_id: dec_str(&arg("step-id")?)?, id: dec_str(&arg("id")?)? })),
        "move-block-to-step" => Ok(FormMutation::MoveBlockToStep(MoveBlockToStep { step_id: dec_str(&arg("step-id")?)?, block_id: dec_str(&arg("block-id")?)?, to_step_id: dec_str(&arg("to-step-id")?)?, index: dec_usize(&arg("index")?)? })),
        "replace-block" => Ok(FormMutation::ReplaceBlock(ReplaceBlock { step_id: dec_str(&arg("step-id")?)?, block: dec_block(&arg("block")?)? })),
        "change-block-field" => Ok(FormMutation::ChangeBlockField(ChangeBlockField { block_id: dec_str(&arg("block-id")?)?, change: dec_change(&arg("change")?)? })),
        "change-form-title" => Ok(FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: dec_opt_str(&arg("new-title")?)? })),
        "commit-response" => Ok(FormMutation::CommitResponse(CommitResponse { response: semio_framework_pack_json::from_json_str(&dec_str(&arg("response")?)?,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.into_message())?, index: dec_opt_usize(&arg("index")?)? })),
        "discard-response" => Ok(FormMutation::DiscardResponse(DiscardResponse { id: dec_str(&arg("id")?)? })),
        other => Err(format!("forms mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for FormMutation {
    fn print_op(&self) -> String {
        print_forms_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_forms_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec








//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<FormMutation> {
    let step = FormStep { id: "s1".into(), title: "Step One".into(), description: Some("A \"quoted\" description".into()), blocks: Vec::new() };
    let block = FormQuestion {
        id: "b1".into(),
        label: "Block One".into(),
        kind: "text".into(),
        description: None,
        required: Some(true),
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
        example_id: None,
        params: None,
        condition: None,
    };
    vec![
        FormMutation::CreateStep(CreateStep { step: step.clone(), index: Some(0) }),
        FormMutation::DeleteStep(DeleteStep { id: "s1".into() }),
        FormMutation::ReorderStep(ReorderStep { id: "s1".into(), to_index: 2 }),
        FormMutation::RenameStep(RenameStep { id: "s1".into(), new_title: "New Title".into() }),
        FormMutation::ChangeStepDescription(ChangeStepDescription { id: "s1".into(), new_description: Some("desc".into()) }),
        FormMutation::CreateBlock(CreateBlock { step_id: "s1".into(), block: block.clone(), index: None }),
        FormMutation::DeleteBlock(DeleteBlock { step_id: "s1".into(), id: "b1".into() }),
        FormMutation::MoveBlockToStep(MoveBlockToStep { step_id: "s1".into(), block_id: "b1".into(), to_step_id: "s2".into(), index: 0 }),
        FormMutation::ReplaceBlock(ReplaceBlock { step_id: "s1".into(), block }),
        FormMutation::ChangeBlockField(ChangeBlockField { block_id: "b1".into(), change: BlockField::Min(Some(2.5)) }),
        FormMutation::ChangeBlockField(ChangeBlockField { block_id: "b1".into(), change: BlockField::Description(None) }),
        FormMutation::ChangeBlockField(ChangeBlockField { block_id: "b1".into(), change: BlockField::Options(Some(vec![crate::FormQuestionOption { value: "a".into(), label: "A \"quoted\" option".into() }])) }),
        FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: None }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{FormsDiff, FormsSnapshot};
use protocol::Mutation;

/// 📥️ Decodes the internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) projection the
/// committed `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors carry.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn decode_form_mutation_json(text: &str) -> Result<FormMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.into_message())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");

#[allow(unused_imports)]
mod mutations_wire_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{FormsDiff, FormsSnapshot};
use protocol::Mutation;

/// 🌱 Seeds the working-scene cache behind this snapshot's composed `s.stdio.semio.value` `structure` child handle from a committed
/// `[FormStep]` JSON document, and hands back what it decoded.
///
/// This subset's persisted snapshot holds only the child HANDLE; the live rows behind it are an
/// ephemeral, session-side scene that a fresh process has never populated. A committed
/// `📸️snapshot/⬅️before/🔣️.json` vector is therefore only HALF of a before-state, and the
/// other half lives today in each leaf's own `🧪️tests/<fixture>/🦀️.rs` as a Rust literal.
/// An external conformance host cannot reach that, so this bridge lets the scene half travel as
/// DATA — the exhaustive `🌵️mutate-forms-1` case carries it in its own `Examples` table, with the leaf
/// it was read from cited there. The right long-term fix is to commit the scene beside the snapshot
/// as a fixture file of its own; until then this is the seam that makes the vectors runnable.
// 🚫️async: E1 pure computation over an in-memory snapshot, consumed from a synchronous external test host — see R9
pub fn seed_form_scene_json(snapshot: &mut FormsSnapshot, steps_json: &str) -> Result<Vec<crate::FormStep>, String> {
    let steps: Vec<crate::FormStep> = semio_framework_pack_json::from_json_str(steps_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.into_message())?;
    crate::replace_forms_steps(snapshot, steps.clone());
    Ok(steps)
}
}
pub use mutations_wire_codec::*;
