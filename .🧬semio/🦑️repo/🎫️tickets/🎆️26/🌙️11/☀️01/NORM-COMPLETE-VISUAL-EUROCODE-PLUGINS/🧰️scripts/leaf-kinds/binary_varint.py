"""Rewrites an artifact's binary mutation codec to a varint tag looked up in `📡️.protocol.semio` (schema-first) plus the canonical JSON body; regenerates the protocol from the mutation enum. Usage: python -I binary_varint.py en1993"""
import os as _o, sys as _s; _o.environ.get("TH_FORCE") or (_s.stderr.write("unsafe against the current tree (destructive or not idempotent), refusing to run (TH_FORCE=1 forces)\n"), _s.exit(2))  # unsafe-guard
import glob, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from common import *

CODEC = '''mod native_codec {
use %(m)s::%(P)sMutation;

const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");

fn malformed(what: &'static str, offset: usize, detail: String) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what, offset, detail }
}

fn kind_of(json: &str) -> Option<String> {
    let rest = &json[json.find("\\"mutation\\"")? + "\\"mutation\\"".len()..];
    let start = rest.find('"')? + 1;
    let name = &rest[start..start + rest[start..].find('"')?];
    let mut kebab = String::new();
    for c in name.chars() {
        if c.is_uppercase() {
            kebab.push('-');
            kebab.extend(c.to_lowercase());
        } else {
            kebab.push(c);
        }
    }
    Some(kebab)
}

impl protocol::OpBinary for %(P)sMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let json = semio_framework_pack_json::to_json_string(self);
        let kind = kind_of(&json).ok_or_else(|| malformed("op kind", 1, "mutation json names no discriminator".into()))?;
        let tag = dsl::protocol_record::records(WIRE_PROTOCOL).find(|(record, _)| *record == kind).map(|(_, tag)| tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record for '{kind}'")))?;
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT];
        store::pack_rt::write_varint_u64(&mut out, tag);
        store::pack_rt::write_varint_u64(&mut out, json.len() as u64);
        out.extend_from_slice(json.as_bytes());
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|error| protocol::ProtocolError::Io(error.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported op format {format}")));
        }
        let tag = reader.read_varint_u64().map_err(|error| protocol::ProtocolError::Io(error.to_string()))?;
        let kind = dsl::protocol_record::kind(WIRE_PROTOCOL, tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record with tag {tag}")))?.to_string();
        let len = reader.read_varint_u64().map_err(|error| protocol::ProtocolError::Io(error.to_string()))? as usize;
        let body = reader.read_bytes(len).map_err(|error| protocol::ProtocolError::Io(error.to_string()))?;
        let json = String::from_utf8(body.to_vec()).map_err(|error| malformed("op body", 1, error.to_string()))?;
        if kind_of(&json).as_deref() != Some(kind.as_str()) {
            return Err(malformed("op tag", 1, format!("tag {tag} names '{kind}' but the body is another kind")));
        }
        semio_framework_pack_json::from_json_str(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Io(error.to_string()))
    }
}
}
'''


def kebab_of(v):
    return "".join(("-" + c.lower()) if c.isupper() else c for c in v[:1].lower() + v[1:])


def run(short, emoji_name):
    art = NORM + "/🗿️artifacts/" + emoji_name
    any_ = art + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
    P = short.replace("en", "En", 1)
    root = rd(any_ + "/🧬️schema/🧬️mutations/🦀️.rs")
    enum = root[root.index("pub enum %sMutation" % P):]
    enum = enum[: enum.index("\n}\n")]
    variants = re.findall(r"^    (\w+)\(", enum, re.M)
    kinds = [kebab_of(v) for v in variants]
    bdir = any_ + "/🚪️io/💾️binary/🧬️mutations"
    old = rd(bdir + "/🦀️.rs")
    had_wrappers = "pub fn encode_op" in old
    had_tests = "mod tests;" in old
    head = old[: old.index("use protocol::OpBinary;")] if had_wrappers else "//! 🧬️ OpBinary for %sMutation: varint tag from `📡️.protocol.semio` plus the canonical JSON body.\n\n//#region 📡️SemioProtocol\npub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!(\"📡️.protocol.semio\");\npub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), \"::📡️.protocol.semio\");\n//#endregion 📡️SemioProtocol\n\n" % P
    mid = ""
    if had_wrappers:
        mid = 'use protocol::OpBinary;\n\n/// 📦️ Encodes a document mutation to its binary op form.\npub fn encode_op(mutation: &%sMutation) -> Result<Vec<u8>, protocol::ProtocolError> {\n    mutation.encode_op()\n}\n\n/// 📖️ Decodes a document mutation from its binary op form.\npub fn decode_op(bytes: &[u8]) -> Result<%sMutation, protocol::ProtocolError> {\n    %sMutation::decode_op(bytes)\n}\n\n' % (P, P, P)
    if had_tests:
        mid += '//#region 🧪️Tests\n#[cfg(test)]\n#[path = "🧪️tests/🔬️unit/🦀️.rs"]\nmod tests;\n//#endregion 🧪️Tests\n\n'
    code = CODEC % dict(m="semio_s_artifact_norm_%s_mutations" % short, P=P)
    open(bdir + "/🦀️.rs", "w", encoding="utf-8").write(head + mid + code)
    p = bdir + "/📡️.protocol.semio"
    t = rd(p)
    open(p, "w", encoding="utf-8").write(t[: t.index("record ")] + "".join("record %s tag=%d\nfield payload bytes\n" % (k, i) for i, k in enumerate(kinds)))
    print(short, "kinds", len(kinds), "wrappers", had_wrappers, "tests", had_tests)


if __name__ == "__main__":
    for s, n in [("en1993", "🔩️en1993"), ("en1998", "🫨️en1998")]:
        run(s, n)
