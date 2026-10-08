use super::*;

#[path = "../../../🧪️tests/🧰️construction-support/🦀️.rs"]
mod support;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🔣️.json");

#[test]
fn every_fixture_case_holds_at_fuel_one_and_at_unbounded_fuel() {
    assert_eq!(support::run_fixture(FIXTURE, COMPUTES), 26);
}

#[test]
fn the_table_and_the_fixture_cover_exactly_the_catalogue_kinds_of_brep_surface() {
    support::assert_category("brep.surface", COMPUTES, FIXTURE);
}

#[test]
fn equal_inputs_give_equal_shape_bytes() {
    assert_eq!(support::assert_deterministic(FIXTURE, COMPUTES), 14);
}

fn filled_face_normal(points: &[[f64; 3]]) -> ([f64; 3], [f64; 3]) {
    let wire = std::sync::Arc::new(super::super::brep_curve::polyline_value(points, true).expect("closed polyline"));
    let winding = fit_plane(&wire_corners(&wire).expect("corners"), WIRE).expect("plane").normal;
    let kind = support::kind_of("brep.surface.faceFromWire");
    let job = Pipeline::new(kind)
        .import(&wire)
        .once(|work| {
            let face = fill_wire(work, 0, "wire")?;
            work.groups = vec![vec![face]];
            Ok(())
        })
        .exported("shape");
    let evaluation = support::drive(job, usize::MAX);
    assert!(evaluation.fault.is_none(), "{:?}", evaluation.fault);
    let Some(GeometryValue::Shape(face)) = evaluation.outputs.get("shape") else { panic!("a face") };
    let source = super::super::brep_sources::SurfaceSource::of(face).expect("a face has a surface");
    (winding, source.normal(0.0, 0.0).expect("a normal"))
}

#[test]
fn a_wire_in_any_plane_is_filled_with_a_face_that_points_the_way_the_wire_winds() {
    let square = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 3.0, 0.0], [0.0, 3.0, 0.0]];
    let tilted = [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0], [2.0, 0.0, 3.0], [0.0, 0.0, 3.0]];
    let skew = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
    for outline in [square, tilted, skew] {
        let reversed: Vec<[f64; 3]> = outline.iter().rev().copied().collect();
        for points in [outline.to_vec(), reversed] {
            let (winding, normal) = filled_face_normal(&points);
            assert!((0..3).all(|axis| (winding[axis] - normal[axis]).abs() < 1e-9), "{points:?}: the wire winds towards {winding:?} but its face points to {normal:?}");
        }
    }
}

#[test]
fn the_planar_fit_refuses_outlines_without_area_or_beyond_their_plane() {
    let line = fit_plane(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [2.0, 0.0, 0.0]], POINTS).expect_err("collinear");
    assert_eq!(line.code, "generation3d.geometry.points-degenerate");
    let bent = fit_plane(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 1.0]], WIRE).expect_err("not planar");
    assert_eq!(bent.code, "generation3d.geometry.wire-not-planar");
    for fault in [line, bent] {
        assert!(!fault.message.en.is_empty() && !fault.message.de.is_empty() && fault.message.en != fault.message.de);
    }
    let plane = fit_plane(&[[0.0, 0.0, 1.0], [2.0, 0.0, 1.0], [2.0, 2.0, 1.0], [0.0, 2.0, 1.0]], WIRE).expect("a square");
    assert_eq!(plane.normal, [0.0, 0.0, 1.0]);
    assert_eq!(plane.centroid, [1.0, 1.0, 1.0]);
}

#[test]
fn an_offset_face_is_cancellable_and_equal_when_computed_twice() {
    assert!(support::assert_cancellable(FIXTURE, COMPUTES) <= 26);
}
