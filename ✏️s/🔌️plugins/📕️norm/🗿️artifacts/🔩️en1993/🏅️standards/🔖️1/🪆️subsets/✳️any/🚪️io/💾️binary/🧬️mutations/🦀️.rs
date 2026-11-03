//! 🧬️ OpBinary for En1993Mutation: varint tag from `📡️.protocol.semio` plus the canonical JSON body.

//#region 📡️SemioProtocol
pub const COMPONENT_PROTOCOL_SEMIO: &str = semio_s_artifact_norm_en1993_mutations_protocol::LEAF;
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

mod native_codec {
use crate::En1993Mutation;

const WIRE_PROTOCOL: &str = semio_s_artifact_norm_en1993_mutations_protocol::LEAF;

fn malformed(what: &'static str, offset: usize, detail: String) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what, offset, detail }
}

fn kind_of(json: &str) -> Option<String> {
    let rest = json.trim_start().strip_prefix('{')?.trim_start().strip_prefix('"')?;
    let name = &rest[..rest.find('"')?];
    let mut kebab = String::new();
    for c in name.chars() {
        if c.is_uppercase() {
            if !kebab.is_empty() {
                kebab.push('-');
            }
            kebab.extend(c.to_lowercase());
        } else {
            kebab.push(c);
        }
    }
    Some(kebab)
}

impl protocol::OpBinary for En1993Mutation {
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
