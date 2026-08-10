#!/usr/bin/env python3
"""Patch w4a artifacts with real codecs and field shapes."""
from __future__ import annotations

import json
from pathlib import Path

ROOT = Path.cwd()
TICKET = list(Path(".🦑️repo/🎫️tickets").rglob("STDIO-ARTIFACTS-AND-IO"))[0]
TOKENS = json.loads((TICKET / "🧪tokens.json").read_text())
ROSTER = json.loads((TICKET / "🧪owner-table.json").read_text())["stdio_roster"]
PLUGIN = ROOT / "✏️s" / "🔌️plugins" / TOKENS["stdio_plugin"]

XML_SNAPSHOT = r'''//! 🧬️ XmlSnapshot schema — persistent fields + real codecs.

use crate::artifacts::xml::STDIO_XML_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️XmlModel
/// 🏷️ XML attribute pair.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlAttr {
    pub name: String,
    pub value: String,
}

/// 🌳 XML node (element or text).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum XmlNode {
    Element {
        name: String,
        #[serde(default)]
        attrs: Vec<XmlAttr>,
        #[serde(default)]
        children: Vec<XmlNode>,
    },
    Text {
        text: String,
    },
}

/// 📰 Well-formed XML document root.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct XmlDocument {
    #[serde(default)]
    pub root: Option<XmlNode>,
}
//#endregion 🔖️XmlModel

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.xml` snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.xml")]
pub struct XmlSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub doc: XmlDocument,
}

impl Default for XmlSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_XML_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️XmlTextCodec
fn xml_escape_text(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

fn xml_escape_attr(s: &str) -> String {
    xml_escape_text(s)
}

pub fn xml_document_to_text(doc: &XmlDocument) -> String {
    match &doc.root {
        None => String::new(),
        Some(node) => xml_node_to_text(node, 0),
    }
}

fn xml_node_to_text(node: &XmlNode, _depth: usize) -> String {
    match node {
        XmlNode::Text { text } => xml_escape_text(text),
        XmlNode::Element { name, attrs, children } => {
            let mut out = format!("<{}", name);
            for attr in attrs {
                out.push_str(&format!(" {}=\"{}\"", attr.name, xml_escape_attr(&attr.value)));
            }
            if children.is_empty() {
                out.push_str("/>");
                return out;
            }
            out.push('>');
            for child in children {
                out.push_str(&xml_node_to_text(child, _depth + 1));
            }
            out.push_str(&format!("</{}>", name));
            out
        }
    }
}

pub fn xml_document_from_text(text: &str) -> Result<XmlDocument, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(XmlDocument::default());
    }
    let mut pos = 0;
    skip_misc(trimmed, &mut pos)?;
    let root = parse_node(trimmed, &mut pos)?;
    skip_misc(trimmed, &mut pos)?;
    if pos < trimmed.len() {
        return Err("trailing content after root element".into());
    }
    Ok(XmlDocument { root: Some(root) })
}

fn skip_misc(s: &str, pos: &mut usize) -> Result<(), String> {
    loop {
        skip_ws(s, pos);
        if s[*pos..].starts_with("<?") {
            let end = s[*pos..].find("?>").map_err(|_| "unclosed processing instruction")?;
            *pos += end + 2;
            continue;
        }
        if s[*pos..].starts_with("<!--") {
            let end = s[*pos..].find("-->").map_err(|_| "unclosed comment")?;
            *pos += end + 3;
            continue;
        }
        break;
    }
    Ok(())
}

fn skip_ws(s: &str, pos: &mut usize) {
    while *pos < s.len() && s[*pos].is_ascii_whitespace() {
        *pos += 1;
    }
}

fn parse_name(s: &str, pos: &mut usize) -> Result<String, String> {
    let start = *pos;
    if *pos >= s.len() || !is_name_start(s[*pos..].chars().next().unwrap()) {
        return Err("expected XML name".into());
    }
    *pos += 1;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if is_name_char(ch) {
            *pos += ch.len_utf8();
        } else {
            break;
        }
    }
    Ok(s[start..*pos].to_string())
}

fn is_name_start(ch: char) -> bool {
    ch == ':' || ch.is_ascii_alphabetic() || ch == '_'
}

fn is_name_char(ch: char) -> bool {
    is_name_start(ch) || ch.is_ascii_digit() || ch == '-' || ch == '.'
}

fn parse_attr_value(s: &str, pos: &mut usize) -> Result<String, String> {
    skip_ws(s, pos);
    let quote = s[*pos..].chars().next().ok_or("expected attribute value")?;
    if quote != '"' && quote != '\'' {
        return Err("attribute value must be quoted".into());
    }
    *pos += 1;
    let start = *pos;
    while *pos < s.len() {
        let ch = s[*pos..].chars().next().unwrap();
        if ch == quote {
            let value = s[start..*pos].to_string();
            *pos += 1;
            return Ok(value);
        }
        *pos += ch.len_utf8();
    }
    Err("unclosed attribute value".into())
}

fn parse_attrs(s: &str, pos: &mut usize) -> Result<Vec<XmlAttr>, String> {
    let mut attrs = Vec::new();
    loop {
        skip_ws(s, pos);
        if *pos >= s.len() || s[*pos..].starts_with(">") || s[*pos..].starts_with("/>") {
            break;
        }
        let name = parse_name(s, pos)?;
        skip_ws(s, pos);
        if s[*pos..].chars().next() != Some('=') {
            return Err("expected = in attribute".into());
        }
        *pos += 1;
        let value = parse_attr_value(s, pos)?;
        attrs.push(XmlAttr { name, value });
    }
    Ok(attrs)
}

fn parse_node(s: &str, pos: &mut usize) -> Result<XmlNode, String> {
    skip_misc(s, pos)?;
    if *pos >= s.len() || !s[*pos..].starts_with('<') {
        return Err("expected element start".into());
    }
  if s[*pos..].starts_with("</") {
        return Err("unexpected closing tag".into());
    }
    *pos += 1;
    let name = parse_name(s, pos)?;
    let attrs = parse_attrs(s, pos)?;
    skip_ws(s, pos);
    if s[*pos..].starts_with("/>") {
        *pos += 2;
        return Ok(XmlNode::Element { name, attrs, children: vec![] });
    }
    if !s[*pos..].starts_with('>') {
        return Err("expected > or />".into());
    }
    *pos += 1;
    let mut children = Vec::new();
    loop {
        skip_misc(s, pos)?;
        if s[*pos..].starts_with("</") {
            *pos += 2;
            let close = parse_name(s, pos)?;
            skip_ws(s, pos);
            if !s[*pos..].starts_with('>') {
                return Err("expected > on closing tag".into());
            }
            *pos += 1;
            if close != name {
                return Err(format!("closing tag mismatch: expected </{}>, got </{}>", name, close));
            }
            break;
        }
        if *pos < s.len() && s[*pos..].starts_with('<') {
            children.push(parse_node(s, pos)?);
            continue;
        }
        let start = *pos;
        while *pos < s.len() && !s[*pos..].starts_with('<') {
            *pos += 1;
        }
        let text = s[start..*pos].to_string();
        if !text.is_empty() {
            children.push(XmlNode::Text { text });
        }
    }
    Ok(XmlNode::Element { name, attrs, children })
}
//#endregion 🔖️XmlTextCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for XmlSnapshot {
    const EXTENSION: &'static str = "xml";
    fn envelope_id() -> &'static str { "stdio.xml" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let doc = xml_document_from_text(body).map_err(|e| {
            store::TextError::new(format!("xml parse: {e}"), dsl::TextSpan::at(1, 1))
        })?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
    }
    fn print_dsl(&self) -> String {
        let body = xml_document_to_text(&self.doc);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for XmlSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(&self.doc).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let doc = serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
'''

CSV_SNAPSHOT = r'''//! 🧬️ CsvSnapshot schema — persistent fields + real codecs.

use crate::artifacts::csv::STDIO_CSV_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.csv` snapshot (RFC4180-ish table).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.csv")]
pub struct CsvSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub headers: Vec<String>,
    #[state(persistent)]
    #[serde(default)]
    pub rows: Vec<Vec<String>>,
}

impl Default for CsvSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_CSV_DOCUMENT_SCHEMA.into(),
            headers: Vec::new(),
            rows: Vec::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️CsvTextCodec
fn csv_escape_row(cells: &[String]) -> String {
    cells
        .iter()
        .map(|c| {
            if c.contains(',') || c.contains('"') || c.contains('\n') || c.contains('\r') {
                format!("\"{}\"", c.replace('"', "\"\""))
            } else {
                c.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn csv_parse_row(line: &str) -> Vec<String> {
    let mut cells = Vec::new();
    let mut cur = String::new();
    let mut in_q = false;
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_q {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cur.push('"');
                } else {
                    in_q = false;
                }
            } else {
                cur.push(ch);
            }
        } else if ch == '"' {
            in_q = true;
        } else if ch == ',' {
            cells.push(cur.clone());
            cur.clear();
        } else {
            cur.push(ch);
        }
    }
    cells.push(cur);
    cells
}

pub fn csv_table_to_text(headers: &[String], rows: &[Vec<String>]) -> String {
    let mut out = csv_escape_row(headers);
    out.push('\n');
    for row in rows {
        out.push_str(&csv_escape_row(row));
        out.push('\n');
    }
    out
}

pub fn csv_table_from_text(text: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let mut lines = text.lines();
    let headers = csv_parse_row(lines.next().unwrap_or(""));
    let rows = lines.filter(|l| !l.is_empty()).map(|l| csv_parse_row(l)).collect();
    (headers, rows)
}
//#endregion 🔖️CsvTextCodec

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for CsvSnapshot {
    const EXTENSION: &'static str = "csv";
    fn envelope_id() -> &'static str { "stdio.csv" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let (headers, rows) = csv_table_from_text(body);
        Ok(Self { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows })
    }
    fn print_dsl(&self) -> String {
        let body = csv_table_to_text(&self.headers, &self.rows);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for CsvSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = csv_table_to_text(&self.headers, &self.rows).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let text = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let (headers, rows) = csv_table_from_text(&text);
        Ok(Self { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
'''

MD_SNAPSHOT = r'''//! 🧬️ MdSnapshot schema — persistent fields + real codecs.

use crate::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted `stdio.md` snapshot (lossless markdown text).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.md")]
pub struct MdSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub body: String,
}

impl Default for MdSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_MD_DOCUMENT_SCHEMA.into(),
            body: String::new(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedDocumentCodecs
impl store::DocumentDsl for MdSnapshot {
    const EXTENSION: &'static str = "md";
    fn envelope_id() -> &'static str { "stdio.md" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest.to_string(),
            Err(_) => text.to_string(),
        };
        Ok(Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body })
    }
    fn print_dsl(&self) -> String {
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &self.body)
    }
}

impl store::DocumentPack for MdSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = self.body.as_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let body = String::from_utf8(inner).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(Self { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body })
    }
}
//#endregion 🔖️HandcraftedDocumentCodecs
'''


def artifact_schema_rs(mid: str, name: str, fields_rs: str, field_names: list[str]) -> str:
    snap = f"{name}Snapshot"
    art = f"{name}Artifact"
    fn = f"{mid}_artifact_schema_descriptor"
    return f'''//! 🧬️ {art} schema — full artifact state.

use crate::artifacts::{mid}::{snap};
use schema::ArtifactSchema;
use serde::{{Deserialize, Serialize}};

//#region 🔖️Artifact
/// 🧬️ Full `stdio.{mid}` artifact state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.{mid}")]
pub struct {art} {{
    #[state(persistent)]
    pub schema: String,
{fields_rs}
}}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for {art} {{
    fn default() -> Self {{
        Self::from_snapshot({snap}::default())
    }}
}}

impl {art} {{
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> {snap} {{
        {snap} {{
            schema: self.schema.clone(),
{"".join(f"            {f}: self.{f}.clone()," for f in field_names)}
        }}
    }}

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: {snap}) -> Self {{
        Self {{
            schema: snapshot.schema,
{"".join(f"            {f}: snapshot.{f}," for f in field_names)}
        }}
    }}

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: {snap}) {{
        self.schema = snapshot.schema;
{"".join(f"        self.{f} = snapshot.{f};" for f in field_names)}
    }}
}}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.{mid}`.
pub fn {fn}() -> schema::ArtifactSchemaDescriptor {{
    schema::ArtifactSchemaDescriptor {{
        id: "s.stdio.{mid}",
        artifact: schema::FacetLeaves {{
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        }},
        snapshot: schema::FacetLeaves {{
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        }},
        diff: schema::FacetLeaves {{
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        }},
    }}
}}
//#endregion 🔖️Descriptor
'''


def io_deser_rs(mid: str, name: str, parse_body: str) -> str:
    snap = f"{name}Snapshot"
    doc = f"STDIO_{mid.upper()}_DOCUMENT_SCHEMA"
    return f'''//! 📥️ Deserialize `stdio.{mid}` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::{mid}::{{{snap}, {doc}}};

//#region 🔖️Codec
/// 🗂️ Register deserializer hooks.
pub fn register() {{}}

/// 📥 Parse {mid} text into a {snap}.
pub fn deserialize(from: &TxtSnapshot) -> Result<{snap}, store::TextError> {{
{parse_body}
}}

/// 📥 Parse DSL/text bytes via txt then {mid}.
pub fn deserialize_text(text: &str) -> Result<{snap}, store::TextError> {{
    deserialize(&<TxtSnapshot as store::DocumentDsl>::parse_dsl(text)?)
}}
//#endregion 🔖️Codec
'''


def io_ser_rs(mid: str, name: str, serialize_body: str) -> str:
    snap = f"{name}Snapshot"
    return f'''//! 📤️ Serialize `stdio.{mid}` to stdio.txt.

use crate::artifacts::txt::{{TxtSnapshot, STDIO_TXT_DOCUMENT_SCHEMA}};
use crate::artifacts::{mid}::{snap};

//#region 🔖️Codec
/// 🗂️ Register serializer hooks.
pub fn register() {{}}

/// 📤️ Encode {mid} into a TxtSnapshot.
pub fn serialize(from: &{snap}) -> Result<TxtSnapshot, store::PackError> {{
{serialize_body}
}}

/// 📤️ Encode as txt DSL.
pub fn serialize_text(from: &{snap}) -> Result<String, store::PackError> {{
    Ok(store::DocumentDsl::print_dsl(&serialize(from)?))
}}
//#endregion 🔖️Codec
'''


def fix_mutations(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    text = text.replace("serde_xml::", "serde_json::")
    text = text.replace("serde_csv::", "serde_json::")
    text = text.replace("serde_md::", "serde_json::")
    path.write_text(text, encoding="utf-8")


def fix_graphql_ts(base: Path, field_line: str, ts_field: str) -> None:
    for rel in [
        "🧬️schema/🔗️component.graphql",
        "🧬️schema/📸️snapshot/🔗️component.graphql",
        "🧬️schema/🔺️diff/🔗️component.graphql",
        "🧬️schema/🟦️component.ts",
        "🧬️schema/📸️snapshot/🟦️component.ts",
    ]:
        p = base / rel
        if not p.exists():
            continue
        if p.suffix == ".graphql":
            if "diff" in rel:
                p.write_text(f"# diff facet\n type Placeholder {{ {field_line} }}\n", encoding="utf-8")
            else:
                p.write_text(f"# schema facet\n type Placeholder {{ {field_line} }}\n", encoding="utf-8")
        else:
            p.write_text(f"/** schema facet */\nexport interface Placeholder {{ {ts_field} }}\n", encoding="utf-8")


# XML
xml_base = PLUGIN / "🗿️artifacts" / ROSTER["xml"]["dir"]
(xml_base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(XML_SNAPSHOT, encoding="utf-8")
(xml_base / "🧬️schema/🦀️component.rs").write_text(
    artifact_schema_rs(
        "xml",
        "Xml",
        "    #[state(persistent)]\n    #[serde(default)]\n    pub doc: crate::artifacts::xml::schema::snapshot::XmlDocument,",
        ["doc"],
    ),
    encoding="utf-8",
)
(xml_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
    io_deser_rs(
        "xml",
        "Xml",
        "    let doc = crate::artifacts::xml::schema::snapshot::xml_document_from_text(from.text.trim()).map_err(|e| {\n"
        "        store::TextError::new(format!(\"xml parse: {e}\"), dsl::TextSpan::at(1, 1))\n"
        "    })?;\n"
        "    Ok(XmlSnapshot { schema: STDIO_XML_DOCUMENT_SCHEMA.into(), doc })",
    ),
    encoding="utf-8",
)
(xml_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
    io_ser_rs(
        "xml",
        "Xml",
        "    let text = crate::artifacts::xml::schema::snapshot::xml_document_to_text(&from.doc);\n"
        "    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })",
    ),
    encoding="utf-8",
)
fix_mutations(xml_base / "🧬️schema/🧬️mutations/🦀️component.rs")
fix_graphql_ts(xml_base, "doc: String!", "doc: unknown;")

# CSV
csv_base = PLUGIN / "🗿️artifacts" / ROSTER["csv"]["dir"]
(csv_base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(CSV_SNAPSHOT, encoding="utf-8")
(csv_base / "🧬️schema/🦀️component.rs").write_text(
    artifact_schema_rs(
        "csv",
        "Csv",
        "    #[state(persistent)]\n    #[serde(default)]\n    pub headers: Vec<String>,\n"
        "    #[state(persistent)]\n    #[serde(default)]\n    pub rows: Vec<Vec<String>>,",
        ["headers", "rows"],
    ),
    encoding="utf-8",
)
(csv_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
    io_deser_rs(
        "csv",
        "Csv",
        "    let (headers, rows) = crate::artifacts::csv::schema::snapshot::csv_table_from_text(from.text.as_str());\n"
        "    Ok(CsvSnapshot { schema: STDIO_CSV_DOCUMENT_SCHEMA.into(), headers, rows })",
    ),
    encoding="utf-8",
)
(csv_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
    io_ser_rs(
        "csv",
        "Csv",
        "    let text = crate::artifacts::csv::schema::snapshot::csv_table_to_text(&from.headers, &from.rows);\n"
        "    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text })",
    ),
    encoding="utf-8",
)
fix_mutations(csv_base / "🧬️schema/🧬️mutations/🦀️component.rs")
fix_graphql_ts(csv_base, "headers: String!", "headers: string[]; rows: string[][];")

# MD
md_base = PLUGIN / "🗿️artifacts" / ROSTER["md"]["dir"]
(md_base / "🧬️schema/📸️snapshot/🦀️component.rs").write_text(MD_SNAPSHOT, encoding="utf-8")
(md_base / "🧬️schema/🦀️component.rs").write_text(
    artifact_schema_rs(
        "md",
        "Md",
        "    #[state(persistent)]\n    #[serde(default)]\n    pub body: String,",
        ["body"],
    ),
    encoding="utf-8",
)
(md_base / "🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
    io_deser_rs(
        "md",
        "Md",
        "    Ok(MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), body: from.text.clone() })",
    ),
    encoding="utf-8",
)
(md_base / "🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄txt/🦀️component.rs").write_text(
    io_ser_rs(
        "md",
        "Md",
        "    Ok(TxtSnapshot { schema: STDIO_TXT_DOCUMENT_SCHEMA.into(), text: from.body.clone() })",
    ),
    encoding="utf-8",
)
fix_mutations(md_base / "🧬️schema/🧬️mutations/🦀️component.rs")
fix_graphql_ts(md_base, "body: String!", "body: string;")

# Media types on root components
(xml_base / "🦀️component.rs").write_text(
    (xml_base / "🦀️component.rs").read_text(encoding="utf-8").replace(
        "MediaForm::Value", "MediaForm::Document"
    ),
    encoding="utf-8",
)
(csv_base / "🦀️component.rs").write_text(
    (csv_base / "🦀️component.rs").read_text(encoding="utf-8").replace(
        "MediaForm::Value", "MediaForm::Document"
    ),
    encoding="utf-8",
)
(md_base / "🦀️component.rs").write_text(
    (md_base / "🦀️component.rs").read_text(encoding="utf-8").replace(
        "MediaClass::Data", "MediaClass::Text"
    ).replace("MediaForm::Value", "MediaForm::Document"),
    encoding="utf-8",
)

# Example assets
(xml_base / "📚️examples/🎬️demo/🖼️assets/example.xml").write_text(
    "<note><to>Tove</to><from>Jani</from><body>Remember me</body></note>\n", encoding="utf-8"
)
(csv_base / "📚️examples/🎬️demo/🖼️assets/example.csv").write_text(
    "name,count\nalpha,1\nbeta,2\n", encoding="utf-8"
)
(md_base / "📚️examples/🎬️demo/🖼️assets/example.md").write_text(
    "# Title\n\nLossless **markdown** body.\n", encoding="utf-8"
)

print("fixed codecs")
