//! 🚪️ forms <- xlsx — foreign `Deserializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub, `IoFidelity::Lossy`, matching `🎬️sequence`'s `TxtIntoSequence` precedent
//! (`📓️w4-sequence-report.md`): xlsx's real spreadsheet/workbook shape has no established mapping
//! onto forms' step/question tree here. The pre-existing code (a naive `serde_json` struct-shape
//! bridge between `FormsSnapshot` and `XlsxSnapshot`) compiled but could not have round-tripped
//! real content either — this stub is honest about the same limitation instead of silently
//! miscompiling on real input.

use crate::artifacts::forms::FormsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct XlsxIntoForms;

impl Deserializer<FormsSnapshot> for XlsxIntoForms {
    const FROM: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<FormsSnapshot> {
        Err(IoError { message: "XlsxIntoForms: not implemented".to_string(), diagnostics: Vec::new() })
    }
}
