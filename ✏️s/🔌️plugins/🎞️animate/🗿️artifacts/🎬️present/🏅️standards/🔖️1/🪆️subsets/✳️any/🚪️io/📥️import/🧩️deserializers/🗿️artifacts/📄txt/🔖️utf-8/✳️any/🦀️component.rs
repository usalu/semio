//! 🚪️ present <- txt — foreign `Deserializer<PresentSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy` since it never
//! succeeds.

use crate::artifacts::present::PresentSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct TxtIntoPresent;

impl Deserializer<PresentSnapshot> for TxtIntoPresent {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<PresentSnapshot> {
        Err(IoError { message: "txt import not yet implemented".into(), diagnostics: Vec::new() })
    }
}
