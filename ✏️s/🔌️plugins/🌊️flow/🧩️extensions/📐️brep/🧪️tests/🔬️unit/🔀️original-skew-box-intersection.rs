#[test]
fn original_skew_box_intersect_keeps_the_certified_tool_support_and_refuses_other_overlap(){
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3};
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-skew-box-intersection.json")).unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    let cases=fixture["cases"].as_array().unwrap();
    let convex=fixture["convexCases"].as_array().unwrap();
    let empty=fixture["empty"].as_array().unwrap();
    let unsupported=fixture["unsupported"].as_array().unwrap();
    for (index,row) in cases.iter().chain(convex).chain(empty).chain(unsupported).enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:100270+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [stock_box,tool_box]=["stock","tool"].map(|key|{
            let spec=&row[key];
            let input=["width","depth","height"].into_iter().zip(fixture_xyz(&spec["size"]))
                .fold(Dictionary::new(),|input,(axis,span)|input.insert(axis,number(span)));
            let raw=retained_boolean_result(&registry,"brep.prim3d.box",input,&mut caller,&mut born,&mut free);
            let angles=[0,1].map(|i|spec["angles"][i].as_f64().unwrap());
            placed_finish_box(&registry,raw.get("solid").unwrap(),fixture_xyz(&spec["origin"]),angles,spec["mirror"].as_bool().unwrap(),
                &mut caller,&mut born,&mut free)
        });
        let input=|first:&Dictionary,second:&Dictionary|Dictionary::new()
            .insert("a",first.get("geometryOut").unwrap().clone()).insert("b",second.get("geometryOut").unwrap().clone());
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        if index==0||index==cases.len(){
            let before=counts();
            let mut job=Some(admit_live_job(&registry,operator,input(&stock_box,&tool_box)));
            for _ in 0..1800{
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                    Ok(())
                });
                assert_eq!(counts()[0],before[0]);
                if counts()[3]>before[3]{break;}
            }
            assert!(counts()[3]>before[3]);
            job.as_mut().unwrap().cancel();
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        for (left,right) in [(&stock_box,&tool_box),(&tool_box,&stock_box)]{
            let before=counts();
            if let Some(expected)=row["gap"].as_f64(){
                let measured=retained_boolean_result(&registry,"brep.measure.distance",input(left,right),&mut caller,&mut born,&mut free);
                assert!((read_channel_number(&measured,"distance").unwrap()-expected).abs()<1e-7);
                assert_eq!(counts(),before);
            }
            let mut job=Some(admit_live_job(&registry,operator,input(left,right)));
            let mut terminal=None;
            for _ in 0..1800{
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    match job.as_mut().unwrap().step(cx){
                        Ok((neural_engine::OperatorJobStep::Working(_),receipt))=>assert!(receipt.fits(grant)),
                        Ok((neural_engine::OperatorJobStep::Done(value),_))=>terminal=Some(Ok(neural_engine::ColdOwner::new(value))),
                        Err(error)=>terminal=Some(Err(format!("{error:?}"))),
                        _=>panic!("unexpected skew Intersect job state"),
                    }
                    Ok(())
                });
                if terminal.is_some(){break;}
                assert_eq!(counts()[0],before[0]);
            }
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            let result=terminal.unwrap();
            if index>=cases.len()+convex.len(){
                let error=result.err().unwrap();
                let expected=if index<cases.len()+convex.len()+empty.len(){"empty"}else{"UnsupportedOwner"};
                assert!(error.contains(expected),"{error}");
                assert_eq!(counts(),before);
                continue;
            }
            let output=result.unwrap();
            let shape=read_geometry(&output,"solid").unwrap();
            assert_eq!(counts()[0],before[0]+1);
            let cap_label=session.with_kernel_read(|kernel|{
                let id=kernel.geometry_solid(&shape).unwrap();
                let body=kernel.representation();
                let reach=body.reachable_from(&[EntityRef::Solid(id)]);
                assert_eq!(reach.vertices.len()+reach.faces.len(),reach.edges.len()+2);
                for &id in &reach.edges{
                    let uses=body.edge_coedges(id);
                    assert_eq!(uses.len(),2);
                    assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
                }
                if index>=cases.len(){
                    for (key,count) in [("vertices",reach.vertices.len()),("edges",reach.edges.len()),("faces",reach.faces.len())]{
                        assert_eq!(count as u64,row[key].as_u64().unwrap());
                    }
                    let mut authored=Vec::new();
                    for operand in [&stock_box,&tool_box]{
                        let handle=read_geometry(operand,"geometryOut").unwrap();
                        let id=kernel.geometry_solid(&handle).unwrap();
                        let stock=body.solids.get(id).unwrap();
                        for &id in &body.shells.get(stock.outer).unwrap().faces{
                            let face=body.faces.get(id).unwrap();
                            let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{unreachable!();};
                            authored.push((frame.origin,if face.flipped{-frame.z}else{frame.z}));
                        }
                    }
                    let mut uses=[false;12];
                    for &id in &reach.faces{
                        let face=body.faces.get(id).unwrap();
                        let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{panic!("expected authored planar support");};
                        let selected=authored.iter().position(|&(origin,normal)|{
                            (normal-frame.z).norm()<1e-10&&((origin-frame.origin).dot(normal)).abs()<1e-10
                        }).unwrap();
                        assert!(!std::mem::replace(&mut uses[selected],true));
                    }
                    return Ok(None);
                }
                let cap_origin=Pnt3::from_array(fixture_xyz(&row["capOrigin"]));
                let cap_normal=Vec3::from_array(fixture_xyz(&row["capNormal"]));
                let (id,face)=reach.faces.iter().find_map(|&id|{
                    let face=body.faces.get(id)?;
                    let Surface::Plane{frame}=body.surfaces.get(face.surface)? else{return None;};
                    ((frame.z-cap_normal).norm()<1e-10&&((frame.origin-cap_origin).dot(cap_normal)).abs()<1e-10)
                        .then_some((id,face))
                }).unwrap();
                assert_eq!(body.reachable_from(&[EntityRef::Face(id)]).vertices.len() as u64,row["capVertices"].as_u64().unwrap());
                Ok(Some(face.label.0.to_string()))
            }).unwrap();
            let mesh=run_funded_preview(&session,identity,shape.as_str(),&mut caller,&mut born,&mut free);
            mesh.validate_component_references().unwrap();
            if let Some(cap_label)=cap_label{assert!(mesh.component_references["face"].contains(&cap_label));}
            let (min,max)=mesh.aabb();
            for (side,actual) in [min,max].into_iter().enumerate(){
                assert!(actual.into_iter().zip(fixture_xyz(&row["bounds"][side])).all(|(a,b)|(f64::from(a)-b).abs()<1e-6));
            }
            let solid=output.get("solid").unwrap();
            for (key,op) in [("volume","brep.measure.volume"),("area","brep.measure.area")]{
                let query=retained_boolean_result(&registry,op,Dictionary::new().insert("geometry",solid.clone()),&mut caller,&mut born,&mut free);
                assert!((read_channel_number(&query,key).unwrap()-row[key].as_f64().unwrap()).abs()<1e-10);
            }
            for probe in row["classify"].as_array().into_iter().flatten(){
                let input=Dictionary::new().insert("solid",solid.clone())
                    .insert("point",Value::Dictionary(point_dictionary(fixture_xyz(&probe[0]))));
                let measured=retained_boolean_result(&registry,"brep.measure.classify",input,&mut caller,&mut born,&mut free);
                assert_eq!(read_channel_number(&measured,"classification").unwrap(),probe[1].as_f64().unwrap());
            }
            if index==0 || row["section"].is_object(){
                let fact=&row["section"];
                let origin=if index==0{[2.,2.,2.]}else{fixture_xyz(&fact["origin"])};
                let normal=if index==0{[0.,0.,1.]}else{fixture_xyz(&fact["normal"])};
                let area=if index==0{8.0}else{fact["area"].as_f64().unwrap()};
                let input=Dictionary::new().insert("solid",solid.clone())
                    .insert("planeOrigin",Value::Dictionary(point_dictionary(origin)))
                    .insert("planeNormal",Value::Dictionary(vector_dictionary(normal)));
                let section=retained_boolean_result(&registry,"brep.intersect.section",input,&mut caller,&mut born,&mut free);
                let faces=section.get("faces").and_then(Value::as_dictionary).unwrap();
                let measured=retained_boolean_result(&registry,"brep.measure.area",
                    Dictionary::new().insert("geometry",faces.get("0").unwrap().clone()),&mut caller,&mut born,&mut free);
                assert!((read_channel_number(&measured,"area").unwrap()-area).abs()<1e-6);
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn disconnected_skew_booleans_keep_closed_original_components() {
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};
    let intersection: serde_json::Value = serde_json::from_str(include_str!("🔣️original-skew-box-intersection.json")).unwrap();
    let difference: serde_json::Value = serde_json::from_str(include_str!("🔣️original-skew-box-difference.json")).unwrap();
    let union = &intersection["separatedUnion"];
    let cut = &difference["compounds"][0];
    let union_members = union["volumes"].as_array().unwrap().iter().zip(union["areas"].as_array().unwrap())
        .map(|(volume, area)| (volume.as_f64().unwrap(), area.as_f64().unwrap(), 2)).collect::<Vec<_>>();
    let cut_members = cut["components"].as_array().unwrap().iter()
        .map(|row| (row["volume"].as_f64().unwrap(), row["area"].as_f64().unwrap(), row["euler"].as_i64().unwrap())).collect::<Vec<_>>();
    let cut_ref = cut["reference"].as_array().unwrap();
    let cut_inputs = &intersection[cut_ref[0].as_str().unwrap()][cut_ref[1].as_u64().unwrap() as usize];
    let union_inputs = &intersection["empty"][union["reference"].as_u64().unwrap() as usize];
    for (index, (operation, inputs, mut expected)) in [
        ("brep.bool.fuse", union_inputs, union_members), ("brep.bool.cut", cut_inputs, cut_members),
    ].into_iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = GeometryWorkIdentity { operation: 100329 + index as u64 * 8, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let [a,b]=fixture_union_operands(&registry,inputs,&mut caller,&mut born,&mut free);
        let input = Dictionary::new().insert("a", a.get("geometryOut").unwrap().clone()).insert("b", b.get("geometryOut").unwrap().clone());
        let counts = || session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            Ok([kernel.live_handles().len(), body.solids.len(), body.shells.len(), body.vertices.len(), body.edges.len(), body.faces.len()])
        }).unwrap();
        let before = counts();
        let grant = Grant { maximum_items: 1, ..stock() };
        let mut pending = Some(admit_live_job(&registry, operation, input.clone()));
        let cancel_at = before[1] + expected.len() - 1;
        for _ in 0..1500 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                let (step, receipt) = pending.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                assert!(receipt.fits(grant));
                Ok(())
            });
            assert_eq!(counts()[0], before[0]);
            if counts()[1] == cancel_at { break; }
        }
        assert_eq!(counts()[1], cancel_at);
        pending.as_mut().unwrap().cancel();
        close_live_job(&mut pending, &mut caller, &mut born, &mut free);
        assert_eq!(counts(), before);
        let merged = retained_boolean_result(&registry, operation, input, &mut caller, &mut born, &mut free);
        assert_eq!(counts()[0], before[0] + 1);
        let root = read_geometry(&merged, "solid").unwrap();
        let projection = run_funded_preview(&session, identity, root.as_str(), &mut caller, &mut born, &mut free);
        projection.validate_component_references().unwrap();
        let (whole_area, whole_volume) = preview_area_volume(&projection);
        assert!((whole_volume - expected.iter().map(|(volume, _, _)| volume).sum::<f64>()).abs() < 1e-5);
        assert!((whole_area - expected.iter().map(|(_, area, _)| area).sum::<f64>()).abs() < 1e-5);
        for (key,preview) in [("volume",whole_volume),("area",whole_area)] {
            let input=Dictionary::new().insert("geometry",merged.get("solid").unwrap().clone());
            let output=retained_boolean_result(&registry,&format!("brep.measure.{key}"),input,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&output,key).unwrap()-preview).abs()<1e-5);
        }
        if operation=="brep.bool.cut" {
            let input=Dictionary::new().insert("geometry",merged.get("solid").unwrap().clone());
            let output=retained_boolean_result(&registry,"brep.measure.centerOfMass",input,&mut caller,&mut born,&mut free);
            let point=output.get("center").and_then(Value::as_dictionary).unwrap();
            for (key,expected) in ["x","y","z"].into_iter().zip(fixture_xyz(&cut["center"])){
                let coordinate=point.get(key).and_then(Value::as_atom).and_then(|atom|atom.as_f64()).unwrap();
                assert!((coordinate-expected).abs()<1e-7);
            }
            for probe in cut["classify"].as_array().unwrap(){
                let input=Dictionary::new().insert("solid",merged.get("solid").unwrap().clone())
                    .insert("point",Value::Dictionary(point_dictionary(fixture_xyz(&probe[0]))));
                let output=retained_boolean_result(&registry,"brep.measure.classify",input,&mut caller,&mut born,&mut free);
                assert_eq!(read_channel_number(&output,"classification").unwrap(),probe[1].as_f64().unwrap());
            }
            let input = Dictionary::new().insert("geometry", merged.get("solid").unwrap().clone());
            let box_result = retained_boolean_result(&registry, "brep.measure.boundingBox", input, &mut caller, &mut born, &mut free);
            let box_handle = read_geometry(&box_result, "box").unwrap();
            let box_mesh = run_funded_preview(&session, identity, box_handle.as_str(), &mut caller, &mut born, &mut free);
            let (min, max) = box_mesh.aabb();
            for (actual, expected) in [min, max].into_iter().zip(cut["bounds"].as_array().unwrap()) {
                assert!(actual.into_iter().zip(fixture_xyz(expected)).all(|(a,b)| (f64::from(a)-b).abs()<1e-6));
            }
            for probe in cut["nearest"].as_array().unwrap(){
                let input=Dictionary::new().insert("geometry",merged.get("solid").unwrap().clone())
                    .insert("point",Value::Dictionary(point_dictionary(fixture_xyz(&probe["probe"]))));
                let output=retained_boolean_result(&registry,"brep.measure.closestPoint",input,&mut caller,&mut born,&mut free);
                let point=output.get("pointOut").and_then(Value::as_dictionary).unwrap();
                for (key,expected) in ["x","y","z"].into_iter().zip(fixture_xyz(&probe["point"])){
                    let coordinate=point.get(key).and_then(Value::as_atom).and_then(|atom|atom.as_f64()).unwrap();
                    assert!((coordinate-expected).abs()<2e-7);
                }
            }
        }
        let input = Dictionary::new().insert("compound", merged.get("solid").unwrap().clone());
        let children = retained_boolean_result(&registry, "brep.topology.explode", input, &mut caller, &mut born, &mut free);
        let Value::Dictionary(solids) = children.get("solids").unwrap() else { panic!("expected native solids"); };
        assert_eq!(solids.len(), expected.len());
        let mut actual = Vec::new();
        for index in 0..solids.len() {
            let value = solids.get(&index.to_string()).unwrap();
            let handle = read_geometry(&Dictionary::new().insert("solid", value.clone()), "solid").unwrap();
            let chi = session.with_kernel_read(|kernel| {
                let id = kernel.geometry_solid(&handle).unwrap();
                let body = kernel.representation();
                let reach = body.reachable_from(&[EntityRef::Solid(id)]);
                assert_eq!(reach.shells.len(), 1);
                for &edge in &reach.edges {
                    let uses = body.edge_coedges(edge);
                    assert_eq!(uses.len(), 2);
                    assert_ne!(body.coedges.get(uses[0]).unwrap().forward, body.coedges.get(uses[1]).unwrap().forward);
                }
                Ok(reach.vertices.len() as i64 - reach.edges.len() as i64 + reach.faces.len() as i64)
            }).unwrap();
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            let (area, volume) = preview_area_volume(&mesh);
            let measured = ["volume", "area"].map(|key| {
                let input = Dictionary::new().insert("geometry", value.clone());
                let result = retained_boolean_result(&registry, &format!("brep.measure.{key}"), input, &mut caller, &mut born, &mut free);
                read_channel_number(&result, key).unwrap()
            });
            assert!((volume - measured[0]).abs() < 1e-5);
            assert!((area - measured[1]).abs() < 1e-5);
            if operation == "brep.bool.cut" && chi == 0 {
                let fact = &cut["handledSection"];
                let origin = fixture_xyz(&fact["origin"]);
                let normal = fixture_xyz(&fact["normal"]);
                for normal in [normal, normal.map(|axis| -axis)] {
                    let input = Dictionary::new().insert("solid", value.clone())
                        .insert("planeOrigin", Value::Dictionary(point_dictionary(origin)))
                        .insert("planeNormal", Value::Dictionary(vector_dictionary(normal)));
                    let result = retained_boolean_result(&registry, "brep.intersect.section", input, &mut caller, &mut born, &mut free);
                    let faces = result.get("faces").and_then(Value::as_dictionary).unwrap();
                    assert!(faces.get("1").is_none());
                    let input = Dictionary::new().insert("geometry", faces.get("0").unwrap().clone());
                    let result = retained_boolean_result(&registry, "brep.measure.area", input, &mut caller, &mut born, &mut free);
                    assert!((read_channel_number(&result, "area").unwrap() - fact["area"].as_f64().unwrap()).abs() < 1e-6);
                }
            }
            actual.push((measured[0], measured[1], chi));
        }
        actual.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        expected.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        for ((volume, area, chi), (expected_volume, expected_area, expected_chi)) in actual.into_iter().zip(expected) {
            assert!((volume - expected_volume).abs() < 1e-9);
            assert!((area - expected_area).abs() < 1e-9);
            assert_eq!(chi, expected_chi);
        }
        caller.retire(stock());
    }
}


fn fixture_union_operands(
    registry:&Registry, inputs:&serde_json::Value, caller:&mut OriginalJobFixtureCaller,
    born:&mut usize, free:&mut usize,
)->[neural_engine::ColdOwner<Dictionary>;2]{
    ["stock","tool"].map(|name|{
        let spec=&inputs[name];
        let input=["width","depth","height"].into_iter().zip(fixture_xyz(&spec["size"]))
            .fold(Dictionary::new(),|input,(key,value)|input.insert(key,number(value)));
        let raw=retained_boolean_result(registry,"brep.prim3d.box",input,caller,born,free);
        let angles=[0,1].map(|i|spec["angles"][i].as_f64().unwrap());
        placed_finish_box(registry,raw.get("solid").unwrap(),fixture_xyz(&spec["origin"]),angles,
            spec["mirror"].as_bool().unwrap(),caller,born,free)
    })
}

#[test]
fn overlapping_skew_union_keeps_a_single_original_solid(){
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-skew-box-intersection.json")).unwrap();
    for (index,row) in fixture["overlappingUnion"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:100330+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [a,b]=fixture_union_operands(&registry,row,&mut caller,&mut born,&mut free);
        for (left,right) in [(&a,&b),(&b,&a)]{
            let input=Dictionary::new().insert("a",left.get("geometryOut").unwrap().clone())
                .insert("b",right.get("geometryOut").unwrap().clone());
            let output=retained_boolean_result(&registry,"brep.bool.fuse",input,&mut caller,&mut born,&mut free);
            let shape=read_geometry(&output,"solid").unwrap();
            session.with_kernel_read(|kernel|{
                let id=kernel.geometry_solid(&shape).unwrap();
                let body=kernel.representation();
                let reach=body.reachable_from(&[EntityRef::Solid(id)]);
                assert_eq!([reach.vertices.len(),reach.edges.len(),reach.faces.len()].map(|value|value as f64),fixture_xyz(&row["nativeTopology"]));
                for &edge in &reach.edges{
                    let uses=body.edge_coedges(edge);
                    assert_eq!(uses.len(),2);
                    assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
                }
                Ok(())
            }).unwrap();
            let mesh=run_funded_preview(&session,identity,shape.as_str(),&mut caller,&mut born,&mut free);
            mesh.validate_component_references().unwrap();
            let (area,volume)=preview_area_volume(&mesh);
            for (key,projected) in [("area",area),("volume",volume)]{
                let expected=row[key].as_f64().unwrap();
                let input=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
                let measured=retained_boolean_result(&registry,&format!("brep.measure.{key}"),input,&mut caller,&mut born,&mut free);
                assert!((read_channel_number(&measured,key).unwrap()-expected).abs()<1e-9);
                assert!((projected-expected).abs()<1e-5);
            }
        }
        caller.retire(stock());
    }
}
