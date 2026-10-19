//! 📥️ txt import via framework `io_mechanism::deserialize_dsl_txt` (UTF-8 DSL carrier).

use crate::PresentationSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{deserialize_dsl_txt, Deserializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};

pub const TXT_DIALECT: Dialect = semio_framework::io_schema::CARRIER_TEXT;

pub struct TxtIntoPresentation;

impl Deserializer<PresentationSnapshot> for TxtIntoPresentation {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload, _control: &mut semio_framework_os_kernel::io::io_mechanism::IoRunControl<'_, '_>) -> IoResult<PresentationSnapshot> {
        deserialize_dsl_txt(payload)
    }
}
