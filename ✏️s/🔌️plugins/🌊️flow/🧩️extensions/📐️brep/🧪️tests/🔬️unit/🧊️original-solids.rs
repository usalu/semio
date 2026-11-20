//! 🧊️ Original BRep solid construction, features, exact sections and transformations.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn circle_live_registry_extrusion_covers_axis_reversal_preview_and_cancellation() {
    use flow_extension_sdk::geometry::{GeometryStep, GeometryWorkIdentity};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json")).unwrap();
    for (name, cancelled, via_face) in [
        ("extrude a circular curve into a cylinder", false, false),
        ("extrude a circular curve into a cylinder", false, true),
        ("extrude against a tilted circle normal", false, false),
        ("extrude a circular curve into a cylinder", true, false),
        ("refuse an open line as an extrusion profile", false, false),
        ("refuse a circular extrusion in the profile plane", false, false),
    ] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = GeometryWorkIdentity { operation: 97150, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let profile = &case["inputs"]["wire"];
        let constructor = profile["kind"].as_str().unwrap();
        let mut parameters = Dictionary::new();
        for (key, value) in profile["inputs"].as_object().unwrap() {
            let value = if value.is_array() {
                Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(value)))
            } else {
                number(value.as_f64().unwrap())
            };
            parameters = parameters.insert(key.as_str(), value);
        }
        let profile = run_live_query(&registry, constructor, parameters, &mut caller, &mut born, &mut free);
        let profile_handle = semio_s_spatial_kernel_semio_session::read_geometry(&profile, "curve").unwrap();
        let curve = profile.get("curve").unwrap().clone();
        let (operator, inputs) = if via_face {
            let face = run_live_query(&registry, "brep.util.faceFromWire", Dictionary::new().insert("wire", curve), &mut caller, &mut born, &mut free);
            ("brep.sweep.extrude", Dictionary::new().insert("face", face.get("face").unwrap().clone()))
        } else {
            ("brep.solid.extrude", Dictionary::new().insert("wire", curve))
        };
        let inputs = inputs.insert("vector", Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(fixture_xyz(&case["inputs"]["vector"]))));

        if case.get("fault").is_some() {
            let mut job = Some(admit_live_job(&registry, operator, inputs));
            let mut refused = false;
            for _ in 0..100 {
                run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                    match job.as_mut().unwrap().step(cx) {
                        Ok((neural_engine::OperatorJobStep::Working(_), _)) => {}
                        Err(_) => refused = true,
                        _ => panic!("{name} published an invalid extrusion"),
                    }
                    Ok(())
                });
                if refused { break; }
            }
            assert!(refused, "{name}");
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap(), std::collections::HashSet::from([profile_handle.as_str().to_owned()]));
        } else if cancelled {
            let mut job = Some(admit_live_job(&registry, operator, inputs));
            for _ in 0..11 {
                run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                    let (step, _) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    Ok(())
                });
            }
            caller.cancel().cancel_now();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                let (step, _) = job.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap(), std::collections::HashSet::from([profile_handle.as_str().to_owned()]));
        } else {
            let solid = run_live_query(&registry, operator, inputs, &mut caller, &mut born, &mut free);
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&solid, "solid").unwrap();
            let volume = run_live_query(&registry, "brep.measure.volume", Dictionary::new().insert("geometry", solid.get("solid").unwrap().clone()), &mut caller, &mut born, &mut free);
            let volume = semio_s_spatial_kernel_semio_session::read_channel_number(&volume, "volume").unwrap();
            let expected = &case["outputs"]["shape"];
            assert!((volume - expected["volume"].as_f64().unwrap()).abs() < case["tolerance"].as_f64().unwrap(), "{name}");

            let tolerance = 0.025;
            let mut mesh = None;
            for _ in 0..10000 {
                let demand = session.tessellation_demands(identity, handle.as_str(), tolerance, 128).unwrap();
                let grant = Grant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(128), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = session.tessellate_step(identity, handle.as_str(), tolerance, cx).unwrap();
                    assert!(receipt.retained.fits(grant));
                    match step {
                        GeometryStep::Ready(geometry) => mesh = Some(geometry),
                        GeometryStep::Working { .. } => {}
                        _ => panic!("{name} preview stopped before producing a mesh"),
                    }
                    Ok(())
                });
                if mesh.is_some() { break; }
            }
            let mesh = mesh.expect("extruded cylinder has a visible preview");
            assert!(mesh.indices.len() >= 12);
            assert!(!mesh.edge_positions.is_empty());
            for (side, reduce) in [(0, f32::min as fn(f32, f32) -> f32), (1, f32::max)] {
                let corner = |axis| mesh.positions[axis..].iter().step_by(3).copied().reduce(reduce).unwrap() as f64;
                let bounds = fixture_xyz(&expected["bbox"][side]);
                assert!((0..3).all(|axis| (corner(axis) - bounds[axis]).abs() <= tolerance), "{name} bounds");
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_curved_primitives_publish_their_exact_surface_topology() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solids.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    for (index, case) in corpus["curved"].as_array().unwrap().iter().enumerate() {
        let operator = case["operator"].as_str().unwrap();
        let input = case["values"].as_object().unwrap().iter()
            .fold(Dictionary::new(), |input, (key, value)| input.insert(key.as_str(), number(value.as_f64().unwrap())));
        let mut job = Some(admit_live_job(&registry, operator, input));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(97150 + index as u64), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let mut output = None;
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        for _ in 0..2000 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                if let neural_engine::OperatorJobStep::Done(result) = job.as_mut().unwrap().step(cx).unwrap().0 { output = Some(result); }
                Ok(())
            });
            if output.is_some() { break; }
        }
        let output = neural_engine::ColdOwner::new(output.unwrap_or_else(|| panic!("{operator} must complete under a one-item grant")));
        let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, "solid").unwrap();
        let expected_volume = case["volume"].as_f64().unwrap();
        let area = run_live_query(&registry,"brep.measure.area",Dictionary::new().insert("geometry",output.get("solid").unwrap().clone()),&mut caller,&mut born,&mut free);
        let actual_area = semio_s_spatial_kernel_semio_session::read_channel_number(&area,"area").unwrap();
        assert!((actual_area-case["area"].as_f64().unwrap()).abs()<1e-12);
        let center = run_live_query(&registry,"brep.measure.centerOfMass",Dictionary::new().insert("geometry",output.get("solid").unwrap().clone()),&mut caller,&mut born,&mut free);
        assert_eq!(semio_s_spatial_kernel_semio_session::read_xyz(&center,"center").unwrap(),fixture_xyz(&case["center"]));
        session.with_kernel_read(|kernel| {
            let map = |error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error);
            let solid = kernel.geometry_solid(&handle).map_err(map)?;
            assert_eq!(kernel.representation().solid_faces(solid).len(), case["faceCount"].as_u64().unwrap() as usize);
            let actual = kernel.volume_sync(&handle).map_err(map)?;
            assert!((actual / expected_volume - 1.0).abs() < case["relativeTolerance"].as_f64().unwrap(),
                "{operator} volume {actual} differs from {expected_volume}");
            Ok(())
        }).unwrap();
        let geometry = output.get("solid").unwrap().clone();
        let input = Dictionary::new().insert("geometry", geometry);
        let mut volume = Some(admit_live_job(&registry, "brep.measure.volume", input));
        let mut measurement = None;
        for _ in 0..2000 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                if let neural_engine::OperatorJobStep::Done(result) = volume.as_mut().unwrap().step(cx).unwrap().0 {
                    measurement = Some(result);
                }
                Ok(())
            });
            if measurement.is_some() { break; }
        }
        let measurement = neural_engine::ColdOwner::new(measurement.expect("retained Volume produces a number"));
        let measured = semio_s_spatial_kernel_semio_session::read_channel_number(&measurement, "volume").unwrap();
        assert!((measured / expected_volume - 1.0).abs() < 1e-12);
        close_live_job(&mut volume, &mut caller, &mut born, &mut free);

        use flow_extension_sdk::geometry::{GeometryStep, GeometryWorkIdentity};
        let identity = GeometryWorkIdentity { operation: 97150 + index as u64, generation: 17 };
        let tolerance = 0.025;
        let mut preview = None;
        for _ in 0..20000 {
            let demand = session.tessellation_demands(identity, handle.as_str(), tolerance, 128).unwrap();
            let loan = Grant {
                maximum_items: 1,
                maximum_copy_bytes: demand.copy_bytes.max(128),
                maximum_capacity_bytes: demand.capacity_bytes,
                maximum_release_bytes: demand.release_bytes,
                maximum_depth: demand.depth.max(1),
            };
            run(&mut caller, loan, &mut born, &mut free, |cx| {
                let (step, work) = session.tessellate_step(identity, handle.as_str(), tolerance, cx).unwrap();
                assert!(work.consumed_units <= 1);
                match step {
                    GeometryStep::Ready(mesh) => preview = Some(mesh),
                    GeometryStep::Working { .. } => {},
                    _ => panic!("{operator} must tessellate its published handle"),
                }
                Ok(())
            });
            if preview.is_some() { break; }
        }
        let preview = preview.expect("analytic solid preview completed");
        preview.validate_component_references().unwrap();
        session.with_kernel_read(|kernel| {
            use semio_framework_3d_brep_representation::topology::EntityRef;
            let solid = kernel.geometry_solid(&handle).map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))?;
            let body = kernel.representation();
            let reachable = body.reachable_from(&[EntityRef::Solid(solid)]);
            let face_labels: Vec<_> = body.solid_faces(solid).into_iter()
                .map(|id| body.faces.get(id).unwrap().label.0.to_string()).collect();
            assert_eq!(&preview.component_references["face"], &face_labels);
            let edge_labels = &preview.component_references["edge"];
            assert!(edge_labels.iter().all(|label| reachable.edges.iter().any(|id| &body.edges.get(*id).unwrap().label.0.to_string() == label)));
            let vertices = &preview.component_references["vertex"];
            assert_eq!(vertices.len(), reachable.vertices.len());
            assert!(vertices.iter().all(|label| reachable.vertices.iter().any(|id| &body.vertices.get(*id).unwrap().label.0.to_string() == label)));
            Ok(())
        }).unwrap();
        assert!(!preview.edge_positions.is_empty());
        assert!(preview.edge_positions.chunks_exact(6).all(|edge| edge[..3] != edge[3..]));
        let vector = |values: &[f32], index: u32| {
            let offset = index as usize * 3;
            semio_framework_3d_brep_vector::Vec3::new(values[offset].into(), values[offset + 1].into(), values[offset + 2].into())
        };
        let mut signed_volume = 0.0;
        for ids in preview.indices.chunks_exact(3) {
            let [a, b, c] = [ids[0], ids[1], ids[2]].map(|id| vector(&preview.positions, id));
            let normal = [ids[0], ids[1], ids[2]].map(|id| vector(&preview.normals, id));
            assert!((b - a).cross(c - a).dot(normal[0] + normal[1] + normal[2]) > 0.0, "{operator} preview winding");
            signed_volume += a.dot(b.cross(c)) / 6.0;
        }
        assert!((signed_volume / expected_volume - 1.0).abs() < 0.06, "{operator} preview volume");
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        caller.retire(stock());
    }
}

#[test]
fn certified_cylinder_transforms_preserve_live_topology_and_mass() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solids.json")).unwrap();
    let row = &fixture["cylinderAffine"];
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97156), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born,mut free)=(0,0);
    let vector = |v| Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(fixture_xyz(v)));
    let solid = run_live_query(&registry,"brep.prim3d.cylinder",Dictionary::new().insert("radius",number(2.0)).insert("height",number(4.0)),&mut caller,&mut born,&mut free);
    let moved = run_live_query(&registry,"brep.xform.translate",Dictionary::new().insert("geometry",solid.get("solid").unwrap().clone()).insert("offset",vector(&row["offset"])),&mut caller,&mut born,&mut free);
    let rotated = run_live_query(&registry,"brep.xform.rotate",Dictionary::new().insert("geometry",moved.get("geometryOut").unwrap().clone()).insert("axis",vector(&row["axis"])).insert("angle",number(row["angle"].as_f64().unwrap())),&mut caller,&mut born,&mut free);
    let handle=semio_s_spatial_kernel_semio_session::read_geometry(&rotated,"geometryOut").unwrap();
    let center=session.original_primitive_center_of_mass(handle.as_str()).unwrap();
    assert!(center.into_iter().zip(fixture_xyz(&row["center"])).all(|(a,b)| (a-b).abs()<1e-12));
    let volume=session.original_primitive_volume(handle.as_str()).unwrap().unwrap();
    assert!((volume-16.0*std::f64::consts::PI).abs()<1e-12);
    caller.retire_exact();
}

#[test]
fn certified_box_booleans_publish_at_the_original_world_coordinates() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-booleans.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97162), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    for (case_group, simple) in [(&fixture, true), (&fixture["orthogonal"], false), (&fixture["cavity"], false)] {
        let [a, b] = ["a", "b"].map(|name| {
            let spec = &case_group[name];
            let input = ["width", "depth", "height"].into_iter().enumerate()
                .fold(Dictionary::new(), |input, (axis, port)| input.insert(port, number(spec["size"][axis].as_f64().unwrap())));
            let original = retained_boolean_result(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free);
            let offset = std::array::from_fn(|i| spec["origin"][i].as_f64().unwrap());
            let input = Dictionary::new().insert("geometry", original.get("solid").unwrap().clone())
                .insert("offset", Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(offset)));
            retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free)
        });
        for case in case_group["cases"].as_array().unwrap() {
            let input = Dictionary::new().insert("a", a.get("geometryOut").unwrap().clone()).insert("b", b.get("geometryOut").unwrap().clone());
            let operator = case["operator"].as_str().unwrap();
            let result = retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&result, "solid").unwrap();
            session.with_kernel_read(|kernel| {
                let solid = kernel.geometry_solid(&handle).unwrap();
                let body = kernel.representation();
                let expected_inners = case["innerShells"].as_u64().unwrap_or(0) as usize;
                assert_eq!(body.solids.get(solid).unwrap().inners.len(), expected_inners);
                let roots = body.reachable_from(&[semio_framework_3d_brep_representation::topology::EntityRef::Solid(solid)]);
                if simple {
                    assert_eq!(roots.vertices.len(), 8);
                    assert_eq!(roots.faces.len(), 6);
                } else {
                    assert!(roots.vertices.len() > 8 && roots.faces.len() > 6);
                    assert!(roots.edges.iter().all(|edge| {
                        let uses = body.edge_coedges(*edge);
                        uses.len() == 2 && body.coedges.get(uses[0]).unwrap().forward != body.coedges.get(uses[1]).unwrap().forward
                    }));
                }
                for axis in 0..3 {
                    let coordinates = roots.vertices.iter().map(|id| {
                        let point = body.vertices.get(*id).unwrap().position;
                        [point.x, point.y, point.z][axis]
                    });
                    assert_eq!(coordinates.clone().fold(f64::INFINITY, f64::min), case["minimum"][axis].as_f64().unwrap());
                    assert_eq!(coordinates.fold(f64::NEG_INFINITY, f64::max), case["maximum"][axis].as_f64().unwrap());
                }
                Ok(())
            }).unwrap();
            let input = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
            let volume = retained_boolean_result(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
            assert_eq!(semio_s_spatial_kernel_semio_session::read_channel_number(&volume, "volume").unwrap(), case["volume"].as_f64().unwrap());
        }
    }
    caller.retire(stock());
}

#[test]
fn certified_coaxial_cylinder_booleans_publish_native_solids() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-booleans.json")).unwrap();
    let fixture = &corpus["cylinders"];
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97163), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let [a, b] = ["aHeight", "bHeight"].map(|name| {
        let input = Dictionary::new().insert("radius", number(fixture["radius"].as_f64().unwrap()))
            .insert("height", number(fixture[name].as_f64().unwrap()));
        retained_boolean_result(&registry, "brep.prim3d.cylinder", input, &mut caller, &mut born, &mut free)
    });
    let offset = [0.0, 0.0, fixture["bStart"].as_f64().unwrap()];
    let input = Dictionary::new().insert("geometry", b.get("solid").unwrap().clone())
        .insert("offset", Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(offset)));
    let placed = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
    for case in fixture["cases"].as_array().unwrap() {
        let source = Dictionary::new().insert("a", a.get("solid").unwrap().clone()).insert("b", placed.get("geometryOut").unwrap().clone());
        let result = retained_boolean_result(&registry, case["operator"].as_str().unwrap(), source, &mut caller, &mut born, &mut free);
        let measure = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let volume = retained_boolean_result(&registry, "brep.measure.volume", measure, &mut caller, &mut born, &mut free);
        let measured = semio_s_spatial_kernel_semio_session::read_channel_number(&volume, "volume").unwrap();
        assert!((measured - case["volume"].as_f64().unwrap()).abs() < 1e-12);
    }
    caller.retire(stock());
}

#[test]
fn general_boolean_acquires_source_faces_before_the_unfunded_imprint_boundary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-booleans.json")).unwrap();
    let acquisition = &fixture["acquisition"];
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97164), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let params = Dictionary::new().insert("width", number(2.0)).insert("depth", number(3.0)).insert("height", number(4.0));
    let a = retained_boolean_result(&registry, "brep.prim3d.box", params, &mut caller, &mut born, &mut free);
    let params = Dictionary::new().insert("radius", number(0.5)).insert("height", number(6.0));
    let b = retained_boolean_result(&registry, "brep.prim3d.cylinder", params, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("a", a.get("solid").unwrap().clone()).insert("b", b.get("solid").unwrap().clone());
    let mut job = Some(admit_live_job(&registry, acquisition["operator"].as_str().unwrap(), input));
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let mut acquired = 0;
    let mut refused = false;
    for _ in 0..100 {
        let current = job.as_ref().unwrap();
        let acquiring = current.progress().phase == "boolean-acquire-faces";
        let copied = current.next_step_copy_byte_demand().unwrap();
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            match job.as_mut().unwrap().step(cx) {
                Ok((neural_engine::OperatorJobStep::Working(_), receipt)) => {
                    if acquiring && copied != 0 {
                        assert_eq!(receipt.copied_bytes, copied);
                        acquired += 1;
                    }
                }
                Err(_) => refused = true,
                _ => panic!("a curved general Boolean cannot publish before imprinting is funded"),
            }
            Ok(())
        });
        if refused { break; }
    }
    assert!(refused);
    assert_eq!(acquired, (acquisition["boxFaces"].as_u64().unwrap() + acquisition["cylinderFaces"].as_u64().unwrap()) as usize);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), 2);
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    caller.retire(stock());
}

#[test]
fn original_box_cylinder_cut_publishes_an_exact_through_bore() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-booleans.json")).unwrap();
    let fixture = &corpus["bore"];
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97165), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let stock_input = ["width", "depth", "height"].into_iter().enumerate()
        .fold(Dictionary::new(), |input, (axis, port)| input.insert(port, number(fixture["stock"]["size"][axis].as_f64().unwrap())));
    let tool_input = Dictionary::new().insert("radius", number(fixture["tool"]["radius"].as_f64().unwrap()))
        .insert("height", number(fixture["tool"]["length"].as_f64().unwrap()));
    let [a,b] = [("stock", "brep.prim3d.box", stock_input), ("tool", "brep.prim3d.cylinder", tool_input)].map(|(def, op, input)| {
        let result = retained_boolean_result(&registry, op, input, &mut caller, &mut born, &mut free);
        let offset = std::array::from_fn(|axis| fixture[def]["origin"][axis].as_f64().unwrap());
        let input = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone())
            .insert("offset", Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(offset)));
        retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free)
    });
    let input = Dictionary::new().insert("a", a.get("geometryOut").unwrap().clone()).insert("b", b.get("geometryOut").unwrap().clone());
    let result = retained_boolean_result(&registry, "brep.bool.cut", input, &mut caller, &mut born, &mut free);
    let handle = semio_s_spatial_kernel_semio_session::read_geometry(&result, "solid").unwrap();
    session.with_kernel_read(|kernel| {
        let solid = kernel.geometry_solid(&handle).unwrap();
        let body = kernel.representation();
        let faces = body.solid_faces(solid);
        assert_eq!(faces.len(), fixture["faces"].as_u64().unwrap() as usize);
        let (certified, preview) = kernel.live_bore_preview(handle.as_str()).expect("live bore has a stable certificate");
        assert_eq!(certified, solid);
        assert_eq!(body.faces.get(faces[6]).unwrap().label.0, preview.face_label(6));
        assert_eq!(faces.iter().filter(|&&id| body.faces.get(id).unwrap().flipped).count(), 1);
        assert_eq!(faces.iter().map(|&id| body.faces.get(id).unwrap().inners.len()).sum::<usize>(), fixture["holes"].as_u64().unwrap() as usize);
        let roots = body.reachable_from(&[semio_framework_3d_brep_representation::topology::EntityRef::Solid(solid)]);
        assert_eq!(roots.vertices.len(), 10);
        assert_eq!(roots.edges.len(), 15);
        for edge in roots.edges {
            let uses = body.edge_coedges(edge);
            assert_eq!(uses.len(), 2);
            let effective_direction = |id| {
                let coedge = body.coedges.get(id).unwrap();
                let face_id = body.loops.get(coedge.loop_id).unwrap().face;
                coedge.forward != body.faces.get(face_id).unwrap().flipped
            };
            assert_ne!(effective_direction(uses[0]), effective_direction(uses[1]));
        }
        Ok(())
    }).unwrap();
    let input = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
    let volume = retained_boolean_result(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
    let measured = semio_s_spatial_kernel_semio_session::read_channel_number(&volume, "volume").unwrap();
    assert!((measured - fixture["volume"].as_f64().unwrap()).abs() < 1e-12);
    let identity=flow_extension_sdk::geometry::GeometryWorkIdentity{operation:97165,generation:17};
    let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
    mesh.validate_component_references().unwrap();
    for (domain,count) in [("face",7),("edge",15),("vertex",10)] {
        assert_eq!(mesh.component_references[domain].len(),count);
    }
    let (_,mesh_volume)=preview_area_volume(&mesh);
    assert!(mesh_volume>=measured-1e-5 && mesh_volume-measured<0.05);
    caller.retire(stock());
}

#[test]
fn original_revolve_constructs_a_placed_torus_from_a_live_circular_face() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, read_xyz, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-revolve.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97166), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let xyz = |value: &serde_json::Value| std::array::from_fn(|axis| value[axis].as_f64().unwrap());
    let result = |operator, input, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize| {
        retained_boolean_result(&registry, operator, input, caller, born, free)
    };
    let circle_input = Dictionary::new().insert("center", Value::Dictionary(point_dictionary(xyz(&fixture["profile"]["center"]))))
        .insert("normal", Value::Dictionary(vector_dictionary(xyz(&fixture["profile"]["normal"]))))
        .insert("radius", number(fixture["profile"]["radius"].as_f64().unwrap()));
    let circle = result("brep.curve.circle", circle_input, &mut caller, &mut born, &mut free);
    let face_input = Dictionary::new().insert("wire", circle.get("curve").unwrap().clone());
    let face = result("brep.util.faceFromWire", face_input, &mut caller, &mut born, &mut free);
    let angle = fixture["angle"].as_f64().unwrap();
    let axis = xyz(&fixture["axis"]["origin"]);
    let direction = xyz(&fixture["axis"]["direction"]);
    let input = |origin, angle| Dictionary::new().insert("face", face.get("face").unwrap().clone())
        .insert("axisOrigin", Value::Dictionary(point_dictionary(origin)))
        .insert("axisDirection", Value::Dictionary(vector_dictionary(direction)))
        .insert("angle", number(angle));

    for sweep in [angle, -angle] {
        let solid = result(fixture["operator"].as_str().unwrap(), input(axis, sweep), &mut caller, &mut born, &mut free);
        let handle = read_geometry(&solid, "solid").unwrap();
        session.with_kernel_read(|kernel| {
            let id = kernel.geometry_solid(&handle).unwrap();
            let body = kernel.representation();
            let reach = body.reachable_from(&[semio_framework_3d_brep_representation::topology::EntityRef::Solid(id)]);
            assert_eq!((reach.vertices.len(), reach.edges.len(), reach.faces.len()), (1, 2, 1));
            let vertex = body.vertices.get(*reach.vertices.iter().next().unwrap()).unwrap().position;
            assert_eq!([vertex.x, vertex.y, vertex.z], xyz(&fixture["expected"]["seam"]));
            let face = body.faces.get(*reach.faces.iter().next().unwrap()).unwrap();
            let surface = body.surfaces.get(face.surface).unwrap();
            let point = surface.eval(0.0, 0.0);
            assert_eq!([point.x, point.y, point.z], xyz(&fixture["expected"]["seam"]));
            Ok(())
        }).unwrap();
        let input = || Dictionary::new().insert("geometry", solid.get("solid").unwrap().clone());
        let volume = result("brep.measure.volume", input(), &mut caller, &mut born, &mut free);
        let actual = read_channel_number(&volume, "volume").unwrap();
        assert!((actual - fixture["expected"]["volume"].as_f64().unwrap()).abs() < 1e-12);
        let center = result("brep.measure.centerOfMass", input(), &mut caller, &mut born, &mut free);
        assert_eq!(read_xyz(&center, "center").unwrap(), xyz(&fixture["expected"]["center"]));
    }
    for rejected in [input(axis, fixture["unsupported"]["angle"].as_f64().unwrap()), input(xyz(&fixture["unsupported"]["axisOrigin"]), angle)] {
        let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
        let mut job = Some(admit_live_job(&registry, fixture["operator"].as_str().unwrap(), rejected));
        let mut refused = false;
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
            refused = job.as_mut().unwrap().step(cx).is_err();
            Ok(())
        });
        assert!(refused);
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    }
    caller.retire(stock());
}

#[test]
fn original_thicken_respects_the_signed_outward_normal_of_planar_faces() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-thicken.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97167), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    for row in fixture["cases"].as_array().unwrap() {
        let input = row["inputs"].as_object().unwrap().iter().fold(Dictionary::new(), |input, (key, value)| {
            let value = match key.as_str() {
                "center" => Value::Dictionary(point_dictionary(fixture_xyz(value))),
                "normal" => Value::Dictionary(vector_dictionary(fixture_xyz(value))),
                _ => number(value.as_f64().unwrap()),
            };
            input.insert(key.as_str(), value)
        });
        let profile = retained_boolean_result(&registry, row["constructor"].as_str().unwrap(), input, &mut caller, &mut born, &mut free);
        let input = Dictionary::new().insert("wire", profile.get(row["boundary"].as_str().unwrap()).unwrap().clone());
        let face = retained_boolean_result(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free);
        let source = read_geometry(&face, "face").unwrap();
        let input = |thickness| Dictionary::new().insert("face", face.get("face").unwrap().clone())
            .insert("thickness", number(thickness));
        let operator = fixture["operator"].as_str().unwrap();
        let solid = retained_boolean_result(&registry, operator, input(row["thickness"].as_f64().unwrap()), &mut caller, &mut born, &mut free);
        let handle = read_geometry(&solid, "solid").unwrap();
        let axis = row["axis"].as_u64().unwrap() as usize;
        session.with_kernel_read(|kernel| {
            assert!(kernel.label_of(&source).is_some());
            let solid = kernel.geometry_solid(&handle).unwrap();
            let body = kernel.representation();
            let reach = body.reachable_from(&[semio_framework_3d_brep_representation::topology::EntityRef::Solid(solid)]);
            assert_eq!(reach.faces.len(), row["faces"].as_u64().unwrap() as usize);
            let (low, high) = reach.vertices.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &id| {
                let p = body.vertices.get(id).unwrap().position;
                let coordinate = [p.x, p.y, p.z][axis];
                (low.min(coordinate), high.max(coordinate))
            });
            assert!((low - row["bounds"][0][axis].as_f64().unwrap()).abs() < 1e-12);
            assert!((high - row["bounds"][1][axis].as_f64().unwrap()).abs() < 1e-12);
            Ok(())
        }).unwrap();
        let measure = Dictionary::new().insert("geometry", solid.get("solid").unwrap().clone());
        let volume = retained_boolean_result(&registry, "brep.measure.volume", measure, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-12);

        let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        let mut cancelled = Some(admit_live_job(&registry, operator, input(row["thickness"].as_f64().unwrap())));
        for _ in 0..10 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, receipt) = cancelled.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                assert!(receipt.copied_items <= 1);
                Ok(())
            });
        }
        cancelled.as_mut().unwrap().cancel();
        run(&mut caller, stock(), &mut born, &mut free, |cx| {
            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
            Ok(())
        });
        close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
        let mut rejected = Some(admit_live_job(&registry, operator, input(0.0)));
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            assert!(rejected.as_mut().unwrap().step(cx).is_err());
            Ok(())
        });
        close_live_job(&mut rejected, &mut caller, &mut born, &mut free);
        assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    }
    caller.retire(stock());
}

#[test]
fn original_solid_offset_preserves_sharp_box_topology_and_world_placement() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solid-offset.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97169, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let operator = fixture["operator"].as_str().unwrap();
    let size = fixture_xyz(&fixture["source"]["size"]);
    let init = ["width", "depth", "height"].into_iter().zip(size).fold(Dictionary::new(), |input, (axis, extent)| input.insert(axis, number(extent)));
    let box_shape = retained_boolean_result(&registry, "brep.prim3d.box", init, &mut caller, &mut born, &mut free);
    let origin = fixture_xyz(&fixture["source"]["origin"]);
    let input = Dictionary::new().insert("geometry", box_shape.get("solid").unwrap().clone())
        .insert("offset", Value::Dictionary(vector_dictionary(origin)));
    let source = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
    let source = source.get("geometryOut").unwrap();
    let offset_input = |distance| Dictionary::new().insert("geometry", source.clone()).insert("distance", number(distance));

    for row in fixture["cases"].as_array().unwrap() {
        let result = retained_boolean_result(&registry, operator, offset_input(row["distance"].as_f64().unwrap()), &mut caller, &mut born, &mut free);
        let handle = read_geometry(&result, "solid").unwrap();
        let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        let bounds = mesh.aabb();
        assert_eq!(bounds.0.map(f64::from), fixture_xyz(&row["bounds"][0]));
        assert_eq!(bounds.1.map(f64::from), fixture_xyz(&row["bounds"][1]));
        for (domain, count) in [("face", 6), ("edge", 12), ("vertex", 8)] {
            assert_eq!(mesh.component_references[domain].len(), count);
        }
        let input = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let measured = retained_boolean_result(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
        assert_eq!(read_channel_number(&measured, "volume").unwrap(), row["volume"].as_f64().unwrap());
    }

    let input = Dictionary::new().insert("geometry", source.clone());
    let measured = retained_boolean_result(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
    assert_eq!(read_channel_number(&measured, "volume").unwrap(), fixture["source"]["volume"].as_f64().unwrap());
    let input = Dictionary::new().insert("radius", number(2.0));
    let sphere = retained_boolean_result(&registry, "brep.prim3d.sphere", input, &mut caller, &mut born, &mut free);
    let unsupported = Dictionary::new().insert("geometry", sphere.get("solid").unwrap().clone()).insert("distance", number(0.5));
    let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
    for invalid in [offset_input(fixture["collapse"].as_f64().unwrap()), offset_input(0.0), unsupported] {
        let mut job = Some(admit_live_job(&registry, operator, invalid));
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
            assert!(job.as_mut().unwrap().step(cx).is_err());
            Ok(())
        });
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
    }
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    caller.retire(stock());
}

#[test]
fn original_box_shell_keeps_picked_openings_and_oriented_cavity_topology() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::{EntityRef, history::PersistentLabel};
    use semio_s_spatial_kernel_semio_session::{geometry_dict, read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-shell.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97172, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let origin = fixture_xyz(&fixture["origin"]);
    let size = fixture_xyz(&fixture["size"]);
    let input = ["width", "depth", "height"].into_iter().zip(size)
        .fold(Dictionary::new(), |input, (axis, extent)| input.insert(axis, number(extent)));
    let canonical = retained_boolean_result(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("geometry", canonical.get("solid").unwrap().clone())
        .insert("offset", Value::Dictionary(vector_dictionary(origin)));
    let source = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
    let handle = read_geometry(&source, "geometryOut").unwrap();
    let source_mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
    let box_faces = ["-z", "+z", "-y", "+y", "-x", "+x"];
    let operator = fixture["operator"].as_str().unwrap();
    let input = |thickness| Dictionary::new().insert("geometry", source.get("geometryOut").unwrap().clone())
        .insert("thickness", number(thickness));

    for row in fixture["cases"].as_array().unwrap() {
        let openings = row["open"].as_array().unwrap();
        let mut parameters = input(fixture["thickness"].as_f64().unwrap());
        if !openings.is_empty() {
            let selected = openings.iter().enumerate().fold(Dictionary::with_schema("list"), |entries, (index, direction)| {
                let slot = box_faces.iter().position(|&face| Some(face) == direction.as_str()).unwrap();
                let label = PersistentLabel(source_mesh.component_references["face"][slot].parse::<u64>().unwrap());
                let payload = session.with_kernel(|kernel| {
                    let handle = kernel.handle_for_label(label).unwrap();
                    geometry_dict(kernel, &handle)
                }).unwrap();
                entries.insert(index.to_string(), Value::Dictionary(payload))
            });
            parameters = parameters.insert("openFaces", Value::Dictionary(selected));
        }
        let result = retained_boolean_result(&registry, operator, parameters, &mut caller, &mut born, &mut free);
        let handle = read_geometry(&result, "solid").unwrap();
        let counts = session.with_kernel_read(|kernel| {
            let id = kernel.geometry_solid(&handle).unwrap();
            let body = kernel.representation();
            let solid = body.solids.get(id).unwrap();
            assert_eq!(1 + solid.inners.len(), row["shells"].as_u64().unwrap() as usize);
            let reach = body.reachable_from(&[EntityRef::Solid(id)]);
            let (vertices, edges, faces) = (reach.vertices.len(), reach.edges.len(), reach.faces.len());
            assert_eq!(vertices as i64 - edges as i64 + faces as i64, row["euler"].as_i64().unwrap());
            for &edge in &reach.edges {
                let uses = body.edge_coedges(edge);
                assert_eq!(uses.len(), 2);
                let orientation = |id| {
                    let use_ = body.coedges.get(id).unwrap();
                    let face = body.loops.get(use_.loop_id).unwrap().face;
                    use_.forward != body.faces.get(face).unwrap().flipped
                };
                assert_ne!(orientation(uses[0]), orientation(uses[1]));
            }
            for &shell in &solid.inners {
                for &face in &body.shells.get(shell).unwrap().faces {
                    let face = body.faces.get(face).unwrap();
                    let surface = body.surfaces.get(face.surface).unwrap();
                    let p = surface.eval(0.0, 0.0);
                    let normal = surface.normal(0.0, 0.0).unwrap();
                    let signed = [p.x, p.y, p.z].into_iter().zip([normal.x, normal.y, normal.z]).enumerate()
                        .map(|(axis, (coordinate, normal))| (coordinate - origin[axis] - size[axis] * 0.5) * normal)
                        .sum::<f64>();
                    assert!(if face.flipped { signed > 0.0 } else { signed < 0.0 });
                }
            }
            Ok([("face", faces), ("edge", edges), ("vertex", vertices)])
        }).unwrap();
        let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        for (kind, count) in counts {
            assert_eq!(mesh.component_references[kind].len(), count);
        }
        assert_eq!(mesh.aabb(), source_mesh.aabb());
        let measure = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let volume = retained_boolean_result(&registry, "brep.measure.volume", measure, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);
    }

    let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let mut rejected = Some(admit_live_job(&registry, operator, input(2.0)));
    let mut fault = false;
    for _ in 0..4 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            fault = rejected.as_mut().unwrap().step(cx).is_err();
            Ok(())
        });
        if fault { break; }
    }
    assert!(fault);
    close_live_job(&mut rejected, &mut caller, &mut born, &mut free);

    let mut cancelled = Some(admit_live_job(&registry, operator, input(fixture["thickness"].as_f64().unwrap())));
    for _ in 0..12 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
            Ok(())
        });
    }
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, stock(), &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    caller.retire(stock());
}

#[test]
fn original_point_hull_merges_coplanar_facets_and_publishes_only_a_closed_solid() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-convex-hull.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97173, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let operator = fixture["operator"].as_str().unwrap();
    let hull_input = |raw: &serde_json::Value| {
        let points = raw.as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"), |list, (index, point)| {
            list.insert(index.to_string(), Value::Dictionary(point_dictionary(fixture_xyz(point))))
        });
        Dictionary::new().insert("points", Value::Dictionary(points))
    };
    for row in fixture["cases"].as_array().unwrap() {
        let result = retained_boolean_result(&registry, operator, hull_input(&row["points"]), &mut caller, &mut born, &mut free);
        let shape = read_geometry(&result, "solid").unwrap();
        let counts = session.with_kernel_read(|kernel| {
            let id = kernel.geometry_solid(&shape).unwrap();
            let body = kernel.representation();
            assert!(body.solids.get(id).unwrap().inners.is_empty());
            let reach = body.reachable_from(&[EntityRef::Solid(id)]);
            for &id in &reach.faces {
                let face = body.faces.get(id).unwrap();
                assert!(face.inners.is_empty());
                assert_eq!(body.loop_coedges(face.outer.unwrap()).len(), row["faceSize"].as_u64().unwrap() as usize);
            }
            for &id in &reach.edges {
                let uses = body.edge_coedges(id);
                assert_eq!(uses.len(), 2);
                let direction = |id| body.coedges.get(id).unwrap().forward;
                assert_ne!(direction(uses[0]), direction(uses[1]));
            }
            Ok([("face", "faces", reach.faces.len()), ("edge", "edges", reach.edges.len()), ("vertex", "vertices", reach.vertices.len())])
        }).unwrap();
        let mesh = run_funded_preview(&session, identity, shape.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        for (domain, key, count) in counts {
            assert_eq!(count, row[key].as_u64().unwrap() as usize);
            assert_eq!(mesh.component_references[domain].len(), count);
        }
        let (min, max) = mesh.aabb();
        assert_eq!(min.map(f64::from), fixture_xyz(&row["bounds"][0]));
        assert_eq!(max.map(f64::from), fixture_xyz(&row["bounds"][1]));
        let input = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let volume = retained_boolean_result(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);
    }

    let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
    let low_cost = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let mut rejected = Some(admit_live_job(&registry, operator, hull_input(&fixture["coplanar"])));
    let mut fault = false;
    for _ in 0..40 {
        run(&mut caller, low_cost, &mut born, &mut free, |cx| {
            fault = rejected.as_mut().unwrap().step(cx).is_err();
            Ok(())
        });
        if fault { break; }
    }
    assert!(fault);
    close_live_job(&mut rejected, &mut caller, &mut born, &mut free);

    let mut cancelled = Some(admit_live_job(&registry, operator, hull_input(&fixture["cases"][1]["points"])));
    for _ in 0..20 {
        run(&mut caller, low_cost, &mut born, &mut free, |cx| {
            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
            Ok(())
        });
    }
    assert_eq!(cancelled.as_ref().unwrap().progress().phase, "convex-hull-support-planes");
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, stock(), &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    caller.retire(stock());
}

#[test]
fn original_straight_sweeps_use_directed_displacement_and_keep_profile_placement() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-straight-sweeps.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97174, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let bent = retained_boolean_result(&registry, "brep.curve.polyline", polygon_fixture_input(&serde_json::json!({"points": fixture["unsupportedPath"]})), &mut caller, &mut born, &mut free);
    for row in fixture["cases"].as_array().unwrap() {
        let (constructor, channel, profile_input) = if row["kind"] == "circle" {
            let input = Dictionary::new()
                .insert("center", Value::Dictionary(point_dictionary(fixture_xyz(&row["center"]))))
                .insert("normal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))))
                .insert("radius", number(row["radius"].as_f64().unwrap()));
            ("brep.curve.circle", "curve", input)
        } else {
            let input = Dictionary::new().insert("width", number(row["size"][0].as_f64().unwrap()))
                .insert("height", number(row["size"][1].as_f64().unwrap()));
            ("brep.curve.rectangle", "wire", input)
        };
        let boundary = retained_boolean_result(&registry, constructor, profile_input, &mut caller, &mut born, &mut free);
        let input = Dictionary::new().insert("wire", boundary.get(channel).unwrap().clone());
        let profile = retained_boolean_result(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free);
        let input = polygon_fixture_input(&serde_json::json!({"points": row["path"]}));
        let path = retained_boolean_result(&registry, "brep.curve.polyline", input, &mut caller, &mut born, &mut free);
        let input = |path: &Dictionary| Dictionary::new().insert("profile", profile.get("face").unwrap().clone())
            .insert("path", path.get("wire").unwrap().clone());
        let operator = row["operator"].as_str().unwrap();
        let result = retained_boolean_result(&registry, operator, input(&path), &mut caller, &mut born, &mut free);
        let shape = read_geometry(&result, "solid").unwrap();
        session.with_kernel_read(|kernel| {
            let solid = kernel.geometry_solid(&shape).unwrap();
            let body = kernel.representation();
            let reach = body.reachable_from(&[EntityRef::Solid(solid)]);
            assert_eq!(reach.faces.len(), row["faces"].as_u64().unwrap() as usize);
            for &id in &reach.edges {
                let uses = body.edge_coedges(id);
                assert_eq!(uses.len(), 2);
                let orientation = |id| {
                    let use_ = body.coedges.get(id).unwrap();
                    let face = body.loops.get(use_.loop_id).unwrap().face;
                    use_.forward != body.faces.get(face).unwrap().flipped
                };
                assert_ne!(orientation(uses[0]), orientation(uses[1]));
            }
            Ok(())
        }).unwrap();
        let mesh = run_funded_preview(&session, identity, shape.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        let (min, max) = mesh.aabb();
        for (actual, expected) in [(min, &row["bounds"][0]), (max, &row["bounds"][1])] {
            assert!(actual.into_iter().zip(fixture_xyz(expected)).all(|(actual, expected)| (f64::from(actual) - expected).abs() < 0.04));
        }
        let read = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let volume = retained_boolean_result(&registry, "brep.measure.volume", read, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);

        let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
        let guide = path.get("wire").unwrap().clone();
        for (operator, refused) in [(operator, input(&bent)), ("brep.sweep.pipe", input(&path).insert("guide", guide))] {
            let mut job = Some(admit_live_job(&registry, operator, refused));
            run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                assert!(job.as_mut().unwrap().step(cx).is_err());
                Ok(())
            });
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
        }
        assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    }
    caller.retire(stock());
}

#[test]
fn original_two_profile_loft_proves_coaxial_or_translated_boundary_correspondence() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-two-profile-loft.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97175, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let circle = &fixture["circle"];
    let circle_input = Dictionary::new()
        .insert("center", Value::Dictionary(point_dictionary(fixture_xyz(&circle["center"]))))
        .insert("normal", Value::Dictionary(vector_dictionary(fixture_xyz(&circle["normal"]))))
        .insert("radius", number(circle["radius"].as_f64().unwrap()));
    let circle_wire = retained_boolean_result(&registry, "brep.curve.circle", circle_input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("wire", circle_wire.get("curve").unwrap().clone());
    let circle_first = retained_boolean_result(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("face", circle_first.get("face").unwrap().clone())
        .insert("distance", number(circle["distance"].as_f64().unwrap()));
    let circle_last = retained_boolean_result(&registry, "brep.surf.offset", input, &mut caller, &mut born, &mut free);

    let polygon = &fixture["polygon"];
    let [polygon_first, polygon_last, mismatch] = [&polygon["first"], &polygon["last"], &fixture["mismatched"]].map(|raw| {
        let input = polygon_fixture_input(&serde_json::json!({"points": raw}));
        let wire = retained_boolean_result(&registry, "brep.curve.polyline", input, &mut caller, &mut born, &mut free);
        let input = Dictionary::new().insert("wire", wire.get("wire").unwrap().clone());
        retained_boolean_result(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free)
    });
    let operator = fixture["operator"].as_str().unwrap();
    let loft_input = |first: &Value, last: &Value, smooth| {
        let profiles = Dictionary::with_schema("list").insert("2", first.clone()).insert("10", last.clone());
        Dictionary::new().insert("profiles", Value::Dictionary(profiles)).insert("smooth", number(smooth))
    };

    for (row, first, last) in [
        (circle, circle_first.get("face").unwrap(), circle_last.get("faceOut").unwrap()),
        (polygon, polygon_first.get("face").unwrap(), polygon_last.get("face").unwrap()),
    ] {
        let input = loft_input(first, last, row["smooth"].as_f64().unwrap());
        let result = retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
        let shape = read_geometry(&result, "solid").unwrap();
        session.with_kernel_read(|kernel| {
            let solid = kernel.geometry_solid(&shape).unwrap();
            let body = kernel.representation();
            let reach = body.reachable_from(&[EntityRef::Solid(solid)]);
            assert_eq!(reach.faces.len(), row["faces"].as_u64().unwrap() as usize);
            for &edge in &reach.edges {
                let uses = body.edge_coedges(edge);
                assert_eq!(uses.len(), 2);
                let forward = |id| {
                    let use_ = body.coedges.get(id).unwrap();
                    let face = body.loops.get(use_.loop_id).unwrap().face;
                    use_.forward != body.faces.get(face).unwrap().flipped
                };
                assert_ne!(forward(uses[0]), forward(uses[1]));
            }
            Ok(())
        }).unwrap();
        let mesh = run_funded_preview(&session, identity, shape.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        let (min, max) = mesh.aabb();
        for (actual, expected) in [(min, &row["bounds"][0]), (max, &row["bounds"][1])] {
            assert!(actual.into_iter().zip(fixture_xyz(expected)).all(|(actual, expected)| (f64::from(actual) - expected).abs() < 0.04));
        }
        let input = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let result = retained_boolean_result(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&result, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);
    }

    let stock_before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
    let input = loft_input(polygon_first.get("face").unwrap(), mismatch.get("face").unwrap(), 0.0);
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let mut rejected = Some(admit_live_job(&registry, operator, input));
    let mut fault = false;
    for _ in 0..12 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            fault = rejected.as_mut().unwrap().step(cx).is_err();
            Ok(())
        });
        if fault { break; }
    }
    assert!(fault);
    close_live_job(&mut rejected, &mut caller, &mut born, &mut free);
    let input = loft_input(polygon_first.get("face").unwrap(), polygon_last.get("face").unwrap(), 0.0);
    let mut cancelled = Some(admit_live_job(&registry, operator, input));
    run(&mut caller, grant, &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
        Ok(())
    });
    assert_eq!(cancelled.as_ref().unwrap().progress().phase, "loft-corresponding-edges");
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, stock(), &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), stock_before);
    caller.retire(stock());
}

#[test]
fn original_transforms_reissue_placed_solids_and_preserve_preview_certificates() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, read_xyz, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-placed-solid-transforms.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97176, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let close_to = |actual: [f64; 3], raw: &serde_json::Value, tolerance| {
        assert!(actual.into_iter().zip(fixture_xyz(raw)).all(|(coordinate, expected)| (coordinate - expected).abs() < tolerance), "{actual:?} vs {raw}");
    };
    for row in fixture["cases"].as_array().unwrap() {
        let input = row["inputs"].as_object().unwrap().iter().fold(Dictionary::new(), |input, (key, value)| {
            input.insert(key.as_str(), number(value.as_f64().unwrap()))
        });
        let mut result = retained_boolean_result(&registry, row["constructor"].as_str().unwrap(), input, &mut caller, &mut born, &mut free);
        let initial = read_geometry(&result, "solid").unwrap();
        let mut channel = "solid";
        for step in row["transforms"].as_array().unwrap() {
            let source = Dictionary::new().insert("geometry", result.get(channel).unwrap().clone());
            let input = step["inputs"].as_object().unwrap().iter().fold(source, |input, (key, value)| {
                let payload = if value.is_array() {
                    let axes = fixture_xyz(value);
                    Value::Dictionary(if key == "origin" { point_dictionary(axes) } else { vector_dictionary(axes) })
                } else {
                    number(value.as_f64().unwrap())
                };
                input.insert(key.as_str(), payload)
            });
            result = retained_boolean_result(&registry, step["operator"].as_str().unwrap(), input, &mut caller, &mut born, &mut free);
            channel = "geometryOut";
        }
        let handle = read_geometry(&result, channel).unwrap();
        assert_ne!(handle, initial);
        session.with_kernel_read(|kernel| {
            let source = kernel.live_solid_primitive(initial.as_str()).expect("the source certificate remains live");
            let placed = kernel.live_solid_primitive(handle.as_str()).expect("transforms retain a primitive certificate");
            assert_eq!(placed.ring_sizes().len(), source.ring_sizes().len());
            assert_eq!(placed.edge_count(), source.edge_count());
            assert_eq!(placed.vertex_count(), source.vertex_count());
            close_to(placed.vertex(0).to_array(), &row["firstVertex"], 1e-10);
            let [min, max] = placed.axis_aligned_bounds();
            close_to(min.to_array(), &row["bounds"][0], 1e-10);
            close_to(max.to_array(), &row["bounds"][1], 1e-10);
            Ok(())
        }).unwrap();
        let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        let (min, max) = mesh.aabb();
        close_to(min.map(f64::from), &row["bounds"][0], 0.065);
        close_to(max.map(f64::from), &row["bounds"][1], 0.065);
        let input = || Dictionary::new().insert("geometry", result.get(channel).unwrap().clone());
        let volume = retained_boolean_result(&registry, "brep.measure.volume", input(), &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);
        let center = retained_boolean_result(&registry, "brep.measure.centerOfMass", input(), &mut caller, &mut born, &mut free);
        close_to(read_xyz(&center, "center").unwrap(), &row["center"], 1e-10);
    }
    caller.retire(stock());
}

#[test]
fn original_box_section_publishes_one_oriented_face_in_the_registered_list() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_engine::GeometryHandle;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-section.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97177, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let dimensions = fixture_xyz(&fixture["box"]["size"]);
    let input = ["width", "depth", "height"].into_iter().zip(dimensions).fold(Dictionary::new(), |input, (axis, size)| {
        input.insert(axis, number(size))
    });
    let original = retained_boolean_result(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("geometry", original.get("solid").unwrap().clone())
        .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&fixture["box"]["origin"]))));
    let stock_box = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
    let section_input = |row: &serde_json::Value| Dictionary::new()
        .insert("solid", stock_box.get("geometryOut").unwrap().clone())
        .insert("planeOrigin", Value::Dictionary(point_dictionary(fixture_xyz(&row["origin"]))))
        .insert("planeNormal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
    let operator = fixture["operator"].as_str().unwrap();

    for row in fixture["cases"].as_array().unwrap() {
        let count_live = || session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
        let initial = count_live();
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        let mut job = Some(admit_live_job(&registry, operator, section_input(row)));
        let mut edges_scanned = 0;
        let mut output = None;
        for _ in 0..2000 {
            let clipping = job.as_ref().unwrap().progress().phase == "box-section-edges";
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                if clipping && receipt.copied_bytes == 6 * std::mem::size_of::<f64>() {
                    edges_scanned += 1;
                }
                match step {
                    neural_engine::OperatorJobStep::Working(_) => {}
                    neural_engine::OperatorJobStep::Done(value) => output = Some(value),
                    _ => panic!("section did not produce its original face"),
                }
                Ok(())
            });
            if output.is_some() { break; }
            assert_eq!(count_live(), initial, "the section became live before the complete faces list");
        }
        assert_eq!(edges_scanned, 12);
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert_eq!(count_live(), initial + 1);
        let output = neural_engine::ColdOwner::new(output.expect("original section did not finish"));
        let faces = output.get("faces").and_then(Value::as_dictionary).unwrap();
        assert_eq!(faces.schema(), Some("list"));
        assert_eq!(faces.keys().filter(|key| key.as_str() != "$schema").count(), 1);
        let member = faces.get("0").unwrap();
        let geometry = member.as_dictionary().unwrap();
        assert_eq!(geometry.get("kind").and_then(Value::as_atom).and_then(Atom::as_str), Some("face"));
        let raw_handle = geometry.get("handle").and_then(Value::as_atom).and_then(Atom::as_str).unwrap();
        let handle = GeometryHandle(raw_handle.to_owned());
        let count = row["vertices"].as_u64().unwrap() as usize;
        session.with_kernel_read(|kernel| {
            let label = kernel.label_of(&handle).unwrap();
            let body = kernel.representation();
            let (id, face) = body.faces.iter().find(|(_, face)| face.label == label).unwrap();
            let reach = body.reachable_from(&[EntityRef::Face(id)]);
            assert_eq!((reach.vertices.len(), reach.edges.len()), (count, count));
            assert!(face.inners.is_empty());
            let normal = fixture_xyz(&row["normal"]);
            let origin = fixture_xyz(&row["origin"]);
            let support = body.surfaces.get(face.surface).unwrap().normal(0.0, 0.0).unwrap();
            let alignment = [support.x, support.y, support.z].into_iter().zip(normal).map(|(a, b)| a * b).sum::<f64>();
            assert!(if face.flipped { alignment < 0.0 } else { alignment > 0.0 });
            for &vertex in &reach.vertices {
                let point = body.vertices.get(vertex).unwrap().position;
                let distance = [point.x, point.y, point.z].into_iter().zip(origin).zip(normal)
                    .map(|((x, o), n)| (x - o) * n).sum::<f64>();
                assert!(distance.abs() < 1e-10);
            }
            Ok(())
        }).unwrap();
        let preview = run_funded_preview(&session, identity, raw_handle, &mut caller, &mut born, &mut free);
        preview.validate_component_references().unwrap();
        assert_eq!(preview.component_references["face"].len(), 1);
        let (min, max) = preview.aabb();
        assert_eq!(min.map(f64::from), fixture_xyz(&row["bounds"][0]));
        assert_eq!(max.map(f64::from), fixture_xyz(&row["bounds"][1]));
        let measure = Dictionary::new().insert("geometry", member.clone());
        let area = retained_boolean_result(&registry, "brep.measure.area", measure, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&area, "area").unwrap() - row["area"].as_f64().unwrap()).abs() < 1e-10);
    }

    let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    for row in fixture["refused"].as_array().unwrap() {
        let mut rejected = Some(admit_live_job(&registry, operator, section_input(row)));
        let mut fault = false;
        for _ in 0..24 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                fault = rejected.as_mut().unwrap().step(cx).is_err();
                Ok(())
            });
            if fault { break; }
        }
        assert!(fault);
        close_live_job(&mut rejected, &mut caller, &mut born, &mut free);
    }
    let mut cancelled = Some(admit_live_job(&registry, operator, section_input(&fixture["cases"][1])));
    for _ in 0..24 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
            Ok(())
        });
    }
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, stock(), &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    caller.retire(stock());
}

#[test]
fn original_box_split_commits_two_halves_and_retires_both_on_cancellation() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-split.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97178, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&fixture["box"]["size"]))
        .fold(Dictionary::new(), |input, (axis, size)| input.insert(axis, number(size)));
    let original = retained_boolean_result(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("geometry", original.get("solid").unwrap().clone())
        .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&fixture["box"]["origin"]))));
    let stock_box = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
    let split_input = |row: &serde_json::Value| Dictionary::new()
        .insert("solid", stock_box.get("geometryOut").unwrap().clone())
        .insert("planeOrigin", Value::Dictionary(point_dictionary(fixture_xyz(&row["origin"]))))
        .insert("planeNormal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
    let operator = fixture["operator"].as_str().unwrap();
    let counts = || session.with_kernel_read(|kernel| Ok((kernel.live_handles().len(), kernel.representation().solids.iter().count()))).unwrap();
    let initial = counts();

    let one_item = Grant { maximum_items: 1, ..stock() };
    let mut cancelled = Some(admit_live_job(&registry, operator, split_input(&fixture["cases"][0])));
    let mut staged = 0;
    for _ in 0..2500 {
        let publishing = cancelled.as_ref().unwrap().progress().phase == "box-split-publication";
        run(&mut caller, one_item, &mut born, &mut free, |cx| {
            let (step, receipt) = cancelled.as_mut().unwrap().step(cx).unwrap();
            assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
            if publishing && receipt == (Progress { copied_items: 1, ..Progress::default() }) { staged += 1; }
            Ok(())
        });
        assert_eq!(counts().0, initial.0);
        if staged == 2 { break; }
    }
    assert_eq!(staged, 2);
    assert_eq!(counts().1, initial.1 + 2);
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, stock(), &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(counts(), initial);

    for row in fixture["cases"].as_array().unwrap() {
        let before = counts();
        let mut job = Some(admit_live_job(&registry, operator, split_input(row)));
        let mut output = None;
        for _ in 0..2500 {
            run(&mut caller, one_item, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                match step {
                    neural_engine::OperatorJobStep::Working(_) => {}
                    neural_engine::OperatorJobStep::Done(value) => {
                        assert_eq!(receipt, Progress { copied_items: 1, ..Progress::default() });
                        output = Some(value);
                    }
                    _ => panic!("original box split did not complete"),
                }
                Ok(())
            });
            if output.is_some() { break; }
            assert_eq!(counts().0, before.0);
        }
        assert_eq!(counts(), (before.0 + 2, before.1 + 2));
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let output = neural_engine::ColdOwner::new(output.expect("original Split did not finish"));
        for channel in ["positive", "negative"] {
            let expected = &row[channel];
            let value = output.get(channel).unwrap();
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, channel).unwrap();
            let preview = run_funded_preview(&session, identity, &handle.0, &mut caller, &mut born, &mut free);
            preview.validate_component_references().unwrap();
            assert_eq!(preview.component_references["face"].len(), 6);
            let (min, max) = preview.aabb();
            assert_eq!(min.map(f64::from), fixture_xyz(&expected["bounds"][0]));
            assert_eq!(max.map(f64::from), fixture_xyz(&expected["bounds"][1]));
            let measure = retained_boolean_result(&registry, "brep.measure.volume", Dictionary::new().insert("geometry", value.clone()),
                &mut caller, &mut born, &mut free);
            assert_eq!(read_channel_number(&measure, "volume").unwrap(), expected["volume"].as_f64().unwrap());
        }
    }

    let before = counts();
    for row in fixture["refused"].as_array().unwrap() {
        let mut refused = Some(admit_live_job(&registry, operator, split_input(row)));
        run(&mut caller, stock(), &mut born, &mut free, |cx| {
            assert!(refused.as_mut().unwrap().step(cx).is_err());
            Ok(())
        });
        close_live_job(&mut refused, &mut caller, &mut born, &mut free);
        assert_eq!(counts(), before);
    }
}

#[test]
fn original_compound_cut_publishes_one_exact_boundary_and_rolls_it_back_on_cancellation() {
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_geometry, vector_dictionary};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-compound-cut.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97179), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let make_box = |row: &serde_json::Value, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize| {
        let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&row["size"]))
            .fold(Dictionary::new(), |input, (axis, size)| input.insert(axis, number(size)));
        let box_out = run_live_query(&registry, "brep.prim3d.box", input, caller, born, free);
        let shape = box_out.get("solid").unwrap().clone();
        let origin = fixture_xyz(&row["origin"]);
        if origin == [0.0; 3] { return shape; }
        let input = Dictionary::new().insert("geometry", shape)
            .insert("offset", Value::Dictionary(vector_dictionary(origin)));
        let translated = run_live_query(&registry, "brep.xform.translate", input, caller, born, free);
        translated.get("geometryOut").unwrap().clone()
    };
    let target = make_box(&fixture["target"], &mut caller, &mut born, &mut free);
    let make_input = |row: &serde_json::Value, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize| {
        let tools = row["tools"].as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"), |tools, (index, tool)| {
            tools.insert(index.to_string(), make_box(tool, caller, born, free))
        });
        Dictionary::new().insert("target", target.clone()).insert("tools", Value::Dictionary(tools))
    };
    let operator = fixture["operator"].as_str().unwrap();
    let counts = || session.with_kernel_read(|kernel| Ok((kernel.live_handles().len(), kernel.representation().solids.iter().count()))).unwrap();
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };

    for row in fixture["cases"].as_array().unwrap() {
        let input = make_input(row, &mut caller, &mut born, &mut free);
        let before = counts();
        let mut job = Some(admit_live_job(&registry, operator, input));
        let mut result = None;
        let mut tool_turns = 0;
        for _ in 0..10000 {
            let collecting = job.as_ref().unwrap().progress().phase == "compound-cut-tools";
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                if collecting && receipt.copied_bytes == std::mem::size_of::<[[f64; 3]; 2]>() { tool_turns += 1; }
                match step {
                    neural_engine::OperatorJobStep::Working(_) => {},
                    neural_engine::OperatorJobStep::Done(output) => result = Some(output),
                    _ => panic!("Compound Cut did not complete"),
                }
                Ok(())
            });
            if result.is_some() { break; }
            assert_eq!(counts().0, before.0, "Compound Cut published an intermediate");
        }
        assert_eq!(tool_turns, row["tools"].as_array().unwrap().len() + 1);
        assert_eq!(counts(), (before.0 + 1, before.1 + 1));
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let result = neural_engine::ColdOwner::new(result.unwrap());
        let handle = read_geometry(&result, "solid").unwrap();
        session.with_kernel_read(|kernel| {
            let id = kernel.geometry_solid(&handle).unwrap();
            let body = kernel.representation();
            let reach = body.reachable_from(&[EntityRef::Solid(id)]);
            let shells = 1 + body.solids.get(id).unwrap().inners.len();
            assert_eq!(shells, row["shells"].as_u64().unwrap() as usize);
            let genus = row["genus"].as_i64().unwrap();
            assert_eq!(reach.vertices.len() as i64 - reach.edges.len() as i64 + reach.faces.len() as i64, 2 * (shells as i64 - genus));
            for &edge in &reach.edges { assert_eq!(body.edge_coedges(edge).len(), 2); }
            Ok(())
        }).unwrap();
        let input = Dictionary::new().insert("geometry", result.get("solid").unwrap().clone());
        let volume = run_live_query(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);
    }

    let input = make_input(&fixture["cases"][0], &mut caller, &mut born, &mut free);
    let before = counts();
    let mut cancelled = Some(admit_live_job(&registry, operator, input));
    for _ in 0..10000 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
            Ok(())
        });
        assert_eq!(counts().0, before.0);
        if counts().1 > before.1 { break; }
    }
    assert_eq!(counts(), (before.0, before.1 + 1));
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, grant, &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(counts(), before);

    for row in fixture["refused"].as_array().unwrap() {
        let input = make_input(row, &mut caller, &mut born, &mut free);
        let before = counts();
        let mut refused = Some(admit_live_job(&registry, operator, input));
        let mut rejected = false;
        for _ in 0..10 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                rejected = refused.as_mut().unwrap().step(cx).is_err();
                Ok(())
            });
            if rejected { break; }
        }
        assert!(rejected);
        close_live_job(&mut refused, &mut caller, &mut born, &mut free);
        assert_eq!(counts(), before);
    }
}
