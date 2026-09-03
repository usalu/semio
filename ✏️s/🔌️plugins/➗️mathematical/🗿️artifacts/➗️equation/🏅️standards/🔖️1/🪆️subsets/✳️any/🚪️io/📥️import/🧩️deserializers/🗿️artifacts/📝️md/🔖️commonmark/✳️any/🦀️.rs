//! 🚪️ equation <- md. Reads the canonical JSON carrier fixture from its code block.

use crate::artifacts::equation::{equation_snapshot_from_fixture, EquationFixture, EquationSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdBlock, MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct MdIntoEquation;

impl Deserializer<EquationSnapshot> for MdIntoEquation {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload) -> IoResult<EquationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError { message: "MdIntoEquation: expected a binary md payload".to_string(), diagnostics: Vec::new() });
        };
        let _ = STDIO_MD_DOCUMENT_SCHEMA;
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError { message: format!("MdIntoEquation: md decode failed: {error}"), diagnostics: Vec::new() })?;
        let literal = match md.blocks.as_slice() {
            [MdBlock::CodeBlock { info: Some(info), literal }] if info == "json" => literal,
            _ => return Err(IoError { message: "MdIntoEquation: expected one json code block".into(), diagnostics: Vec::new() }),
        };
        let fixture: EquationFixture = pack::json::from_json_str(literal).map_err(|error| IoError { message: format!("MdIntoEquation: {error}"), diagnostics: Vec::new() })?;
        Ok(IoOutcome::clean(equation_snapshot_from_fixture(fixture)))
    }
}
