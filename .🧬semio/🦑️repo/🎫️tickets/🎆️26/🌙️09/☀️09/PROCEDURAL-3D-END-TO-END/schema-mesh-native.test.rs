extern crate self as semio_framework_3d;
extern crate self as semio_s_artifact_procedural_3d_geometry_compute;
extern crate self as semio_s_artifact_procedural_3d_geometry_inputs;
extern crate self as semio_s_artifact_procedural_3d_geometry_value;
extern crate self as semio_s_artifact_procedural_3d_model;

pub use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::retained_clone::RetainedCloneProgress;
use serde_json::Value;
use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

pub mod mesh { pub use semio_bevel_modeling::*; }
pub mod catalogue { pub use super::Kind; }
use mesh::{EdgeId, HalfedgeMesh, MeshKernelError, MeshModelingJob, MeshModelingOps, MeshModelingProgress, MeshModelingStep, MeshSurfaceJob, MeshTessellationJob, MeshTessellationStep};

#[derive(Clone, Copy)]
pub enum Quality { PolygonMesh }

pub struct Kind { pub quality: Quality }

#[derive(Debug, PartialEq)]
pub struct WidgetFault { pub code: String, pub port: Option<String>, pub detail: String }
impl WidgetFault {
    pub fn new(code: &str, en: impl Into<String>, _de: impl Into<String>) -> Self { Self { code: code.into(), port: None, detail: en.into() } }
    pub fn at(self, port: &str) -> Self { Self { port: Some(port.into()), ..self } }
}

pub enum SelectionKind { Vertex, Edge, Face }
pub struct Selection { pub ids: Vec<u64> }
pub struct WidgetInputs { pub meshes: BTreeMap<String, Arc<HalfedgeMesh>>, pub selections: BTreeMap<String, Selection> }
impl WidgetInputs {
    pub fn mesh(&self, name: &str) -> Result<&Arc<HalfedgeMesh>, WidgetFault> {
        self.meshes.get(name).ok_or_else(|| WidgetFault::new("input", "", "").at(name))
    }
    pub fn selection(&self, name: &str) -> Result<&Selection, WidgetFault> {
        self.selections.get(name).ok_or_else(|| WidgetFault::new("input", "", "").at(name))
    }
}

pub type Outputs = HalfedgeMesh;

pub struct WidgetEvaluation { pub mesh: Option<HalfedgeMesh>, pub fault: Option<WidgetFault> }
impl WidgetEvaluation {
    pub fn faulted(fault: WidgetFault, _quality: Quality) -> Self { Self { mesh: None, fault: Some(fault) } }
    pub fn ok(mesh: HalfedgeMesh, _quality: Quality) -> Self { Self { mesh: Some(mesh), fault: None } }
}

pub enum WidgetStep { Working { progress: f32 }, Done(WidgetEvaluation) }
pub trait WidgetJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> Result<Option<WidgetStep>, OriginalTurnFailure>;
    fn cancel(&mut self);
}

fn mesh_output(mesh: HalfedgeMesh) -> Outputs { mesh }

fn fixture() -> Value {
    serde_json::from_str(&std::fs::read_to_string(std::env::var("SEMIO_MESH_EDIT_FIXTURE").unwrap()).unwrap()).unwrap()
}

fn fixture_inputs(case: &Value) -> WidgetInputs {
    let mut inputs = WidgetInputs { meshes: BTreeMap::new(), selections: BTreeMap::new() };
    for (name, recipe) in case["inputs"].as_object().unwrap() {
        if recipe.is_array() {
            inputs.selections.insert(name.clone(), Selection { ids: serde_json::from_value(recipe.clone()).unwrap() });
        } else {
            let vertices: Vec<[f32; 3]> = serde_json::from_value(recipe["positions"].clone()).unwrap();
            let faces: Vec<Vec<u32>> = serde_json::from_value(recipe["faces"].clone()).unwrap();
            inputs.meshes.insert(name.clone(), Arc::new(HalfedgeMesh::from_faces(&vertices, &faces).unwrap()));
        }
    }
    inputs
}

fn job(case: &Value) -> Box<dyn WidgetJob> {
    let kind = Kind { quality: Quality::PolygonMesh };
    let inputs = fixture_inputs(case);
    match case["kind"].as_str().unwrap() {
        "mesh.edit.bridge" => bridge(&kind, inputs),
        "mesh.edit.combine" => combine::combine(&kind, inputs),
        "mesh.edit.intersect" => combine::intersect(&kind, inputs),
        "mesh.edit.difference" => combine::difference(&kind, inputs),
        "mesh.edit.union" => combine::union(&kind, inputs),
        _ => panic!("unhandled fixture operation"),
    }
}

fn finish(job: &mut dyn WidgetJob, fuel: usize, label: &Value) -> WidgetEvaluation {
    let mut previous = 0.0;
    for _ in 0..65_536 {
        match testing::step_with_fuel(job, fuel) {
            WidgetStep::Working { progress } => {
                assert!(progress >= previous, "progress went backward from {previous} to {progress}");
                previous = progress;
            }
            WidgetStep::Done(result) => return result,
        }
    }
    panic!("{label} did not finish under fuel {fuel} at progress {previous}");
}

fn assert_mesh_summary(mesh: &HalfedgeMesh, expected: &Value, label: &Value) {
    let positions = mesh.positions();
    let polygons = mesh.polygons();
    assert_eq!(positions.len() as u64, expected["vertices"].as_u64().unwrap(), "{label}");
    assert_eq!(polygons.len() as u64, expected["faces"].as_u64().unwrap(), "{label}");
    let edges: std::collections::BTreeSet<_> = polygons.iter().flat_map(|ring| {
        ring.iter().zip(ring.iter().cycle().skip(1)).map(|(&a, &b)| (a.min(b), a.max(b)))
    }).collect();
    assert_eq!(edges.len() as u64, expected["edges"].as_u64().unwrap(), "{label}");
    if positions.is_empty() {
        assert_eq!(expected["area"].as_f64(), Some(0.0));
        assert_eq!(expected["volume"].as_f64(), Some(0.0));
        return;
    }
    assert!(mesh.halfedges.iter().all(|edge| edge.twin.is_some()), "{label}");
    assert_eq!((positions.len() + polygons.len() - edges.len()) as u64, expected["euler"].as_u64().unwrap(), "{label}");
    let mut bounds = [positions[0]; 2];
    for position in &positions {
        for (axis, coordinate) in position.iter().enumerate() {
            bounds[0][axis] = bounds[0][axis].min(*coordinate);
            bounds[1][axis] = bounds[1][axis].max(*coordinate);
        }
    }
    let expected_bounds: [[f32; 3]; 2] = serde_json::from_value(expected["bbox"].clone()).unwrap();
    for (actual, expected) in bounds.into_iter().flatten().zip(expected_bounds.into_iter().flatten()) {
        assert!((actual - expected).abs() < 1e-6, "{label} bounds");
    }
    let tessellation = mesh.tessellate().unwrap();
    let mut area = 0.0;
    let mut volume = 0.0;
    for triangle in tessellation.indices.chunks_exact(3) {
        let point = |id: u32| {
            let i = id as usize * 3;
            mesh::Vec3::new(tessellation.positions[i], tessellation.positions[i + 1], tessellation.positions[i + 2])
        };
        let (a, b, c) = (point(triangle[0]), point(triangle[1]), point(triangle[2]));
        let normal = b.sub(a).cross(c.sub(a));
        area += f64::from(normal.dot(normal)).sqrt() / 2.0;
        volume += f64::from(a.dot(b.cross(c))) / 6.0;
    }
    assert!((area - expected["area"].as_f64().unwrap()).abs() < 1e-6, "{label} area {area}");
    assert!((volume - expected["volume"].as_f64().unwrap()).abs() < 1e-6, "{label} volume {volume}");
}

#[test]
fn schema_mesh_fixtures_finish_with_identical_topology_at_all_fuel_sizes() {
    let test = fixture();
    let cases = test["cases"].as_array().unwrap().iter().filter(|case| matches!(case["kind"].as_str(), Some("mesh.edit.bridge" | "mesh.edit.combine" | "mesh.edit.intersect" | "mesh.edit.difference" | "mesh.edit.union")));
    let mut seen = 0;
    for case in cases {
        let mut first_topology = None;
        for fuel in [1, 7, 1024] {
            let result = finish(&mut *job(case), fuel, &case["name"]);
            if let Some(fault) = result.fault {
                assert!(result.mesh.is_none());
                assert_eq!(Some(fault.code.as_str()), case["fault"]["code"].as_str(), "{} at fuel {fuel}: {}", case["name"], fault.detail);
                assert_eq!(fault.port.as_deref(), case["fault"]["port"].as_str());
            } else {
                let mesh = result.mesh.unwrap();
                let expected = &case["outputs"]["mesh"];
                let topology = (mesh.positions(), mesh.polygons());
                if let Some(prior) = &first_topology {
                    assert_eq!(&topology, prior, "{}", case["name"]);
                }
                if matches!(case["kind"].as_str(), Some("mesh.edit.intersect" | "mesh.edit.difference" | "mesh.edit.union")) {
                    assert_mesh_summary(&mesh, expected, &case["name"]);
                } else {
                    let positions: Vec<[f32; 3]> = serde_json::from_value(expected["positions"].clone()).unwrap();
                    let polygons: Vec<Vec<u32>> = serde_json::from_value(expected["polygons"].clone()).unwrap();
                    assert_eq!(topology, (positions, polygons), "{}", case["name"]);
                }
                first_topology = Some(topology);
            }
        }
        seen += 1;
    }
    assert_eq!(seen, 33);
}

#[test]
fn capture_and_production_can_be_cancelled_without_publishing_meshes() {
    let fixture = fixture();
    for id in ["mesh.edit.combine", "mesh.edit.intersect", "mesh.edit.difference", "mesh.edit.union"] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["kind"] == id).unwrap();
        for target_progress in [0.0, 0.2] {
            let mut job = job(case);
            let mut reached = false;
            for _ in 0..1024 {
                let WidgetStep::Working { progress } = testing::step_with_fuel(&mut *job, 1) else { panic!("{id} published before cancellation") };
                if progress > target_progress { reached = true; break; }
            }
            assert!(reached, "{id}");
            job.cancel();
            let result = finish(&mut *job, 1, &case["name"]);
            assert!(result.mesh.is_none());
            assert_eq!(result.fault.unwrap().code, CANCELLED);
        }
    }
}

#[test]
fn advanced_mesh_difference_cancels_during_authored_projection() {
    let law = fixture();
    for case in law["cases"].as_array().unwrap().iter().filter(|case| case.get("cancelAfter").is_some()) {
        let threshold = case["cancelAfter"].as_f64().unwrap() as f32;
        let mut job = job(case);
        let mut reached = false;
        for _ in 0..65_536 {
            let WidgetStep::Working { progress } = testing::step_with_fuel(&mut *job, 1) else {
                panic!("{} published before projection could be cancelled", case["name"]);
            };
            if progress > threshold {
                assert!(progress < 0.6, "{} arrived in import", case["name"]);
                reached = true;
                break;
            }
        }
        assert!(reached, "{}", case["name"]);
        job.cancel();
        let result = finish(&mut *job, 1, &case["name"]);
        assert!(result.mesh.is_none());
        assert_eq!(result.fault.unwrap().code, CANCELLED);
    }
}


#[test]
fn schema_mesh_difference_publishes_every_disconnected_shell_at_all_fuels(){
    let fixture=compound::fixture();
    for row in fixture["cases"].as_array().unwrap(){
        let [first,second]=compound::operands(&fixture["faces"],row,"positions");
        let case=serde_json::json!({"name":row["name"],"kind":"mesh.edit.difference","inputs":{"first":first,"second":second}});
        let mut prior=None;
        for fuel in [1,7,1024]{
            let mesh=finish(&mut *job(&case),fuel,&row["name"]).mesh.unwrap();
            let next=(mesh.positions(),mesh.polygons());
            if let Some(prior)=&prior{assert_eq!(&next,prior);}
            compound::verify(&next.0,&next.1,row);
            prior=Some(next);
        }
    }
}


#[test]
fn schema_mesh_union_sews_both_orders_of_placed_stock_at_all_fuels(){
    let fixture=compound::fixture_named("🧩️placed-union.json");
    for row in fixture["cases"].as_array().unwrap(){
        let sources=compound::operands(&fixture["faces"],row,"positions");
        for order in [[0,1],[1,0]]{
            let case=serde_json::json!({"name":row["name"],"kind":"mesh.edit.union",
                "inputs":{"first":sources[order[0]],"second":sources[order[1]]}});
            let mut prior=None;
            for fuel in [1,7,1024]{
                let mesh=finish(&mut *job(&case),fuel,&row["name"]).mesh.unwrap();
                let next=(mesh.positions(),mesh.polygons());
                if let Some(prior)=&prior{assert_eq!(&next,prior);}
                compound::verify(&next.0,&next.1,row);
                prior=Some(next);
            }
        }
    }
}

#[test]
fn schema_mesh_handles_the_contained_concave_cutter_at_all_fuels(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let sources=compound::prism_operands(&law,"positions");
    for (operation,expected) in [("difference",&law),("intersect",&law["intersection"]),("union",&law["union"])]{
        for order in [[0,1],[1,0]]{
            let label=serde_json::json!("contained concave prism");
            let case=serde_json::json!({"name":label,"kind":format!("mesh.edit.{operation}"),
                "inputs":{"first":sources[order[0]],"second":sources[order[1]]}});
            let mut prior=None;
            for fuel in [1,7,1024]{
                let mesh=finish(&mut *job(&case),fuel,&label).mesh.unwrap();
                let next=(mesh.positions(),mesh.polygons());
                if let Some(prior)=&prior{assert_eq!(&next,prior);}
                if operation=="difference"&&order[0]==1{assert!(next.1.is_empty());}
                else{compound::verify(&next.0,&next.1,expected);}
                prior=Some(next);
            }
        }
    }
}

#[test]
fn schema_mesh_intersects_a_prism_transversely_at_all_fuels(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let [_,prism]=compound::prism_operands(&law,"positions");
    let slab=compound::prism_enclosure(&law,"clipped","positions");
    let label=serde_json::json!("transverse concave prism");
    for [a,b] in [[&slab,&prism],[&prism,&slab]]{
        let case=serde_json::json!({"name":label,"kind":"mesh.edit.intersect","inputs":{"first":a,"second":b}});
        let mut prior=None;
        for fuel in [1,7,1024]{
            let mesh=finish(&mut *job(&case),fuel,&label).mesh.unwrap();
            let next=(mesh.positions(),mesh.polygons());
            if let Some(prior)=&prior{assert_eq!(&next,prior);}
            compound::verify(&next.0,&next.1,&law["clipped"]);
            prior=Some(next);
        }
    }
}

#[test]
fn schema_mesh_difference_splits_a_concave_prism_at_all_fuels(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let [_,prism]=compound::prism_operands(&law,"positions");
    let slab=compound::prism_enclosure(&law,"split","positions");
    let label=serde_json::json!("split concave prism");
    let request=serde_json::json!({"name":label,"kind":"mesh.edit.difference","inputs":{"first":prism,"second":slab}});
    let mut prior=None;
    for fuel in [1,7,1024]{
        let mesh=finish(&mut *job(&request),fuel,&label).mesh.unwrap();
        let next=(mesh.positions(),mesh.polygons());
        if let Some(prior)=&prior{assert_eq!(&next,prior);}
        compound::verify(&next.0,&next.1,&law["split"]);
        prior=Some(next);
    }
}

#[test]
fn schema_mesh_clips_the_prism_along_one_original_wall_at_all_fuels(){
    let law=compound::fixture_named("🕳️nested-prismatic-difference.json");
    let [_,prism]=compound::prism_operands(&law,"positions");
    let wall=compound::prism_enclosure(&law,"lateral","positions");
    let label=serde_json::json!("longitudinal concave prism");
    for (a,b,operation,expected) in [(&prism,&wall,"intersect","intersection"),
        (&wall,&prism,"intersect","intersection"),(&prism,&wall,"difference","difference")]{
        let request=serde_json::json!({"name":label,"kind":format!("mesh.edit.{operation}"),"inputs":{"first":a,"second":b}});
        let mut prior=None;
        for fuel in [1,7,1024]{
            let mesh=finish(&mut *job(&request),fuel,&label).mesh.unwrap();
            let next=(mesh.positions(),mesh.polygons());
            if let Some(prior)=&prior{assert_eq!(&next,prior);}
            compound::verify(&next.0,&next.1,&law["lateral"][expected]);
            prior=Some(next);
        }
    }
}

