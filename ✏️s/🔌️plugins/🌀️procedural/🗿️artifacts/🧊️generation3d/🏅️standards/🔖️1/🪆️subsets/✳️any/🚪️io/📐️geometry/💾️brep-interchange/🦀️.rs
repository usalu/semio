//! 💾️ The `brep.interchange` widgets: STEP, STL, OBJ and DWG text of a shape and the shapes those texts describe.
//!
//! STEP carries the exact geometry; the mesh formats triangulate the shape first (a stepped tessellation job) or rebuild a faceted solid from the triangles (a stepped mesh import cursor). Binary payloads travel as standard base64 text.

use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_brep_curve::guarded;
use semio_s_artifact_procedural_3d_phased_job::cancelled_fault;
use semio_s_artifact_procedural_3d_phased_job::Flow;
use semio_s_artifact_procedural_3d_phased_job::{Pipeline, Work};
use semio_s_artifact_procedural_3d_geometry_compute::finish;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::{StepContext, StepFault, fuel_budget, settle, spend};
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::kernel_fault;
use semio_s_artifact_procedural_3d_geometry_value::outputs;
use semio_s_artifact_procedural_3d_geometry_value::GeometryValue;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;
use {semio_framework_3d_brep_engine::{BrepError, GeometryKind, MeshImportCursor}, semio_framework_3d_brep_representation::contract::MeshTransfer};
use semio_framework_mesh_engine::MeshData;
use semio_framework_mesh_engine::io::text::{mesh_from_obj, mesh_to_obj};
use semio_framework_mesh_engine::io::binary::{mesh_from_stl, mesh_to_stl};
use semio_framework_io_base64::{Base64Control, Base64DecodeCursor, Base64DecodeState, Base64Input, Base64InputRange, Base64Phase, base64_standard_decode, base64_standard_encode};

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

fn decode_fault(error: impl std::fmt::Display) -> WidgetFault {
    WidgetFault::new(DECODE, format!("The text is not valid base64: {error}."), format!("Der Text ist kein gültiges Base64: {error}.")).at("data")
}

fn decode(text: &str) -> Result<Vec<u8>, WidgetFault> {
    base64_standard_decode(text.trim()).map_err(decode_fault)
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
            let text = semio_s_artifact_stdio_step_geometry::geometry::export_step(work.session.brep(), &solids).map_err(kernel)?;
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
        let invalid = |reason| WidgetFault::new(PARSE, format!("The DWG data could not be written: {reason}."), format!("Die DWG-Daten konnten nicht geschrieben werden: {reason}.")).at("shape");
        let drawing = semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::mesh_to_dwg_drawing(data).map_err(&invalid)?;
        let bytes = semio_s_artifact_stdio_dwg_write::standards::v_ac1024::subsets::any::io::export::dwg_to_bytes(&drawing).map_err(invalid)?;
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
            let imported = semio_s_artifact_stdio_step_geometry::geometry::import_step(work.session.brep(), &data).map_err(kernel)?;
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
        let mut cursor = mesh_import_cursor(mesh, tolerance)?;
        Ok(Pipeline::new(kind).step(move |work, cx| step_mesh_import(&mut cursor, work, cx)).exported("shape"))
    })
}

fn mesh_import_cursor(mesh: MeshData, tolerance: f64) -> Result<MeshImportCursor, WidgetFault> {
    MeshImportCursor::new(mesh.positions, mesh.normals, mesh.indices, tolerance).map_err(|error| parse_fault("mesh", error))
}

fn step_mesh_import(importing: &mut MeshImportCursor, work: &mut Work, cx: &mut StepContext<'_>) -> Result<Flow, StepFault> {
    let fuel = fuel_budget(cx);
    let step = work.session.brep().step_mesh_import_sync(importing, fuel.saturating_mul(IMPORT_UNITS_PER_FUEL)).map_err(kernel)?;
    settle(cx, fuel, step.is_none());
    match step {
        Some(handle) => {
            work.groups = vec![vec![handle]];
            Ok(Flow::Next)
        }
        None => {
            let (done, total, _) = importing.progress();
            Ok(Flow::Working((done as f32 / total.max(1) as f32).clamp(0.0, 1.0)))
        }
    }
}

fn import_stl(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    faceted(kind, &inputs, |text| mesh_from_stl(&decode(text)?).map_err(|reason| parse_fault("STL", reason)))
}

fn import_obj(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    faceted(kind, &inputs, |text| mesh_from_obj(text).map_err(|reason| parse_fault("OBJ", reason)))
}

fn import_dwg(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    use semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::{DwgMeshProjection, DwgMeshProjectionStep};
    use semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::{DwgDrawingRead, DwgDrawingReadStep};
    guarded(kind, || {
        inputs.text("data")?;
        let tolerance = inputs.number("tolerance")?;
        let mut data = Some(inputs.into_text("data")?);
        let mut transport = None;
        let mut decoded = None;
        let mut read = None;
        let mut project = None;
        let mut cursor = None;
        Ok(Pipeline::new(kind).step(move |work, cx| {
            if let Some(cursor) = &mut cursor {
                return step_mesh_import(cursor, work, cx).map(|step| match step {
                    Flow::Working(progress) => Flow::Working(0.75 + 0.25 * progress),
                    other => other,
                });
            }
            if let Some(text) = data.take() {
                transport = Some(dwg_input_decoder(text)?);
                spend(cx, 1);
            }
            if let Some(decoder) = &mut transport {
                if let Some(bytes) = step_dwg_transport(decoder, cx)? {
                    decoded = Some(bytes);
                    transport = None;
                    return Ok(Flow::Working(0.25));
                }
                let progress = decoder.progress();
                let fraction = progress.completed as f32 / progress.total.max(1) as f32;
                let decoded_phase = if progress.phase == Base64Phase::Decode { 1.0 } else { 0.0 };
                return Ok(Flow::Working((decoded_phase + fraction) * 0.125));
            }
            if let Some(bytes) = decoded.take() {
                read = Some(DwgDrawingRead::open(bytes).map_err(|error| parse_fault("DWG", error))?);
                spend(cx, 1);
            }
            if let Some(reader) = &mut read {
                while !cx.should_yield() {
                    let step = reader.step().map_err(|error| parse_fault("DWG", error))?;
                    spend(cx, 1);
                    if let DwgDrawingReadStep::Complete(drawing) = step {
                        project = Some(DwgMeshProjection::new(drawing.entities.into_iter()));
                        read = None;
                        return Ok(Flow::Working(0.5));
                    }
                }
                return Ok(Flow::Working(0.25 + 0.25 * reader.progress()));
            }
            let projection = project.as_mut().ok_or_else(cancelled_fault)?;
            while !cx.should_yield() {
                let step = projection.step().map_err(|error| parse_fault("DWG", error))?;
                spend(cx, 1);
                if let DwgMeshProjectionStep::Complete = step {
                    let mesh = projection.take_mesh().ok_or_else(cancelled_fault)?;
                    cursor = Some(mesh_import_cursor(mesh, tolerance)?);
                    project = None;
                    return Ok(Flow::Working(0.75));
                }
            }
            Ok(Flow::Working(0.5 + 0.25 * projection.progress()))
        }).exported("shape"))
    })
}

fn dwg_input_decoder(text: String) -> Result<Base64DecodeCursor<'static>, WidgetFault> {
    let suffix = text.trim_start();
    let start = text.len() - suffix.len();
    let end = start + suffix.trim_end().len();
    if start == end {
        return Err(empty("data"));
    }
    Ok(Base64DecodeCursor::new_range(Base64Input::OwnedText(text), Base64InputRange { start, end }))
}

fn step_dwg_transport(decoder: &mut Base64DecodeCursor<'_>, cx: &mut StepContext<'_>) -> Result<Option<Vec<u8>>, WidgetFault> {
    let mut control = Base64Control { maximum_output_bytes: decoder.expected_output_bytes(), progress: &mut |_| true };
    while !cx.should_yield() {
        let step = decoder.step(4096, &mut control).map_err(decode_fault)?;
        spend(cx, 1);
        if step == Base64DecodeState::Complete {
            return decoder.take_output().map(Some).ok_or_else(cancelled_fault);
        }
    }
    Ok(None)
}
//#endregion 🔖️Import

/// 📚️ This facet's catalogue category: its slug and its schema-first definition.
pub const CATEGORY: (&str, &str) = ("brep-interchange", include_str!("../../../🧬️schema/🗂️catalogue/🔣️brep-interchange.json"));
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
