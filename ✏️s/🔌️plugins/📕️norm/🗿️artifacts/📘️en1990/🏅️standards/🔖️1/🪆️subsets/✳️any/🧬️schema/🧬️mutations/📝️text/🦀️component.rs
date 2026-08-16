//! ⚡️ EN 1990 basis of structural design — hand-rolled `OpText`/`OpBinary` for `En1990Mutation`.
//! `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see `../🦀️component.rs`'s
//! `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted here, one keyword per
//! semantic verb, grammar `keyword key1=value1 key2=value2 ...`. Every field (scalar or structured)
//! already derives `Serialize`/`Deserialize`, so it round-trips through a quoted JSON atom uniformly
//! (same rationale `din4108`'s sibling facet documents, applied here given this facet's field-count).

pub use crate::artifacts::en1990::schema::mutations::En1990Mutation;

use crate::artifacts::en1990::schema::mutations::{
    change_consequence_class::mutation::ChangeConsequenceClass, change_permanent_action::mutation::ChangePermanentAction, change_resistance::mutation::ChangeResistance, change_seismic_action::mutation::ChangeSeismicAction,
    change_variable_action_category::mutation::ChangeVariableActionCategory, change_variable_action_value::mutation::ChangeVariableActionValue, insert_variable_action::mutation::InsertVariableAction,
    remove_variable_action::mutation::RemoveVariableAction, reorder_variable_actions::mutation::ReorderVariableActions, set_snapshot,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other token stays space-free and tokenizable by [`tokenize_args`].
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
/// 🧬️ Every payload field already derives `Serialize`/`Deserialize` — a quoted JSON atom reuses
/// that losslessly instead of a second handcrafted grammar per field type.
fn enc_json<T: serde::Serialize>(value: &T) -> String {
    enc_str(&serde_json::to_string(value).expect("en1990 mutation payload field always serializes"))
}
fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because string/JSON payloads may contain spaces.
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

//#region 🔖️OpText
fn print_en1990_mutation(mutation: &En1990Mutation) -> String {
    match mutation {
        En1990Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1990Mutation::ChangePermanentAction(p) => format!("change-permanent-action new-g-k={}", enc_json(&p.new_g_k)),
        En1990Mutation::ChangeResistance(p) => format!("change-resistance new-resistance-kn={}", enc_json(&p.new_resistance_kn)),
        En1990Mutation::ChangeConsequenceClass(p) => format!("change-consequence-class new-consequence-class={}", enc_json(&p.new_consequence_class)),
        En1990Mutation::ChangeSeismicAction(p) => format!("change-seismic-action new-seismic-a-ed-kn={}", enc_json(&p.new_seismic_a_ed_kn)),
        En1990Mutation::InsertVariableAction(p) => format!("insert-variable-action index={} category={} value={}", enc_json(&p.index), enc_json(&p.category), enc_json(&p.value)),
        En1990Mutation::RemoveVariableAction(p) => format!("remove-variable-action index={}", enc_json(&p.index)),
        En1990Mutation::ChangeVariableActionCategory(p) => format!("change-variable-action-category index={} new-category={}", enc_json(&p.index), enc_json(&p.new_category)),
        En1990Mutation::ChangeVariableActionValue(p) => format!("change-variable-action-value index={} new-value={}", enc_json(&p.index), enc_json(&p.new_value)),
        En1990Mutation::ReorderVariableActions(p) => format!("reorder-variable-actions from={} to={}", enc_json(&p.from), enc_json(&p.to)),
    }
}

fn parse_en1990_mutation(line: &str) -> Result<En1990Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1990 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-annex" => Ok(En1990Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),
        "change-permanent-action" => Ok(En1990Mutation::ChangePermanentAction(ChangePermanentAction { new_g_k: dec_json(&arg("new-g-k")?)? })),
        "change-resistance" => Ok(En1990Mutation::ChangeResistance(ChangeResistance { new_resistance_kn: dec_json(&arg("new-resistance-kn")?)? })),
        "change-consequence-class" => Ok(En1990Mutation::ChangeConsequenceClass(ChangeConsequenceClass { new_consequence_class: dec_json(&arg("new-consequence-class")?)? })),
        "change-seismic-action" => Ok(En1990Mutation::ChangeSeismicAction(ChangeSeismicAction { new_seismic_a_ed_kn: dec_json(&arg("new-seismic-a-ed-kn")?)? })),
        "insert-variable-action" => Ok(En1990Mutation::InsertVariableAction(InsertVariableAction { index: dec_json(&arg("index")?)?, category: dec_json(&arg("category")?)?, value: dec_json(&arg("value")?)? })),
        "remove-variable-action" => Ok(En1990Mutation::RemoveVariableAction(RemoveVariableAction { index: dec_json(&arg("index")?)? })),
        "change-variable-action-category" => Ok(En1990Mutation::ChangeVariableActionCategory(ChangeVariableActionCategory { index: dec_json(&arg("index")?)?, new_category: dec_json(&arg("new-category")?)? })),
        "change-variable-action-value" => Ok(En1990Mutation::ChangeVariableActionValue(ChangeVariableActionValue { index: dec_json(&arg("index")?)?, new_value: dec_json(&arg("new-value")?)? })),
        "reorder-variable-actions" => Ok(En1990Mutation::ReorderVariableActions(ReorderVariableActions { from: dec_json(&arg("from")?)?, to: dec_json(&arg("to")?)? })),
        other => Err(format!("en1990 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for En1990Mutation {
    fn print_op(&self) -> String {
        print_en1990_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_en1990_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | json-string-per-field` — the JSON-per-field
/// consolidation used by `OpText` above applies equally here.
fn write_json_bin<T: serde::Serialize>(out: &mut Vec<u8>, value: &T) {
    let bytes = serde_json::to_string(value).expect("en1990 mutation payload field always serializes");
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes.as_bytes());
}
fn read_json_bin<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    serde_json::from_str(text).map_err(|e| e.to_string())
}

impl protocol::OpBinary for En1990Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1990Mutation::ChangeAnnex(_) => 0,
            En1990Mutation::ChangePermanentAction(_) => 1,
            En1990Mutation::ChangeResistance(_) => 2,
            En1990Mutation::ChangeConsequenceClass(_) => 3,
            En1990Mutation::ChangeSeismicAction(_) => 4,
            En1990Mutation::InsertVariableAction(_) => 5,
            En1990Mutation::RemoveVariableAction(_) => 6,
            En1990Mutation::ChangeVariableActionCategory(_) => 7,
            En1990Mutation::ChangeVariableActionValue(_) => 8,
            En1990Mutation::ReorderVariableActions(_) => 9,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            En1990Mutation::ChangeAnnex(p) => write_json_bin(&mut out, &p.new_annex),
            En1990Mutation::ChangePermanentAction(p) => write_json_bin(&mut out, &p.new_g_k),
            En1990Mutation::ChangeResistance(p) => write_json_bin(&mut out, &p.new_resistance_kn),
            En1990Mutation::ChangeConsequenceClass(p) => write_json_bin(&mut out, &p.new_consequence_class),
            En1990Mutation::ChangeSeismicAction(p) => write_json_bin(&mut out, &p.new_seismic_a_ed_kn),
            En1990Mutation::InsertVariableAction(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.category);
                write_json_bin(&mut out, &p.value);
            }
            En1990Mutation::RemoveVariableAction(p) => write_json_bin(&mut out, &p.index),
            En1990Mutation::ChangeVariableActionCategory(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.new_category);
            }
            En1990Mutation::ChangeVariableActionValue(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.new_value);
            }
            En1990Mutation::ReorderVariableActions(p) => {
                write_json_bin(&mut out, &p.from);
                write_json_bin(&mut out, &p.to);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => {
                let new_annex = read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))?;
                Ok(En1990Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex }))
            }
            1 => {
                let new_g_k = read_json_bin(&mut reader).map_err(|e| malformed("new_g_k", reader.position(), e))?;
                Ok(En1990Mutation::ChangePermanentAction(ChangePermanentAction { new_g_k }))
            }
            2 => {
                let new_resistance_kn = read_json_bin(&mut reader).map_err(|e| malformed("new_resistance_kn", reader.position(), e))?;
                Ok(En1990Mutation::ChangeResistance(ChangeResistance { new_resistance_kn }))
            }
            3 => {
                let new_consequence_class = read_json_bin(&mut reader).map_err(|e| malformed("new_consequence_class", reader.position(), e))?;
                Ok(En1990Mutation::ChangeConsequenceClass(ChangeConsequenceClass { new_consequence_class }))
            }
            4 => {
                let new_seismic_a_ed_kn = read_json_bin(&mut reader).map_err(|e| malformed("new_seismic_a_ed_kn", reader.position(), e))?;
                Ok(En1990Mutation::ChangeSeismicAction(ChangeSeismicAction { new_seismic_a_ed_kn }))
            }
            5 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let category = read_json_bin(&mut reader).map_err(|e| malformed("category", reader.position(), e))?;
                let value = read_json_bin(&mut reader).map_err(|e| malformed("value", reader.position(), e))?;
                Ok(En1990Mutation::InsertVariableAction(InsertVariableAction { index, category, value }))
            }
            6 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1990Mutation::RemoveVariableAction(RemoveVariableAction { index }))
            }
            7 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let new_category = read_json_bin(&mut reader).map_err(|e| malformed("new_category", reader.position(), e))?;
                Ok(En1990Mutation::ChangeVariableActionCategory(ChangeVariableActionCategory { index, new_category }))
            }
            8 => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let new_value = read_json_bin(&mut reader).map_err(|e| malformed("new_value", reader.position(), e))?;
                Ok(En1990Mutation::ChangeVariableActionValue(ChangeVariableActionValue { index, new_value }))
            }
            9 => {
                let from = read_json_bin(&mut reader).map_err(|e| malformed("from", reader.position(), e))?;
                let to = read_json_bin(&mut reader).map_err(|e| malformed("to", reader.position(), e))?;
                Ok(En1990Mutation::ReorderVariableActions(ReorderVariableActions { from, to }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1990Mutation> {
    vec![
        En1990Mutation::ChangeAnnex(set_snapshot::mutation::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1990Mutation::ChangePermanentAction(ChangePermanentAction { new_g_k: 120.0 }),
        En1990Mutation::ChangeResistance(ChangeResistance { new_resistance_kn: 350.0 }),
        En1990Mutation::ChangeConsequenceClass(ChangeConsequenceClass { new_consequence_class: 3 }),
        En1990Mutation::ChangeSeismicAction(ChangeSeismicAction { new_seismic_a_ed_kn: 60.0 }),
        En1990Mutation::InsertVariableAction(InsertVariableAction { index: 1, category: "snow".into(), value: 20.0 }),
        En1990Mutation::RemoveVariableAction(RemoveVariableAction { index: 1 }),
        En1990Mutation::ChangeVariableActionCategory(ChangeVariableActionCategory { index: 1, new_category: "storage".into() }),
        En1990Mutation::ChangeVariableActionValue(ChangeVariableActionValue { index: 1, new_value: 65.0 }),
        En1990Mutation::ReorderVariableActions(ReorderVariableActions { from: 0, to: 1 }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{OpBinary, OpText};

    #[test]
    fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = <En1990Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {e}"));
            let decoded = <En1990Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }
    }
}
//#endregion 🧪️Tests
