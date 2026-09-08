//! 🚪️ fem2d → txt — foreign `Serializer<Fem2dSnapshot>` on the framework's `io_mechanism` channel.
//! `s.stdio.txt@utf-8` for this subset IS its own `.semio` DSL snapshot text
//! (`🧬️schema/📸️snapshot/📝️text`): the very bytes `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` carries
//! and `<Fem2dSnapshot as store::ArtifactDsl>::parse_dsl` reads back, so the hop is
//! `IoFidelity::Exact` and the sibling `📥️import` leaf is its exact inverse.
//!
//! 🐛️ Repaired here (ticket 26/09/06/FEM-PLUGIN-END-TO-END, W4): this file used to be an
//! `Err("txt export not yet implemented")` stub that ALSO carried a stray `deserialize_bytes` — an
//! import-direction function inside the export tree, left behind by a copy-paste of stdio's own
//! json↔txt bridge. Both are gone.

use crate::Fem2dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

/// 🎯️ The foreign dialect this leaf writes.
pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

/// 🔤️ This subset's snapshot as `.semio` DSL text — also the authoritative payload the `📊️csv` and
/// `📝️md` envelope leaves carry.
pub fn dsl_text(from: &Fem2dSnapshot) -> String {
    <Fem2dSnapshot as store::ArtifactDsl>::print_dsl(from)
}

/// 🧵️ `s.fem.fem2d@1/*` → `s.stdio.txt@utf-8/*`.
pub struct Fem2dIntoTxt;

impl Serializer<Fem2dSnapshot> for Fem2dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem2dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(dsl_text(from))))
    }
}
