//! 🚪️ equation -> txt — foreign `Serializer<EquationSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest stub pending a real txt
//! export implementation — see the sibling `Deserializer`'s doc comment.

use crate::artifacts::equation::EquationSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct EquationIntoTxt;

impl Serializer<EquationSnapshot> for EquationIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &EquationSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "txt export not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
