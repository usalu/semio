//! ✅️ The analysis.check widget computes: validity and watertightness of a shape, its topology counts, the interference of two solids and the quality of a mesh.
//!
//! A check answers with facts, never with a repaired shape: every finding the kernel reports is carried in the `report` text. The interference
//! of two solids runs the kernel's exact intersection as a stepped job (one operand pair at a time, the boolean advanced with the fuel it is
//! granted); the other checks are staged units over the shape's own arena, so no check touches a kernel session it does not need.
//!
//! 🔗️ [Euler characteristic](https://en.wikipedia.org/wiki/Euler_characteristic) · [Manifold](https://en.wikipedia.org/wiki/Manifold) · [Watertight mesh](https://en.wikipedia.org/wiki/Polygon_mesh)

use super::analysis_measure::{cancelled, lost_state, query_fault, scope_of, staged};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;
use semio_framework_3d::brep::engine::{BrepBooleanAdmission, BrepBooleanJob, BrepBooleanStep, GeometryHandle, ShapeRoot, ShapeValue};
use semio_framework_3d::brep::operations::boolean::BooleanOp;
use semio_framework_3d::brep::queries::analysis::{manifold_report, topology_counts, validity, IssueSeverity, ManifoldReport, ShapeScope, ValidityReport, Watertightness};
use semio_framework_3d::mesh::{HalfedgeMesh, MeshQualityReport};
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

const OVERLAP_FLOOR: f64 = 1e-9;

//#region 🔖️Health
struct Health {
    shape: Arc<ShapeValue>,
    scope: ShapeScope,
    validity: Option<ValidityReport>,
    manifold: Option<ManifoldReport>,
}

/// 🏷️ The kebab-case name of a watertightness verdict.
fn verdict_name(verdict: Watertightness) -> &'static str {
    match verdict {
        Watertightness::Watertight => "watertight",
        Watertightness::Open => "open",
        Watertightness::NonManifold => "non-manifold",
        Watertightness::Misoriented => "misoriented",
    }
}

/// 📝️ The report text: one line per kernel finding (`severity code entity: message`), then the watertightness verdict.
fn describe(report: &ValidityReport, manifold: &ManifoldReport) -> String {
    let mut lines: Vec<String> = report
        .issues
        .iter()
        .map(|issue| format!("{} {} {}: {}", if issue.severity == IssueSeverity::Error { "error" } else { "warning" }, issue.code, issue.entity, issue.message))
        .collect();
    lines.push(format!("verdict {}", verdict_name(manifold.verdict)));
    lines.join("\n")
}

/// 🩺️ Validity and watertightness of a shape: the kernel's structural and geometric checks (one unit), then the edge-use census (one unit).
fn validity_compute(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = (|| {
        let shape = inputs.shape("shape")?.clone();
        let scope = scope_of(&shape, "shape")?;
        Ok((Health { shape, scope, validity: None, manifold: None }, 2))
    })();
    staged(
        kind,
        prepared,
        |state, index| {
            if index == 0 {
                state.validity = Some(validity(&state.shape.body, &state.scope).map_err(query_fault)?);
            } else {
                state.manifold = Some(manifold_report(&state.shape.body, &state.scope).map_err(query_fault)?);
            }
            Ok(())
        },
        |state, quality| {
            let (Some(report), Some(manifold)) = (state.validity, state.manifold) else { return Err(lost_state()) };
            Ok((
                outputs([
                    ("valid", GeometryValue::Boolean(report.ok)),
                    ("watertight", GeometryValue::Boolean(manifold.verdict == Watertightness::Watertight)),
                    ("report", GeometryValue::Text(describe(&report, &manifold))),
                ]),
                quality,
            ))
        },
    )
}
//#endregion 🔖️Health

//#region 🔖️Topology
/// 🔢️ Vertex, edge, face, shell and solid counts and the Euler characteristic; edges that collapse to a point (a sphere's poles) are not counted, so `V - E + F` matches the characteristic of every shape without inner loops.
fn topology_counts_compute(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let shape = inputs.shape("shape")?;
            let counts = topology_counts(&shape.body, &scope_of(shape, "shape")?).map_err(query_fault)?;
            let integer = |count: usize| GeometryValue::Integer(count as i64);
            Ok(outputs([
                ("vertices", integer(counts.vertices)),
                ("edges", integer(counts.edges - counts.degenerate_edges)),
                ("faces", integer(counts.faces)),
                ("shells", integer(counts.shells)),
                ("solids", integer(counts.solids)),
                ("euler", GeometryValue::Integer(counts.euler_characteristic)),
            ]))
        })(),
    )
}
//#endregion 🔖️Topology


//#region 🔖️Interference
enum Stage {
    Import,
    Pairs { pairs: Vec<(GeometryHandle, GeometryHandle)>, next: usize, running: Option<BrepBooleanJob>, results: Vec<GeometryHandle>, floor: f64 },
    Conclude { results: Vec<GeometryHandle>, floor: f64 },
    Gone,
}

/// 🪜️ What one advance of the interference job reports: continue with the next stage, wait for more fuel in this stage, or the finished outputs.
enum Advance {
    Continue(Stage),
    Pending(Stage, f32),
    Finished(Outputs),
}

struct Interference {
    quality: Quality,
    first: Arc<ShapeValue>,
    second: Arc<ShapeValue>,
    session: KernelSession,
    stage: Stage,
    finished: Option<WidgetEvaluation>,
}

/// 🔎️ Whether the kernel refused an intersection because the operands share no volume.
fn is_empty_intersection(fault: &WidgetFault) -> bool {
    fault.message.en.contains("intersect is empty")
}

fn panicked() -> WidgetFault {
    WidgetFault::new("generation3d.geometry.kernel", "The geometry kernel stopped unexpectedly while computing this widget.", "Der Geometriekern wurde beim Berechnen dieses Widgets unerwartet beendet.")
}

impl Interference {
    fn end(&mut self, result: Result<Outputs, WidgetFault>) -> WidgetStep {
        self.stage = Stage::Gone;
        let evaluation = match result {
            Ok(outputs) => WidgetEvaluation::ok(outputs, self.quality),
            Err(fault) => WidgetEvaluation::faulted(fault, self.quality),
        };
        self.finished = Some(evaluation.clone());
        WidgetStep::Done(evaluation)
    }

    /// 🧱️ The solid handles of an imported operand: the solid itself, or the members of a compound.
    fn solids(&mut self, source: &ShapeValue, handle: &GeometryHandle) -> Result<Vec<GeometryHandle>, WidgetFault> {
        match source.root {
            ShapeRoot::Compound { .. } => self.session.brep().explode_sync(handle).map_err(|error| kernel_fault(&error)),
            _ => Ok(vec![handle.clone()]),
        }
    }

    fn volume(&mut self, handle: &GeometryHandle) -> Result<f64, WidgetFault> {
        self.session.brep().volume_sync(handle).map_err(|error| kernel_fault(&error))
    }

    /// 📥️ Imports both operands and lists every solid pair to intersect; the overlap floor is relative to the smaller operand.
    fn import(&mut self) -> Result<Stage, WidgetFault> {
        let (a, b) = (self.first.clone(), self.second.clone());
        let (first, second) = (self.session.import(&a)?.handle, self.session.import(&b)?.handle);
        let floor = OVERLAP_FLOOR * self.volume(&first)?.min(self.volume(&second)?);
        let (left, right) = (self.solids(&a, &first)?, self.solids(&b, &second)?);
        let pairs = left.iter().flat_map(|l| right.iter().map(move |r| (l.clone(), r.clone()))).collect();
        Ok(Stage::Pairs { pairs, next: 0, running: None, results: Vec::new(), floor })
    }

    /// 🔀️ Starts or advances the intersection of the next pair with `grant` units; a pair with no common volume counts as done.
    fn intersect(&mut self, pairs: Vec<(GeometryHandle, GeometryHandle)>, mut next: usize, mut running: Option<BrepBooleanJob>, mut results: Vec<GeometryHandle>, floor: f64, grant: usize) -> Result<Advance, WidgetFault> {
        let Some((a, b)) = pairs.get(next).cloned() else { return Ok(Advance::Continue(Stage::Conclude { results, floor })) };
        let outcome = match running.take() {
            Some(mut job) => match self.session.brep().step_boolean_job_sync(&mut job, grant).map_err(|error| kernel_fault(&error)) {
                Ok(BrepBooleanStep::Working(progress)) => {
                    let fraction = progress.units_done as f32 / progress.units_total.max(1) as f32;
                    let stage = Stage::Pairs { pairs, next, running: Some(job), results, floor };
                    return Ok(Advance::Pending(stage, (next as f32 + fraction.min(1.0)) / (next + 1) as f32));
                }
                Ok(BrepBooleanStep::Ready(handle)) => Ok(Some(handle)),
                Ok(BrepBooleanStep::Cancelled(_)) => return Err(cancelled()),
                Err(fault) => Err(fault),
            },
            None => match self.session.brep().boolean_job_sync(&a, &b, BooleanOp::Intersect).map_err(|error| kernel_fault(&error)) {
                Ok(BrepBooleanAdmission::Answered(handle)) => Ok(Some(handle)),
                Ok(BrepBooleanAdmission::Job(job)) => {
                    return Ok(Advance::Continue(Stage::Pairs { pairs, next, running: Some(job), results, floor }));
                }
                Err(fault) => Err(fault),
            },
        };
        match outcome {
            Ok(Some(handle)) => results.push(handle),
            Ok(None) => {}
            Err(fault) if is_empty_intersection(&fault) => {}
            Err(fault) => return Err(fault),
        }
        next += 1;
        Ok(Advance::Continue(Stage::Pairs { pairs, next, running: None, results, floor }))
    }

    /// 🏁️ Measures the intersection volume and exports the overlap when it exceeds the floor.
    fn conclude(&mut self, results: Vec<GeometryHandle>, floor: f64) -> Result<Outputs, WidgetFault> {
        let mut volume = 0.0;
        for handle in &results {
            volume += self.volume(handle)?;
        }
        let interferes = !results.is_empty() && volume > floor;
        let mut found = outputs([("interferes", GeometryValue::Boolean(interferes)), ("volume", GeometryValue::Number(if interferes { volume } else { 0.0 }))]);
        if interferes {
            let handle = match results.as_slice() {
                [single] => single.clone(),
                many => self.session.brep().compound_sync(many).map_err(|error| kernel_fault(&error))?,
            };
            found.insert("shape".to_string(), GeometryValue::shape(self.session.export(&handle)?));
        }
        Ok(found)
    }

    fn advance(&mut self, stage: Stage, grant: usize) -> Result<Advance, WidgetFault> {
        match stage {
            Stage::Import => self.import().map(Advance::Continue),
            Stage::Pairs { pairs, next, running, results, floor } => self.intersect(pairs, next, running, results, floor, grant),
            Stage::Conclude { results, floor } => self.conclude(results, floor).map(Advance::Finished),
            Stage::Gone => Err(lost_state()),
        }
    }
}

impl WidgetJob for Interference {
    fn step(&mut self, fuel: usize) -> WidgetStep {
        if let Some(evaluation) = &self.finished {
            return WidgetStep::Done(evaluation.clone());
        }
        let mut budget = fuel.max(1);
        loop {
            let stage = std::mem::replace(&mut self.stage, Stage::Gone);
            let outcome = catch_unwind(AssertUnwindSafe(|| self.advance(stage, budget))).unwrap_or_else(|_| Err(panicked()));
            match outcome {
                Err(fault) => return self.end(Err(fault)),
                Ok(Advance::Finished(found)) => return self.end(Ok(found)),
                Ok(Advance::Pending(stage, progress)) => {
                    self.stage = stage;
                    return WidgetStep::Working { progress: progress.clamp(0.0, 0.999) };
                }
                Ok(Advance::Continue(stage)) => {
                    self.stage = stage;
                    budget -= 1;
                    if budget == 0 {
                        return WidgetStep::Working { progress: 0.0 };
                    }
                }
            }
        }
    }

    fn cancel(&mut self) {
        if self.finished.is_none() {
            if let Stage::Pairs { running: Some(job), .. } = &mut self.stage {
                job.cancel();
            }
            self.end(Err(cancelled()));
        }
    }
}

/// 💥️ Whether two solids (or compounds of solids) overlap: the exact intersection runs pair by pair as a stepped job, the volume decides.
fn interference(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    match (inputs.shape("a"), inputs.shape("b")) {
        (Ok(first), Ok(second)) => Box::new(Interference { quality: kind.quality, first: first.clone(), second: second.clone(), session: KernelSession::new(), stage: Stage::Import, finished: None }),
        (Err(fault), _) | (_, Err(fault)) => failed(fault, kind.quality),
    }
}
//#endregion 🔖️Interference

//#region 🔖️Mesh
/// 🔁️ How many undirected edges are used by two faces in the same direction, which no consistently wound surface does.
fn inconsistent_edges(polygons: &[Vec<u32>]) -> usize {
    let mut uses: BTreeMap<(u32, u32), (u32, u32)> = BTreeMap::new();
    for polygon in polygons {
        for (corner, &from) in polygon.iter().enumerate() {
            let to = polygon[(corner + 1) % polygon.len()];
            if from == to {
                continue;
            }
            let entry = uses.entry((from.min(to), from.max(to))).or_default();
            if from < to {
                entry.0 += 1;
            } else {
                entry.1 += 1;
            }
        }
    }
    uses.values().filter(|(forward, backward)| forward + backward == 2 && (*forward == 2 || *backward == 2)).count()
}

struct MeshHealth {
    mesh: Arc<HalfedgeMesh>,
    report: Option<MeshQualityReport>,
    inconsistent: usize,
}

fn mesh_empty() -> WidgetFault {
    WidgetFault::new("generation3d.geometry.mesh-empty", "The mesh has no vertices to measure.", "Das Netz hat keine Eckpunkte zum Messen.").at("mesh")
}

/// 🪞️ Element counts, open, non-manifold, inconsistent and degenerate parts, area, volume and bounds of a mesh: the report (one unit), then the winding census (one unit).
fn mesh_quality(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    let prepared = inputs.mesh("mesh").map(|mesh| (MeshHealth { mesh: mesh.clone(), report: None, inconsistent: 0 }, 2));
    staged(
        kind,
        prepared,
        |state, index| {
            if index == 0 {
                state.report = Some(state.mesh.quality_report());
            } else {
                state.inconsistent = inconsistent_edges(&state.mesh.polygons());
            }
            Ok(())
        },
        |state, quality| {
            let report = state.report.ok_or_else(lost_state)?;
            let bounds = report.bounding_box.ok_or_else(mesh_empty)?;
            let integer = |count: usize| GeometryValue::Integer(count as i64);
            let mut found = outputs([
                ("vertices", integer(report.vertex_count)),
                ("faces", integer(report.face_count)),
                ("edges", integer(report.edge_count)),
                ("triangles", integer(report.triangle_count)),
                ("boundaryEdges", integer(report.boundary_edges)),
                ("nonManifoldEdges", integer(report.non_manifold_edges)),
                ("inconsistentEdges", integer(state.inconsistent)),
                ("degenerateTriangles", integer(report.degenerate_faces)),
                ("closed", GeometryValue::Boolean(report.closed)),
                ("area", GeometryValue::Number(report.area)),
                ("minimum", GeometryValue::Point(bounds.min)),
                ("maximum", GeometryValue::Point(bounds.max)),
            ]);
            if let Some(mass) = report.mass {
                found.insert("volume".to_string(), GeometryValue::Number(mass.volume));
            }
            Ok((found, quality))
        },
    )
}
//#endregion 🔖️Mesh

/// 🗃️ Every analysis.check catalogue kind and the compute that starts it.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "analysis.validity", start: validity_compute },
    ComputeEntry { id: "analysis.topologyCounts", start: topology_counts_compute },
    ComputeEntry { id: "analysis.interference", start: interference },
    ComputeEntry { id: "analysis.meshQuality", start: mesh_quality },
];

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
