//! 📋️ forms -> zip — the shared document archive (`encode_document_archive`): this artifact's DSL as
//! the authoritative member plus its rfc8259 rendition, as real zip 2.0 bytes (`IoFidelity::Exact`).
use crate::FormsSnapshot;
use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};
use semio_framework::io_schema::{Dialect, IoError, IoFidelity, IoOutcome, IoPayload, IoResult};
use semio_framework_plugin::{StandardId, SubsetId};
use semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::io::{decode_zip, encode_document_archive};

pub const ZIP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.zip", standard: StandardId("2.0"), subset: SubsetId::ANY };

pub struct FormsIntoZip;

impl Serializer<FormsSnapshot> for FormsIntoZip {
    const INTO: Dialect = ZIP_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &FormsSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let archive = decode_zip(&encode_document_archive(from).map_err(|e| IoError::from_value_error(e.into_value_error()))?).map_err(|e| IoError::from_value_error(e.into_value_error()))?;
        Ok(IoOutcome::clean(IoPayload::Binary(store::ArtifactPack::encode_pack(&archive))))
    }
}
