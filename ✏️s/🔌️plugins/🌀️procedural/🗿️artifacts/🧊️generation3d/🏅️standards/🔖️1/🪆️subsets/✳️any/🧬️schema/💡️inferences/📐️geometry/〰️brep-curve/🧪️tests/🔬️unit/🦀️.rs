use super::*;
use semio_framework_3d::brep::engine::Brep;

#[path = "../../../🧪️tests/🧰️construction-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert_eq!(support::run_fixture(FIXTURE, COMPUTES), 28);
}

#[test]
fn the_table_and_the_fixture_cover_exactly_the_catalogue_kinds_of_brep_curve() {
    support::assert_category("brep.curve", COMPUTES, FIXTURE);
}

#[test]
fn equal_inputs_give_equal_shape_bytes() {
    assert_eq!(support::assert_deterministic(FIXTURE, COMPUTES), 15);
}

#[test]
fn a_polyline_value_is_a_fixed_point_of_import_and_export() {
    for (points, closed) in [(vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]], false), (vec![[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 1.0, 0.0]], true)] {
        let value = polyline_value(&points, closed).expect("polyline value");
        let mut session = KernelSession::new();
        let handle = session.brep().import_shape(&value).expect("imports");
        assert_eq!(session.export(&handle).expect("exports"), value, "closed = {closed}");
    }
}

#[test]
fn a_curve_and_an_edge_read_as_one_edge_wires_that_import_and_measure_like_their_curve() {
    let mut session = KernelSession::new();
    let line = session.brep().line_curve_sync([0.0, 0.0, 0.0], [0.0, 0.0, 4.0]).expect("line");
    let arc = session.brep().arc_curve_sync([0.0; 3], [0.0, 0.0, 1.0], 2.0, 0.0, std::f64::consts::FRAC_PI_2).expect("arc");
    let circle = session.brep().circle_curve_sync([0.0; 3], [0.0, 0.0, 1.0], 1.0).expect("circle");
    for (handle, length, closed) in [(line, 4.0, false), (arc, std::f64::consts::PI, false), (circle, std::f64::consts::TAU, true)] {
        let curve = Arc::new(session.export(&handle).expect("curve value"));
        let wire = path_wire(&curve, "path").expect("curve wire");
        assert!(matches!(&wire.root, ShapeRoot::Wire(w) if w.closed == closed && w.members.len() == 1));
        let mut fresh = Brep::new();
        let imported = fresh.import_shape(&wire).expect("the wire imports");
        assert!((fresh.length_sync(&imported).expect("length") - length).abs() < 1e-6);
        assert_eq!(path_wire(&Arc::new((*wire).clone()), "path").expect("a wire stays"), wire);
    }
}

#[test]
fn a_path_that_is_not_a_curve_edge_or_wire_is_refused_in_both_languages() {
    let mut session = KernelSession::new();
    let solid = session.brep().box_prim_sync(1.0, 1.0, 1.0).expect("box");
    let fault = path_wire(&Arc::new(session.export(&solid).expect("box value")), "path").expect_err("a solid is no path");
    assert_eq!((fault.code.as_str(), fault.port.as_deref()), ("generation3d.geometry.shape-kind", Some("path")));
    assert!(!fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de);
}

#[test]
fn the_local_x_axis_of_a_plane_is_a_unit_vector_in_it_and_does_not_depend_on_the_kernel_frame() {
    assert_eq!(local_x([0.0, 0.0, 1.0]), [1.0, 0.0, 0.0]);
    assert_eq!(local_x([1.0, 0.0, 0.0]), [0.0, 1.0, 0.0]);
    let slanted = [0.6, 0.0, 0.8];
    let axis = local_x(slanted);
    assert!((norm(axis) - 1.0).abs() < 1e-12 && dot(axis, slanted).abs() < 1e-12);
}
