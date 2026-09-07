//! 🚪️ fem3d → json — foreign `Serializer<Fem3dSnapshot>` on the framework's `io_mechanism` channel.
//! `Fem3dSnapshot` is a pure `dsl::ToValue` record tree (nodes/elements/materials/sections/solids/
//! supports/load-cases/combinations/analysis, no external child references), so its rfc8259
//! rendition carries every field and the sibling `📥️import` leaf reconstructs the snapshot exactly:
//! `IoFidelity::Exact`. The text is written by stdio's own real RFC 8259 codec (`write_json_text`),
//! never a re-derived encoder; `dsl::json::from_dsl_value` keeps integers integral across the bridge
//! instead of widening them to `f64`.

use crate::artifacts::fem3d::Fem3dSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::json::schema::snapshot::write_json_text;
use semio_s_plugin_stdio::artifacts::json::JsonSnapshot;

/// 🎯️ The foreign dialect this leaf writes.
pub const JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId::ANY };

/// 🔣️ This subset's snapshot as compact rfc8259 text.
pub fn json_text(from: &Fem3dSnapshot) -> String {
    write_json_text(&JsonSnapshot::from_value(dsl::json::from_dsl_value(&dsl::ToValue::to_value(from))).value)
}

/// 🧵️ `s.fem.fem3d@1/*` → `s.stdio.json@rfc8259/*`.
pub struct Fem3dIntoJson;

impl Serializer<Fem3dSnapshot> for Fem3dIntoJson {
    const INTO: Dialect = JSON_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem3dSnapshot) -> IoResult<IoPayload> {
        Ok(IoOutcome::clean(IoPayload::Text(json_text(from))))
    }
}
