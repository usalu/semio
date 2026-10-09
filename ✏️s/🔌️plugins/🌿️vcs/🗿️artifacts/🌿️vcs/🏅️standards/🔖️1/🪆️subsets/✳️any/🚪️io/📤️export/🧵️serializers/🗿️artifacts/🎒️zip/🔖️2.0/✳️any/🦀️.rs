//! 🌿️ vcs → zip — the shared document archive (`encode_document_archive`): this artifact's DSL as the
//! authoritative member plus its rfc8259 rendition (`IoFidelity::Exact`).
use crate::VcsSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoOutcome,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{decode_zip, encode_document_archive};

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct VcsIntoZip;

impl Serializer<VcsSnapshot> for VcsIntoZip {
    const INTO: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &VcsSnapshot, _: &ArchiveChildren, control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        let error = |message: String| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("VcsIntoZip: {message}")));
        let archive = decode_zip(&encode_document_archive(from).map_err(|e| error(e.to_string()))?).map_err(|e| error(e.to_string()))?;
        Ok(IoOutcome::clean(IoPayload::Binary(store::ArtifactPack::encode_pack(&archive))))
    }
}
