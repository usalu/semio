//! 🚪️ mathematical -> json — foreign `Serializer<MathematicalSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Direct first-party
//! `ToValue`/`pack::json` serialization of every field (no `serde_json` — `MathematicalSnapshot`
//! only needs to be written, never JsonSnapshot-bridged, on this hop; see the sibling `import/json`
//! leaf's own docstring for why the read direction still needs `serde_json`), so this hop is
//! `IoFidelity::Exact`. No pretty-printer exists in `pack::json` yet (no consumer needed one) — the
//! compact form is still exact, lossless JSON, and nothing round-trips through this hop byte-for-byte.

use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::STDIO_JSON_DOCUMENT_SCHEMA;

pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

pub struct MathematicalIntoJson;

impl Serializer<MathematicalSnapshot> for MathematicalIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    fn serialize(from: &MathematicalSnapshot) -> IoResult<IoPayload> {
        let _ = STDIO_JSON_DOCUMENT_SCHEMA;
        let bytes = pack::json::to_json_string(from).into_bytes();
        Ok(IoOutcome::clean(IoPayload::Binary(bytes)))
    }
}
