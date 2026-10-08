use super::harness::{self as h, oracle};
use super::*;
use semio_framework_3d::mesh::MeshKernelError;
use std::collections::BTreeMap;

fn sphere(subdivisions: i64) -> std::sync::Arc<HalfedgeMesh> {
    std::sync::Arc::new(HalfedgeMesh::ico_sphere_prim(1.0, subdivisions as u32).unwrap())
}

fn inputs(kind_id: &str, entries: Vec<(&str, GeometryValue)>) -> (&'static Kind, WidgetInputs) {
    let kind = oracle::kind_of(kind_id);
    (kind, WidgetInputs::new("w", kind, entries.into_iter().map(|(port, value)| (port.to_string(), value)).collect::<BTreeMap<_, _>>()))
}

fn start(kind_id: &str, entries: Vec<(&str, GeometryValue)>) -> Box<dyn WidgetJob> {
    let (kind, values) = inputs(kind_id, entries);
    super::super::mesh_edit::COMPUTES.iter().find(|entry| entry.id == kind_id).map(|entry| (entry.start)(kind, values)).expect("a mesh.edit kind")
}

#[test]
fn every_kernel_error_has_its_own_code_and_distinct_localized_text() {
    let errors = [MeshKernelError::InvalidHandle, MeshKernelError::NonManifold, MeshKernelError::DegenerateOperation, MeshKernelError::EmptySelection, MeshKernelError::InvalidInput("detail".into())];
    let faults: Vec<WidgetFault> = errors.iter().map(mesh_fault).collect();
    let mut codes: Vec<&str> = faults.iter().map(|fault| fault.code.as_str()).collect();
    codes.sort();
    codes.dedup();
    assert_eq!(codes.len(), errors.len());
    for fault in &faults {
        assert!(fault.code.starts_with("generation3d.geometry.") && !fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de);
    }
    assert!(faults[4].message.en.contains("detail") && faults[4].message.de.contains("detail"));
}

#[test]
fn a_selection_is_sorted_deduplicated_and_validated_against_the_mesh() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let (_, values) = inputs("mesh.edit.extrude", vec![("faces", GeometryValue::Selection(SelectionValue { component: SelectionKind::Face, ids: vec![5, 1, 5, 0] }))]);
    assert_eq!(picked(&values, "faces", &mesh, Element::Face).unwrap(), vec![0, 1, 5]);
    let (_, stale) = inputs("mesh.edit.extrude", vec![("faces", GeometryValue::Selection(SelectionValue { component: SelectionKind::Face, ids: vec![0, 6] }))]);
    let fault = picked(&stale, "faces", &mesh, Element::Face).unwrap_err();
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), (SELECTION_STALE, Some("faces")));
    assert!(fault.message.en.contains("5") && fault.message.de.contains("5") && fault.message.en != fault.message.de);
    assert_eq!(picked_one(&values, "faces", &mesh, Element::Face).unwrap_err().code, SELECTION_COUNT);
}

#[test]
fn edge_ids_are_bounded_by_the_half_edges_not_the_undirected_edges() {
    let mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    assert_eq!((Element::Vertex.bound(&mesh), Element::Edge.bound(&mesh), Element::Face.bound(&mesh)), (8, 24, 6));
    assert_eq!(mesh.edge_count(), 12);
}

#[test]
fn a_started_job_reports_monotone_progress_and_finishes_at_every_fuel() {
    for fuel in [1, 2, 64] {
        let evaluation = oracle::drive(start("mesh.edit.triangulate", vec![("mesh", GeometryValue::Mesh(sphere(3)))]), fuel);
        assert!(evaluation.fault.is_none());
        let GeometryValue::Mesh(mesh) = &evaluation.outputs["mesh"] else { panic!("a mesh output") };
        assert_eq!(mesh.face_count(), 1280);
    }
}

#[test]
fn work_is_charged_to_the_grant_not_done_at_start() {
    let mut job = start("mesh.edit.triangulate", vec![("mesh", GeometryValue::Mesh(sphere(4)))]);
    assert!(matches!(job.step(1), WidgetStep::Working { .. }), "one fuel unit cannot finish 5120 faces");
}

#[test]
fn a_cancelled_job_answers_a_localized_cancellation_and_nothing_else() {
    let mut job = start("mesh.edit.triangulate", vec![("mesh", GeometryValue::Mesh(sphere(4)))]);
    assert!(matches!(job.step(1), WidgetStep::Working { .. }));
    job.cancel();
    job.cancel();
    let WidgetStep::Done(evaluation) = job.step(1) else { panic!("a cancelled job is done") };
    let fault = evaluation.fault.expect("a cancellation fault");
    assert_eq!(fault.code, CANCELLED);
    assert!(evaluation.outputs.is_empty() && fault.message.en != fault.message.de);
}

#[test]
fn a_cancel_before_the_first_step_never_builds_the_kernel_job() {
    let mut job = start("mesh.edit.triangulate", vec![("mesh", GeometryValue::Mesh(sphere(1)))]);
    job.cancel();
    let WidgetStep::Done(evaluation) = job.step(100) else { panic!("done") };
    assert_eq!(evaluation.fault.expect("fault").code, CANCELLED);
}

#[test]
fn a_missing_mesh_input_is_a_localized_fault_at_that_port_and_never_a_panic() {
    let evaluation = oracle::drive(start("mesh.edit.triangulate", vec![]), 1);
    let fault = evaluation.fault.expect("a fault");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.input-missing", Some("mesh")));
}

#[test]
fn a_mesh_of_the_wrong_type_is_refused_at_its_port() {
    let evaluation = oracle::drive(start("mesh.edit.triangulate", vec![("mesh", GeometryValue::Number(1.0))]), 1);
    assert_eq!(evaluation.fault.expect("a fault").code, "generation3d.geometry.input-type");
}

#[test]
fn the_fixture_measures_agree_with_the_kernel_analysis_of_the_same_mesh() {
    let mesh = HalfedgeMesh::ico_sphere_prim(1.0, 2).unwrap();
    let measured = h::measure(&mesh);
    let report = semio_framework_3d::mesh::analyze_polygon_soup(&mesh.positions(), &mesh.polygons());
    assert_eq!(measured.faces, 320);
    assert_eq!(measured.euler, 2);
    assert!(measured.boundary == 0 && measured.non_manifold == 0 && measured.oriented);
    assert!(report.closed && report.euler_characteristic == 2);
    assert!((measured.volume - report.signed_volume).abs() < 1e-4 * measured.volume, "{} {}", measured.volume, report.signed_volume);
    assert!((measured.area - report.area).abs() < 1e-4 * measured.area);
}
