//! 🔀️ `mesh.convert` computes: a B-Rep shape to a triangle mesh through the shape value's resumable tessellation job, and a mesh to a faceted B-Rep shape through the kernel's retained mesh import.
//!
//! A mesh is turned into B-Rep as triangles: polygons are triangulated first, every triangle becomes a planar face, a closed oriented mesh becomes a solid and an open one a shell. Curved surfaces are not reconstructed.

use super::mesh_support::{cancelled_fault, capacity_fault, mesh_output, ratio, run, Flow, KernelResult, Machine, BATCH};
use super::super::prelude::*;
use semio_framework_3d::brep::engine::{MeshImportCursor, ShapeTessellationJob, ShapeValue};
use semio_framework_3d::brep::queries::tessellation::TessellationStep;
use semio_framework_3d::mesh::{HalfedgeMesh, MeshModelingJob, MeshModelingStep, MeshTessellationJob, MeshTessellationStep};

const FACE_LIMIT: usize = 100_000;

struct FromBrep {
    tessellation: Option<ShapeTessellationJob>,
    job: Option<MeshModelingJob>,
}

impl Machine for FromBrep {
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        if let Some(tessellation) = self.tessellation.as_mut() {
            return match tessellation.step(fuel).map_err(|error| kernel_fault(&error))? {
                TessellationStep::Working(progress) => Ok(Flow::Working(0.5 * progress.units_done as f32 / progress.units_total.max(1) as f32)),
                TessellationStep::Cancelled(_) => Err(cancelled_fault()),
                TessellationStep::Done(_) => {
                    let Some((transfer, _)) = self.tessellation.take().and_then(ShapeTessellationJob::into_mesh) else { return Err(cancelled_fault()) };
                    let triangles = transfer.index.len() / 3;
                    if triangles > FACE_LIMIT {
                        return Err(capacity_fault(("The tessellation", "Die Triangulierung"), triangles, FACE_LIMIT, "deflection"));
                    }
                    self.job = Some(HalfedgeMesh::indexed_triangle_job(transfer.position, transfer.index, Vec::new()).kernel()?);
                    Ok(Flow::Working(0.5))
                }
            };
        }
        let Some(job) = self.job.as_mut() else { return Err(cancelled_fault()) };
        match job.step(fuel.saturating_mul(BATCH)).kernel()? {
            MeshModelingStep::Working(progress) => Ok(Flow::Working(0.5 + 0.5 * ratio(progress))),
            MeshModelingStep::Done(mesh) => Ok(Flow::Done(mesh_output(mesh))),
            MeshModelingStep::Cancelled(_) => Err(cancelled_fault()),
        }
    }

    fn cancel(&mut self) {
        if let Some(tessellation) = self.tessellation.as_mut() {
            tessellation.cancel();
        }
        if let Some(job) = self.job.as_mut() {
            job.cancel();
        }
        self.tessellation = None;
        self.job = None;
    }
}

fn from_brep(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    run(kind, move || {
        let shape: &ShapeValue = inputs.shape("shape")?;
        let tessellation = shape.tessellate_job(inputs.number("deflection")?).map_err(|error| kernel_fault(&error))?;
        Ok(Box::new(FromBrep { tessellation: Some(tessellation), job: None }) as Box<dyn Machine>)
    })
}

struct ToBrep {
    tessellation: Option<MeshTessellationJob>,
    session: KernelSession,
    cursor: Option<MeshImportCursor>,
    tolerance: f64,
}

impl Machine for ToBrep {
    fn advance(&mut self, fuel: usize) -> Result<Flow, WidgetFault> {
        if let Some(job) = self.tessellation.as_mut() {
            return match job.step(fuel.saturating_mul(BATCH)).kernel()? {
                MeshTessellationStep::Working(progress) => Ok(Flow::Working(0.3 * ratio(progress))),
                MeshTessellationStep::Cancelled(_) => Err(cancelled_fault()),
                MeshTessellationStep::Done(transfer) => {
                    self.tessellation = None;
                    MeshImportCursor::validate_admission_counts(transfer.positions.len(), transfer.indices.len(), self.tolerance).map_err(|error| kernel_fault(&error.into()))?;
                    self.cursor = Some(MeshImportCursor::new(transfer.positions, transfer.normals, transfer.indices, self.tolerance).map_err(|error| kernel_fault(&error.into()))?);
                    Ok(Flow::Working(0.3))
                }
            };
        }
        let Some(cursor) = self.cursor.as_mut() else { return Err(cancelled_fault()) };
        match self.session.brep().step_mesh_import_sync(cursor, fuel.saturating_mul(BATCH)).map_err(|error| kernel_fault(&error))? {
            Some(handle) => Ok(Flow::Done(outputs([("shape", GeometryValue::shape(self.session.export(&handle)?))]))),
            None => {
                let (done, total, _) = cursor.progress();
                Ok(Flow::Working(0.3 + 0.7 * done as f32 / total.max(1) as f32))
            }
        }
    }

    fn cancel(&mut self) {
        if let Some(cursor) = self.cursor.as_mut() {
            cursor.cancel();
        }
        self.tessellation = None;
        self.cursor = None;
    }
}

fn to_brep(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    run(kind, move || {
        let mesh = (**inputs.mesh("mesh")?).clone();
        Ok(Box::new(ToBrep { tessellation: Some(MeshTessellationJob::new(mesh)), session: KernelSession::new(), cursor: None, tolerance: inputs.number("tolerance")? }) as Box<dyn Machine>)
    })
}

/// 🗃️ The `mesh.convert` registrations.
pub const COMPUTES: &[ComputeEntry] = &[ComputeEntry { id: "mesh.convert.fromBrep", start: from_brep }, ComputeEntry { id: "mesh.convert.toBrep", start: to_brep }];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
