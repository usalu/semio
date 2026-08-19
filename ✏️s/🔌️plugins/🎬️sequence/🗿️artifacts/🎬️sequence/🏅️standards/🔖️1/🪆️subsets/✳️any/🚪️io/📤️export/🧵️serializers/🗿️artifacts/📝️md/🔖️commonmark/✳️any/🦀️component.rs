//! 🚪️ sequence -> md — foreign `Serializer<SequenceSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Writes sequence's own
//! `.sequence` DSL text as MD's text body — a lossless wrap, `IoFidelity::Canonical`.

use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct SequenceIntoMd;

impl Serializer<SequenceSnapshot> for SequenceIntoMd {
    const INTO: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    fn serialize(from: &SequenceSnapshot) -> IoResult<IoPayload> {
        let md = MdSnapshot::from_text(&<SequenceSnapshot as store::ArtifactDsl>::print_dsl(from));
        let _ = STDIO_MD_DOCUMENT_SCHEMA;
        Ok(IoOutcome::clean(IoPayload::Binary(<MdSnapshot as store::ArtifactPack>::encode_pack(&md))))
    }
}
