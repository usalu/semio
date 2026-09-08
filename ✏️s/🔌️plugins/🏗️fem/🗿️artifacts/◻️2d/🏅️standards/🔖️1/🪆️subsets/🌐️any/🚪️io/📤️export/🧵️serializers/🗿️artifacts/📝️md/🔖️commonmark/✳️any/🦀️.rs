//! 🚪️ fem2d → md — foreign `Serializer<Fem2dSnapshot>` on the framework's `io_mechanism` channel.
//! CommonMark has no schema of its own to project eight heterogeneous fem tables onto, so this is an
//! ENVELOPE like the `📊️csv` sibling: one fenced code block whose `literal` is this subset's own
//! `.semio` DSL text, tagged with the `fem2d` info string. A code block's literal is carried
//! verbatim by stdio's own real renderer/parser pair, so nothing about the snapshot is lost and the
//! sibling `📥️import` leaf reconstructs it exactly: `IoFidelity::Exact`.
//!
//! 🐛️ Repaired here (ticket 26/09/06/FEM-PLUGIN-END-TO-END, W4): the previous leaf built the same
//! `MdSnapshot` but its `serialize_bytes` threw it away and wrote BARE `print_dsl` bytes under the
//! `s.stdio.md` name — DSL text mislabelled as CommonMark, which no markdown reader could open.
//! The info string is new too: the old envelope wrote `info: None`, so nothing on the wire said what
//! the fenced block contained.

use crate::Fem2dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_md::schema::snapshot::MdBlock;
use semio_s_artifact_stdio_md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

/// 🎯️ The foreign dialect this leaf writes.
pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

/// 🏷️ The fenced block's info string — what the sibling importer looks for first.
pub const FENCE_INFO: &str = "fem2d";

/// 📝️ The envelope as an `MdSnapshot`: exactly one fenced code block carrying the DSL text.
pub fn md_snapshot(from: &Fem2dSnapshot) -> MdSnapshot {
    MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: vec![MdBlock::CodeBlock { info: Some(FENCE_INFO.to_string()), literal: <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(from) }] }
}

/// 📝️ The envelope as real CommonMark text.
pub fn md_text(from: &Fem2dSnapshot) -> String {
    md_snapshot(from).to_text()
}

/// 🧵️ `s.fem.fem2d@1/*` → `s.stdio.md@commonmark/*`.
pub struct Fem2dIntoMd;

impl Serializer<Fem2dSnapshot> for Fem2dIntoMd {
    const INTO: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem2dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(md_text(from))))
    }
}
