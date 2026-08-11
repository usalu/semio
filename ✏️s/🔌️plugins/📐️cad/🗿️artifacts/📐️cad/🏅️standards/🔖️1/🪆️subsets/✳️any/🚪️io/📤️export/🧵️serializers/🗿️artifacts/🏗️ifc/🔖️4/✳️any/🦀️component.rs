//! Serialize cad to stdio.ifc.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::ifc::schema::snapshot::{IfcEntity, IfcHeader, IfcValue};
use semio_s_plugin_stdio::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<IfcSnapshot, store::PackError> {
    let raw = cad_to_wire(from);
    let mut entities = Vec::new();
    let mut next_id = 1u64;
    let mut i = 0;
    while i < raw.len() {
        let mut chunk = [0u8; 12];
        let n = (raw.len() - i).min(12);
        chunk[..n].copy_from_slice(&raw[i..i + n]);
        let x = f64::from(f32::from_le_bytes(chunk[0..4].try_into().unwrap()));
        let y = f64::from(f32::from_le_bytes(chunk[4..8].try_into().unwrap()));
        let z = f64::from(f32::from_le_bytes(chunk[8..12].try_into().unwrap()));
        entities.push(IfcEntity {
            id: next_id,
            name: "IFCCARTESIANPOINT".into(),
            args: vec![IfcValue::Aggregate(vec![IfcValue::Real(x), IfcValue::Real(y), IfcValue::Real(z)])],
            complex: vec![],
        });
        next_id += 1;
        i += 12;
    }
    let header = IfcHeader {
        file_description: vec![IfcValue::Aggregate(vec![IfcValue::String(String::new())]), IfcValue::String("2;1".into())],
        file_name: vec![
            IfcValue::String("semio.ifc".into()), IfcValue::String(String::new()),
            IfcValue::Aggregate(vec![IfcValue::String(String::new())]), IfcValue::Aggregate(vec![IfcValue::String(String::new())]),
            IfcValue::String("semio".into()), IfcValue::String(String::new()), IfcValue::String(String::new()),
        ],
        file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])],
    };
    Ok(IfcSnapshot { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), header, entities })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
