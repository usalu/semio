//! 🚪️ forms <- zip — foreign `Deserializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub, `IoFidelity::Lossy`, matching `🎬️sequence`'s `TxtIntoSequence` precedent
//! (`📓️w4-sequence-report.md`): a zip archive has no established mapping onto forms' step/question
//! tree here. The pre-existing code (a naive `serde_json` struct-shape bridge between
//! `FormsSnapshot` and `ZipSnapshot`) compiled but could not have round-tripped real content
//! either — this stub is honest about the same limitation instead of silently miscompiling on
//! real input.

use crate::artifacts::forms::FormsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct ZipIntoForms;

impl Deserializer<FormsSnapshot> for ZipIntoForms {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn deserialize(_payload: &IoPayload) -> IoResult<FormsSnapshot> {
        Err(IoError { message: "ZipIntoForms: not implemented".to_string(), diagnostics: Vec::new() })
    }
}
