//! binary rep for stdio.xml 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1_0::subsets::base::schema::diff::*;
use crate::schema::snapshot::{validate_xml_document_boundaries, XmlAttr, XmlDeclaration, XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlNode, XmlQuote};
use crate::XmlSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_contract::deserialize_double_option;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_prolog_bin(prolog: &Vec<XmlNode>, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, prolog.len() as u64);
    for node in prolog {
        enc_xml_node_bin(node, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_prolog_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<XmlNode>, String> {
    let count = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    (0..count).map(|_| dec_xml_node_bin(reader)).collect()
}

/// 🧪️ P2-FG1: real LEB128-varint-framed binary primitives (length-prefixed bytes/utf8) backing the
/// upgraded `OpBinary`/`DiffCodec` frames below (and, via re-export, `../🧬️mutations/🦀️.rs`'s
/// own upgraded `OpBinary`) -- reuses `store::pack_rt::write_varint_u64`/`store::ByteReader` rather
/// than reinventing varint encode/decode, same shape json's own `write_str_lp`/`read_str_lp` uses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_doctype_bin(doctype: &XmlDoctype, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, doctype.prolog_position);
    write_str_lp(out, &doctype.name);
    match &doctype.external_id {
        None => out.push(0),
        Some(XmlExternalId::System { system_id }) => {
            out.push(1);
            write_str_lp(out, system_id);
        }
        Some(XmlExternalId::Public { public_id, system_id }) => {
            out.push(2);
            write_str_lp(out, public_id);
            write_str_lp(out, system_id);
        }
    }
    store::pack_rt::write_varint_u64(out, doctype.declarations.len() as u64);
    for declaration in &doctype.declarations {
        match declaration {
            XmlDtdDeclaration::Entity { parameter, name, value } => {
                out.push(1);
                out.push(u8::from(*parameter));
                write_str_lp(out, name);
                write_str_lp(out, value);
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_doctype_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlDoctype, String> {
    let prolog_position = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let name = read_str_lp(reader)?;
    let external_id = match reader.read_u8().map_err(|error| error.to_string())? {
        0 => None,
        1 => Some(XmlExternalId::System { system_id: read_str_lp(reader)? }),
        2 => Some(XmlExternalId::Public { public_id: read_str_lp(reader)?, system_id: read_str_lp(reader)? }),
        tag => return Err(format!("unknown XML external id tag {tag}")),
    };
    let count = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    let mut declarations = Vec::with_capacity(count);
    for _ in 0..count {
        match reader.read_u8().map_err(|error| error.to_string())? {
            1 => declarations.push(XmlDtdDeclaration::Entity { parameter: reader.read_u8().map_err(|error| error.to_string())? != 0, name: read_str_lp(reader)?, value: read_str_lp(reader)? }),
            tag => return Err(format!("unknown XML DTD declaration tag {tag}")),
        }
    }
    Ok(XmlDoctype { prolog_position, name, external_id, declarations })
}

/// 🧪️ P2-FG1: real recursive binary twins of [`enc_xml_node`]/[`dec_xml_node`] and
/// [`enc_declaration`]/[`dec_declaration`] above -- a 1-byte kind tag (`0`=Element/`1`=Text/
/// `2`=CData/`3`=Comment/`4`=ProcessingInstruction, distinct numbering from the text codec's letter
/// tags) followed by the real payload (length-prefixed strings for scalars, a varint COUNT then
/// that many recursively-encoded elements for `Element`'s attrs/children -- genuinely recursive,
/// not text-as-bytes). Backs the upgraded `OpBinary` frame (`../🧬️mutations/🦀️.rs`) and
/// the `Replace`/added-item payloads inside [`enc_node_diff_bin`] below. `pub(crate)` so the
/// sibling `../🧬️mutations/🦀️.rs` (same artifact, different facet module) can reuse these
/// rather than duplicating them a second time, matching this file's own existing text-codec reuse
/// convention.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attr_bin(a: &XmlAttr, out: &mut Vec<u8>) {
    write_str_lp(out, &a.name);
    write_str_lp(out, &a.value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attr_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttr, String> {
    let name = read_str_lp(reader)?;
    let value = read_str_lp(reader)?;
    Ok(XmlAttr { name, value })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_declaration_bin(d: &XmlDeclaration, out: &mut Vec<u8>) {
    write_str_lp(out, &d.version);
    out.push(if d.encoding.is_some() { 1 } else { 0 });
    if let Some(encoding) = &d.encoding {
        write_str_lp(out, encoding);
    }
    out.push(if d.standalone.is_some() { 1 } else { 0 });
    if let Some(standalone) = d.standalone {
        out.push(if standalone { 1 } else { 0 });
    }
    out.push(u8::from(!d.quote.is_double()));
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_declaration_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlDeclaration, String> {
    let version = read_str_lp(reader)?;
    let encoding = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
    let standalone = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(reader.read_u8().map_err(|e| e.to_string())? != 0) } else { None };
    let quote = if reader.read_u8().map_err(|e| e.to_string())? != 0 { XmlQuote::Single } else { XmlQuote::Double };
    Ok(XmlDeclaration { version, encoding, standalone, quote })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn enc_xml_node_bin(node: &XmlNode, out: &mut Vec<u8>) {
    let mut stack = vec![node];
    while let Some(node) = stack.pop() {
        match node {
            XmlNode::Element { name, attrs, children } => {
                out.push(0); write_str_lp(out, name); store::pack_rt::write_varint_u64(out, attrs.len() as u64);
                for attr in attrs { enc_attr_bin(attr, out); }
                store::pack_rt::write_varint_u64(out, children.len() as u64); stack.extend(children.iter().rev());
            },
            XmlNode::Text { text } | XmlNode::CData { text } | XmlNode::Comment { text } => { out.push(match node { XmlNode::Text { .. } => 1, XmlNode::CData { .. } => 2, _ => 3 }); write_str_lp(out, text); },
            XmlNode::ProcessingInstruction { target, data } => { out.push(4); write_str_lp(out, target); write_str_lp(out, data); },
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_node_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNode, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = read_str_lp(reader)?;
            let attr_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut attrs = Vec::with_capacity(attr_count as usize);
            for _ in 0..attr_count {
                attrs.push(dec_attr_bin(reader)?);
            }
            let child_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut children = Vec::with_capacity(child_count as usize);
            for _ in 0..child_count {
                children.push(dec_xml_node_bin(reader)?);
            }
            Ok(XmlNode::Element { name, attrs, children })
        }
        1 => Ok(XmlNode::Text { text: read_str_lp(reader)? }),
        2 => Ok(XmlNode::CData { text: read_str_lp(reader)? }),
        3 => Ok(XmlNode::Comment { text: read_str_lp(reader)? }),
        4 => {
            let target = read_str_lp(reader)?;
            let data = read_str_lp(reader)?;
            Ok(XmlNode::ProcessingInstruction { target, data })
        }
        other => Err(format!("xml node binary: unknown tag {other}")),
    }
}

/// 🧪️ P2-FG1: real recursive binary twin of [`enc_node_diff`]/[`dec_node_diff`] -- same 1-byte tag
/// numbering scheme as [`enc_xml_node_bin`] (`0`=Element/`1`=Text) plus `2`=`Replace` (needs its own
/// arm since `Replace` wraps a whole [`XmlNode`], not a bare scalar payload). `attrs`/`children`
/// collection triples encode as three varint-counted, recursively-encoded lists (removed/modified/
/// added) -- genuinely structured binary, backing the upgraded `DiffBinary::encode_diff`/
/// `decode_diff` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_node_diff_bin(diff: &XmlNodeDiff, out: &mut Vec<u8>) {
    match diff {
        XmlNodeDiff::Element(e) => {
            out.push(0);
            out.push(if e.name.is_some() { 1 } else { 0 });
            if let Some(name) = &e.name {
                write_str_lp(out, name);
            }
            out.push(if e.attributes.is_some() { 1 } else { 0 });
            if let Some(attrs) = &e.attributes {
                enc_attrs_diff_bin(attrs, out);
            }
            out.push(if e.children.is_some() { 1 } else { 0 });
            if let Some(children) = &e.children {
                enc_children_diff_bin(children, out);
            }
        }
        XmlNodeDiff::Text { text } => {
            out.push(1);
            out.push(if text.is_some() { 1 } else { 0 });
            if let Some(text) = text {
                write_str_lp(out, text);
            }
        }
        XmlNodeDiff::Replace { node } => {
            out.push(2);
            out.push(if node.is_some() { 1 } else { 0 });
            if let Some(node) = node {
                enc_xml_node_bin(node, out);
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_node_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNodeDiff, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let name = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            let attributes = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_attrs_diff_bin(reader)?) } else { None };
            let children = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_children_diff_bin(reader)?) } else { None };
            Ok(XmlNodeDiff::Element(XmlElementDiff { name, attributes, children }))
        }
        1 => {
            let text = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(read_str_lp(reader)?) } else { None };
            Ok(XmlNodeDiff::Text { text })
        }
        2 => {
            let node = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
            Ok(XmlNodeDiff::Replace { node })
        }
        other => Err(format!("xml node diff binary: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_attrs_diff_bin(diff: &XmlAttributesDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for name in &diff.removed {
        write_str_lp(out, name);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        write_str_lp(out, &entry.name);
        write_str_lp(out, &entry.value);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        write_str_lp(out, &entry.name);
        write_str_lp(out, &entry.value);
    }
    store::pack_rt::write_varint_u64(out, diff.order.len() as u64);
    for name in &diff.order {
        write_str_lp(out, name);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_attrs_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlAttributesDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(read_str_lp(reader)?);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        modified.push(XmlAttrModified { name, value });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let name = read_str_lp(reader)?;
        let value = read_str_lp(reader)?;
        added.push(XmlAttrAdded { name, value });
    }
    let order_count = reader.read_varint_u64().map_err(|error| error.to_string())?;
    let mut order = Vec::with_capacity(order_count as usize);
    for _ in 0..order_count {
        order.push(read_str_lp(reader)?);
    }
    Ok(XmlAttributesDiff { removed, modified, added, order })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_children_diff_bin(diff: &XmlChildrenDiff, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, diff.removed.len() as u64);
    for index in &diff.removed {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
    store::pack_rt::write_varint_u64(out, diff.modified.len() as u64);
    for entry in &diff.modified {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_node_diff_bin(&entry.diff, out);
    }
    store::pack_rt::write_varint_u64(out, diff.added.len() as u64);
    for entry in &diff.added {
        store::pack_rt::write_varint_u64(out, entry.index as u64);
        enc_xml_node_bin(&entry.item, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_children_diff_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlChildrenDiff, String> {
    let removed_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut removed = Vec::with_capacity(removed_count as usize);
    for _ in 0..removed_count {
        removed.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    let modified_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut modified = Vec::with_capacity(modified_count as usize);
    for _ in 0..modified_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let diff = dec_node_diff_bin(reader)?;
        modified.push(XmlChildModified { index, diff });
    }
    let added_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut added = Vec::with_capacity(added_count as usize);
    for _ in 0..added_count {
        let index = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
        let item = dec_xml_node_bin(reader)?;
        added.push(XmlChildAdded { index, item });
    }
    Ok(XmlChildrenDiff { removed, modified, added })
}

impl protocol::DiffBinary for XmlDiff {
/// 🧪️ P2-FG1: REAL binary frame (`format u8 | flags u8 | [declaration][doctype][root]`),
/// matching `../💾️binary/📡️.protocol.semio`'s `header fixed 2` + `chain payload
/// bytes` shape — upgraded from F6's `print_diff().into_bytes()` text-as-binary shortcut (100%
/// of stdio's `DiffCodec` impls were still on that shortcut per the P2-W0 census). `flags` bits
/// 0/1/2 mark `declaration`/`doctype`/`root` presence; each present field's own tri-state/
/// recursive payload follows in that fixed order.
fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
    let mut flags: u8 = 0;
    if self.declaration.is_some() {
        flags |= 0b001;
    }
    if self.doctype.is_some() {
        flags |= 0b010;
    }
    if self.root.is_some() {
        flags |= 0b100;
    }
    if self.prolog.is_some() {
        flags |= 0b1000;
    }
    if self.epilog.is_some() {
        flags |= 0b1_0000;
    }
    let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, flags];
    if let Some(declaration) = &self.declaration {
        out.push(if declaration.is_some() { 1 } else { 0 });
        if let Some(declaration) = declaration {
            enc_declaration_bin(declaration, &mut out);
        }
    }
    if let Some(doctype) = &self.doctype {
        out.push(if doctype.is_some() { 1 } else { 0 });
        if let Some(doctype) = doctype {
            enc_doctype_bin(doctype, &mut out);
        }
    }
    if let Some(root) = &self.root {
        enc_node_diff_bin(root, &mut out);
    }
    if let Some(prolog) = &self.prolog {
        enc_prolog_bin(prolog, &mut out);
    }
    if let Some(epilog) = &self.epilog {
        enc_prolog_bin(epilog, &mut out);
    }
    Ok(out)
}
fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
    let mut reader = store::ByteReader::new(bytes);
    let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
    let _format = reader.read_u8().map_err(|e| malformed("diff format", 0, e.to_string()))?;
    let flags = reader.read_u8().map_err(|e| malformed("diff flags", 1, e.to_string()))?;
    let declaration = if flags & 0b001 != 0 {
        let has = reader.read_u8().map_err(|e| malformed("diff declaration presence", reader.position(), e.to_string()))?;
        Some(if has != 0 { Some(dec_declaration_bin(&mut reader).map_err(|e| malformed("diff declaration", reader.position(), e))?) } else { None })
    } else {
        None
    };
    let doctype = if flags & 0b010 != 0 {
        let has = reader.read_u8().map_err(|e| malformed("diff doctype presence", reader.position(), e.to_string()))?;
        Some(if has != 0 { Some(dec_doctype_bin(&mut reader).map_err(|e| malformed("diff doctype", reader.position(), e))?) } else { None })
    } else {
        None
    };
    let root = if flags & 0b100 != 0 { Some(dec_node_diff_bin(&mut reader).map_err(|e| malformed("diff root", reader.position(), e))?) } else { None };
    let prolog = if flags & 0b1000 != 0 { Some(dec_prolog_bin(&mut reader).map_err(|e| malformed("diff prolog", reader.position(), e))?) } else { None };
    let epilog = if flags & 0b1_0000 != 0 { Some(dec_prolog_bin(&mut reader).map_err(|e| malformed("diff epilog", reader.position(), e))?) } else { None };
    Ok(XmlDiff { prolog, epilog, declaration, doctype, root })
}
}
}
pub use diff_codec::*;

impl crate::schema::diff::XmlChildrenDiff {
    pub fn encode_binary(&self, output: &mut Vec<u8>) {
        enc_children_diff_bin(self, output);
    }
}

impl crate::schema::diff::XmlChildrenDiff {
    pub fn decode_binary(reader: &mut store::ByteReader<'_>) -> Result<Self, String> {
        dec_children_diff_bin(reader)
    }
}
