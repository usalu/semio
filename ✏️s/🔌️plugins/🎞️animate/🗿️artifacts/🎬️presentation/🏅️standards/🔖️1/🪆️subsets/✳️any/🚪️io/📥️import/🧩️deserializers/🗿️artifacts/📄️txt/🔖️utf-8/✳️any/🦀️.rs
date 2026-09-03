//! 🚪️ presentation <- txt — foreign `Deserializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy` since it never
//! succeeds.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct TxtIntoPresentation;

impl Deserializer<PresentationSnapshot> for TxtIntoPresentation {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<PresentationSnapshot> {
        Err(IoError { message: "txt import not yet implemented".into(), diagnostics: Vec::new() })
    }
}
