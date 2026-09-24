//! lowpoly <- dwg
//!
//! Geometry over the real `decode_dwg` / `dwg_drawing_to_mesh` pipeline: polyface-mesh entities
//! become one lowpoly object of triangles.
use crate::io::mesh_geometry::{snapshot_from_parts, text_error, PolygonPart};
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_dwg::schema::snapshot::decode_dwg;
use semio_s_artifact_stdio_dwg::{dwg_drawing_to_mesh, DwgSnapshot};

pub fn register() {}

pub fn deserialize(from: &DwgSnapshot) -> Result<LowpolySnapshot, store::TextError> {
    let drawing = from.drawing.to_native().map_err(|e| text_error(format!("dwg->lowpoly: {e}")))?;
    let mesh = dwg_drawing_to_mesh(&drawing);
    if mesh.indices.len() < 3 || mesh.positions.len() < 9 {
        return Err(text_error("dwg->lowpoly: the drawing contains no polyface mesh faces to import"));
    }
    let mut part = PolygonPart { name: "DWG Mesh".into(), ..Default::default() };
    for chunk in mesh.positions.chunks_exact(3) {
        part.positions.push([chunk[0], chunk[1], chunk[2]]);
    }
    for tri in mesh.indices.chunks_exact(3) {
        part.faces.push(vec![tri[0], tri[1], tri[2]]);
    }
    snapshot_from_parts("dwg", vec![part])
}

pub fn deserialize_bytes(bytes: &[u8]) -> Result<LowpolySnapshot, store::TextError> {
    let snap = decode_dwg(bytes).map_err(|e| text_error(format!("dwg->lowpoly: {e}")))?;
    deserialize(&snap)
}
