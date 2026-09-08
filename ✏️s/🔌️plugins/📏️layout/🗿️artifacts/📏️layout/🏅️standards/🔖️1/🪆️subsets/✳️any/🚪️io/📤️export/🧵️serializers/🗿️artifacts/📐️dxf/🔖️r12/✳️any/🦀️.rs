//! Serialize layout to stdio.dxf.
use crate::LayoutSnapshot;
use semio_s_artifact_stdio_dxf::DxfSnapshot;

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<DxfSnapshot, store::PackError> {
    <DxfSnapshot as dsl::FromValue>::from_value(dsl::ToValue::to_value(from)).map_err(|error| store::PackError::Schema(error.to_string()))
}
