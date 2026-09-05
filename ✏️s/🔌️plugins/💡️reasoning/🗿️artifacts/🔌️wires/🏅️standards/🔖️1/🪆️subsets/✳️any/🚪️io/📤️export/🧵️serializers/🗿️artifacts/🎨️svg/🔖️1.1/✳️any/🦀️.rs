//! 🚪️ wires -> svg — foreign `Serializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! 🐛️ Fixes a pre-migration bug: the old `serialize` encoded the `WiresSnapshot` to its OWN pack
//! bytes and then tried to decode those bytes as an `SvgSnapshot` pack — a confused type-pun that
//! would always fail (the two packs' binary shapes are unrelated). No real wires-graph<->svg
//! mapping exists — this is now an honest not-yet-implemented stub, the same treatment `🔤️txt`
//! already had. `IoFidelity::Lossy`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct WiresIntoSvg;

impl Serializer<WiresSnapshot> for WiresIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &WiresSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "svg export not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
