//! 📋️ forms <- zip — the DSL member of a document archive (`document_archive_member`) parsed as this
//! artifact's own DSL, the inverse of the sibling export (`IoFidelity::Exact`).
use crate::FormsSnapshot;
use semio_framework_os_kernel::io::io_mechanism::Deserializer;
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::document_archive_member;
use semio_s_artifact_stdio_zip::ZipSnapshot;

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct ZipIntoForms;

impl Deserializer<FormsSnapshot> for ZipIntoForms {
    const FROM: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<FormsSnapshot> {
        let invalid = |message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message));
        let IoPayload::Binary(bytes) = payload else {
            return Err(invalid("ZipIntoForms: expected a binary zip snapshot".to_string()));
        };
        let archive = <ZipSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|e| IoError::from_value_error(match e.into_value_error() { Ok(cause) => cause, Err(error) => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, format!("in-memory decode reported a transport failure: {error}")) }))?;
        let member = document_archive_member::<FormsSnapshot>();
        let entry = archive.entries.iter().find(|entry| entry.name == member).ok_or_else(|| invalid(format!("ZipIntoForms: the archive has no {member} member")))?;
        let text = std::str::from_utf8(&entry.data).map_err(|e| invalid(e.to_string()))?;
        Ok(IoOutcome::clean(store::ArtifactDsl::parse_dsl(text).map_err(|e| IoError::from_value_error(semio_framework_value::ValueError::new(e.kind,e.message)))?))
    }
}
