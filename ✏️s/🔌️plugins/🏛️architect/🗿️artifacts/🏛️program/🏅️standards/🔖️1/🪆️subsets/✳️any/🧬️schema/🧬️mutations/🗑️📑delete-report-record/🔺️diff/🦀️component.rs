//! 🔺️ Sparse diff construction for the `delete-report-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `📑reports` per Wave C.

use super::mutation::DeleteReportRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramReportsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteReportRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { reports: Some(ProgramReportsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
