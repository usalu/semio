//! 🚪️ wires -> png — foreign `Serializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3).
//!
//! 🐛️ Fixes a pre-migration bug: the old `serialize` encoded the `WiresSnapshot` to its OWN pack
//! bytes and then tried to decode those bytes as a `PngSnapshot` pack — a confused type-pun that
//! would always fail. No real wires-graph<->raster-image mapping exists (a real implementation
//! would rasterize the board, a genuine feature, not this migration's scope) — this is now an
//! honest not-yet-implemented stub, the same treatment `🔤️txt` already had. `IoFidelity::Lossy`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };

pub struct WiresIntoPng;

impl Serializer<WiresSnapshot> for WiresIntoPng {
    const INTO: Dialect = PNG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &WiresSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "png export not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
