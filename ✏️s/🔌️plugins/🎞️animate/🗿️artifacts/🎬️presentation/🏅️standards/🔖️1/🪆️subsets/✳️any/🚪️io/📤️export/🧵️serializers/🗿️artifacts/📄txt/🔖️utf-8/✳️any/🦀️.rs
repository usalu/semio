//! 🚪️ presentation -> txt — foreign `Serializer<PresentationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub (unchanged behaviour, pre-dates this ticket) — `IoFidelity::Lossy` since it never
//! succeeds.

use crate::artifacts::presentation::PresentationSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct PresentationIntoTxt;

impl Serializer<PresentationSnapshot> for PresentationIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(_from: &PresentationSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "txt export not yet implemented".into(), diagnostics: Vec::new() })
    }
}
