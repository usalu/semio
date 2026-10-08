//! 🚪️ fem3d → json — foreign `Serializer<Fem3dSnapshot>` on the framework's `io_mechanism` channel.
//! `Fem3dSnapshot` is a pure `semio_framework_value::ToValue` record tree (nodes/elements/materials/sections/solids/
//! supports/load-cases/combinations/analysis, no external child references), so its rfc8259
//! rendition carries every field and the sibling `📥️import` leaf reconstructs the snapshot exactly:
//! `IoFidelity::Exact`. The text is written by stdio's own real RFC 8259 codec (`write_json_text`),
//! never a re-derived encoder; `semio_framework_pack_json::from_dsl_value` keeps integers integral across the bridge
//! instead of widening them to `f64`.

use crate::Fem3dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::io::text::snapshot::write_json_text;
use semio_s_artifact_stdio_json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ This subset's snapshot as compact rfc8259 text.
pub fn json_text(from: &Fem3dSnapshot) -> String {
    write_json_text(&JsonSnapshot::from_value(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(from))).value)
}

/// 🧵️ `s.fem.fem3d@1/*` → `s.stdio.json@rfc8259/*`.
pub struct Fem3dIntoJson;

impl Serializer<Fem3dSnapshot> for Fem3dIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem3dSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(json_text(from))))
    }
}
