from pathlib import Path
import subprocess

base = Path("🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/✂️intersect")
fixture = Path("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️line-conic-intersections.json").resolve()
circles = fixture.with_name("🔣️circle-pair-intersections.json")
spatial = fixture.with_name("🔣️spatial-circle-intersections.json")
spatial_conics = fixture.with_name("🔣️spatial-conic-intersections.json")
coplanar_conics = fixture.with_name("🔣️coplanar-conic-intersections.json")
mixed_section = fixture.with_name("🔣️mixed-section-distance.json")
planar_face = fixture.with_name("🔣️planar-face-projection.json")
surface_distance = fixture.with_name("🔣️surface-vertex-distance.json")
elliptic_trim = fixture.with_name("🔣️elliptic-trim.json")
length_spline = fixture.with_name("🔣️spline-length.json")
nearest_spline = fixture.with_name("🔣️spline-nearest.json")
spline_plane = fixture.with_name("🔣️spline-plane-intersections.json")
conic_plane = next(Path("✏️s/🔌️plugins/🌀️procedural").glob("**/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).resolve()
analysis = base.parent / "🔎️analysis"
bounded_length = (analysis / "📏️bounded-length/🦀️.rs").resolve()
measure = (analysis / "📐️analytic-measure/🦀️.rs").read_text()
measure = measure[measure.index("pub fn analytic_curve_length"):measure.index("/// 🧭️ Samples a point")]
analytic_nearest = (analysis / "🎯️nearest-analytic/🦀️.rs").resolve()
planar = (analysis / "🎯️planar-closest/🦀️.rs").resolve()
linear_distance = (analysis / "📏️linear-distance/🦀️.rs").resolve()
directory = Path(__file__).parent / "🗑️generated"
directory.mkdir(exist_ok=True)
check = r"""
use semio_framework_3d_brep_curve::{bounded_spline::{closest_spline_parameter, ArcLengthCursor, ArcLengthRefusal, SplineRefusal}, bspline::KnotVector, Curve2, Curve3};
use semio_framework_3d_brep_vector::{Pnt2, Pnt3, Vec2, Vec3, matrix::Frame3};
use semio_line_conic_intersect::{intersect_curve_curve, intersect_original_curve_curve, intersect_original_curve_plane};
use semio_framework_value::{ValueError, ValueRefusalKind};
use serde_json::Value;
#[allow(dead_code)]
#[path = "__ANALYTIC_NEAREST__"]
mod nearest_analytic;
#[path = "__BOUNDED_LENGTH__"]
mod bounded_length;
use semio_framework_3d_brep_curve::curve_ops::ellipse_perimeter;
use semio_framework_3d_brep_representation::topology::{Body, Edge, Face, Vertex, Wire, make_loop};
use semio_framework_3d_brep_surface::Surface;
__ANALYTIC_MEASURE__
#[path = "__LINEAR_DISTANCE__"]
mod linear_distance;
#[path = "__PLANAR__"]
mod planar_projection;
use planar_projection::planar_dual_coordinates;
fn xyz(raw: &Value) -> [f64; 3] { std::array::from_fn(|index| raw[index].as_f64().unwrap()) }
fn range(raw: &Value) -> Option<(f64, f64)> { Some((raw.get(0)?.as_f64()?, raw.get(1)?.as_f64()?)) }
fn conic(raw: &Value) -> (Curve3, Option<(f64, f64)>) {
    let frame = Frame3::from_x_z(Pnt3::from_array(xyz(&raw["center"])), Vec3::from_array(xyz(&raw["axis"])), Vec3::from_array(xyz(&raw["normal"]))).unwrap();
    let curve = if let Some(radius) = raw["radius"].as_f64() { Curve3::Circle { frame, radius } } else {
        let radii: [f64; 2] = serde_json::from_value(raw["radii"].clone()).unwrap();
        Curve3::Ellipse { frame, major_radius: radii[0], minor_radius: radii[1] }
    };
    (curve, range(&raw["range"]))
}
fn verify_conic_pairs(fixture: &str) {
    let fixture: Value = serde_json::from_str(fixture).unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let (a, trim_a) = conic(&row["a"]);
        let (b, trim_b) = conic(&row["b"]);
        let result = intersect_original_curve_curve((&a, trim_a), (&b, trim_b), tolerance);
        match row["fault"].as_str() {
            Some(fault) => { assert_eq!(result.unwrap_err().message(), fault, "{}", row["id"]); continue; }
            None => {}
        }
        let hits = result.unwrap().unwrap();
        let expected = row["points"].as_array().unwrap();
        assert_eq!(hits.iter().flatten().count(), expected.len(), "{}: {hits:?}", row["id"]);
        if trim_a.is_none() && trim_b.is_none() {
            assert_eq!(intersect_curve_curve(&a, &b, tolerance).unwrap(), hits.into_iter().flatten().collect::<Vec<_>>());
        }
        for (index, (hit, point)) in hits.into_iter().flatten().zip(expected).enumerate() {
            assert!(hit.point.distance(Pnt3::from_array(xyz(point))) <= row["pointTolerance"].as_f64().unwrap_or(tolerance), "{}: {hit:?}", row["id"]);
            let tangent = row["tangencies"][index].as_bool().or_else(|| row["tangent"].as_bool()).unwrap_or(false);
            assert_eq!(hit.tangent, tangent, "{}: {hit:?}", row["id"]);
            assert!(hit.point.distance(a.eval(hit.t_a)) <= tolerance);
            assert!(hit.point.distance(b.eval(hit.t_b)) <= tolerance);
            for (parameter, trim) in [(hit.t_a, trim_a), (hit.t_b, trim_b)] {
                assert!(trim.is_none_or(|(lo, hi)| (lo..=hi).contains(&parameter)), "{} {parameter} {trim:?}", row["id"]);
            }
        }
    }
}
fn plane(raw: &Value) -> Frame3 {
    Frame3::from_normal(Pnt3::from_array(xyz(&raw["origin"])), Vec3::from_array(xyz(&raw["normal"]))).unwrap()
}
fn plane_curve(raw: &Value) -> Curve3 {
    if let Some(radii) = raw["ellipse"].as_array() {
        let origin = raw.get("origin").map(xyz).unwrap_or_default();
        let normal = raw.get("normal").map(xyz).unwrap_or([0.0, 0.0, 1.0]);
        let frame = Frame3::from_x_z(Pnt3::from_array(origin), Vec3::X, Vec3::from_array(normal)).unwrap();
        return Curve3::Ellipse { frame, major_radius: radii[0].as_f64().unwrap(), minor_radius: radii[1].as_f64().unwrap() };
    }
    if let Some(radius) = raw["circle"].as_f64() {
        let frame = Frame3::from_x_z(Pnt3::from_array([0.0; 3]), Vec3::X, Vec3::Z).unwrap();
        return Curve3::Circle { frame, radius };
    }
    if let Some(line) = raw["line"].as_array() {
        let origin = Pnt3::from_array(xyz(&line[0]));
        return Curve3::Line { origin, dir: Pnt3::from_array(xyz(&line[1])) - origin };
    }
    let degree = raw["degree"].as_u64().unwrap() as usize;
    let controls: Vec<_> = raw["controls"].as_array().unwrap().iter().map(|value| Pnt3::from_array(xyz(value))).collect();
    let weights = serde_json::from_value(raw["weights"].clone()).unwrap();
    let knots = KnotVector::new(serde_json::from_value(raw["knots"].clone()).unwrap(), degree, controls.len()).unwrap();
    Curve3::Nurbs { knots, controls, weights }
}
fn verify_plane_contacts() {
    let data: Value = serde_json::from_str(include_str!("__SPLINE_PLANE__")).unwrap();
    let tol = data["tolerance"].as_f64().unwrap();
    for row in data["cases"].as_array().unwrap() {
        let curve = plane_curve(&row["curve"]);
        let frame = plane(row.get("plane").unwrap_or(&data["plane"]));
        let result = intersect_original_curve_plane((&curve, range(&row["range"])), &frame, tol);
        if let Some(fault) = row["fault"].as_str() {
            match result {
                Err(reason) => assert!(reason.message().contains(fault), "{}: {reason:?}", row["id"]),
                Ok(None) => assert!(fault.starts_with("requires a supported"), "{}", row["id"]),
                Ok(Some(hits)) => panic!("{} unexpectedly published {hits:?}", row["id"]),
            }
            continue;
        }
        let hits = result.unwrap_or_else(|reason| panic!("{}: {reason:?}", row["id"])).unwrap();
        let points = row["points"].as_array().unwrap();
        assert_eq!(hits.iter().flatten().count(), points.len(), "{}: {hits:?}", row["id"]);
        for (index, (hit, point)) in hits.into_iter().flatten().zip(points).enumerate() {
            assert!(hit.point.distance(Pnt3::from_array(xyz(point))) <= tol, "{}: {hit:?}", row["id"]);
            assert!((hit.t - row["parameters"][index].as_f64().unwrap()).abs() < 1e-10, "{}: {hit:?}", row["id"]);
            assert!(hit.point.distance(curve.eval(hit.t)) <= tol);
            assert!(frame.to_local(hit.point).z.abs() <= tol);
        }
    }
    let data: Value = serde_json::from_str(include_str!("__CONIC_PLANE__")).unwrap();
    let tol = data["tolerance"].as_f64().unwrap();
    for row in data["retainedConicPlane"].as_array().unwrap() {
        let raw = &row["curve"];
        let frame = Frame3::from_normal(Pnt3::from_array(xyz(&raw["center"])), Vec3::from_array(xyz(&raw["normal"]))).unwrap();
        let [a, b]: [f64; 2] = serde_json::from_value(raw["radii"].clone()).unwrap();
        let curve = if a == b { Curve3::Circle { frame, radius: a } }
            else { Curve3::Ellipse { frame, major_radius: a, minor_radius: b } };
        let result = intersect_original_curve_plane((&curve, None), &plane(&row["plane"]), tol);
        if row["fault"].as_str() == Some("infinitely many intersections") {
            assert_eq!(result.unwrap_err().message(), "infinitely many intersections");
            continue;
        }
        let hits = result.unwrap().unwrap();
        let points = row["points"].as_array().map(Vec::as_slice).unwrap_or_default();
        assert_eq!(hits.iter().flatten().count(), points.len(), "{row}");
        for (hit, point) in hits.into_iter().flatten().zip(points) {
            assert!(hit.point.distance(Pnt3::from_array(xyz(point))) <= tol, "{row}: {hit:?}");
        }
    }
}
fn verify_spline_nearest() {
    let data: Value = serde_json::from_str(include_str!("__NEAREST_SPLINE__")).unwrap();
    let tol = data["absoluteTolerance"].as_f64().unwrap().min(1e-10);
    for row in data["cases"].as_array().unwrap() {
        let mut curve = plane_curve(&row["curve"]);
        let target = Pnt3::from_array(xyz(&row["target"]));
        let trim = range(&row["range"]);
        let result = closest_spline_parameter(&curve, trim, target);
        let projection = nearest_analytic::curve_nearest((&curve, trim), target);
        if let Some(fault) = row["fault"].as_str() {
            assert_eq!(result.unwrap_err(), SplineRefusal::Unsupported, "{}", row["id"]);
            assert!(format!("{:?}", projection.unwrap_err()).contains(fault), "{}", row["id"]);
            continue;
        }
        let (t, point, distance) = result.unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        assert_eq!(projection.unwrap(), ([t], point, distance));
        assert!((t - row["parameter"].as_f64().unwrap()).abs() <= tol, "{}: t = {t}", row["id"]);
        assert!(point.distance(Pnt3::from_array(xyz(&row["point"]))) <= tol, "{}: point = {point:?}", row["id"]);
        assert!((distance - row["distance"].as_f64().unwrap()).abs() <= tol, "{}: distance = {distance}", row["id"]);
        if let Curve3::Nurbs { weights, .. } = &mut curve {
            let scale = weights.iter().copied().fold(0.0, f64::max);
            for weight in weights { *weight /= scale; }
        }
        assert!(point.distance(curve.eval(t)) <= tol);
    }
}
fn verify_spline_length() {
    let data: Value = serde_json::from_str(include_str!("__LENGTH_SPLINE__")).unwrap();
    let abs_tol = data["absoluteTolerance"].as_f64().unwrap();
    let rel_tol = data["relativeTolerance"].as_f64().unwrap();
    let measure = |raw: &Value, trim| {
        let curve = plane_curve(raw);
        let mut cursor = ArcLengthCursor::default();
        let mut owner = bounded_length::LengthAccumulator::default();
        let empty_body = Body::new();
        for step in 1..16_386 {
            let source = || bounded_length::LengthSource::Curve(&curve, trim);
            assert_eq!(owner.demand(&empty_body, source()).unwrap().copy_bytes, 16);
            let measured = cursor.step_spline(&curve, trim);
            let owned = owner.step(&empty_body, source());
            match measured {
                Ok(None) => assert_eq!(owned.unwrap(), None),
                Ok(Some(length)) => { assert_eq!(owned.unwrap(), Some(length)); return (Ok(length), step); },
                Err(error) => { assert!(owned.is_err()); return (Err(error), step); },
            }
        }
        panic!("length cursor failed to terminate");
    };
    for row in data["cases"].as_array().unwrap() {
        if let Some(curves) = row["wire"].as_array() {
            let mut body = Body::new();
            let edges: Vec<_> = curves.iter().map(|raw| {
                let curve = plane_curve(raw);
                let range = range(&raw["range"]).unwrap_or_else(|| if matches!(curve, Curve3::Line { .. }) { (0.0, 1.0) } else { curve.domain() });
                let [v0, v1] = [range.0, range.1].map(|t| {
                    let label = body.new_label();
                    body.vertices.insert(Vertex { position: curve.eval(t), label, tol: Default::default() })
                });
                let curve = body.curves3.insert(curve);
                let label = body.new_label();
                body.edges.insert(Edge { curve, range, v0, v1, label, tol: Default::default() })
            }).collect();
            let members = row["members"].as_array().unwrap().iter().enumerate()
                .map(|(index, id)| (edges[id.as_u64().unwrap() as usize], index != 1)).collect();
            let wire = Wire { members, vertices: vec![], closed: false };
            let label = body.new_label();
            let mut cursor = bounded_length::LengthAccumulator::default();
            let target = row["length"].as_f64().unwrap();
            for step in 1..10_000 {
                let source = || bounded_length::LengthSource::Wire(&wire, label.0);
                assert!(cursor.demand(&body, source()).unwrap().copy_bytes <= 16);
                if let Some(length) = cursor.step(&body, source()).unwrap() {
                    assert!((length - target).abs() <= 2.0 * abs_tol + rel_tol * target, "{}", row["id"]);
                    break;
                }
                assert!(step < 9_999, "{}", row["id"]);
            }
            continue;
        }
        let (result, steps) = measure(&row["curve"], range(&row["range"]));
        if row["fault"].is_string() {
            assert_eq!(result.unwrap_err(), ArcLengthRefusal::Source(SplineRefusal::Unsupported), "{}", row["id"]);
            continue;
        }
        let actual = result.unwrap_or_else(|reason| panic!("{} at {steps}: {reason:?}", row["id"]));
        let target = row["length"].as_f64().unwrap();
        assert!((actual - target).abs() <= abs_tol + rel_tol * target, "{}: {actual} != {target}, {steps} steps", row["id"]);
        assert!(steps < 10_000, "{}: {steps} steps", row["id"]);
    }
}
fn verify_elliptic_trim() {
    let data: Value = serde_json::from_str(include_str!("__ELLIPTIC_TRIM__")).unwrap();
    for row in data["cases"].as_array().unwrap() {
        let curve = plane_curve(&row["curve"]);
        let trim = range(&row["range"]).unwrap();
        let target = Pnt3::from_array(xyz(&row["target"]));
        let ([t], point, distance) = nearest_analytic::curve_nearest((&curve, Some(trim)), target)
            .unwrap_or_else(|error| panic!("{}: {error:?}", row["id"]));
        assert!((t - row["parameter"].as_f64().unwrap()).abs() < 1e-10, "{}: t = {t}", row["id"]);
        assert!(point.distance(Pnt3::from_array(xyz(&row["point"]))) < 1e-9, "{}: {point:?}", row["id"]);
        assert!((distance - row["distance"].as_f64().unwrap()).abs() < 1e-9);
        let mut owner = bounded_length::LengthAccumulator::default();
        let body = Body::new();
        let expected = row["length"].as_f64().unwrap();
        let tolerance = data["absoluteTolerance"].as_f64().unwrap() + data["relativeTolerance"].as_f64().unwrap() * expected;
        for step in 1..10_000 {
            let source = || bounded_length::LengthSource::Curve(&curve, Some(trim));
            let Some(actual) = owner.step(&body, source()).unwrap() else {
                assert!(step < 9_999, "{}", row["id"]);
                continue;
            };
            assert!((actual - expected).abs() <= tolerance, "{}: {actual} != {expected}", row["id"]);
            break;
        }
    }
}
fn verify_face_distances() {
    let data: Value = serde_json::from_str(include_str!("__PLANAR_FACE__")).unwrap();
    let frame = &data["frame"];
    let frame = Frame3 {
        origin: Pnt3::from_array(xyz(&frame["origin"])), x: Vec3::from_array(xyz(&frame["xAxis"])),
        y: Vec3::from_array(xyz(&frame["yAxis"])), z: Vec3::from_array(xyz(&frame["normal"])),
    };
    let mut body = Body::new();
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let label = body.new_label();
    let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: true, tol: Default::default(), label });
    for (index, row) in std::iter::once(&data["outer"]).chain(data["holes"].as_array().unwrap()).enumerate() {
        let vertices: Vec<_> = row.as_array().unwrap().iter().map(|position| {
            let position = Pnt3::from_array(xyz(position));
            let label = body.new_label();
            (body.vertices.insert(Vertex { position, label, tol: Default::default() }), position)
        }).collect();
        let ring: Vec<_> = (0..vertices.len()).map(|i| {
            let forward = i % 2 == 0;
            let (a,b) = (vertices[i], vertices[(i + 1) % vertices.len()]);
            let ((v0, a), (v1, b)) = if forward { (a, b) } else { (b, a) };
            let dir = (b-a) * 0.5;
            let curve = body.curves3.insert(Curve3::Line { origin: a - dir * 2.0, dir });
            let label = body.new_label();
            (body.edges.insert(Edge { curve, range: (2.0, 4.0), v0, v1, label, tol: Default::default() }), forward)
        }).collect();
        let loop_id = make_loop(&mut body, face, &ring);
        let record = body.faces.get_mut(face).unwrap();
        if index == 0 { record.outer = Some(loop_id); } else { record.inners.push(loop_id); }
    }
    assert!(planar_projection::closest_face_depth(&body, face).unwrap() > 0);
    for row in data["cases"].as_array().unwrap() {
        let target = Pnt3::from_array(xyz(&row["target"]));
        let mut cursor = planar_projection::PlanarClosest::new(face, target);
        assert_eq!(cursor.source(), face);
        let point = (0..64).find_map(|_| {
            let demand = cursor.next_demand(&body).unwrap();
            assert!(demand.copy_bytes <= 128 && demand.depth > 0);
            cursor.step(&body).unwrap()
        }).unwrap();
        assert!(point.distance(Pnt3::from_array(xyz(&row["point"]))) < 1e-10, "{}", row["name"]);
        assert!((point.distance(target) - row["distance"].as_f64().unwrap()).abs() < 1e-10);
    }
    let data: Value = serde_json::from_str(include_str!("__SURFACE_DISTANCE__")).unwrap();
    let normal = Vec3::from_array(xyz(&data["normal"]));
    for row in data["cases"].as_array().unwrap() {
        let target = Pnt3::from_array(xyz(&row["target"]));
        let origin = Pnt3::from_array(xyz(&row["origin"]));
        let kind = row["kind"].as_str().unwrap();
        let scalar = |key: &str| row[key].as_f64().unwrap();
        let point = if kind.ends_with("-face") {
            let frame = Frame3::from_normal(origin, normal).unwrap();
            let curve = if kind == "circle-face" { Curve3::Circle { frame, radius: scalar("radius") } }
                else { Curve3::Ellipse { frame, major_radius: scalar("major"), minor_radius: scalar("minor") } };
            nearest_analytic::conic_disk_nearest(&curve, target).unwrap()
        } else {
            let frame = Frame3::from_x_z(origin, Vec3::X, normal).unwrap();
            let surface = match kind {
                "plane" => Surface::Plane { frame },
                "cylinder" => Surface::Cylinder { frame, radius: scalar("radius") },
                "sphere" => Surface::Sphere { frame, radius: scalar("radius") },
                "cone" => Surface::Cone { frame, half_angle: scalar("halfAngle") },
                "torus" => Surface::Torus { frame, major_radius: scalar("major"), minor_radius: scalar("minor") },
                _ => unreachable!(),
            };
            nearest_analytic::surface_nearest(&surface, target).unwrap().1
        };
        assert!(point.distance(Pnt3::from_array(xyz(&row["point"]))) < 1e-10, "{}", row["id"]);
        assert!((point.distance(target) - scalar("distance")).abs() < 1e-10, "{}", row["id"]);
    }
}
fn verify_mixed_sections() {
    use std::f64::consts::PI;
    let data: Value = serde_json::from_str(include_str!("__MIXED_SECTION__")).unwrap();
    for row in data["sections"].as_array().unwrap() {
        let x = Vec3::from_array(xyz(&row["axisU"]));
        let y = Vec3::from_array(xyz(&row["axisV"]));
        let frame = Frame3 { origin: Pnt3::from_array(xyz(&row["center"])), x, y, z: x.cross(y) };
        let world = |uv: Pnt2| frame.origin + frame.x * uv.x + frame.y * uv.y;
        let mut body = Body::new();
        let surface = body.surfaces.insert(Surface::Plane { frame });
        let label = body.new_label();
        let face = body.faces.insert(Face { surface, outer: None, inners: vec![], flipped: false, tol: Default::default(), label });
        let mut arcs = vec![];
        if row["kind"] == "cylinder" {
            let a = 2.0 * 2.0_f64.sqrt();
            let curve = Curve3::Ellipse { frame, major_radius: a, minor_radius: 2.0 };
            let uv = Curve2::Ellipse { center: Pnt2::new(0.0, 0.0), x_axis: Vec2::new(1.0, 0.0), major_radius: a, minor_radius: 2.0 };
            let charts = if row["height"] == 4.0 { vec![(-2.0 * PI / 3.0, 2.0 * PI / 3.0)] }
                else { vec![(PI / 3.0, 2.0 * PI / 3.0), (4.0 * PI / 3.0, 5.0 * PI / 3.0)] };
            for range in charts { arcs.push((curve.clone(), uv.clone(), range)); }
        } else {
            let slope = row["normal"][0].as_f64().unwrap();
            let scale = slope.hypot(1.0);
            let (cap, mid, rise, weight) = if slope == 2.0 { (1.5, -4.0, 13.75_f64.sqrt(), 1.0) }
                else { (1.0, -2.0, 15.0_f64.sqrt(), 2.0) };
            let uv = [Pnt2::new(scale * cap, -rise), Pnt2::new(scale * mid, 0.0), Pnt2::new(scale * cap, rise)];
            let weights = vec![1.0, weight, 1.0];
            let knots = KnotVector::clamped_uniform(3, 2);
            let curve = Curve3::Nurbs { knots: knots.clone(), controls: uv.map(world).to_vec(), weights: weights.clone() };
            let trim = Curve2::Nurbs { knots, controls: uv.to_vec(), weights };
            arcs.push((curve, trim, (0.0, 1.0)));
        }
        let vertices: Vec<_> = arcs.iter().flat_map(|(_, uv, range)| [uv.eval(range.0), uv.eval(range.1)])
            .map(|uv| {
                let point = world(uv);
                let label = body.new_label();
                (body.vertices.insert(Vertex { position: point, label, tol: Default::default() }), point, uv)
            }).collect();
        let mut uses = vec![];
        for (index, (curve, trim, range)) in arcs.into_iter().enumerate() {
            let a = vertices[2 * index];
            let b = vertices[2 * index + 1];
            let c = vertices[(2 * index + 2) % vertices.len()];
            let line = Curve3::Line { origin: b.1, dir: c.1 - b.1 };
            let uv_line = Curve2::Line { origin: b.2, dir: c.2 - b.2 };
            for (curve, trim, range, v0, v1, forward) in [
                (curve, trim, range, a.0, b.0, true),
                (line, uv_line, (0.0, 1.0), b.0, c.0, true),
            ] {
                let curve = body.curves3.insert(curve);
                let trim = body.curves2.insert(trim);
                let label = body.new_label();
                uses.push(((body.edges.insert(Edge { curve, range, v0, v1, label, tol: Default::default() }), forward), trim, range));
            }
        }
        let loop_id = make_loop(&mut body, face, &uses.iter().map(|row| row.0).collect::<Vec<_>>());
        body.faces.get_mut(face).unwrap().outer = Some(loop_id);
        let mut coedge = body.loops.get(loop_id).unwrap().first;
        for (_, trim, range) in uses {
            let record = body.coedges.get_mut(coedge).unwrap();
            record.pcurve = Some(trim);
            record.prange = range;
            coedge = record.next;
        }
        for query in row["queries"].as_array().unwrap() {
            let target = Pnt3::from_array(xyz(&query["target"]));
            let mut cursor = planar_projection::PlanarClosest::new(face, target);
            let point = (0..32).find_map(|_| {
                let demand = cursor.next_demand(&body).unwrap();
                assert!(demand.copy_bytes <= 128);
                cursor.step(&body).unwrap_or_else(|error| panic!("{}: {error:?}", query["id"]))
            }).unwrap();
            assert!(point.distance(Pnt3::from_array(xyz(&query["point"]))) < 1e-9, "{}: {point:?}", query["id"]);
            assert!((point.distance(target) - query["distance"].as_f64().unwrap()).abs() < 1e-9);
        }
    }
}
fn main() {
    verify_mixed_sections();
    verify_face_distances();
    verify_elliptic_trim();
    verify_spline_length();
    verify_spline_nearest();
    verify_plane_contacts();
    verify_conic_pairs(include_str!(r#"__CIRCLES__"#));
    verify_conic_pairs(include_str!(r#"__SPATIAL__"#));
    verify_conic_pairs(include_str!(r#"__SPATIAL_CONICS__"#));
    verify_conic_pairs(include_str!(r#"__COPLANAR_CONICS__"#));
    let fixture: Value = serde_json::from_str(include_str!(r#"__FIXTURE__"#)).unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let start = Pnt3::from_array(xyz(&row["line"]["start"]));
        let end = Pnt3::from_array(xyz(&row["line"]["end"]));
        let conic = &row["conic"];
        let frame = Frame3::from_x_z(Pnt3::from_array(xyz(&conic["center"])), Vec3::from_array(xyz(&conic["axis"])), Vec3::from_array(xyz(&conic["normal"]))).unwrap();
        let a = conic["radii"][0].as_f64().unwrap();
        let b = conic["radii"][1].as_f64().unwrap();
        let line = Curve3::Line { origin: start, dir: end - start };
        let conic = if a == b { Curve3::Circle { frame, radius: a } } else { Curve3::Ellipse { frame, major_radius:a, minor_radius:b } };
        let operands = [(&line, range(&row["lineRange"])), (&conic, range(&row["conicRange"]))];
        let swap = row["swap"].as_bool().unwrap_or(false);
        let [one, two] = if swap { [operands[1], operands[0]] } else { operands };
        let hits = intersect_original_curve_curve(one, two, tolerance).unwrap().unwrap();
        let expected = row["points"].as_array().unwrap();
        assert_eq!(hits.iter().flatten().count(), expected.len(), "{}: {hits:?}", row["id"]);
        if one.1.is_none() && two.1.is_none() {
            assert_eq!(intersect_curve_curve(one.0, two.0, tolerance).unwrap(), hits.into_iter().flatten().collect::<Vec<_>>());
        }
        for (hit, point) in hits.into_iter().flatten().zip(expected) {
            assert!(hit.point.distance(Pnt3::from_array(xyz(point))) <= row["pointTolerance"].as_f64().unwrap_or(tolerance), "{}: {hit:?}", row["id"]);
            assert_eq!(hit.tangent, row["tangent"].as_bool().unwrap_or(false), "{}", row["id"]);
            let (line_t, conic_t) = if swap { (hit.t_b, hit.t_a) } else { (hit.t_a, hit.t_b) };
            assert!(hit.point.distance(line.eval(line_t)) <= tolerance);
            assert!(hit.point.distance(conic.eval(conic_t)) <= tolerance);
        }
    }
}
""".replace("__FIXTURE__", str(fixture)).replace("__CIRCLES__", str(circles)).replace("__SPATIAL__", str(spatial)).replace("__SPATIAL_CONICS__", str(spatial_conics)).replace("__COPLANAR_CONICS__", str(coplanar_conics)).replace("__SPLINE_PLANE__", str(spline_plane)).replace("__CONIC_PLANE__", str(conic_plane)).replace("__NEAREST_SPLINE__", str(nearest_spline)).replace("__LENGTH_SPLINE__", str(length_spline)).replace("__ELLIPTIC_TRIM__", str(elliptic_trim)).replace("__PLANAR_FACE__", str(planar_face)).replace("__MIXED_SECTION__", str(mixed_section)).replace("__SURFACE_DISTANCE__", str(surface_distance)).replace("__ANALYTIC_NEAREST__", str(analytic_nearest)).replace("__BOUNDED_LENGTH__", str(bounded_length)).replace("__ANALYTIC_MEASURE__", measure).replace("__PLANAR__", str(planar)).replace("__LINEAR_DISTANCE__", str(linear_distance))
output = directory / "line-conic-source"
source = output.with_suffix(".rs")
source.write_text(check)
cached = tuple(Path(".cache/b/r/m/u/l/p46a1ba/prototype/build").glob("*/*/out"))
args = ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20", "rustc", "--edition=2024"]
for path in cached:
    args += ["-L", "dependency=" + str(path)]
local = {}
def dependencies(names):
    result = []
    for name in names:
        if name in local:
            result += ["--extern", name+"="+str(local[name])]
            continue
        library = max((p for d in cached for extension in ("rlib", "dylib") for p in d.glob("lib"+name+"-*."+extension)), key=lambda p: p.stat().st_size)
        result += ["--extern", name+"="+str(library)]
        if library.with_suffix(".rmeta").is_file():
            result += ["--extern", name+"="+str(library.with_suffix(".rmeta"))]
    return result

curve_name = "semio_framework_3d_brep_curve"
curve = directory / ("lib" + curve_name + ".rlib")
curve_args = ["semio_framework_3d_brep_polynomial", "semio_framework_3d_brep_vector", "semio_framework_value", "semio_framework_value_derive"]
curve_root = base.parent.parent / "📸️representation/➰️curve/🦀️.rs"
if subprocess.run(args + dependencies(curve_args) + [
    "--crate-name", curve_name, "--crate-type", "rlib", str(curve_root), "-o", str(curve),
]).returncode:
    raise SystemExit("curve library compilation failed")
local[curve_name] = curve
args += ["-L", "dependency=" + str(directory)]
representation_name = "semio_framework_3d_brep_representation"
representation = directory / ("lib" + representation_name + ".rlib")
representation_root = base.parent.parent / "📸️representation"
manifest = (representation_root / "📦️packages/🦀️rust/Cargo.toml").read_text().split("[dependencies]")[1].split("[")[0]
representation_args = tuple(line.split(".")[0].replace("-", "_") for line in manifest.splitlines() if line.strip())
representation_command = ["--edition=2021" if item == "--edition=2024" else item for item in args]
if subprocess.run(representation_command + dependencies(representation_args) + [
    "--crate-name", representation_name, "--crate-type", "rlib", str(representation_root / "🦀️.rs"), "-o", str(representation),
]).returncode:
    raise SystemExit("representation library compilation failed")
local[representation_name] = representation
geometry = ("semio_framework_3d_brep_curve", "semio_framework_3d_brep_vector")
query = directory / "libsemio_line_conic_intersect.rlib"
result = subprocess.run(args + dependencies(geometry + ("semio_framework_3d_brep_surface", "semio_framework_3d_brep_representation")) + [
    "--crate-name", "semio_line_conic_intersect", "--crate-type", "rlib", str(base / "🦀️.rs"), "-o", str(query),
])
if result.returncode:
    raise SystemExit("intersection library compilation failed")
result = subprocess.run(args + dependencies(geometry + ("serde_json", "semio_framework_value", representation_name, "semio_framework_3d_brep_surface", "semio_framework_3d_brep_construction")) + [
    "--extern", "semio_line_conic_intersect="+str(query), str(source), "-o", str(output),
])
if result.returncode:
    raise SystemExit("line/conic native witness compilation failed")
subprocess.run([str(output)], check=True)
