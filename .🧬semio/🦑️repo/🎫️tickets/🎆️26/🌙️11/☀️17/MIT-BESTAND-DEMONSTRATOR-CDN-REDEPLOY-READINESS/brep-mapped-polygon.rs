use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_representation::topology::{Body, Edge, Face, Vertex, make_loop};
use semio_framework_3d_brep_solid_topology::mapped_boundary::{FaceMapping, MappedBoundary};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::matrix::{Affine3, Frame3};
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

#[unsafe(no_mangle)]
pub extern "C" fn mapped(mode: u32, index: usize, coordinate: usize) -> f64 {
    let tol = Tol::DEFAULT;
    let mut body = Body::new();
    let points = [Pnt3::new(0., 0., 2.), Pnt3::new(4., 0., 2.), Pnt3::new(0., 3., 2.)];
    let surface = body.surfaces.insert(Surface::Plane {
        frame: Frame3 { origin: points[0], x: Vec3::X, y: Vec3::Y, z: Vec3::Z },
    });
    let label = body.new_label();
    let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: mode == 1, tol, label });
    let vertices = points.map(|position| {
        let label = body.new_label();
        body.vertices.insert(Vertex { position, tol, label })
    });
    let members = std::array::from_fn::<_, 3, _>(|i| {
        let j = (i + 1) % points.len();
        let curve = body.curves3.insert(Curve3::Line { origin: points[i], dir: points[j] - points[i] });
        let label = body.new_label();
        let v0 = vertices[i];
        let v1 = vertices[if mode == 3 && i == 1 { 0 } else { j }];
        (body.edges.insert(Edge { curve, range: (0., 1.), v0, v1, tol, label }), true)
    });
    let ring = make_loop(&mut body, face, &members);
    body.faces.get_mut(face).unwrap().outer = Some(ring);
    let mapping = match mode {
        0 | 1 | 3 => FaceMapping::Offset(5.),
        2 => FaceMapping::Affine(Affine3::scaling(Pnt3::new(0., 0., 0.), Vec3::new(-2., 3., 4.))),
        _ => FaceMapping::Affine(Affine3::scaling(Pnt3::new(0., 0., 0.), Vec3::new(0., 3., 4.))),
    };
    let (mut boundary, opening) = MappedBoundary::admit(&body, face, || Ok(mapping)).unwrap().unwrap();
    assert_eq!(boundary.source(), face);
    assert!(opening.is_none());
    assert!(boundary.scan_demand(&body).unwrap().copy_bytes > 0);
    if boundary.prepare(&body).is_err() { return f64::NAN; }
    for edge in 0..3 {
        assert!(boundary.scan_demand(&body).unwrap().copy_bytes > 0);
        match boundary.scan(&body) {
            Ok(done) => assert_eq!(done, edge == 2),
            Err(_) => return f64::NAN,
        }
    }
    assert_eq!(boundary.edge_count(), 3);
    let plane = boundary.plane().unwrap();
    if index == 3 { return f64::from(u8::from(plane.flipped)); }
    for ordinal in 0..=index {
        assert!(boundary.point_demand(&body).unwrap().copy_bytes > 0);
        let (point, next) = boundary.point(&body).unwrap();
        if ordinal == index { return point[coordinate]; }
        boundary.advance(next);
    }
    unreachable!()
}
