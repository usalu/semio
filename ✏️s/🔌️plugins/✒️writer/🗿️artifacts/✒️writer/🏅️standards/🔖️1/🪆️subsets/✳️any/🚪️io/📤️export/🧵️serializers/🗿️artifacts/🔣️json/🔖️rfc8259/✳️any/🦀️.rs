//! 📤️ Foreign leaf — serialize `WriterSnapshot` INTO `s.stdio.json@rfc8259/*`. Full struct fidelity
//! via `serde_json`, then wire-encoded through `JsonSnapshot`'s own `ArtifactDsl` (json is the
//! universal bridge dialect every domain artifact in this repo exports to).

use crate::WriterSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

//#region 🔖️Serializer
pub struct WriterIntoJson;
impl Serializer<WriterSnapshot> for WriterIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    /// 🪧️ Exact — see the sibling deserializer's doc comment.
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &WriterSnapshot) -> IoResult<IoPayload> {
        let json = <JsonSnapshot as store::ArtifactDsl>::parse_dsl(&dsl::os_pack::json::to_json_string(from)).map_err(|error| IoError { message: format!("WriterIntoJson: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome { value: IoPayload::Text(store::ArtifactDsl::print_dsl(&json)), diagnostics: Vec::new() })
    }
}
//#endregion 🔖️Serializer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
