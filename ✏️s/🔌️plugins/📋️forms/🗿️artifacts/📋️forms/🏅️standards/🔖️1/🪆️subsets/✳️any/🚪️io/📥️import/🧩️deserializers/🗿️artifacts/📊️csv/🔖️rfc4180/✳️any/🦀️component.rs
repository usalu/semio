//! 🚪️ forms <- csv — foreign `Deserializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub, `IoFidelity::Lossy`, matching `🎬️sequence`'s `TxtIntoSequence` precedent
//! (`📓️w4-sequence-report.md`): a flattened results grid cannot reconstruct `steps` — this
//! plugin's own composition doc (`🗿️artifacts/📋️forms/🦀️component.rs` `🔖️Composition`) is explicit
//! that `results` is a DERIVED, non-reconstructive projection of `structure`, never the other way
//! round. The pre-existing code here (a naive `serde_json` struct-shape bridge between
//! `FormsSnapshot` and `CsvSnapshot`) compiled but could not have round-tripped real content either
//! — this stub is honest about the same limitation instead of silently miscompiling on real input.

use crate::artifacts::forms::FormsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const CSV_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.csv", standard: StandardId("rfc4180"), subset: SubsetId::ANY };

pub struct CsvIntoForms;

impl Deserializer<FormsSnapshot> for CsvIntoForms {
    const FROM: Dialect = CSV_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<FormsSnapshot> {
        Err(IoError { message: "CsvIntoForms: forms cannot be reconstructed from a flattened results table — not implemented".to_string(), diagnostics: Vec::new() })
    }
}
