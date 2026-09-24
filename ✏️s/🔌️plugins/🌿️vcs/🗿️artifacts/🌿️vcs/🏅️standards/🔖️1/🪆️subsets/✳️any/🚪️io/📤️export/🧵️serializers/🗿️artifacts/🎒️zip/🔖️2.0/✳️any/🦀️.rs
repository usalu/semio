//! 🌿️ vcs → zip — the shared document archive (`encode_document_archive`): this artifact's DSL as the
//! authoritative member plus its rfc8259 rendition (`IoFidelity::Exact`).
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::Serializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_zip::io::{decode_zip, encode_document_archive};

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct VcsIntoZip;

impl Serializer<VcsSnapshot> for VcsIntoZip {
    const INTO: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &VcsSnapshot) -> IoResult<IoPayload> {
        let error = |message: String| IoError { message: format!("VcsIntoZip: {message}"), diagnostics: Vec::new() };
        let archive = decode_zip(&encode_document_archive(from).map_err(|e| error(e.to_string()))?).map_err(|e| error(e.to_string()))?;
        Ok(IoOutcome::clean(IoPayload::Binary(store::ArtifactPack::encode_pack(&archive))))
    }
}
