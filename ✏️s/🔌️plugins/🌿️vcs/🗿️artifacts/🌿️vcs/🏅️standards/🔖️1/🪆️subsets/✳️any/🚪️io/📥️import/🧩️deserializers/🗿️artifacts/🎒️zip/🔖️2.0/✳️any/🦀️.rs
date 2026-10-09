//! 🌿️ vcs ← zip — the DSL member of a document archive (`document_archive_member`) parsed as this
//! artifact's own DSL (`IoFidelity::Exact`).
use crate::VcsSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::document_archive_member;
use semio_s_artifact_stdio_zip::ZipSnapshot;

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct ZipIntoVcs;

impl Deserializer<VcsSnapshot> for ZipIntoVcs {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<VcsSnapshot> {
        let error = |message: String| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("ZipIntoVcs: {message}")));
        let IoPayload::Binary(bytes) = payload else {
            return Err(error("expected a binary zip payload".into()));
        };
        let archive = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| error(e.to_string()))?;
        let member = document_archive_member::<VcsSnapshot>();
        let entry = archive.entries.iter().find(|entry| entry.name == member).ok_or_else(|| error(format!("the archive has no {member} member")))?;
        Ok(IoOutcome::clean(store::ArtifactDsl::parse_dsl(std::str::from_utf8(&entry.data).map_err(|e| error(e.to_string()))?).map_err(|e| error(e.to_string()))?))
    }
}
