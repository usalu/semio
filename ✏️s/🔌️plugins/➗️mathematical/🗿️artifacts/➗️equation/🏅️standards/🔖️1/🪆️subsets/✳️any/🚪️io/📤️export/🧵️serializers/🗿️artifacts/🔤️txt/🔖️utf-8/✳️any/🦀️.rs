//! txt export via framework `io_mechanism::serialize_dsl_txt` (UTF-8 DSL carrier).

use crate::EquationSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{serialize_dsl_txt, ArchiveChildren, Serializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};

pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;

pub struct EquationIntoTxt;

impl Serializer<EquationSnapshot> for EquationIntoTxt {
    const INTO: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn serialize(from: &EquationSnapshot, _: &ArchiveChildren, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<IoPayload> {
        serialize_dsl_txt(from)
    }
}
