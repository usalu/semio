//! 📂️ Funded BRep exchange, topology publication and exact round trips.


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
fn original_step_import_certifies_authored_closed_shells_and_retires_an_index() {
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, read_channel_number, read_geometry, text_dictionary};

    let fixtures: serde_json::Value = serde_json::from_str(include_str!("🔣️original-step-import.json")).unwrap();
    let authored = [include_str!("../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🧫️fixtures/📐️booleans-non-manifold-corner-touch-boxes/🅰️operand-a.step"),include_str!("../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧪️tests/🧫️fixtures/📐️booleans-non-manifold-corner-touch-boxes/🅱️operand-b.step")];
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97824), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let input = |source: &str| Dictionary::new().insert("data", Value::Dictionary(text_dictionary(source.to_owned())));
    let handles = || session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap();
    for (source, row) in authored.into_iter().zip(fixtures["cases"].as_array().unwrap()) {
        let before = handles();
        let mut job = Some(admit_live_job(&registry, "brep.io.importStep", input(source)));
        let mut output = None;
        for _ in 0..10000 {
            let copy = job.as_ref().unwrap().next_step_copy_byte_demand().unwrap().max(128);
            run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: copy, ..stock() }, &mut born, &mut free, |cx| {
                let (step, _) = job.as_mut().unwrap().step(cx).unwrap();
                if let neural_engine::OperatorJobStep::Done(value) = step { output = Some(value); }
                Ok(())
            });
            if output.is_some() { break; }
            assert_eq!(handles(), before);
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let output = neural_engine::ColdOwner::new(output.expect("certified STEP solid"));
        let handle = read_geometry(&output, "geometry").unwrap();
        let shape = session.with_kernel_read(|kernel| kernel.export_shape(&handle).map_err(|error| map_kernel_error(&error))).unwrap();
        assert!(matches!(shape.root, semio_framework_3d_brep_engine::ShapeRoot::Solid(_)));
        assert_eq!(handles().len(), before.len() + 1);
        let body = &shape.body;
        let origin = fixture_xyz(&row["origin"]);
        let size = fixture_xyz(&row["size"]);
        for (_, vertex) in body.vertices.iter() {
            let at = vertex.position.to_array();
            assert!((0..3).all(|axis| at[axis] == origin[axis] || at[axis] == origin[axis] + size[axis]));
        }
        for (actual, key) in [(body.vertices.len(), "vertices"), (body.edges.len(), "edges"), (body.faces.len(), "faces")] {
            assert_eq!(actual, row[key].as_u64().unwrap() as usize);
        }
        let solid = output.get("geometry").unwrap();
        for (method, channel) in [("brep.measure.volume", "volume"), ("brep.measure.area", "area")] {
            let value = run_live_query(&registry, method, Dictionary::new().insert("geometry", solid.clone()), &mut caller, &mut born, &mut free);
            assert_eq!(read_channel_number(&value, channel).unwrap(), row[channel].as_f64().unwrap());
        }
    }
    let before = handles();
    let mut job = Some(admit_live_job(&registry, "brep.io.importStep", input(authored[1])));
    let mut allocated = false;
    for _ in 0..10000 {
        let copy = job.as_ref().unwrap().next_step_copy_byte_demand().unwrap().max(128);
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: copy, ..stock() }, &mut born, &mut free, |cx| {
            let active = job.as_mut().unwrap();
            let decoding = active.progress().phase == "step-solid-certification";
            let (step, receipt) = active.step(cx).unwrap();
            assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
            allocated = decoding && receipt.retained_capacity_bytes > 0;
            Ok(())
        });
        if allocated { break; }
    }
    assert!(allocated);
    job.as_mut().unwrap().cancel();
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert_eq!(handles(), before);
    caller.retire_exact();
}

#[test]
fn original_step_exchange_preserves_authored_full_cylinders() {
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, read_channel_number, read_geometry, read_text, text_dictionary};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-step-cylinders.json")).unwrap();
    let sources = [include_str!("../../../../../🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧪️tests/🧫️fixtures/❌️fuse-cylinder-cross/🅰️operand-a.step"), include_str!("../../../../../🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🧪️tests/🧫️fixtures/🎯️cut-tangent-cylinder-exact/🅱️operand-b.step")];
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97826), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let handles = || session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap();
    for (source, row) in sources.into_iter().zip(fixture["cases"].as_array().unwrap()) {
        let before = handles();
        let input = Dictionary::new().insert("data", Value::Dictionary(text_dictionary(source.to_owned())));
        let mut job = Some(admit_live_job(&registry, "brep.io.importStep", input));
        let mut output = None;
        for _ in 0..10000 {
            let copy = job.as_ref().unwrap().next_step_copy_byte_demand().unwrap().max(128);
            run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: copy, ..stock() }, &mut born, &mut free, |cx| {
                if let neural_engine::OperatorJobStep::Done(value) = job.as_mut().unwrap().step(cx).unwrap().0 { output = Some(value); }
                Ok(())
            });
            if output.is_some() { break; }
            assert_eq!(handles(), before);
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let output = neural_engine::ColdOwner::new(output.expect("authored STEP cylinder"));
        let handle = read_geometry(&output, "geometry").unwrap();
        let shape = session.with_kernel_read(|kernel| kernel.export_shape(&handle).map_err(|error| map_kernel_error(&error))).unwrap();
        let (vertices,edges,faces) = (shape.body.vertices.len(),shape.body.edges.len(),shape.body.faces.len());
        assert_eq!((vertices,edges,faces), (2,3,3));
        assert_eq!(handles().len(),before.len()+1);
        let source_handle = handles();
        let export_input = Dictionary::new().insert("geometry",output.get("geometry").unwrap().clone());
        let mut export = Some(admit_live_job(&registry,"brep.io.exportStep",export_input));
        let mut text = None;
        for _ in 0..10000 {
            let copy = export.as_ref().unwrap().next_step_copy_byte_demand().unwrap().max(128);
            run(&mut caller,Grant {maximum_items:1,maximum_copy_bytes:copy,..stock()},&mut born,&mut free,|cx| {
                if let neural_engine::OperatorJobStep::Done(value) = export.as_mut().unwrap().step(cx).unwrap().0 { text = Some(value); }
                Ok(())
            });
            if text.is_some() { break; }
        }
        close_live_job(&mut export,&mut caller,&mut born,&mut free);
        let text = neural_engine::ColdOwner::new(text.expect("authored STEP cylinder output"));
        let document = read_text(&text,"step").unwrap();
        for (entity,count) in [("CYLINDRICAL_SURFACE",1),("SEAM_CURVE",1),("SURFACE_CURVE",2),("PCURVE",6)] {
            let marker = format!(" = {entity}(");
            assert_eq!(document.lines().filter(|line|line.contains(&marker)).count(),count);
        }
        assert_eq!(handles(),source_handle);
        for (operation, channel) in [("brep.measure.volume","volume"),("brep.measure.area","area")] {
            let input = Dictionary::new().insert("geometry", output.get("geometry").unwrap().clone());
            let measured = run_live_query(&registry,operation,input,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&measured,channel).unwrap() - row[channel].as_f64().unwrap()).abs() < 1e-9);
        }
    }
    caller.retire_exact();
}

#[test]
fn original_stl_import_publishes_binary_and_ascii_after_funded_topology() {
    original_mesh_import("brep.io.importStl", "base64", include_str!("🔣️original-stl-import.json"), 97822);
}

#[test]
fn original_step_exchange_roundtrips_box_prism_and_orthogonal_topology_and_cancels_text() {
    use semio_s_spatial_kernel_semio_session::{map_kernel_error, point_dictionary, read_channel_number, read_geometry, read_text, vector_dictionary};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-step-export.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97825), Generation(17), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let box_row = &fixture["cases"][0];
    let prism_row = &fixture["cases"][1];
    let [width, depth, height] = fixture_xyz(&box_row["size"]);
    let primitive = run_live_query(&registry, "brep.prim3d.box",
        Dictionary::new().insert("width", number(width)).insert("depth", number(depth)).insert("height", number(height)),
        &mut caller, &mut born, &mut free);
    let moved = run_live_query(&registry, "brep.xform.translate",
        Dictionary::new().insert("geometry", primitive.get("solid").unwrap().clone())
            .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&box_row["origin"])))),
        &mut caller, &mut born, &mut free);
    let mut points = Dictionary::new().insert("$schema", Value::Atom(Atom::String("list".into())));
    for (index, point) in prism_row["ring"].as_array().unwrap().iter().enumerate() {
        points = points.insert(index.to_string(), Value::Dictionary(point_dictionary(fixture_xyz(point))));
    }
    let profile = run_live_query(&registry, "brep.surf.planarFace",
        Dictionary::new().insert("points", Value::Dictionary(points)), &mut caller, &mut born, &mut free);
    let prism = run_live_query(&registry, "brep.sweep.extrude",
        Dictionary::new().insert("face", profile.get("face").unwrap().clone())
            .insert("vector", Value::Dictionary(vector_dictionary(fixture_xyz(&prism_row["vector"])))),
        &mut caller, &mut born, &mut free);
    let orthogonal_row = &fixture["cases"][2];
    let mut box_at_origin = |dimensions: &serde_json::Value| {
        let [width, depth, height] = fixture_xyz(dimensions);
        run_live_query(&registry, "brep.prim3d.box",
            Dictionary::new().insert("width", number(width)).insert("depth", number(depth)).insert("height", number(height)),
            &mut caller, &mut born, &mut free)
    };
    let a = box_at_origin(&orthogonal_row["a"][1]);
    let b = box_at_origin(&orthogonal_row["b"][1]);
    let orthogonal = run_live_query(&registry, "brep.bool.fuse", Dictionary::new()
        .insert("a", a.get("solid").unwrap().clone()).insert("b", b.get("solid").unwrap().clone()),
        &mut caller, &mut born, &mut free);
    let export_input = |geometry: &Value| Dictionary::new().insert("geometry", geometry.clone());
    let handles = || session.with_kernel_read(|kernel| Ok(kernel.live_handles())).unwrap();
    let mut exchange = |operator, input| {
        let mut job = Some(admit_live_job(&registry, operator, input));
        let mut output = None;
        for _ in 0..10000 {
            let copy = job.as_ref().unwrap().next_step_copy_byte_demand().unwrap().max(128);
            run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: copy, ..stock() }, &mut born, &mut free, |cx| {
                if let neural_engine::OperatorJobStep::Done(value) = job.as_mut().unwrap().step(cx).unwrap().0 { output = Some(value); }
                Ok(())
            });
            if output.is_some() { break; }
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        neural_engine::ColdOwner::new(output.expect("funded STEP exchange"))
    };
    for (row, source) in [
        (box_row, moved.get("geometryOut").unwrap()),
        (prism_row, prism.get("solid").unwrap()),
        (orthogonal_row, orthogonal.get("solid").unwrap()),
    ] {
        let before = handles();
        let data = exchange("brep.io.exportStep", export_input(source));
        let authored = read_text(&data, "step").unwrap();
        for (entity, field) in [("VERTEX_POINT", "vertices"), ("EDGE_CURVE", "edges"), ("ADVANCED_FACE", "faces")] {
            let marker = format!(" = {entity}(");
            assert_eq!(authored.lines().filter(|line| line.contains(&marker)).count(), row[field].as_u64().unwrap() as usize);
        }
        assert_eq!(handles(), before);
        let imported = exchange("brep.io.importStep", Dictionary::new().insert("data", data.get("step").unwrap().clone()));
        let handle = read_geometry(&imported, "geometry").unwrap();
        let shape = session.with_kernel_read(|kernel| kernel.export_shape(&handle).map_err(|error| map_kernel_error(&error))).unwrap();
        assert_eq!(handles().len(), before.len() + 1);
        for (actual, key) in [(shape.body.vertices.len(), "vertices"), (shape.body.edges.len(), "edges"), (shape.body.faces.len(), "faces")] {
            assert_eq!(actual, row[key].as_u64().unwrap() as usize);
        }
        let geometry = imported.get("geometry").unwrap();
        let measured = exchange("brep.measure.volume", export_input(geometry));
        assert!((read_channel_number(&measured, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);
    }

    let before = handles();
    let mut job = Some(admit_live_job(&registry, "brep.io.exportStep", export_input(prism.get("solid").unwrap())));
    let mut allocated = false;
    for _ in 0..10000 {
        let copy = job.as_ref().unwrap().next_step_copy_byte_demand().unwrap().max(128);
        run(&mut caller, Grant { maximum_items: 1, maximum_copy_bytes: copy, ..stock() }, &mut born, &mut free, |cx| {
            let active = job.as_mut().unwrap();
            let sizing = active.progress().phase == "step-export-sizing";
            let (step, receipt) = active.step(cx).unwrap();
            assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
            allocated = sizing && receipt.retained_capacity_bytes != 0;
            Ok(())
        });
        if allocated { break; }
    }
    assert!(allocated);
    job.as_mut().unwrap().cancel();
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert_eq!(handles(), before);
    let source = read_geometry(&prism, "solid").unwrap();
    assert!(session.with_kernel_read(|kernel| Ok(kernel.kind_sync(&source).is_ok())).unwrap());
    caller.retire_exact();
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

