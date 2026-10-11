//! 💾️ The `brep.interchange` widgets: STEP, STL, OBJ and DWG text of a shape and the shapes those texts describe.
//!
//! STEP carries the exact geometry; the mesh formats triangulate the shape first (a stepped tessellation job) or rebuild a faceted solid from the triangles (a stepped mesh import cursor). Binary payloads travel as standard base64 text.

use crate::standards::v1::subsets::any::schema::inferences::geometry::registry::brep_curve::guarded;
use crate::standards::v1::subsets::any::schema::inferences::geometry::registry::phased_job::{cancelled_fault, Flow, Pipeline};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{BrepError, GeometryKind, MeshImportCursor, MeshTransfer};
use semio_framework_mesh_engine::{MeshData};
use semio_framework_mesh_engine::io::text::{mesh_from_obj, mesh_to_obj};
use semio_framework_mesh_engine::io::binary::{mesh_from_stl, mesh_to_stl};
use semio_framework_io_base64::{base64_standard_decode, base64_standard_encode};

const EMPTY: &str = "generation3d.geometry.interchange-empty";
const DECODE: &str = "generation3d.geometry.interchange-decode";
const PARSE: &str = "generation3d.geometry.interchange-parse";
const IMPORT_UNITS_PER_FUEL: usize = 64;

fn kernel(error: BrepError) -> WidgetFault {
    kernel_fault(&error)
}

fn empty(port: &str) -> WidgetFault {
    WidgetFault::new(EMPTY, "There is nothing to read: the text is empty.", "Es gibt nichts zu lesen: der Text ist leer.").at(port)
}

fn parse_fault(format: &str, reason: impl std::fmt::Display) -> WidgetFault {
    WidgetFault::new(PARSE, format!("The {format} data could not be read: {reason}."), format!("Die {format}-Daten konnten nicht gelesen werden: {reason}.")).at("data")
}

fn decode(text: &str) -> Result<Vec<u8>, WidgetFault> {
    base64_standard_decode(text.trim()).map_err(|error| WidgetFault::new(DECODE, format!("The text is not valid base64: {error}."), format!("Der Text ist kein gültiges Base64: {error}.")).at("data"))
}

//#region 🔖️Export
fn mesh_data(transfer: &MeshTransfer) -> Result<MeshData, WidgetFault> {
    if transfer.index.len() < 3 {
        return Err(WidgetFault::new(EMPTY, "The shape has no surface to triangulate.", "Die Form hat keine Oberfläche zum Triangulieren.").at("shape"));
    }
    Ok(MeshData { positions: transfer.position.clone(), normals: transfer.normal.clone(), indices: transfer.index.clone(), ..MeshData::default() })
}

fn export_mesh(kind: &Kind, inputs: &WidgetInputs, port: &'static str, encode: fn(&MeshData) -> Result<String, WidgetFault>) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let shape = inputs.shape("shape")?.clone();
        let deflection = inputs.number("deflection")?;
        Ok(Pipeline::new(kind).input(&shape).tessellate(0, deflection).finish(move |work| {
            let transfer = work.meshes.first().ok_or_else(|| empty("shape"))?;
            Ok(outputs([(port, GeometryValue::Text(encode(&mesh_data(transfer)?)?))]))
        }))
    })
}

fn export_step(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let shape = inputs.shape("shape")?.clone();
        let compound = shape.kind() == GeometryKind::Compound;
        Ok(Pipeline::new(kind).import(&shape).finish(move |work| {
            let handle = work.handle(0)?;
            let solids = if compound { work.session.brep().explode_sync(&handle).map_err(kernel)? } else { vec![handle] };
            let text = semio_s_artifact_stdio_step::geometry::export_step(work.session.brep(), &solids).map_err(kernel)?;
            Ok(outputs([("step", GeometryValue::Text(text))]))
        }))
    })
}

fn export_stl(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    export_mesh(kind, &inputs, "stl", |data| Ok(base64_standard_encode(mesh_to_stl(data))))
}

fn export_obj(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    export_mesh(kind, &inputs, "obj", |data| Ok(mesh_to_obj(data, "mesh")))
}

fn export_dwg(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    export_mesh(kind, &inputs, "dwg", |data| {
        let bytes = semio_s_artifact_stdio_dwg::dwg_to_bytes(&semio_s_artifact_stdio_dwg::mesh_to_dwg_drawing(data)).map_err(|reason| WidgetFault::new(PARSE, format!("The DWG data could not be written: {reason}."), format!("Die DWG-Daten konnten nicht geschrieben werden: {reason}.")).at("shape"))?;
        Ok(base64_standard_encode(bytes))
    })
}
//#endregion 🔖️Export

//#region 🔖️Import
fn import_step(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let data = inputs.text("data")?.to_string();
        if data.trim().is_empty() {
            return Err(empty("data"));
        }
        Ok(Pipeline::new(kind).once(move |work| {
            let imported = semio_s_artifact_stdio_step::geometry::import_step(work.session.brep(), &data).map_err(kernel)?;
            work.groups = vec![imported.into_iter().take(1).collect()];
            if work.groups[0].is_empty() {
                return Err(WidgetFault::new(EMPTY, "The STEP text holds no solid.", "Der STEP-Text enthält keinen Körper.").at("data"));
            }
            Ok(())
        }).exported("shape"))
    })
}

fn faceted(kind: &Kind, inputs: &WidgetInputs, read: impl FnOnce(&str) -> Result<MeshData, WidgetFault>) -> Box<dyn WidgetJob> {
    guarded(kind, || {
        let data = inputs.text("data")?.to_string();
        if data.trim().is_empty() {
            return Err(empty("data"));
        }
        let tolerance = inputs.number("tolerance")?;
        let mesh = read(&data)?;
        let mut cursor = Some(MeshImportCursor::new(mesh.positions, mesh.normals, mesh.indices, tolerance).map_err(|error| parse_fault("mesh", error))?);
        let mut running: Option<MeshImportCursor> = None;
        Ok(Pipeline::new(kind).step(move |work, fuel| {
            if running.is_none() {
                running = cursor.take();
            }
            let importing = running.as_mut().ok_or_else(|| cancelled_fault())?;
            match work.session.brep().step_mesh_import_sync(importing, fuel.saturating_mul(IMPORT_UNITS_PER_FUEL)).map_err(kernel)? {
                Some(handle) => {
                    work.groups = vec![vec![handle]];
                    Ok(Flow::Next)
                }
                None => {
                    let (done, total, _) = importing.progress();
                    Ok(Flow::Working(if total == 0 { 0.0 } else { (done as f32 / total as f32).clamp(0.0, 1.0) }))
                }
            }
        }).exported("shape"))
    })
}

fn import_stl(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    faceted(kind, &inputs, |text| mesh_from_stl(&decode(text)?).map_err(|reason| parse_fault("STL", reason)))
}

fn import_obj(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    faceted(kind, &inputs, |text| mesh_from_obj(text).map_err(|reason| parse_fault("OBJ", reason)))
}

fn import_dwg(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    faceted(kind, &inputs, |text| {
        let drawing = semio_s_artifact_stdio_dwg::dwg_from_bytes(&decode(text)?).map_err(|reason| parse_fault("DWG", reason))?;
        Ok(semio_s_artifact_stdio_dwg::dwg_drawing_to_mesh(&drawing))
    })
}
//#endregion 🔖️Import

/// 🗃️ Every `brep.interchange` kind and the compute that starts its job.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "brep.interchange.exportStep", start: export_step },
    ComputeEntry { id: "brep.interchange.importStep", start: import_step },
    ComputeEntry { id: "brep.interchange.exportStl", start: export_stl },
    ComputeEntry { id: "brep.interchange.importStl", start: import_stl },
    ComputeEntry { id: "brep.interchange.exportObj", start: export_obj },
    ComputeEntry { id: "brep.interchange.importObj", start: import_obj },
    ComputeEntry { id: "brep.interchange.exportDwg", start: export_dwg },
    ComputeEntry { id: "brep.interchange.importDwg", start: import_dwg },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
