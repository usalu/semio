//! 🚪️ forms -> xlsx — foreign `Serializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub, `IoFidelity::Lossy` — see the twin import leaf's doc for why a real bridge is deferred.

use crate::artifacts::forms::FormsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const XLSX_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.xlsx", standard: StandardId("ecma-376"), subset: SubsetId::ANY };

pub struct FormsIntoXlsx;

impl Serializer<FormsSnapshot> for FormsIntoXlsx {
    const INTO: Dialect = XLSX_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &FormsSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "FormsIntoXlsx: not implemented".to_string(), diagnostics: Vec::new() })
    }
}
