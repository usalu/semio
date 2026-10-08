use super::*;
use semio_framework_3d::brep::engine::GeometryHandle;

fn value_of(build: impl FnOnce(&mut KernelSession) -> GeometryHandle) -> ShapeValue {
    let mut session = KernelSession::new();
    let handle = build(&mut session);
    session.export(&handle).expect("export")
}

fn near(left: [f64; 3], right: [f64; 3]) -> bool {
    (0..3).all(|axis| (left[axis] - right[axis]).abs() < 1e-9)
}

#[test]
fn a_wire_is_parameterised_by_member_index_plus_fraction() {
    let wire = value_of(|session| session.brep().rectangle_wire_sync(2.0, 1.0).expect("wire"));
    let source = CurveSource::of(&wire).expect("a wire is a curve source");
    assert_eq!(source.domain(), (0.0, 4.0));
    assert_eq!(source.pieces().len(), 4);
    assert!(near(source.point(0.5), [1.0, 0.0, 0.0]) && near(source.point(1.5), [2.0, 0.5, 0.0]) && near(source.point(3.5), [0.0, 0.5, 0.0]));
    assert!(near(source.point(-3.0), [0.0, 0.0, 0.0]) && near(source.point(9.0), [0.0, 0.0, 0.0]), "parameters clamp into the domain");
}

#[test]
fn a_member_run_against_its_edge_reverses_point_and_tangent() {
    let mut wire = value_of(|session| session.brep().rectangle_wire_sync(2.0, 1.0).expect("wire"));
    let ShapeRoot::Wire(inner) = &mut wire.root else { panic!("wire") };
    inner.members[1].1 = false;
    let source = CurveSource::of(&wire).expect("source");
    assert!(near(source.point(1.0), [2.0, 1.0, 0.0]) && near(source.point(2.0), [2.0, 0.0, 0.0]));
    let tangent = source.tangent(1.5).expect("tangent");
    assert!(near(tangent, [0.0, -1.0, 0.0]), "{tangent:?}");
    let (parameter, point, distance) = source.closest([3.0, 0.5, 0.0]);
    assert!((parameter - 1.5).abs() < 1e-9 && near(point, [2.0, 0.5, 0.0]) && (distance - 1.0).abs() < 1e-9);
}

#[test]
fn an_edge_and_a_curve_use_their_own_parameter() {
    let line = value_of(|session| session.brep().line_curve_sync([0.0; 3], [2.0, 0.0, 0.0]).expect("line"));
    let source = CurveSource::of(&line).expect("line");
    assert!(source.domain().0.is_infinite() && source.domain().1.is_infinite());
    assert!(near(source.point(0.5), [1.0, 0.0, 0.0]) && near(source.tangent(0.5).expect("tangent"), [1.0, 0.0, 0.0]));
    let circle = value_of(|session| session.brep().circle_curve_sync([0.0; 3], [0.0, 0.0, 1.0], 2.0).expect("circle"));
    let source = CurveSource::of(&circle).expect("circle");
    assert!((source.domain().1 - std::f64::consts::TAU).abs() < 1e-12 && (source.curvature(1.0) - 0.5).abs() < 1e-9);
}

#[test]
fn other_kinds_are_no_curve_and_no_surface_sources() {
    let solid = value_of(|session| session.brep().box_prim_sync(1.0, 1.0, 1.0).expect("box"));
    assert!(CurveSource::of(&solid).is_none() && SurfaceSource::of(&solid).is_none());
    let vertex = value_of(|session| session.brep().vertex_sync([0.0; 3]).expect("vertex"));
    assert!(CurveSource::of(&vertex).is_none());
}

#[test]
fn a_face_answers_through_its_surface_with_the_normal_turned_outward() {
    let whole = value_of(|session| session.brep().box_prim_sync(2.0, 2.0, 2.0).expect("box"));
    let mut session = KernelSession::new();
    let imported = session.import(&whole).expect("import").handle;
    let faces = session.brep().deconstruct_sync(&imported).expect("deconstruct").faces;
    let mut normals = Vec::new();
    for face in faces {
        let source = SurfaceSource::of(&session.export(&face).expect("face")).expect("a face is a surface source");
        normals.push(source.normal(0.0, 0.0).expect("normal"));
    }
    for wanted in [[0.0, 0.0, 1.0], [0.0, 0.0, -1.0], [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0]] {
        assert!(normals.iter().any(|found| near(*found, wanted)), "no face points along {wanted:?}: {normals:?}");
    }
}
