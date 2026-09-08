//! 🧾️ Serialize layout through the first-party JSON artifact codec.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::JsonSnapshot;

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<JsonSnapshot, store::PackError> {
    <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&serialize_text(from)?).map_err(|error| store::PackError::Schema(error.to_string()))
}

pub fn serialize_text(from: &LayoutSnapshot) -> Result<String, store::PackError> {
    Ok(dsl::os_pack::json::to_json_string(from))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn json_artifact_round_trip_preserves_the_language_neutral_snapshot() {
        let fixture = include_str!("../../../../../../../🧬️schema/🧬️mutations/🔀reorder-pages/🧪️tests/🔀️moves-page-1-behind-page-2/📸️snapshot/⬅️before/🔣️.json");
        let snapshot = crate::artifacts::layout::schema::parse_layout_document(fixture).unwrap();
        let actual = serialize_text(&snapshot).unwrap();
        let oracle: serde_json::Value = serde_json::from_str(fixture).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&actual).unwrap();
        for (key, value) in oracle.as_object().unwrap() { assert_eq!(&parsed[key], value, "field {key}"); }
        let artifact = serialize(&snapshot).unwrap();
        let artifact_json = semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_text(&artifact.value);
        assert_eq!(serde_json::from_str::<serde_json::Value>(&artifact_json).unwrap(), parsed);
        assert_eq!(crate::artifacts::layout::schema::parse_layout_document(&artifact_json).unwrap(), snapshot);
    }
}
