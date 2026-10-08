//! 📼️ `mesh.interchange` computes: OBJ, JSON, binary STL and binary GLB export (the binary formats as base64 text) and OBJ, STL and GLB import, all through the mesh engine codecs and typed `HalfedgeMesh` values.
//!
//! Writers and readers advance in bounded slices; the format codecs of the mesh engine (STL and GLB encode and decode, JSON encode) run as one unit each.

use super::mesh_support::{capacity_fault, export_fault, from_source, import_fault, mesh_output, ratio, run, Flow, KernelResult, Machine, Parse, BATCH};
use super::super::prelude::*;
use semio_framework_3d::mesh::{HalfedgeMesh, MeshModelingJob, MeshModelingStep, MeshObjExport, MeshTessellationJob, MeshTessellationStep};
use semio_framework_io_base64::{base64_standard_decode, base64_standard_encode};
use semio_framework_mesh_engine::{mesh_from_glb, mesh_from_stl, mesh_to_glb, mesh_to_stl, MeshData, ObjSourceCursor};
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
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
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

struct ExportJson {
    mesh: Arc<HalfedgeMesh>,
    source: Option<semio_framework_mesh_engine::PolygonMeshSource>,
}

impl Machine for ExportJson {
    fn advance(&mut self, _fuel: usize) -> Result<Flow, WidgetFault> {
        match self.source.take() {
            None => {
                self.source = Some(self.mesh.polygon_source().kernel()?);
                Ok(Flow::Working(0.5))
            }
            Some(source) => Ok(Flow::Done(text_output("text", source.encode()))),
        }
    }

    fn cancel(&mut self) {
        self.source = None;
    }
}

fn export_json(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    run(kind, move || Ok(Box::new(ExportJson { mesh: inputs.mesh("mesh")?.clone(), source: None }) as Box<dyn Machine>))
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
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        if let Some(job) = self.tessellation.as_mut() {
            return match job.step(fuel.saturating_mul(BATCH)).kernel()? {
                MeshTessellationStep::Working(progress) => Ok(Flow::Working(0.4 * ratio(progress))),
                MeshTessellationStep::Cancelled(_) => Err(super::mesh_support::cancelled_fault()),
                MeshTessellationStep::Done(transfer) => {
                    self.tessellation = None;
                    let data = MeshData { positions: transfer.positions, normals: transfer.normals, colors: transfer.colors, indices: transfer.indices, uvs: transfer.uvs, ..MeshData::default() };
                    self.bytes = match self.format {
                        Binary::Stl => mesh_to_stl(&data),
                        Binary::Glb => mesh_to_glb(&data),
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
        Ok(Box::new(move |fuel: usize| cursor.step(&text, fuel.saturating_mul(BATCH * 16)).map_err(import_fault)) as Parse)
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
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        if self.job.is_none() {
            if self.cursor < self.encoded.len() {
                let end = (self.cursor + DECODE_CHUNK.saturating_mul(fuel)).min(self.encoded.len());
                self.decoded.extend(base64_standard_decode(&self.encoded[self.cursor..end]).map_err(import_fault)?);
                self.cursor = end;
                return Ok(Flow::Working(0.3 * self.cursor as f32 / self.encoded.len().max(1) as f32));
            }
            let data = match self.format {
                Binary::Stl => mesh_from_stl(&self.decoded),
                Binary::Glb => mesh_from_glb(&self.decoded),
            }
            .map_err(import_fault)?;
            self.decoded = Vec::new();
            let triangles = data.indices.len() / 3;
            if triangles > FACE_LIMIT {
                return Err(capacity_fault(("The file", "Die Datei"), triangles, FACE_LIMIT, "data"));
            }
            self.job = Some(HalfedgeMesh::indexed_triangle_job(data.positions, data.indices, data.normals).kernel()?);
            return Ok(Flow::Working(0.35));
        }
        let Some(job) = self.job.as_mut() else { return Ok(Flow::Working(0.35)) };
        match job.step(fuel.saturating_mul(BATCH)).kernel()? {
            MeshModelingStep::Working(progress) => Ok(Flow::Working(0.35 + 0.65 * ratio(progress))),
            MeshModelingStep::Done(mesh) => Ok(Flow::Done(mesh_output(mesh))),
            MeshModelingStep::Cancelled(_) => Err(super::mesh_support::cancelled_fault()),
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
