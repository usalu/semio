//! 🚪️ wires -> json — foreign `Serializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Symmetric with the sibling
//! `Deserializer`: emits `WiresSnapshot`'s own canonical JSON shape verbatim, so `IoFidelity::Exact`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct WiresIntoJson;

impl Serializer<WiresSnapshot> for WiresIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn serialize(from: &WiresSnapshot) -> IoResult<IoPayload> {
        let value = dsl::os_pack::json::from_dsl_value(&dsl::ToValue::to_value(from));
        let text = dsl::os_pack::json::to_string_pretty(&value);
        Ok(IoOutcome::clean(IoPayload::Text(text)))
    }
}
