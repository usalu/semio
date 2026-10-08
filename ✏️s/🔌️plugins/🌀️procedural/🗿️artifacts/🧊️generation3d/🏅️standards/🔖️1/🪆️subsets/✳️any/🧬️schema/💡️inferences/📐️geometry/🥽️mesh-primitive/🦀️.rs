//! 🥽️ `mesh.primitive` computes: box, plane, cylinder, cone, ico sphere, torus, uv sphere and a mesh built from JSON text, all as stepped kernel mesh jobs.
//!
//! Torus and uv sphere are generated as indexed polygon sources by the mesh engine and reconstructed by the kernel's polygon source job.

use super::mesh_support::{capacity_fault, count, from_source, import_fault, modeling, scalar, KernelResult, Parse, BATCH};
use super::super::prelude::*;
use semio_framework_3d::mesh::HalfedgeMesh;
use semio_framework_mesh_engine::{PolygonMeshSource, PolygonSourcePreparation};

const FACE_LIMIT: usize = 100_000;

fn box_(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || HalfedgeMesh::box_primitive_job(scalar(&inputs, "width")?, scalar(&inputs, "height")?, scalar(&inputs, "depth")?).kernel())
}

fn plane(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || HalfedgeMesh::plane_primitive_job(scalar(&inputs, "width")?, scalar(&inputs, "depth")?).kernel())
}

fn sphere(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || HalfedgeMesh::sphere_primitive_job(scalar(&inputs, "radius")?, count(&inputs, "subdivisions")?).kernel())
}

fn cylinder(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || HalfedgeMesh::cylinder_primitive_job(scalar(&inputs, "radius")?, scalar(&inputs, "height")?, count(&inputs, "segments")?).kernel())
}

fn cone(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || HalfedgeMesh::cone_primitive_job(scalar(&inputs, "radius")?, scalar(&inputs, "height")?, count(&inputs, "segments")?).kernel())
}

fn torus(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let (major, minor, segments, rings) = (scalar(&inputs, "major")?, scalar(&inputs, "minor")?, count(&inputs, "segments")?, count(&inputs, "rings")?);
        if minor >= major {
            return Err(WidgetFault::new("generation3d.geometry.torus-self-intersecting", "The tube radius must be smaller than the ring radius, otherwise the torus intersects itself.", "Der Röhrenradius muss kleiner als der Ringradius sein, sonst durchdringt sich der Torus.").at("minor"));
        }
        let faces = segments as usize * rings as usize;
        if faces > FACE_LIMIT {
            return Err(capacity_fault(("The torus", "Der Torus"), faces, FACE_LIMIT, "rings"));
        }
        let source = PolygonMeshSource::torus(major, minor, segments, rings).map_err(import_fault)?;
        HalfedgeMesh::polygon_source_job(source).kernel()
    })
}

fn uv_sphere(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    modeling(kind, move || {
        let (radius, segments, rings) = (scalar(&inputs, "radius")?, count(&inputs, "segments")?, count(&inputs, "rings")?);
        let faces = segments as usize * rings as usize;
        if faces > FACE_LIMIT {
            return Err(capacity_fault(("The sphere", "Die Kugel"), faces, FACE_LIMIT, "rings"));
        }
        let source = PolygonMeshSource::uv_sphere(radius, segments, rings).map_err(import_fault)?;
        HalfedgeMesh::polygon_source_job(source).kernel()
    })
}

fn construct(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    from_source(kind, move || {
        let text = inputs.text("data")?.to_string();
        let mut preparation = PolygonSourcePreparation::new();
        Ok(Box::new(move |fuel: usize| preparation.step(&text, fuel.saturating_mul(BATCH), fuel.saturating_mul(4096)).map_err(import_fault)) as Parse)
    })
}

/// 🗃️ The `mesh.primitive` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "mesh.primitive.construct", start: construct },
    ComputeEntry { id: "mesh.primitive.box", start: box_ },
    ComputeEntry { id: "mesh.primitive.plane", start: plane },
    ComputeEntry { id: "mesh.primitive.sphere", start: sphere },
    ComputeEntry { id: "mesh.primitive.cylinder", start: cylinder },
    ComputeEntry { id: "mesh.primitive.cone", start: cone },
    ComputeEntry { id: "mesh.primitive.torus", start: torus },
    ComputeEntry { id: "mesh.primitive.uvSphere", start: uv_sphere },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
