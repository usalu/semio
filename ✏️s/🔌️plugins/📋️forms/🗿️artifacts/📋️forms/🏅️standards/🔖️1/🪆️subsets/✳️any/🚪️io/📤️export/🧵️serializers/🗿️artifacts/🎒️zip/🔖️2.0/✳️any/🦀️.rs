//! 🚪️ forms -> zip — foreign `Serializer<FormsSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest not-yet-implemented
//! stub, `IoFidelity::Lossy` — see the twin import leaf's doc for why a real bridge is deferred.

use crate::artifacts::forms::FormsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct FormsIntoZip;

impl Serializer<FormsSnapshot> for FormsIntoZip {
    const INTO: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn serialize(_from: &FormsSnapshot) -> IoResult<IoPayload> {
        Err(IoError { message: "FormsIntoZip: not implemented".to_string(), diagnostics: Vec::new() })
    }
}
