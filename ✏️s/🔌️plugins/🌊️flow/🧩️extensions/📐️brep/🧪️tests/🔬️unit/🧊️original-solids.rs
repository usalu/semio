//! 🧊️ Original BRep solid construction, features, exact sections and transformations.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn original_box_fillet_cancels_or_publishes_analytic_surfaces_and_closed_preview() {
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_geometry, read_text, vector_dictionary};
    let fixture: serde_json::Value=serde_json::from_str(include_str!("🔣️original-box-all-edge-fillet.json")).unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    for (index,row) in fixture["cases"].as_array().unwrap().iter().enumerate() {
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=semio_os_flow_geometry::GeometryWorkIdentity { operation: 98750+index as u64, generation: 17 };
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let dimensions=["width","depth","height"].into_iter().zip(fixture_xyz(&row["size"]))
            .fold(Dictionary::new(),|input,(axis,size)|input.insert(axis,number(size)));
        let raw=run_live_query(&registry,"brep.prim3d.box",dimensions,&mut caller,&mut born,&mut free);
        let place=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let solid=run_live_query(&registry,"brep.xform.translate",place,&mut caller,&mut born,&mut free);
        let input=|radius|Dictionary::new().insert("geometry",solid.get("geometryOut").unwrap().clone()).insert("radius",number(radius));
        let counts=||session.with_kernel_read(|kernel|{
            let b=kernel.representation();
            Ok([kernel.live_handles().len(),b.vertices.len(),b.edges.len(),b.faces.len(),b.solids.len()])
        }).unwrap();
        let surface_kinds=||session.with_kernel_read(|kernel|{
            let mut kinds=[0;3];
            for (_,surface) in kernel.representation().surfaces.iter(){
                match surface { Surface::Plane{..}=>kinds[0]+=1,Surface::Cylinder{..}=>kinds[1]+=1,Surface::Sphere{..}=>kinds[2]+=1,_=>{} }
            }
            Ok(kinds)
        }).unwrap();
        let old_kinds=surface_kinds();
        let before=counts();
        let radius=row["radius"].as_f64().unwrap();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let mut cancelled=Some(admit_live_job(&registry,operator,input(radius)));
        run(&mut caller,Grant{maximum_items:0,..stock()},&mut born,&mut free,|cx|{
            let (step,receipt)=cancelled.as_mut().unwrap().step(cx).unwrap();
            assert!(matches!(step,neural_engine::OperatorJobStep::Working(_)));
            assert_eq!(receipt,Progress::default());
            Ok(())
        });
        assert_eq!(counts(),before);
        for _ in 0..2000 {
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=cancelled.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            assert_eq!(counts()[0],before[0]);
            if counts()[3]>before[3]{break;}
        }
        assert!(counts()[3]>before[3]);
        cancelled.as_mut().unwrap().cancel();
        close_live_job(&mut cancelled,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let mut refused=Some(admit_live_job(&registry,operator,input(row["refused"][2].as_f64().unwrap())));
        run(&mut caller,stock(),&mut born,&mut free,|cx|{
            let error=refused.as_mut().unwrap().step(cx).err().expect("collapsed inset must be refused");
            assert!(format!("{error:?}").contains("all-edge fillet radius"));
            Ok(())
        });
        close_live_job(&mut refused,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let output=run_live_query(&registry,operator,input(radius),&mut caller,&mut born,&mut free);
        assert_eq!(counts(),[before[0]+1,before[1]+24,before[2]+48,before[3]+26,before[4]+1]);
        assert_eq!(surface_kinds(),[old_kinds[0]+6,old_kinds[1]+12,old_kinds[2]+8]);
        let handle=read_geometry(&output,"solid").unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        for (domain,key) in [("face","faceCount"),("edge","edgeCount"),("vertex","vertexCount")] {
            assert_eq!(mesh.component_references[domain].len(),fixture[key].as_u64().unwrap() as usize);
        }
        let (area,volume)=preview_area_volume(&mesh);
        for (operator,quantity,preview) in [("brep.measure.area","area",area),("brep.measure.volume","volume",volume)]{
            let expected=row[quantity].as_f64().unwrap();
            assert!((preview/expected-1.).abs()<0.04);
            let input=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
            let measured=run_live_query(&registry,operator,input,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&measured,quantity).unwrap()-expected).abs()<1e-9);
        }
        let export_input=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone()).insert("deflection",number(0.025));
        let obj=run_live_query(&registry,"brep.io.exportObj",export_input.clone(),&mut caller,&mut born,&mut free);
        assert_eq!(read_text(&obj,"obj").unwrap().lines().filter(|line|line.starts_with("f ")).count(),mesh.indices.len()/3);
        let stl=run_live_query(&registry,"brep.io.exportStl",export_input,&mut caller,&mut born,&mut free);
        let bytes=semio_framework_value::base64_standard_decode(&read_text(&stl,"stl").unwrap()).unwrap();
        assert_eq!(bytes.len(),84+50*(mesh.indices.len()/3));
        assert_eq!(counts()[0],before[0]+1);
        let step_input=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
        let step_fixture=&fixture["step"];
        let step=run_live_query(&registry,step_fixture["exportOperator"].as_str().unwrap(),step_input,&mut caller,&mut born,&mut free);
        let text=read_text(&step,"step").unwrap();
        for entity in step_fixture["entities"].as_array().unwrap(){
            let pattern=format!(" = {}(",entity[0].as_str().unwrap());
            assert_eq!(text.matches(pattern.as_str()).count(),entity[1].as_u64().unwrap() as usize);
        }
        assert_eq!(counts()[0],before[0]+1);
        let import=Dictionary::new().insert("data",step.get("step").unwrap().clone());
        let roundtrip=run_live_query(&registry,step_fixture["importOperator"].as_str().unwrap(),import,&mut caller,&mut born,&mut free);
        assert_eq!(counts()[0],before[0]+2);
        let query=Dictionary::new().insert("geometry",roundtrip.get("geometry").unwrap().clone());
        let volume=run_live_query(&registry,"brep.measure.volume",query,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-row["volume"].as_f64().unwrap()).abs()<1e-9);
        caller.retire(stock());
    }
}

#[test]
fn circle_live_registry_extrusion_covers_axis_reversal_preview_and_cancellation() {
    use semio_os_flow_geometry::{GeometryStep, GeometryWorkIdentity};

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

        use semio_os_flow_geometry::{GeometryStep, GeometryWorkIdentity};
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
    let identity=semio_os_flow_geometry::GeometryWorkIdentity{operation:97165,generation:17};
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
    use semio_os_flow_geometry::GeometryWorkIdentity;
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
    let input = Dictionary::new().insert("radius", number(2.0)).insert("height", number(3.0));
    let cylinder = retained_boolean_result(&registry, "brep.prim3d.cylinder", input, &mut caller, &mut born, &mut free);
    let unsupported = Dictionary::new().insert("geometry", cylinder.get("solid").unwrap().clone()).insert("distance", number(0.5));
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
    use semio_os_flow_geometry::GeometryWorkIdentity;
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
    use semio_os_flow_geometry::GeometryWorkIdentity;
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
fn original_two_profile_loft_proves_coaxial_or_translated_boundary_correspondence() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
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
    use semio_os_flow_geometry::GeometryWorkIdentity;
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
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_engine::GeometryHandle;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-box-section.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97177, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 131072);
    let (mut born, mut free) = (0, 0);
    let dimensions = fixture_xyz(&fixture["box"]["size"]);
    let input = ["width", "depth", "height"].into_iter().zip(dimensions).fold(Dictionary::new(), |input, (axis, size)| {
        input.insert(axis, number(size))
    });
    let original = retained_boolean_result(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free);
    let input = Dictionary::new().insert("geometry", original.get("solid").unwrap().clone())
        .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&fixture["box"]["origin"]))));
    let stock_box = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
    let cases = fixture["cases"].as_array().unwrap();
    let clipped_row = cases.iter().find(|row| row["clip"].is_object()).unwrap();
    let rotation = Dictionary::new().insert("geometry", original.get("solid").unwrap().clone())
        .insert("angle", number(clipped_row["rotation"].as_f64().unwrap().to_radians()))
        .insert("axis", Value::Dictionary(vector_dictionary([0., 0., 1.])));
    let rotated = retained_boolean_result(&registry, "brep.xform.rotate", rotation, &mut caller, &mut born, &mut free);
    let translation = Dictionary::new().insert("geometry", rotated.get("geometryOut").unwrap().clone())
        .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&fixture["box"]["origin"]))));
    let rotated = retained_boolean_result(&registry, "brep.xform.translate", translation, &mut caller, &mut born, &mut free);
    let plane_input = |solid: &Value, row: &serde_json::Value| Dictionary::new()
        .insert("solid", solid.clone())
        .insert("planeOrigin", Value::Dictionary(point_dictionary(fixture_xyz(&row["origin"]))))
        .insert("planeNormal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
    let split_input = plane_input(rotated.get("geometryOut").unwrap(), &clipped_row["clip"]);
    let clipped = retained_boolean_result(&registry, "brep.intersect.split", split_input, &mut caller, &mut born, &mut free);
    let section_input = |row: &serde_json::Value| {
        let source = if row["clip"].is_object() { clipped.get("positive") }
            else if row["rotation"].is_number() { rotated.get("geometryOut") }
            else { stock_box.get("geometryOut") };
        plane_input(source.unwrap(), row)
    };
    let operator = fixture["operator"].as_str().unwrap();

    for row in cases {
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
        assert_eq!(edges_scanned, row["clip"]["edges"].as_u64().unwrap_or(12));
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
        close_to(min.map(f64::from), &row["bounds"][0], 1e-6);
        close_to(max.map(f64::from), &row["bounds"][1], 1e-6);
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
    let mut cancelled = Some(admit_live_job(&registry, operator, section_input(clipped_row)));
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
    check_original_plane_split(include_str!("🔣️original-box-split.json"), 97178);
}

#[test]
fn original_cylinder_split_commits_two_exact_caps_and_retires_both_on_cancellation() {
    check_original_plane_split(include_str!("🔣️original-cylinder-split.json"), 97572);
}

#[test]
fn original_cone_split_commits_a_cone_and_frustum_around_one_exact_rim() {
    check_original_plane_split(include_str!("🔣️original-cone-split.json"), 97574);
}

fn check_original_plane_split(fixture: &str, operation: u64) {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3, Vec3};
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    let curved = fixture.get("cylinder").or_else(|| fixture.get("cone"));
    let geometry = if let Some(axial) = curved {
        let input = ["radius", "height"].into_iter().fold(Dictionary::new(), |input, key|
            input.insert(key, number(axial[key].as_f64().unwrap())));
        let operator = if fixture.get("cone").is_some() { "brep.prim3d.cone" } else { "brep.prim3d.cylinder" };
        let solid = retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
        let rotate = Dictionary::new().insert("geometry", solid.get("solid").unwrap().clone())
            .insert("axis", Value::Dictionary(vector_dictionary([0., 1., 0.])))
            .insert("angle", number(axial["angleY"].as_f64().unwrap()));
        let rotated = retained_boolean_result(&registry, "brep.xform.rotate", rotate, &mut caller, &mut born, &mut free);
        rotated.get("geometryOut").unwrap().clone()
    } else {
        let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&fixture["box"]["size"]))
            .fold(Dictionary::new(), |input, (axis, size)| input.insert(axis, number(size)));
        let solid = retained_boolean_result(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free);
        solid.get("solid").unwrap().clone()
    };
    let definition = curved.unwrap_or(&fixture["box"]);
    let input = Dictionary::new().insert("geometry", geometry)
        .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&definition["origin"]))));
    let placed = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
    let split_input = |row: &serde_json::Value| Dictionary::new()
        .insert("solid", placed.get("geometryOut").unwrap().clone())
        .insert("planeOrigin", Value::Dictionary(point_dictionary(fixture_xyz(&row["origin"]))))
        .insert("planeNormal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
    let operator = fixture["operator"].as_str().unwrap();
    let counts = || session.with_kernel_read(|kernel| Ok((kernel.live_handles().len(), kernel.representation().solids.iter().count()))).unwrap();
    let initial = counts();

    let one_item = Grant { maximum_items: 1, ..stock() };
    let mut cancelled = Some(admit_live_job(&registry, operator, split_input(&fixture["cases"][0])));
    let mut staged = 0;
    for _ in 0..2500 {
        let publishing = cancelled.as_ref().unwrap().progress().phase == "split-publication";
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
                    _ => panic!("original Split did not complete"),
                }
                Ok(())
            });
            if output.is_some() { break; }
            assert_eq!(counts().0, before.0);
        }
        assert_eq!(counts(), (before.0 + 2, before.1 + 2));
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let output = neural_engine::ColdOwner::new(output.expect("original Split did not finish"));
        let mut total_volume = 0.;
        for channel in ["positive", "negative"] {
            let expected = &row[channel];
            let value = output.get(channel).unwrap();
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, channel).unwrap();
            let preview = run_funded_preview(&session, identity, &handle.0, &mut caller, &mut born, &mut free);
            preview.validate_component_references().unwrap();
            if let Some(axial) = curved {
                let center = Pnt3::from_array(fixture_xyz(&row["sectionCenter"]));
                let normal = Vec3::from_array(fixture_xyz(&row["normal"]));
                let outward = normal * (if channel == "positive" { -1. } else { 1. } / normal.norm());
                let radius = row["sectionRadius"].as_f64().unwrap_or_else(|| axial["radius"].as_f64().unwrap());
                let expected_count = if expected["kind"] == "cone" { 2 } else { 3 };
                assert_eq!(preview.component_references["face"].len(), expected_count);
                session.with_kernel_read(|kernel| {
                    let body = kernel.representation();
                    let id = kernel.geometry_solid(&handle).unwrap();
                    let reach = body.reachable_from(&[EntityRef::Solid(id)]);
                    assert_eq!((reach.vertices.len(), reach.edges.len(), reach.faces.len()), (2, expected_count, expected_count));
                    if fixture.get("cone").is_some() {
                        let angle = axial["angleY"].as_f64().unwrap();
                        let height = axial["height"].as_f64().unwrap();
                        let apex = Pnt3::from_array(fixture_xyz(&axial["origin"])) + Vec3::new(angle.sin(), 0., angle.cos()) * height;
                        let authored_angle = axial["radius"].as_f64().unwrap().atan2(height);
                        let mut supports = reach.faces.iter().filter_map(|&id| {
                            let face = body.faces.get(id).unwrap();
                            let Surface::Cone { frame, half_angle } = body.surfaces.get(face.surface).unwrap() else { return None; };
                            Some((frame, *half_angle))
                        });
                        let (frame, angle) = supports.next().unwrap();
                        assert!(supports.next().is_none());
                        assert!(frame.origin.distance(apex) < 1e-12 && (angle - authored_angle).abs() < 1e-12);
                    }

                    let cap = reach.faces.iter().filter_map(|&id| {
                        let face = body.faces.get(id).unwrap();
                        let Surface::Plane { frame } = body.surfaces.get(face.surface).unwrap() else { return None; };
                        ((frame.origin - center).norm() < 1e-12).then_some((face, frame))
                    }).collect::<Vec<_>>();
                    let [(face, frame)] = cap.as_slice() else { panic!("one cut cap required"); };
                    assert!(!face.flipped && (frame.z - outward).norm() < 1e-12);
                    assert!(preview.component_references["face"].contains(&face.label.0.to_string()));
                    let rim = reach.edges.iter().copied().filter(|&id| {
                        let edge = body.edges.get(id).unwrap();
                        matches!(body.curves3.get(edge.curve).unwrap(), Curve3::Circle { frame, radius: r }
                            if (frame.origin - center).norm() < 1e-12 && (*r - radius).abs() < 1e-12)
                    }).collect::<Vec<_>>();
                    let [rim] = rim.as_slice() else { panic!("one circular split edge required"); };
                    let coedges = body.edge_coedges(*rim);
                    assert_eq!(coedges.len(), 2);
                    assert_ne!(body.coedges.get(coedges[0]).unwrap().forward, body.coedges.get(coedges[1]).unwrap().forward);
                    let circle = body.curves3.get(body.edges.get(*rim).unwrap().curve).unwrap();
                    for id in coedges {
                        let usage = body.coedges.get(id).unwrap();
                        let face = body.faces.get(body.loops.get(usage.loop_id).unwrap().face).unwrap();
                        let support = body.surfaces.get(face.surface).unwrap();
                        let trim = body.curves2.get(usage.pcurve.unwrap()).unwrap();
                        for t in [0.0_f64, 0.5, 1.5] {
                            let uv = trim.eval(t.copysign(usage.prange.1));
                            assert!(support.eval(uv.x, uv.y).distance(circle.eval(t)) < 1e-12);
                        }
                    }
                    Ok(())
                }).unwrap();
            } else {
                assert_eq!(preview.component_references["face"].len(), 6);
            }
            let (min, max) = preview.aabb();
            for (index, coords) in [min, max].into_iter().enumerate() {
                for (actual, expected) in coords.into_iter().zip(fixture_xyz(&expected["bounds"][index])) {
                    assert!((f64::from(actual) - expected).abs() < 1e-5);
                }
            }
            let measure = retained_boolean_result(&registry, "brep.measure.volume", Dictionary::new().insert("geometry", value.clone()),
                &mut caller, &mut born, &mut free);
            let volume = read_channel_number(&measure, "volume").unwrap();
            assert!((volume - expected["volume"].as_f64().unwrap()).abs() < 1e-10);
            total_volume += volume;
        }
        let measure = retained_boolean_result(&registry, "brep.measure.volume",
            Dictionary::new().insert("geometry", placed.get("geometryOut").unwrap().clone()), &mut caller, &mut born, &mut free);
        assert!((total_volume - read_channel_number(&measure, "volume").unwrap()).abs() < 1e-10);
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

#[test]
fn original_orthogonal_results_can_be_used_as_either_boolean_operand() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-orthogonal-chain.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 97180, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(identity.generation), stock(), || Some(1000), 1500, 32768);
    let (mut born, mut free) = (0, 0);
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Expected { Ready, Refused, Cancelled }
    let mut run_case = |row: &serde_json::Value, expected: Expected| {
        let box_value = |recipe: &serde_json::Value, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize| {
            let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&recipe["size"]))
                .fold(Dictionary::new(), |input, (axis, size)| input.insert(axis, number(size)));
            let made = run_live_query(&registry, "brep.prim3d.box", input, caller, born, free);
            let shape = made.get("solid").unwrap().clone();
            let origin = fixture_xyz(&recipe["origin"]);
            if origin == [0.0; 3] { return shape; }
            let input = Dictionary::new().insert("geometry", shape).insert("offset", Value::Dictionary(vector_dictionary(origin)));
            run_live_query(&registry, "brep.xform.translate", input, caller, born, free).get("geometryOut").unwrap().clone()
        };
        let mut solid = |recipe: &serde_json::Value| {
            let stock = box_value(&recipe["stock"], &mut caller, &mut born, &mut free);
            let Some(void) = recipe.get("void") else { return stock; };
            let tool = box_value(void, &mut caller, &mut born, &mut free);
            let input = Dictionary::new().insert("a", stock).insert("b", tool);
            run_live_query(&registry, "brep.bool.cut", input, &mut caller, &mut born, &mut free).get("solid").unwrap().clone()
        };
        let input = Dictionary::new().insert("a", solid(&row["a"])).insert("b", solid(&row["b"]));
        let operator = format!("brep.bool.{}", row["operation"].as_str().unwrap());
        let count = || session.with_kernel_read(|kernel| Ok((kernel.live_handles().len(), kernel.representation().solids.iter().count()))).unwrap();
        let initial = count();
        let mut job = Some(admit_live_job(&registry, &operator, input));
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        let mut output = None;
        let mut outcome = None;
        let mut cancel_requested = false;
        for _ in 0..10000 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                match job.as_mut().unwrap().step(cx) {
                    Ok((neural_engine::OperatorJobStep::Working(_), _)) => {},
                    Ok((neural_engine::OperatorJobStep::Done(result), _)) => { output = Some(result); outcome = Some(Expected::Ready); },
                    Ok((neural_engine::OperatorJobStep::Cancelled(_), _)) => outcome = Some(Expected::Cancelled),
                    Err(_) => outcome = Some(Expected::Refused),
                    _ => panic!("unexpected original rectilinear Boolean disposition"),
                }
                Ok(())
            });
            if outcome.is_some() { break; }
            let (live, solids) = count();
            assert_eq!(live, initial.0, "Boolean result became public before completion");
            if expected == Expected::Cancelled && !cancel_requested && solids > initial.1 {
                job.as_mut().unwrap().cancel();
                cancel_requested = true;
            }
        }
        assert!(outcome == Some(expected), "{}", row["name"]);
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let created = usize::from(expected == Expected::Ready);
        assert_eq!(count(), (initial.0 + created, initial.1 + created));
        if expected != Expected::Ready { return; }
        let output = neural_engine::ColdOwner::new(output.expect("exact orthogonal result"));
        let handle = read_geometry(&output, "solid").unwrap();
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
        let preview = run_funded_preview(&session, identity, &handle.0, &mut caller, &mut born, &mut free);
        preview.validate_component_references().unwrap();
        let (min, max) = preview.aabb();
        assert_eq!(min.map(f64::from), fixture_xyz(&row["bounds"][0]));
        assert_eq!(max.map(f64::from), fixture_xyz(&row["bounds"][1]));
        let input = Dictionary::new().insert("geometry", output.get("solid").unwrap().clone());
        let volume = run_live_query(&registry, "brep.measure.volume", input, &mut caller, &mut born, &mut free);
        assert!((read_channel_number(&volume, "volume").unwrap() - row["volume"].as_f64().unwrap()).abs() < 1e-10);
    };
    for row in fixture["cases"].as_array().unwrap() { run_case(row, Expected::Ready); }
    for row in fixture["refused"].as_array().unwrap() { run_case(row, Expected::Refused); }
    run_case(&fixture["cases"][1], Expected::Cancelled);
}

#[test]
fn original_closed_helical_sweep_constructs_one_exact_torus_and_retires_a_draft() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Vec3;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json")).unwrap();
    let row=&fixture["retainedClosedHelicalSweep"];
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity=GeometryWorkIdentity{operation:98601,generation:17};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    let point=|raw|Value::Dictionary(point_dictionary(fixture_xyz(raw)));
    let vector=|raw|Value::Dictionary(vector_dictionary(fixture_xyz(raw)));
    let minor=row["profile"]["radius"].as_f64().unwrap();
    let source=Dictionary::new().insert("center",point(&row["profile"]["center"]))
        .insert("normal",vector(&row["profile"]["normal"])).insert("radius",number(minor));
    let circle=retained_boolean_result(&registry,"brep.curve.circle",source,&mut caller,&mut born,&mut free);
    let input=Dictionary::new().insert("wire",circle.get("curve").unwrap().clone());
    let face=retained_boolean_result(&registry,"brep.util.faceFromWire",input,&mut caller,&mut born,&mut free);
    let profile=read_geometry(&face,"face").unwrap();
    let input=|radius,pitch,turns|Dictionary::new().insert("profile",face.get("face").unwrap().clone())
        .insert("axisOrigin",point(&row["axisOrigin"])).insert("axisDirection",vector(&row["axisDirection"]))
        .insert("radius",number(radius)).insert("pitch",number(pitch)).insert("turns",number(turns));
    let operator="brep.sweep.helical";
    let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
    let counts=||session.with_kernel_read(|kernel|{
        let body=kernel.representation();
        Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
    }).unwrap();
    let before=counts();
    for refused in row["refused"].as_array().unwrap() {
        let values=input(refused["radius"].as_f64().unwrap(),refused["pitch"].as_f64().unwrap(),refused["turns"].as_f64().unwrap());
        let mut job=Some(admit_live_job(&registry,operator,values));
        let mut failed=false;
        for _ in 0..60 {
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                match job.as_mut().unwrap().step(cx) {
                    Ok((neural_engine::OperatorJobStep::Working(_),receipt))=>assert!(receipt.fits(grant)),
                    Err(_)=>failed=true,
                    Ok(_)=>panic!("unexpected closed helical output"),
                }
                Ok(())
            });
            if failed{break;}
        }
        assert!(failed);
        close_live_job(&mut job,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
    }
    let major=row["radius"].as_f64().unwrap();
    let pitch=row["pitch"].as_f64().unwrap();
    let mut draft=Some(admit_live_job(&registry,operator,input(major,pitch,1.)));
    for _ in 0..100 {
        run(&mut caller,grant,&mut born,&mut free,|cx|{
            let (step,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
            assert!(matches!(step,neural_engine::OperatorJobStep::Working(_)));
            assert!(receipt.fits(grant));
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
    for turns in row["turns"].as_array().unwrap() {
        let shape=retained_boolean_result(&registry,operator,input(major,pitch,turns.as_f64().unwrap()),&mut caller,&mut born,&mut free);
        let handle=read_geometry(&shape,"solid").unwrap();
        let axis=Vec3::from_array(fixture_xyz(&row["axisDirection"])).normalized().unwrap();
        session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            assert!(kernel.label_of(&profile).is_some());
            let solid=kernel.geometry_solid(&handle).unwrap();
            let faces=body.solid_faces(solid);
            assert_eq!(faces.len(),1);
            let face=body.faces.get(faces[0]).unwrap();
            let Surface::Torus{frame,major_radius,minor_radius}=body.surfaces.get(face.surface).unwrap()else{panic!("expected analytic torus");};
            assert_eq!(frame.origin.to_array(),fixture_xyz(&row["axisOrigin"]));
            assert!((frame.z-axis).norm()<1e-14);
            assert_eq!((*major_radius,*minor_radius),(major,minor));
            Ok(())
        }).unwrap();
        let measure=Dictionary::new().insert("geometry",shape.get("solid").unwrap().clone());
        let volume=retained_boolean_result(&registry,"brep.measure.volume",measure,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-row["volume"].as_f64().unwrap()).abs()<1e-12);
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["face"].len(),1);
    }
    caller.retire(stock());
}

#[test]
fn original_smooth_offset_keeps_the_analytic_frame_and_retires_the_draft() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🛠️brep-feature/🔣️.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity=GeometryWorkIdentity{operation:98602,generation:17};
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,32768);
    let (mut born,mut free)=(0,0);
    let vector=|raw|Value::Dictionary(vector_dictionary(fixture_xyz(raw)));
    let counts=||session.with_kernel_read(|kernel|{
        let body=kernel.representation();
        Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
    }).unwrap();
    let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
    for row in fixture["retainedSmoothSolidOffset"].as_array().unwrap() {
        let sphere=row["kind"]=="sphere";
        let minor=row["minor"].as_f64().unwrap();
        let major=row["major"].as_f64().unwrap();
        let (constructor,source)=if sphere {("brep.prim3d.sphere",Dictionary::new().insert("radius",number(minor)))}
        else {("brep.prim3d.torus",Dictionary::new().insert("major",number(major)).insert("minor",number(minor)))};
        let shape=retained_boolean_result(&registry,constructor,source,&mut caller,&mut born,&mut free);
        let rotation=Dictionary::new().insert("geometry",shape.get("solid").unwrap().clone())
            .insert("axis",vector(&row["axis"])).insert("angle",number(row["angle"].as_f64().unwrap()));
        let rotated=retained_boolean_result(&registry,"brep.xform.rotate",rotation,&mut caller,&mut born,&mut free);
        let translation=Dictionary::new().insert("geometry",rotated.get("geometryOut").unwrap().clone()).insert("offset",vector(&row["origin"]));
        let source=retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
        let source_handle=read_geometry(&source,"geometryOut").unwrap();
        let surface=|handle:&semio_framework_3d_brep_engine::GeometryHandle|session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let faces=body.solid_faces(kernel.geometry_solid(handle).unwrap());
            assert_eq!(faces.len(),1);
            let face=body.faces.get(faces[0]).unwrap();
            Ok(body.surfaces.get(face.surface).unwrap().clone())
        }).unwrap();
        let initial=surface(&source_handle);
        let input=|distance|Dictionary::new().insert("geometry",source.get("geometryOut").unwrap().clone()).insert("distance",number(distance));
        let operator="brep.solid.offsetSolid";
        let before=counts();
        for distance in row["refused"].as_array().unwrap() {
            let mut job=Some(admit_live_job(&registry,operator,input(distance.as_f64().unwrap())));
            run(&mut caller,grant,&mut born,&mut free,|cx| {
                assert!(job.as_mut().unwrap().step(cx).is_err());
                Ok(())
            });
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let cases=row["cases"].as_array().unwrap();
        let mut draft=Some(admit_live_job(&registry,operator,input(cases[0]["distance"].as_f64().unwrap())));
        for _ in 0..100 {
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_)));assert!(receipt.fits(grant));
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
        for case in cases {
            let output=retained_boolean_result(&registry,operator,input(case["distance"].as_f64().unwrap()),&mut caller,&mut born,&mut free);
            let handle=read_geometry(&output,"solid").unwrap();
            let actual=surface(&handle);
            let radius=case["radius"].as_f64().unwrap();
            match (&initial,actual) {
                (Surface::Sphere{frame:old,..},Surface::Sphere{frame,radius:actual})=>{
                    assert_eq!(frame,*old);assert_eq!(frame.origin.to_array(),fixture_xyz(&row["origin"]));assert_eq!(actual,radius);
                }
                (Surface::Torus{frame:old,..},Surface::Torus{frame,major_radius,minor_radius})=>{
                    assert_eq!(frame,*old);assert_eq!(frame.origin.to_array(),fixture_xyz(&row["origin"]));
                    assert_eq!((major_radius,minor_radius),(major,radius));
                }
                _=>panic!("offset changed the analytic primitive family")
            }
            let measure=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
            let volume=retained_boolean_result(&registry,"brep.measure.volume",measure,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&volume,"volume").unwrap()-case["volume"].as_f64().unwrap()).abs()<1e-11);
            let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.component_references["face"].len(),1);
        }
        assert_eq!(surface(&source_handle),initial);
    }
    caller.retire(stock());
}

#[test]
fn original_loft_publishes_the_exact_two_circle_frustum_and_cancels_its_draft() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,read_channel_number,text_dictionary,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json")).unwrap();
    for (i,row) in corpus["retainedConicalLoft"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98681+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let sections=[("start","radius0"),("end","radius1")].map(|(center,radius)|{
            let input=Dictionary::new().insert("center",Value::Dictionary(point_dictionary(fixture_xyz(&row[center]))))
                .insert("normal",Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))))
                .insert("radius",number(row[radius].as_f64().unwrap()));
            let remaining=retained_boolean_result(&registry,"brep.curve.circle",input,&mut caller,&mut born,&mut free);
            let input=Dictionary::new().insert("wire",remaining.get("curve").unwrap().clone());
            retained_boolean_result(&registry,"brep.util.faceFromWire",input,&mut caller,&mut born,&mut free)
        });
        let handles=sections.each_ref().map(|section|read_geometry(section,"face").unwrap());
        let profiles=Dictionary::with_schema("list").insert("2",sections[0].get("face").unwrap().clone())
            .insert("10",sections[1].get("face").unwrap().clone());
        let smooth=u8::from(row["smooth"].as_bool().unwrap());
        let input=Dictionary::new().insert("profiles",Value::Dictionary(profiles)).insert("smooth",number(f64::from(smooth)));
        let operator="brep.sweep.loft";
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
        for _ in 0..100{
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

        let solid=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&solid,"solid").unwrap();
        let (side,rims)=session.with_kernel_read(|kernel|{
            assert!(handles.iter().all(|handle|kernel.label_of(handle).is_some()));
            let body=kernel.representation();
            let reach=body.reachable_from(&[EntityRef::Solid(kernel.geometry_solid(&handle).unwrap())]);
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),(2,3,3));
            let side=reach.faces.iter().find_map(|&id|{
                let face=body.faces.get(id)?;
                matches!(body.surfaces.get(face.surface),Some(Surface::Cone{..})).then(||face.label.0.to_string())
            }).unwrap();
            let mut labels=Vec::new();
            for &id in &reach.edges{
                let uses=body.edge_coedges(id);
                assert_eq!(uses.len(),2);
                assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
                let edge=body.edges.get(id).unwrap();
                if let Some(Curve3::Circle{frame,radius})=body.curves3.get(edge.curve){
                    let expected=[("start","radius0"),("end","radius1")].into_iter().find(|(_,r)|row[*r].as_f64().unwrap()==*radius).unwrap();
                    assert!(frame.origin.distance(Pnt3::from_array(fixture_xyz(&row[expected.0])))<1e-12);
                    labels.push(edge.label.0.to_string());
                }
            }
            assert_eq!(labels.len(),2);
            Ok((side,labels))
        }).unwrap();
        let measure=Dictionary::new().insert("geometry",solid.get("solid").unwrap().clone());
        let volume=retained_boolean_result(&registry,"brep.measure.volume",measure,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-row["volume"].as_f64().unwrap()).abs()<1e-10);
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(mesh.component_references["face"].contains(&side));
        assert!(rims.iter().all(|label|mesh.component_references["edge"].contains(label)));
        assert!((preview_area_volume(&mesh).1-row["volume"].as_f64().unwrap()).abs()<0.4);
        let text=|value|Value::Dictionary(text_dictionary(value));
        let select=Dictionary::new().insert("brep",solid.get("solid").unwrap().clone())
            .insert("faceLabels",text(serde_json::to_string(&[&side]).unwrap())).insert("edgeLabels",text("[]".to_owned()))
            .insert("sourceHandle",text(String::new()));
        let selected=retained_boolean_result(&registry,"brep.brep",select,&mut caller,&mut born,&mut free);
        let faces=selected.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
        let side_handle=read_geometry(faces,"0").unwrap();
        let preview=run_funded_preview(&session,identity,side_handle.as_str(),&mut caller,&mut born,&mut free);
        preview.validate_component_references().unwrap();
        assert_eq!(preview.component_references["face"],[side]);
        assert!(rims.iter().all(|label|preview.component_references["edge"].contains(label)));
        caller.retire(stock());
    }
}

#[test]
fn original_partial_revolve_publishes_capped_native_torus_and_cancels_draft() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,read_channel_number,text_dictionary,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json")).unwrap();
    for (i,row) in corpus["retainedPartialRevolve"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98731+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let point=|raw|Value::Dictionary(point_dictionary(fixture_xyz(raw)));
        let vector=|raw|Value::Dictionary(vector_dictionary(fixture_xyz(raw)));
        let radius=row["radius"].as_f64().unwrap();
        let source=Dictionary::new().insert("center",point(&row["center"])).insert("normal",vector(&row["normal"])).insert("radius",number(radius));
        let circle=retained_boolean_result(&registry,"brep.curve.circle",source,&mut caller,&mut born,&mut free);
        let input=Dictionary::new().insert("wire",circle.get("curve").unwrap().clone());
        let face=retained_boolean_result(&registry,"brep.util.faceFromWire",input,&mut caller,&mut born,&mut free);
        let profile=read_geometry(&face,"face").unwrap();
        let input=|angle|Dictionary::new().insert("face",face.get("face").unwrap().clone())
            .insert("axisOrigin",point(&row["origin"])).insert("axisDirection",vector(&row["axis"])).insert("angle",number(angle));
        let operator="brep.sweep.revolve";
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let mut zero=Some(admit_live_job(&registry,operator,input(0.)));
        run(&mut caller,grant,&mut born,&mut free,|cx|{
            assert!(zero.as_mut().unwrap().step(cx).is_err());
            Ok(())
        });
        close_live_job(&mut zero,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let angle=row["angle"].as_f64().unwrap();
        let mut draft=Some(admit_live_job(&registry,operator,input(angle)));
        for _ in 0..100 {
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_)));assert!(receipt.fits(grant));
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
        let output=retained_boolean_result(&registry,operator,input(angle),&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"solid").unwrap();
        let (side,cap,rims)=session.with_kernel_read(|kernel|{
            assert!(kernel.label_of(&profile).is_some());
            let body=kernel.representation();
            let reach=body.reachable_from(&[EntityRef::Solid(kernel.geometry_solid(&handle).unwrap())]);
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),(2,3,3));
            let center=Pnt3::from_array(fixture_xyz(&row["center"]));
            let mut side=None;
            let mut cap=None;
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                match body.surfaces.get(face.surface).unwrap(){
                    Surface::Plane{frame} if frame.origin.distance(center)<1e-12=>cap=Some(face.label.0.to_string()),
                    Surface::Torus{major_radius,minor_radius,..}=>{
                        assert_eq!((*major_radius,*minor_radius),(row["major"].as_f64().unwrap(),radius));
                        side=Some(face.label.0.to_string());
                    }
                    _=>{}
                }
            }
            let mut rims=Vec::new();
            for &id in &reach.edges{
                let uses=body.edge_coedges(id);
                assert_eq!(uses.len(),2);
                assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
                let edge=body.edges.get(id).unwrap();
                let Curve3::Circle{frame,radius:actual}=body.curves3.get(edge.curve).unwrap()else{unreachable!()};
                if edge.v0==edge.v1{
                    assert_eq!(*actual,radius);
                    assert!([center,Pnt3::from_array(fixture_xyz(&row["end"]))].into_iter().any(|p|frame.origin.distance(p)<1e-12));
                    rims.push(edge.label.0.to_string());
                }else{assert_eq!(edge.range,(0.,angle.abs()));}
            }
            assert_eq!(rims.len(),2);
            Ok((side.unwrap(),cap.unwrap(),rims))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(mesh.component_references["face"].contains(&side));
        assert!(mesh.component_references["face"].contains(&cap));
        assert!(rims.iter().all(|label|mesh.component_references["edge"].contains(label)));
        let input=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
        let volume=retained_boolean_result(&registry,"brep.measure.volume",input,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-row["volume"].as_f64().unwrap()).abs()<1e-11);
        let text=|value|Value::Dictionary(text_dictionary(value));
        let select=Dictionary::new().insert("brep",output.get("solid").unwrap().clone())
            .insert("faceLabels",text(serde_json::to_string(&[&cap]).unwrap())).insert("edgeLabels",text("[]".to_owned()))
            .insert("sourceHandle",text(String::new()));
        let selected=retained_boolean_result(&registry,"brep.brep",select,&mut caller,&mut born,&mut free);
        let cap_list=selected.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
        let cap_handle=read_geometry(cap_list,"0").unwrap();
        let preview=run_funded_preview(&session,identity,cap_handle.as_str(),&mut caller,&mut born,&mut free);
        preview.validate_component_references().unwrap();
        assert_eq!(preview.component_references["face"],[cap]);
        caller.retire(stock());
    }
}

#[test]
fn original_revolve_preserves_full_and_partial_annular_boundaries_and_cancels() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,read_channel_number,read_xyz,text_dictionary,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json")).unwrap();
    for (index,row) in corpus["retainedMeridionalRevolve"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98751+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let profile=retained_boolean_result(&registry,"brep.surf.planarFace",polygon_fixture_input(&serde_json::json!({"points":row["points"]})),&mut caller,&mut born,&mut free);
        let profile_handle=read_geometry(&profile,"face").unwrap();
        let input=|origin:&serde_json::Value|Dictionary::new().insert("face",profile.get("face").unwrap().clone())
            .insert("axisOrigin",Value::Dictionary(point_dictionary(fixture_xyz(origin))))
            .insert("axisDirection",Value::Dictionary(vector_dictionary(fixture_xyz(&row["axis"]))))
            .insert("angle",number(row["angle"].as_f64().unwrap()));
        let operator="brep.sweep.revolve";
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let mut invalid=Some(admit_live_job(&registry,operator,input(&row["points"][0])));
        run(&mut caller,grant,&mut born,&mut free,|cx|{
            assert!(invalid.as_mut().unwrap().step(cx).is_err());
            Ok(())
        });
        close_live_job(&mut invalid,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let authored=input(&row["origin"]);
        let mut draft=Some(admit_live_job(&registry,operator,authored.clone()));
        for _ in 0..100{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (state,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(state,neural_engine::OperatorJobStep::Working(_)));assert!(receipt.fits(grant));
                Ok(())
            });
            if counts()[2]>before[2]{break;}
        }
        assert!(counts()[2]>before[2]);assert_eq!(counts()[0],before[0]);
        draft.as_mut().unwrap().cancel();
        run(&mut caller,grant,&mut born,&mut free,|cx|{
            assert!(matches!(draft.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Cancelled(_)));
            Ok(())
        });
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let output=retained_boolean_result(&registry,operator,authored,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"solid").unwrap();
        let sweep=row["angle"].as_f64().unwrap().abs();
        let full=sweep==std::f64::consts::TAU;
        let (face_labels,edge_labels,annulus)=session.with_kernel_read(|kernel|{
            assert!(kernel.label_of(&profile_handle).is_some());
            let body=kernel.representation();
            let reach=body.reachable_from(&[EntityRef::Solid(kernel.geometry_solid(&handle).unwrap())]);
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),if full{(4,6,4)}else{(8,12,6)});
            let mut cylinders=Vec::new();
            let mut annuli=Vec::new();
            let mut faces=Vec::new();
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                faces.push(face.label.0.to_string());
                if let Surface::Cylinder{radius,..}=body.surfaces.get(face.surface).unwrap(){
                    cylinders.push((*radius,face.flipped));
                }else if !face.inners.is_empty(){annuli.push(face.label.0.to_string());}
            }
            cylinders.sort_by(|a,b|a.0.total_cmp(&b.0));
            assert!((cylinders[0].0-row["inner"].as_f64().unwrap()).abs()<1e-12&&cylinders[0].1);
            assert!((cylinders[1].0-row["outer"].as_f64().unwrap()).abs()<1e-12&&!cylinders[1].1);
            assert_eq!(annuli.len(),if full{2}else{0});
            let mut edges=Vec::new();
            for &id in &reach.edges{
                let uses=body.edge_coedges(id);
                assert_eq!(uses.len(),2);
                let orientation=|id|{
                    let use_=body.coedges.get(id).unwrap();
                    let owner=body.loops.get(use_.loop_id).unwrap().face;
                    use_.forward!=body.faces.get(owner).unwrap().flipped
                };
                assert_ne!(orientation(uses[0]),orientation(uses[1]));
                let edge=body.edges.get(id).unwrap();
                if matches!(body.curves3.get(edge.curve),Some(Curve3::Circle{..})){
                    assert_eq!(edge.range,(0.,sweep));edges.push(edge.label.0.to_string());
                }
            }
            assert_eq!(edges.len(),4);
            Ok((faces,edges,annuli.into_iter().next()))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(face_labels.iter().all(|label|mesh.component_references["face"].contains(label)));
        assert!(edge_labels.iter().all(|label|mesh.component_references["edge"].contains(label)));
        let query=||Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
        for (op,key) in [("brep.measure.volume","volume"),("brep.measure.area","area")]{
            let read=retained_boolean_result(&registry,op,query(),&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&read,key).unwrap()-row[key].as_f64().unwrap()).abs()<1e-10);
        }
        let center=retained_boolean_result(&registry,"brep.measure.centerOfMass",query(),&mut caller,&mut born,&mut free);
        assert!(read_xyz(&center,"center").unwrap().into_iter().zip(fixture_xyz(&row["centroid"]))
            .all(|(a,b)|(a-b).abs()<1e-10));
        if let Some(label)=annulus {
            let text=|value|Value::Dictionary(text_dictionary(value));
            let select=Dictionary::new().insert("brep",output.get("solid").unwrap().clone())
                .insert("faceLabels",text(serde_json::to_string(&[&label]).unwrap())).insert("edgeLabels",text("[]".to_owned()))
                .insert("sourceHandle",text(String::new()));
            let selected=retained_boolean_result(&registry,"brep.brep",select,&mut caller,&mut born,&mut free);
            let list=selected.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
            let picked=read_geometry(list,"0").unwrap();
            let cap=run_funded_preview(&session,identity,picked.as_str(),&mut caller,&mut born,&mut free);
            cap.validate_component_references().unwrap();
            assert_eq!(cap.component_references["face"],[label]);
            let expected=std::f64::consts::PI*(row["outer"].as_f64().unwrap().powi(2)-row["inner"].as_f64().unwrap().powi(2));
            assert!((preview_area_volume(&cap).0-expected).abs()<0.35);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_closed_spherical_shell_keeps_inward_void_and_paid_preview() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🏗️brep-solid/🔣️.json")).unwrap();
    for (i,row) in fixture["retainedSphericalShell"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98871+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let radius=row["radius"].as_f64().unwrap();
        let inner=row["inner"].as_f64().unwrap();
        let center=fixture_xyz(&row["center"]);
        let raw=retained_boolean_result(&registry,"brep.prim3d.sphere",Dictionary::new().insert("radius",number(radius)),&mut caller,&mut born,&mut free);
        let translation=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(center)));
        let source=retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
        let shell_input=|thickness|Dictionary::new().insert("geometry",source.get("geometryOut").unwrap().clone())
            .insert("thickness",number(thickness)).insert("openFaces",Value::Dictionary(Dictionary::with_schema("list")));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.shells.len(),body.faces.len(),body.edges.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let operator="brep.solid.shell";
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for thickness in row["refusedThicknesses"].as_array().unwrap(){
            let mut refused=Some(admit_live_job(&registry,operator,shell_input(thickness.as_f64().unwrap())));
            run(&mut caller,grant,&mut born,&mut free,|cx|{assert!(refused.as_mut().unwrap().step(cx).is_err());Ok(())});
            close_live_job(&mut refused,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let input=shell_input(row["thickness"].as_f64().unwrap());
        let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
        for _ in 0..500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (state,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(state,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            if counts()[1]==before[1]+2{break;}
        }
        assert_eq!(counts()[1],before[1]+2);assert_eq!(counts()[0],before[0]);
        draft.as_mut().unwrap().cancel();
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let output=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"solid").unwrap();
        let labels=session.with_kernel_read(|kernel|{
            assert!(kernel.label_of(&read_geometry(&source,"geometryOut").unwrap()).is_some());
            let body=kernel.representation();
            let solid=body.solids.get(kernel.geometry_solid(&handle).unwrap()).unwrap();
            assert_eq!(solid.inners.len(),1);
            let mut labels=Vec::new();
            for (shell,expected,flipped) in [(solid.outer,radius,false),(solid.inners[0],inner,true)]{
                let shell=body.shells.get(shell).unwrap();
                assert_eq!(shell.faces.len(),1);
                let face=body.faces.get(shell.faces[0]).unwrap();
                let Surface::Sphere{frame,radius}=body.surfaces.get(face.surface).unwrap()else{panic!("expected exact sphere support")};
                assert_eq!(frame.origin.to_array(),center);
                assert_eq!(*radius,expected);
                assert_eq!(face.flipped,flipped);
                labels.push(face.label.0.to_string());
            }
            Ok(labels)
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(labels.iter().all(|key|mesh.component_references["face"].contains(key)));
        let mut sampled=[0,0];
        for (p,n) in mesh.positions.chunks_exact(3).zip(mesh.normals.chunks_exact(3)){
            let p=Pnt3::new(p[0] as f64,p[1] as f64,p[2] as f64)-Pnt3::from_array(center);
            let n=Pnt3::new(n[0] as f64,n[1] as f64,n[2] as f64);
            let inside=(p.norm()-inner).abs()<1e-5;
            assert!((p.norm()-if inside{inner}else{radius}).abs()<1e-5);
            assert!(if inside{p.dot(n)<0.}else{p.dot(n)>0.});
            sampled[usize::from(inside)]+=1;
        }
        assert!(sampled.into_iter().all(|count|count>0));
        let query=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
        let measure=retained_boolean_result(&registry,"brep.measure.volume",query,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&measure,"volume").unwrap()-row["volume"].as_f64().unwrap()).abs()<1e-9);
        caller.retire(stock());
    }
}
