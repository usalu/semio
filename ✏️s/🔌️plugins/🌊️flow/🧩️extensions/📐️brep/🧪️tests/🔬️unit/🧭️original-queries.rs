//! 🧭️ Original BRep queries, topology component aliasing and selected previews.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn analytic_curves_and_plane_return_the_closest_point_through_the_original_registry() {
    let fixture = evaluate_fixture();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97149), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    for (name, constructor, geometry_channel) in [
        ("closest parameter on a line", "brep.curve.line", "curve"),
        ("closest parameter on a circle", "brep.curve.circle", "curve"),
        ("closest parameters on a plane", "brep.surf.plane", "surface"),
    ] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let inputs = &case["inputs"];
        let mut parameters = Dictionary::new();
        for (key, value) in inputs[geometry_channel].as_object().unwrap() {
            if key == "recipe" { continue; }
            let value = if value.is_array() {
                Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(value)))
            } else {
                number(value.as_f64().unwrap())
            };
            parameters = parameters.insert(key.as_str(), value);
        }
        let geometry = run_live_query(&registry, constructor, parameters, &mut caller, &mut born, &mut free);
        let input = Dictionary::new()
            .insert("geometry", geometry.get(geometry_channel).unwrap().clone())
            .insert("point", Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(&inputs["point"]))));
        let output = run_live_query(&registry, "brep.measure.closestPoint", input, &mut caller, &mut born, &mut free);
        let actual = semio_s_spatial_kernel_semio_session::read_xyz(&output, "pointOut").unwrap();
        assert_eq!(actual, fixture_xyz(&case["outputs"]["point"]), "{name}");
    }
    caller.retire(stock());
}

#[test]
fn analytic_plane_surface_queries_use_the_original_registry() {
    let fixture = evaluate_fixture();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97148), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);

    for (name, operator, channel) in [
        ("plane point at the origin of its frame", "brep.eval.surfPoint", "point"),
        ("plane normal", "brep.eval.surfNormal", "normal"),
    ] {
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"].as_str() == Some(name)).unwrap();
        let plane = &case["inputs"]["surface"];
        let input = Dictionary::new()
            .insert("origin", Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(&plane["origin"]))))
            .insert("normal", Value::Dictionary(semio_s_spatial_kernel_semio_session::vector_dictionary(fixture_xyz(&plane["normal"]))));
        let surface = run_live_query(&registry, "brep.surf.plane", input, &mut caller, &mut born, &mut free);
        let preview = &fixture["planePreview"];
        if preview["source"].as_str() == Some(name) {
            use flow_extension_sdk::geometry::{GeometryStep, GeometryWorkIdentity};
            let shape = semio_s_spatial_kernel_semio_session::read_geometry(&surface, "surface").unwrap();
            let identity = GeometryWorkIdentity { operation: 97148, generation: 17 };
            let tolerance = 0.025;
            let mut mesh = None;
            for _ in 0..1000 {
                let demand = session.tessellation_demands(identity, shape.as_str(), tolerance, 128).unwrap();
                let grant = Grant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(128), maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = session.tessellate_step(identity, shape.as_str(), tolerance, cx).unwrap();
                    assert!(receipt.retained.fits(grant));
                    match step {
                        GeometryStep::Ready(geometry) => mesh = Some(geometry),
                        GeometryStep::Working { .. } => {}
                        _ => panic!("plane preview stopped before producing a mesh"),
                    }
                    Ok(())
                });
                if mesh.is_some() { break; }
            }
            let mesh = mesh.expect("retained Plane has a visible preview");
            assert_eq!(mesh.indices.len() / 3, preview["triangles"].as_u64().unwrap() as usize);
            assert_eq!(mesh.edge_positions.len() / 6, preview["edges"].as_u64().unwrap() as usize);
            let corner = |reduce: fn(f32, f32) -> f32| std::array::from_fn(|axis| mesh.positions[axis..].iter().step_by(3).copied().reduce(reduce).unwrap() as f64);
            assert_eq!(corner(f32::min), fixture_xyz(&preview["bbox"][0]));
            assert_eq!(corner(f32::max), fixture_xyz(&preview["bbox"][1]));
            assert!(mesh.normals.chunks_exact(3).all(|normal| normal == [0.0, 0.0, 1.0]));
        }
        let input = Dictionary::new()
            .insert("surface", surface.get("surface").unwrap().clone())
            .insert("u", number(case["inputs"]["u"].as_f64().unwrap()))
            .insert("v", number(case["inputs"]["v"].as_f64().unwrap()));
        let result = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
        assert_eq!(semio_s_spatial_kernel_semio_session::read_xyz(&result, channel).unwrap(), fixture_xyz(&case["outputs"][channel]));
    }
    caller.retire(stock());
}

#[test]
fn original_volume_integrates_a_live_polygonal_solid_one_coedge_at_a_time() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solids.json")).unwrap();
    let case = &fixture["planarVolume"];
    let points: Vec<[f64; 3]> = case["points"].as_array().unwrap().iter()
        .map(|point| std::array::from_fn(|axis| point[axis].as_f64().unwrap())).collect();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let input = session.with_kernel(|kernel| {
        let handle = kernel.convex_hull_sync(&points).map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))?;
        let payload = semio_s_spatial_kernel_semio_session::geometry_dict(kernel, &handle)?;
        Ok(neural_engine::channel_output("geometry", payload))
    }).unwrap();
    let mut job = Some(admit_live_job(&registry, "brep.measure.volume", input.clone()));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97160), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let mut output = None;
    let mut integration_turns = 0;
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    for _ in 0..2000 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            match job.as_mut().unwrap().step(cx).unwrap().0 {
                neural_engine::OperatorJobStep::Done(result) => output = Some(result),
                neural_engine::OperatorJobStep::Working(progress) if progress.phase == "geometry-volume-integrating" => integration_turns += 1,
                _ => {}
            }
            Ok(())
        });
        if output.is_some() { break; }
    }
    assert!(integration_turns >= 12);
    let output = neural_engine::ColdOwner::new(output.expect("planar solid volume completes under one-item turns"));
    let measured = semio_s_spatial_kernel_semio_session::read_channel_number(&output, "volume").unwrap();
    assert!((measured - case["volume"].as_f64().unwrap()).abs() < 1e-12);
    let area = run_live_query(&registry,"brep.measure.area",input,&mut caller,&mut born,&mut free);
    let measured = semio_s_spatial_kernel_semio_session::read_channel_number(&area,"area").unwrap();
    assert!((measured-case["area"].as_f64().unwrap()).abs()<1e-12);
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    caller.retire(stock());
}

#[test]
fn original_deconstructed_cylinder_side_previews_its_native_face() {
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, read_geometry};
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 97220, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let primitive = run_live_query(&registry, "brep.prim3d.cylinder", Dictionary::new()
        .insert("radius", number(1.2)).insert("height", number(3.25)), &mut caller, &mut born, &mut free);
    let solid = read_geometry(&primitive, "solid").unwrap();
    let (side, label) = session.with_kernel(|kernel| {
        let topology = kernel.deconstruct_sync(&solid).map_err(|error| map_kernel_error(&error))?;
        let body = kernel.representation();
        Ok(topology.faces.into_iter().find_map(|handle| {
            let label = kernel.label_of(&handle)?;
            let (_, face) = body.faces.iter().find(|(_, face)| face.label == label)?;
            matches!(body.surfaces.get(face.surface), Some(Surface::Cylinder { .. })).then_some((handle, label.0.to_string()))
        }).unwrap())
    }).unwrap();
    let mesh = run_funded_preview(&session, identity, side.as_str(), &mut caller, &mut born, &mut free);
    mesh.validate_component_references().unwrap();
    assert_eq!(mesh.component_references["face"], [label]);
    assert_eq!(mesh.component_references["edge"].len(), 3);
    assert_eq!(mesh.component_references["vertex"].len(), 2);
    assert!((preview_area_volume(&mesh).0 - 2.0 * std::f64::consts::PI * 1.2 * 3.25).abs() < 0.025);
    caller.retire(stock());
}

#[test]
fn original_bounding_box_uses_exact_analytic_world_axes() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-bounding-box.json")).unwrap();
    let revolve: serde_json::Value = serde_json::from_str(include_str!("🔣️original-revolve.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97170, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let point = |axes| Value::Dictionary(point_dictionary(axes));
    let vector = |axes| Value::Dictionary(vector_dictionary(axes));
    let circle_input = |row: &serde_json::Value| Dictionary::new().insert("center", point(fixture_xyz(&row["center"])))
        .insert("normal", vector(fixture_xyz(&row["normal"]))).insert("radius", number(row["radius"].as_f64().unwrap()));
    let cylinder = &fixture["cylinder"];
    let profile = retained_boolean_result(&registry, "brep.curve.circle", circle_input(cylinder), &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("wire", profile.get("curve").unwrap().clone())
        .insert("vector", vector(fixture_xyz(&cylinder["vector"])));
    let swept = retained_boolean_result(&registry, "brep.solid.extrude", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("geometry", profile.get("curve").unwrap().clone())
        .insert("offset", vector(fixture_xyz(&fixture["curve"]["offset"])));
    let curve = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);

    let profile = retained_boolean_result(&registry, "brep.curve.circle", circle_input(&revolve["profile"]), &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("wire", profile.get("curve").unwrap().clone());
    let face = retained_boolean_result(&registry, "brep.util.faceFromWire", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("face", face.get("face").unwrap().clone())
        .insert("axisOrigin", point(fixture_xyz(&revolve["axis"]["origin"])))
        .insert("axisDirection", vector(fixture_xyz(&revolve["axis"]["direction"])))
        .insert("angle", number(revolve["angle"].as_f64().unwrap()));
    let torus = retained_boolean_result(&registry, revolve["operator"].as_str().unwrap(), input, &mut caller, &mut born, &mut free);

    let operator = fixture["operator"].as_str().unwrap();
    for (geometry, minimum, maximum) in [
        (swept.get("solid").unwrap(), &cylinder["bounds"][0], &cylinder["bounds"][1]),
        (curve.get("geometryOut").unwrap(), &fixture["curve"]["bounds"][0], &fixture["curve"]["bounds"][1]),
        (torus.get("solid").unwrap(), &revolve["expected"]["minimum"], &revolve["expected"]["maximum"]),
    ] {
        let input = Dictionary::new().insert("geometry", geometry.clone());
        let result = retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
        let handle = read_geometry(&result, "box").unwrap();
        let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        mesh.validate_component_references().unwrap();
        let bounds = mesh.aabb();
        for (approx, raw) in [(bounds.0, minimum), (bounds.1, maximum)] {
            assert!(approx.into_iter().zip(fixture_xyz(raw)).all(|(actual, expected)| (f64::from(actual) - expected).abs() < 1e-6));
        }
        for (domain, count) in [("face", 6), ("edge", 12), ("vertex", 8)] {
            assert_eq!(mesh.component_references[domain].len(), count);
        }
    }

    let input = Dictionary::new().insert("center", point([0.; 3])).insert("normal", vector([0., 0., 1.])).insert("radius", number(2.0));
    let circle = retained_boolean_result(&registry, "brep.curve.circle", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("geometry", circle.get("curve").unwrap().clone());
    let before = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
    let mut rejected = Some(admit_live_job(&registry, operator, input));
    run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
        assert!(rejected.as_mut().unwrap().step(cx).is_err());
        Ok(())
    });
    close_live_job(&mut rejected, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), before);
    caller.retire(stock());
}

#[test]
fn retained_analytic_projection_publishes_every_result_channel() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_xyz};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-analytic-projection.json")).unwrap();
    let axes=|row:&serde_json::Value|serde_json::from_value::<[f64;3]>(row.clone()).unwrap();
    let coords=|row:&serde_json::Value|Value::Dictionary(point_dictionary(axes(row)));
    let near=|value:f64,expected:f64|assert!((value-expected).abs()<1e-10,"{value} differs from {expected}");
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97283),Generation(17),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    for name in ["line","ellipse"] {
        let row=&fixture[name];
        let input=if name=="line"{
            Dictionary::new().insert("start",coords(&row["start"])).insert("end",coords(&row["end"]))
        }else{
            Dictionary::new().insert("center",Value::Dictionary(point_dictionary([0.0;3])))
                .insert("normal",Value::Dictionary(point_dictionary([0.0,0.0,1.0])))
                .insert("semiMajor",number(row["radii"][0].as_f64().unwrap()))
                .insert("semiMinor",number(row["radii"][1].as_f64().unwrap()))
        };
        let operation=if name=="line"{"brep.curve.line"}else{"brep.curve.ellipse"};
        let curve=run_live_query(&registry,operation,input,&mut caller,&mut born,&mut free);
        let input=Dictionary::new().insert("curve",curve.get("curve").unwrap().clone()).insert("point",coords(&row["target"]));
        let result=run_live_query(&registry,"brep.eval.curveClosestParameter",input,&mut caller,&mut born,&mut free);
        near(read_channel_number(&result,"parameter").unwrap(),row["parameter"].as_f64().unwrap());
        near(read_channel_number(&result,"distance").unwrap(),row["distance"].as_f64().unwrap());
        for (actual,expected) in read_xyz(&result,"pointOut").unwrap().into_iter().zip(axes(&row["point"])){near(actual,expected);}
    }
    let plane=&fixture["plane"];
    let input=Dictionary::new().insert("origin",coords(&plane["origin"])).insert("normal",coords(&plane["normal"]));
    let surface=run_live_query(&registry,"brep.surf.plane",input,&mut caller,&mut born,&mut free);
    let input=Dictionary::new().insert("surface",surface.get("surface").unwrap().clone()).insert("point",coords(&plane["target"]));
    let result=run_live_query(&registry,"brep.eval.surfaceClosestUv",input,&mut caller,&mut born,&mut free);
    near(read_channel_number(&result,"u").unwrap(),plane["uv"][0].as_f64().unwrap());
    near(read_channel_number(&result,"v").unwrap(),plane["uv"][1].as_f64().unwrap());
    near(read_channel_number(&result,"distance").unwrap(),plane["distance"].as_f64().unwrap());
    for (actual,expected) in read_xyz(&result,"pointOut").unwrap().into_iter().zip(axes(&plane["point"])){near(actual,expected);}
    caller.retire_exact();
}

#[test]
fn retained_vertex_publishes_and_previews_with_rollback_before_publication() {
    use flow_extension_sdk::geometry::{GeometryStep, GeometryWorkIdentity};
    use neural_engine::OperatorJobStep;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-vertex.json")).unwrap();
    let operator = fixture["operator"].as_str().unwrap();
    let channel = fixture["output"].as_str().unwrap();
    let tolerance = fixture["previewDeflection"].as_f64().unwrap();
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        for cancelled in [false, true] {
            let session = Session::new();
            let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
            let point = fixture_xyz(&row["coordinates"]);
            let input = Dictionary::new().insert(fixture["input"].as_str().unwrap(),
                Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(point)));
            let mut job = Some(admit_live_job(&registry, operator, input));
            let operation = 98220 + (index * 2 + usize::from(cancelled)) as u64;
            let mut caller = OriginalJobFixtureCaller::admit(OperationId(operation), Generation(17), stock(), || Some(1000), 1500, 32768);
            let (mut born, mut free) = (0, 0);
            let mut output = None;
            let mut stopped = false;
            let mut staged = false;
            for _ in 0..1000 {
                let owner = job.as_ref().unwrap();
                let copy = owner.next_step_copy_byte_demand().unwrap();
                let grant = Grant {
                    maximum_items: 1,
                    maximum_copy_bytes: copy,
                    maximum_capacity_bytes: owner.next_step_capacity_byte_demand(copy).unwrap(),
                    maximum_release_bytes: owner.next_step_release_byte_demand().unwrap(),
                    maximum_depth: owner.next_step_depth_demand().unwrap(),
                };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    match job.as_mut().unwrap().step(cx).unwrap().0 {
                        OperatorJobStep::Done(value) => output = Some(value),
                        OperatorJobStep::Cancelled(_) => stopped = true,
                        OperatorJobStep::Working(_) => {}
                    }
                    Ok(())
                });
                if stopped || output.is_some() { break; }
                let count = session.with_kernel_read(|kernel| {
                    assert!(kernel.live_handles().is_empty());
                    Ok(kernel.representation().vertices.len())
                }).unwrap();
                if count == 1 {
                    staged = true;
                    if cancelled { caller.cancel().cancel_now(); }
                }
            }
            assert!(staged, "expected one funded native vertex before publication");
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            if cancelled {
                assert!(stopped && output.is_none());
                session.with_kernel_read(|kernel| {
                    assert!(kernel.live_handles().is_empty());
                    assert!(kernel.representation().vertices.is_empty());
                    Ok(())
                }).unwrap();
                caller.retire(stock());
                continue;
            }
            let value = neural_engine::ColdOwner::new(output.expect("retained Vertex publishes its geometry"));
            let kind = value.get(channel).and_then(Value::as_dictionary).and_then(|geometry| geometry.get("kind"));
            let Some(Value::Atom(Atom::String(kind))) = kind else { panic!("vertex output must declare its geometry kind"); };
            assert_eq!(kind.as_str(), fixture["kind"].as_str().unwrap());
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&value, channel).unwrap();
            let label = session.with_kernel_read(|kernel| {
                assert_eq!(kernel.kind_sync(&handle).unwrap(), semio_framework_3d_brep_engine::GeometryKind::Vertex);
                assert_eq!(kernel.live_handles().len(), 1);
                assert_eq!(kernel.representation().vertices.len(), 1);
                Ok(kernel.label_of(&handle).unwrap().0.to_string())
            }).unwrap();
            let identity = GeometryWorkIdentity { operation, generation: 17 };
            let mut preview = None;
            for _ in 0..1000 {
                let demand = session.tessellation_demands(identity, handle.as_str(), tolerance, 128).unwrap();
                let grant = Grant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes,
                    maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    match session.tessellate_step(identity, handle.as_str(), tolerance, cx).unwrap().0 {
                        GeometryStep::Ready(mesh) => preview = Some(mesh),
                        GeometryStep::Working { .. } => {}
                        _ => panic!("retained vertex preview refused"),
                    }
                    Ok(())
                });
                if preview.is_some() { break; }
            }
            let mesh = preview.expect("published vertex produces a point preview");
            let expected = &row["preview"];
            assert_eq!(mesh.positions, fixture_xyz(&expected["positions"]).map(|axis| axis as f32));
            assert_eq!(mesh.indices.len(), expected["indices"].as_array().unwrap().len());
            assert_eq!(mesh.edge_positions.len(), expected["edgePositions"].as_array().unwrap().len());
            assert_eq!(mesh.vertex_ids, [expected["vertexIds"][0].as_u64().unwrap() as u32]);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.component_references["vertex"], [label]);
            caller.retire(stock());
        }
    }
}

#[test]
fn original_solid_distance_measures_supported_solid_pairs() {
    use semio_s_spatial_kernel_semio_session::{read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-sphere-distance.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(98670), Generation(17), stock(), || Some(1000), 1500, 131072);
    let (mut born, mut free) = (0, 0);
    let mut evaluate = |operator: &str, input| run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
    let vector = |axes| Value::Dictionary(vector_dictionary(axes));
    let place = |evaluate: &mut dyn FnMut(&str, Dictionary) -> neural_engine::ColdOwner<Dictionary>, geometry, offset| {
        let input = Dictionary::new().insert("geometry", geometry).insert("offset", vector(offset));
        evaluate("brep.xform.translate", input).get("geometryOut").unwrap().clone()
    };
    let make_sphere = |evaluate: &mut dyn FnMut(&str, Dictionary) -> neural_engine::ColdOwner<Dictionary>, spec: &serde_json::Value| {
        let input = Dictionary::new().insert("radius", number(spec["radius"].as_f64().unwrap()));
        let geometry = evaluate("brep.prim3d.sphere", input).get("solid").unwrap().clone();
        place(evaluate, geometry, fixture_xyz(&spec["center"]))
    };
    let check = |evaluate: &mut dyn FnMut(&str, Dictionary) -> neural_engine::ColdOwner<Dictionary>, a: Value, b: Value, spec: &serde_json::Value| {
        let expected = spec["expected"].as_f64().unwrap();
        for (left, right) in [(a.clone(), b.clone()), (b, a)] {
            let input = Dictionary::new().insert("a", left).insert("b", right);
            let output = evaluate(fixture["operator"].as_str().unwrap(), input);
            let actual = read_channel_number(&output, "distance").unwrap();
            assert!((actual - expected).abs() < 1e-12, "{}: {actual} != {expected}", spec["name"]);
        }
    };
    for pair in fixture["spherePairs"].as_array().unwrap() {
        let a = make_sphere(&mut evaluate, &pair["left"]);
        let b = make_sphere(&mut evaluate, &pair["right"]);
        check(&mut evaluate, a, b, pair);
    }
    let make_box = |evaluate: &mut dyn FnMut(&str, Dictionary) -> neural_engine::ColdOwner<Dictionary>, spec: &serde_json::Value| {
        let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&spec["size"]))
            .fold(Dictionary::new(), |input, (axis, extent)| input.insert(axis, number(extent)));
        let mut box_solid = evaluate("brep.prim3d.box", input).get("solid").unwrap().clone();
        let angle = spec["rotation"].as_f64().unwrap();
        if angle != 0.0 {
            let input = Dictionary::new().insert("geometry", box_solid)
                .insert("axis", vector([0.0, 0.0, 1.0])).insert("angle", number(angle));
            box_solid = evaluate("brep.xform.rotate", input).get("geometryOut").unwrap().clone();
        }
        place(evaluate, box_solid, fixture_xyz(&spec["origin"]))
    };
    for spec in fixture["sphereBoxes"].as_array().unwrap() {
        let box_solid = make_box(&mut evaluate, spec);
        let sphere = make_sphere(&mut evaluate, &spec["sphere"]);
        check(&mut evaluate, box_solid, sphere, spec);
    }
    for spec in fixture["curvedSolids"].as_array().unwrap() {
        let parameters = spec["parameters"].as_object().unwrap();
        let input = parameters.iter().fold(Dictionary::new(), |input, (name, value)| {
            input.insert(name.as_str(), number(value.as_f64().unwrap()))
        });
        let constructor = format!("brep.prim3d.{}", spec["kind"].as_str().unwrap());
        let geometry = evaluate(&constructor, input).get("solid").unwrap().clone();
        let solid = place(&mut evaluate, geometry, fixture_xyz(&spec["origin"]));
        let sphere = make_sphere(&mut evaluate, &spec["sphere"]);
        check(&mut evaluate, solid, sphere, spec);
    }
    let cylinders: serde_json::Value = serde_json::from_str(include_str!("🔣️original-cylinder-distance.json")).unwrap();
    let cylinder = |evaluate: &mut dyn FnMut(&str, Dictionary) -> neural_engine::ColdOwner<Dictionary>, spec: &serde_json::Value| {
        let input = Dictionary::new().insert("radius", number(spec["radius"].as_f64().unwrap()))
            .insert("height", number(spec["height"].as_f64().unwrap()));
        let mut geometry = evaluate("brep.prim3d.cylinder", input).get("solid").unwrap().clone();
        if spec["reverse"] == true {
            let point = |xyz| Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(xyz));
            let reflection = Dictionary::new().insert("geometry", geometry).insert("origin", point([0.; 3]))
                .insert("normal", point([0., 0., 1.]));
            geometry = evaluate("brep.xform.mirror", reflection).get("geometryOut").unwrap().clone();
        }
        place(evaluate, geometry, fixture_xyz(&spec["center"]))
    };
    let first = cylinder(&mut evaluate, &cylinders["stock"]);
    for spec in cylinders["cases"].as_array().unwrap() {
        let second = cylinder(&mut evaluate, spec);
        check(&mut evaluate, first.clone(), second, spec);
    }
    for spec in cylinders["boxes"].as_array().unwrap() {
        let box_solid = make_box(&mut evaluate, &spec["box"]);
        let tube = cylinder(&mut evaluate, &spec["cylinder"]);
        check(&mut evaluate, box_solid, tube, spec);
    }
    let cavities: serde_json::Value = serde_json::from_str(include_str!("🔣️original-sphere-cavity-distance.json")).unwrap();
    for shell in cavities["shells"].as_array().unwrap() {
        let stock = if shell["kind"] == "sphere" {
            let input = Dictionary::new().insert("radius", number(shell["radius"].as_f64().unwrap()));
            evaluate("brep.prim3d.sphere", input).get("solid").unwrap().clone()
        } else {
            let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&shell["size"]))
                .fold(Dictionary::new(), |input, (axis, extent)| input.insert(axis, number(extent)));
            evaluate("brep.prim3d.box", input).get("solid").unwrap().clone()
        };
        let stock = place(&mut evaluate, stock, fixture_xyz(&shell["origin"]));
        let cavity = make_sphere(&mut evaluate, &shell["void"]);
        let input = Dictionary::new().insert("a", stock).insert("b", cavity);
        let material = evaluate("brep.bool.cut", input).get("solid").unwrap().clone();
        for spec in shell["cases"].as_array().unwrap() {
            let probe = make_sphere(&mut evaluate, spec);
            check(&mut evaluate, material.clone(), probe, spec);
        }
    }
    caller.retire(stock());
}

#[test]
fn original_certified_solid_queries_cover_contact_and_exact_box_distance() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solid-queries.json")).unwrap();
    let bounds: serde_json::Value = serde_json::from_str(include_str!("🔣️original-bounding-box.json")).unwrap();
    let booleans: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-booleans.json")).unwrap();
    let revolve: serde_json::Value = serde_json::from_str(include_str!("🔣️original-revolve.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97171), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let mut evaluate = |operator: &str, input| retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
    let point = |axes| Value::Dictionary(point_dictionary(axes));
    let vector = |axes| Value::Dictionary(vector_dictionary(axes));
    let translate = |geometry, axes| Dictionary::new().insert("geometry", geometry).insert("offset", vector(axes));
    let box_input = |row: &serde_json::Value| ["width", "depth", "height"].into_iter().zip(fixture_xyz(&row["size"]))
        .fold(Dictionary::new(), |input, (axis, extent)| input.insert(axis, number(extent)));
    let circle_input = |row: &serde_json::Value| Dictionary::new().insert("center", point(fixture_xyz(&row["center"])))
        .insert("normal", vector(fixture_xyz(&row["normal"]))).insert("radius", number(row["radius"].as_f64().unwrap()));

    let source = evaluate("brep.prim3d.box", box_input(&fixture["box"]));
    let input = translate(source.get("solid").unwrap().clone(), fixture_xyz(&fixture["box"]["origin"]));
    let placed_box = evaluate("brep.xform.translate", input);
    let sphere = evaluate("brep.prim3d.sphere", Dictionary::new().insert("radius", number(fixture["primitives"]["sphere"]["radius"].as_f64().unwrap())));
    let cone = evaluate("brep.prim3d.cone", Dictionary::new().insert("radius", number(fixture["primitives"]["cone"]["radius"].as_f64().unwrap()))
        .insert("height", number(fixture["primitives"]["cone"]["height"].as_f64().unwrap())));

    let profile = evaluate("brep.curve.circle", circle_input(&bounds["cylinder"]));
    let input = Dictionary::new().insert("wire", profile.get("curve").unwrap().clone()).insert("vector", vector(fixture_xyz(&bounds["cylinder"]["vector"])));
    let cylinder = evaluate("brep.solid.extrude", input);
    let profile = evaluate("brep.curve.circle", circle_input(&revolve["profile"]));
    let face = evaluate("brep.util.faceFromWire", Dictionary::new().insert("wire", profile.get("curve").unwrap().clone()));
    let input = Dictionary::new().insert("face", face.get("face").unwrap().clone())
        .insert("axisOrigin", point(fixture_xyz(&revolve["axis"]["origin"])))
        .insert("axisDirection", vector(fixture_xyz(&revolve["axis"]["direction"])))
        .insert("angle", number(revolve["angle"].as_f64().unwrap()));
    let torus = evaluate(revolve["operator"].as_str().unwrap(), input);

    let bore_stock = &booleans["bore"]["stock"];
    let tool = &booleans["bore"]["tool"];
    let outer = evaluate("brep.prim3d.box", box_input(bore_stock));
    let input = Dictionary::new().insert("radius", number(tool["radius"].as_f64().unwrap()))
        .insert("height", number(tool["length"].as_f64().unwrap()));
    let drill = evaluate("brep.prim3d.cylinder", input);
    let outer = evaluate("brep.xform.translate", translate(outer.get("solid").unwrap().clone(), fixture_xyz(&bore_stock["origin"])));
    let drill = evaluate("brep.xform.translate", translate(drill.get("solid").unwrap().clone(), fixture_xyz(&tool["origin"])));
    let input = Dictionary::new().insert("a", outer.get("geometryOut").unwrap().clone()).insert("b", drill.get("geometryOut").unwrap().clone());
    let bore = evaluate("brep.bool.cut", input);
    let source = placed_box.get("geometryOut").unwrap();

    for (name, solid) in [
        ("box", source), ("sphere", sphere.get("solid").unwrap()), ("cone", cone.get("solid").unwrap()),
        ("cylinder", cylinder.get("solid").unwrap()), ("torus", torus.get("solid").unwrap()), ("bore", bore.get("solid").unwrap()),
    ] {
        for row in fixture["cases"][name].as_array().unwrap() {
            let input = Dictionary::new().insert("solid", solid.clone()).insert("point", point(fixture_xyz(&row[0])));
            let result = evaluate(fixture["classify"].as_str().unwrap(), input);
            assert_eq!(read_channel_number(&result, "classification").unwrap(), row[1].as_f64().unwrap(), "{name} {}", row[0]);
        }
    }

    for row in fixture["boxDistance"].as_array().unwrap() {
        let canonical = evaluate("brep.prim3d.box", box_input(row));
        let input = translate(canonical.get("solid").unwrap().clone(), fixture_xyz(&row["origin"]));
        let target = evaluate("brep.xform.translate", input);
        let input = Dictionary::new().insert("a", source.clone()).insert("b", target.get("geometryOut").unwrap().clone());
        let result = evaluate(fixture["distance"].as_str().unwrap(), input);
        assert_eq!(read_channel_number(&result, "distance").unwrap(), row["expected"].as_f64().unwrap());
    }
    caller.retire(stock());
}

#[test]
fn original_classify_reads_retained_orthogonal_boolean_material() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solid-queries.json")).unwrap();
    let booleans: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-booleans.json")).unwrap();
    for (index, row) in fixture["orthogonalClassify"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(97580 + index as u64), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let mut evaluate = |operator: &str, input| retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
        let mut place_box = |row: &serde_json::Value| {
            let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&row["size"]))
                .fold(Dictionary::new(), |input, (axis, extent)| input.insert(axis, number(extent)));
            let solid = evaluate("brep.prim3d.box", input);
            let input = Dictionary::new().insert("geometry", solid.get("solid").unwrap().clone())
                .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
            evaluate("brep.xform.translate", input).get("geometryOut").unwrap().clone()
        };
        let source = &booleans[row["source"].as_str().unwrap()];
        let [a, b] = ["a", "b"].map(|key| place_box(&source[key]));
        let clip = row.get("clip").map(&mut place_box);
        let input = Dictionary::new().insert("a", a).insert("b", b);
        let result = evaluate(row["operator"].as_str().unwrap(), input);
        let mut solid = result.get("solid").unwrap().clone();
        if let Some(clip) = clip {
            let input = Dictionary::new().insert("a", solid).insert("b", clip);
            solid = evaluate("brep.bool.intersect", input).get("solid").unwrap().clone();
        }
        for probe in row["points"].as_array().unwrap() {
            let input = Dictionary::new().insert("solid", solid.clone())
                .insert("point", Value::Dictionary(point_dictionary(fixture_xyz(&probe[0]))));
            let result = evaluate(fixture["classify"].as_str().unwrap(), input);
            assert_eq!(read_channel_number(&result, "classification").unwrap(), probe[1].as_f64().unwrap(), "{} {}", row["name"], probe[0]);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_classify_uses_spherical_trims_and_inward_cavity_boundaries() {
    check_original_curved_classification("sphericalClassify", 97610);
}

#[test]
fn original_classify_uses_cylindrical_profiles_and_trimmed_axial_caps() {
    check_original_curved_classification("cylindricalClassify", 97620);
}

#[test]
fn original_classify_reaches_finite_revolutions_and_eccentric_axial_steps() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-revolved-classification.json")).unwrap();
    for (index,row) in fixture["cases"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(99230+index as u64),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let mut evaluate=|op,input|run_live_query(&registry,op,input,&mut caller,&mut born,&mut free);
        let [a,b]=std::array::from_fn(|i|row["radii"][i].as_f64().unwrap());
        let point=|p|Value::Dictionary(point_dictionary(p));
        let vector=|p|Value::Dictionary(vector_dictionary(p));
        let height=||row["height"].as_f64().unwrap();
        let kind=row["kind"].as_str().unwrap();
        let geometry=match kind{
            "frustum"=>{
                let crown=height()*b/(b-a);
                let cone=evaluate("brep.prim3d.cone",Dictionary::new().insert("radius",number(b)).insert("height",number(crown)));
                let halves=evaluate("brep.intersect.split",Dictionary::new().insert("solid",cone.get("solid").unwrap().clone())
                    .insert("planeOrigin",point([0.,0.,height()])).insert("planeNormal",vector([0.,0.,1.])));
                halves.get("negative").unwrap().clone()
            }
            "torus"|"annular-sector"=>{
                let profile=if kind=="torus"{
                    let circle=evaluate("brep.curve.circle",Dictionary::new().insert("center",point([a,0.,0.]))
                        .insert("normal",vector([0.,-1.,0.])).insert("radius",number(b)));
                    evaluate("brep.util.faceFromWire",Dictionary::new().insert("wire",circle.get("curve").unwrap().clone()))
                }else{
                    let points=[[a,0.,0.],[b,0.,0.],[b,0.,height()],[a,0.,height()]];
                    evaluate("brep.surf.planarFace",polygon_fixture_input(&serde_json::json!({"points":points})))
                };
                let result=evaluate("brep.sweep.revolve",Dictionary::new().insert("face",profile.get("face").unwrap().clone())
                    .insert("axisOrigin",point([0.;3])).insert("axisDirection",vector([0.,0.,1.]))
                    .insert("angle",number(row["rotationDegrees"].as_f64().unwrap().to_radians())));
                result.get("solid").unwrap().clone()
            }
            "bore"|"step"=>{
                let mut cylinder=|radius,extent,offset:[f64;3]|{
                    let solid=evaluate("brep.prim3d.cylinder",Dictionary::new().insert("radius",number(radius)).insert("height",number(extent)));
                    let solid=solid.get("solid").unwrap().clone();
                    if offset==[0.;3]{return solid;}
                    let result=evaluate("brep.xform.translate",Dictionary::new().insert("geometry",solid).insert("offset",vector(offset)));
                    result.get("geometryOut").unwrap().clone()
                };
                let [lo,hi]=std::array::from_fn(|i|row["span"][i].as_f64().unwrap());
                let radial=row["offset"].as_f64().unwrap();
                let (op,outer,inner)=if kind=="bore"{
                    ("brep.bool.cut",cylinder(b,height(),[0.;3]),cylinder(a,hi-lo,[radial,0.,lo]))
                }else{
                    ("brep.bool.fuse",cylinder(b,hi-lo,[0.,0.,lo]),cylinder(a,height(),[radial,0.,0.]))
                };
                evaluate(op,Dictionary::new().insert("a",outer).insert("b",inner)).get("solid").unwrap().clone()
            }
            _=>panic!("unknown retained revolution fixture"),
        };
        let handles=||session.with_kernel_read(|kernel|Ok(kernel.live_handles())).unwrap();
        let initial=handles();
        for probe in row["probes"].as_array().unwrap(){
            let [x,y,z]=fixture_xyz(&probe[0]);
            let world=if kind=="frustum"{[x,-y,height()-z]}else{[x,y,z]};
            let distance=probe[1].as_f64().unwrap();
            let expected=if distance.abs()<=1e-6{2.}else if distance<0.{0.}else{1.};
            let result=evaluate(fixture["classify"].as_str().unwrap(),Dictionary::new()
                .insert("solid",geometry.clone()).insert("point",point(world)));
            assert_eq!(read_channel_number(&result,"classification").unwrap(),expected,"{row:?} {world:?}");
        }
        assert_eq!(handles(),initial);
        caller.retire(stock());
    }
}

fn check_original_curved_classification(family: &str, operation: u64) {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solid-queries.json")).unwrap();
    for (index, row) in fixture[family].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(operation + index as u64), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let mut evaluate = |operator: &str, input| retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
        let [a, b] = ["a", "b"].map(|port| {
            let spec = &row[port];
            let (operator, input) = match spec["kind"].as_str().unwrap() {
                "box" => ("brep.prim3d.box", ["width", "depth", "height"].into_iter().zip(fixture_xyz(&spec["size"]))
                    .fold(Dictionary::new(), |input, (axis, extent)| input.insert(axis, number(extent)))),
                "cylinder" => ("brep.prim3d.cylinder", ["radius", "height"].into_iter()
                    .fold(Dictionary::new(), |input, key| input.insert(key, number(spec[key].as_f64().unwrap())))),
                "sphere" => ("brep.prim3d.sphere", Dictionary::new().insert("radius", number(spec["radius"].as_f64().unwrap()))),
                _ => panic!("unknown curved Boolean operand"),
            };
            let solid = evaluate(operator, input).get("solid").unwrap().clone();
            let offset = fixture_xyz(&spec["origin"]);
            if offset == [0.; 3] { return solid; }
            let translate = Dictionary::new().insert("geometry", solid).insert("offset", Value::Dictionary(vector_dictionary(offset)));
            evaluate("brep.xform.translate", translate).get("geometryOut").unwrap().clone()
        });
        let result = evaluate(row["operator"].as_str().unwrap(), Dictionary::new().insert("a", a).insert("b", b));
        let solid = result.get("solid").unwrap();
        for probe in row["points"].as_array().unwrap() {
            let input = Dictionary::new().insert("solid", solid.clone())
                .insert("point", Value::Dictionary(point_dictionary(fixture_xyz(&probe[0]))));
            let result = evaluate(fixture["classify"].as_str().unwrap(), input);
            assert_eq!(read_channel_number(&result, "classification").unwrap(), probe[1].as_f64().unwrap(), "{} {}", row["name"], probe[0]);
        }
        caller.retire(stock());
    }
}

#[test]
fn retained_compound_previews_each_original_planar_member() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-compound.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity=GeometryWorkIdentity{operation:97297,generation:17};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    let dims=|i:usize|fixture["dimensions"][i].as_f64().unwrap();
    let mut solids=Dictionary::new().insert("$schema",Value::Atom(Atom::String("list".into())));
    for (i,offset) in fixture["offsets"].as_array().unwrap().iter().enumerate(){
        let inputs=Dictionary::new().insert("width",number(dims(0))).insert("depth",number(dims(1))).insert("height",number(dims(2)));
        let box_=run_live_query(&registry,"brep.prim3d.box",inputs,&mut caller,&mut born,&mut free);
        let offset:[f64;3]=serde_json::from_value(offset.clone()).unwrap();
        let inputs=Dictionary::new().insert("geometry",box_.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(offset)));
        let moved=run_live_query(&registry,"brep.xform.translate",inputs,&mut caller,&mut born,&mut free);
        solids=solids.insert(i.to_string(),moved.get("geometryOut").unwrap().clone());
    }
    let input=Dictionary::new().insert("solids",Value::Dictionary(solids));
    let compound=run_live_query(&registry,"brep.topology.compound",input,&mut caller,&mut born,&mut free);
    let handle=read_geometry(&compound,"compound").unwrap();
    session.with_kernel_read(|kernel|{
        assert!(kernel.preview_validation_root(handle.as_str(),0).unwrap().is_some());
        assert!(kernel.preview_validation_root(handle.as_str(),1).unwrap().is_some());
        assert!(kernel.preview_validation_root(handle.as_str(),2).unwrap().is_none());
        Ok(())
    }).unwrap();
    let mesh=run_polygon_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
    mesh.validate_component_references().unwrap();
    let (area,volume)=preview_area_volume(&mesh);
    assert!((volume-fixture["volume"].as_f64().unwrap()).abs()<1e-6);
    assert!((area-fixture["area"].as_f64().unwrap()).abs()<1e-6);
    assert_eq!(mesh.indices.len()/3,fixture["triangles"].as_u64().unwrap() as usize);
    for (domain,key) in [("face","faces"),("edge","edges"),("vertex","vertices")]{
        assert_eq!(mesh.component_references[domain].len(),fixture[key].as_u64().unwrap() as usize);
    }
    caller.retire_exact();
}

#[test]
fn retained_line_intersections_match_the_registered_single_hit_contract() {
    use neural_engine::OperatorJobStep;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️line-intersections.json")).unwrap();
    let channel = fixture["output"].as_str().unwrap();
    let default_tolerance = fixture["tolerance"].as_f64().unwrap();
    let coordinates = |value: &serde_json::Value| Value::Dictionary(semio_s_spatial_kernel_semio_session::point_dictionary(fixture_xyz(value)));
    for (operator, rows) in [("brep.intersect.curveCurve", "curveCurve"), ("brep.intersect.curveSurface", "curveSurface")] {
        for (index, row) in fixture[rows].as_array().unwrap().iter().enumerate() {
            let session = Session::new();
            let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
            let mut caller = OriginalJobFixtureCaller::admit(OperationId(98260 + index as u64), Generation(17), stock(), || Some(1000), 1500, 32768);
            let (mut born, mut free) = (0, 0);
            let mut construct = |operation, shape: &serde_json::Value, input_names: [&str; 2]| {
                let input = input_names.into_iter().fold(Dictionary::new(), |input, field| input.insert(field, coordinates(&shape[field])));
                run_live_query(&registry, operation, input, &mut caller, &mut born, &mut free)
            };
            let (left, right, ports) = if rows == "curveCurve" {
                let a = construct("brep.curve.line", &row["a"], ["start", "end"]);
                let b = construct("brep.curve.line", &row["b"], ["start", "end"]);
                (a.get("curve").unwrap().clone(), b.get("curve").unwrap().clone(), ["a", "b"])
            } else {
                let line = construct("brep.curve.line", &row["line"], ["start", "end"]);
                let plane = construct("brep.surf.plane", &row["plane"], ["origin", "normal"]);
                (line.get("curve").unwrap().clone(), plane.get("surface").unwrap().clone(), ["curve", "surface"])
            };
            let input = || Dictionary::new().insert(ports[0], left.clone()).insert(ports[1], right.clone())
                .insert("tolerance", number(row["tolerance"].as_f64().unwrap_or(default_tolerance)));
            if let Some(reason) = row["refusal"].as_str() {
                let mut job = Some(admit_live_job(&registry, operator, input()));
                let mut refusal = None;
                for _ in 0..100 {
                    run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 256, ..stock() }, &mut born, &mut free, |cx| {
                        match job.as_mut().unwrap().step(cx) {
                            Ok((OperatorJobStep::Working(_), _)) => {},
                            Ok(_) => panic!("{} must refuse {}", operator, row["id"]),
                            Err(error) => refusal = Some(error),
                        }
                        Ok(())
                    });
                    if refusal.is_some() { break; }
                }
                assert!(format!("{:?}", refusal.expect("bounded intersection refuses")).contains(reason), "{}", row["id"]);
                close_live_job(&mut job, &mut caller, &mut born, &mut free);
                session.with_kernel_read(|kernel| {
                    assert!(kernel.representation().vertices.is_empty());
                    assert_eq!(kernel.live_handles().len(), 2);
                    Ok(())
                }).unwrap();
            } else {
                let result = run_live_query(&registry, operator, input(), &mut caller, &mut born, &mut free);
                let hit = semio_s_spatial_kernel_semio_session::read_geometry(&result, channel).unwrap();
                let kind = result.get(channel).and_then(Value::as_dictionary).and_then(|value| value.get("kind"));
                assert!(matches!(kind, Some(Value::Atom(Atom::String(kind))) if kind.as_str() == fixture["kind"].as_str().unwrap()));
                session.with_kernel_read(|kernel| {
                    assert_eq!(kernel.kind_sync(&hit).unwrap(), semio_framework_3d_brep_engine::GeometryKind::Vertex);
                    assert_eq!(kernel.live_handles().len(), 3);
                    let body = kernel.representation();
                    assert_eq!(body.vertices.len(), 1);
                    let (_, vertex) = body.vertices.iter().next().unwrap();
                    assert_eq!(kernel.label_of(&hit).unwrap(), vertex.label);
                    let actual = [vertex.position.x, vertex.position.y, vertex.position.z];
                    assert!(actual.into_iter().zip(fixture_xyz(&row["hit"])).all(|(a, b)| (a - b).abs() < 1e-12));
                    Ok(())
                }).unwrap();
            }
            caller.retire(stock());
        }
    }
}

#[test]
fn retained_validate_reports_the_selected_primitive_and_planar_face() {
    use semio_s_spatial_kernel_semio_session::point_dictionary;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-validation-report.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97322),Generation(17),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    let solid=run_live_query(&registry,"brep.prim3d.box",Dictionary::new().insert("width",number(1.0)).insert("depth",number(2.0)).insert("height",number(3.0)),
        &mut caller,&mut born,&mut free);
    let mut points=Dictionary::new().insert("$schema",Value::Atom(Atom::String("list".into())));
    for (i,point) in fixture["face"].as_array().unwrap().iter().enumerate(){
        let axes=serde_json::from_value::<[f64;3]>(point.clone()).unwrap();
        points=points.insert(i.to_string(),Value::Dictionary(point_dictionary(axes)));
    }
    let face=run_live_query(&registry,"brep.surf.planarFace",Dictionary::new().insert("points",Value::Dictionary(points)),&mut caller,&mut born,&mut free);
    for (value,channel) in [(&solid,"solid"),(&face,"face")] {
        let input=Dictionary::new().insert("geometry",value.get(channel).unwrap().clone());
        let report=run_live_query(&registry,"brep.measure.validate",input,&mut caller,&mut born,&mut free);
        let text=report.get("report").and_then(Value::as_dictionary).unwrap();
        assert_eq!(text.schema(),Some("text"));
        let json=text.get("value").and_then(Value::as_atom).and_then(Atom::as_str).unwrap();
        let report:serde_json::Value=serde_json::from_str(json).unwrap();
        assert_eq!(report,fixture["expected"]);
    }
    caller.retire_exact();
}

#[test]
fn retained_shell_selection_preserves_original_labels_and_order() {
    use neural_engine::OperatorJobStep;
    use semio_framework_3d_brep_representation::topology::{Body, Shell, Solid, history::{LabelSource, PersistentLabel}};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️shell-selection.json")).unwrap();
    let operator = fixture["operator"].as_str().unwrap();
    let channel = fixture["channel"].as_str().unwrap();
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let mut body = Body::new();
        let mut shell = |label: &serde_json::Value| body.shells.insert(Shell { faces: Vec::new(), label: PersistentLabel(label.as_u64().unwrap()) });
        let outer = shell(&row["outer"]);
        let inners = row["inners"].as_array().unwrap().iter().map(shell).collect();
        body.solids.insert(Solid { outer, inners, label: PersistentLabel(100) });
        body.labels = LabelSource::from_next(101);
        let offset = session.with_kernel_read(|kernel| Ok(kernel.representation().labels.next())).unwrap();
        let source = session.with_kernel(|kernel| Ok(kernel.merge_representation(body).remove(0))).unwrap();
        let expected = session.with_kernel_read(|kernel| {
            let id = kernel.geometry_solid(&source).unwrap();
            let body = kernel.representation();
            Ok(body.solid_shells(id).into_iter().map(|id| body.shells.get(id).unwrap().label).collect::<Vec<_>>())
        }).unwrap();
        let labels: Vec<_> = row["expected"].as_array().unwrap().iter().map(|value| PersistentLabel(offset + value.as_u64().unwrap())).collect();
        assert_eq!(expected, labels);
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let input = || Dictionary::new().insert(fixture["source"].as_str().unwrap(), Value::Dictionary(
            session.with_kernel_read(|kernel| semio_s_spatial_kernel_semio_session::geometry_dict(kernel, &source)).unwrap()
        ));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(98300 + index as u64), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let mut job = Some(admit_live_job(&registry, operator, input()));
        for _ in 0..10000 {
            if job.as_ref().unwrap().progress().phase == "shells-staged-claim" { break; }
            run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, OperatorJobStep::Working(_)));
                Ok(())
            });
        }
        assert_eq!(job.as_ref().unwrap().progress().phase, "shells-staged-claim");
        job.as_mut().unwrap().cancel();
        run(&mut caller, stock(), &mut born, &mut free, |cx| {
            assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, OperatorJobStep::Cancelled(_)));
            Ok(())
        });
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), 1);

        for _ in 0..2 {
            let value = run_live_query(&registry, operator, input(), &mut caller, &mut born, &mut free);
            let list = value.get(channel).and_then(Value::as_dictionary).unwrap();
            assert_eq!(list.schema(), fixture["schema"].as_str());
            assert_eq!(list.len(), expected.len() + 1);
            session.with_kernel_read(|kernel| {
                for (slot, label) in expected.iter().enumerate() {
                    let name = slot.to_string();
                    let member = list.get(&name).and_then(Value::as_dictionary).unwrap();
                    assert_eq!(member.schema(), fixture["member"]["schema"].as_str());
                    assert!(matches!(member.get("kind"), Some(Value::Atom(Atom::String(kind))) if kind.as_str() == fixture["member"]["kind"].as_str().unwrap()));
                    let handle = semio_s_spatial_kernel_semio_session::read_geometry(list, &name).unwrap();
                    assert_eq!(kernel.label_of(&handle).unwrap(), *label);
                    assert_eq!(kernel.kind_sync(&handle).unwrap(), semio_framework_3d_brep_engine::GeometryKind::Shell);
                }
                assert_eq!(kernel.live_handles().len(), expected.len() + 1);
                Ok(())
            }).unwrap();
        }
        caller.retire(stock());
    }
}

#[test]
fn retained_deconstruct_lists_only_original_reachable_box_components() {
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, read_geometry, text_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️deconstruct.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(98321), Generation(18), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let dimensions = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&fixture["box"]))
        .fold(Dictionary::new(), |input, (port, size)| input.insert(port, number(size)));
    let solid = run_live_query(&registry, "brep.prim3d.box", dimensions, &mut caller, &mut born, &mut free);
    let handle = read_geometry(&solid, "solid").unwrap();
    let operator = fixture["operator"].as_str().unwrap();
    let input = |collection: bool| {
        let value = solid.get("solid").unwrap().clone();
        let (value, captured) = if collection {
            (Value::Dictionary(Dictionary::with_schema("list").insert("9", value)), handle.as_str())
        } else { (value, "") };
        Dictionary::new().insert("brep", value)
            .insert("edgeLabels", Value::Dictionary(text_dictionary("[]")))
            .insert("faceLabels", Value::Dictionary(text_dictionary("[]")))
            .insert("sourceHandle", Value::Dictionary(text_dictionary(captured)))
    };
    let mut cancelled = Some(admit_live_job(&registry, operator, input(false)));
    for _ in 0..3 {
        run(&mut caller, stock(), &mut born, &mut free, |cx| {
            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
            Ok(())
        });
    }
    assert_eq!(cancelled.as_ref().unwrap().progress().phase, "deconstruct-reach");
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, stock(), &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), 1);

    let missing = input(false).insert("edgeLabels", Value::Dictionary(text_dictionary(r#"["18446744073709551615"]"#)));
    let mut refused = Some(admit_live_job(&registry, operator, missing));
    let mut failed = false;
    for _ in 0..2000 {
        run(&mut caller, stock(), &mut born, &mut free, |cx| {
            match refused.as_mut().unwrap().step(cx) {
                Ok((neural_engine::OperatorJobStep::Working(_), _)) => {},
                Err(_) => failed = true,
                _ => panic!("a missing label cannot publish topology"),
            }
            Ok(())
        });
        if failed { break; }
    }
    assert!(failed);
    close_live_job(&mut refused, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), 1);

    let mut selected = None;
    for collection in [false, true] {
        let value = run_live_query(&registry, operator, input(collection), &mut caller, &mut born, &mut free);
        assert_eq!(read_geometry(&value, "brepOut").unwrap().as_str(), handle.as_str());
        let index = if collection { 9 } else { fixture["sourceIndex"].as_u64().unwrap() };
        let source_index = value.get("sourceIndex").and_then(Value::as_dictionary).unwrap();
        assert_eq!(source_index.schema(), Some("number"));
        assert!(matches!(source_index.get("value"), Some(Value::Atom(Atom::Decimal(value))) if *value == index as f64));
        for channel in fixture["emptyChannels"].as_array().unwrap() {
            let list = value.get(channel.as_str().unwrap()).and_then(Value::as_dictionary).unwrap();
            assert_eq!(list.schema(), Some("list"));
            assert_eq!(list.len(), 1);
        }
        session.with_kernel(|kernel| {
            let stock = kernel.deconstruct_sync(&handle).map_err(|error| map_kernel_error(&error))?;
            for (channel, original) in [("vertex", stock.vertices), ("edge", stock.edges), ("face", stock.faces), ("shell", stock.shells)] {
                let list = value.get(channel).and_then(Value::as_dictionary).unwrap();
                assert_eq!(list.len() - 1, fixture["counts"][channel].as_u64().unwrap() as usize);
                assert_eq!(list.schema(), Some("list"));
                for (index, sought) in original.iter().enumerate() {
                    let row = index.to_string();
                    let actual = read_geometry(list, &row)?;
                    assert_eq!(actual.as_str(), sought.as_str());
                    assert_eq!(list.get(&row).and_then(Value::as_dictionary).unwrap().schema(), Some(channel));
                }
            }
            let rows = |channel: &str| {
                let list = value.get(channel).and_then(Value::as_dictionary).unwrap();
                let mut labels: Vec<_> = ["0", "1"].into_iter().map(|row| {
                    let handle = read_geometry(list, row).unwrap();
                    (kernel.label_of(&handle).unwrap().0, handle)
                }).collect();
                labels.sort_by_key(|(label, _)| *label);
                labels
            };
            selected = Some((rows("edge"), rows("face")));
            Ok(())
        }).unwrap();
    }
    let (edges, faces) = selected.unwrap();
    let labels = |rows: &[(u64, _)]| Value::Dictionary(text_dictionary(format!(r#"["{}","{}","{}"]"#, rows[1].0, rows[0].0, rows[1].0)));
    let selection = input(true).insert("edgeLabels", labels(&edges)).insert("faceLabels", labels(&faces));
    let output = run_live_query(&registry, operator, selection, &mut caller, &mut born, &mut free);
    for (channel, expected) in [("selectedEdges", edges), ("selectedFaces", faces)] {
        let list = output.get(channel).and_then(Value::as_dictionary).unwrap();
        assert_eq!(list.len(), 3);
        for (index, (_, handle)) in expected.iter().enumerate() {
            assert_eq!(read_geometry(list, &index.to_string()).unwrap().as_str(), handle.as_str());
        }
    }
    caller.retire(stock());
}

#[test]
fn retained_explode_restores_compound_member_aliases_in_authored_order() {
    use neural_engine::OperatorJobStep;
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, read_geometry};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️explode.json")).unwrap();
    let operator = fixture["operator"].as_str().unwrap();
    let channel = fixture["channel"].as_str().unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(98320), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let solids: Vec<_> = fixture["boxes"].as_array().unwrap().iter().map(|row| {
        let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(row))
            .fold(Dictionary::new(), |input, (port, dimension)| input.insert(port, number(dimension)));
        run_live_query(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free)
    }).collect();
    let handles: Vec<_> = solids.iter().map(|value| read_geometry(value, "solid").unwrap()).collect();
    let members: Vec<_> = fixture["members"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as usize).collect();
    let list = members.iter().enumerate().fold(Dictionary::with_schema("list"), |list, (index, &member)| {
        list.insert(index.to_string(), solids[member].get("solid").unwrap().clone())
    });
    let compound = run_live_query(&registry, "brep.topology.compound", Dictionary::new().insert("solids", Value::Dictionary(list)),
        &mut caller, &mut born, &mut free);
    let compound_handle = read_geometry(&compound, "compound").unwrap();
    let labels: Vec<_> = session.with_kernel_read(|kernel| {
        Ok(handles.iter().map(|handle| kernel.label_of(handle).unwrap()).collect())
    }).unwrap();
    let retained = fixture["keepAlias"].as_u64().unwrap() as usize;
    session.retain_geometry_handles(&[compound_handle.as_str().to_owned(), handles[retained].as_str().to_owned()]);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), 2);
    assert!(session.with_kernel_read(|kernel| Ok(kernel.live_solid_primitive(handles[retained].as_str()).is_some())).unwrap());

    let input = || Dictionary::new().insert(fixture["source"].as_str().unwrap(), compound.get("compound").unwrap().clone());
    let mut cancelled = Some(admit_live_job(&registry, operator, input()));
    for _ in 0..10000 {
        if cancelled.as_ref().unwrap().progress().phase == "explode-staged-claim" { break; }
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() }, &mut born, &mut free, |cx| {
            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, OperatorJobStep::Working(_)));
            Ok(())
        });
    }
    assert_eq!(cancelled.as_ref().unwrap().progress().phase, "explode-staged-claim");
    cancelled.as_mut().unwrap().cancel();
    run(&mut caller, stock(), &mut born, &mut free, |cx| {
        assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, OperatorJobStep::Cancelled(_)));
        Ok(())
    });
    close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
    assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), 2);

    for _ in 0..2 {
        let output = run_live_query(&registry, operator, input(), &mut caller, &mut born, &mut free);
        let list = output.get(channel).and_then(Value::as_dictionary).unwrap();
        assert_eq!(list.schema(), fixture["schema"].as_str());
        assert_eq!(list.len(), members.len() + 1);
        session.with_kernel_read(|kernel| {
            for (index, &member_index) in members.iter().enumerate() {
                let key = index.to_string();
                let handle = read_geometry(list, &key)?;
                assert_eq!(handle.as_str(), handles[member_index].as_str());
                assert_eq!(kernel.label_of(&handle).unwrap(), labels[member_index]);
                let member = list.get(&key).and_then(Value::as_dictionary).unwrap();
                assert!(matches!(member.get("kind"), Some(Value::Atom(Atom::String(kind))) if kind.as_str() == fixture["memberKind"].as_str().unwrap()));
                let expected = fixture["expectedVolumes"][index].as_f64().unwrap();
                let actual = kernel.volume_sync(&handle).map_err(|error| map_kernel_error(&error))?;
                assert!((actual - expected).abs() < 1e-9);
            }
            assert_eq!(kernel.live_handles().len(), 3);
            assert!(kernel.live_solid_primitive(handles[retained].as_str()).is_some());
            Ok(())
        }).unwrap();
    }
    caller.retire(stock());
}

#[test]
fn original_closest_point_respects_skew_plane_and_circular_trims(){
    use semio_framework_3d_brep_curve::{Curve3,bspline::KnotVector};
    use semio_framework_3d_brep_engine::{ShapeRoot,ShapeValue};
    use semio_framework_3d_brep_representation::topology::{Body,Edge,Vertex};
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
    use semio_s_spatial_kernel_semio_session::{geometry_dict,map_kernel_error,point_dictionary,read_channel_number,read_xyz};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-analytic-projection.json")).unwrap();
    let xyz=|row:&serde_json::Value|Value::Dictionary(point_dictionary(fixture_xyz(row)));
    let point=|row:&serde_json::Value|{let [x,y,z]=fixture_xyz(row);Pnt3::new(x,y,z)};
    let vector=|row:&serde_json::Value|{let [x,y,z]=fixture_xyz(row);Vec3::new(x,y,z)};
    let assert_point=|value:&Dictionary,row:&serde_json::Value|{
        for (a,b) in read_xyz(value,"pointOut").unwrap().into_iter().zip(fixture_xyz(&row["point"])){assert!((a-b).abs()<1e-10);}
    };
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let import=|shape:ShapeValue|Value::Dictionary(session.with_kernel(|kernel|{
        let handle=kernel.import_shape(&shape).map_err(|e|map_kernel_error(&e))?;
        geometry_dict(kernel,&handle)
    }).unwrap());
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97483),Generation(17),stock(),||Some(1000),1500,32768);
    let(mut born,mut free)=(0,0);

    let row=&fixture["skewPlane"];
    let frame=Frame3{origin:point(&row["origin"]),x:vector(&row["xAxis"]),y:vector(&row["yAxis"]),z:vector(&row["normal"])};
    let mut body=Body::new();
    let label=body.new_label();
    let surface=import(ShapeValue{root:ShapeRoot::Surface{surface:Surface::Plane{frame},label},body});
    let result=run_live_query(&registry,"brep.eval.surfaceClosestUv",Dictionary::new().insert("surface",surface).insert("point",xyz(&row["target"])),&mut caller,&mut born,&mut free);
    assert_point(&result,row);
    for (i,key) in ["u","v"].into_iter().enumerate(){assert!((read_channel_number(&result,key).unwrap()-row["uv"][i].as_f64().unwrap()).abs()<1e-10);}

    let rational:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/rational-plane-nearest.json")).unwrap();
    let knots=|axis:usize|{
        let [low,high]:[f64;2]=serde_json::from_value(rational["domain"][axis].clone()).unwrap();
        KnotVector::new(vec![low,low,high,high],1,2).unwrap()
    };
    let controls=rational["controls"].as_array().unwrap().iter()
        .map(|row|row.as_array().unwrap().iter().map(point).collect()).collect();
    let surface=Surface::Nurbs{u_knots:knots(0),v_knots:knots(1),controls,
        weights:serde_json::from_value(rational["weights"].clone()).unwrap()};
    let mut body=Body::new();
    let label=body.new_label();
    let patch=import(ShapeValue{root:ShapeRoot::Surface{surface,label},body});
    for row in rational["queries"].as_array().unwrap(){
        let input=Dictionary::new().insert("surface",patch.clone()).insert("point",xyz(&row["target"]));
        let output=run_live_query(&registry,"brep.eval.surfaceClosestUv",input,&mut caller,&mut born,&mut free);
        assert_point(&output,row);
        for (index,key) in ["u","v"].into_iter().enumerate(){
            assert!((read_channel_number(&output,key).unwrap()-row["uv"][index].as_f64().unwrap()).abs()<1e-10);
        }
        assert!((read_channel_number(&output,"distance").unwrap()-row["distance"].as_f64().unwrap()).abs()<1e-10);
        let input=Dictionary::new().insert("geometry",patch.clone()).insert("point",xyz(&row["target"]));
        let output=run_live_query(&registry,"brep.measure.closestPoint",input,&mut caller,&mut born,&mut free);
        assert_point(&output,row);
    }

    for row in fixture["circularEdge"].as_array().unwrap(){
        let [lo,hi]:[f64;2]=serde_json::from_value(row["range"].clone()).unwrap();
        let mut body=Body::new();
        let curve=Curve3::Circle{frame:Frame3::WORLD,radius:row["radius"].as_f64().unwrap()};
        let [v0,v1]=[lo,hi].map(|t|{
            let label=body.new_label();
            body.vertices.insert(Vertex{position:curve.eval(t),label,tol:Default::default()})
        });
        let curve=body.curves3.insert(curve);
        let label=body.new_label();
        let id=body.edges.insert(Edge{curve,range:(lo,hi),v0,v1,label,tol:Default::default()});
        let edge=import(ShapeValue{root:ShapeRoot::Edge(id),body});
        let result=run_live_query(&registry,"brep.measure.closestPoint",Dictionary::new().insert("geometry",edge).insert("point",xyz(&row["target"])),&mut caller,&mut born,&mut free);
        assert_point(&result,row);
    }
    let disk=&fixture["disk"];
    let curve=run_live_query(&registry,"brep.curve.circle",Dictionary::new()
        .insert("center",Value::Dictionary(point_dictionary([0.0;3])))
        .insert("normal",Value::Dictionary(point_dictionary([0.0,0.0,1.0])))
        .insert("radius",number(disk["radius"].as_f64().unwrap())),&mut caller,&mut born,&mut free);
    let face=run_live_query(&registry,"brep.util.faceFromWire",Dictionary::new().insert("wire",curve.get("curve").unwrap().clone()),&mut caller,&mut born,&mut free);
    for row in disk["points"].as_array().unwrap(){
        let result=run_live_query(&registry,"brep.measure.closestPoint",Dictionary::new().insert("geometry",face.get("face").unwrap().clone()).insert("point",xyz(&row["target"])),&mut caller,&mut born,&mut free);
        assert_point(&result,row);
    }
    caller.retire_exact();
}

#[test]
fn original_closest_point_uses_certified_primitive_boundaries(){
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_xyz};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-analytic-projection.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97484),Generation(17),stock(),||Some(1000),1500,32768);
    let(mut born,mut free)=(0,0);
    for solid in fixture["solids"].as_array().unwrap(){
        let kind=solid["kind"].as_str().unwrap();
        let method=format!("brep.prim3d.{kind}");
        let parameters=solid["parameters"].as_object().unwrap().iter().fold(Dictionary::new(),|dict,(key,value)|dict.insert(key.as_str(),number(value.as_f64().unwrap())));
        let out=run_live_query(&registry,&method,parameters,&mut caller,&mut born,&mut free);
        let handle=out.get("solid").unwrap();
        for row in solid["points"].as_array().unwrap(){
            let query=Dictionary::new().insert("geometry",handle.clone()).insert("point",Value::Dictionary(point_dictionary(fixture_xyz(&row["target"]))));
            let value=run_live_query(&registry,"brep.measure.closestPoint",query,&mut caller,&mut born,&mut free);
            for (actual,expected) in read_xyz(&value,"pointOut").unwrap().into_iter().zip(fixture_xyz(&row["point"])){
                assert!((actual-expected).abs()<1e-10,"{kind}: {actual} != {expected}");
            }
        }
    }
    caller.retire_exact();
}

#[test]
fn original_closest_point_keeps_analytic_boolean_cavity_boundaries(){
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,read_xyz,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/derived-solid-nearest.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97485),Generation(17),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    for kind in ["sphereShell","boxShell"]{
        let row=&fixture[kind];
        let mut placed=|operator:&str,parameters:Dictionary,offset:&serde_json::Value|{
            let source=run_live_query(&registry,operator,parameters,&mut caller,&mut born,&mut free);
            let input=Dictionary::new().insert("geometry",source.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(offset))));
            run_live_query(&registry,"brep.xform.translate",input,&mut caller,&mut born,&mut free)
                .get("geometryOut").unwrap().clone()
        };
        let outer=if kind=="sphereShell"{
            placed("brep.prim3d.sphere",Dictionary::new().insert("radius",number(row["outerRadius"].as_f64().unwrap())),&row["outer"])
        }else{
            let [width,depth,height]=fixture_xyz(&row["size"]);
            let parameters=Dictionary::new().insert("width",number(width)).insert("depth",number(depth)).insert("height",number(height));
            placed("brep.prim3d.box",parameters,&row["origin"])
        };
        let inner=placed("brep.prim3d.sphere",Dictionary::new().insert("radius",number(row["innerRadius"].as_f64().unwrap())),&row["inner"]);
        let input=Dictionary::new().insert("a",outer).insert("b",inner);
        let cut=run_live_query(&registry,"brep.bool.cut",input,&mut caller,&mut born,&mut free);
        let input=Dictionary::new().insert("geometry",cut.get("solid").unwrap().clone());
        let output=run_live_query(&registry,"brep.measure.boundingBox",input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"box").unwrap();
        let actual=session.with_kernel_read(|kernel|Ok(kernel.live_solid_primitive(handle.as_str()).unwrap().axis_aligned_bounds())).unwrap();
        for (point,expected) in actual.into_iter().zip(row["bounds"].as_array().unwrap()){
            assert!(point.to_array().into_iter().zip(fixture_xyz(expected)).all(|(a,b)|(a-b).abs()<1e-10));
        }
        for probe in row["queries"].as_array().unwrap(){
            let query=Dictionary::new().insert("geometry",cut.get("solid").unwrap().clone())
                .insert("point",Value::Dictionary(point_dictionary(fixture_xyz(&probe["target"]))));
            let output=run_live_query(&registry,"brep.measure.closestPoint",query,&mut caller,&mut born,&mut free);
            assert!(read_xyz(&output,"pointOut").unwrap().into_iter().zip(fixture_xyz(&probe["point"]))
                .all(|(actual,expected)|(actual-expected).abs()<1e-10));
        }
    }
    caller.retire_exact();
}

#[test]
fn original_closest_point_traverses_the_authored_planar_face_with_holes() {
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_engine::{ShapeRoot, ShapeValue};
    use semio_framework_3d_brep_representation::topology::{Body, Edge, Face, Vertex, make_loop};
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};
    use semio_s_spatial_kernel_semio_session::{geometry_dict, map_kernel_error, point_dictionary, read_channel_number, read_xyz};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️planar-face-projection.json")).unwrap();
    let point = |row: &serde_json::Value| {
        let [x, y, z] = fixture_xyz(row);
        Pnt3::new(x, y, z)
    };
    let vector = |row: &serde_json::Value| {
        let [x, y, z] = fixture_xyz(row);
        Vec3::new(x, y, z)
    };
    let frame = &fixture["frame"];
    let frame = Frame3 { origin: point(&frame["origin"]), x: vector(&frame["xAxis"]), y: vector(&frame["yAxis"]), z: vector(&frame["normal"]) };
    let mut body = Body::new();
    let surface = body.surfaces.insert(Surface::Plane { frame });
    let label = body.new_label();
    let face = body.faces.insert(Face { surface, outer: None, inners: Vec::new(), flipped: true, tol: Default::default(), label });
    for (index, ring) in std::iter::once(&fixture["outer"]).chain(fixture["holes"].as_array().unwrap()).enumerate() {
        let vertices: Vec<_> = ring.as_array().unwrap().iter().map(|row| {
            let position = point(row);
            let label = body.new_label();
            (body.vertices.insert(Vertex { position, label, tol: Default::default() }), position)
        }).collect();
        let members: Vec<_> = (0..vertices.len()).map(|i| {
            let forward = i % 2 == 0;
            let first = vertices[i];
            let next = vertices[(i + 1) % vertices.len()];
            let ((v0, a), (v1, b)) = if forward { (first, next) } else { (next, first) };
            let dir = (b - a) * 0.5;
            let curve = body.curves3.insert(Curve3::Line { origin: a - dir * 2.0, dir });
            let label = body.new_label();
            let edge = body.edges.insert(Edge { curve, range: (2.0, 4.0), v0, v1, label, tol: Default::default() });
            (edge, forward)
        }).collect();
        let ring = make_loop(&mut body, face, &members);
        let data = body.faces.get_mut(face).unwrap();
        if index == 0 { data.outer = Some(ring); } else { data.inners.push(ring); }
    }
    let shape = ShapeValue { root: ShapeRoot::Face(face), body };
    let session = Session::new();
    let (face_handle, handle) = session.with_kernel(|kernel| {
        let handle = kernel.import_shape(&shape).map_err(|error| map_kernel_error(&error))?;
        let dictionary = geometry_dict(kernel, &handle)?;
        Ok((handle, dictionary))
    }).unwrap();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97485), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let area = run_live_query(&registry, "brep.measure.area", Dictionary::new().insert("geometry", Value::Dictionary(handle.clone())), &mut caller, &mut born, &mut free);
    assert!((read_channel_number(&area, "area").unwrap() - fixture["area"].as_f64().unwrap()).abs() < 1e-10);
    let expected_turns = 2 + 3 + fixture["outer"].as_array().unwrap().len() +
        fixture["holes"].as_array().unwrap().iter().map(|ring| ring.as_array().unwrap().len()).sum::<usize>();
    for (index, row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let target = Value::Dictionary(point_dictionary(fixture_xyz(&row["target"])));
        let vertex = run_live_query(&registry, "brep.util.vertex", Dictionary::new().insert("point", target.clone()), &mut caller, &mut born, &mut free)
            .get("vertex").unwrap().clone();
        let face = Value::Dictionary(handle.clone());
        let closest = Dictionary::new().insert("geometry", face.clone()).insert("point", target);
        let jobs = [("brep.measure.closestPoint", closest), ("brep.measure.distance", Dictionary::new().insert("a", vertex.clone()).insert("b", face.clone())),
            ("brep.measure.distance", Dictionary::new().insert("a", face).insert("b", vertex))];
        if index == 0 {
            let mut cancelled = Some(admit_live_job(&registry, "brep.measure.distance", jobs[1].1.clone()));
            for _ in 0..7 {
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (status, receipt) = cancelled.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(status, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.fits(grant));
                    Ok(())
                });
            }
            assert_eq!(cancelled.as_ref().unwrap().progress().phase, "planar-closest-point");
            cancelled.as_mut().unwrap().cancel();
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
        }
        for (operator, input) in jobs {
            let mut job = Some(admit_live_job(&registry, operator, input));
            let mut turns = 0;
            let mut result = None;
            for _ in 0..100 {
                let job = job.as_mut().unwrap();
                let projecting = job.progress().phase == "planar-closest-point";
                if projecting && turns == 0 {
                    run(&mut caller, Grant { maximum_copy_bytes: 0, ..grant }, &mut born, &mut free, |cx| {
                        let (step, receipt) = job.step(cx).unwrap();
                        assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                        assert_eq!(receipt, Progress::default());
                        Ok(())
                    });
                }
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.step(cx).unwrap();
                    if projecting {
                        assert_eq!(receipt.copied_items, 1);
                        assert_eq!(receipt.retained_capacity_bytes, 0);
                        turns += 1;
                    }
                    if let neural_engine::OperatorJobStep::Done(value) = step { result = Some(value); }
                    Ok(())
                });
                if result.is_some() { break; }
            }
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            assert_eq!(turns, expected_turns);
            let result = neural_engine::ColdOwner::new(result.expect("planar projection completes under one-item grants"));
            if operator == "brep.measure.distance" {
                let actual = read_channel_number(&result, "distance").unwrap();
                assert!((actual - row["distance"].as_f64().unwrap()).abs() < 1e-10, "{}: {actual}", row["name"]);
            } else {
                let actual = read_xyz(&result, "pointOut").unwrap();
                let expected = fixture_xyz(&row["point"]);
                assert!(actual.into_iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-10), "{} {:?}", row["name"], actual);
            }
        }
    }

    for (index, row) in fixture["linearDistance"].as_array().unwrap().iter().enumerate() {
        let origin = point(&row["origin"]);
        let direction = vector(&row["direction"]);
        let source = if row["range"].is_array() {
            let range: (f64, f64) = serde_json::from_value(row["range"].clone()).unwrap();
            let mut body = Body::new();
            let support = Curve3::Line { origin, dir: direction };
            let [v0, v1] = [range.0, range.1].map(|t| {
                let label = body.new_label();
                body.vertices.insert(Vertex { position: support.eval(t), label, tol: Default::default() })
            });
            let curve = body.curves3.insert(support);
            let label = body.new_label();
            let edge = body.edges.insert(Edge { curve, range, v0, v1, label, tol: Default::default() });
            let shape = ShapeValue { root: ShapeRoot::Edge(edge), body };
            session.with_kernel(|kernel| {
                let live = kernel.import_shape(&shape).map_err(|error| map_kernel_error(&error))?;
                Ok(Value::Dictionary(geometry_dict(kernel, &live)?))
            }).unwrap()
        } else {
            let input = Dictionary::new().insert("start", Value::Dictionary(point_dictionary(origin.to_array())))
                .insert("end", Value::Dictionary(point_dictionary((origin + direction).to_array())));
            run_live_query(&registry, "brep.curve.line", input, &mut caller, &mut born, &mut free).get("curve").unwrap().clone()
        };
        let face = Value::Dictionary(handle.clone());
        for (order, (left, right)) in [(&source, &face), (&face, &source)].into_iter().enumerate() {
            let input = Dictionary::new().insert("a", left.clone()).insert("b", right.clone());
            if index == 0 && order == 0 {
                let mut cancelled = Some(admit_live_job(&registry, "brep.measure.distance", input.clone()));
                for _ in 0..6 {
                    run(&mut caller, grant, &mut born, &mut free, |cx| {
                        let (step, receipt) = cancelled.as_mut().unwrap().step(cx).unwrap();
                        assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)) && receipt.fits(grant));
                        Ok(())
                    });
                }
                assert_eq!(cancelled.as_ref().unwrap().progress().phase, "planar-closest-point");
                cancelled.as_mut().unwrap().cancel();
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                    Ok(())
                });
                close_live_job(&mut cancelled, &mut caller, &mut born, &mut free);
            }
            let result = retained_boolean_result(&registry, "brep.measure.distance", input, &mut caller, &mut born, &mut free);
            let distance = read_channel_number(&result, "distance").unwrap();
            assert!((distance - row["distance"].as_f64().unwrap()).abs() < 1e-10, "{}: {distance}", row["id"]);
        }
    }

    let mut curved = shape;
    let outer = curved.body.faces.get(face).unwrap().outer.unwrap();
    let first = curved.body.loops.get(outer).unwrap().first;
    let edge = curved.body.coedges.get(first).unwrap().edge;
    let unit = std::f64::consts::FRAC_1_SQRT_2;
    let arc_frame = Frame3 {
        origin: Pnt3::new(14.0, 20.0, 30.0),
        x: Vec3::new(1.0, 0.0, 0.0),
        y: Vec3::new(0.0, unit, unit),
        z: Vec3::new(0.0, -unit, unit),
    };
    let arc = curved.body.curves3.insert(Curve3::Circle { frame: arc_frame, radius: 4.0 });
    let edge = curved.body.edges.get_mut(edge).unwrap();
    edge.curve = arc;
    edge.range = (std::f64::consts::PI, std::f64::consts::TAU);
    let (other_face, curved_handle) = session.with_kernel(|kernel| {
        let handle = kernel.import_shape(&curved).map_err(|error| map_kernel_error(&error))?;
        let dictionary = geometry_dict(kernel, &handle)?;
        Ok((handle, dictionary))
    }).unwrap();
    let curved = Value::Dictionary(curved_handle);
    let target = Value::Dictionary(point_dictionary([14.0, 20.0, 30.0]));
    let vertex = run_live_query(&registry, "brep.util.vertex", Dictionary::new().insert("point", target.clone()), &mut caller, &mut born, &mut free)
        .get("vertex").unwrap().clone();
    for (operator, input) in [
        ("brep.measure.closestPoint", Dictionary::new().insert("geometry", curved.clone()).insert("point", target)),
        ("brep.measure.distance", Dictionary::new().insert("a", vertex.clone()).insert("b", curved.clone())),
        ("brep.measure.distance", Dictionary::new().insert("a", curved).insert("b", vertex)),
    ] {
        let mut job = Some(admit_live_job(&registry, operator, input));
        let mut error = None;
        for _ in 0..8 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                match job.as_mut().unwrap().step(cx) {
                    Ok((neural_engine::OperatorJobStep::Working(_), _)) => {}
                    Ok(_) => panic!("mixed face boundary must refuse {operator}"),
                    Err(refusal) => error = Some(refusal),
                }
                Ok(())
            });
            if error.is_some() { break; }
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert!(format!("{:?}", error.expect("mixed circular trim must refuse")).contains("UnsupportedOwner"));
    }

    let solid = session.with_kernel(|kernel| kernel.box_prim_sync(1.0, 1.0, 1.0).map_err(|error| map_kernel_error(&error))).unwrap();
    let cursor = session.plan_planar_closest(face_handle.as_str(), [14.0, 20.0, 30.0]).unwrap().unwrap();
    assert!(session.planar_closest_demand(face_handle.as_str(), &cursor).is_ok());
    assert!(session.planar_closest_demand(other_face.as_str(), &cursor).is_err());
    assert!(Session::new().planar_closest_demand(face_handle.as_str(), &cursor).is_err());
    session.with_kernel(|kernel| kernel.heal_solid_sync(&solid, 1e-6).map_err(|error| map_kernel_error(&error))).unwrap();
    assert!(session.planar_closest_demand(face_handle.as_str(), &cursor).is_err());
    caller.retire_exact();
}

#[test]
fn original_certified_solid_patterns_publish_one_compound_and_roll_back_private_members() {
    use semio_framework_3d_brep_engine::ShapeRoot;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, point_dictionary, read_geometry};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️certified-patterns.json")).unwrap();
    let points = |axes| Value::Dictionary(point_dictionary(axes));
    let ports = |row: &serde_json::Value, source: &Value| {
        row.as_object().unwrap().iter().fold(Dictionary::new().insert("geometry", source.clone()), |result, (key, value)| {
            result.insert(key.as_str(), if value.is_array() { points(fixture_xyz(value)) } else { number(value.as_f64().unwrap()) })
        })
    };
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97743), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 512, ..stock() };
    let mut rollback_source = None;
    for row in fixture["cases"].as_array().unwrap() {
        let (constructor, input) = if row["source"]["kind"] == "box" {
            let [width, depth, height] = fixture_xyz(&row["source"]["size"]);
            ("brep.prim3d.box", Dictionary::new().insert("width", number(width)).insert("depth", number(depth)).insert("height", number(height)))
        } else {
            ("brep.prim3d.sphere", Dictionary::new().insert("radius", number(row["source"]["radius"].as_f64().unwrap())))
        };
        let solid = run_live_query(&registry, constructor, input, &mut caller, &mut born, &mut free);
        let source = run_live_query(&registry, "brep.xform.translate", Dictionary::new()
            .insert("geometry", solid.get("solid").unwrap().clone()).insert("offset", points(fixture_xyz(&row["source"]["origin"]))),
            &mut caller, &mut born, &mut free);
        let geometry = source.get("geometryOut").unwrap();
        let original = read_geometry(&source, "geometryOut").unwrap();
        let original_label = session.with_kernel_read(|kernel| Ok(kernel.label_of(&original).unwrap())).unwrap();
        let initial_handles = session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap();
        let method = match row["kind"].as_str().unwrap() {
            "linear" => { rollback_source = Some(geometry.clone()); "brep.xform.linearPattern" }
            "circular" => "brep.xform.circularPattern",
            "grid" => "brep.xform.gridPattern",
            _ => unreachable!(),
        };
        let mut job = Some(admit_live_job(&registry, method, ports(&row["parameters"], geometry)));
        let mut output = None;
        for _ in 0..6000 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                assert!(receipt.copied_items <= 1);
                if let neural_engine::OperatorJobStep::Done(value) = step { output = Some(value); }
                Ok(())
            });
            if output.is_some() { break; }
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let output = neural_engine::ColdOwner::new(output.expect("registered pattern completes under one work item"));
        let compound = read_geometry(&output, "compound").unwrap();
        let shape = session.with_kernel_read(|kernel| {
            assert_eq!(kernel.live_handles().len(), initial_handles + 1);
            kernel.export_shape(&compound).map_err(|error| map_kernel_error(&error))
        }).unwrap();
        let ShapeRoot::Compound { solids, .. } = shape.root else { panic!("pattern must publish the registered compound kind"); };
        assert_eq!(shape.body.solids.get(solids[0]).unwrap().label, original_label);
        let centers = row["centers"].as_array().unwrap();
        assert_eq!(solids.len(), centers.len());
        for (id, center) in solids.into_iter().zip(centers) {
            let reach = shape.body.reachable_from(&[EntityRef::Solid(id)]);
            let mut lo = [f64::INFINITY; 3];
            let mut hi = [f64::NEG_INFINITY; 3];
            for id in reach.vertices.iter() {
                let point = shape.body.vertices.get(*id).unwrap().position.to_array();
                for i in 0..3 { lo[i] = lo[i].min(point[i]); hi[i] = hi[i].max(point[i]); }
            }
            let expected = fixture_xyz(center);
            assert!((0..3).all(|i| (0.5 * (lo[i] + hi[i]) - expected[i]).abs() < 1e-10), "{method} {center}");
        }
    }

    let geometry = rollback_source.unwrap();
    let totals = || session.with_kernel_read(|kernel| {
        let b = kernel.tessellation_body();
        Ok((kernel.live_handles().len(), [b.solids.len(), b.shells.len(), b.faces.len(), b.loops.len(),
            b.coedges.len(), b.edges.len(), b.vertices.len(), b.surfaces.len(), b.curves2.len(), b.curves3.len()]))
    }).unwrap();
    let before = totals();
    let mut job = Some(admit_live_job(&registry, "brep.xform.linearPattern", ports(&fixture["cases"][0]["parameters"], &geometry)));
    for _ in 0..500 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
            Ok(())
        });
        if totals().1[0] > before.1[0] { break; }
    }
    assert!(totals().1[0] > before.1[0]);
    assert_eq!(totals().0, before.0);
    job.as_mut().unwrap().cancel();
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert_eq!(totals(), before);

    let mut job = Some(admit_live_job(&registry, "brep.xform.linearPattern", ports(&fixture["overlap"], &geometry)));
    let mut refusal = None;
    for _ in 0..16 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            match job.as_mut().unwrap().step(cx) {
                Ok((neural_engine::OperatorJobStep::Working(_), _)) => {}
                Ok(_) => panic!("overlapping sources cannot publish a pattern compound"),
                Err(error) => refusal = Some(error),
            }
            Ok(())
        });
        if refusal.is_some() { break; }
    }
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert!(format!("{:?}", refusal.expect("overlap must refuse")).contains("UnsupportedOwner"));
    assert_eq!(totals(), before);
    caller.retire_exact();
}

#[test]
fn original_surface_intersection_emits_one_continuous_registered_curve() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_geometry, read_xyz};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    for (index, row) in fixture["retainedPlaneIntersection"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 98420 + index as u64, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        let (mut born, mut free) = (0,0);
        let mut input = Dictionary::new().insert("tolerance", number(tolerance));
        for key in ["a","b"] {
            let plane = ["origin","normal"].into_iter().fold(Dictionary::new(), |dict, port| {
                dict.insert(port, Value::Dictionary(point_dictionary(fixture_xyz(&row[key][port]))))
            });
            let out = run_live_query(&registry, "brep.surf.plane", plane, &mut caller, &mut born, &mut free);
            input = input.insert(key, out.get("surface").unwrap().clone());
        }
        let operator = "brep.intersect.surfaceSurface";
        let mut job = Some(admit_live_job(&registry, operator, input.clone()));
        if let Some(reason) = row["fault"].as_str() {
            let mut failure = None;
            for _ in 0..32 {
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    match job.as_mut().unwrap().step(cx) {
                        Ok((neural_engine::OperatorJobStep::Working(_), _)) => {}
                        Ok(_) => panic!("expected refused supporting planes"),
                        Err(error) => failure = Some(error),
                    }
                    Ok(())
                });
                if failure.is_some() { break; }
            }
            assert!(format!("{:?}",failure.expect("intersection must refuse")).contains(reason));
        } else {
            for _ in 0..8 {
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.fits(grant));
                    Ok(())
                });
            }
            job.as_mut().unwrap().cancel();
            run(&mut caller, stock(), &mut born, &mut free, |cx| {
                assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert_eq!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().len())).unwrap(), 2);
        if let Some(samples) = row["samples"].as_array() {
            let output = run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
            let Value::Dictionary(list) = output.get("wires").unwrap() else { panic!("expected a wires list"); };
            assert!(list.get("1").is_none());
            let curve = Dictionary::new().insert("curve", list.get("0").unwrap().clone());
            let handle = read_geometry(&curve, "curve").unwrap();
            for sample in samples {
                let input = curve.clone().insert("parameter", number(sample[0].as_f64().unwrap()));
                let point = run_live_query(&registry, "brep.eval.curvePoint", input, &mut caller, &mut born, &mut free);
                assert!(read_xyz(&point, "point").unwrap().into_iter().zip(fixture_xyz(&sample[1]))
                    .all(|(actual,expected)| (actual-expected).abs() < tolerance));
            }
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.edge_positions.len(), 6);
            assert_eq!(mesh.component_references["edge"].len(), 1);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_surface_intersection_publishes_axial_circles_and_preserves_tangency_fault() {
    use semio_framework_3d_brep_engine::{ShapeRoot,ShapeValue};
    use semio_framework_3d_brep_representation::topology::Body;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3,matrix::Frame3};
    use semio_s_spatial_kernel_semio_session::{geometry_dict,map_kernel_error,point_dictionary,read_geometry,read_xyz};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).unwrap();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    for (index,row) in fixture["retainedAxialIntersection"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 98480 + index as u64, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0,0);
        let plane = ["origin","normal"].into_iter().fold(Dictionary::new(), |dict,port| {
            dict.insert(port,Value::Dictionary(point_dictionary(fixture_xyz(&row["plane"][port]))))
        });
        let plane = run_live_query(&registry,"brep.surf.plane",plane,&mut caller,&mut born,&mut free);
        let axial = &row["axial"];
        let frame = Frame3::from_normal(Pnt3::from_array(fixture_xyz(&axial["origin"])),Vec3::from_array(fixture_xyz(&axial["axis"]))).unwrap();
        let radius = axial["radius"].as_f64().unwrap();
        let surface = if axial["kind"] == "sphere" { Surface::Sphere { frame,radius } } else { Surface::Cylinder { frame,radius } };
        let axial = session.with_kernel(|kernel| {
            let mut body = Body::new();
            let label = body.new_label();
            let root = ShapeRoot::Surface { surface,label };
            let handle = kernel.import_shape(&ShapeValue { root,body }).map_err(|error| map_kernel_error(&error))?;
            geometry_dict(kernel,&handle)
        }).unwrap();
        let a = plane.get("surface").unwrap().clone();
        let b = Value::Dictionary(axial);
        let (a,b) = if row["swap"].as_bool() == Some(true) { (b,a) } else { (a,b) };
        let input = Dictionary::new().insert("a",a).insert("b",b).insert("tolerance",number(tolerance));
        let operator = "brep.intersect.surfaceSurface";
        if let Some(reason) = row["fault"].as_str() {
            let mut job = Some(admit_live_job(&registry,operator,input));
            let mut failure = None;
            for _ in 0..16 {
                run(&mut caller,Grant { maximum_items:1,maximum_copy_bytes:128,..stock() },&mut born,&mut free,|cx| {
                    match job.as_mut().unwrap().step(cx) {
                        Ok((neural_engine::OperatorJobStep::Working(_), _)) => {}
                        Ok(_) => panic!("expected surface intersection refusal"),
                        Err(error) => failure=Some(error),
                    }
                    Ok(())
                });
                if failure.is_some() { break; }
            }
            assert!(format!("{:?}",failure.expect("axial pair must refuse")).contains(reason));
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(session.with_kernel_read(|kernel|Ok(kernel.live_handles().len())).unwrap(),2);
        } else {
            let output=run_live_query(&registry,operator,input,&mut caller,&mut born,&mut free);
            let Value::Dictionary(list)=output.get("wires").unwrap() else{panic!("expected wires list")};
            assert!(list.get("1").is_none());
            let curve=Dictionary::new().insert("curve",list.get("0").unwrap().clone());
            let handle=read_geometry(&curve,"curve").unwrap();
            let center=Pnt3::from_array(fixture_xyz(&row["center"]));
            let radius=row["radius"].as_f64().unwrap();
            let normal=Vec3::from_array(fixture_xyz(&row["plane"]["normal"])).normalized().unwrap();
            for parameter in [0.,0.6,2.,4.25,6.] {
                let sample=run_live_query(&registry,"brep.eval.curvePoint",curve.clone().insert("parameter",number(parameter)),&mut caller,&mut born,&mut free);
                let offset=Pnt3::from_array(read_xyz(&sample,"point").unwrap())-center;
                assert!(offset.dot(normal).abs()<tolerance);
                assert!((offset.norm()-radius).abs()<tolerance);
            }
            let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
            mesh.validate_component_references().unwrap();
            assert!(mesh.edge_positions.len()>48);
            assert_eq!(mesh.component_references["edge"].len(),1);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_conic_plane_contacts_publish_exact_tangent_or_secant_and_retire_draft() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).unwrap();
    let tolerance=corpus["tolerance"].as_f64().unwrap();
    for (i,row) in corpus["retainedConicPlane"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98701+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,32768);
        let (mut born,mut free)=(0,0);
        let point=|raw|Value::Dictionary(point_dictionary(fixture_xyz(raw)));
        let vector=|raw|Value::Dictionary(vector_dictionary(fixture_xyz(raw)));
        let a=row["curve"]["radii"][0].as_f64().unwrap();
        let b=row["curve"]["radii"][1].as_f64().unwrap();
        let base=Dictionary::new().insert("center",point(&row["curve"]["center"])).insert("normal",vector(&row["curve"]["normal"]));
        let (constructor,parameters)=if a==b{("brep.curve.circle",base.insert("radius",number(a)))}
            else{("brep.curve.ellipse",base.insert("semiMajor",number(a)).insert("semiMinor",number(b)))};
        let curve=retained_boolean_result(&registry,constructor,parameters,&mut caller,&mut born,&mut free);
        let plane=Dictionary::new().insert("origin",point(&row["plane"]["origin"])).insert("normal",vector(&row["plane"]["normal"]));
        let plane=retained_boolean_result(&registry,"brep.surf.plane",plane,&mut caller,&mut born,&mut free);
        let input=Dictionary::new().insert("curve",curve.get("curve").unwrap().clone())
            .insert("surface",plane.get("surface").unwrap().clone()).insert("tolerance",number(tolerance));
        let operator="brep.intersect.curveSurface";
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.curves3.len()])
        }).unwrap();
        let before=counts();
        if let Some(reason)=row["fault"].as_str(){
            let mut invalid=Some(admit_live_job(&registry,operator,input));
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let result=invalid.as_mut().unwrap().step(cx);
                assert!(matches!(result,Err(error) if format!("{error:?}").contains(reason)));
                Ok(())
            });
            close_live_job(&mut invalid,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
            caller.retire(stock());
            continue;
        }
        let points=row["points"].as_array().unwrap();
        if points.len()==2 {
            let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..100 {
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    let (state,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(state,neural_engine::OperatorJobStep::Working(_)));assert!(receipt.fits(grant));
                    Ok(())
                });
                if counts()[1]>before[1]{break;}
            }
            assert!(counts()[1]>before[1]);assert_eq!(counts()[0],before[0]);
            draft.as_mut().unwrap().cancel();
            run(&mut caller,stock(),&mut born,&mut free,|cx|{
                assert!(matches!(draft.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut draft,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&result,"wire").unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        let xyz=if points.len()==2{&mesh.edge_positions}else{&mesh.positions};
        assert_eq!(xyz.len(),3*points.len());
        for (actual,expected) in xyz.chunks_exact(3).zip(points){
            assert!(actual.into_iter().zip(fixture_xyz(expected)).all(|(&x,y)|(f64::from(x)-y).abs()<1e-5));
        }
        assert_eq!(mesh.component_references["vertex"].len(),points.len());
        caller.retire(stock());
    }
}

#[test]
fn original_section_preserves_spherical_cap_and_band_chords() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/spherical-strip-section.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97486),Generation(17),stock(),||Some(1000),1500,65536);
    let (mut born,mut free)=(0,0);
    let mut evaluate=|operator:&str,input|run_live_query(&registry,operator,input,&mut caller,&mut born,&mut free);
    let sphere=evaluate("brep.prim3d.sphere",Dictionary::new().insert("radius",number(fixture["sphereRadius"].as_f64().unwrap())));
    let original=sphere.get("solid").unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let [low,high]:[f64;2]=serde_json::from_value(row["axial"].clone()).unwrap();
        let clip=evaluate("brep.prim3d.box",Dictionary::new().insert("width",number(8.)).insert("depth",number(8.)).insert("height",number(high-low)));
        let clip=evaluate("brep.xform.translate",Dictionary::new().insert("geometry",clip.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary([-4.,-4.,low]))));
        let material=evaluate("brep.bool.intersect",Dictionary::new().insert("a",original.clone()).insert("b",clip.get("geometryOut").unwrap().clone()));
        let normal=fixture_xyz(&row["normal"]);
        for axis in [normal,normal.map(|value|-value)]{
            let input=Dictionary::new().insert("solid",material.get("solid").unwrap().clone())
                .insert("planeOrigin",Value::Dictionary(point_dictionary(fixture_xyz(&row["plane"]))))
                .insert("planeNormal",Value::Dictionary(vector_dictionary(axis)));
            let result=evaluate("brep.intersect.section",input);
            let faces=result.get("faces").and_then(Value::as_dictionary).unwrap();
            assert!(faces.get("1").is_none());
            let area=evaluate("brep.measure.area",Dictionary::new().insert("geometry",faces.get("0").unwrap().clone()));
            let area=read_channel_number(&area,"area").unwrap();
            assert!((area-row["area"].as_f64().unwrap()).abs()<1e-6,"{}: {area}",row["name"]);
        }
    }
    caller.retire_exact();
}

#[test]
fn original_split_publishes_both_signed_spherical_halves() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/spherical-plane-split.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(97487),Generation(17),stock(),||Some(1000),1500,65536);
    let (mut born,mut free)=(0,0);
    let mut evaluate=|operator:&str,input|run_live_query(&registry,operator,input,&mut caller,&mut born,&mut free);
    let sphere=evaluate("brep.prim3d.sphere",Dictionary::new().insert("radius",number(fixture["radius"].as_f64().unwrap())));
    for row in fixture["cases"].as_array().unwrap(){
        let mut stock=sphere.get("solid").unwrap().clone();
        if let Some(span)=row["span"].as_array(){
            let low=span[0].as_f64().unwrap();
            let high=span[1].as_f64().unwrap();
            let box_shape=evaluate("brep.prim3d.box",Dictionary::new().insert("width",number(8.)).insert("depth",number(8.)).insert("height",number(high-low)));
            let tool=evaluate("brep.xform.translate",Dictionary::new().insert("geometry",box_shape.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary([-4.,-4.,low]))));
            stock=evaluate("brep.bool.intersect",Dictionary::new().insert("a",stock).insert("b",tool.get("geometryOut").unwrap().clone()))
                .get("solid").unwrap().clone();
        }
        let normal=fixture_xyz(&row["normal"]);
        for sign in [1.,-1.]{
            let halves=evaluate("brep.intersect.split",Dictionary::new().insert("solid",stock.clone())
                .insert("planeOrigin",Value::Dictionary(point_dictionary(fixture_xyz(&row["origin"]))))
                .insert("planeNormal",Value::Dictionary(vector_dictionary(normal.map(|value|value*sign)))));
            for (index,channel) in ["positive","negative"].into_iter().enumerate(){
                let input=Dictionary::new().insert("geometry",halves.get(channel).unwrap().clone());
                let result=evaluate("brep.measure.volume",input);
                let actual=read_channel_number(&result,"volume").unwrap();
                let expected=row["volumes"][index^usize::from(sign<0.)].as_f64().unwrap();
                assert!((actual-expected).abs()<1e-9,"{} {channel}: {actual}",row["name"]);
            }
        }
    }
    caller.retire_exact();
}

#[test]
fn original_queries_measure_spherical_clearance_and_nearest_point_on_orthogonal_material() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_xyz, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solid-queries.json")).unwrap();
    let shapes: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-booleans.json")).unwrap();
    for (index, row) in fixture["orthogonalSphereDistance"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(99430 + index as u64), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let mut evaluate = |operator: &str, input| run_live_query(&registry, operator, input, &mut caller, &mut born, &mut free);
        let vector = |value| Value::Dictionary(vector_dictionary(value));
        let translate = |geometry, offset| Dictionary::new().insert("geometry", geometry).insert("offset", vector(offset));
        let place = |mut geometry, evaluate: &mut dyn FnMut(&str, Dictionary) -> Dictionary| {
            if let Some(placement) = row.get("placement") {
                let input = Dictionary::new().insert("geometry", geometry).insert("axis", vector([0.0, 0.0, 1.0]))
                    .insert("angle", number(placement["angle"].as_f64().unwrap()));
                geometry = evaluate("brep.xform.rotate", input).get("geometryOut").unwrap().clone();
                geometry = evaluate("brep.xform.translate", translate(geometry, fixture_xyz(&placement["offset"])))
                    .get("geometryOut").unwrap().clone();
            }
            geometry
        };
        let box_geometry = |spec: &serde_json::Value, evaluate: &mut dyn FnMut(&str, Dictionary) -> Dictionary| {
            let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&spec["size"]))
                .fold(Dictionary::new(), |value, (key, dimension)| value.insert(key, number(dimension)));
            let solid = evaluate("brep.prim3d.box", input).get("solid").unwrap().clone();
            evaluate("brep.xform.translate", translate(solid, fixture_xyz(&spec["origin"])))
                .get("geometryOut").unwrap().clone()
        };
        let solid = if let Some(stock) = row.get("stock") {
            let canonical = box_geometry(stock, &mut evaluate);
            let placed = place(canonical, &mut evaluate);
            let input = Dictionary::new().insert("geometry", placed)
                .insert("thickness", number(stock["thickness"].as_f64().unwrap()))
                .insert("openFaces", Value::Dictionary(Dictionary::with_schema("list")));
            evaluate("brep.solid.shell", input).get("solid").unwrap().clone()
        } else {
            let source = &shapes[row["source"].as_str().unwrap()];
            let [a, b] = ["a", "b"].map(|key| box_geometry(&source[key], &mut evaluate));
            let input = Dictionary::new().insert("a", a).insert("b", b);
            evaluate(row["operator"].as_str().unwrap(), input).get("solid").unwrap().clone()
        };
        for probe in row["probes"].as_array().unwrap() {
            let input = Dictionary::new().insert("radius", number(probe["radius"].as_f64().unwrap()));
            let sphere = evaluate("brep.prim3d.sphere", input).get("solid").unwrap().clone();
            let sphere = evaluate("brep.xform.translate", translate(sphere, fixture_xyz(&probe["center"])))
                .get("geometryOut").unwrap().clone();
            let sphere = place(sphere, &mut evaluate);
            let distances = [[sphere.clone(), solid.clone()], [solid.clone(), sphere]].map(|[a,b]| {
                let input = Dictionary::new().insert("a", a).insert("b", b);
                let outcome = evaluate(fixture["distance"].as_str().unwrap(), input);
                read_channel_number(&outcome, "distance").unwrap()
            });
            let expected = probe["expected"].as_f64().unwrap();
            assert!((distances[0]-expected).abs() < 1e-10, "{} {}: {distances:?}", row["name"], probe["center"]);
            assert_eq!(distances[0], distances[1]);
        }
        let world = |local: [f64; 3]| {
            let Some(placement) = row.get("placement") else { return local; };
            let angle = placement["angle"].as_f64().unwrap();
            let (sin, cos) = angle.sin_cos();
            let [x, y, z] = local;
            let [dx, dy, dz] = fixture_xyz(&placement["offset"]);
            [dx+x*cos-y*sin, dy+x*sin+y*cos, dz+z]
        };
        for pair in row["closestPoints"].as_array().unwrap() {
            let input = Dictionary::new().insert("geometry", solid.clone())
                .insert("point", Value::Dictionary(point_dictionary(world(fixture_xyz(&pair[0])))));
            let result = evaluate("brep.measure.closestPoint", input);
            let actual = read_xyz(&result, "pointOut").unwrap();
            let expected = world(fixture_xyz(&pair[1]));
            assert!(actual.into_iter().zip(expected).all(|(a,b)| (a-b).abs() < 1e-10),
                "{} {}: {actual:?} != {expected:?}", row["name"], pair[0]);
        }
        caller.retire(stock());
    }
}


#[test]
fn original_bounding_box_conserves_annular_sectors_and_cylinder_shoulders() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-bounding-box.json")).unwrap();
    for (index,row) in fixture["annular"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = GeometryWorkIdentity { operation: 99540 + index as u64, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let mut evaluate = |op: &str, input| run_live_query(&registry, op, input, &mut caller, &mut born, &mut free);
        let point = |axes| Value::Dictionary(point_dictionary(axes));
        let vector = |axes| Value::Dictionary(vector_dictionary(axes));
        let operator = row["operator"].as_str().unwrap();
        let geometry = if operator == "brep.sweep.revolve" {
            let profile = evaluate("brep.surf.planarFace", polygon_fixture_input(&serde_json::json!({"points":&row["points"]})));
            let input = Dictionary::new().insert("face", profile.get("face").unwrap().clone())
                .insert("axisOrigin", point(fixture_xyz(&row["origin"]))).insert("axisDirection", vector([0.0,0.0,1.0]))
                .insert("angle", number(row["angle"].as_f64().unwrap()));
            evaluate(operator,input).get("solid").unwrap().clone()
        } else {
            let [a,b] = ["a","b"].map(|key| {
                let spec=&row[key];
                let input=Dictionary::new().insert("radius",number(spec["radius"].as_f64().unwrap()))
                    .insert("height",number(spec["height"].as_f64().unwrap()));
                let mut geometry=evaluate("brep.prim3d.cylinder",input).get("solid").unwrap().clone();
                let input=Dictionary::new().insert("geometry",geometry).insert("offset",vector(fixture_xyz(&spec["origin"])));
                geometry=evaluate("brep.xform.translate",input).get("geometryOut").unwrap().clone();
                if let Some(angle)=row["rotation"].as_f64() {
                    let input=Dictionary::new().insert("geometry",geometry).insert("axis",vector([0.0,1.0,0.0])).insert("angle",number(angle));
                    geometry=evaluate("brep.xform.rotate",input).get("geometryOut").unwrap().clone();
                    let input=Dictionary::new().insert("geometry",geometry).insert("offset",vector(fixture_xyz(&row["offset"])));
                    geometry=evaluate("brep.xform.translate",input).get("geometryOut").unwrap().clone();
                }
                geometry
            });
            evaluate(operator,Dictionary::new().insert("a",a).insert("b",b)).get("solid").unwrap().clone()
        };
        let output=evaluate(fixture["operator"].as_str().unwrap(),Dictionary::new().insert("geometry",geometry));
        let handle=read_geometry(&output,"box").unwrap();
        let preview=run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
        let actual=preview.aabb();
        for (extent, expected) in [(actual.0,&row["bounds"][0]),(actual.1,&row["bounds"][1])] {
            assert!(extent.into_iter().zip(fixture_xyz(expected)).all(|(a,b)| (f64::from(a)-b).abs() < 1e-6), "{}", row["name"]);
        }
        assert_eq!(preview.component_references["face"].len(),6);
        caller.retire(stock());
    }
}
