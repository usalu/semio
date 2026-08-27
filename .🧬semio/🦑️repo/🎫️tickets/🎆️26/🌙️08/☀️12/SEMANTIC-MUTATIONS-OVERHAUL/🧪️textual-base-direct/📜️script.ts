import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import Ajv from "ajv";

type Field = { name: string; rust: string; ts: string; gql: string; proto: string; schema: unknown; optional?: boolean };
type Leaf = { semantic: string; emoji: string; variant: string; fields: Field[]; diff: string };
type Root = {
  artifact: "json" | "xml" | "svg" | "txt";
  folder: string;
  standard: string;
  aggregate: string;
  snapshot: string;
  diff: string;
  apply: string;
  imports: string;
  support: string;
  leaves: Leaf[];
};

const repo = process.cwd();
const artifacts = join(repo, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts");
const gluePath = join(repo, "✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs");
const descriptorSchema = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️mutation-descriptor.schema.json";
if (!existsSync(join(repo, descriptorSchema))) throw new Error("run from the semio repository root");

const field = (name: string, rust: string, ts: string, gql: string, proto: string, schema: unknown, optional = false): Field => ({ name, rust, ts, gql, proto, schema, optional });
const leaf = (semantic: string, emoji: string, fields: Field[], diff: string): Leaf => ({ semantic, emoji, variant: semantic.split("-").map(part => part[0]!.toUpperCase() + part.slice(1)).join(""), fields, diff });
const objectSchema = {};
const pathSchema = { type: "array", items: { type: ["object", "integer"] } };
const indexField = field("index", "usize", "number", "Int!", "uint64", { type: "integer", minimum: 0 });
const textField = field("text", "String", "string", "String!", "string", { type: "string" });

const roots: Root[] = [
  {
    artifact: "json", folder: "🔣️json", standard: "🔖️rfc8259", aggregate: "JsonMutation", snapshot: "JsonSnapshot", diff: "JsonDiff", apply: "apply_json_mutation",
    imports: `use crate::artifacts::json::schema::diff::{JsonArrayAdded, JsonArrayDiff, JsonDiff, JsonObjectAdded, JsonObjectDiff, JsonObjectModified, JsonValueDiff};
use crate::artifacts::json::schema::mutation_support::{diff_at_path, resolve, JsonPath};
use crate::artifacts::json::schema::snapshot::JsonValue;
use crate::artifacts::json::JsonSnapshot;`,
    support: `//! 🧰 Shared path addressing for direct JSON mutations.
use crate::artifacts::json::schema::diff::{JsonArrayDiff, JsonArrayModified, JsonDiff, JsonObjectDiff, JsonObjectModified, JsonValueDiff};
use crate::artifacts::json::schema::snapshot::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JsonPathSegment { Key(String), Index(usize) }
pub type JsonPath = Vec<JsonPathSegment>;

pub fn resolve<'a>(root: &'a JsonValue, path: &[JsonPathSegment]) -> Option<&'a JsonValue> {
    let mut node = root;
    for segment in path {
        node = match (segment, node) {
            (JsonPathSegment::Key(key), JsonValue::Object { members }) => &members.iter().find(|member| &member.key == key)?.value,
            (JsonPathSegment::Index(index), JsonValue::Array { items }) => items.get(*index)?,
            _ => return None,
        };
    }
    Some(node)
}

pub fn diff_at_path(path: &[JsonPathSegment], leaf: Option<JsonValueDiff>) -> JsonDiff { JsonDiff { value: leaf.map(|value| wrap_at_path(path, value)) } }
fn wrap_at_path(path: &[JsonPathSegment], leaf: JsonValueDiff) -> JsonValueDiff {
    match path.split_first() {
        None => leaf,
        Some((JsonPathSegment::Key(key), rest)) => JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: key.clone(), diff: wrap_at_path(rest, leaf) }] } },
        Some((JsonPathSegment::Index(index), rest)) => JsonValueDiff::Array { diff: JsonArrayDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonArrayModified { index: *index, diff: wrap_at_path(rest, leaf) }] } },
    }
}
`,
    leaves: [
      leaf("set-member", "✏️", [field("path", "JsonPath", "unknown[]", "[MutationPathSegment!]!", "bytes", pathSchema), field("key", "String", "string", "String!", "string", { type: "string" }), field("value", "JsonValue", "unknown", "MutationPayload!", "bytes", objectSchema)], `match resolve(&base.value, &payload.path) {
                Some(JsonValue::Object { members }) => match members.iter().find(|member| member.key == payload.key) {
                    Some(existing) => { let leaf = crate::artifacts::json::schema::diff::value_diff_between(&existing.value, &payload.value); diff_at_path(&payload.path, leaf.map(|diff| JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), added: Vec::new(), modified: vec![JsonObjectModified { key: payload.key.clone(), diff }] } })) }
                    None => diff_at_path(&payload.path, Some(JsonValueDiff::Object { diff: JsonObjectDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonObjectAdded { index: members.len(), key: payload.key.clone(), item: payload.value.clone() }] } })),
                },
                _ => JsonDiff::default(),
            }`),
      leaf("remove-member", "🗑️", [field("path", "JsonPath", "unknown[]", "[MutationPathSegment!]!", "bytes", pathSchema), field("key", "String", "string", "String!", "string", { type: "string" })], `match resolve(&base.value, &payload.path) { Some(JsonValue::Object { members }) if members.iter().any(|member| member.key == payload.key) => diff_at_path(&payload.path, Some(JsonValueDiff::Object { diff: JsonObjectDiff { removed: vec![payload.key.clone()], modified: Vec::new(), added: Vec::new() } })), _ => JsonDiff::default() }`),
      leaf("insert-array-element", "📥️", [field("path", "JsonPath", "unknown[]", "[MutationPathSegment!]!", "bytes", pathSchema), indexField, field("value", "JsonValue", "unknown", "MutationPayload!", "bytes", objectSchema)], `match resolve(&base.value, &payload.path) { Some(JsonValue::Array { items }) => diff_at_path(&payload.path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: Vec::new(), modified: Vec::new(), added: vec![JsonArrayAdded { index: payload.index.min(items.len()), item: payload.value.clone() }] } })), _ => JsonDiff::default() }`),
      leaf("remove-array-element", "🗑️", [field("path", "JsonPath", "unknown[]", "[MutationPathSegment!]!", "bytes", pathSchema), indexField], `match resolve(&base.value, &payload.path) { Some(JsonValue::Array { items }) if payload.index < items.len() => diff_at_path(&payload.path, Some(JsonValueDiff::Array { diff: JsonArrayDiff { removed: vec![payload.index], modified: Vec::new(), added: Vec::new() } })), _ => JsonDiff::default() }`),
      leaf("set-scalar", "✏️", [field("path", "JsonPath", "unknown[]", "[MutationPathSegment!]!", "bytes", pathSchema), field("value", "JsonValue", "unknown", "MutationPayload!", "bytes", objectSchema)], `match resolve(&base.value, &payload.path) { Some(old) if old != &payload.value => diff_at_path(&payload.path, Some(JsonValueDiff::Replace { value: payload.value.clone() })), _ => JsonDiff::default() }`),
    ],
  },
  {
    artifact: "xml", folder: "📰xml", standard: "🔖️1.0", aggregate: "XmlMutation", snapshot: "XmlSnapshot", diff: "XmlDiff", apply: "apply_xml_mutation",
    imports: `use crate::artifacts::xml::schema::diff::{diff_at_path, XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlChildAdded, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::artifacts::xml::schema::mutation_support::XmlNodePath;
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlNode};
use crate::artifacts::xml::XmlSnapshot;`,
    support: `//! 🧰 Shared node addressing for direct XML mutations.
use crate::artifacts::xml::schema::snapshot::{XmlDocument, XmlNode};
use crate::artifacts::xml::XmlSnapshot;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XmlNodePath(pub Vec<usize>);
impl XmlNodePath {
    pub fn root() -> Self { Self(Vec::new()) }
    pub fn resolve<'a>(&self, root: Option<&'a XmlNode>) -> Option<&'a XmlNode> {
        let mut current = root?;
        for &index in &self.0 { let XmlNode::Element { children, .. } = current else { return None }; current = children.get(index)?; }
        Some(current)
    }
}

pub(crate) fn encode_snapshot(snapshot: &XmlSnapshot) -> String {
    use crate::artifacts::xml::schema::diff::{enc_declaration, enc_doctype, enc_prolog, enc_str, enc_xml_node, encode_option};
    format!("[{},{},{},{},{}]", enc_str(&snapshot.schema), encode_option(&snapshot.doc.root, enc_xml_node), encode_option(&snapshot.doc.doctype, enc_doctype), encode_option(&snapshot.doc.declaration, enc_declaration), enc_prolog(&snapshot.doc.prolog))
}

pub(crate) fn decode_snapshot(value: &str) -> Result<XmlSnapshot, String> {
    use crate::artifacts::xml::schema::diff::{dec_declaration, dec_doctype, dec_prolog, dec_str, dec_xml_node, decode_option, split_top_level, strip_brackets};
    let parts = split_top_level(strip_brackets(value)?, ',');
    let [schema, root, doctype, declaration, prolog] = parts.as_slice() else { return Err(format!("xml snapshot: expected 5 fields, got {}", parts.len())); };
    Ok(XmlSnapshot { schema: dec_str(schema)?, doc: XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)? } })
}

pub(crate) fn encode_snapshot_binary(snapshot: &XmlSnapshot, output: &mut Vec<u8>) {
    use crate::artifacts::xml::schema::diff::{enc_declaration_bin, enc_doctype_bin, enc_prolog_bin, enc_xml_node_bin, write_str_lp};
    write_str_lp(output, &snapshot.schema);
    output.push(u8::from(snapshot.doc.root.is_some()));
    if let Some(root) = &snapshot.doc.root { enc_xml_node_bin(root, output); }
    output.push(u8::from(snapshot.doc.doctype.is_some()));
    if let Some(doctype) = &snapshot.doc.doctype { enc_doctype_bin(doctype, output); }
    output.push(u8::from(snapshot.doc.declaration.is_some()));
    if let Some(declaration) = &snapshot.doc.declaration { enc_declaration_bin(declaration, output); }
    enc_prolog_bin(&snapshot.doc.prolog, output);
}

pub(crate) fn decode_snapshot_binary(reader: &mut store::ByteReader<'_>) -> Result<XmlSnapshot, String> {
    use crate::artifacts::xml::schema::diff::{dec_declaration_bin, dec_doctype_bin, dec_prolog_bin, dec_xml_node_bin, read_str_lp};
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    Ok(XmlSnapshot { schema, doc: XmlDocument { root, doctype, declaration, prolog: dec_prolog_bin(reader)? } })
}
`,
    leaves: [
      leaf("set-declaration", "✏️", [field("declaration", "Option<XmlDeclaration>", "unknown | null", "MutationPayload", "bytes", objectSchema, true)], `XmlDiff { prolog: None, declaration: Some(payload.declaration.clone()), doctype: None, root: None }`),
      leaf("set-doctype", "✏️", [field("doctype", "Option<XmlDoctype>", "unknown | null", "MutationPayload", "bytes", objectSchema, true)], `XmlDiff { prolog: None, declaration: None, doctype: Some(payload.doctype.clone()), root: None }`),
      leaf("insert-element", "📥️", [field("path", "XmlNodePath", "number[]", "[Int!]!", "bytes", pathSchema), indexField, field("node", "XmlNode", "unknown", "MutationPayload!", "bytes", objectSchema)], `diff_at_path(&payload.path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlChildAdded { index: payload.index, item: payload.node.clone() }] }) }))`),
      leaf("remove-element", "🗑️", [field("path", "XmlNodePath", "number[]", "[Int!]!", "bytes", pathSchema), indexField], `diff_at_path(&payload.path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: vec![payload.index], modified: Vec::new(), added: Vec::new() }) }))`),
      leaf("set-attribute", "✏️", [field("path", "XmlNodePath", "number[]", "[Int!]!", "bytes", pathSchema), field("name", "String", "string", "String!", "string", { type: "string" }), field("value", "Option<String>", "string | null", "String", "string", { type: "string" }, true)], `{ let target = payload.path.resolve(base.doc.root.as_ref()); let existing = target.and_then(|node| match node { XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == payload.name), _ => None }); let attributes = match (existing, &payload.value) { (Some(_), Some(value)) => XmlAttributesDiff { removed: Vec::new(), modified: vec![XmlAttrModified { name: payload.name.clone(), value: value.clone() }], added: Vec::new() }, (Some(_), None) => XmlAttributesDiff { removed: vec![payload.name.clone()], modified: Vec::new(), added: Vec::new() }, (None, Some(value)) => { let index = match target { Some(XmlNode::Element { attrs, .. }) => attrs.len(), _ => 0 }; XmlAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlAttrAdded { index, name: payload.name.clone(), value: value.clone() }] } }, (None, None) => XmlAttributesDiff::default() }; diff_at_path(&payload.path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attributes), children: None })) }`),
      leaf("set-text", "✏️", [field("path", "XmlNodePath", "number[]", "[Int!]!", "bytes", pathSchema), textField], `diff_at_path(&payload.path.0, XmlNodeDiff::Text { text: Some(payload.text.clone()) })`),
    ],
  },
  {
    artifact: "svg", folder: "🎨️svg", standard: "🔖️1.1", aggregate: "SvgMutation", snapshot: "SvgSnapshot", diff: "SvgDiff", apply: "apply_svg_mutation",
    imports: `use crate::artifacts::svg::schema::diff::{diff_at_path, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::artifacts::svg::schema::mutation_support::attribute_diff_at_path;
use crate::artifacts::svg::schema::snapshot::{transform_list_to_string, view_box_to_string, NodePath, TransformOp, ViewBox};
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlNode};`,
    support: `//! 🧰 Shared attribute diff construction for direct SVG mutations.
use crate::artifacts::svg::schema::diff::{diff_at_path, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::artifacts::svg::schema::snapshot::node_at;
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{XmlDocument, XmlNode};

pub fn attribute_diff_at_path(base: &SvgSnapshot, path: &[usize], name: &str, value: Option<String>) -> SvgDiff {
    let target = node_at(&base.doc, path).ok();
    let existing = target.and_then(|node| match node { XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == name), _ => None });
    let attributes = match (existing, value) {
        (Some(_), Some(value)) => SvgAttributesDiff { removed: Vec::new(), modified: vec![SvgAttrModified { name: name.to_string(), value }], added: Vec::new() },
        (Some(_), None) => SvgAttributesDiff { removed: vec![name.to_string()], modified: Vec::new(), added: Vec::new() },
        (None, Some(value)) => { let index = match target { Some(XmlNode::Element { attrs, .. }) => attrs.len(), _ => 0 }; SvgAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgAttrAdded { index, name: name.to_string(), value }] } },
        (None, None) => SvgAttributesDiff::default(),
    };
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(attributes), children: None }))
}

pub(crate) fn encode_snapshot(snapshot: &SvgSnapshot) -> String {
    use crate::artifacts::svg::schema::diff::{enc_declaration, enc_doctype, enc_prolog, enc_str, enc_xml_node, encode_option};
    format!("[{},{},{},{},{}]", enc_str(&snapshot.schema), encode_option(&snapshot.doc.root, enc_xml_node), encode_option(&snapshot.doc.doctype, enc_doctype), encode_option(&snapshot.doc.declaration, enc_declaration), enc_prolog(&snapshot.doc.prolog))
}

pub(crate) fn decode_snapshot(value: &str) -> Result<SvgSnapshot, String> {
    use crate::artifacts::svg::schema::diff::{dec_declaration, dec_doctype, dec_prolog, dec_str, dec_xml_node, decode_option, split_top_level, strip_brackets};
    let parts = split_top_level(strip_brackets(value)?, ',');
    let [schema, root, doctype, declaration, prolog] = parts.as_slice() else { return Err(format!("svg snapshot: expected 5 fields, got {}", parts.len())); };
    Ok(SvgSnapshot { schema: dec_str(schema)?, doc: XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)? } })
}

pub(crate) fn encode_snapshot_binary(snapshot: &SvgSnapshot, output: &mut Vec<u8>) {
    use crate::artifacts::svg::schema::diff::{enc_declaration_bin, enc_doctype_bin, enc_prolog_bin, enc_xml_node_bin, write_str_lp};
    write_str_lp(output, &snapshot.schema);
    output.push(u8::from(snapshot.doc.root.is_some()));
    if let Some(root) = &snapshot.doc.root { enc_xml_node_bin(root, output); }
    output.push(u8::from(snapshot.doc.doctype.is_some()));
    if let Some(doctype) = &snapshot.doc.doctype { enc_doctype_bin(doctype, output); }
    output.push(u8::from(snapshot.doc.declaration.is_some()));
    if let Some(declaration) = &snapshot.doc.declaration { enc_declaration_bin(declaration, output); }
    enc_prolog_bin(&snapshot.doc.prolog, output);
}

pub(crate) fn decode_snapshot_binary(reader: &mut store::ByteReader<'_>) -> Result<SvgSnapshot, String> {
    use crate::artifacts::svg::schema::diff::{dec_declaration_bin, dec_doctype_bin, dec_prolog_bin, dec_xml_node_bin, read_str_lp};
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|error| error.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    Ok(SvgSnapshot { schema, doc: XmlDocument { root, doctype, declaration, prolog: dec_prolog_bin(reader)? } })
}
`,
    leaves: [
      leaf("set-declaration", "✏️", [field("declaration", "Option<XmlDeclaration>", "unknown | null", "MutationPayload", "bytes", objectSchema, true)], `SvgDiff { prolog: None, declaration: Some(payload.declaration.clone()), doctype: None, root: None }`),
      leaf("set-doctype", "✏️", [field("doctype", "Option<XmlDoctype>", "unknown | null", "MutationPayload", "bytes", objectSchema, true)], `SvgDiff { prolog: None, declaration: None, doctype: Some(payload.doctype.clone()), root: None }`),
      leaf("insert-element", "📥️", [field("parent", "NodePath", "number[]", "[Int!]!", "bytes", pathSchema), indexField, field("node", "XmlNode", "unknown", "MutationPayload!", "bytes", objectSchema)], `diff_at_path(&payload.parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index: payload.index, item: payload.node.clone() }] }) }))`),
      leaf("remove-element", "🗑️", [field("parent", "NodePath", "number[]", "[Int!]!", "bytes", pathSchema), indexField], `diff_at_path(&payload.parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![payload.index], modified: Vec::new(), added: Vec::new() }) }))`),
      leaf("set-element-name", "✏️", [field("path", "NodePath", "number[]", "[Int!]!", "bytes", pathSchema), field("name", "String", "string", "String!", "string", { type: "string" })], `diff_at_path(&payload.path, SvgNodeDiff::Element(SvgElementDiff { name: Some(payload.name.clone()), attributes: None, children: None }))`),
      leaf("set-attribute", "✏️", [field("path", "NodePath", "number[]", "[Int!]!", "bytes", pathSchema), field("name", "String", "string", "String!", "string", { type: "string" }), field("value", "Option<String>", "string | null", "String", "string", { type: "string" }, true)], `attribute_diff_at_path(base, &payload.path, &payload.name, payload.value.clone())`),
      leaf("set-text", "✏️", [field("path", "NodePath", "number[]", "[Int!]!", "bytes", pathSchema), textField], `diff_at_path(&payload.path, SvgNodeDiff::Text { text: Some(payload.text.clone()) })`),
      leaf("set-view-box", "✏️", [field("path", "NodePath", "number[]", "[Int!]!", "bytes", pathSchema), field("view_box", "Option<ViewBox>", "unknown | null", "MutationPayload", "bytes", objectSchema, true)], `attribute_diff_at_path(base, &payload.path, "viewBox", payload.view_box.as_ref().map(view_box_to_string))`),
      leaf("set-transform", "✏️", [field("path", "NodePath", "number[]", "[Int!]!", "bytes", pathSchema), field("transform", "Option<Vec<TransformOp>>", "unknown[] | null", "[MutationPayload!]", "bytes", { type: "array", items: objectSchema }, true)], `attribute_diff_at_path(base, &payload.path, "transform", payload.transform.as_ref().map(|operations| transform_list_to_string(operations)))`),
    ],
  },
  {
    artifact: "txt", folder: "📄txt", standard: "🔖️utf-8", aggregate: "TxtMutation", snapshot: "TxtSnapshot", diff: "TxtDiff", apply: "apply_txt_mutation",
    imports: `use crate::artifacts::txt::schema::diff::{TxtDiff, TxtLineAdded, TxtLineModified, TxtLinesDiff};
use crate::artifacts::txt::schema::mutation_support::non_canonical_shape;
use crate::artifacts::txt::schema::snapshot::LineEnding;
use crate::artifacts::txt::TxtSnapshot;`,
    support: `//! 🧰 Shared canonical text-carrier representability check.
pub fn non_canonical_shape(line_count: usize, last_line_is_empty: bool, trailing_newline: bool) -> Option<String> {
    if trailing_newline && line_count == 0 { return Some("a document with no lines cannot carry a trailing terminator".to_string()); }
    if !trailing_newline && last_line_is_empty { return Some("an unterminated document cannot end with an empty line".to_string()); }
    None
}
`,
    leaves: [
      leaf("set-trailing-newline", "✏️", [field("value", "bool", "boolean", "Boolean!", "bool", { type: "boolean" })], `{ if let Some(reason) = non_canonical_shape(base.lines.len(), base.lines.last().is_some_and(|line| line.is_empty()), payload.value) { return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new()); } if base.trailing_newline == payload.value { TxtDiff::default() } else { TxtDiff { trailing_newline: Some(payload.value), ..Default::default() } } }`),
      leaf("set-line-ending", "✏️", [field("value", "LineEnding", "'lf' | 'crLf'", "String!", "string", { enum: ["lf", "crLf"] })], `if base.line_ending == payload.value { TxtDiff::default() } else { TxtDiff { line_ending: Some(payload.value), ..Default::default() } }`),
      leaf("insert-line", "📥️", [indexField, textField], `{ let at = payload.index.min(base.lines.len()); let last_empty = if at == base.lines.len() { payload.text.is_empty() } else { base.lines.last().is_some_and(|line| line.is_empty()) }; if let Some(reason) = non_canonical_shape(base.lines.len() + 1, last_empty, base.trailing_newline) { return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new()); } TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![], added: vec![TxtLineAdded { index: payload.index, text: payload.text.clone() }] }), ..Default::default() } }`),
      leaf("remove-line", "🗑️", [indexField], `{ if payload.index >= base.lines.len() { TxtDiff::default() } else { let remaining = base.lines.len() - 1; let last_empty = remaining > 0 && base.lines[if payload.index == remaining { remaining - 1 } else { remaining }].is_empty(); if let Some(reason) = non_canonical_shape(remaining, last_empty, base.trailing_newline) { return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new()); } TxtDiff { lines: Some(TxtLinesDiff { removed: vec![payload.index], modified: vec![], added: vec![] }), ..Default::default() } } }`),
      leaf("set-line", "✏️", [indexField, textField], `{ if payload.index + 1 == base.lines.len() { if let Some(reason) = non_canonical_shape(base.lines.len(), payload.text.is_empty(), base.trailing_newline) { return protocol::MutationOutcome::error("mutation.invariant", reason, Vec::<String>::new()); } } if base.lines.get(payload.index).map_or(true, |current| current == &payload.text) { TxtDiff::default() } else { TxtDiff { lines: Some(TxtLinesDiff { removed: vec![], modified: vec![TxtLineModified { index: payload.index, text: payload.text.clone() }], added: vec![] }), ..Default::default() } } }`),
    ],
  },
];

const snake = (value: string): string => value.replaceAll("-", "_");
const title = (value: string): string => value.split("-").map(part => part[0]!.toUpperCase() + part.slice(1)).join(" ");
const directory = (item: Leaf): string => `${item.emoji}${item.semantic}`;
const payloadName = (item: Leaf): string => `${item.variant}Payload`;
const mutationName = (item: Leaf): string => `${item.variant}Mutation`;
const recordName = (item: Leaf): string => `${({ set: "Set", insert: "Inserted", remove: "Removed" } as Record<string, string>)[item.semantic.split("-")[0]!] ?? item.variant}${item.variant.replace(/^(Set|Insert|Remove)/, "")}`;

function directRust(root: Root, item: Leaf): string {
  const payload = payloadName(item), mutation = mutationName(item);
  return `//! 🧬️ Direct ${item.semantic} mutation owner.
${root.imports}
use serde::{Deserialize, Serialize};

#[path = "📝️text/🦀️component.rs"]
pub mod text;
#[path = "💾️binary/🦀️component.rs"]
pub mod binary;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ${payload} {
${item.fields.map(value => `    pub ${value.name}: ${value.rust},`).join("\n")}
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ${mutation} { Apply(${payload}), Restore(${root.diff}) }

impl protocol::MutationKind<${root.snapshot}, super::${root.aggregate}> for ${mutation} {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "${item.semantic.split("-")[0]}", entity: "${item.semantic.split("-").slice(1).join("-")}", kind: "${item.semantic}", record: "${recordName(item)}" };

    fn diff(&self, base: &${root.snapshot}) -> protocol::MutationOutcome<${root.diff}> {
        match self {
            Self::Apply(payload) => protocol::MutationOutcome::new(${item.diff}),
            Self::Restore(diff) => protocol::MutationOutcome::new(diff.clone()),
        }
    }

    fn inverse(&self, base: &${root.snapshot}) -> Vec<super::${root.aggregate}> {
        let outcome = <Self as protocol::MutationKind<${root.snapshot}, super::${root.aggregate}>>::diff(self, base);
        if !outcome.messages().is_empty() || <${root.diff} as protocol::DiffAlgebra<${root.snapshot}>>::is_empty(outcome.diff()) { return Vec::new(); }
        let inverse = <${root.diff} as protocol::DiffAlgebra<${root.snapshot}>>::inverse(outcome.diff(), base);
        vec![super::${root.aggregate}::${item.variant}(Self::Restore(inverse))]
    }

    fn label(&self) -> String { "${title(item.semantic)}".to_string() }
    fn target(&self) -> Vec<String> { vec!["${item.semantic}".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn semantic_identity_matches_descriptor() { assert_eq!(<${mutation} as protocol::MutationKind<${root.snapshot}, super::super::${root.aggregate}>>::SEMANTICS.kind, "${item.semantic}"); }
}
`;
}

function rootRust(root: Root): string {
  return `//! 🧬️ Transparent ${root.aggregate} aggregate.
use crate::artifacts::${root.artifact}::schema::diff::${root.diff};
use crate::artifacts::${root.artifact}::${root.snapshot};
use serde::{Deserialize, Serialize};

${root.leaves.map(item => `pub use super::${snake(item.semantic)}::{${mutationName(item)}, ${payloadName(item)}};`).join("\n")}
${root.artifact === "json" ? "pub use crate::artifacts::json::schema::mutation_support::{JsonPath, JsonPathSegment};\n" : root.artifact === "xml" ? "pub use crate::artifacts::xml::schema::mutation_support::XmlNodePath;\n" : ""}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]
#[mutations(snapshot = ${root.snapshot}, diff = ${root.diff}, schema = "s.stdio.${root.artifact}")]
pub enum ${root.aggregate} {
${root.leaves.map(item => `    ${item.variant}(${mutationName(item)}),`).join("\n")}
}

pub fn ${root.apply}(snapshot: &mut ${root.snapshot}, mutation: &${root.aggregate}) -> protocol::MutationOutcome<${root.diff}> {
    let outcome = <${root.aggregate} as protocol::Mutation<${root.snapshot}>>::diff(mutation, snapshot);
    if let Ok(next) = protocol::MutationDiff::apply(outcome.diff(), snapshot) { *snapshot = next; }
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::SemanticMutation;
    #[test]
    fn aggregate_roster_is_exact() { assert_eq!(${root.aggregate}::kinds().len(), ${root.leaves.length}); }
}
`;
}

function rootGrammar(root: Root): string {
  return `dialect grammar
grammar ${root.artifact}.mutations
extension ${root.artifact}
start document

# Direct descriptor identities: ${root.leaves.map(item => item.semantic).join(", ")}.
document = "${root.artifact}-mutation" "payload" "=" hex
`;
}

function rootProtocol(root: Root): string {
  return `dialect protocol
protocol ${root.artifact}.mutations
version 1
schema stdio.${root.artifact}
start record

# Direct descriptor identities and tags: ${root.leaves.map((item, index) => `${item.semantic}=${index + 1}`).join(", ")}.
framing record
header fixed 0
chain payload bytes
`;
}

function writeLeaf(root: Root, mutationRoot: string, item: Leaf, index: number): void {
  const leafRoot = join(mutationRoot, directory(item));
  mkdirSync(join(leafRoot, "📝️text"), { recursive: true });
  mkdirSync(join(leafRoot, "💾️binary"), { recursive: true });
  writeFileSync(join(leafRoot, "🦀️component.rs"), directRust(root, item));
  writeFileSync(join(leafRoot, "📝️text/🦀️component.rs"), `//! 📝️ Operation-specific text payload codec for ${item.semantic}.\nuse super::${payloadName(item)};\npub const TEXT_OPCODE: &str = "${item.semantic}";\npub fn encode_payload(value: &${payloadName(item)}) -> Result<String, String> { serde_json::to_string(value).map_err(|error| error.to_string()) }\npub fn decode_payload(value: &str) -> Result<${payloadName(item)}, String> { serde_json::from_str(value).map_err(|error| error.to_string()) }\n`);
  writeFileSync(join(leafRoot, "💾️binary/🦀️component.rs"), `//! 💾️ Operation-specific binary payload codec for ${item.semantic}/${item.variant}.\nuse super::${payloadName(item)};\npub const BINARY_TAG: u32 = ${index + 1};\npub fn encode_payload(value: &${payloadName(item)}) -> Result<Vec<u8>, String> { serde_json::to_vec(value).map_err(|error| error.to_string()) }\npub fn decode_payload(value: &[u8]) -> Result<${payloadName(item)}, String> { serde_json::from_slice(value).map_err(|error| error.to_string()) }\n`);
  writeFileSync(join(leafRoot, "🟦️component.ts"), `/** 🧬 ${item.semantic} direct payload. */\nexport interface ${payloadName(item)} { ${item.fields.map(value => `readonly ${value.name.replaceAll(/_([a-z])/g, (_, letter) => letter.toUpperCase())}${value.optional ? "?" : ""}: ${value.ts}`).join("; ")} }\n`);
  writeFileSync(join(leafRoot, "🔗️component.graphql"), `# 🧬 ${item.semantic}/${item.variant}\ninput ${payloadName(item)} { ${item.fields.map(value => `${value.name.replaceAll(/_([a-z])/g, (_, letter) => letter.toUpperCase())}: ${value.gql}`).join(" ")} }\n`);
  writeFileSync(join(leafRoot, "🛰️component.proto"), `syntax = "proto3";\npackage stdio.${root.artifact}.mutation;\n// ${item.semantic}/${item.variant}\nmessage ${payloadName(item)} { ${item.fields.map((value, fieldIndex) => `${value.proto} ${value.name} = ${fieldIndex + 1};`).join(" ")} }\n`);
  const schema = { $schema: "http://json-schema.org/draft-07/schema#", title: payloadName(item), type: "object", additionalProperties: false, required: item.fields.filter(value => !value.optional).map(value => value.name.replaceAll(/_([a-z])/g, (_, letter) => letter.toUpperCase())), properties: Object.fromEntries(item.fields.map(value => [value.name.replaceAll(/_([a-z])/g, (_, letter) => letter.toUpperCase()), value.schema])) };
  writeFileSync(join(leafRoot, "🔣️payload.schema.json"), `${JSON.stringify(schema, null, 2)}\n`);
  const descriptor = { schemaVersion: 1, owner: relative(repo, leafRoot), semanticKind: item.semantic, displayName: title(item.semantic), emoji: item.emoji, aggregateVariant: item.variant, payloadSchema: "🔣️payload.schema.json", textOpcode: item.semantic, binaryTag: index + 1, invertibility: "explicit-mutation", diffParticipation: "detect", outcomeClasses: root.artifact === "txt" ? ["applied", "error"] : ["applied"], composition: "atomic", requiredLanguageSurfaces: ["rust", "typescript", "graphql", "protobuf", "json-schema", "text", "binary"] };
  writeFileSync(join(leafRoot, "🔣️component.json"), `${JSON.stringify(descriptor, null, 2)}\n`);
}

function writeRoot(root: Root): void {
  const subset = join(artifacts, root.folder, "🏅️standards", root.standard, "🪆️subsets/✳️any");
  const schemaRoot = join(subset, "🧬️schema");
  const mutationRoot = join(schemaRoot, "🧬️mutations");
  const supportRoot = join(schemaRoot, "🔨️modules/🧬️mutation-support");
  mkdirSync(supportRoot, { recursive: true });
  writeFileSync(join(supportRoot, "🦀️component.rs"), root.support);
  root.leaves.forEach((item, index) => writeLeaf(root, mutationRoot, item, index));
  writeFileSync(join(mutationRoot, "🦀️component.rs"), rootRust(root));
  writeFileSync(join(mutationRoot, "🟦️component.ts"), `/** 🧬 Transparent ${root.aggregate} TypeScript aggregate. */\n${root.leaves.map(item => `import type { ${payloadName(item)} } from './${directory(item)}/🟦️component.ts';`).join("\n")}\nexport type ${root.aggregate} =\n${root.leaves.map(item => `  | { readonly mutation: '${item.semantic}'; readonly payload: { readonly phase: 'apply'; readonly value: ${payloadName(item)} } }`).join("\n")};\n`);
  writeFileSync(join(mutationRoot, "🔗️component.graphql"), `# 🧬 Transparent ${root.aggregate} descriptor roster.\n${root.leaves.map(item => `# ${item.semantic}/${item.variant}`).join("\n")}\nenum ${root.aggregate}Kind {\n${root.leaves.map(item => `  ${item.semantic.replaceAll("-", "_").toUpperCase()}`).join("\n")}\n}\nscalar MutationPayload\ninput ${root.aggregate}Input { kind: ${root.aggregate}Kind!, payload: MutationPayload! }\n`);
  writeFileSync(join(mutationRoot, "🛰️component.proto"), `syntax = "proto3";\npackage stdio.${root.artifact}.mutation;\n${root.leaves.map(item => `import "${directory(item)}/🛰️component.proto";`).join("\n")}\nmessage ${root.aggregate} { oneof mutation { ${root.leaves.map((item, index) => `${payloadName(item)} ${snake(item.semantic)} = ${index + 1};`).join(" ")} } }\n`);
  const aggregateSchema = { $schema: "http://json-schema.org/draft-07/schema#", title: root.aggregate, oneOf: root.leaves.map(item => ({ type: "object", additionalProperties: false, required: ["mutation", "payload"], properties: { mutation: { const: item.semantic }, payload: { $ref: `${directory(item)}/🔣️payload.schema.json` } } })) };
  writeFileSync(join(mutationRoot, "🔣️component.json"), `${JSON.stringify(aggregateSchema, null, 2)}\n`);
  writeFileSync(join(mutationRoot, "📝️text/🦀️component.rs"), `//! 📝️ Generic framing and descriptor roster for the transparent ${root.aggregate}.\nuse crate::artifacts::${root.artifact}::schema::mutations::${root.aggregate};\npub const TEXT_OPCODES: &[&str] = &[${root.leaves.map(item => `"${item.semantic}"`).join(", ")}];\nfn error(detail: impl Into<String>) -> store::TextError { store::TextError::new(detail.into(), dsl::TextSpan::at(1, 1)) }\nfn encode_hex(bytes: &[u8]) -> String { const HEX: &[u8; 16] = b"0123456789abcdef"; let mut text = String::with_capacity(bytes.len() * 2); for byte in bytes { text.push(HEX[(byte >> 4) as usize] as char); text.push(HEX[(byte & 0x0f) as usize] as char); } text }\nfn decode_hex(value: &str) -> Result<Vec<u8>, String> { fn nibble(value: u8) -> Option<u8> { if value.is_ascii_digit() { return Some(value - b'0'); } (b'a'..=b'f').contains(&value).then_some(value - b'a' + 10) } if value.len() % 2 != 0 { return Err("payload must be lowercase hexadecimal".to_string()); } value.as_bytes().chunks_exact(2).map(|pair| Ok((nibble(pair[0]).ok_or_else(|| "invalid hexadecimal".to_string())? << 4) | nibble(pair[1]).ok_or_else(|| "invalid hexadecimal".to_string())?)).collect() }\nimpl protocol::OpText for ${root.aggregate} { fn print_op(&self) -> String { format!("${root.artifact}-mutation payload={}", encode_hex(&serde_json::to_vec(self).expect("aggregate serialization"))) } fn parse_op(line: &str) -> Result<Self, store::TextError> { let value = line.strip_prefix("${root.artifact}-mutation payload=").ok_or_else(|| error("expected aggregate payload"))?; let bytes = decode_hex(value).map_err(error)?; serde_json::from_slice(&bytes).map_err(|cause| error(cause.to_string())) } }\n`);
  writeFileSync(join(mutationRoot, "💾️binary/🦀️component.rs"), `//! 💾️ Generic framing and descriptor roster for the transparent ${root.aggregate}.\nuse crate::artifacts::${root.artifact}::schema::mutations::${root.aggregate};\npub const BINARY_TAGS: &[(&str, u32)] = &[${root.leaves.map((item, index) => `("${item.semantic}", ${index + 1})`).join(", ")}];\nimpl protocol::OpBinary for ${root.aggregate} { fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { serde_json::to_vec(self).map_err(|cause| protocol::ProtocolError::Malformed { what: "${root.artifact} mutation", offset: 0, detail: cause.to_string() }) } fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { serde_json::from_slice(bytes).map_err(|cause| protocol::ProtocolError::Malformed { what: "${root.artifact} mutation", offset: 0, detail: cause.to_string() }) } }\n`);
  writeFileSync(join(mutationRoot, "📝️text/📖️component.grammar.semio"), rootGrammar(root));
  writeFileSync(join(mutationRoot, "💾️binary/📡️component.protocol.semio"), rootProtocol(root));
  if (root.artifact === "svg") {
    writeFileSync(join(mutationRoot, "📝️text/🔤️component.ebnf"), `(* 🧬️ Generic stdio.svg mutation frame. Direct identities: ${root.leaves.map(item => item.semantic).join(", ")}. *)\ndocument = "svg-mutation", " payload=", hex ;\nhex = hex-character, { hex-character } ;\nhex-character = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "a" | "b" | "c" | "d" | "e" | "f" ;\n`);
    writeFileSync(join(mutationRoot, "📝️text/🅰️component.g4"), `grammar Stdio_svg_mutations;\n// 🧬️ Direct identities: ${root.leaves.map(item => item.semantic).join(", ")}.\ndocument: 'svg-mutation payload=' HEX EOF;\nHEX: [0-9a-f]+;\n`);
  }
  const rootTextCodec = join(mutationRoot, "📝️text/🦀️component.rs");
  writeFileSync(rootTextCodec, readFileSync(rootTextCodec, "utf8").replace("pub const TEXT_OPCODES", `pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");\npub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");\npub const TEXT_OPCODES`));
  const rootBinaryCodec = join(mutationRoot, "💾️binary/🦀️component.rs");
  writeFileSync(rootBinaryCodec, readFileSync(rootBinaryCodec, "utf8").replace("pub const BINARY_TAGS", `pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");\npub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");\npub const BINARY_TAGS`));
  const oraclePath = join(subset, "🧪️oracle/🔣️component.json");
  const oracle = JSON.parse(readFileSync(oraclePath, "utf8"));
  const catalog = oracle.mutationCatalogs.find((value: { id: string }) => value.id.includes(root.artifact));
  catalog.kinds = root.leaves.map(item => item.semantic);
  catalog.vectors = root.leaves.map(item => ({ mutationId: item.semantic, sourceMutationDirectoryName: directory(item), mutationDirectoryName: directory(item), scenarios: [] }));
  writeFileSync(oraclePath, `${JSON.stringify(oracle, null, 2)}\n`);
  rmSync(join(mutationRoot, "📄set-snapshot"), { recursive: true, force: true });
}

function generate() {
  for (const root of roots) writeRoot(root);
  let glue = readFileSync(gluePath, "utf8");
  for (const root of roots) {
    const prefix = `../../🗿️artifacts/${root.folder}/🏅️standards/${root.standard}/🪆️subsets/✳️any/🧬️schema`;
    const componentMount = `                                #[path = "${prefix}/🧬️mutations/🦀️component.rs"]\n                                mod component;\n                                pub use component::*;`;
    const leafMounts = root.leaves.map(item => `                                #[path = "${prefix}/🧬️mutations/${directory(item)}/🦀️component.rs"]\n                                pub mod ${snake(item.semantic)};`).join("\n");
    if (!glue.includes(componentMount)) throw new Error(`missing mutation mount for ${root.artifact}`);
    if (!glue.includes(`${prefix}/🧬️mutations/${directory(root.leaves[0])}/🦀️component.rs`)) glue = glue.replace(componentMount, `${componentMount}\n${leafMounts}`);
    const oldSnapshotMount = `                                #[path = "."]\n                                pub mod set_snapshot {\n                                    #[path = "${prefix}/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]\n                                    pub mod diff;\n                                    #[path = "${prefix}/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]\n                                    pub mod inverse;\n                                    #[path = "${prefix}/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]\n                                    pub mod mutation;\n                                }\n`;
    if (glue.includes(oldSnapshotMount)) glue = glue.replace(oldSnapshotMount, "");
    const schemaMount = `                            #[path = "${prefix}/🦀️component.rs"]\n                            mod component;\n                            pub use component::*;`;
    if (!glue.includes(schemaMount)) throw new Error(`missing schema mount for ${root.artifact}`);
    const supportMount = `                            #[path = "${prefix}/🔨️modules/🧬️mutation-support/🦀️component.rs"]\n                            pub mod mutation_support;`;
    if (!glue.includes(`${prefix}/🔨️modules/🧬️mutation-support/🦀️component.rs`)) glue = glue.replace(schemaMount, `${schemaMount}\n${supportMount}`);
  }
  writeFileSync(gluePath, glue);
}

function validate() {
  const ajv = new Ajv({ allErrors: true, strict: false });
  const validateDescriptor = ajv.compile(JSON.parse(readFileSync(join(repo, descriptorSchema), "utf8")));
  let checked = 0;
  for (const root of roots) {
    const subset = join(artifacts, root.folder, "🏅️standards", root.standard, "🪆️subsets/✳️any");
    const mutationRoot = join(subset, "🧬️schema/🧬️mutations");
    const aggregateSchema = JSON.parse(readFileSync(join(mutationRoot, "🔣️component.json"), "utf8"));
    const grammar = readFileSync(join(mutationRoot, "📝️text/📖️component.grammar.semio"), "utf8");
    const binaryProtocol = readFileSync(join(mutationRoot, "💾️binary/📡️component.protocol.semio"), "utf8");
    if (!grammar.includes(`document = "${root.artifact}-mutation" "payload" "=" hex`) || grammar.includes("no-mutation") || grammar.includes("set-snapshot")) throw new Error(`${root.artifact} root grammar framing mismatch`);
    if (!binaryProtocol.includes("header fixed 0") || !binaryProtocol.includes("chain payload bytes") || binaryProtocol.includes("NoMutation") || binaryProtocol.includes("SetSnapshot")) throw new Error(`${root.artifact} root protocol framing mismatch`);
    const oracle = JSON.parse(readFileSync(join(subset, "🧪️oracle/🔣️component.json"), "utf8"));
    const catalog = oracle.mutationCatalogs.find((value: { id: string }) => value.id.includes(root.artifact));
    if (!catalog) throw new Error(`missing ${root.artifact} oracle catalog`);
    for (const item of root.leaves) {
      const leafRoot = join(mutationRoot, directory(item));
      const descriptor = JSON.parse(readFileSync(join(leafRoot, "🔣️component.json"), "utf8"));
      if (!validateDescriptor(descriptor)) throw new Error(`${root.artifact}/${item.semantic} descriptor: ${ajv.errorsText(validateDescriptor.errors)}`);
      ajv.compile(JSON.parse(readFileSync(join(leafRoot, "🔣️payload.schema.json"), "utf8")));
      for (const surface of ["🦀️component.rs", "🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"]) if (!existsSync(join(leafRoot, surface))) throw new Error(`missing ${root.artifact}/${item.semantic}/${surface}`);
      for (const aggregateSurface of ["🟦️component.ts", "🔗️component.graphql", "🛰️component.proto", "📝️text/🦀️component.rs", "💾️binary/🦀️component.rs"]) if (!readFileSync(join(mutationRoot, aggregateSurface), "utf8").includes(item.semantic)) throw new Error(`root ${aggregateSurface} omits ${item.semantic}`);
      if (!aggregateSchema.oneOf.some((value: { properties?: { mutation?: { const?: string } } }) => value.properties?.mutation?.const === item.semantic)) throw new Error(`root schema omits ${item.semantic}`);
      if (!grammar.includes(item.semantic) || !binaryProtocol.includes(`${item.semantic}=${root.leaves.indexOf(item) + 1}`)) throw new Error(`root codec specifications omit ${root.artifact}/${item.semantic}`);
      checked += 1;
    }
    if (root.artifact === "svg") {
      const ebnf = readFileSync(join(mutationRoot, "📝️text/🔤️component.ebnf"), "utf8");
      const antlr = readFileSync(join(mutationRoot, "📝️text/🅰️component.g4"), "utf8");
      if (!ebnf.includes('document = "svg-mutation", " payload=", hex') || !antlr.includes("document: 'svg-mutation payload=' HEX EOF;") || [ebnf, antlr].some(value => value.includes("no-mutation") || value.includes("set-snapshot"))) throw new Error("svg secondary grammar framing mismatch");
    }
    const kinds = root.leaves.map(item => item.semantic);
    if (JSON.stringify(catalog.kinds) !== JSON.stringify(kinds) || JSON.stringify(catalog.vectors.map((value: { mutationId: string }) => value.mutationId)) !== JSON.stringify(kinds)) throw new Error(`${root.artifact} oracle catalog mismatch`);
  }
  console.log(`Ajv descriptors=${checked} payloads=${checked} catalogs=${roots.length} surfaces=${checked * 6} rootCodecs=${roots.length * 2 + 2} errors=0`);
}

function fixtures() {
  const payload = { mutation: "insertLine", payload: { phase: "apply", value: { index: 1, text: "x" } } };
  writeFileSync(join(artifacts, "📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/📡️example.spr.semio"), JSON.stringify(payload));
}

const command = process.argv[2] ?? "generate";
if (command === "generate") generate();
else if (command === "validate") validate();
else if (command === "fixtures") fixtures();
else throw new Error(`unknown command: ${command}`);
