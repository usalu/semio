//! 🚪️ mathematical <- md — foreign `Deserializer<MathematicalSnapshot>` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3). MD's own text IS mathematical's
//! native `.mathematical` DSL text, so this hop is a lossless wrap/unwrap — `IoFidelity::Canonical`.

use crate::artifacts::mathematical::MathematicalSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct MdIntoMathematical;

impl Deserializer<MathematicalSnapshot> for MdIntoMathematical {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload) -> IoResult<MathematicalSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "MdIntoMathematical: expected a binary md payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_MD_DOCUMENT_SCHEMA;
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("MdIntoMathematical: md decode failed: {error}"), diagnostics: Vec::new() })?;
        let snapshot = <MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(&md.to_text()).map_err(|error| IoError { message: format!("MdIntoMathematical: dsl parse failed: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(snapshot))
    }
}
