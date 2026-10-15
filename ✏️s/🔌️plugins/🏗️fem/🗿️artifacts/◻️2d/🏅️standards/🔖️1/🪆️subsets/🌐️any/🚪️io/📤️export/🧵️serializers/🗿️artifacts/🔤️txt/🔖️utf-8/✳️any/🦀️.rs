//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{serialize_dsl_txt, ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};

pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;

pub struct Fem2dIntoTxt;

/// 🏗 This subset snapshot as UTF-8 DSL text — also the body the zip container leaf embeds.
pub fn dsl_text(from: &Fem2dSnapshot) -> String {
    <Fem2dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(from)
}

impl Serializer<Fem2dSnapshot> for Fem2dIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &Fem2dSnapshot, _: &ArchiveChildren, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}
