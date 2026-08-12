#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Scratch-only generator for en1995's mutations/📝️text/component.rs (hand-rolled OpText+OpBinary)
and mutations/💾️binary/component.rs (thin wrapper), mirroring en1992's precedent exactly. See
gen.py in this same scratch dir for the field-derivation source of truth."""
import os

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"

# (snake, rust_type, sample_value_literal)
FIELDS = [
    ("m_ed_knm", "f64", "999.0"),
    ("n_ed_kn", "f64", "111.0"),
    ("v_ed_kn", "f64", "77.0"),
    ("w_mm3", "f64", "2_000_000.0"),
    ("a_mm2", "f64", "30_000.0"),
    ("b_mm", "f64", "250.0"),
    ("h_mm", "f64", "400.0"),
    ("f_m_k", "f64", "28.0"),
    ("f_c_0_k", "f64", "24.0"),
    ("service_class", "String", '"sc2".into()'),
    ("load_duration", "String", '"short".into()'),
    ("m_crit_knm", "f64", "95.0"),
    ("f_ed_kn", "f64", "22.0"),
    ("a_ef_mm2", "f64", "14_000.0"),
    ("f_v_k", "f64", "4.5"),
    ("fire_duration_min", "f64", "60.0"),
    ("section_depth_mm", "f64", "350.0"),
    ("a_vert_m_s2", "f64", "0.5"),
    ("n_cycles_bridge", "f64", "750_000.0"),
]

def pascal(snake: str) -> str:
    return "".join((seg[0].upper() + seg[1:]) if seg else "" for seg in snake.split("_"))

def kebab(snake: str) -> str:
    return snake.replace("_", "-")

lines_use = []
for snake, _, _ in FIELDS:
    mod = f"change_{snake}"
    variant = f"Change{pascal(snake)}"
    lines_use.append(f"    {mod}::mutation::{variant},")
use_block = "\n".join(sorted(lines_use))

def write_bin_fn(rust_type):
    return {"f64": "write_f64_bin", "String": "write_str_bin"}[rust_type]
def read_bin_fn(rust_type):
    return {"f64": "read_f64_bin", "String": "read_str_bin"}[rust_type]
def dec_fn(rust_type):
    return {"f64": "dec_f64", "String": "dec_str_owned"}[rust_type]

print_arms = ['        En1995Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),']
parse_arms = ['        "change-annex" => Ok(En1995Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),']
tag_arms = ["            En1995Mutation::ChangeAnnex(_) => 0,"]
enc_arms = ["            En1995Mutation::ChangeAnnex(p) => write_json_bin(&mut out, &p.new_annex),"]
dec_arms = ['            0 => Ok(En1995Mutation::ChangeAnnex(ChangeAnnex { new_annex: read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))? })),']
demo_lines = ["        En1995Mutation::ChangeAnnex(ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),"]

for i, (snake, rust_type, sample) in enumerate(FIELDS, start=1):
    kb = kebab(snake)
    variant = f"Change{pascal(snake)}"
    field = f"new_{snake}"

    if rust_type == "f64":
        print_arms.append(f'        En1995Mutation::{variant}(p) => format!("change-{kb} new-{snake}={{}}", p.{field}),')
        parse_arms.append(f'        "change-{kb}" => Ok(En1995Mutation::{variant}({variant} {{ {field}: dec_f64(&arg("new-{snake}")?)? }})),')
        enc_arms.append(f"            En1995Mutation::{variant}(p) => write_f64_bin(&mut out, p.{field}),")
        dec_arms.append(f'            {i} => Ok(En1995Mutation::{variant}({variant} {{ {field}: read_f64_bin(&mut reader).map_err(|e| malformed("{field}", reader.position(), e))? }})),')
    else:  # String
        print_arms.append(f'        En1995Mutation::{variant}(p) => format!("change-{kb} new-{snake}={{}}", enc_str(&p.{field})),')
        parse_arms.append(f'        "change-{kb}" => Ok(En1995Mutation::{variant}({variant} {{ {field}: dec_str(&arg("new-{snake}")?)? }})),')
        enc_arms.append(f"            En1995Mutation::{variant}(p) => write_str_bin(&mut out, &p.{field}),")
        dec_arms.append(f'            {i} => Ok(En1995Mutation::{variant}({variant} {{ {field}: read_str_bin(&mut reader).map_err(|e| malformed("{field}", reader.position(), e))? }})),')

    tag_arms.append(f"            En1995Mutation::{variant}(_) => {i},")
    demo_lines.append(f"        En1995Mutation::{variant}({variant} {{ {field}: {sample} }}),")

text_content = f'''//! ⚡️ EN 1995 design of timber structures — hand-rolled `OpText`/`OpBinary` for
//! `En1995Mutation`. `#[derive(dsl::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../🦀️component.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay
//! handcrafted here, one keyword per semantic verb, grammar `change-<field> new-<field>=<value>`.
//! The one enum-typed field (`annex`) round-trips through a quoted JSON string — it already
//! derives `Serialize`/`Deserialize`, so a second handcrafted grammar for it would just duplicate
//! that losslessly. Mirrors this ticket's `en1992` precedent exactly.

pub use crate::artifacts::en1995::schema::mutations::En1995Mutation;

use crate::artifacts::en1995::schema::mutations::{{
{use_block}
}};
use crate::artifacts::en1995::schema::mutations::set_snapshot::mutation::ChangeAnnex;

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, used both
/// for the `String` fields (`service_class`/`load_duration`) and to wrap the JSON form of `annex`.
fn enc_str(s: &str) -> String {{
    format!("\\"{{}}\\"", s.replace('\\\\', "\\\\\\\\").replace('"', "\\\\\\""))
}}
fn dec_str(s: &str) -> Result<String, String> {{
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {{s:?}}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {{
        if c != '\\\\' {{
            out.push(c);
            continue;
        }}
        match chars.next() {{
            Some('\\\\') => out.push('\\\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\\\{{other}}")),
            None => return Err("dangling escape".into()),
        }}
    }}
    Ok(out)
}}
fn enc_json<T: serde::Serialize>(value: &T) -> String {{
    enc_str(&serde_json::to_string(value).expect("en1995 mutation payload field always serializes"))
}}
fn dec_json<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {{
    serde_json::from_str(&dec_str(s)?).map_err(|e| e.to_string())
}}
fn dec_f64(s: &str) -> Result<f64, String> {{
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value.
fn tokenize_args(rest: &str) -> Vec<String> {{
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {{
        match c {{
            '"' => {{
                current.push(c);
                in_quotes = !in_quotes;
            }}
            '\\\\' if in_quotes => {{
                current.push(c);
                if let Some(next) = chars.next() {{
                    current.push(next);
                }}
            }}
            ' ' if !in_quotes => {{
                if !current.is_empty() {{
                    tokens.push(std::mem::take(&mut current));
                }}
            }}
            _ => current.push(c),
        }}
    }}
    if !current.is_empty() {{
        tokens.push(current);
    }}
    tokens
}}
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {{
    tokenize_args(rest)
        .into_iter()
        .map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {{token:?}}")))
        .collect()
}}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
fn print_en1995_mutation(mutation: &En1995Mutation) -> String {{
    match mutation {{
{chr(10).join(print_arms)}
    }}
}}

fn parse_en1995_mutation(line: &str) -> Result<En1995Mutation, String> {{
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1995 mutation: missing arg '{{k}}' for '{{keyword}}'"));
    match keyword {{
{chr(10).join(parse_arms)}
        other => Err(format!("en1995 mutation: unknown keyword {{other:?}}")),
    }}
}}

impl protocol::OpText for En1995Mutation {{
    fn print_op(&self) -> String {{
        print_en1995_mutation(self)
    }}
    fn parse_op(line: &str) -> Result<Self, store::TextError> {{
        parse_en1995_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }}
}}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | value`; `f64` fields write their native binary form
/// directly, `String` fields write length-prefixed UTF-8, and `annex` goes through the same JSON
/// bridge as `OpText` above.
fn write_str_bin(out: &mut Vec<u8>, s: &str) {{
    store::pack_rt::write_varint_u64(out, s.len() as u64);
    out.extend_from_slice(s.as_bytes());
}}
fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {{
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}}
fn write_json_bin<T: serde::Serialize>(out: &mut Vec<u8>, value: &T) {{
    write_str_bin(out, &serde_json::to_string(value).expect("en1995 mutation payload field always serializes"));
}}
fn read_json_bin<T: serde::de::DeserializeOwned>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {{
    serde_json::from_str(&read_str_bin(reader)?).map_err(|e| e.to_string())
}}
fn write_f64_bin(out: &mut Vec<u8>, v: f64) {{
    out.extend_from_slice(&v.to_le_bytes());
}}
fn read_f64_bin(reader: &mut store::ByteReader<'_>) -> Result<f64, String> {{
    reader.read_f64_le().map_err(|e| e.to_string())
}}

impl protocol::OpBinary for En1995Mutation {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        let tag: u8 = match self {{
{chr(10).join(tag_arms)}
        }};
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {{
{chr(10).join(enc_arms)}
        }}
        Ok(out)
    }}

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed {{ what, offset: offset as u64, detail }};
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {{
{chr(10).join(dec_arms)}
            other => Err(malformed("op tag", 1, format!("unknown tag {{other}}"))),
        }}
    }}
}}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1995Mutation> {{
    vec![
{chr(10).join(demo_lines)}
    ]
}}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {{
    use super::*;
    use protocol::{{OpBinary, OpText}};

    #[test]
    fn op_text_binary_roundtrip_law() {{
        for mutation in demo_mutation_cases() {{
            let printed = mutation.print_op();
            assert!(!printed.contains('\\n'), "print_op must be one line, got {{printed:?}}");
            let parsed = <En1995Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({{printed:?}}) failed: {{e}}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch (printed {{printed:?}})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op failed: {{e}}"));
            let decoded = <En1995Mutation as OpBinary>::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {{e}}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch");
        }}
    }}
}}
//#endregion 🧪️Tests
'''

with open(f"{ROOT}/📝️text/🦀️component.rs", "w", encoding="utf-8") as f:
    f.write(text_content)

binary_content = '''//! ⚖️ EN 1995 design of timber structures — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


use crate::artifacts::en1995::schema::mutations::text::En1995Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1995Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1995Mutation, protocol::ProtocolError> {
    En1995Mutation::decode_op(bytes)
}
'''

with open(f"{ROOT}/💾️binary/🦀️component.rs", "w", encoding="utf-8") as f:
    f.write(binary_content)

print("wrote text + binary component.rs")
