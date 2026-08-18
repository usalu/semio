//! 🚪️ wires -> txt — foreign `Serializer<WiresSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub, unchanged behavior from the pre-migration free-function version. `IoFidelity::Lossy`.

use crate::artifacts::wires::WiresSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct WiresIntoTxt;

impl Serializer<WiresSnapshot> for WiresIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &WiresSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "txt export not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
