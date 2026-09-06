//! 🚪️ wires <- txt — foreign `Deserializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub, unchanged behavior from the pre-migration free-function version — a real txt<->wires
//! design is out of scope for this migration pass. `IoFidelity::Lossy`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct TxtIntoWires;

impl Deserializer<WiresSnapshot> for TxtIntoWires {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<WiresSnapshot> {
        Err(IoError { message: "txt import not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
