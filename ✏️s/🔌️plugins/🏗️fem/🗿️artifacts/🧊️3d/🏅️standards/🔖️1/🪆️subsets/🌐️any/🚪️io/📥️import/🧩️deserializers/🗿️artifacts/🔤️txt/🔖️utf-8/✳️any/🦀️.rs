//! txt import via framework `io_mechanism::deserialize_dsl_txt` (UTF-8 DSL carrier).

use crate::Fem3dSnapshot;
use semio_framework_os_kernel::io::io_mechanism::{deserialize_dsl_txt, Deserializer};
use {semio_framework_artifact_reference::Dialect,semio_framework::io_schema::IoError,semio_framework::io_schema::IoFidelity,semio_framework::io_schema::IoPayload,semio_framework::io_schema::IoResult};
use {semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

pub const TXT_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId::ANY };

pub struct TxtIntoFem3d;

/// 🏗 Parses UTF-8 DSL text into this subset snapshot — also used by the zip container leaf.
pub fn from_dsl_text(text: &str) -> Result<Fem3dSnapshot, IoError> {
    <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
}

impl Deserializer<Fem3dSnapshot> for TxtIntoFem3d {
    const FROM: Dialect = TXT_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Exact;
    async fn deserialize(payload: &IoPayload) -> IoResult<Fem3dSnapshot> {
        deserialize_dsl_txt(payload)
    }
}
