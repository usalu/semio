//! 🚪️ present -> json — foreign `Serializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct `serde_json`
//! serialization of every field via stdio's own `JsonSnapshot::from_value`/`write_json_pretty`
//! text codec, so this hop is `IoFidelity::Exact`.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_pretty;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct PresentIntoJson;

impl Serializer<PresentSnapshot> for PresentIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &PresentSnapshot) -> IoResult<IoPayload> {
        let value = serde_json::to_value(from).map_err(|error| IoError { message: format!("PresentIntoJson: {error}"), diagnostics: Vec::new() })?;
        let json = JsonSnapshot::from_value(value);
        Ok(IoOutcome::clean(IoPayload::Binary(write_json_pretty(&json.value).into_bytes())).await)
    }
}
