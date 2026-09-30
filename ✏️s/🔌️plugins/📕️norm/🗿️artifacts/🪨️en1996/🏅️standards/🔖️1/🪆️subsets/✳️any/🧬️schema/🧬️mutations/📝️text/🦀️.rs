//! ⚡️ En1996 mutations — OpText via JSON tokens, OpBinary via the protocol-tagged payload wire value.

pub use crate::artifact_schema::mutations::En1996Mutation;

use protocol::{OpBinary, OpText};

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

impl OpText for En1996Mutation {
    fn print_op(&self) -> String {
        pack::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        pack::json::from_json_str(line).map_err(|e| store::TextError::new(e.to_string(), store::TextSpan::at(1, 1)))
    }
}

/// 📡️ The wire protocol whose `record <kind> tag=<n>` lines are the only source of op tags.
const WIRE_PROTOCOL: &str = include_str!("../💾️binary/📡️.protocol.semio");

fn malformed(what: &'static str, offset: u64, detail: String) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what, offset, detail }
}

/// 💾️ `format u8 | tag varint | payload wire value`: the tag is the protocol record of the leaf descriptor's semantic
/// kind and the payload is the leaf's own `payload_value()`, so decoding is `from_payload_value(kind, payload)`.
impl OpBinary for En1996Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let kind = <Self as protocol::Mutation<crate::En1996Snapshot>>::descriptor(self).semantic_kind;
        let tag = dsl::protocol_record::records(WIRE_PROTOCOL).find(|(record, _)| *record == kind).map(|(_, tag)| tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record for '{kind}'")))?;
        let mut bytes = vec![store::pack_rt::OP_BINARY_FORMAT];
        store::pack_rt::write_varint_u64(&mut bytes, tag);
        bytes.extend(store::pack_rt::encode_wire_value(&<Self as protocol::Mutation<crate::En1996Snapshot>>::payload_value(self)));
        Ok(bytes)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("unsupported op format {format}")));
        }
        let tag = reader.read_varint_u64()?;
        let kind = dsl::protocol_record::kind(WIRE_PROTOCOL, tag).ok_or_else(|| malformed("op tag", 1, format!("📡️.protocol.semio declares no record with tag {tag}")))?;
        let offset = reader.position();
        let payload = store::pack_rt::decode_wire_value(&bytes[offset..]).map_err(|error| malformed("op payload", offset as u64, error.to_string()))?;
        let decoded = <Self as protocol::Mutation<crate::En1996Snapshot>>::from_payload_value(kind, payload).map_err(|error| malformed("op value", offset as u64, error.to_string()))?;
        if decoded.encode_op()? != bytes {
            return Err(malformed("op encoding", 0, "operation bytes are not canonical".into()));
        }
        Ok(decoded)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
