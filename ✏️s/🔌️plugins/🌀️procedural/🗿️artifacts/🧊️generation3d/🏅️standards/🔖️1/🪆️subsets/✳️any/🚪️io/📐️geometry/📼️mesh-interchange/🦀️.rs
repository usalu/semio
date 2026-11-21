//! 📼️ `mesh.interchange` computes: OBJ, JSON, binary STL and binary GLB export (the binary formats as base64 text) and OBJ, STL and GLB import, all through the mesh engine codecs and typed `HalfedgeMesh` values.
//!
//! JSON and OBJ writers advance in bounded slices, as do the binary format's text codecs.

use semio_framework_3d::mesh::{MeshModelingOps};
use semio_s_artifact_procedural_3d_model::catalogue::Kind;
use semio_s_artifact_procedural_3d_mesh_support::{cancelled_fault,capacity_fault,polygon_capture_slice};
use semio_s_artifact_procedural_3d_mesh_support::export_fault;
use semio_s_artifact_procedural_3d_mesh_support::from_source;
use semio_s_artifact_procedural_3d_mesh_support::import_fault;
use semio_s_artifact_procedural_3d_mesh_support::mesh_output;
use semio_s_artifact_procedural_3d_mesh_support::modeling_slice;
use semio_s_artifact_procedural_3d_mesh_support::ratio;
use semio_s_artifact_procedural_3d_mesh_support::run;
use semio_s_artifact_procedural_3d_mesh_support::tessellation_slice;
use semio_s_artifact_procedural_3d_mesh_support::Flow;
use semio_s_artifact_procedural_3d_mesh_support::KernelResult;
use semio_s_artifact_procedural_3d_mesh_support::Machine;
use semio_s_artifact_procedural_3d_mesh_support::Parse;
use semio_s_artifact_procedural_3d_mesh_support::BATCH;
use semio_s_artifact_procedural_3d_geometry_compute::ComputeEntry;
use semio_s_artifact_procedural_3d_geometry_compute::{fuel_budget, StepContext};
use semio_s_artifact_procedural_3d_geometry_compute::WidgetJob;
use semio_s_artifact_procedural_3d_geometry_inputs::WidgetInputs;
use semio_s_artifact_procedural_3d_geometry_value::outputs;
use semio_s_artifact_procedural_3d_geometry_value::GeometryValue;
use semio_s_artifact_procedural_3d_geometry_value::Outputs;
use semio_s_artifact_procedural_3d_geometry_value::WidgetFault;
use semio_framework_3d::mesh::{HalfedgeMesh, MeshKernelError, MeshModelingJob, MeshModelingStep, MeshObjExport, MeshPolygonSourceCapture, MeshPolygonSourceStep, MeshTessellationJob, MeshTessellationStep};
use semio_framework_io_base64::{base64_standard_decode, base64_standard_encode};
use semio_framework_mesh_engine::{MeshData};
use semio_framework_mesh_engine::io::binary::{mesh_from_glb, mesh_from_stl, mesh_to_glb, mesh_to_stl};
use semio_framework_mesh_engine::io::text::{ObjSourceCursor,PolygonJsonExport};
use std::sync::Arc;

const FACE_LIMIT: usize = 100_000;
const ENCODE_CHUNK: usize = 49_152;
const DECODE_CHUNK: usize = 65_536;

fn text_output(port: &str, text: String) -> Outputs {
    outputs([(port, GeometryValue::Text(text))])
}

//#region 🔖️Export
struct ExportObj {
    mesh: Arc<HalfedgeMesh>,
    export: MeshObjExport,
}

impl Machine for ExportObj {
    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {
        let fuel = fuel_budget(cx);
        match self.export.step(&self.mesh, fuel).kernel()? {
            Some(text) => Ok(Flow::Done(text_output("text", text))),
            None => {
                let (done, total) = self.export.progress(&self.mesh);
                Ok(Flow::Working(done as f32 / total.max(1) as f32))
            }
        }
    }

    fn cancel(&mut self) {}
}

fn export_obj(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    run(kind, move || Ok(Box::new(ExportObj { mesh: inputs.mesh("mesh")?.clone(), export: MeshObjExport::new() }) as Box<dyn Machine>))
}

enum ExportJson {
    Capture {mesh:Arc<HalfedgeMesh>,cursor:Box<MeshPolygonSourceCapture>},
    Encode(Box<PolygonJsonExport>),
}

impl Machine for ExportJson {
    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {
        if cx.is_cancelled(){return Err(cancelled_fault());}
        match self {
            Self::Capture{mesh,cursor}=>{
                cx.set_stage("mesh-json-capture");
                match polygon_capture_slice(cursor,mesh,cx).kernel()? {
                    MeshPolygonSourceStep::Working=>{
                        let (done,total)=cursor.progress(mesh);
                        Ok(Flow::Working(0.4*done as f32/total.max(1) as f32))
                    },
                    MeshPolygonSourceStep::Done(source)=>{
                        *self=Self::Encode(Box::new(PolygonJsonExport::new(source)));
                        Ok(Flow::Working(0.4))
                    },
                    MeshPolygonSourceStep::Cancelled=>Err(cancelled_fault()),
                }
            },
            Self::Encode(writer)=>{
                cx.set_stage("mesh-json-encode");
                for _ in 0..fuel_budget(cx).min(BATCH){
                    if cx.should_yield(){break;}
                    let grant=cx.retained_grant();
                    let demand=writer.demand(grant.maximum_copy_bytes).map_err(MeshKernelError::Retained).kernel()?;
                    if !cx.admit_work_step(demand).map_err(MeshKernelError::Retained).kernel()?{break;}
                    let Some((result,work))=cx.with_original_effect(grant,|_,effect|writer.step(effect))
                        .map_err(MeshKernelError::Retained).kernel()? else {break;};
                    cx.consume_work(work);
                    let receipt=result.as_ref().map_or_else(|error|error.retained_progress(),|(_,receipt)|*receipt);
                    if receipt!=Default::default(){cx.consume_retained(receipt).map_err(MeshKernelError::Retained).kernel()?;}
                    if let Some(text)=result.map_err(MeshKernelError::Retained).kernel()?.0 {
                        if cx.is_cancelled(){return Err(cancelled_fault());}
                        return Ok(Flow::Done(text_output("text",text)));
                    }
                    if work==0{break;}
                }
                Ok(Flow::Working(0.4+0.6*writer.progress()))
            }
        }
    }

    fn cancel(&mut self) {
        match self {Self::Capture{cursor,..}=>cursor.cancel(),Self::Encode(writer)=>writer.cancel()}
    }
}

fn export_json(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    run(kind, move || Ok(Box::new(ExportJson::Capture {
        mesh:inputs.mesh("mesh")?.clone(),cursor:Box::new(MeshPolygonSourceCapture::new()),
    }) as Box<dyn Machine>))
}

#[derive(Clone, Copy)]
enum Binary {
    Stl,
    Glb,
}

struct ExportBinary {
    format: Binary,
    port: &'static str,
    tessellation: Option<MeshTessellationJob>,
    bytes: Vec<u8>,
    encoded: String,
    cursor: usize,
    encoding: bool,
}

impl Machine for ExportBinary {
    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {
        let fuel = fuel_budget(cx);
        if let Some(job) = self.tessellation.as_mut() {
            return match tessellation_slice(job, fuel.saturating_mul(BATCH), cx).kernel()? {
                MeshTessellationStep::Working(progress) => Ok(Flow::Working(0.4 * ratio(progress))),
                MeshTessellationStep::Cancelled(_) => Err(semio_s_artifact_procedural_3d_mesh_support::cancelled_fault()),
                MeshTessellationStep::Done(transfer) => {
                    self.tessellation = None;
                    let data = MeshData { positions: transfer.positions, normals: transfer.normals, colors: transfer.colors, indices: transfer.indices, uvs: transfer.uvs, ..MeshData::default() };
                    self.bytes = match self.format {
                        Binary::Stl => mesh_to_stl(&data),
                        Binary::Glb => mesh_to_glb(&data).map_err(|error| export_fault(&error))?,
                    };
                    self.encoding = true;
                    Ok(Flow::Working(0.5))
                }
            };
        }
        if !self.encoding {
            return Err(export_fault("the writer has no mesh"));
        }
        let end = (self.cursor + ENCODE_CHUNK.saturating_mul(fuel)).min(self.bytes.len());
        self.encoded.push_str(&base64_standard_encode(&self.bytes[self.cursor..end]));
        self.cursor = end;
        if self.cursor == self.bytes.len() {
            return Ok(Flow::Done(text_output(self.port, std::mem::take(&mut self.encoded))));
        }
        Ok(Flow::Working(0.5 + 0.5 * self.cursor as f32 / self.bytes.len().max(1) as f32))
    }

    fn cancel(&mut self) {
        self.tessellation = None;
        self.bytes = Vec::new();
        self.encoded = String::new();
    }
}

fn export_binary(kind: &Kind, inputs: WidgetInputs, format: Binary, port: &'static str) -> Box<dyn WidgetJob> {
    run(kind, move || {
        let mesh = (**inputs.mesh("mesh")?).clone();
        Ok(Box::new(ExportBinary { format, port, tessellation: Some(MeshTessellationJob::new(mesh)), bytes: Vec::new(), encoded: String::new(), cursor: 0, encoding: false }) as Box<dyn Machine>)
    })
}

fn export_stl(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    export_binary(kind, inputs, Binary::Stl, "stl")
}

fn export_glb(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    export_binary(kind, inputs, Binary::Glb, "glb")
}
//#endregion 🔖️Export

//#region 🔖️Import
fn import_obj(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    from_source(kind, move || {
        let text = inputs.text("data")?.to_string();
        let mut cursor = ObjSourceCursor::new();
        Ok(Box::new(move |fuel: usize, cx: &mut StepContext<'_>| {
            for _ in 0..fuel {
                if cx.should_yield() {break;}
                if !cx.admit_work_step(cursor.normal_step_demand(&text)).map_err(|error|import_fault(error).at("data"))? {break;}
                let grant=cx.retained_grant();
                let Some((source,work))=cx.with_original_effect(grant,|_,effect|cursor.step_original(&text,effect))
                    .map_err(|error|import_fault(error).at("data"))? else {break;};
                cx.consume_work(work);
                cx.consume_retained(cursor.take_normal_progress()).map_err(|error|import_fault(error).at("data"))?;
                if let Some(source)=source.map_err(|error|import_fault(error).at("data"))? {return Ok(Some(source));}
                if work==0 {break;}
            }
            Ok(None)
        }) as Parse)
    })
}

struct ImportBinary {
    format: Binary,
    encoded: Vec<u8>,
    decoded: Vec<u8>,
    cursor: usize,
    job: Option<MeshModelingJob>,
}

impl Machine for ImportBinary {
    fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<Flow, WidgetFault> {
        let fuel = fuel_budget(cx);
        if self.job.is_none() {
            if self.cursor < self.encoded.len() {
                let end = (self.cursor + DECODE_CHUNK.saturating_mul(fuel)).min(self.encoded.len());
                self.decoded.extend(base64_standard_decode(&self.encoded[self.cursor..end]).map_err(|error| import_fault(error).at("data"))?);
                self.cursor = end;
                return Ok(Flow::Working(0.3 * self.cursor as f32 / self.encoded.len().max(1) as f32));
            }
            let data = match self.format {
                Binary::Stl => mesh_from_stl(&self.decoded),
                Binary::Glb => mesh_from_glb(&self.decoded),
            }
            .map_err(|error| import_fault(error).at("data"))?;
            self.decoded = Vec::new();
            let triangles = data.indices.len() / 3;
            if triangles > FACE_LIMIT {
                return Err(capacity_fault(("The file", "Die Datei"), triangles, FACE_LIMIT, "data"));
            }
            self.job = Some(HalfedgeMesh::indexed_triangle_job(data.positions, data.indices, data.normals).kernel()?);
            return Ok(Flow::Working(0.35));
        }
        let Some(job) = self.job.as_mut() else { return Ok(Flow::Working(0.35)) };
        match modeling_slice(job, fuel.saturating_mul(BATCH), cx).kernel()? {
            MeshModelingStep::Working(progress) => Ok(Flow::Working(0.35 + 0.65 * ratio(progress))),
            MeshModelingStep::Done(mesh) => Ok(Flow::Done(mesh_output(mesh))),
            MeshModelingStep::Cancelled(_) => Err(semio_s_artifact_procedural_3d_mesh_support::cancelled_fault()),
        }
    }

    fn cancel(&mut self) {
        if let Some(job) = self.job.as_mut() {
            job.cancel();
        }
        self.decoded = Vec::new();
    }
}

fn import_binary(kind: &Kind, inputs: WidgetInputs, format: Binary) -> Box<dyn WidgetJob> {
    run(kind, move || {
        let encoded: Vec<u8> = inputs.text("data")?.bytes().filter(|byte| !byte.is_ascii_whitespace()).collect();
        Ok(Box::new(ImportBinary { format, encoded, decoded: Vec::new(), cursor: 0, job: None }) as Box<dyn Machine>)
    })
}

fn import_stl(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    import_binary(kind, inputs, Binary::Stl)
}

fn import_glb(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    import_binary(kind, inputs, Binary::Glb)
}
//#endregion 🔖️Import

/// 📚️ This facet's catalogue category: its slug and its schema-first definition.
pub const CATEGORY: (&str, &str) = ("mesh-interchange", include_str!("../../../🧬️schema/🗂️catalogue/🔣️mesh-interchange.json"));
/// 🗃️ The `mesh.interchange` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "mesh.interchange.exportObj", start: export_obj },
    ComputeEntry { id: "mesh.interchange.exportJson", start: export_json },
    ComputeEntry { id: "mesh.interchange.exportStl", start: export_stl },
    ComputeEntry { id: "mesh.interchange.exportGlb", start: export_glb },
    ComputeEntry { id: "mesh.interchange.importObj", start: import_obj },
    ComputeEntry { id: "mesh.interchange.importStl", start: import_stl },
    ComputeEntry { id: "mesh.interchange.importGlb", start: import_glb },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
