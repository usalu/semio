//! playground <- json
use crate::artifacts::playground::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot;
use crate::artifacts::playground::PLAYGROUND_DOCUMENT_SCHEMA;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::parse_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub async fn deserialize(from: &JsonSnapshot) -> Result<PlaygroundSnapshot, store::TextError> {
    let mut out: PlaygroundSnapshot = serde_json::from_value(from.to_serde_value()).map_err(|e| store::TextError::new(format!("playground<-json: {e}"), dsl::TextSpan::at(1, 1)))?;
    if out.schema.is_empty() {
        out.schema = PLAYGROUND_DOCUMENT_SCHEMA.into();
    }
    Ok(out)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<PlaygroundSnapshot, store::TextError> {
    let text = std::str::from_utf8(bytes).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    let value = parse_json_text(text)?;
    deserialize(&JsonSnapshot::from_value(value))
}
