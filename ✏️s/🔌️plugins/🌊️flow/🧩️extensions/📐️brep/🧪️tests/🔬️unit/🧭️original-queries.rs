//! 🧭️ Original BRep queries, topology component aliasing and selected previews.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn original_obj_import_publishes_shell_or_solid_after_funded_topology() {
    original_mesh_import("brep.io.importObj", "obj", include_str!("🔣️original-obj-import.json"), 97820);
}

#[test]
fn original_stl_import_publishes_binary_and_ascii_after_funded_topology() {
    original_mesh_import("brep.io.importStl", "base64", include_str!("🔣️original-stl-import.json"), 97822);
}

#[test]
fn original_obj_and_stl_export_bound_the_source_through_registered_roundtrip() {
    use semio_s_spatial_kernel_semio_session::{read_geometry, read_text, read_channel_number};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-mesh-export.json")).unwrap();
    let facts = &fixture["closedBox"];
    let [width, depth, height] = fixture_xyz(&facts["size"]);
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97823), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let primitive = run_live_query(&registry, "brep.prim3d.box",
        Dictionary::new().insert("width", number(width)).insert("depth", number(depth)).insert("height", number(height)),
        &mut caller, &mut born, &mut free);
    let source = primitive.get("solid").unwrap();
    let source_handle = read_geometry(&primitive, "solid").unwrap();
    let export_input = |geometry: &Value| Dictionary::new().insert("geometry", geometry.clone()).insert("deflection", number(0.1));
    let handles = || session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap();
    let mut planar = None;
    for (export, import, channel) in [("brep.io.exportObj", "brep.io.importObj", "obj"), ("brep.io.exportStl", "brep.io.importStl", "stl")] {
        let before = handles();
        let encoded = run_live_query(&registry, export, export_input(source), &mut caller, &mut born, &mut free);
        assert_eq!(handles(), before);
        let text = read_text(&encoded, channel).unwrap();
        if channel == "obj" { assert_eq!(text.lines().filter(|line|line.starts_with("f ")).count(), facts["triangles"].as_u64().unwrap() as usize); }
        else { assert_eq!(semio_framework_value::base64_standard_decode(&text).unwrap().len(), 84 + 50 * facts["triangles"].as_u64().unwrap() as usize); }
        let decoded = run_live_query(&registry, import, Dictionary::new().insert("data", encoded.get(channel).unwrap().clone())
            .insert("tolerance", number(1e-6)), &mut caller, &mut born, &mut free);
        let shape = decoded.get("geometry").unwrap();
        for (method, output, key) in [("brep.measure.volume", "volume", "volume"), ("brep.measure.area", "area", "area")] {
            let measured = run_live_query(&registry, method, Dictionary::new().insert("geometry", shape.clone()), &mut caller, &mut born, &mut free);
            assert!((read_channel_number(&measured, output).unwrap() - facts[key].as_f64().unwrap()).abs() < 1e-9);
        }
        planar = Some(shape.clone());
    }
    let before = handles();
    let encoded = run_live_query(&registry, "brep.io.exportObj", export_input(&planar.unwrap()), &mut caller, &mut born, &mut free);
    assert_eq!(read_text(&encoded, "obj").unwrap().lines().filter(|line|line.starts_with("f ")).count(), facts["triangles"].as_u64().unwrap() as usize);
    assert_eq!(handles(), before);

    let mut job = Some(admit_live_job(&registry, "brep.io.exportStl", export_input(source)));
    let mut text_allocated = false;
    for _ in 0..10000 {
        let job = job.as_mut().unwrap();
        let encoding = job.progress().phase == "mesh-export-encoding";
        let copy = job.next_step_copy_byte_demand().unwrap().max(128);
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: copy, ..stock() }, &mut born, &mut free, |cx| {
            let (step, receipt) = job.step(cx).unwrap();
            assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
            text_allocated = encoding && receipt.retained_capacity_bytes != 0;
            Ok(())
        });
        if text_allocated { break; }
    }
    assert!(text_allocated);
    job.as_mut().unwrap().cancel();
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert_eq!(handles(), before);
    assert!(session.with_kernel_read(|kernel|Ok(kernel.kind_sync(&source_handle).is_ok())).unwrap());
    caller.retire_exact();
}

fn original_mesh_import(operator: &str, field: &str, source: &str, operation: u64) {
    use semio_framework_3d_brep_engine::ShapeRoot;
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, read_geometry};

    let fixtures: serde_json::Value = serde_json::from_str(source).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(operation), Generation(17), stock(), || Some(1000), 1500, 32768);
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let (mut born, mut free) = (0, 0);
    let input = |row: &serde_json::Value| {
        let data = semio_s_spatial_kernel_semio_session::text_dictionary(row[field].as_str().unwrap().to_owned());
        Dictionary::new().insert("data", Value::Dictionary(data)).insert("tolerance", number(1e-6))
    };
    let sizes = || session.with_kernel_read(|kernel| {
        let body = kernel.tessellation_body();
        Ok([kernel.live_handles().len(), body.vertices.len(), body.edges.len(), body.faces.len(), body.solids.len(), body.shells.len()])
    }).unwrap();
    for row in fixtures["cases"].as_array().unwrap() {
        let before = sizes();
        let output = run_live_query(&registry, operator, input(row), &mut caller, &mut born, &mut free);
        let handle = read_geometry(&output, "geometry").unwrap();
        let shape = session.with_kernel_read(|kernel| kernel.export_shape(&handle).map_err(|error| map_kernel_error(&error))).unwrap();
        let kind = match shape.root { ShapeRoot::Solid(_) => "solid", ShapeRoot::Shell(_) => "shell", _ => panic!("mesh has a native shell or solid root") };
        assert_eq!(kind, row["kind"].as_str().unwrap());
        assert_eq!(sizes()[0], before[0] + 1);
        let body = &shape.body;
        for (actual, key) in [(body.vertices.len(), "vertices"), (body.edges.len(), "edges"), (body.faces.len(), "faces")] {
            assert_eq!(actual, row[key].as_u64().unwrap() as usize);
        }
        let (area, volume) = body.faces.iter().fold((0.0, 0.0), |(area, volume), (_, face)| {
            let first = body.loops.get(face.outer.unwrap()).unwrap().first;
            let second = body.coedges.get(first).unwrap().next;
            let third = body.coedges.get(second).unwrap().next;
            let [a, b, c] = [first, second, third].map(|id| body.vertices.get(body.coedge_endpoints(id).unwrap().0).unwrap().position);
            (area + 0.5 * (b - a).cross(c - a).norm(), volume + a.to_vec().dot(b.to_vec().cross(c.to_vec())) / 6.0)
        });
        assert!((area - row["area"].as_f64().unwrap()).abs() < 1e-10);
        assert!((volume - row["volume"].as_f64().unwrap()).abs() < 1e-10);
    }

    let before = sizes();
    let mut job = Some(admit_live_job(&registry, operator, input(&fixtures["cases"][0])));
    for _ in 0..2000 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Working(_)));
            Ok(())
        });
        if sizes()[3] > before[3] { break; }
    }
    assert!(sizes()[3] > before[3]);
    assert_eq!(sizes()[0], before[0]);
    job.as_mut().unwrap().cancel();
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert_eq!(sizes(), before);

    for row in fixtures["invalid"].as_array().unwrap() {
        let mut job = Some(admit_live_job(&registry, operator, input(row)));
        let mut refusal = None;
        for _ in 0..2000 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                match job.as_mut().unwrap().step(cx) {
                    Ok((neural_engine::OperatorJobStep::Working(_), _)) => {}
                    Ok(_) => panic!("an invalid mesh cannot publish a root"),
                    Err(error) => refusal = Some(error),
                }
                Ok(())
            });
            if refusal.is_some() { break; }
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        assert!(format!("{:?}", refusal.expect("invalid original mesh refuses")).contains("InvalidValue"));
        assert_eq!(sizes(), before);
    }
    caller.retire_exact();
}

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
    use semio_framework_3d_brep_curve::Curve3;
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
fn original_closest_point_traverses_the_authored_planar_face_with_holes() {
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_engine::{ShapeRoot, ShapeValue};
    use semio_framework_3d_brep_representation::topology::{Body, Edge, Face, Vertex, make_loop};
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3, Vec3, matrix::Frame3};
    use semio_s_spatial_kernel_semio_session::{geometry_dict, map_kernel_error, point_dictionary, read_xyz};

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
    let expected_turns = 2 + 3 + fixture["outer"].as_array().unwrap().len() +
        fixture["holes"].as_array().unwrap().iter().map(|ring| ring.as_array().unwrap().len()).sum::<usize>();
    for row in fixture["cases"].as_array().unwrap() {
        let input = Dictionary::new()
            .insert("geometry", Value::Dictionary(handle.clone()))
            .insert("point", Value::Dictionary(point_dictionary(fixture_xyz(&row["target"]))));
        let mut job = Some(admit_live_job(&registry, "brep.measure.closestPoint", input));
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
        let actual = read_xyz(&result, "pointOut").unwrap();
        let expected = fixture_xyz(&row["point"]);
        assert!(actual.into_iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-10), "{} {:?}", row["name"], actual);
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
    let input = Dictionary::new().insert("geometry", Value::Dictionary(curved_handle))
        .insert("point", Value::Dictionary(point_dictionary([14.0, 20.0, 30.0])));
    let mut job = Some(admit_live_job(&registry, "brep.measure.closestPoint", input));
    let mut error = None;
    for _ in 0..8 {
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            match job.as_mut().unwrap().step(cx) {
                Ok((neural_engine::OperatorJobStep::Working(_), _)) => {}
                Ok(_) => panic!("mixed face boundary must refuse original closest point"),
                Err(refusal) => error = Some(refusal),
            }
            Ok(())
        });
        if error.is_some() { break; }
    }
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert!(format!("{:?}", error.expect("original closest point refuses circular trim in mixed boundary")).contains("UnsupportedOwner"));

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
