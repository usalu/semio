//! 🚪️ wires <- svg — foreign `Deserializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! 🐛️ Fixes a pre-migration bug: the old `deserialize_bytes` re-encoded the incoming `SvgSnapshot`
//! to bytes and then tried to decode THOSE bytes as a `WiresSnapshot` pack (falling back to
//! `WiresSnapshot::parse_dsl` on failure) — a confused type-pun that would either always fail (the
//! two packs' binary shapes are unrelated) or, worse, silently accept a payload that merely
//! happened to already be `.wires`-shaped text as if it were a real SVG import. No real
//! svg-graph<->wires-graph mapping exists — this is now an honest not-yet-implemented stub, the
//! same treatment `📄️txt` already had. `IoFidelity::Lossy`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct SvgIntoWires;

impl Deserializer<WiresSnapshot> for SvgIntoWires {
    const FROM: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<WiresSnapshot> {
        Err(IoError { message: "svg import not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
