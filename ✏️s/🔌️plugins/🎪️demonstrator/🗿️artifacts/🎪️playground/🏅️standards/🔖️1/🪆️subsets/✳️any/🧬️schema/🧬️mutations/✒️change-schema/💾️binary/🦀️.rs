//! 💾️ Direct `change-schema` binary payload codec and aggregate wire bridge.

use super::super::PlaygroundMutation;
use super::ChangeSchema;

/// 🏷️ Stable binary tag for `ChangeSchema`.
pub const BINARY_TAG: u32 = 0;

fn write_string(output: &mut Vec<u8>, value: &str) {
    store::pack_rt::write_varint_u64(output, value.len() as u64);
    output.extend_from_slice(value.as_bytes());
}

fn read_string(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let length = reader.read_varint_u64().map_err(|error| error.to_string())? as usize;
    let bytes = reader.read_bytes(length).map_err(|error| error.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|error| error.to_string())
}

//#region 🔖️OpBinary
impl protocol::OpBinary for PlaygroundMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut output = vec![store::pack_rt::OP_BINARY_FORMAT, BINARY_TAG as u8];
        match self {
            PlaygroundMutation::ChangeSchema(payload) => write_string(&mut output, &payload.new_schema),
        }
        Ok(output)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|error| malformed("op format", 0, error.to_string()))?;
        let tag = reader.read_u8().map_err(|error| malformed("op tag", 1, error.to_string()))?;
        match tag as u32 {
            BINARY_TAG => {
                let new_schema = read_string(&mut reader).map_err(|error| malformed("new_schema", reader.position(), error))?;
                Ok(PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinary

/// 📦️ Encodes the direct payload independently of aggregate framing.
pub fn encode_payload(value: &ChangeSchema) -> Result<Vec<u8>, String> {
    Ok(dsl::os_pack::json::to_json_string(value).into_bytes())
}

/// 📖️ Decodes the direct payload independently of aggregate framing.
pub fn decode_payload(value: &[u8]) -> Result<ChangeSchema, String> {
    let text = std::str::from_utf8(value).map_err(|error| error.to_string())?;
    dsl::os_pack::json::from_json_str(text).map_err(|error| error.to_string())
}

//#region 🧪️RoundTrip
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_and_text_wire_forms_agree() {
        let operation = PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: "playground.custom".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
        assert_eq!(decode_payload(&encode_payload(&ChangeSchema { new_schema: "playground.custom".into() }).expect("encode payload")).expect("decode payload").new_schema, "playground.custom");
    }
}
//#endregion 🧪️RoundTrip
