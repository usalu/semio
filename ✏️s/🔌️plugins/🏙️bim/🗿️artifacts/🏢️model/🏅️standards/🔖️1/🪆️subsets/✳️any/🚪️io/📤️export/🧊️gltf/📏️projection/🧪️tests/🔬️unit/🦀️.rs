use super::*;
use crate::standards::v1::subsets::any::io::export::gltf::scene::y_up;
use crate::standards::v1::subsets::any::io::export::gltf::testkit::{house, read};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::compute_element_solids;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;

fn build(snapshot: &crate::ModelSnapshot) -> (GltfModel, Vec<String>) {
    inference::with_inference(None, snapshot, |inferred| crate::standards::v1::subsets::any::io::export::gltf::scene::build(snapshot, inferred))
}

#[test]
fn the_report_counts_what_the_scene_holds() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    let solids = compute_element_solids(&snapshot);
    let report = project(&model);
    assert_eq!(report.nodes, model.nodes.len());
    assert_eq!(report.meshes, solids.len());
    assert_eq!(report.triangles, solids.values().map(|solid| solid.triangle_count()).sum::<usize>());
    assert_eq!(report.primitives, model.meshes.iter().map(|mesh| mesh.primitives.len()).sum::<usize>());
    assert_eq!(report.materials, model.materials.len());
    assert_eq!(report.kinds.values().sum::<usize>(), solids.len());
    assert_eq!(report.storeys.values().map(|count| count.elements).sum::<usize>(), solids.len());
    assert_eq!(report.storeys.values().map(|count| count.triangles).sum::<usize>(), report.triangles);
    assert_eq!(report.storeys.keys().map(String::as_str).collect::<Vec<_>>(), snapshot.storeys.keys().filter(|storey| solids.values().any(|solid| solid.storey == **storey)).map(String::as_str).collect::<Vec<_>>());
}

#[test]
fn the_world_bounds_are_the_bounds_of_the_model_solids_in_the_y_up_frame() {
    let snapshot = house();
    let (model, _) = build(&snapshot);
    let report = project(&model);
    let (mut min, mut max) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for solid in compute_element_solids(&snapshot).values() {
        let (sin, cos) = solid.placement.rotation.sin_cos();
        for corner in solid.indices.iter().map(|corner| &solid.positions[*corner as usize * 3..*corner as usize * 3 + 3]) {
            let world = y_up(solid.placement.x + cos * corner[0] - sin * corner[1], solid.placement.y + sin * corner[0] + cos * corner[1], solid.placement.z + corner[2]);
            for axis in 0..3 {
                min[axis] = min[axis].min(world[axis]);
                max[axis] = max[axis].max(world[axis]);
            }
        }
    }
    for axis in 0..3 {
        assert!((report.min[axis] - min[axis]).abs() < 2e-4 && (report.max[axis] - max[axis]).abs() < 2e-4, "axis {axis}: {:?} {:?} vs {min:?} {max:?}", report.min, report.max);
    }
    assert!(report.min[1] < report.max[1] && report.min[1] > snapshot.sites.values().next().unwrap().elevation - 5.0);
}

#[test]
fn through_applies_the_rotation_before_the_translation() {
    let node = GltfNode { name: String::new(), translation: Some([1.0, 2.0, 3.0]), rotation: Some([0.0, (std::f64::consts::FRAC_PI_2 / 2.0).sin(), 0.0, (std::f64::consts::FRAC_PI_2 / 2.0).cos()]), mesh: None, children: Vec::new(), extras: semio_framework_value::DslValue::Null };
    let [x, y, z] = through(&node, [1.0, 0.0, 0.0]);
    assert!((x - 1.0).abs() < 1e-12 && (y - 2.0).abs() < 1e-12 && (z - 2.0).abs() < 1e-12, "{x} {y} {z}");
}

#[test]
fn an_empty_scene_reports_zero_bounds() {
    let report = project(&build(&crate::ModelSnapshot::default()).0);
    assert_eq!((report.nodes, report.triangles, report.min, report.max), (0, 0, [0.0; 3], [0.0; 3]));
}

#[test]
fn the_json_form_parses_and_keeps_the_numbers() {
    let (model, _) = build(&house());
    let report = project(&model);
    let parsed: serde_json::Value = serde_json::from_str(&report.to_json()).expect("valid JSON");
    assert_eq!(parsed["nodes"].as_u64(), Some(report.nodes as u64));
    assert_eq!(parsed["triangles"].as_u64(), Some(report.triangles as u64));
    assert_eq!(parsed["bounds"]["min"][1].as_f64(), Some(report.min[1]));
    assert_eq!(parsed["storeys"]["st-ground"]["elements"].as_u64(), Some(report.storeys["st-ground"].elements as u64));
}

#[test]
fn the_subject_report_equals_the_table_the_three_oracle_measured_from_the_committed_file() {
    let oracle: serde_json::Value = serde_json::from_slice(&read("🔬️measure/🔣️.json")).expect("the oracle table");
    let (model, _) = build(&house());
    let ours: serde_json::Value = serde_json::from_str(&project(&model).to_json()).unwrap();
    for key in ["nodes", "meshes", "primitives", "triangles", "materials"] {
        assert_eq!(oracle[key], ours[key], "{key}");
    }
    assert_eq!(oracle["kinds"], ours["kinds"]);
    assert_eq!(oracle["storeys"], ours["storeys"]);
    for bound in ["min", "max"] {
        for axis in 0..3 {
            let (measured, written) = (oracle["bounds"][bound][axis].as_f64().unwrap(), ours["bounds"][bound][axis].as_f64().unwrap());
            assert!((measured - written).abs() < 1e-9, "{bound}[{axis}]: three {measured}, written {written}");
        }
    }
}
