//! 🚪️ wires <- md — foreign `Deserializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Wraps the full `.wires` DSL text
//! losslessly inside `md`'s own document text — every field survives round trip, so
//! `IoFidelity::Canonical` (not `Exact`: the wire bytes are `md`'s, not `wires`'s own).

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct MdIntoWires;

impl Deserializer<WiresSnapshot> for MdIntoWires {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload) -> IoResult<WiresSnapshot> {
        let _ = STDIO_MD_DOCUMENT_SCHEMA;
        let md = match payload {
            IoPayload::Binary(bytes) => <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("MdIntoWires: md decode failed: {error}"), diagnostics: Vec::new() })?,
            IoPayload::Text(text) => MdSnapshot::from_text(text),
        };
        let snapshot = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&md.to_text()).map_err(|error| IoError { message: format!("MdIntoWires: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
