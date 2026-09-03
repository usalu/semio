//! ⚡️ Forms artifact — hand-rolled `OpText`/`OpBinary` for `FormMutation`. `#[derive(dsl::Mutations)]`
//! only generates `Mutation`/`SemanticMutation` (see `../🦀️.rs`'s `🔖️FormMutation` region)
//! — the wire-text/wire-binary codecs stay handcrafted here, one keyword per semantic verb, grammar
//! `keyword key1=value1 key2=value2 ...`. `🦀️.rs`'s `op` shim re-exports this module's `*`
//! (constants only — the trait impls below attach directly to `FormMutation`).

pub use crate::artifacts::forms::mutations::FormMutation;

use crate::artifacts::forms::mutations::{
    change_form_title::mutation::ChangeFormTitle, change_step_description::mutation::ChangeStepDescription, create_block::mutation::CreateBlock, create_step::mutation::CreateStep, delete_block::mutation::DeleteBlock,
    delete_step::mutation::DeleteStep, move_block_to_step::mutation::MoveBlockToStep, rename_step::mutation::RenameStep, reorder_step::mutation::ReorderStep, replace_block::mutation::ReplaceBlock,
};
use crate::artifacts::forms::{FormQuestion, FormStep};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's text form stays space-free and tokenizable by [`tokenize_args`].
async fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
async fn dec_str(s: &str) -> Result<String, String> {
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
async fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
async fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_str(s)?))
    }
}
async fn enc_usize(v: usize) -> String {
    v.to_string()
}
async fn dec_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
async fn enc_opt_usize(v: &Option<usize>) -> String {
    match v {
        Some(x) => enc_usize(*x),
        None => "-".to_string(),
    }
}
async fn dec_opt_usize(s: &str) -> Result<Option<usize>, String> {
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
async fn tokenize_args(rest: &str) -> Vec<String> {
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
async fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️StructCodec
/// 🌳️ Whole-`FormStep`/`FormQuestion` text form — a quoted JSON string (both already derive
/// `ToValue`/`FromValue`) rather than a second handcrafted step/block grammar; `enc_str`/
/// `dec_str`'s backslash/quote escaping round-trips it byte-for-byte.
async fn enc_step(step: &FormStep) -> String {
    enc_str(&dsl::os_pack::json::to_json_string(step))
}
async fn dec_step(s: &str) -> Result<FormStep, String> {
    dsl::os_pack::json::from_json_str(&dec_str(s)?).map_err(|e| e.to_string())
}
async fn enc_block(block: &FormQuestion) -> String {
    enc_str(&dsl::os_pack::json::to_json_string(block))
}
async fn dec_block(s: &str) -> Result<FormQuestion, String> {
    dsl::os_pack::json::from_json_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️StructCodec

//#region 🔖️OpText
async fn print_forms_mutation(mutation: &FormMutation) -> String {
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
        FormMutation::ChangeFormTitle(p) => format!("change-form-title new-title={}", enc_opt_str(&p.new_title)),
    }
}

async fn parse_forms_mutation(line: &str) -> Result<FormMutation, String> {
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
        "change-form-title" => Ok(FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: dec_opt_str(&arg("new-title")?)? })),
        other => Err(format!("forms mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for FormMutation {
    async fn print_op(&self) -> String {
        print_forms_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_forms_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
async fn write_str_bin(out: &mut Vec<u8>, s: &str) {
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}
async fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
async fn write_opt_str_bin(out: &mut Vec<u8>, s: &Option<String>) {
    match s {
        Some(v) => {
            out.push(1);
            write_str_bin(out, v);
        }
        None => out.push(0),
    }
}
async fn read_opt_str_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_bin(reader)?)),
        other => Err(format!("bad option tag {other}")),
    }
}
async fn write_opt_usize_bin(out: &mut Vec<u8>, v: &Option<usize>) {
    match v {
        Some(x) => {
            out.push(1);
            store::pack_rt::write_varint_u64(out, *x as u64);
        }
        None => out.push(0),
    }
}
async fn read_opt_usize_bin(reader: &mut store::ByteReader<'_>) -> Result<Option<usize>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(reader.read_varint_u64().map_err(|e| e.to_string())? as usize)),
        other => Err(format!("bad option tag {other}")),
    }
}

impl protocol::OpBinary for FormMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            FormMutation::CreateStep(_) => 0,
            FormMutation::DeleteStep(_) => 1,
            FormMutation::ReorderStep(_) => 2,
            FormMutation::RenameStep(_) => 3,
            FormMutation::ChangeStepDescription(_) => 4,
            FormMutation::CreateBlock(_) => 5,
            FormMutation::DeleteBlock(_) => 6,
            FormMutation::MoveBlockToStep(_) => 7,
            FormMutation::ReplaceBlock(_) => 8,
            FormMutation::ChangeFormTitle(_) => 9,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            FormMutation::CreateStep(p) => {
                write_str_bin(&mut out, &enc_step(&p.step));
                write_opt_usize_bin(&mut out, &p.index);
            }
            FormMutation::DeleteStep(p) => write_str_bin(&mut out, &p.id),
            FormMutation::ReorderStep(p) => {
                write_str_bin(&mut out, &p.id);
                store::pack_rt::write_varint_u64(&mut out, p.to_index as u64);
            }
            FormMutation::RenameStep(p) => {
                write_str_bin(&mut out, &p.id);
                write_str_bin(&mut out, &p.new_title);
            }
            FormMutation::ChangeStepDescription(p) => {
                write_str_bin(&mut out, &p.id);
                write_opt_str_bin(&mut out, &p.new_description);
            }
            FormMutation::CreateBlock(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &enc_block(&p.block));
                write_opt_usize_bin(&mut out, &p.index);
            }
            FormMutation::DeleteBlock(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &p.id);
            }
            FormMutation::MoveBlockToStep(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &p.block_id);
                write_str_bin(&mut out, &p.to_step_id);
                store::pack_rt::write_varint_u64(&mut out, p.index as u64);
            }
            FormMutation::ReplaceBlock(p) => {
                write_str_bin(&mut out, &p.step_id);
                write_str_bin(&mut out, &enc_block(&p.block));
            }
            FormMutation::ChangeFormTitle(p) => write_opt_str_bin(&mut out, &p.new_title),
        }
        Ok(out)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => {
                let step_text = read_str_bin(&mut reader).map_err(|e| malformed("step", reader.position(), e))?;
                let step = dec_step(&step_text).map_err(|e| malformed("step", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(FormMutation::CreateStep(CreateStep { step, index }))
            }
            1 => Ok(FormMutation::DeleteStep(DeleteStep { id: read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))? })),
            2 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let to_index = reader.read_varint_u64().map_err(|e| malformed("to_index", reader.position(), e.to_string()))? as usize;
                Ok(FormMutation::ReorderStep(ReorderStep { id, to_index }))
            }
            3 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_title = read_str_bin(&mut reader).map_err(|e| malformed("new_title", reader.position(), e))?;
                Ok(FormMutation::RenameStep(RenameStep { id, new_title }))
            }
            4 => {
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                let new_description = read_opt_str_bin(&mut reader).map_err(|e| malformed("new_description", reader.position(), e))?;
                Ok(FormMutation::ChangeStepDescription(ChangeStepDescription { id, new_description }))
            }
            5 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let block_text = read_str_bin(&mut reader).map_err(|e| malformed("block", reader.position(), e))?;
                let block = dec_block(&block_text).map_err(|e| malformed("block", reader.position(), e))?;
                let index = read_opt_usize_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(FormMutation::CreateBlock(CreateBlock { step_id, block, index }))
            }
            6 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let id = read_str_bin(&mut reader).map_err(|e| malformed("id", reader.position(), e))?;
                Ok(FormMutation::DeleteBlock(DeleteBlock { step_id, id }))
            }
            7 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let block_id = read_str_bin(&mut reader).map_err(|e| malformed("block_id", reader.position(), e))?;
                let to_step_id = read_str_bin(&mut reader).map_err(|e| malformed("to_step_id", reader.position(), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("index", reader.position(), e.to_string()))? as usize;
                Ok(FormMutation::MoveBlockToStep(MoveBlockToStep { step_id, block_id, to_step_id, index }))
            }
            8 => {
                let step_id = read_str_bin(&mut reader).map_err(|e| malformed("step_id", reader.position(), e))?;
                let block_text = read_str_bin(&mut reader).map_err(|e| malformed("block", reader.position(), e))?;
                let block = dec_block(&block_text).map_err(|e| malformed("block", reader.position(), e))?;
                Ok(FormMutation::ReplaceBlock(ReplaceBlock { step_id, block }))
            }
            9 => Ok(FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: read_opt_str_bin(&mut reader).map_err(|e| malformed("new_title", reader.position(), e))? })),
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<FormMutation> {
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
        fixture_slug: None,
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
        FormMutation::ChangeFormTitle(ChangeFormTitle { new_title: None }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{OpBinary, OpText};

    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <FormMutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <FormMutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
