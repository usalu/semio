//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.json@rfc8259/*`. Full struct fidelity
//! via `serde_json`, then wire-encoded through `JsonSnapshot`'s own `ArtifactDsl` (json is the
//! universal bridge dialect every domain artifact in this repo exports to).

use crate::artifacts::writer::WriterSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoJson;
impl Serializer<WriterSnapshot> for WriterIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    /// 🪧️ Exact — see the sibling deserializer's doc comment.
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn serialize(from: &WriterSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("WriterIntoJson: {error}"), diagnostics: Vec::new() })?;
        let json = JsonSnapshot::from_value(value);
        Ok(IoOutcome { value: IoPayload::Text(store::ArtifactDsl::print_dsl(&json)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_into_json_round_trips_through_json_into_writer() {
        let original = crate::artifacts::writer::writer_snapshot_with_text("writer.document", "id", "plain", "writer://id", "hello");
        let outcome = WriterIntoJson::serialize(&original).expect("serialize");
        let back = crate::artifacts::writer::io::import::deserializers::artifacts::json::v_rfc8259::any::JsonIntoWriter::deserialize(&outcome.value).expect("deserialize");
        assert_eq!(back.value, original);
    }
}
