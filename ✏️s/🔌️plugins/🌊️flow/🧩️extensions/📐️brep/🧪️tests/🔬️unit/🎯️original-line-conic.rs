//! 🎯️ Registered line and conic contacts publish the portable points through the original source lane.
use super::*;
use semio_os_flow_geometry::GeometryWorkIdentity;
use semio_framework_3d_brep_curve::{bspline::KnotVector, Curve3};
use semio_framework_3d_brep_engine::{GeometryKind, ShapeRoot, ShapeValue, ShapeWire};
use semio_framework_3d_brep_representation::{arena::{EdgeId, VertexId}, topology::{Body, Edge, Vertex}};
use semio_framework_3d_brep_vector::{matrix::Frame3, Pnt3, Vec3};
use semio_s_spatial_kernel_semio_session::{geometry_dict, map_kernel_error, point_dictionary, read_geometry, vector_dictionary};

fn trimmed_source(session: &Session, curve: Curve3, range: &serde_json::Value) -> Value {
    let range = serde_json::from_value(range.clone()).unwrap();
    let mut body = Body::new();
    let edge = native_edge(&mut body, curve, range, None);
    import_source(session, ShapeValue { root: ShapeRoot::Edge(edge), body })
}

fn native_edge(body: &mut Body, curve: Curve3, range: (f64, f64), start: Option<VertexId>) -> EdgeId {
    let mut vertex = |parameter| {
        let label = body.new_label();
        body.vertices.insert(Vertex { position: curve.eval(parameter), label, tol: Default::default() })
    };
    let v0 = start.unwrap_or_else(|| vertex(range.0));
    let v1 = vertex(range.1);
    let curve = body.curves3.insert(curve);
    let label = body.new_label();
    body.edges.insert(Edge { curve, range, v0, v1, label, tol: Default::default() })
}

fn import_source(session: &Session, shape: ShapeValue) -> Value {
    session.with_kernel(|kernel| {
        let handle = kernel.import_shape(&shape).map_err(|error| map_kernel_error(&error))?;
        Ok(Value::Dictionary(geometry_dict(kernel, &handle)?))
    }).unwrap()
}

fn conic_source(session: &Session, conic: &serde_json::Value, range: &serde_json::Value, build: &mut impl FnMut(&str, Dictionary) -> Value) -> Value {
    let center = fixture_xyz(&conic["center"]);
    let normal = fixture_xyz(&conic["normal"]);
    let [rx, ry] = conic["radius"].as_f64().map_or_else(|| serde_json::from_value(conic["radii"].clone()).unwrap(), |radius| [radius; 2]);
    if range.is_array() {
        let frame = Frame3::from_x_z(Pnt3::from_array(center), Vec3::from_array(fixture_xyz(&conic["axis"])), Vec3::from_array(normal)).unwrap();
        let curve = if rx == ry { Curve3::Circle { frame, radius: rx } } else { Curve3::Ellipse { frame, major_radius: rx, minor_radius: ry } };
        return trimmed_source(session, curve, range);
    }
    let args = Dictionary::new().insert("center", Value::Dictionary(point_dictionary(center))).insert("normal", Value::Dictionary(vector_dictionary(normal)));
    if rx == ry { build("brep.curve.circle", args.insert("radius", number(rx))) }
    else { build("brep.curve.ellipse", args.insert("semiMajor", number(rx)).insert("semiMinor", number(ry))) }
}

#[test]
fn original_line_conic_contacts_respect_trim_and_publish_exact_points() {
    verify_contacts(include_str!("🔣️line-conic-intersections.json"), 98900);
}

#[test]
fn original_circle_pair_contacts_respect_both_periodic_trims() {
    verify_contacts(include_str!("🔣️circle-pair-intersections.json"), 99000);
}

#[test]
fn original_spatial_circle_contacts_publish_only_shared_trimmed_points() {
    verify_contacts(include_str!("🔣️spatial-circle-intersections.json"), 99100);
}

#[test]
fn original_spatial_ellipse_contacts_publish_bounded_mixed_and_elliptic_points() {
    verify_contacts(include_str!("🔣️spatial-conic-intersections.json"), 99200);
}

#[test]
fn original_coplanar_conics_publish_up_to_four_ordered_contacts() {
    verify_contacts(include_str!("🔣️coplanar-conic-intersections.json"), 99300);
}

#[test]
fn original_splines_meet_planes_only_at_retained_isolated_contacts() {
    verify_contacts(include_str!("🔣️spline-plane-intersections.json"), 99400);
}

fn retained_curve_source(session: &Session, row: &serde_json::Value, range: &serde_json::Value) -> Value {
    let curve = retained_curve(row);
    if range.is_array() { return trimmed_source(session, curve, range); }
    let mut body = Body::new();
    let label = body.new_label();
    import_source(session, ShapeValue { root: ShapeRoot::Curve { curve, label }, body })
}

fn retained_curve(row: &serde_json::Value) -> Curve3 {
    if let Some(radii) = row["ellipse"].as_array() {
        let origin = row.get("origin").map(fixture_xyz).unwrap_or_default();
        let normal = row.get("normal").map(fixture_xyz).unwrap_or([0.0, 0.0, 1.0]);
        let frame = Frame3::from_x_z(Pnt3::from_array(origin), Vec3::X, Vec3::from_array(normal)).unwrap();
        Curve3::Ellipse { frame, major_radius: radii[0].as_f64().unwrap(), minor_radius: radii[1].as_f64().unwrap() }
    } else if let Some(radius) = row["circle"].as_f64() {
        let frame = Frame3::from_x_z(Pnt3::from_array([0.0; 3]), Vec3::X, Vec3::Z).unwrap();
        Curve3::Circle { frame, radius }
    } else if let Some(line) = row["line"].as_array() {
        let origin = Pnt3::from_array(fixture_xyz(&line[0]));
        let dir = Pnt3::from_array(fixture_xyz(&line[1])) - origin;
        Curve3::Line { origin, dir }
    } else {
        let degree = row["degree"].as_u64().unwrap() as usize;
        let controls: Vec<_> = row["controls"].as_array().unwrap().iter().map(|point| Pnt3::from_array(fixture_xyz(point))).collect();
        let weights = serde_json::from_value(row["weights"].clone()).unwrap();
        let knots = KnotVector::new(serde_json::from_value(row["knots"].clone()).unwrap(), degree, controls.len()).unwrap();
        Curve3::Nurbs { knots, controls, weights }
    }
}

fn verify_contacts(fixtures: &str, operation_base: u64) {
    let fixture: serde_json::Value = serde_json::from_str(fixtures).unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    let output = fixture["output"].as_str().unwrap();
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = GeometryWorkIdentity { operation: operation_base + index as u64, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let mut build = |kind: &str, args| run_live_query(&registry, kind, args, &mut caller, &mut born, &mut free).get("curve").unwrap().clone();
        let (operator, input) = if let Some((curve, plane)) = row.get("curve").map(|curve| (curve, row.get("plane").unwrap_or(&fixture["plane"]))) {
            let curve = retained_curve_source(&session, curve, &row["range"]);
            let args = Dictionary::new().insert("origin", Value::Dictionary(point_dictionary(fixture_xyz(&plane["origin"]))))
                .insert("normal", Value::Dictionary(vector_dictionary(fixture_xyz(&plane["normal"]))));
            let surface = run_live_query(&registry, "brep.surf.plane", args, &mut caller, &mut born, &mut free).get("surface").unwrap().clone();
            ("brep.intersect.curveSurface", Dictionary::new().insert("curve", curve).insert("surface", surface))
        } else {
            let (line, conic) = if row["line"].is_object() {
                let start = fixture_xyz(&row["line"]["start"]);
                let end = fixture_xyz(&row["line"]["end"]);
                let line = if row["lineRange"].is_array() {
                    let origin = Pnt3::from_array(start);
                    let dir = Pnt3::from_array(end) - origin;
                    trimmed_source(&session, Curve3::Line { origin, dir }, &row["lineRange"])
                } else {
                    let args = Dictionary::new().insert("start", Value::Dictionary(point_dictionary(start))).insert("end", Value::Dictionary(point_dictionary(end)));
                    build("brep.curve.line", args)
                };
                (line, conic_source(&session, &row["conic"], &row["conicRange"], &mut build))
            } else {
                let a = &row["a"];
                let b = &row["b"];
                (conic_source(&session, a, &a["range"], &mut build), conic_source(&session, b, &b["range"], &mut build))
            };
            let (left, right) = if row["swap"].as_bool().unwrap_or(false) { (conic, line) } else { (line, conic) };
            ("brep.intersect.curveCurve", Dictionary::new().insert("a", left).insert("b", right))
        };
        let input = input.insert("tolerance", number(tolerance));
        let counts = || session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            Ok([kernel.live_handles().len(), body.vertices.len(), body.edges.len(), body.curves3.len()])
        }).unwrap();
        let before = counts();
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        let points = row["points"].as_array().unwrap();
        if points.is_empty() {
            let refusal = row["fault"].as_str().unwrap_or("no intersection");
            let mut job = Some(admit_live_job(&registry, operator, input));
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx), Err(error) if format!("{error:?}").contains(refusal)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert_eq!(counts(), before);
        } else {
            if index == 0 || points.len() == 4 {
                let mut draft = Some(admit_live_job(&registry, operator, input.clone()));
                for _ in 0..100 {
                    run(&mut caller, grant, &mut born, &mut free, |cx| {
                        let (status, receipt) = draft.as_mut().unwrap().step(cx).unwrap();
                        assert!(matches!(status, neural_engine::OperatorJobStep::Working(_)));
                        assert!(receipt.fits(grant));
                        Ok(())
                    });
                    if counts()[1] > before[1] { break; }
                }
                assert!(counts()[1] > before[1]);
                assert_eq!(counts()[0], before[0]);
                draft.as_mut().unwrap().cancel();
                run(&mut caller, stock(), &mut born, &mut free, |cx| {
                    assert!(matches!(draft.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                    Ok(())
                });
                close_live_job(&mut draft, &mut caller, &mut born, &mut free);
                assert_eq!(counts(), before);
            }
            let result = retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
            let handle = read_geometry(&result, output).unwrap();
            let kind = session.with_kernel_read(|kernel| Ok(kernel.kind_sync(&handle).unwrap())).unwrap();
            assert_eq!(kind, if points.len() == 1 { GeometryKind::Vertex } else { GeometryKind::Wire });
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            let (xyz, expected) = if points.len() == 1 {
                (&mesh.positions, points.iter().collect::<Vec<_>>())
            } else {
                (&mesh.edge_positions, points.windows(2).flatten().collect::<Vec<_>>())
            };
            assert_eq!(xyz.len(), 3 * expected.len());
            let point_tolerance = row["pointTolerance"].as_f64().unwrap_or(1e-5);
            for (point, expected) in xyz.chunks_exact(3).zip(expected) {
                assert!(point.iter().zip(fixture_xyz(expected)).all(|(&x, y)| (f64::from(x) - y).abs() < point_tolerance), "{}", row["id"]);
            }
            assert_eq!(mesh.component_references["vertex"].len(), points.len());
            if points.len() > 1 { assert_eq!(mesh.component_references["edge"].len(), points.len() - 1); }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_spline_projection_ports_agree_on_the_retained_global_minimum() {
    verify_retained_projection(include_str!("🔣️spline-nearest.json"), 99500);
}

#[test]
fn original_elliptic_trims_keep_nearest_parameters_and_length() {
    verify_retained_projection(include_str!("🔣️elliptic-trim.json"), 99700);
}

fn verify_retained_projection(fixture: &str, identity: u64) {
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_xyz};
    let data: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let tolerance = data["absoluteTolerance"].as_f64().unwrap();
    for (index, row) in data["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let curve = retained_curve_source(&session, &row["curve"], &row["range"]);
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity + index as u64), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let target = Value::Dictionary(point_dictionary(fixture_xyz(&row["target"])));
        for (operation, source) in [("brep.eval.curveClosestParameter", "curve"), ("brep.measure.closestPoint", "geometry")] {
            let input = Dictionary::new().insert(source, curve.clone()).insert("point", target.clone());
            if let Some(fault) = row["fault"].as_str() {
                let mut job = Some(admit_live_job(&registry, operation, input));
                run(&mut caller, stock(), &mut born, &mut free, |cx| {
                    assert!(matches!(job.as_mut().unwrap().step(cx), Err(error) if format!("{error:?}").contains(fault)));
                    Ok(())
                });
                close_live_job(&mut job, &mut caller, &mut born, &mut free);
                continue;
            }
            let result = run_live_query(&registry, operation, input, &mut caller, &mut born, &mut free);
            let near = |actual: f64, expected| assert!((actual - expected).abs() <= tolerance, "{}: {actual} != {expected}", row["id"]);
            if source == "curve" {
                near(read_channel_number(&result, "parameter").unwrap(), row["parameter"].as_f64().unwrap());
                near(read_channel_number(&result, "distance").unwrap(), row["distance"].as_f64().unwrap());
            }
            for (actual, expected) in read_xyz(&result, "pointOut").unwrap().into_iter().zip(fixture_xyz(&row["point"])) {
                near(actual, expected);
            }
        }
        if let Some(expected) = row["distance"].as_f64() {
            let output = run_live_query(&registry, "brep.util.vertex", Dictionary::new().insert("point", target), &mut caller, &mut born, &mut free);
            let vertex = output.get("vertex").unwrap();
            for (a, b) in [(&curve, vertex), (vertex, &curve)] {
                let input = Dictionary::new().insert("a", a.clone()).insert("b", b.clone());
                let result = run_live_query(&registry, "brep.measure.distance", input, &mut caller, &mut born, &mut free);
                let actual = read_channel_number(&result, "distance").unwrap();
                assert!((actual - expected).abs() <= tolerance, "{}: {actual} != {expected}", row["id"]);
            }
        }
        if let Some(expected) = row["length"].as_f64() {
            let input = Dictionary::new().insert("geometry", curve);
            if index == 0 { cancel_length_turns(&registry, input.clone(), &mut caller, &mut born, &mut free); }
            let result = run_live_query(&registry, "brep.measure.length", input, &mut caller, &mut born, &mut free);
            let actual = read_channel_number(&result, "length").unwrap();
            let length_tolerance = tolerance + data["relativeTolerance"].as_f64().unwrap() * expected;
            assert!((actual - expected).abs() <= length_tolerance, "{}: {actual} != {expected}", row["id"]);
        }
        caller.retire_exact();
    }
}

#[test]
fn original_spline_length_preserves_trims_and_resumes_under_small_grants() {
    use semio_s_spatial_kernel_semio_session::read_channel_number;
    let data: serde_json::Value = serde_json::from_str(include_str!("🔣️spline-length.json")).unwrap();
    let absolute = data["absoluteTolerance"].as_f64().unwrap();
    let relative = data["relativeTolerance"].as_f64().unwrap();
    let operator = "brep.measure.length";
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    for (index, row) in data["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let source = if let Some(wire) = row["wire"].as_array() {
            let mut body = Body::new();
            let arc = retained_curve(&wire[0]);
            let range = serde_json::from_value(wire[0]["range"].clone()).unwrap_or_else(|_| arc.domain());
            let first = native_edge(&mut body, arc, range, None);
            let start = body.edges.get(first).unwrap().v0;
            let second = native_edge(&mut body, retained_curve(&wire[1]), (0.0, 1.0), Some(start));
            let a = body.edges.get(first).unwrap();
            let b = body.edges.get(second).unwrap();
            let vertices = vec![a.v0, a.v1, b.v1];
            let edges = [first, second];
            let members = row["members"].as_array().unwrap().iter().enumerate()
                .map(|(index, id)| (edges[id.as_u64().unwrap() as usize], index != 1)).collect();
            let root = ShapeRoot::Wire(ShapeWire { members, vertices, closed: false, label: body.new_label() });
            import_source(&session, ShapeValue { root, body })
        } else {
            retained_curve_source(&session, &row["curve"], &row["range"])
        };
        let input = Dictionary::new().insert("geometry", source);
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(99600 + index as u64), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        if index == 0 { cancel_length_turns(&registry, input.clone(), &mut caller, &mut born, &mut free); }
        if let Some(fault) = row["fault"].as_str() {
            let mut job = Some(admit_live_job(&registry, operator, input));
            let mut observed = false;
            for _ in 0..8 {
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    if let Err(error) = job.as_mut().unwrap().step(cx) {
                        assert!(format!("{error:?}").contains(fault), "{}: {error:?}", row["id"]);
                        observed = true;
                    }
                    Ok(())
                });
                if observed { break; }
            }
            assert!(observed, "{}", row["id"]);
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
        } else {
            let result = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
            let expected = row["length"].as_f64().unwrap();
            let actual = read_channel_number(&result, "length").unwrap();
            let tolerance = absolute * if row["wire"].is_array() { 2.0 } else { 1.0 } + relative * expected;
            assert!((actual - expected).abs() <= tolerance, "{}: {actual} != {expected}", row["id"]);
        }
        caller.retire_exact();
    }
}

fn cancel_length_turns(registry: &Registry, input: Dictionary, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize) {
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let mut paused = Some(admit_live_job(registry, "brep.measure.length", input));
    for _ in 0..8 {
        run(caller, grant, born, free, |cx| {
            let (status, receipt) = paused.as_mut().unwrap().step(cx).unwrap();
            assert!(matches!(status, neural_engine::OperatorJobStep::Working(_)));
            assert!(receipt.fits(grant));
            Ok(())
        });
    }
    assert_eq!(paused.as_ref().unwrap().progress().phase, "bounded-length-measuring");
    paused.as_mut().unwrap().cancel();
    run(caller, stock(), born, free, |cx| {
        assert!(matches!(paused.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut paused, caller, born, free);
}

#[test]
fn retained_vertex_distances_use_analytic_surfaces_and_finite_conic_faces() {
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::read_channel_number;

    let data: serde_json::Value = serde_json::from_str(include_str!("🔣️surface-vertex-distance.json")).unwrap();
    let normal = fixture_xyz(&data["normal"]);
    for (index, row) in data["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(99800 + index as u64), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let origin = fixture_xyz(&row["origin"]);
        let scalar = |key: &str| row[key].as_f64().unwrap();
        let kind = row["kind"].as_str().unwrap();
        let shape = if kind.ends_with("-face") {
            let params = Dictionary::new().insert("center", Value::Dictionary(point_dictionary(origin)))
                .insert("normal", Value::Dictionary(vector_dictionary(normal)));
            let curve = if kind == "circle-face" {
                run_live_query(&registry, "brep.curve.circle", params.insert("radius", number(scalar("radius"))), &mut caller, &mut born, &mut free)
            } else {
                let params = params.insert("semiMajor", number(scalar("major"))).insert("semiMinor", number(scalar("minor")));
                run_live_query(&registry, "brep.curve.ellipse", params, &mut caller, &mut born, &mut free)
            };
            let input = Dictionary::new().insert("wire", curve.get("curve").unwrap().clone());
            run_live_query(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free).get("face").unwrap().clone()
        } else {
            let frame = Frame3::from_x_z(Pnt3::from_array(origin), Vec3::X, Vec3::from_array(normal)).unwrap();
            let surface = match kind {
                "plane" => Surface::Plane { frame },
                "cylinder" => Surface::Cylinder { frame, radius: scalar("radius") },
                "sphere" => Surface::Sphere { frame, radius: scalar("radius") },
                "cone" => Surface::Cone { frame, half_angle: scalar("halfAngle") },
                "torus" => Surface::Torus { frame, major_radius: scalar("major"), minor_radius: scalar("minor") },
                _ => panic!("unexpected fixture surface {kind}"),
            };
            let mut body = Body::new();
            let label = body.new_label();
            import_source(&session, ShapeValue { root: ShapeRoot::Surface { surface, label }, body })
        };
        let input = Dictionary::new().insert("point", Value::Dictionary(point_dictionary(fixture_xyz(&row["target"]))));
        let point = run_live_query(&registry, "brep.util.vertex", input, &mut caller, &mut born, &mut free).get("vertex").unwrap().clone();
        for (a, b) in [(&shape, &point), (&point, &shape)] {
            let input = Dictionary::new().insert("a", a.clone()).insert("b", b.clone());
            let result = run_live_query(&registry, "brep.measure.distance", input, &mut caller, &mut born, &mut free);
            let actual = read_channel_number(&result, "distance").unwrap();
            assert!((actual - scalar("distance")).abs() < 1e-10, "{}: {actual}", row["id"]);
        }
        caller.retire_exact();
    }
}

#[test]
fn retained_linear_distance_preserves_both_authored_domains() {
    use semio_s_spatial_kernel_semio_session::read_channel_number;

    let data: serde_json::Value = serde_json::from_str(include_str!("🔣️linear-distance.json")).unwrap();
    let tolerance = data["tolerance"].as_f64().unwrap();
    for (index, row) in data["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(99960 + index as u64), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let mut source = |name: &str| {
            let raw = &row[name];
            let origin = Pnt3::from_array(fixture_xyz(&raw["origin"]));
            let dir = Vec3::from_array(fixture_xyz(&raw["direction"]));
            if raw["range"].is_array() {
                trimmed_source(&session, Curve3::Line { origin, dir }, &raw["range"])
            } else {
                let input = Dictionary::new().insert("start", Value::Dictionary(point_dictionary(origin.to_array())))
                    .insert("end", Value::Dictionary(point_dictionary((origin + dir).to_array())));
                run_live_query(&registry, "brep.curve.line", input, &mut caller, &mut born, &mut free).get("curve").unwrap().clone()
            }
        };
        let (a, b) = (source("a"), source("b"));
        for (a, b) in [(&a, &b), (&b, &a)] {
            let input = Dictionary::new().insert("a", a.clone()).insert("b", b.clone());
            let output = retained_boolean_result(&registry, "brep.measure.distance", input, &mut caller, &mut born, &mut free);
            let actual = read_channel_number(&output, "distance").unwrap();
            let expected = row["distance"].as_f64().unwrap();
            assert!((actual-expected).abs() <= tolerance * expected.max(1.0), "{}: {actual} != {expected}", row["id"]);
        }
        caller.retire_exact();
    }
}
