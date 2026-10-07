//! binary rep for stdio.json 🔺️diff

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

impl semio_framework_os_kernel::DiffBinary for crate::standards::v1::subsets::any::schema::diff::Din16798Diff {
    fn encode_diff(&self) -> Result<Vec<u8>, semio_framework_os_kernel::ProtocolError> {
        Ok(semio_framework_os_kernel::os_store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, semio_framework_os_kernel::ProtocolError> {
        let value = semio_framework_os_kernel::os_store::pack_rt::decode_wire_value(bytes).map_err(|error| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })?;
        semio_framework_value::FromValue::from_value(value).map_err(|error: semio_framework_value::ValueError| semio_framework_os_kernel::ProtocolError::Malformed { what: "artifact diff", offset: 0, detail: error.to_string() })
    }
}
