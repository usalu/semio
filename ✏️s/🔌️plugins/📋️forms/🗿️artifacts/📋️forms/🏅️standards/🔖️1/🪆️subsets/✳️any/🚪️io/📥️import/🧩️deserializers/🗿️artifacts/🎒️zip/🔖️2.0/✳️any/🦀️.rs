//! 📋️ forms <- zip — the DSL member of a document archive (`document_archive_member`) parsed as this
//! artifact's own DSL, the inverse of the sibling export (`IoFidelity::Exact`).
use crate::FormsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_zip::io::document_archive_member;
use semio_s_artifact_stdio_zip::ZipSnapshot;

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct ZipIntoForms;

impl Deserializer<FormsSnapshot> for ZipIntoForms {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<FormsSnapshot> {
        let error = |message: String| IoError { message: format!("ZipIntoForms: {message}"), diagnostics: Vec::new() };
        let IoPayload::Binary(bytes) = payload else {
            return Err(error("expected a binary zip snapshot".into()));
        };
        let archive = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| error(e.to_string()))?;
        let member = document_archive_member::<FormsSnapshot>();
        let entry = archive.entries.iter().find(|entry| entry.name == member).ok_or_else(|| error(format!("the archive has no {member} member")))?;
        let text = std::str::from_utf8(&entry.data).map_err(|e| error(e.to_string()))?;
        Ok(IoOutcome::clean(store::ArtifactDsl::parse_dsl(text).map_err(|e| error(e.to_string()))?))
    }
}
