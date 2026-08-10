//! Serialize cad to stdio.dwg.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::dwg::{DwgDecodeStatus, DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<DwgSnapshot, store::PackError> {
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version: "AC1027".into(),
        bytes: cad_to_wire(from),
        section_names: Vec::new(),
        // 🎫️26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION: this
        // serializer emits synthetic bytes, not a real R2004+ file -- no real section decode
        // applies here, hence the honest `SentinelOnly` status rather than fabricating one.
        sections: Vec::new(),
        decode_status: DwgDecodeStatus::SentinelOnly,
    })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
