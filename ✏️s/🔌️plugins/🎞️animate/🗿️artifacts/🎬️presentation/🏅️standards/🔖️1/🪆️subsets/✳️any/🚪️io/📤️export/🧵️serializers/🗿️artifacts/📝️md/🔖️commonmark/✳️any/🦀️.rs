//! 🚪️ presentation -> md — foreign `Serializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Degenerate placeholder
//! (unchanged behaviour, pre-dates this ticket): the whole `.presentation` DSL text as one
//! `Paragraph`/`Text` block, not a real presentation->markdown semantic mapping (out of scope here).
//! Lossless wrap of the native text, so this hop is `IoFidelity::Canonical`, not `Lossy` —
//! mirrors `🎬️sequence`'s identical md placeholder precedent.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::schema::snapshot::{MdBlock, MdInline};
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct PresentationIntoMd;

impl Serializer<PresentationSnapshot> for PresentationIntoMd {
    const INTO: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn serialize(from: &PresentationSnapshot) -> IoResult<IoPayload> {
        let text = <PresentationSnapshot as store::ArtifactDsl>::print_dsl(from);
        let md = MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: vec![MdBlock::Paragraph { inlines: vec![MdInline::Text { text }] }] };
        Ok(IoOutcome::clean(IoPayload::Binary(<MdSnapshot as store::ArtifactPack>::encode_pack(&md))))
    }
}
