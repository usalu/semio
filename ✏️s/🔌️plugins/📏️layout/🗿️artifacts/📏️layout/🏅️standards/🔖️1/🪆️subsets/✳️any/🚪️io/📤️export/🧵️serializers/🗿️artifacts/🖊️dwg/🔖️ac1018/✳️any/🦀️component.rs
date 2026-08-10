//! Serialize layout to stdio.dwg.
use crate::artifacts::layout::LayoutSnapshot;
use semio_s_plugin_stdio::artifacts::dwg::{DwgDecodeStatus, DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn serialize(from: &LayoutSnapshot) -> Result<DwgSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    let bytes = serde_json::to_vec(&value).map_err(|e| store::PackError::Schema(e.to_string()))?;
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version: String::new(),
        bytes,
        section_names: Vec::new(),
        // 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: this
        // serializer emits synthetic JSON bytes, not a real R2004+ file -- honest `SentinelOnly`
        // rather than fabricating a decode status.
        sections: Vec::new(),
        decode_status: DwgDecodeStatus::SentinelOnly,
    })
}
