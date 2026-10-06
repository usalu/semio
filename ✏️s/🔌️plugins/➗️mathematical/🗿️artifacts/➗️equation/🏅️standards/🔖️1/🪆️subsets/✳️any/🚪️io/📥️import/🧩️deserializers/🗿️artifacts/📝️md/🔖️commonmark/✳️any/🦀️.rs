//! 🚪️ equation <- md. Reads the canonical JSON carrier fixture from its code block.

use crate::{equation_snapshot_from_host_snapshot, EquationCarrierSnapshot, EquationSnapshot};
use semio_framework::io::io_mechanism::Deserializer;
use crate::standards::v1::subsets::any::io::{invalid_payload, pack_decode_refusal};
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_md::standards::v_commonmark::subsets::any::schema::snapshot::MdBlock;
use semio_s_artifact_stdio_md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct MdIntoEquation;

impl Deserializer<EquationSnapshot> for MdIntoEquation {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload) -> IoResult<EquationSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(invalid_payload("MdIntoEquation", "expected a binary md payload"));
        };
        let _ = STDIO_MD_DOCUMENT_SCHEMA;
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| pack_decode_refusal("MdIntoEquation", error))?;
        let literal = match md.blocks.as_slice() {
            [MdBlock::CodeBlock { info: Some(info), literal }] if info == "json" => literal,
            _ => return Err(invalid_payload("MdIntoEquation", "expected one json code block")),
        };
        let fixture: EquationCarrierSnapshot = semio_framework_pack_json::from_json_str(literal, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoError::from_value_error(error.under("MdIntoEquation")))?;
        Ok(IoOutcome::clean(equation_snapshot_from_host_snapshot(fixture)))
    }
}
