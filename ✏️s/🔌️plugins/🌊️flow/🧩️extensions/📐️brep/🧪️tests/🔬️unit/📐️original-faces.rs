//! 📐️ Original BRep curve and face construction, transforms and boundary preview.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn analytic_curves_include_circle_live_registry_publication_with_paid_original_turns() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json")).unwrap();
    for name in ["line", "circle", "ellipse"] {
        let case = corpus["cases"].as_array().unwrap().iter().find(|case| case["name"] == name).unwrap();
        for cancelled in [false, true] {
            run_curve_case(case, &corpus["queries"][name], corpus["tolerance"].as_f64().unwrap(), cancelled);
        }
    }
}

#[test]
fn original_polygon_wire_face_and_extrusion_use_the_live_registry() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json")).unwrap();
    for (name, face_operator) in [
        ("extrude a rectangle wire", Some("brep.surf.planarFaceWire")),
        ("extrude a triangle along a negative oblique vector", Some("brep.util.faceFromWire")),
        ("extrude a hexagon wire", None),
        ("extrude a clockwise wire", Some("brep.util.faceFromWire")),
        ("extrude a wire in the XZ plane", Some("brep.util.faceFromWire")),
    ] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97153, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let profile = &case["inputs"]["wire"];
        let wire = run_live_query(&registry, profile["kind"].as_str().unwrap(), polygon_fixture_input(&profile["inputs"]), &mut caller, &mut born, &mut free);
        let direction = Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(fixture_xyz(&case["inputs"]["vector"])));
        let (operator, input) = if let Some(face_operator) = face_operator {
            let face = run_live_query(&registry, face_operator, Dictionary::new().insert("wire", wire.get("wire").unwrap().clone()), &mut caller, &mut born, &mut free);
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&face, "face").unwrap();
            let preview = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            preview.validate_component_references().unwrap();
            let label = session.with_kernel_read(|kernel| Ok(kernel.label_of(&handle).unwrap().0.to_string())).unwrap();
            assert_eq!(preview.component_references["face"], [label]);
            let vertices = preview.component_references["vertex"].len();
            assert!(vertices >= 3);
            assert_eq!(preview.component_references["edge"].len(), vertices);
            assert_eq!(preview.indices.len() / 3, vertices - 2);
            ("brep.sweep.extrude", Dictionary::new().insert("face", face.get("face").unwrap().clone()))
        } else {
            ("brep.solid.extrude", Dictionary::new().insert("wire", wire.get("wire").unwrap().clone()))
        };
        let output = run_live_query(&registry, operator, input.insert("vector", direction), &mut caller, &mut born, &mut free);
        let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, "solid").unwrap();
        let expected = &case["outputs"]["shape"];
        let tolerance = case["tolerance"].as_f64().unwrap_or(1e-6);
        for metric in ["volume", "area"] {
            let operator = format!("brep.measure.{metric}");
            let value = run_live_query(&registry, &operator, Dictionary::new().insert("geometry", output.get("solid").unwrap().clone()), &mut caller, &mut born, &mut free);
            let value = semio_s_spatial_kernel_semio_session::read_channel_number(&value, metric).unwrap();
            assert!((value - expected[metric].as_f64().unwrap()).abs() < tolerance, "{name} {metric}: {value}");
        }
        let preview = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        preview.validate_component_references().unwrap();
        let (area, volume) = preview_area_volume(&preview);
        assert!((area - expected["area"].as_f64().unwrap()).abs() < tolerance.max(1e-5));
        assert!((volume - expected["volume"].as_f64().unwrap()).abs() < tolerance.max(1e-5));
        assert!(!preview.edge_positions.is_empty());
        let labels = session.with_kernel_read(|kernel| {
            let solid = kernel.geometry_solid(&handle).map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))?;
            let body = kernel.representation();
            Ok(body.solid_faces(solid).into_iter().map(|id| body.faces.get(id).unwrap().label.0.to_string()).collect::<std::collections::BTreeSet<_>>())
        }).unwrap();
        assert_eq!(preview.component_references["face"].iter().cloned().collect::<std::collections::BTreeSet<_>>(), labels);
        if let Some(faces) = expected["faces"].as_u64() {
            assert_eq!(labels.len(), faces as usize, "{name}");
        }
        caller.retire(stock());
    }
}

#[test]
fn original_polygon_face_and_prism_cancel_without_publishing_partial_topology() {
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97154), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let wire = run_live_query(&registry, "brep.curve.polygon", Dictionary::new().insert("radius", number(2.)).insert("sides", number(6.)), &mut caller, &mut born, &mut free);
    let handle = semio_s_spatial_kernel_semio_session::read_geometry(&wire, "wire").unwrap();
    let source = wire.get("wire").unwrap().clone();
    let vector = Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary([1., 0., -2.]));
    for (operator, input, turns) in [
        ("brep.surf.planarFaceWire", Dictionary::new().insert("wire", source.clone()), 26),
        ("brep.solid.extrude", Dictionary::new().insert("wire", source).insert("vector", vector), 30),
    ] {
        let mut job = Some(admit_live_job(&registry, operator, input));
        for _ in 0..turns {
            run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                assert!(receipt.copied_items <= 1);
                Ok(())
            });
        }
        job.as_mut().unwrap().cancel();
        run(&mut caller, stock(), &mut born, &mut free, |cx| {
            let (step, _) = job.as_mut().unwrap().step(cx).unwrap();
            assert!(matches!(step, neural_engine::OperatorJobStep::Cancelled(_)));
            Ok(())
        });
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap(), std::collections::HashSet::from([handle.as_str().to_owned()]));
    }
    caller.retire(stock());
}

#[test]
fn original_helix_keeps_the_cold_wire_pitch_and_handedness() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json")).unwrap();
    let cases = fixture["originalHelix"].as_array().unwrap();
    for (index, case) in cases.iter().chain(std::iter::once(&cases[0])).enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97214, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let input = ["origin", "axis"].into_iter().fold(Dictionary::new(), |input, port| {
            input.insert(port, Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(&case[port]))))
        });
        let input = ["radius", "pitch", "turns"].into_iter().fold(input, |input, port| input.insert(port, number(case[port].as_f64().unwrap())));
        if index == cases.len() {
            let mut job = Some(admit_live_job(&registry, "brep.curve.helix", input));
            let mut interrupted = false;
            for _ in 0..600 {
                let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.fits(grant));
                    Ok(())
                });
                interrupted = session.with_kernel_read(|kernel| {
                    assert!(kernel.live_handles().is_empty());
                    Ok(!kernel.representation().edges.is_empty())
                }).unwrap();
                if interrupted { break; }
            }
            assert!(interrupted);
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            session.with_kernel_read(|kernel| {
                assert!(kernel.live_handles().is_empty());
                let body = kernel.representation();
                assert!(body.vertices.is_empty() && body.edges.is_empty() && body.curves3.is_empty());
                Ok(())
            }).unwrap();
        } else {
            let output = run_live_query(&registry, "brep.curve.helix", input, &mut caller, &mut born, &mut free);
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, "curve").unwrap();
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            let segments = case["segments"].as_u64().unwrap() as usize;
            assert_eq!(mesh.edge_positions.len(), segments * 6);
            assert_eq!(mesh.component_references["edge"].len(), segments);
            assert_eq!(mesh.component_references["vertex"].len(), segments + 1);
            let measures = run_live_query(&registry, "brep.measure.length", Dictionary::new().insert("geometry", output.get("curve").unwrap().clone()),
                &mut caller, &mut born, &mut free);
            let length = semio_s_spatial_kernel_semio_session::read_channel_number(&measures, "length").unwrap();
            assert!((length - case["length"].as_f64().unwrap()).abs() < 1e-9);
            for sample in case["samples"].as_array().unwrap() {
                let n = sample[0].as_u64().unwrap() as usize;
                let i = if n == segments { 6 * n - 3 } else { 6 * n };
                let expected = fixture_xyz(&sample[1]);
                assert!(mesh.edge_positions[i..i + 3].iter().zip(expected).all(|(a,b)| (f64::from(*a)-b).abs() < 1e-6));
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_polygonal_wire_recipes_match_the_shared_curve_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json")).unwrap();
    for name in ["rectangle", "regular hexagon", "open polyline", "polyline that returns to its start closes"] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97152, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let wire = run_live_query(&registry, case["kind"].as_str().unwrap(), polygon_fixture_input(&case["inputs"]), &mut caller, &mut born, &mut free);
        let handle = semio_s_spatial_kernel_semio_session::read_geometry(&wire, "wire").unwrap();
        let input = Dictionary::new().insert("geometry", wire.get("wire").unwrap().clone());
        let measured = run_live_query(&registry, "brep.measure.length", input, &mut caller, &mut born, &mut free);
        let length = semio_s_spatial_kernel_semio_session::read_channel_number(&measured, "length").unwrap();
        let preview = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        preview.validate_component_references().unwrap();
        let reference = &fixture["wirePreview"][name];
        assert_eq!(preview.edge_positions.len() / 6, reference["segments"].as_u64().unwrap() as usize);
        assert_eq!(preview.component_references["edge"].len(), reference["edgeLabels"].as_u64().unwrap() as usize);
        assert_eq!(preview.component_references["vertex"].len(), reference["vertexLabels"].as_u64().unwrap() as usize);
        let closed = preview.edge_positions[..3] == preview.edge_positions[preview.edge_positions.len() - 3..];
        let label = preview.component_references["edge"][0].parse::<u64>().unwrap();
        let edge = session.with_kernel(|kernel| {
            use semio_framework_3d_brep_representation::topology::history::PersistentLabel;
            Ok(kernel.handle_for_label(PersistentLabel(label)).unwrap())
        }).unwrap();
        let edge_preview = run_funded_preview(&session, identity, edge.as_str(), &mut caller, &mut born, &mut free);
        edge_preview.validate_component_references().unwrap();
        assert_eq!(edge_preview.component_references["edge"], [label.to_string()]);
        assert_eq!(edge_preview.component_references["vertex"].len(), 2);
        assert_eq!(edge_preview.edge_positions.as_slice(), &preview.edge_positions[..6]);
        let bounds = preview.aabb();
        let expected_bounds = &case["outputs"]["shape"]["bbox"];
        for (side, actual) in [(0, bounds.0), (1, bounds.1)] {
            let expected = fixture_xyz(&expected_bounds[side]);
            assert!((0..3).all(|axis| (expected[axis] - actual[axis] as f64).abs() < 1e-6), "{name} bounds");
        }
        assert!((length - case["outputs"]["shape"]["length"].as_f64().unwrap()).abs() < 1e-6, "{name}");
        assert_eq!(closed, case["outputs"]["shape"]["closed"].as_bool().unwrap(), "{name}");
        caller.retire(stock());
    }
}

#[test]
fn circle_live_registry_planar_fills_publish_exact_circular_and_elliptical_faces() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    for (name, face_operator) in [
        ("fill a circular curve on a translated XY plane", "brep.surf.planarFaceWire"),
        ("fill an elliptical curve in the YZ plane", "brep.util.faceFromWire"),
    ] {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(97151), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let profile = &case["inputs"]["wire"];
        let mut parameters = Dictionary::new();
        for (key, value) in profile["inputs"].as_object().unwrap() {
            let value = if value.is_array() {
                Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(value)))
            } else {
                number(value.as_f64().unwrap())
            };
            parameters = parameters.insert(key.as_str(), value);
        }
        let curve = run_live_query(&registry, profile["kind"].as_str().unwrap(), parameters, &mut caller, &mut born, &mut free);
        let face = run_live_query(&registry, face_operator, Dictionary::new().insert("wire", curve.get("curve").unwrap().clone()), &mut caller, &mut born, &mut free);
        let handle = semio_s_spatial_kernel_semio_session::read_geometry(&face, "face").unwrap();
        let area = session.with_kernel_read(|kernel| kernel.area_sync(&handle).map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))).unwrap();
        assert!((area - case["outputs"]["shape"]["area"].as_f64().unwrap()).abs() < case["tolerance"].as_f64().unwrap(), "{name}");
        caller.retire(stock());
    }
}

#[test]
fn original_interpolated_curves_pass_through_centripetal_samples_and_preview() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json")).unwrap();
    for (name, cancelled) in [
        ("interpolated line through two points", false),
        ("interpolated quadratic with unequal chords", false),
        ("interpolated cubic through four points", false),
        ("interpolated cubic with interior knots", false),
        ("interpolated cubic through four points", true),
    ] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let input = polygon_fixture_input(&case["inputs"]);
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97157, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);

        if cancelled {
            let mut job = Some(admit_live_job(&registry, "brep.curve.interpolate", input));
            for _ in 0..12 {
                run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.copied_items <= 1);
                    Ok(())
                });
            }
            assert!(born > 0, "cancellation follows funded knot storage");
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().is_empty())).unwrap());
        } else {
            let curve = run_live_query(&registry, "brep.curve.interpolate", input, &mut caller, &mut born, &mut free);
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&curve, "curve").unwrap();
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            assert!(!mesh.edge_positions.is_empty(), "{name}");
            let raw_points = case["inputs"]["points"].as_array().unwrap();
            let parameters = fixture["interpolationParameters"][name].as_array().unwrap();
            for (raw, parameter) in raw_points.iter().zip(parameters) {
                let expected = fixture_xyz(raw);
                let t = parameter.as_f64().unwrap();
                let input = Dictionary::new().insert("curve", curve.get("curve").unwrap().clone()).insert("parameter", number(t));
                let result = run_live_query(&registry, "brep.eval.curvePoint", input, &mut caller, &mut born, &mut free);
                let point = semio_s_spatial_kernel_semio_session::read_xyz(&result, "point").unwrap();
                assert!((0..3).all(|axis| (point[axis] - expected[axis]).abs() < 1e-8), "{name} at {t}: {point:?}");
                assert!(line_preview_distance(&mesh.edge_positions, expected) <= 0.025, "{name} preview at {t}");
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_approximate_curve_matches_the_pinned_least_squares_spline() {
    use semio_framework_3d_brep_engine::ShapeRoot;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/〰️brep-curve/🔣️.json")).unwrap();
    let cases = fixture["originalApproximation"].as_array().unwrap();
    for (index, case) in cases.iter().chain(std::iter::once(&cases[2])).enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97213, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let points = case["points"].as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"), |list, (i, position)| {
            list.insert(i.to_string(), Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(position))))
        });
        let input = Dictionary::new()
            .insert("points", Value::Dictionary(points))
            .insert("degree", number(case["degree"].as_f64().unwrap()))
            .insert("controlPoints", number(case["controlPoints"].as_f64().unwrap()));
        if index == cases.len() {
            let mut job = Some(admit_live_job(&registry, "brep.curve.approximate", input));
            for _ in 0..25 {
                run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.copied_items <= 1);
                    Ok(())
                });
            }
            assert!(born > 0);
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().is_empty())).unwrap());
        } else {
            let result = run_live_query(&registry, "brep.curve.approximate", input, &mut caller, &mut born, &mut free);
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&result, "curve").unwrap();
            let shape = session.with_kernel_read(|kernel| kernel.export_shape(&handle)
                .map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))).unwrap();
            let ShapeRoot::Curve { curve, .. } = shape.root else { panic!("expected a curve shape"); };
            assert!(curve.original_spline_span_range().is_some());
            let spline = curve.to_nurbs(curve.domain());
            let (knots, controls, weights) = (spline.knots, spline.controls, spline.weights);
            let expected_knots: Vec<f64> = serde_json::from_value(case["knots"].clone()).unwrap();
            let expected_controls: Vec<[f64;3]> = serde_json::from_value(case["controls"].clone()).unwrap();
            assert_eq!(knots.degree + controls.len() + 1, knots.knots.len());
            assert_eq!(knots.knots.len(), expected_knots.len());
            assert_eq!(controls.len(), expected_controls.len());
            assert_eq!(weights, vec![1.; controls.len()]);
            for (actual, expected) in knots.knots.iter().zip(expected_knots) { assert!((actual - expected).abs() < 1e-12); }
            for (actual, expected) in controls.iter().zip(expected_controls) {
                assert!(actual.to_array().iter().zip(expected).all(|(a,b)| (a-b).abs() < 1e-8));
            }
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            for witness in case["samples"].as_array().unwrap() {
                let t = witness[0].as_f64().unwrap();
                let expected = fixture_xyz(&witness[1]);
                let query = Dictionary::new().insert("curve", result.get("curve").unwrap().clone()).insert("parameter", number(t));
                let evaluated = run_live_query(&registry, "brep.eval.curvePoint", query, &mut caller, &mut born, &mut free);
                let point = semio_s_spatial_kernel_semio_session::read_xyz(&evaluated, "point").unwrap();
                assert!(point.iter().zip(expected).all(|(a,b)| (a-b).abs() < 1e-8));
                assert!(line_preview_distance(&mesh.edge_positions, expected) <= 0.025);
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_point_list_face_publishes_only_the_face_and_retires_both_drafts() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    let witness = &fixture["pointFacePreview"];
    for (name, cancelled) in [
        ("face from the corners of a rectangle", false),
        ("face from a closed point list", false),
        ("face from a triangle in the XZ plane", false),
        ("face from a concave pentagon", false),
        ("face from a concave pentagon", true),
        ("face from points that leave their plane", false),
        ("face from collinear points", false),
        ("face from two points", false),
    ] {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97212, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let case = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"].as_str() == Some(name)).unwrap();
        let points = case["inputs"]["points"].as_array().unwrap();
        let list = points.iter().enumerate().fold(Dictionary::with_schema("list"), |list, (i, point)| {
            list.insert(i.to_string(), Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(point))))
        });
        let mut job = Some(admit_live_job(&registry, "brep.surf.planarFace", Dictionary::new().insert("points", Value::Dictionary(list))));
        let mut output = None;
        let mut refused = false;
        let mut both_drafts = false;
        for _ in 0..1000 {
            let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                match job.as_mut().unwrap().step(cx) {
                    Ok((neural_engine::OperatorJobStep::Working(_), receipt)) => assert!(receipt.fits(grant)),
                    Ok((neural_engine::OperatorJobStep::Done(value), receipt)) => {
                        assert!(receipt.fits(grant));
                        output = Some(value);
                    }
                    Err(_) => refused = true,
                    _ => panic!("{name} stopped early"),
                }
                Ok(())
            });
            if output.is_some() || refused { break; }
            both_drafts = session.with_kernel_read(|kernel| {
                assert!(kernel.live_handles().is_empty());
                let body = kernel.representation();
                Ok(!body.faces.is_empty() && !body.coedges.is_empty() && !body.edges.is_empty())
            }).unwrap();
            if cancelled && both_drafts {
                caller.cancel().cancel_now();
                run(&mut caller, stock(), &mut born, &mut free, |cx| {
                    assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                    Ok(())
                });
                break;
            }
        }
        let invalid = case.get("fault").is_some();
        assert_eq!(invalid, refused, "{name}");
        if cancelled { assert!(both_drafts); }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        if invalid || cancelled {
            assert!(output.is_none(), "{name}");
            session.with_kernel_read(|kernel| {
                let body = kernel.representation();
                assert!(kernel.live_handles().is_empty());
                assert!(body.vertices.is_empty() && body.edges.is_empty() && body.curves3.is_empty());
                assert!(body.faces.is_empty() && body.loops.is_empty() && body.coedges.is_empty());
                assert!(body.surfaces.is_empty() && body.curves2.is_empty());
                Ok(())
            }).unwrap();
        } else {
            let face = output.expect(name);
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&face, "face").unwrap();
            assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap(), std::collections::HashSet::from([handle.as_str().to_owned()]));
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            let edges = points.len() - usize::from(points.first() == points.last());
            assert_eq!(mesh.component_references["vertex"].len(), edges);
            assert_eq!(mesh.component_references["edge"].len(), edges);
            assert_eq!(mesh.component_references["face"].len(), 1);
            let expected = case["outputs"]["shape"]["area"].as_f64().unwrap();
            assert!((preview_area_volume(&mesh).0 - expected).abs() < 1e-6);
            if witness["name"].as_str() == Some(name) {
                assert_eq!(mesh.indices.len() / 3, witness["triangles"].as_u64().unwrap() as usize);
                assert!(preview_distance(&mesh, fixture_xyz(&witness["inside"])) < 1e-6);
                let distance = preview_distance(&mesh, fixture_xyz(&witness["notch"]));
                assert!((f64::from(distance) - witness["notchDistance"].as_f64().unwrap()).abs() < 1e-6);
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_coons_fits_oriented_boundary_samples_and_previews_the_published_surface() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    for (name, cancelled) in [
        ("bilinear Coons patch with mixed straight boundary degrees", false),
        ("coons patch between four boundaries", false),
        ("coons patch between four boundaries", true),
        ("coons patch whose corners do not meet", false),
    ] {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97156, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let boundaries = ["bottom", "right", "top", "left"];
        let source = &case["inputs"];
        let nested = boundaries.into_iter().enumerate().fold(Dictionary::with_schema("list"), |list, (index, key)| {
            let row = source[key].as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"), |row, (point, position)| {
                row.insert(point.to_string(), Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(position))))
            });
            list.insert(index.to_string(), Value::Dictionary(row))
        });
        let input = Dictionary::new().insert("curves", Value::Dictionary(nested));
        if cancelled || case.get("fault").is_some() {
            let mut job = Some(admit_live_job(&registry, "brep.surf.coons", input));
            let mut refused = false;
            let turns = if cancelled { 25 } else { 100 };
            for _ in 0..turns {
                run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                    match job.as_mut().unwrap().step(cx) {
                        Ok((neural_engine::OperatorJobStep::Working(_), receipt)) => assert!(receipt.copied_items <= 1),
                        Err(_) if !cancelled => refused = true,
                        _ => panic!("{name} must not publish a surface"),
                    }
                    Ok(())
                });
                if refused { break; }
            }
            if cancelled {
                assert!(born > 0, "cancellation follows the first funded spline buffer");
                job.as_mut().unwrap().cancel();
                run(&mut caller, stock(), &mut born, &mut free, |cx| {
                    assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                    Ok(())
                });
            } else {
                assert!(refused, "disconnected Coons boundaries must be refused");
            }
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().is_empty())).unwrap());
        } else {
            let surface = run_live_query(&registry, "brep.surf.coons", input, &mut caller, &mut born, &mut free);
            let shape = semio_s_spatial_kernel_semio_session::read_geometry(&surface, "surface").unwrap();
            let mesh = run_funded_preview(&session, identity, shape.as_str(), &mut caller, &mut born, &mut free);
            assert!(!mesh.indices.is_empty());
            assert_eq!(mesh.normals.len(), mesh.positions.len());
            let expected = &case["outputs"]["shape"];
            let queries = [
                (0., 0., &expected["corners"][0]), (1., 0., &expected["corners"][1]),
                (0., 1., &expected["corners"][2]), (1., 1., &expected["corners"][3]),
                (0.5, 0.5, &expected["center"]), (0.5, 1., &source["top"][1]),
            ];
            for (u, v, raw) in queries {
                let expected = fixture_xyz(raw);
                let input = Dictionary::new().insert("surface", surface.get("surface").unwrap().clone())
                    .insert("u", number(u)).insert("v", number(v));
                let sample = run_live_query(&registry, "brep.eval.surfPoint", input, &mut caller, &mut born, &mut free);
                let actual = semio_s_spatial_kernel_semio_session::read_xyz(&sample, "point").unwrap();
                assert!((0..3).all(|axis| (actual[axis] - expected[axis]).abs() < 1e-8), "{name} at {u}, {v}: {actual:?}");
                assert!(preview_distance(&mesh, expected) <= 0.025, "{name} preview at {u}, {v}");
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_nurbs_grid_interpolates_samples_and_cancels_before_publication() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    for (name, cancel) in [
        ("bilinear grid of four samples", false),
        ("grid of three by three points", false),
        ("grid of two by three points", false),
        ("bicubic grid of six by six planar samples", false),
        ("grid of three by three points", true),
    ] {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97155, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let parameters = polygon_fixture_input(&case["inputs"]);
        if cancel {
            let mut job = Some(admit_live_job(&registry, "brep.surf.nurbsGrid", parameters));
            for _ in 0..42 {
                run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.copied_items <= 1);
                    Ok(())
                });
            }
            assert!(born > 0, "cancellation follows original knot storage allocation");
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().is_empty())).unwrap());
        } else {
            let surface = run_live_query(&registry, "brep.surf.nurbsGrid", parameters, &mut caller, &mut born, &mut free);
            let shape = semio_s_spatial_kernel_semio_session::read_geometry(&surface, "surface").unwrap();
            let preview = run_funded_preview(&session, identity, shape.as_str(), &mut caller, &mut born, &mut free);
            assert!(!preview.indices.is_empty());
            assert_eq!(preview.indices.len() % 3, 0);
            assert_eq!(preview.normals.len(), preview.positions.len());
            assert!(preview.normals.chunks_exact(3).all(|normal| {
                normal.iter().all(|value| value.is_finite()) && normal.iter().map(|value| value * value).sum::<f32>() > 0.99
            }));
            let expected = &case["outputs"]["shape"];
            let queries = [
                (0., 0., &expected["corners"][0]), (0., 1., &expected["corners"][1]),
                (1., 0., &expected["corners"][2]), (1., 1., &expected["corners"][3]),
                (0.5, 0.5, &expected["center"]),
            ];
            for (u, v, expected) in queries {
                let input = Dictionary::new().insert("surface", surface.get("surface").unwrap().clone())
                    .insert("u", number(u)).insert("v", number(v));
                let point = run_live_query(&registry, "brep.eval.surfPoint", input, &mut caller, &mut born, &mut free);
                let actual = semio_s_spatial_kernel_semio_session::read_xyz(&point, "point").unwrap();
                let expected = fixture_xyz(expected);
                assert!((0..3).all(|axis| (actual[axis] - expected[axis]).abs() <= 1e-8), "{name} at {u}, {v}: {actual:?}");
                assert!(preview_distance(&preview, expected) <= 0.025, "{name} preview at {u}, {v}");
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn retained_affine_operators_transform_original_line_parameterization() {
    let fixture = evaluate_fixture();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97152), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let point = |axes| Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(axes));
    let vector = |axes| Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(axes));
    let number = |value| Value::Dictionary(semio_s_spatial_kernel_semio_session::number_dictionary(value));
    let source = run_live_query(&registry, "brep.curve.line", Dictionary::new().insert("start",point([1.0,1.0,0.0])).insert("end",point([3.0,1.0,0.0])), &mut caller, &mut born, &mut free);
    for row in fixture["analyticTransforms"].as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        let mut input = Dictionary::new().insert("geometry", source.get("curve").unwrap().clone());
        if let Some(channel) = row["channel"].as_str() { input = input.insert(channel, vector(fixture_xyz(&row["vector"]))); }
        if !row["origin"].is_null() { input = input.insert(if name=="scale" {"center"} else {"origin"}, point(fixture_xyz(&row["origin"]))); }
        if let Some(angle) = row["angle"].as_f64() { input = input.insert("angle", number(angle)); }
        let output = run_live_query(&registry,&format!("brep.xform.{name}"),input,&mut caller,&mut born,&mut free);
        let shape = semio_s_spatial_kernel_semio_session::read_geometry(&output,"geometryOut").unwrap();
        let actual = session.original_curve_sample(shape.as_str(),0.5,false).unwrap();
        for (got, expected) in actual.into_iter().zip(fixture_xyz(&row["expected"])) { assert!((got-expected).abs() < 1e-12, "{name}"); }
        assert_ne!(shape.as_str(),semio_s_spatial_kernel_semio_session::read_geometry(&source,"curve").unwrap().as_str());
    }
    caller.retire_exact();
}

#[test]
fn conic_face_publishes_area_and_preview_from_the_live_registry() {
    use flow_extension_sdk::geometry::{GeometryStep,GeometryWorkIdentity};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🧪️tests/🧫️fixtures/curve-preview.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97157),Generation(17),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    let case=&fixture[1];
    let shape=run_live_query(&registry,"brep.curve.ellipse",Dictionary::new()
        .insert("center",Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary([0.0;3])))
        .insert("normal",Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary([0.0,0.0,1.0])))
        .insert("semiMajor",number(case["major"].as_f64().unwrap())).insert("semiMinor",number(case["minor"].as_f64().unwrap())),&mut caller,&mut born,&mut free);
    let face=run_live_query(&registry,"brep.surf.planarFaceWire",Dictionary::new().insert("wire",shape.get("curve").unwrap().clone()),&mut caller,&mut born,&mut free);
    let area=run_live_query(&registry,"brep.measure.area",Dictionary::new().insert("geometry",face.get("face").unwrap().clone()),&mut caller,&mut born,&mut free);
    let exact=semio_s_spatial_kernel_semio_session::read_channel_number(&area,"area").unwrap();
    assert!((exact-case["diskArea"].as_f64().unwrap()).abs()<1e-12);
    let handle=semio_s_spatial_kernel_semio_session::read_geometry(&face,"face").unwrap();
    let id=GeometryWorkIdentity {operation:97157,generation:17};
    let tolerance=case["deflection"].as_f64().unwrap();
    let mut mesh=None;
    for _ in 0..2000 {
        let d=session.tessellation_demands(id,handle.as_str(),tolerance,128).unwrap();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(128),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        run(&mut caller,grant,&mut born,&mut free,|cx|{
            match session.tessellate_step(id,handle.as_str(),tolerance,cx).unwrap().0 {
                GeometryStep::Ready(value)=>mesh=Some(value),
                GeometryStep::Working{..}=>{},
                _=>panic!("retained conic face preview refused"),
            }
            Ok(())
        });
        if mesh.is_some(){break}
    }
    let mesh=mesh.expect("live conic face tessellates");
    mesh.validate_component_references().unwrap();
    let label=session.with_kernel_read(|kernel|Ok(kernel.label_of(&handle).unwrap().0.to_string())).unwrap();
    assert_eq!(mesh.component_references["face"],[label]);
    assert_eq!(mesh.component_references["edge"].len(),1);
    assert_eq!(mesh.component_references["vertex"].len(),1);
    assert_eq!(mesh.vertex_ids.iter().filter(|id|**id==0).count(),1);
    assert!(mesh.indices.len()/3>=case["minimumSegments"].as_u64().unwrap() as usize);
    assert_eq!(&mesh.edge_positions[..3],&mesh.edge_positions[mesh.edge_positions.len()-3..]);
    caller.retire_exact();
}

#[test]
fn retained_arc_admits_its_controls_and_previews_from_the_live_registry() {
    use flow_extension_sdk::geometry::{GeometryStep,GeometryWorkIdentity};
    let f:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/➰️curve/🧪️tests/🧫️fixtures/rational-jets.json")).unwrap();
    let row=&f[0];
    let arc=&row["arc"];
    let radius=arc["radius"].as_f64().unwrap();
    let start=arc["start"].as_f64().unwrap();
    let end=arc["end"].as_f64().unwrap();
    let center=fixture_xyz(&arc["center"]);
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let input=||Dictionary::new()
        .insert("center",Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(center)))
        .insert("normal",Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(fixture_xyz(&arc["normal"]))))
        .insert("radius",number(radius)).insert("startAngle",number(start)).insert("endAngle",number(end));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97158),Generation(17),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    let curve=run_live_query(&registry,"brep.curve.arc",input(),&mut caller,&mut born,&mut free);
    let handle=semio_s_spatial_kernel_semio_session::read_geometry(&curve,"curve").unwrap();
    let sample=|u|session.original_curve_sample(handle.as_str(),u,false).unwrap();
    let startpoint=sample(start);
    let endpoint=sample(end);
    let midpoint=sample((start+end)*0.5);
    for axis in 0..3 {
        let expected=center[axis]+((startpoint[axis]-center[axis])+(endpoint[axis]-center[axis]))*std::f64::consts::FRAC_1_SQRT_2;
        assert!((midpoint[axis]-expected).abs()<1e-12);
    }
    let id=GeometryWorkIdentity{operation:97158,generation:17};
    let tolerance=row["previewDeflection"].as_f64().unwrap();
    let mut result=None;
    for _ in 0..2000 {
        let d=session.tessellation_demands(id,handle.as_str(),tolerance,128).unwrap();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:d.copy_bytes.max(128),maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        run(&mut caller,grant,&mut born,&mut free,|cx| {
            match session.tessellate_step(id,handle.as_str(),tolerance,cx).unwrap().0 {
                GeometryStep::Ready(mesh)=>result=Some(mesh),
                GeometryStep::Working{..}=>{},
                _=>panic!("retained rational arc preview refused"),
            }
            Ok(())
        });
        if result.is_some(){break;}
    }
    let mesh=result.expect("arc preview completed");
    for (position,expected) in [&mesh.edge_positions[..3],&mesh.edge_positions[mesh.edge_positions.len()-3..]].into_iter().zip([startpoint,endpoint]) {
        assert!(position.iter().zip(expected).all(|(a,b)|(*a as f64-b).abs()<1e-6));
    }
    assert!(mesh.edge_positions.len()>6);
    let mut canceled=Some(admit_live_job(&registry,"brep.curve.arc",input()));
    for _ in 0..5 {
        run(&mut caller,Grant{maximum_items:1,maximum_copy_bytes:128,..stock()},&mut born,&mut free,|cx| {
            canceled.as_mut().unwrap().step(cx).unwrap();
            Ok(())
        });
    }
    canceled.as_mut().unwrap().cancel();
    close_live_job(&mut canceled,&mut caller,&mut born,&mut free);
    caller.retire_exact();
}

#[test]
fn original_conic_face_offset_preserves_trim_and_area_in_the_translated_frame() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-offset-face.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97168), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    for row in fixture["cases"].as_array().unwrap() {
        let center = fixture_xyz(&row["center"]);
        let target = fixture_xyz(&row["target"]);
        let normal = fixture_xyz(&row["normal"]);
        let inputs = Dictionary::new().insert("center", Value::Dictionary(point_dictionary(center)))
            .insert("normal", Value::Dictionary(vector_dictionary(normal)));
        let major = number(row["radii"][0].as_f64().unwrap());
        let minor = number(row["radii"][1].as_f64().unwrap());
        let (constructor, inputs) = if row["kind"] == "circle" {
            ("brep.curve.circle", inputs.insert("radius", major))
        } else {
            ("brep.curve.ellipse", inputs.insert("semiMajor", major).insert("semiMinor", minor))
        };
        let curve = retained_boolean_result(&registry, constructor, inputs, &mut caller, &mut born, &mut free);
        let input = Dictionary::new().insert("wire", curve.get("curve").unwrap().clone());
        let face = retained_boolean_result(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free);
        let source = read_geometry(&face, "face").unwrap();
        let input = Dictionary::new().insert("face", face.get("face").unwrap().clone())
            .insert("distance", number(row["distance"].as_f64().unwrap()));
        let result = retained_boolean_result(&registry, fixture["operator"].as_str().unwrap(), input, &mut caller, &mut born, &mut free);
        let offset = read_geometry(&result, "faceOut").unwrap();
        assert_ne!(source, offset);
        session.with_kernel_read(|kernel| {
            use semio_framework_3d_brep_representation::topology::EntityRef;
            let body = kernel.representation();
            let inspect = |handle| {
                let label = kernel.label_of(handle).unwrap();
                let (id, face) = body.faces.iter().find(|(_, face)| face.label == label).unwrap();
                let surface = body.surfaces.get(face.surface).unwrap();
                let origin = surface.eval(0.0, 0.0);
                let reach = body.reachable_from(&[EntityRef::Face(id)]);
                assert_eq!((reach.vertices.len(), reach.edges.len(), reach.faces.len()), (1, 1, 1));
                let seam = body.vertices.get(*reach.vertices.iter().next().unwrap()).unwrap().position;
                ([origin.x, origin.y, origin.z], seam - origin)
            };
            let (old_center, old_seam) = inspect(&source);
            let (new_center, new_seam) = inspect(&offset);
            assert_eq!(old_center, center);
            assert_eq!(new_center, target);
            assert_eq!(old_seam, new_seam);
            Ok(())
        }).unwrap();
        let area = Dictionary::new().insert("geometry", result.get("faceOut").unwrap().clone());
        let measured = retained_boolean_result(&registry, "brep.measure.area", area, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&measured, "area").unwrap() - row["area"].as_f64().unwrap()).abs() < 1e-12);
    }
    caller.retire(stock());
}

#[test]
fn original_standalone_affine_images_preserve_cold_parameter_samples() {
    use semio_framework_3d_brep_engine::{ShapeRoot, ShapeValue};
    use semio_framework_3d_brep_representation::topology::{Body, history::PersistentLabel};
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::matrix::Frame3;
    use semio_s_spatial_kernel_semio_session::{geometry_dict, map_kernel_error, point_dictionary, read_geometry, read_xyz, vector_dictionary};
    let fixture = evaluate_fixture();
    let rows = fixture["standaloneAffineTransforms"].as_array().unwrap();
    let ellipse = rows.iter().find(|row| row["kind"] == "ellipse").unwrap();
    for (index, row) in rows.iter().chain(std::iter::once(ellipse)).enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97219, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let kind = row["kind"].as_str().unwrap();
        let is_curve = matches!(kind, "circle" | "ellipse");
        let (source, payload) = if is_curve {
            let mut input = Dictionary::new()
                .insert("center", Value::Dictionary(point_dictionary(fixture_xyz(&row["center"]))))
                .insert("normal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
            let radius = number(row["radii"][0].as_f64().unwrap());
            let operator = if kind == "circle" {
                input = input.insert("radius", radius);
                "brep.curve.circle"
            } else {
                input = input.insert("semiMajor", radius).insert("semiMinor", number(row["radii"][1].as_f64().unwrap()));
                "brep.curve.ellipse"
            };
            let values = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
            (read_geometry(&values, "curve").unwrap(), values.get("curve").unwrap().clone())
        } else {
            let frame = Frame3::WORLD;
            let scalar = |key| row[key].as_f64().unwrap();
            let surface = match kind {
                "cylinder" => Surface::Cylinder { frame, radius: scalar("radius") },
                "cone" => Surface::Cone { frame, half_angle: scalar("halfAngle") },
                "sphere" => Surface::Sphere { frame, radius: scalar("radius") },
                "torus" => Surface::Torus { frame, major_radius: scalar("major"), minor_radius: scalar("minor") },
                _ => panic!("unsupported analytic fixture"),
            };
            session.with_kernel(|kernel| {
                let handle = kernel.import_shape(ShapeValue {
                    root: ShapeRoot::Surface { surface, label: PersistentLabel(1) }, body: Body::new(),
                }).map_err(|error| map_kernel_error(&error))?;
                let payload = geometry_dict(kernel, &handle)?;
                Ok((handle, Value::Dictionary(payload)))
            }).unwrap()
        };
        let operation = row["operation"].as_str().unwrap();
        let operator = format!("brep.xform.{operation}");
        let origin = fixture_xyz(&row["origin"]);
        let vector = fixture_xyz(&row["vector"]);
        let input = Dictionary::new().insert("geometry", payload);
        let input = match operation {
            "scale" => input.insert("center", Value::Dictionary(point_dictionary(origin))).insert("factor", Value::Dictionary(vector_dictionary(vector))),
            "mirror" => input.insert("origin", Value::Dictionary(point_dictionary(origin))).insert("normal", Value::Dictionary(vector_dictionary(vector))),
            _ => input.insert("origin", Value::Dictionary(point_dictionary(origin))).insert("axis", Value::Dictionary(vector_dictionary(vector)))
                .insert("angle", number(row["angle"].as_f64().unwrap())),
        };
        if index == rows.len() {
            let mut job = Some(admit_live_job(&registry, &operator, input));
            for _ in 0..12 {
                let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.fits(grant));
                    Ok(())
                });
            }
            assert!(born > 0);
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap(), std::collections::HashSet::from([source.as_str().to_owned()]));
        } else {
            let output = run_live_query(&registry, &operator, input, &mut caller, &mut born, &mut free);
            let handle = read_geometry(&output, "geometryOut").unwrap();
            assert_ne!(source, handle);
            let input = output.get("geometryOut").unwrap().clone();
            let preview = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            for witness in row["witnesses"].as_array().unwrap() {
                let parameters = &witness[0];
                let expected = fixture_xyz(&witness[1]);
                let u = number(parameters[0].as_f64().unwrap());
                let (sample, input) = if is_curve {
                    ("brep.eval.curvePoint", Dictionary::new().insert("curve", input.clone()).insert("parameter", u))
                } else {
                    ("brep.eval.surfPoint", Dictionary::new().insert("surface", input.clone()).insert("u", u)
                        .insert("v", number(parameters[1].as_f64().unwrap())))
                };
                let value = run_live_query(&registry, sample, input, &mut caller, &mut born, &mut free);
                let point = read_xyz(&value, "point").unwrap();
                assert!(point.iter().zip(expected).all(|(a,b)| (a-b).abs() < 1e-8), "{kind} {operation}");
                let distance = if is_curve { line_preview_distance(&preview.edge_positions, expected) }
                    else { preview_distance(&preview, expected) };
                assert!(distance <= 0.025, "{kind} {operation} preview deviation {distance}");
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_planar_polygon_transforms_keep_exact_trims_and_preview_orientation() {
    use semio_framework_3d_brep_vector::Vec3;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    let fixture = &fixture["polygonTransforms"];
    let cases = fixture["cases"].as_array().unwrap();
    let scaled = cases.iter().find(|case| case["operator"] == "brep.xform.scale").unwrap();
    for (index, case) in cases.iter().chain(std::iter::once(scaled)).enumerate() {
        let operator = case["operator"].as_str().unwrap();
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97218, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let points = fixture["source"].as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"), |list, (i, point)| {
            list.insert(i.to_string(), Value::Dictionary(point_dictionary(fixture_xyz(point))))
        });
        let source = run_live_query(&registry, "brep.surf.planarFace", Dictionary::new().insert("points", Value::Dictionary(points)),
            &mut caller, &mut born, &mut free);
        let handle = read_geometry(&source, "face").unwrap();
        let before = session.with_kernel_read(|kernel| kernel.export_shape(&handle)
            .map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))).unwrap();
        let input = Dictionary::new().insert("geometry", source.get("face").unwrap().clone());
        let input = case["parameters"].as_object().unwrap().iter().fold(input, |input, (key, value)| {
            let field = if value.is_number() { number(value.as_f64().unwrap()) }
                else if key == "factor" { Value::Dictionary(vector_dictionary(fixture_xyz(value))) }
                else { Value::Dictionary(point_dictionary(fixture_xyz(value))) };
            input.insert(key.as_str(), field)
        });
        if index == cases.len() {
            let baseline = session.with_kernel_read(|kernel| Ok(kernel.representation().coedges.len())).unwrap();
            let mut job = Some(admit_live_job(&registry, operator, input));
            let mut interrupted = false;
            for _ in 0..600 {
                let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.fits(grant));
                    Ok(())
                });
                interrupted = session.with_kernel_read(|kernel| {
                    assert_eq!(kernel.live_handles().len(), 1);
                    Ok(kernel.representation().coedges.len() > baseline)
                }).unwrap();
                if interrupted { break; }
            }
            assert!(interrupted);
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            session.with_kernel_read(|kernel| {
                assert_eq!(kernel.live_handles().len(), 1);
                assert_eq!(kernel.representation().coedges.len(), baseline);
                Ok(())
            }).unwrap();
        } else {
            let transformed = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
            let geometry = read_geometry(&transformed, "geometryOut").unwrap();
            assert_ne!(geometry, handle);
            session.with_kernel_read(|kernel| {
                let body = kernel.representation();
                let label = kernel.label_of(&geometry).unwrap();
                let (_, face) = body.faces.iter().find(|(_, face)| face.label == label).unwrap();
                assert_eq!(face.flipped, case["flipped"].as_bool().unwrap());
                let surface = body.surfaces.get(face.surface).unwrap();
                let coedges = body.loop_coedges(face.outer.unwrap());
                assert_eq!(coedges.len(), 3);
                let expected = case["vertices"].as_array().unwrap();
                for (id, point) in coedges.into_iter().zip(expected) {
                    let coedge = body.coedges.get(id).unwrap();
                    let edge = body.edges.get(coedge.edge).unwrap();
                    let vertex = body.vertices.get(if coedge.forward { edge.v0 } else { edge.v1 }).unwrap();
                    assert!(vertex.position.to_array().iter().zip(fixture_xyz(point)).all(|(a,b)| (a-b).abs() < 1e-9));
                    let curve = body.curves3.get(edge.curve).unwrap();
                    let trim = body.curves2.get(coedge.pcurve.unwrap()).unwrap();
                    for mix in [0.0, 0.5, 1.0] {
                        let t = edge.range.0 + (edge.range.1 - edge.range.0) * mix;
                        let p = coedge.prange.0 + (coedge.prange.1 - coedge.prange.0) * mix;
                        let uv = trim.eval(p);
                        assert!(curve.eval(t).distance(surface.eval(uv.x, uv.y)) < 1e-9);
                    }
                }
                Ok(())
            }).unwrap();
            let mesh = run_funded_preview(&session, identity, geometry.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.indices.len(), 3);
            assert!((preview_area_volume(&mesh).0 - case["area"].as_f64().unwrap()).abs() < 1e-5);
            let normal = Vec3::from_array(fixture_xyz(&case["normal"])).normalized().unwrap();
            let point = |id| {
                let i = id as usize * 3;
                Vec3::new(mesh.positions[i].into(), mesh.positions[i+1].into(), mesh.positions[i+2].into())
            };
            let (p, q, r) = (point(mesh.indices[0]), point(mesh.indices[1]), point(mesh.indices[2]));
            assert!((q-p).cross(r-p).normalized().unwrap().dot(normal) > 1.0 - 1e-6);
        }
        let after = session.with_kernel_read(|kernel| kernel.export_shape(&handle)
            .map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))).unwrap();
        assert_eq!(before, after);
        caller.retire(stock());
    }
}

#[test]
fn original_polygonal_face_offset_keeps_winding_and_cancels_without_changing_the_source() {
    use semio_framework_3d_brep_vector::Vec3;
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_geometry};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    let cases = fixture["polygonOffset"].as_array().unwrap();
    for (index, case) in cases.iter().chain(std::iter::once(&cases[0])).enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97216, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let points = case["points"].as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"), |list, (i, point)| {
            list.insert(i.to_string(), Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(point))))
        });
        let face = run_live_query(&registry, "brep.surf.planarFace", Dictionary::new().insert("points", Value::Dictionary(points)),
            &mut caller, &mut born, &mut free);
        let source = read_geometry(&face, "face").unwrap();
        let baseline = session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            Ok([body.vertices.len(), body.edges.len(), body.curves3.len(), body.curves2.len(),
                body.faces.len(), body.loops.len(), body.coedges.len(), body.surfaces.len()])
        }).unwrap();
        let input = Dictionary::new().insert("face", face.get("face").unwrap().clone())
            .insert("distance", number(case["distance"].as_f64().unwrap()));
        if index == cases.len() {
            let mut job = Some(admit_live_job(&registry, "brep.surf.offset", input));
            let mut interrupted = false;
            for _ in 0..1000 {
                let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.fits(grant));
                    Ok(())
                });
                interrupted = session.with_kernel_read(|kernel| {
                    assert_eq!(kernel.live_handles(), std::collections::HashSet::from([source.as_str().to_owned()]));
                    let body = kernel.representation();
                    Ok(body.faces.len() > baseline[4] && body.coedges.len() > baseline[6])
                }).unwrap();
                if interrupted { break; }
            }
            assert!(interrupted);
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            session.with_kernel_read(|kernel| {
                assert_eq!(kernel.live_handles(), std::collections::HashSet::from([source.as_str().to_owned()]));
                let body = kernel.representation();
                assert_eq!([body.vertices.len(), body.edges.len(), body.curves3.len(), body.curves2.len(),
                    body.faces.len(), body.loops.len(), body.coedges.len(), body.surfaces.len()], baseline);
                Ok(())
            }).unwrap();
        } else {
            let result = run_live_query(&registry, "brep.surf.offset", input, &mut caller, &mut born, &mut free);
            let offset = read_geometry(&result, "faceOut").unwrap();
            assert_ne!(offset, source);
            session.with_kernel_read(|kernel| {
                let body = kernel.representation();
                assert_eq!(kernel.live_handles().len(), 2);
                for (handle, expected) in [(&source, &case["points"]), (&offset, &case["offsetPoints"])] {
                    let label = kernel.label_of(handle).unwrap();
                    let (_, face) = body.faces.iter().find(|(_, face)| face.label == label).unwrap();
                    let coedges = body.loop_coedges(face.outer.unwrap());
                    assert_eq!(coedges.len(), expected.as_array().unwrap().len());
                    for (id, position) in coedges.into_iter().zip(expected.as_array().unwrap()) {
                        let coedge = body.coedges.get(id).unwrap();
                        let edge = body.edges.get(coedge.edge).unwrap();
                        let vertex = body.vertices.get(if coedge.forward { edge.v0 } else { edge.v1 }).unwrap();
                        assert!(vertex.position.to_array().iter().zip(fixture_xyz(position)).all(|(a,b)| (a-b).abs() < 1e-9));
                    }
                }
                Ok(())
            }).unwrap();
            let area = run_live_query(&registry, "brep.measure.area", Dictionary::new().insert("geometry", result.get("faceOut").unwrap().clone()),
                &mut caller, &mut born, &mut free);
            let expected_area = case["area"].as_f64().unwrap();
            assert!((read_channel_number(&area, "area").unwrap() - expected_area).abs() < 1e-9);
            let mesh = run_funded_preview(&session, identity, offset.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.indices.len() / 3, case["triangles"].as_u64().unwrap() as usize);
            assert!((preview_area_volume(&mesh).0 - expected_area).abs() < 1e-6);
            let normal = Vec3::from_array(fixture_xyz(&case["normal"]));
            for triangle in mesh.indices.chunks_exact(3) {
                let vertex = |id| {
                    let i = id as usize * 3;
                    Vec3::new(mesh.positions[i].into(), mesh.positions[i+1].into(), mesh.positions[i+2].into())
                };
                let (p,q,r) = (vertex(triangle[0]), vertex(triangle[1]), vertex(triangle[2]));
                assert!((q-p).cross(r-p).dot(normal) > 0.0);
            }
        }
        caller.retire(stock());
    }
}

fn line_preview_distance(segments: &[f32], point: [f64; 3]) -> f32 {
    use parry3d::{math::Point, query::PointQuery, shape::Segment};
    let query = Point::new(point[0] as f32, point[1] as f32, point[2] as f32);
    segments.chunks_exact(6).map(|edge| {
        let segment = Segment::new(Point::new(edge[0], edge[1], edge[2]), Point::new(edge[3], edge[4], edge[5]));
        (segment.project_local_point(&query, false).point - query).norm()
    }).fold(f32::INFINITY, f32::min)
}

fn preview_distance(mesh: &semio_framework::MeshData, point: [f64; 3]) -> f32 {
    use parry3d::{math::Point, query::PointQuery, shape::Triangle};
    let query = Point::new(point[0] as f32, point[1] as f32, point[2] as f32);
    let vertex = |id: u32| {
        let index = id as usize * 3;
        Point::new(mesh.positions[index], mesh.positions[index + 1], mesh.positions[index + 2])
    };
    mesh.indices.chunks_exact(3).map(|triangle| {
        let surface = Triangle::new(vertex(triangle[0]), vertex(triangle[1]), vertex(triangle[2]));
        (surface.project_local_point(&query, false).point - query).norm()
    }).fold(f32::INFINITY, f32::min)
}

fn run_curve_case(case: &serde_json::Value, queries: &serde_json::Value, default_tolerance: f64, cancelled: bool) {
    let operator = case["kind"].as_str().unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut parameters = Dictionary::new();
    for (port, param) in case["inputs"].as_object().unwrap() {
        let value = if param.is_array() {
            let xyz = std::array::from_fn(|axis| param[axis].as_f64().unwrap());
            Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(xyz))
        } else {
            number(param.as_f64().unwrap())
        };
        parameters = parameters.insert(port.as_str(), value);
    }
    let mut job = Some(admit_live_job(&registry, operator, parameters));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97147), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let mut output = None;
    run(&mut caller, Grant { maximum_items: 0, ..stock() }, &mut born, &mut free, |cx| {
        let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
        assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
        assert_eq!(receipt, Progress::default());
        Ok(())
    });
    for turn in 0..10000 {
        if cancelled && turn == 7 { caller.cancel().cancel_now(); }
        let mut stopped = false;
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
            let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
            assert_eq!(receipt, job.as_ref().unwrap().normal_step_progress());
            match step {
                neural_engine::OperatorJobStep::Done(result) => output = Some(result),
                neural_engine::OperatorJobStep::Cancelled(_) => stopped = true,
                _ => {}
            }
            Ok(())
        });
        if output.is_some() || stopped { break; }
    }
    if cancelled {
        assert!(output.is_none());
        assert!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().is_empty())).unwrap());
    } else {
        let output = neural_engine::ColdOwner::new(output.expect("analytic curve completes its original producer"));
        let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, "curve").unwrap();
        let length = session.with_kernel_read(|kernel| {
            let error = |error: semio_framework_3d_brep_engine::BrepError| semio_s_spatial_kernel_semio_session::map_kernel_error(&error);
            if operator == "brep.curve.line" {
                let start = kernel.curve_point_sync(&handle, 0.0).map_err(error)?;
                let end = kernel.curve_point_sync(&handle, 1.0).map_err(error)?;
                return Ok((0..3).map(|axis| (end[axis] - start[axis]).powi(2)).sum::<f64>().sqrt());
            }
            kernel.length_sync(&handle).map_err(error)
        }).unwrap();
        let expected = case["outputs"]["shape"]["length"].as_f64().unwrap();
        assert!((length - expected).abs() < case["tolerance"].as_f64().unwrap_or(default_tolerance), "{operator}: {length} != {expected}");

        use flow_extension_sdk::geometry::{GeometryStep, GeometryWorkIdentity};
        let identity = GeometryWorkIdentity { operation: 97147, generation: 17 };
        let tolerance = 0.025;
        let mut mesh = None;
        for _ in 0..10000 {
            let demand = session.tessellation_demands(identity, handle.as_str(), tolerance, 64).unwrap();
            let grant = Grant {
                maximum_items: 1,
                maximum_copy_bytes: demand.copy_bytes.max(64),
                maximum_capacity_bytes: demand.capacity_bytes,
                maximum_release_bytes: demand.release_bytes,
                maximum_depth: demand.depth,
            };
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, work) = session.tessellate_step(identity, handle.as_str(), tolerance, cx).unwrap();
                assert!(work.consumed_units <= 1);
                assert!(work.retained.fits(grant));
                match step {
                    GeometryStep::Ready(output) => mesh = Some(output),
                    GeometryStep::Working { .. } => {}
                    _ => panic!("{operator} preview stopped before producing a mesh"),
                }
                Ok(())
            });
            if mesh.is_some() { break; }
        }
        let mesh = mesh.expect("original curve has a visible line preview");
        let edges = &mesh.edge_positions;
        assert!(!edges.is_empty() && edges.len() % 6 == 0);
        assert!(edges.iter().all(|value| value.is_finite()));
        if operator == "brep.curve.line" {
            let endpoints: Vec<_> = ["start", "end"].into_iter().flat_map(|port| case["inputs"][port].as_array().unwrap().iter().map(|axis| axis.as_f64().unwrap() as f32)).collect();
            assert_eq!(edges, &endpoints);

            let parameter = queries["parameter"].as_f64().unwrap();
            for (operator, channel) in [
                ("brep.eval.curvePoint", "point"),
                ("brep.eval.curveTangent", "tangent"),
            ] {
                let input = Dictionary::new().insert("curve", output.get("curve").unwrap().clone()).insert("parameter", number(parameter));
                let result = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
                let xyz = semio_s_spatial_kernel_semio_session::read_xyz(&result, channel).unwrap();
                let expected = std::array::from_fn(|axis| queries[channel][axis].as_f64().unwrap());
                assert_eq!(xyz, expected, "{operator}");
            }
        } else {
            assert!(edges.len() > 6);
            for (previous, next) in edges.chunks_exact(6).zip(edges[6..].chunks_exact(6)) {
                assert_eq!(&previous[3..], &next[..3]);
            }
            let ends = edges[..3].iter().zip(&edges[edges.len()-3..]);
            assert!(ends.into_iter().all(|(first, last)| (first - last).abs() < 1e-5));
        }
        if operator != "brep.curve.line" {
            for (operator, input_channel, output_channel, expected) in [
                ("brep.eval.curveDomain", "curve", "span", queries["span"].as_f64().unwrap()),
                ("brep.measure.length", "geometry", "length", expected),
            ] {
                let input = Dictionary::new().insert(input_channel, output.get("curve").unwrap().clone());
                let result = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
                let measured = semio_s_spatial_kernel_semio_session::read_channel_number(&result, output_channel).unwrap();
                assert!((measured - expected).abs() < case["tolerance"].as_f64().unwrap_or(default_tolerance), "{operator}: {measured} != {expected}");
            }
        }

        let input = Dictionary::new().insert("curve", output.get("curve").unwrap().clone()).insert("parameter", number(queries["parameter"].as_f64().unwrap()));
        let curvature = run_live_query(&registry, "brep.eval.curveCurvature", input, &mut caller, &mut born, &mut free);
        let curvature = semio_s_spatial_kernel_semio_session::read_channel_number(&curvature, "curvature").unwrap();
        assert!((curvature - queries["curvature"].as_f64().unwrap()).abs() < default_tolerance);

        let input = Dictionary::new().insert("geometry", output.get("curve").unwrap().clone());
        let result = run_live_query(&registry, "brep.topology.label", input, &mut caller, &mut born, &mut free);
        let label = result.get("label").and_then(Value::as_dictionary).and_then(|value| value.get("value")).and_then(Value::as_atom).and_then(Atom::as_str).unwrap();
        let expected = session.with_kernel_read(|kernel| Ok(kernel.label_of(&handle).unwrap().0)).unwrap();
        assert_eq!(label.parse::<u64>().unwrap(), expected);
    }
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    caller.retire(stock());
}

#[test]
fn original_face_from_wire_preserves_trimmed_rational_pcurve_and_rolls_back() {
    use semio_framework_3d_brep_curve::{Curve2, Curve3, bspline::KnotVector};
    use semio_framework_3d_brep_engine::{ShapeRoot, ShapeValue, ShapeWire};
    use semio_framework_3d_brep_representation::topology::{Body, Edge, Vertex};
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    let row = &fixture["exactSplineFace"];
    let point = |value| { let [x, y, z] = fixture_xyz(value); Pnt3::new(x, y, z) };
    let scalars = |value: &serde_json::Value| value.as_array().unwrap().iter().map(|n| n.as_f64().unwrap()).collect::<Vec<_>>();
    let knots = scalars(&row["knots"]);
    let weights = scalars(&row["weights"]);
    let controls: Vec<_> = row["controls"].as_array().unwrap().iter().map(point).collect();
    let trim = scalars(&row["trim"]);
    let range = (trim[0], trim[1]);
    for reversed in [false, true] {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut body = Body::new();
        let positions = [point(&row["ends"][0]), point(&row["ends"][1]), point(&row["third"])];
        let vertices = positions.map(|position| {
            let label = body.new_label();
            body.vertices.insert(Vertex { position, label, tol: Default::default() })
        });
        let edges = std::array::from_fn::<_, 3, _>(|i| {
            let j = (i + 1) % 3;
            let curve = if i == 0 { Curve3::Nurbs { knots: KnotVector::new(knots.clone(), 2, 3).unwrap(), controls: controls.clone(), weights: weights.clone() } }
                else { Curve3::Line { origin: positions[i], dir: positions[j] - positions[i] } };
            let curve = body.curves3.insert(curve);
            let label = body.new_label();
            body.edges.insert(Edge { curve, range: if i == 0 { range } else { (0.,1.) }, v0: vertices[i], v1: vertices[j], label, tol: Default::default() })
        });
        let member_order = if reversed { [0,2,1] } else { [0,1,2] };
        let vertex_order = if reversed { [1,0,2] } else { [0,1,2] };
        let root = ShapeRoot::Wire(ShapeWire {
            members: member_order.map(|i| (edges[i], !reversed)).to_vec(),
            vertices: vertex_order.map(|i| vertices[i]).to_vec(),
            closed: true,
            label: body.new_label(),
        });
        let source = session.with_kernel(|kernel| {
            let wire = kernel.import_shape(&ShapeValue { root, body }).map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))?;
            semio_s_spatial_kernel_semio_session::geometry_dict(kernel, &wire)
        }).unwrap();
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97251, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let input = Dictionary::new().insert("wire", Value::Dictionary(source));
        let operator = "brep.util.faceFromWire";
        let mut job = Some(admit_live_job(&registry, operator, input.clone()));
        let mut inserted = false;
        for _ in 0..512 {
            let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                assert!(receipt.fits(grant));
                Ok(())
            });
            inserted = session.with_kernel_read(|kernel| Ok(kernel.representation().curves2.iter().any(|(_, curve)| matches!(curve, Curve2::Nurbs { .. })))).unwrap();
            if inserted { break; }
        }
        assert!(inserted);
        job.as_mut().unwrap().cancel();
        run(&mut caller, stock(), &mut born, &mut free, |cx| {
            assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
            Ok(())
        });
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            assert!(body.curves2.is_empty() && body.faces.is_empty() && body.loops.is_empty() && body.coedges.is_empty());
            assert_eq!(body.edges.len(), 3);
            assert_eq!(kernel.live_handles().len(), 1);
            Ok(())
        }).unwrap();

        let output = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
        let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, "face").unwrap();
        let edge_label = session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            let (coedge, pcurve) = body.coedges.iter().find_map(|(_, coedge)| {
                let pcurve = body.curves2.get(coedge.pcurve?)?;
                matches!(pcurve, Curve2::Nurbs { .. }).then_some((coedge,pcurve))
            }).unwrap();
            let Curve2::Nurbs { knots: projected_knots, weights: projected_weights, .. } = pcurve else { unreachable!() };
            assert_eq!(projected_knots.knots, knots);
            assert_eq!(*projected_weights, weights);
            assert_eq!(coedge.prange, range);
            assert_eq!(coedge.forward, !reversed);
            let loop_ = body.loops.get(coedge.loop_id).unwrap();
            let face = body.faces.get(loop_.face).unwrap();
            let Surface::Plane { frame } = body.surfaces.get(face.surface).unwrap() else { unreachable!() };
            let edge = body.edges.get(coedge.edge).unwrap();
            let curve = body.curves3.get(edge.curve).unwrap();
            for i in 0..=32 {
                let t = range.0 + (range.1 - range.0) * f64::from(i) / 32.;
                let uv = pcurve.eval(t);
                let rebuilt = frame.to_world(Pnt3::new(uv.x, uv.y, 0.));
                assert!((rebuilt - curve.eval(t)).norm() < 1e-12);
            }
            Ok(edge.label.0.to_string())
        }).unwrap();
        let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        assert!(mesh.component_references["edge"].iter().filter(|label| **label == edge_label).count() > 1);
        assert_eq!(mesh.component_references["vertex"].len(), 3);
        assert!((preview_area_volume(&mesh).0 - row["area"].as_f64().unwrap()).abs() < 0.02);
        caller.retire(stock());
    }
}

#[test]
fn retained_sew_rebuilds_exact_shared_cuboid_incidence_without_mutating_faces() {
    use semio_s_spatial_kernel_semio_session::read_geometry;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🐚️brep-topology/🔣️.json")).unwrap();
    let row = &fixture["retainedCuboidSew"];
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 98462, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0,0);
    let faces: Vec<_> = row["rings"].as_array().unwrap().iter().map(|ring| {
        let input = polygon_fixture_input(&serde_json::json!({"points": ring}));
        let result = run_live_query(&registry, "brep.surf.planarFace", input, &mut caller, &mut born, &mut free);
        result.get("face").unwrap().clone()
    }).collect();
    let input = |duplicate: bool| {
        let list = faces.iter().enumerate().fold(Dictionary::with_schema("list"), |list, (i, face)| {
            list.insert(i.to_string(), if duplicate && i == 5 { faces[0].clone() } else { face.clone() })
        });
        Dictionary::new().insert("faces", Value::Dictionary(list)).insert("tolerance", number(1e-6))
    };
    let counts = || session.with_kernel_read(|kernel| {
        let b = kernel.representation();
        Ok([kernel.live_handles().len(), b.faces.len(), b.edges.len(), b.vertices.len(), b.shells.len(), b.solids.len()])
    }).unwrap();
    let before = counts();
    assert_eq!(before, [6,6,24,24,0,0]);
    let operator = "brep.util.sew";
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    for duplicate in [true,false] {
        let mut job = Some(admit_live_job(&registry, operator, input(duplicate)));
        let mut failure = None;
        for _ in 0..512 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                match job.as_mut().unwrap().step(cx) {
                    Ok((neural_engine::OperatorJobStep::Working(_), receipt)) => assert!(receipt.fits(grant)),
                    Ok(_) => panic!("sew must still be unpublished"),
                    Err(error) => failure = Some(error),
                }
                Ok(())
            });
            if failure.is_some() || counts()[2] > before[2] { break; }
        }
        if duplicate {
            assert!(format!("{:?}", failure.expect("duplicate face must be refused")).contains("six exact outward cuboid faces"));
        } else {
            assert!(failure.is_none());
            assert!(counts()[2] > before[2]);
            assert_eq!(counts()[0],before[0]);
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert_eq!(counts(),before);
    }
    let output = run_live_query(&registry, operator, input(false), &mut caller, &mut born, &mut free);
    let handle = read_geometry(&output, "solid").unwrap();
    session.with_kernel_read(|kernel| {
        let id = kernel.geometry_solid(&handle).map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))?;
        let body = kernel.representation();
        let solid = body.solids.get(id).unwrap();
        let faces = &body.shells.get(solid.outer).unwrap().faces;
        let mut edges = std::collections::BTreeMap::new();
        let mut vertices = std::collections::BTreeSet::new();
        assert_eq!(faces.len(),6);
        for &id in faces {
            let face = body.faces.get(id).unwrap();
            for use_id in body.loop_coedges(face.outer.unwrap()) {
                let use_ = body.coedges.get(use_id).unwrap();
                edges.entry(use_.edge).or_insert_with(Vec::new).push(use_.forward != face.flipped);
                let edge = body.edges.get(use_.edge).unwrap();
                vertices.extend([edge.v0,edge.v1]);
            }
        }
        assert_eq!(edges.len(),12);
        assert_eq!(vertices.len(),8);
        for mut uses in edges.into_values() { uses.sort(); assert_eq!(uses,[false,true]); }
        Ok(())
    }).unwrap();
    assert_eq!(counts(), [7,12,36,32,1,1]);
    let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
    let (area, volume) = preview_area_volume(&mesh);
    assert!((area-row["area"].as_f64().unwrap()).abs()<1e-6);
    assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-6);
    caller.retire(stock());
}

#[test]
fn original_box_edge_chamfer_uses_the_selected_native_edge_and_rolls_back() {
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{read_geometry,text_dictionary,vector_dictionary};
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🛠️brep-feature/🔣️.json")).unwrap();
    for (index,row) in corpus["retainedBoxEdgeChamfer"].as_array().unwrap().iter().enumerate() {
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=flow_extension_sdk::geometry::GeometryWorkIdentity { operation:98510+index as u64,generation:17 };
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [width,depth,height]=fixture_xyz(&row["size"]);
        let parameters=Dictionary::new().insert("width",number(width)).insert("depth",number(depth)).insert("height",number(height));
        let raw=run_live_query(&registry,"brep.prim3d.box",parameters,&mut caller,&mut born,&mut free);
        let placement=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let placed=run_live_query(&registry,"brep.xform.translate",placement,&mut caller,&mut born,&mut free);
        let solid=placed.get("geometryOut").unwrap().clone();
        let a=Pnt3::from_array(fixture_xyz(&row["edge"][0]));
        let b=Pnt3::from_array(fixture_xyz(&row["edge"][1]));
        let edge_label=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let (_,edge)=body.edges.iter().find(|(_,edge)|{
                let p=body.vertices.get(edge.v0).unwrap().position;
                let q=body.vertices.get(edge.v1).unwrap().position;
                (p==a && q==b)||(p==b && q==a)
            }).unwrap();
            Ok(edge.label.0.to_string())
        }).unwrap();
        let text=|value|Value::Dictionary(text_dictionary(value));
        let select=Dictionary::new().insert("brep",solid.clone())
            .insert("edgeLabels",text(format!(r#"["{edge_label}"]"#)))
            .insert("faceLabels",text("[]".to_owned())).insert("sourceHandle",text(String::new()));
        let selection=run_live_query(&registry,"brep.brep",select,&mut caller,&mut born,&mut free);
        let edges=selection.get("selectedEdges").unwrap().clone();
        let input=|selected,dist|Dictionary::new().insert("geometry",selected).insert("edges",edges.clone()).insert("distance",number(dist));
        let operator="brep.solid.chamferEdges";
        let counts=||session.with_kernel_read(|kernel|{
            let b=kernel.representation();
            Ok((kernel.live_handles().len(),b.vertices.len(),b.edges.len(),b.solids.len()))
        }).unwrap();
        let before=counts();
        let distance=row["distance"].as_f64().unwrap();
        for (source,dist,reason) in [
            (raw.get("solid").unwrap().clone(),distance,Some("original edge of a certified box")),
            (solid.clone(),width.max(depth).max(height),Some("both adjoining box sides")),
            (solid.clone(),distance,None),
        ] {
            let mut job=Some(admit_live_job(&registry,operator,input(source,dist)));
            let grant=Grant { maximum_items:1,maximum_copy_bytes:128,..stock() };
            let mut failure=None;
            for _ in 0..512 {
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    match job.as_mut().unwrap().step(cx){
                        Ok((neural_engine::OperatorJobStep::Working(_),receipt))=>assert!(receipt.fits(grant)),
                        Ok(_)=>panic!("unexpected chamfer publication"),
                        Err(error)=>failure=Some(error),
                    }
                    Ok(())
                });
                if failure.is_some()||counts().2>before.2{break;}
            }
            if let Some(reason)=reason {
                assert!(format!("{:?}",failure.expect("invalid selected chamfer refused")).contains(reason));
            } else {
                assert!(failure.is_none()&&counts().2>before.2);
                assert_eq!(counts().0,before.0);
                job.as_mut().unwrap().cancel();
                run(&mut caller,stock(),&mut born,&mut free,|cx|{
                    assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
                    Ok(())
                });
            }
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let output=run_live_query(&registry,operator,input(solid,distance),&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"solid").unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["face"].len(),7);
        assert_eq!(mesh.component_references["edge"].len(),15);
        assert_eq!(mesh.component_references["vertex"].len(),10);
        let (area,volume)=preview_area_volume(&mesh);
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-5);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-5);
        caller.retire(stock());
    }
}

#[test]
fn original_all_edge_chamfer_funds_exact_topology_and_commits_once() {
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-all-edge-chamfer.json")).unwrap();
    let operator = fixture["operator"].as_str().unwrap();
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 98720 + index as u64, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let dimensions = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&row["size"]))
            .fold(Dictionary::new(), |input, (axis, size)| input.insert(axis, number(size)));
        let raw = run_live_query(&registry, "brep.prim3d.box", dimensions, &mut caller, &mut born, &mut free);
        let placement = Dictionary::new().insert("geometry", raw.get("solid").unwrap().clone())
            .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let placed = run_live_query(&registry, "brep.xform.translate", placement, &mut caller, &mut born, &mut free);
        let input = |distance| Dictionary::new().insert("geometry", placed.get("geometryOut").unwrap().clone())
            .insert("distance", number(distance));
        let counts = || session.with_kernel_read(|kernel| {
            let b = kernel.representation();
            Ok([kernel.live_handles().len(), b.vertices.len(), b.edges.len(), b.faces.len(), b.solids.len()])
        }).unwrap();
        let before = counts();
        let distance = row["distance"].as_f64().unwrap();
        let one_item = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        let mut cancelled = Some(admit_live_job(&registry, operator, input(distance)));
        run(&mut caller, Grant { maximum_items: 0, ..stock() }, &mut born, &mut free, |cx| {
            let (step, receipt) = cancelled.as_mut().unwrap().step(cx).unwrap();
            assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
            assert_eq!(receipt, Progress::default());
            Ok(())
        });
        assert_eq!(counts(), before);
        for _ in 0..2000 {
            run(&mut caller, one_item, &mut born, &mut free, |cx| {
                let (step, receipt) = cancelled.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)) && receipt.fits(one_item));
                Ok(())
            });
            assert_eq!(counts()[0], before[0]);
            if counts()[3] > before[3] { break; }
        }
        assert!(counts()[3] > before[3]);
        cancelled.as_mut().unwrap().cancel();
        close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
        assert_eq!(counts(), before);

        for refusal in row["refused"].as_array().unwrap() {
            let mut job = Some(admit_live_job(&registry, operator, input(refusal.as_f64().unwrap())));
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                let error = job.as_mut().unwrap().step(cx).err().expect("invalid all-edge distance must be refused");
                assert!(format!("{error:?}").contains("all-edge chamfer distance"), "{error:?}");
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert_eq!(counts(), before);
        }

        let mut job = Some(admit_live_job(&registry, operator, input(distance)));
        let mut output = None;
        for _ in 0..2400 {
            run(&mut caller, one_item, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                assert!(receipt.fits(one_item));
                match step {
                    neural_engine::OperatorJobStep::Working(_) => assert_eq!(counts()[0], before[0]),
                    neural_engine::OperatorJobStep::Done(value) => output = Some(value),
                    _ => panic!("unexpected all-edge chamfer progress"),
                }
                Ok(())
            });
            if output.is_some() { break; }
        }
        assert_eq!(counts(), [before[0] + 1, before[1] + 24, before[2] + 48, before[3] + 26, before[4] + 1]);
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let output = neural_engine::ColdOwner::new(output.expect("all-edge chamfer did not publish"));
        let handle = read_geometry(&output, "solid").unwrap();
        let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        for (domain, fixture_key) in [("face", "faceCount"), ("edge", "edgeCount"), ("vertex", "vertexCount")] {
            assert_eq!(mesh.component_references[domain].len(), fixture[fixture_key].as_u64().unwrap() as usize);
        }
        let (area, volume) = preview_area_volume(&mesh);
        assert!((area - row["area"].as_f64().unwrap()).abs() < 1e-5);
        assert!((volume - row["volume"].as_f64().unwrap()).abs() < 1e-5);
        for (operator, quantity) in [("brep.measure.area", "area"), ("brep.measure.volume", "volume")] {
            let input = Dictionary::new().insert("geometry", output.get("solid").unwrap().clone());
            let measured = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
            let actual = read_channel_number(&measured, quantity).unwrap();
            assert!((actual - row[quantity].as_f64().unwrap()).abs() < 1e-9);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_box_face_draft_preserves_the_neutral_line_and_rolls_back() {
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{map_kernel_error,point_dictionary,read_geometry,text_dictionary,vector_dictionary};
    let corpus: serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🛠️brep-feature/🔣️.json")).unwrap();
    for (index,row) in corpus["retainedBoxFaceDraft"].as_array().unwrap().iter().enumerate() {
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=flow_extension_sdk::geometry::GeometryWorkIdentity { operation:98550+index as u64,generation:17 };
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [width,depth,height]=fixture_xyz(&row["size"]);
        let parameters=Dictionary::new().insert("width",number(width)).insert("depth",number(depth)).insert("height",number(height));
        let raw=run_live_query(&registry,"brep.prim3d.box",parameters,&mut caller,&mut born,&mut free);
        let placement=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let placed=run_live_query(&registry,"brep.xform.translate",placement,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&placed,"geometryOut").unwrap();
        let normal=fixture_xyz(&row["normal"]);
        let label=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).map_err(|error|map_kernel_error(&error))?;
            Ok(body.solid_faces(id).into_iter().find_map(|face|{
                let face=body.faces.get(face)?;
                let Surface::Plane{frame}=body.surfaces.get(face.surface)? else{return None};
                (frame.z.to_array()==normal).then(||face.label.0.to_string())
            }).unwrap())
        }).unwrap();
        let preview=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        assert!(preview.component_references["face"].contains(&label));
        let solid=placed.get("geometryOut").unwrap().clone();
        let text=|value|Value::Dictionary(text_dictionary(value));
        let select=Dictionary::new().insert("brep",solid.clone()).insert("edgeLabels",text("[]".to_owned()))
            .insert("faceLabels",text(format!(r#"["{label}"]"#))).insert("sourceHandle",text(String::new()));
        let selection=run_live_query(&registry,"brep.brep",select,&mut caller,&mut born,&mut free);
        let input=Dictionary::new().insert("geometry",solid).insert("faces",selection.get("selectedFaces").unwrap().clone())
            .insert("pullDirection",Value::Dictionary(vector_dictionary(fixture_xyz(&row["pullDirection"]))))
            .insert("neutralPoint",Value::Dictionary(point_dictionary(fixture_xyz(&row["neutralPoint"]))))
            .insert("angle",number(row["angle"].as_f64().unwrap()));
        let operator="brep.solid.draft";
        if index==0 {
            let counts=||session.with_kernel_read(|kernel|{
                let body=kernel.representation();
                Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
            }).unwrap();
            let before=counts();
            let mut job=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..512{
                let grant=Grant { maximum_items:1,maximum_copy_bytes:128,..stock() };
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    let (step,receipt)=job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                    Ok(())
                });
                if counts()[2]>before[2]{break;}
            }
            assert!(counts()[2]>before[2]);assert_eq!(counts()[0],before[0]);
            job.as_mut().unwrap().cancel();
            run(&mut caller,stock(),&mut born,&mut free,|cx|{
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let output=run_live_query(&registry,operator,input,&mut caller,&mut born,&mut free);
        let solid=read_geometry(&output,"solid").unwrap();
        let mesh=run_funded_preview(&session,identity,solid.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["face"].len(),6);
        assert_eq!(mesh.component_references["edge"].len(),12);
        let (area,volume)=preview_area_volume(&mesh);
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-5);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-5);
        caller.retire(stock());
    }
}

#[test]
fn original_builders_preserve_the_picked_circular_opening_and_roll_back() {
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{read_geometry,text_dictionary,vector_dictionary};
    let corpus: serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏳️brep-surface/🔣️.json")).unwrap();
    let row=&corpus["retainedBoreCapOffset"];
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity=flow_extension_sdk::geometry::GeometryWorkIdentity { operation:98580,generation:17 };
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
    let (mut born,mut free)=(0,0);
    let [width,depth,height]=fixture_xyz(&row["stock"]);
    let box_params=Dictionary::new().insert("width",number(width)).insert("depth",number(depth)).insert("height",number(height));
    let box_result=run_live_query(&registry,"brep.prim3d.box",box_params,&mut caller,&mut born,&mut free);
    let radius=row["bore"]["radius"].as_f64().unwrap();
    let cylinder_input=Dictionary::new().insert("radius",number(radius)).insert("height",number(row["bore"]["height"].as_f64().unwrap()));
    let cylinder=run_live_query(&registry,"brep.prim3d.cylinder",cylinder_input,&mut caller,&mut born,&mut free);
    let placement=Dictionary::new().insert("geometry",cylinder.get("solid").unwrap().clone())
        .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["bore"]["origin"]))));
    let placed=run_live_query(&registry,"brep.xform.translate",placement,&mut caller,&mut born,&mut free);
    let cut_input=Dictionary::new().insert("a",box_result.get("solid").unwrap().clone()).insert("b",placed.get("geometryOut").unwrap().clone());
    let cut=retained_boolean_result(&registry,"brep.bool.cut",cut_input,&mut caller,&mut born,&mut free);
    let cut_handle=read_geometry(&cut,"solid").unwrap();
    let preview=run_funded_preview(&session,identity,cut_handle.as_str(),&mut caller,&mut born,&mut free);
    let counts=||session.with_kernel_read(|kernel|{
        let body=kernel.representation();
        Ok([kernel.live_handles().len(),body.faces.len(),body.edges.len(),body.loops.len(),
            body.curves3.iter().filter(|(_,curve)|matches!(curve,Curve3::Circle{..})).count(),
            body.solids.len()])
    }).unwrap();
    for (index,case) in row["cases"].as_array().unwrap().iter().enumerate() {
        let normal=fixture_xyz(&case["normal"]);
        let label=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&cut_handle).unwrap();
            Ok(body.solid_faces(id).into_iter().find_map(|id|{
                let face=body.faces.get(id)?;
                let Surface::Plane{frame}=body.surfaces.get(face.surface)? else{return None};
                (face.inners.len()==1 && frame.z.to_array()==normal).then(||face.label.0.to_string())
            }).unwrap())
        }).unwrap();
        assert!(preview.component_references["face"].contains(&label));
        let text=|value|Value::Dictionary(text_dictionary(value));
        let select=Dictionary::new().insert("brep",cut.get("solid").unwrap().clone()).insert("edgeLabels",text("[]".to_owned()))
            .insert("faceLabels",text(format!(r#"["{label}"]"#))).insert("sourceHandle",text(String::new()));
        let selection=run_live_query(&registry,"brep.brep",select,&mut caller,&mut born,&mut free);
        let face=selection.get("selectedFaces").and_then(Value::as_dictionary).unwrap().get("0").unwrap().clone();
        let input=Dictionary::new().insert("face",face).insert("distance",number(case["distance"].as_f64().unwrap()));
        let operator="brep.surf.offset";
        if index==0 {
            let before=counts();
            let mut job=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..1024 {
                run(&mut caller,Grant { maximum_items:1,maximum_copy_bytes:128,..stock() },&mut born,&mut free,|cx|{
                    assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                    Ok(())
                });
                if counts()[4]>before[4]{break;}
            }
            assert!(counts()[4]>before[4]);assert_eq!(counts()[0],before[0]);
            job.as_mut().unwrap().cancel();
            run(&mut caller,stock(),&mut born,&mut free,|cx|{
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let output=run_live_query(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"faceOut").unwrap();
        let [x,y,_]=fixture_xyz(&row["bore"]["origin"]);
        let z=case["z"].as_f64().unwrap();
        let edge_label=session.with_kernel_read(|kernel|{
            let face_label=kernel.label_of(&handle).unwrap();
            let body=kernel.representation();
            let (_,face)=body.faces.iter().find(|(_,face)|face.label==face_label).unwrap();
            let Surface::Plane{frame:plane}=body.surfaces.get(face.surface).unwrap() else{unreachable!()};
            assert_eq!(plane.origin.z,z);
            let [inner]=face.inners.as_slice() else{panic!("missing translated opening")};
            let use_=body.coedges.get(body.loops.get(*inner).unwrap().first).unwrap();
            let edge=body.edges.get(use_.edge).unwrap();
            let Curve3::Circle{frame,radius:actual}=body.curves3.get(edge.curve).unwrap() else{unreachable!()};
            assert_eq!(frame.origin.to_array(),[x,y,z]);assert_eq!(*actual,radius);
            let curve=body.curves3.get(edge.curve).unwrap();
            let pcurve=body.curves2.get(use_.pcurve.unwrap()).unwrap();
            for t in [0.,0.5,2.,4.,6.] {
                let uv=pcurve.eval(t);
                assert!((curve.eval(t)-plane.to_world(Pnt3::new(uv.x,uv.y,0.))).norm()<1e-12);
            }
            Ok(edge.label.0.to_string())
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(mesh.component_references["edge"].contains(&edge_label));
        assert!(preview_distance(&mesh,[x,y,z])>radius as f32-0.05);
        assert!((preview_area_volume(&mesh).0-row["area"].as_f64().unwrap()).abs()<0.12);
        let thickness=case["thickness"].as_f64().unwrap();
        let input=Dictionary::new().insert("face",output.get("faceOut").unwrap().clone()).insert("thickness",number(thickness));
        let operator="brep.surf.thicken";
        if index==0 {
            let before=counts();
            let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..1024 {
                run(&mut caller,Grant { maximum_items:1,maximum_copy_bytes:128,..stock() },&mut born,&mut free,|cx|{
                    assert!(matches!(draft.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                    Ok(())
                });
                if counts()[4]>before[4]{break;}
            }
            assert!(counts()[4]>before[4]);assert_eq!(counts()[0],before[0]);
            draft.as_mut().unwrap().cancel();
            run(&mut caller,stock(),&mut born,&mut free,|cx|{
                assert!(matches!(draft.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut draft,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let solid=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let solid_handle=read_geometry(&solid,"solid").unwrap();
        let exact_caps=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&solid_handle).unwrap();
            let mut levels=Vec::new();
            let mut tube=false;
            for id in body.solid_faces(id) {
                let face=body.faces.get(id).unwrap();
                match body.surfaces.get(face.surface).unwrap() {
                    Surface::Plane{frame} if face.inners.len()==1=>levels.push(frame.origin.z),
                    Surface::Cylinder{frame,radius:wall_radius}=>{
                        assert_eq!([frame.origin.x,frame.origin.y],[x,y]);
                        assert_eq!(*wall_radius,radius);
                        assert!(face.flipped);
                        tube=true;
                    }
                    _=>{}
                }
            }
            assert!(tube);
            assert!(kernel.label_of(&handle).is_some());
            levels.sort_by(f64::total_cmp);
            Ok(levels)
        }).unwrap();
        let expected=[case["thickZ"][0].as_f64().unwrap(),case["thickZ"][1].as_f64().unwrap()];
        assert_eq!(exact_caps,expected);
        let solid_preview=run_funded_preview(&session,identity,solid_handle.as_str(),&mut caller,&mut born,&mut free);
        solid_preview.validate_component_references().unwrap();
        assert_eq!(solid_preview.component_references["face"].len(),7);
        assert!((preview_area_volume(&solid_preview).1-case["volume"].as_f64().unwrap()).abs()<0.15);
        for level in expected {
            assert!(preview_distance(&solid_preview,[x,y,level])>radius as f32-0.05);
        }
        let extrusion=&case["extrude"];
        let operator=extrusion["operator"].as_str().unwrap();
        let parameters=|vector|Dictionary::new().insert("face",output.get("faceOut").unwrap().clone())
            .insert("vector",Value::Dictionary(vector_dictionary(vector)));
        let vector=fixture_xyz(&extrusion["vector"]);
        let input=if operator=="brep.sweep.extrude" {parameters(vector)} else {
            let stroke=serde_json::json!({"points":[[x,y,z],[x+vector[0],y+vector[1],z+vector[2]]]});
            let path=retained_boolean_result(&registry,"brep.curve.polyline",polygon_fixture_input(&stroke),&mut caller,&mut born,&mut free);
            Dictionary::new().insert("profile",output.get("faceOut").unwrap().clone()).insert("path",path.get("wire").unwrap().clone())
        };
        if index==0 {
            let before=counts();
            for invalid in row["refusedVectors"].as_array().unwrap() {
                let mut job=Some(admit_live_job(&registry,operator,parameters(fixture_xyz(invalid))));
                let mut failed=false;
                for _ in 0..64 {
                    run(&mut caller,Grant{maximum_items:1,maximum_copy_bytes:128,..stock()},&mut born,&mut free,|cx|{
                        match job.as_mut().unwrap().step(cx) {
                            Err(_)=>failed=true,
                            Ok((neural_engine::OperatorJobStep::Working(_),_))=>{},
                            Ok(_)=>panic!("invalid perforated extrusion published"),
                        }
                        Ok(())
                    });
                    if failed{break;}
                }
                assert!(failed);
                close_live_job(&mut job,&mut caller,&mut born,&mut free);
                assert_eq!(counts(),before);
            }
            let mut job=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..1024 {
                run(&mut caller,Grant{maximum_items:1,maximum_copy_bytes:128,..stock()},&mut born,&mut free,|cx|{
                    assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                    Ok(())
                });
                if counts()[4]>before[4]{break;}
            }
            assert!(counts()[4]>before[4]);assert_eq!(counts()[0],before[0]);
            job.as_mut().unwrap().cancel();
            run(&mut caller,stock(),&mut born,&mut free,|cx|{
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let extruded=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let extrusion_handle=read_geometry(&extruded,"solid").unwrap();
        let geometry=run_funded_preview(&session,identity,extrusion_handle.as_str(),&mut caller,&mut born,&mut free);
        geometry.validate_component_references().unwrap();
        assert_eq!(geometry.component_references["face"].len(),7);
        let [low,high]=[extrusion["levels"][0].as_f64().unwrap(),extrusion["levels"][1].as_f64().unwrap()];
        let bounds=geometry.aabb();
        assert_eq!([f64::from(bounds.0[2]),f64::from(bounds.1[2])],[low,high]);
        assert!((preview_area_volume(&geometry).1-extrusion["volume"].as_f64().unwrap()).abs()<0.15);
        for level in [low,high] {
            assert!(preview_distance(&geometry,[x,y,level])>radius as f32-0.05);
        }
    }
    caller.retire(stock());
}

#[test]
fn original_box_edge_fillet_picks_exact_cylinders_and_rolls_back_the_draft() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,text_dictionary,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🛠️brep-feature/🔣️.json")).unwrap();
    let rows=corpus["retainedBoxEdgeFillet"].as_array().unwrap().iter().chain(corpus["retainedMultiBoxFillet"].as_array().unwrap());
    for (i,row) in rows.enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98631+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [width,depth,height]=fixture_xyz(&row["size"]);
        let stock_input=Dictionary::new().insert("width",number(width)).insert("depth",number(depth)).insert("height",number(height));
        let raw=retained_boolean_result(&registry,"brep.prim3d.box",stock_input,&mut caller,&mut born,&mut free);
        let placement=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let placed=retained_boolean_result(&registry,"brep.xform.translate",placement,&mut caller,&mut born,&mut free);
        let original=read_geometry(&placed,"geometryOut").unwrap();
        let preview=run_funded_preview(&session,identity,original.as_str(),&mut caller,&mut born,&mut free);
        let authored=row["edges"].as_array().map_or_else(||vec![&row["edge"]],|edges|edges.iter().collect());
        let axis=row["axis"].as_u64().unwrap() as usize;
        let (labels,perpendicular)=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let reach=body.reachable_from(&[EntityRef::Solid(kernel.geometry_solid(&original).unwrap())]);
            let endpoints=|&id|{
                let edge=body.edges.get(id).unwrap();
                let p=body.vertices.get(edge.v0).unwrap().position;
                let q=body.vertices.get(edge.v1).unwrap().position;
                (p,q,edge.label.0.to_string())
            };
            let labels=authored.iter().map(|edge|{
                let [a,b]=[&edge[0],&edge[1]].map(|p|Pnt3::from_array(fixture_xyz(p)));
                reach.edges.iter().map(endpoints).find_map(|(p,q,label)|((p==a&&q==b)||(p==b&&q==a)).then_some(label)).unwrap()
            }).collect::<Vec<_>>();
            let perpendicular=reach.edges.iter().map(endpoints).find_map(|(p,q,label)|
                (p.to_array()[axis]==q.to_array()[axis]).then_some(label)).unwrap();
            Ok((labels,perpendicular))
        }).unwrap();
        assert!(labels.iter().all(|label|preview.component_references["edge"].contains(label)));
        let geometry=placed.get("geometryOut").unwrap();
        let text=|text|Value::Dictionary(text_dictionary(text));
        let selector=|labels:&[String]|Dictionary::new().insert("brep",geometry.clone())
            .insert("edgeLabels",text(serde_json::to_string(labels).unwrap()))
            .insert("faceLabels",text("[]".to_owned())).insert("sourceHandle",text(String::new()));
        let selection=retained_boolean_result(&registry,"brep.brep",selector(&labels),&mut caller,&mut born,&mut free);
        let mixed=retained_boolean_result(&registry,"brep.brep",selector(&[labels[0].clone(),perpendicular]),&mut caller,&mut born,&mut free);
        let duplicate=retained_boolean_result(&registry,"brep.brep",selector(&[labels[0].clone(),labels[0].clone()]),&mut caller,&mut born,&mut free);
        let member=selection.get("selectedEdges").unwrap();
        let params=|source:&Value,radius|Dictionary::new().insert("geometry",source.clone()).insert("edges",member.clone()).insert("radius",number(radius));
        let operator="brep.solid.filletEdges";
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let radius=row["radius"].as_f64().unwrap();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let invalid=row["invalidRadius"].as_f64().unwrap_or(width.max(depth).max(height));
        let invalid_inputs=[params(geometry,0.),params(geometry,invalid),params(raw.get("solid").unwrap(),radius),
            params(geometry,radius).insert("edges",mixed.get("selectedEdges").unwrap().clone()),
            params(geometry,radius).insert("edges",duplicate.get("selectedEdges").unwrap().clone())];
        for invalid in invalid_inputs{
            let mut job=Some(admit_live_job(&registry,operator,invalid));
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                assert!(job.as_mut().unwrap().step(cx).is_err());
                Ok(())
            });
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let input=params(geometry,radius);
        let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
        for _ in 0..200{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (state,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(state,neural_engine::OperatorJobStep::Working(_)));assert!(receipt.fits(grant));
                Ok(())
            });
            if counts()[2]>before[2]{break;}
        }
        assert!(counts()[2]>before[2]);assert_eq!(counts()[0],before[0]);
        draft.as_mut().unwrap().cancel();
        run(&mut caller,stock(),&mut born,&mut free,|cx|{
            assert!(matches!(draft.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
            Ok(())
        });
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let output=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"solid").unwrap();
        let (face_labels,arcs)=session.with_kernel_read(|kernel|{
            assert!(kernel.geometry_solid(&original).is_ok());
            let body=kernel.representation();
            let reach=body.reachable_from(&[EntityRef::Solid(kernel.geometry_solid(&handle).unwrap())]);
            let n=4+authored.len();
            assert_eq!((reach.faces.len(),reach.edges.len(),reach.vertices.len()),(n+2,3*n,2*n));
            let cylinders=reach.faces.iter().filter_map(|&id| {
                let face=body.faces.get(id)?;
                let Surface::Cylinder{frame,radius:actual}=body.surfaces.get(face.surface)?else{return None;};
                assert_eq!(*actual,radius);assert_eq!(frame.z.to_array()[axis],1.);
                Some(face.label.0.to_string())
            }).collect::<Vec<_>>();
            assert_eq!(cylinders.len(),authored.len());
            let arcs=reach.edges.iter().filter_map(|&id|{
                let edge=body.edges.get(id)?;
                let Curve3::Circle{radius:actual,..}=body.curves3.get(edge.curve)?else{return None;};
                assert_eq!(*actual,radius);assert_eq!(edge.range,(0.,std::f64::consts::FRAC_PI_2));
                Some(edge.label.0.to_string())
            }).collect::<Vec<_>>();
            assert_eq!(arcs.len(),2*authored.len());
            Ok((cylinders,arcs))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(face_labels.iter().all(|label|mesh.component_references["face"].contains(label)));
        assert!(arcs.iter().all(|label|mesh.component_references["edge"].contains(label)));
        let pick_filleted_face=Dictionary::new().insert("brep",output.get("solid").unwrap().clone())
            .insert("edgeLabels",text("[]".to_owned())).insert("faceLabels",text(serde_json::to_string(&face_labels).unwrap()))
            .insert("sourceHandle",text(String::new()));
        let face=retained_boolean_result(&registry,"brep.brep",pick_filleted_face,&mut caller,&mut born,&mut free);
        let list=face.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
        let mut patch_arcs=Vec::new();
        for (index,label) in face_labels.iter().enumerate(){
            let picked=read_geometry(list,&index.to_string()).unwrap();
            let patch=run_funded_preview(&session,identity,picked.as_str(),&mut caller,&mut born,&mut free);
            patch.validate_component_references().unwrap();
            assert_eq!(&patch.component_references["face"],std::slice::from_ref(label));
            patch_arcs.extend(patch.component_references["edge"].iter().filter(|label|arcs.contains(label)).cloned());
        }
        assert_eq!(patch_arcs.len(),arcs.len());
        assert!(arcs.iter().all(|label|patch_arcs.contains(label)));
        let (area,volume)=preview_area_volume(&mesh);
        assert!((area-row["area"].as_f64().unwrap()).abs()<0.09);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<0.08);
        let input=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
        let volume=retained_boolean_result(&registry,"brep.measure.volume",input,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-row["volume"].as_f64().unwrap()).abs()<1e-10);
        caller.retire(stock());
    }
}
