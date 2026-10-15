//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{serialize_dsl_txt, ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};

pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;

pub struct Fem3dIntoTxt;

/// 🏗 This subset snapshot as UTF-8 DSL text — also the body the zip container leaf embeds.
pub fn dsl_text(from: &Fem3dSnapshot) -> String {
    <Fem3dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(from)
}

impl Serializer<Fem3dSnapshot> for Fem3dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem3dSnapshot, _: &ArchiveChildren, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}
