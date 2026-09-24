//! 🌿️ vcs ← zip — the DSL member of a document archive (`document_archive_member`) parsed as this
//! artifact's own DSL (`IoFidelity::Exact`).
use crate::VcsSnapshot;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_zip::io::document_archive_member;
use semio_s_artifact_stdio_zip::ZipSnapshot;

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct ZipIntoVcs;

impl Deserializer<VcsSnapshot> for ZipIntoVcs {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<VcsSnapshot> {
        let error = |message: String| IoError { message: format!("ZipIntoVcs: {message}"), diagnostics: Vec::new() };
        let IoPayload::Binary(bytes) = payload else {
            return Err(error("expected a binary zip payload".into()));
        };
        let archive = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| error(e.to_string()))?;
        let member = document_archive_member::<VcsSnapshot>();
        let entry = archive.entries.iter().find(|entry| entry.name == member).ok_or_else(|| error(format!("the archive has no {member} member")))?;
        Ok(IoOutcome::clean(store::ArtifactDsl::parse_dsl(std::str::from_utf8(&entry.data).map_err(|e| error(e.to_string()))?).map_err(|e| error(e.to_string()))?))
    }
}
