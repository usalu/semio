//! 🚪️ sequence -> md. The complete carrier fixture is embedded in a canonical JSON code block.

use crate::SequenceSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_md::schema::snapshot::MdBlock;
use semio_s_artifact_stdio_md::{MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct SequenceIntoMd;

impl Serializer<SequenceSnapshot> for SequenceIntoMd {
    const INTO: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn serialize(from: &SequenceSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let fixture = from.try_to_host_snapshot().map_err(|error| IoError::from_value_error(ValueError::new(ValueRefusalKind::UnsupportedOwner, format!("SequenceIntoMd: {error}"))))?;
        let literal = semio_framework_pack_json::to_json_string(&fixture);
        let md = MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: vec![MdBlock::CodeBlock { info: Some("json".into()), literal }] };
        Ok(IoOutcome::clean(IoPayload::Binary(<MdSnapshot as store::ArtifactPack>::encode_pack(&md))))
    }
}
