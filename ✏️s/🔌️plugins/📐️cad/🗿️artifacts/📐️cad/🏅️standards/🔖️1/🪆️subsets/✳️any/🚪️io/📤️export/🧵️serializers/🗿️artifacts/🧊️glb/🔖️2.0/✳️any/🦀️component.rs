//! Serialize cad to stdio.glb.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::glb::{GlbSnapshot, STDIO_GLB_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<GlbSnapshot, store::PackError> {
    Ok(GlbSnapshot {
        schema: STDIO_GLB_DOCUMENT_SCHEMA.into(),
        payload: semio_s_plugin_stdio::artifacts::glb::schema::snapshot::GlbPayload {
            gltf_json: String::new(),
            bin: cad_to_wire(from),
        },
    })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
