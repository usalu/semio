//! Serialize cad to stdio.ifc.

use crate::artifacts::cad::CadSnapshot;
use crate::artifacts::cad::io::cad_to_wire;
use semio_s_plugin_stdio::artifacts::step::engine::part21::{Part21Builder, Part21Header, Part21Value};
use semio_s_plugin_stdio::artifacts::ifc::{IfcSnapshot, STDIO_IFC_DOCUMENT_SCHEMA};

//#region Serialize
pub fn register() {}

pub fn serialize(from: &CadSnapshot) -> Result<IfcSnapshot, store::PackError> {
    let raw = cad_to_wire(from);
    let mut b = Part21Builder::new();
    let mut i = 0;
    while i < raw.len() {
        let mut chunk = [0u8; 12];
        let n = (raw.len() - i).min(12);
        chunk[..n].copy_from_slice(&raw[i..i + n]);
        let x = f64::from(f32::from_le_bytes(chunk[0..4].try_into().unwrap()));
        let y = f64::from(f32::from_le_bytes(chunk[4..8].try_into().unwrap()));
        let z = f64::from(f32::from_le_bytes(chunk[8..12].try_into().unwrap()));
        b.alloc("IFCCARTESIANPOINT", vec![Part21Value::List(vec![Part21Value::Real(x), Part21Value::Real(y), Part21Value::Real(z)])]);
        i += 12;
    }
    let document = b.build(Part21Header {
        file_description: vec![Part21Value::List(vec![Part21Value::Str(String::new())]), Part21Value::Str("2;1".into())],
        file_name: vec![
            Part21Value::Str("semio.ifc".into()), Part21Value::Str(String::new()),
            Part21Value::List(vec![Part21Value::Str(String::new())]), Part21Value::List(vec![Part21Value::Str(String::new())]),
            Part21Value::Str("semio".into()), Part21Value::Str(String::new()), Part21Value::Str(String::new()),
        ],
        file_schema: vec![Part21Value::List(vec![Part21Value::Str("IFC4".into())])],
    });
    Ok(IfcSnapshot { schema: STDIO_IFC_DOCUMENT_SCHEMA.into(), document })
}

pub fn serialize_text(from: &CadSnapshot) -> Result<String, store::PackError> {
    Ok(<CadSnapshot as store::ArtifactDsl>::print_dsl(from))
}
//#endregion Serialize
