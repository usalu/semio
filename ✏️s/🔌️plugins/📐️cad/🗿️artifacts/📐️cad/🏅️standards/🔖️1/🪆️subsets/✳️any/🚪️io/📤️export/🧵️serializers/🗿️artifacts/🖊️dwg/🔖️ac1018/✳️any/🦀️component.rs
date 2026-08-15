//! Serialize cad to stdio.dwg.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::dwg::{DwgSnapshot, STDIO_DWG_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(_from: &CadSnapshot) -> Result<DwgSnapshot, store::PackError> {
    Ok(DwgSnapshot {
        schema: STDIO_DWG_DOCUMENT_SCHEMA.into(),
        version: "AC1027".into(),
        maintenance_version: 0,
        codepage: 0,
        drawing: Default::default(),
        header: Default::default(),
        classes: Vec::new(),
        dependencies: Vec::new(),
        summary: Default::default(),
        application: Default::default(),
        template: Default::default(),
        auxiliary_header: Default::default(),
        revision_history: Default::default(),
        preview: Default::default(),
        application_history: Default::default(),
    })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
