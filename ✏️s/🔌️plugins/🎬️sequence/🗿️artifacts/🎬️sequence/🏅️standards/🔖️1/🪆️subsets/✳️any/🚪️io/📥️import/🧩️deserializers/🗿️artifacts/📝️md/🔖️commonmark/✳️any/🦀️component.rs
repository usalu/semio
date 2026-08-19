//! 🚪️ sequence <- md — foreign `Deserializer<SequenceSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). MD's own text IS sequence's
//! native `.sequence` DSL text, so this hop is a lossless wrap/unwrap — `IoFidelity::Canonical`.

use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct MdIntoSequence;

impl Deserializer<SequenceSnapshot> for MdIntoSequence {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload) -> IoResult<SequenceSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "MdIntoSequence: expected a binary md payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_MD_DOCUMENT_SCHEMA;
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("MdIntoSequence: md decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = <SequenceSnapshot as store::ArtifactDsl>::parse_dsl(&md.to_text()).map_err(|error| IoError { message: format!("MdIntoSequence: dsl parse failed: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
