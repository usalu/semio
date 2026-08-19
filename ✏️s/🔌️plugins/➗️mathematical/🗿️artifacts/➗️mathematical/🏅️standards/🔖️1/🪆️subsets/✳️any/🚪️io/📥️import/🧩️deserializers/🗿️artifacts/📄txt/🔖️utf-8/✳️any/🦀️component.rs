//! 🚪️ mathematical <- txt — foreign `Deserializer<MathematicalSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). Honest stub — the pre-migration
//! content here already produced this artifact's own real snapshot type with an always-`Err` body
//! (never a real implementation); this impl keeps that exact honest behavior, wired as a real (if
//! always-failing) `IoEntry` row rather than a dead composer-table entry. See the sibling
//! `Serializer`'s doc comment.

use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct TxtIntoMathematical;

impl Deserializer<MathematicalSnapshot> for TxtIntoMathematical {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    fn deserialize(_payload: &IoPayload) -> IoResult<MathematicalSnapshot> {
        Err(IoError { message: "txt import not yet implemented".to_string(), diagnostics: Vec::new() })
    }
}
