//! 🚪️ sequence <- md. Reads the canonical JSON carrier fixture from its code block.

use crate::{SequenceHostSnapshot, SequenceSnapshot};
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_md::schema::snapshot::MdBlock;
use semio_s_artifact_stdio_md::MdSnapshot;

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct MdIntoSequence;

impl Deserializer<SequenceSnapshot> for MdIntoSequence {
    const FROM: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<SequenceSnapshot> {
        let IoPayload::Binary(bytes) = payload else {
            return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "MdIntoSequence: expected a binary md payload".to_string())));
        };
        let md = <MdSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| IoError::from_value_error(match error.into_value_error() { Ok(error) => error.under("MdIntoSequence"), Err(error) => ValueError::new(ValueRefusalKind::InvariantViolated, format!("MdIntoSequence: in-memory decode reported a transport failure: {error}")) }))?;
        let literal = match md.blocks.as_slice() {
            [MdBlock::CodeBlock { info: Some(info), literal }] if info == "json" => literal,
            _ => return Err(IoError::from_value_error(ValueError::new(ValueRefusalKind::InvalidValue, "MdIntoSequence: expected one json code block"))),
        };
        let fixture: SequenceHostSnapshot = semio_framework_pack_json::from_json_str(literal, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| IoError::from_value_error(error.under("MdIntoSequence")))?;
        Ok(IoOutcome::clean(SequenceSnapshot::from_host_snapshot(fixture)))
    }
}
