//! 🚪️ equation -> md. The complete carrier fixture is embedded in a canonical JSON code block.

use crate::artifacts::equation::{equation_fixture, EquationSnapshot};
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::md::{MdBlock, MdSnapshot, STDIO_MD_DOCUMENT_SCHEMA};

pub const MD_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.md", standard: StandardId("commonmark"), subset: SubsetId::ANY };

pub struct EquationIntoMd;

impl Serializer<EquationSnapshot> for EquationIntoMd {
    const INTO: Dialect = MD_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Canonical;
    async fn serialize(from: &EquationSnapshot) -> IoResult<IoPayload> {
        let fixture = equation_fixture(from).map_err(|error| IoError { message: format!("EquationIntoMd: {error}"), diagnostics: Vec::new() })?;
        let md = MdSnapshot { schema: STDIO_MD_DOCUMENT_SCHEMA.into(), blocks: vec![MdBlock::CodeBlock { info: Some("json".into()), literal: pack::json::to_json_string(&fixture) }] };
        let _ = STDIO_MD_DOCUMENT_SCHEMA;
        Ok(IoOutcome::clean(IoPayload::Binary(<MdSnapshot as store::ArtifactPack>::encode_pack(&md))))
    }
}
