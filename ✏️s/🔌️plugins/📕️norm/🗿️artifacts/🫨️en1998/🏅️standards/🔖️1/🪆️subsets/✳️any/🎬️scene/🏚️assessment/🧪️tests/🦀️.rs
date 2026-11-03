use super::*;
use semio_s_artifact_norm_en1998_inferences::check_full_seismic;

fn document(shear_n: f64) -> En1998Snapshot {
    let mut document = En1998Snapshot::full_showcase();
    document.assessments.iter_mut().find(|a| a.id == "as-2").expect("as-2").shear_force_demand_n = shear_n;
    document
}

#[test]
fn the_diagram_binds_both_mechanisms_to_their_demand_fields() {
    let document = document(1.0e5);
    let report = check_full_seismic(&document);
    let all = diagrams(&document, &report);
    let diagram = all.iter().find(|d| d.id == "assessment-dc:as-2").expect("assessment diagram");
    assert_eq!(diagram.validate(), Ok(()));
    let shear = diagram.markers.iter().find(|m| m.id == "shear-as-2").expect("shear marker");
    let binding = shear.binding.as_ref().expect("shear binding");
    assert_eq!(binding.path, "assessments[id=as-2].shearForceDemandN");
    assert!((binding.map.k * shear.at.y - 1.0e5).abs() <= 1.0e-6 * 1.0e5);
    assert!(shear.bound_check.as_deref().is_some_and(|id| id.ends_with(".cyclicShear")));
    let chord = diagram.markers.iter().find(|m| m.id == "chord-as-2").expect("chord marker");
    let binding = chord.binding.as_ref().expect("chord binding");
    assert_eq!(binding.path, "assessments[id=as-2].chordRotationDemandRad");
    let demand = document.assessments.iter().find(|a| a.id == "as-2").expect("as-2").chord_rotation_demand_rad;
    assert!((binding.map.k * chord.at.y - demand).abs() <= 1.0e-9);
    assert!(chord.bound_check.as_deref().is_some_and(|id| id.ends_with(".chordRotation")));
}

#[test]
fn the_shear_marker_sits_at_zero_while_no_shear_demand_is_given() {
    let document = document(0.0);
    let report = check_full_seismic(&document);
    let all = diagrams(&document, &report);
    let diagram = all.iter().find(|d| d.id == "assessment-dc:as-2").expect("assessment diagram");
    let shear = diagram.markers.iter().find(|m| m.id == "shear-as-2").expect("shear marker");
    assert_eq!(shear.at.y, 0.0);
    assert!(shear.bound_check.is_none());
}
