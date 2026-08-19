//! 🚪️ wires <- png — foreign `Deserializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! 🐛️ Fixes a pre-migration bug: the old `deserialize_bytes` re-encoded the incoming `PngSnapshot`
//! to bytes and then tried to decode THOSE bytes as a `WiresSnapshot` pack (falling back to
//! `WiresSnapshot::parse_dsl` on failure) — a confused type-pun, same class as the sibling `🎨️svg`
//! leaf's bug. No real raster-image<->wires-graph mapping exists — this is now an honest
//! not-yet-implemented stub, the same treatment `📄txt` already had. `IoFidelity::Lossy`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct PngIntoWires;

impl Deserializer<WiresSnapshot> for PngIntoWires {
    const FROM: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<WiresSnapshot> {
        Err(IoError { message: "png import not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
