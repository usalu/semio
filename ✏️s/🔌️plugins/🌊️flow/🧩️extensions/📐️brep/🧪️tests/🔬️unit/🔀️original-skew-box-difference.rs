#[test]
fn original_skew_box_cut_keeps_exact_complements_cavities_and_sewn_recesses(){
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3};
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-skew-box-difference.json")).unwrap();
    let references:serde_json::Value=serde_json::from_str(include_str!("🔣️original-skew-box-intersection.json")).unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    let cases=fixture["cases"].as_array().unwrap();
    for (index,row) in cases.iter().chain(fixture["refused"].as_array().unwrap()).enumerate(){
        let reverse=row["reverse"].as_bool().unwrap_or(false);
        let resolved=row["reference"].as_array().map_or(row,|keys|{
            let source=if reverse{&fixture}else{&references};
            &source[keys[0].as_str().unwrap()][keys[1].as_u64().unwrap() as usize]
        });
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:100310+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let keys=if reverse{["tool","stock"]}else{["stock","tool"]};
        let [stock_box,tool_box]=keys.map(|key|{
            let spec=&resolved[key];
            let input=["width","depth","height"].into_iter().zip(fixture_xyz(&spec["size"]))
                .fold(Dictionary::new(),|input,(axis,span)|input.insert(axis,number(span)));
            let raw=retained_boolean_result(&registry,"brep.prim3d.box",input,&mut caller,&mut born,&mut free);
            let angles=[0,1].map(|i|spec["angles"][i].as_f64().unwrap());
            placed_finish_box(&registry,raw.get("solid").unwrap(),fixture_xyz(&spec["origin"]),angles,spec["mirror"].as_bool().unwrap(),
                &mut caller,&mut born,&mut free)
        });
        let input=Dictionary::new().insert("a",stock_box.get("geometryOut").unwrap().clone())
            .insert("b",tool_box.get("geometryOut").unwrap().clone());
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.shells.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let kind=row["kind"].as_str().unwrap();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        if matches!(kind,"cavity"|"recess"){
            let shells=if kind=="cavity"{2}else{1};
            let mut cancelled=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..1800{
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                    Ok(())
                });
                assert_eq!(counts()[0],before[0]);
                if counts()[4]==before[4]+shells{break;}
            }
            assert_eq!(counts()[4],before[4]+shells);
            cancelled.as_mut().unwrap().cancel();
            close_live_job(&mut cancelled,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let mut job=Some(admit_live_job(&registry,operator,input));
        let mut terminal=None;
        for _ in 0..1800{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                match job.as_mut().unwrap().step(cx){
                    Ok((neural_engine::OperatorJobStep::Working(_),receipt))=>assert!(receipt.fits(grant)),
                    Ok((neural_engine::OperatorJobStep::Done(value),_))=>terminal=Some(Ok(neural_engine::ColdOwner::new(value))),
                    Err(error)=>terminal=Some(Err(format!("{error:?}"))),
                    _=>panic!("unexpected skew Cut job state"),
                }
                Ok(())
            });
            if terminal.is_some(){break;}
            assert_eq!(counts()[0],before[0]);
        }
        close_live_job(&mut job,&mut caller,&mut born,&mut free);
        let terminal=terminal.unwrap();
        if index>=cases.len(){
            let error=terminal.err().unwrap();
            assert!(error.contains(if kind=="empty"{"empty"}else{"UnsupportedOwner"}),"{error}");
            assert_eq!(counts(),before);
            caller.retire(stock());
            continue;
        }
        let result=terminal.unwrap();
        let handle=read_geometry(&result,"solid").unwrap();
        assert_eq!(counts()[0],before[0]+1);
        let cap=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            let solid=body.solids.get(id).unwrap();
            assert_eq!(solid.inners.len(),usize::from(kind=="cavity"));
            let euler=reach.vertices.len() as i64-reach.edges.len() as i64+reach.faces.len() as i64;
            assert_eq!(euler,row["euler"].as_i64().unwrap());
            for &edge in &reach.edges{
                let uses=body.edge_coedges(edge);
                assert_eq!(uses.len(),2);
                assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
            }
            if kind=="cavity"{
                for (key,count) in [("vertices",reach.vertices.len()),("edges",reach.edges.len()),("faces",reach.faces.len())]{
                    assert_eq!(count as u64,row[key].as_u64().unwrap());
                }
                assert_eq!(body.shells.get(solid.outer).unwrap().faces.len(),6);
                let inner=body.shells.get(solid.inners[0]).unwrap();
                assert_eq!(inner.faces.len(),6);
                let cutter=read_geometry(&tool_box,"geometryOut").unwrap();
                let cutter_id=kernel.geometry_solid(&cutter).unwrap();
                let cutter=body.shells.get(body.solids.get(cutter_id).unwrap().outer).unwrap();
                for &id in &inner.faces{
                    let face=body.faces.get(id).unwrap();
                    let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{unreachable!();};
                    assert!(cutter.faces.iter().any(|&source|{
                        let source=body.faces.get(source).unwrap();
                        let Some(Surface::Plane{frame:tool})=body.surfaces.get(source.surface)else{return false;};
                        (tool.z+frame.z).norm()<1e-10&&(tool.origin-frame.origin).dot(frame.z).abs()<1e-10
                    }));
                }
                return Ok(None);
            }
            if kind=="recess"{
                assert_eq!([reach.vertices.len(),reach.edges.len(),reach.faces.len()].map(|n| n as f64),fixture_xyz(&row["nativeTopology"]));
                let cutter=read_geometry(&tool_box,"geometryOut").unwrap();
                let cutter=body.solids.get(kernel.geometry_solid(&cutter).unwrap()).unwrap();
                let mut walls=Vec::new();
                for &id in &reach.faces{
                    let face=body.faces.get(id).unwrap();
                    let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{unreachable!();};
                    let inward=body.shells.get(cutter.outer).unwrap().faces.iter().any(|&source|{
                        let source=body.faces.get(source).unwrap();
                        let Some(Surface::Plane{frame:tool})=body.surfaces.get(source.surface)else{return false;};
                        (tool.z+frame.z).norm()<1e-10&&(tool.origin-frame.origin).dot(frame.z).abs()<1e-10
                    });
                    if inward{walls.push(face.label.0.to_string());}
                }
                assert_eq!(walls.len(),row["cutterFaces"].as_array().map_or(2,Vec::len));
            }
            if kind!="clip"{return Ok(None);}
            let origin=Pnt3::from_array(fixture_xyz(&resolved["capOrigin"]));
            let normal=-Vec3::from_array(fixture_xyz(&resolved["capNormal"]));
            let cap=reach.faces.iter().find_map(|&id|{
                let face=body.faces.get(id)?;
                let Surface::Plane{frame}=body.surfaces.get(face.surface)?else{return None;};
                ((frame.z-normal).norm()<1e-10&&(frame.origin-origin).dot(normal).abs()<1e-10).then_some(face.label.0.to_string())
            }).unwrap();
            Ok(Some(cap))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        if kind=="recess"{
            let (area,volume)=preview_area_volume(&mesh);
            assert!((area-row["area"].as_f64().unwrap()).abs()<1e-5);
            assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-5);
        }
        if let Some(cap)=cap{assert!(mesh.component_references["face"].contains(&cap));}
        let (min,max)=mesh.aabb();
        for (side,values) in [min,max].into_iter().enumerate(){
            assert!(values.into_iter().zip(fixture_xyz(&row["bounds"][side])).all(|(a,b)|(f64::from(a)-b).abs()<1e-6));
        }
        let solid=result.get("solid").unwrap();
        for (key,op) in [("area","brep.measure.area"),("volume","brep.measure.volume")]{
            let input=Dictionary::new().insert("geometry",solid.clone());
            let measured=retained_boolean_result(&registry,op,input,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&measured,key).unwrap()-row[key].as_f64().unwrap()).abs()<1e-10);
        }
        if row["center"].is_array(){
            let input=Dictionary::new().insert("geometry",solid.clone());
            let output=retained_boolean_result(&registry,"brep.measure.centerOfMass",input,&mut caller,&mut born,&mut free);
            let point=output.get("center").and_then(Value::as_dictionary).unwrap();
            for (key,expected) in ["x","y","z"].into_iter().zip(fixture_xyz(&row["center"])){
                let coordinate=point.get(key).and_then(Value::as_atom).and_then(|atom|atom.as_f64()).unwrap();
                assert!((coordinate-expected).abs()<1e-7);
            }
        }
        for fact in ["section","vertexSection","edgeSection","coincidentFloor","voidSection","connectedVoidSection"].into_iter().filter_map(|key|row[key].as_object()){
            let origin=fixture_xyz(&fact["origin"]);
            let normal=fixture_xyz(&fact["normal"]);
            for normal in [normal,normal.map(|component|-component)]{
                let input=Dictionary::new().insert("solid",solid.clone())
                    .insert("planeOrigin",Value::Dictionary(point_dictionary(origin)))
                    .insert("planeNormal",Value::Dictionary(vector_dictionary(normal)));
                if fact.contains_key("ringAreas") && normal == fixture_xyz(&fact["normal"]){
                    let topology_counts=||session.with_kernel_read(|kernel|{
                        let body=kernel.representation();
                        Ok([body.vertices.len(),body.edges.len(),body.loops.len(),body.coedges.len(),body.faces.len(),
                            body.surfaces.len(),body.curves3.len(),body.curves2.len()])
                    }).unwrap();
                    let before=topology_counts();
                    let mut cancelled=Some(admit_live_job(&registry,"brep.intersect.section",input.clone()));
                    for _ in 0..1024{
                        run(&mut caller,grant,&mut born,&mut free,|cx|{
                            assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                            Ok(())
                        });
                        if topology_counts()[2] == before[2] + 2 { break; }
                    }
                    assert_eq!(topology_counts()[2],before[2]+2);
                    cancelled.as_mut().unwrap().cancel();
                    close_live_job(&mut cancelled,&mut caller,&mut born,&mut free);
                    assert_eq!(topology_counts(),before);
                }
                let section=retained_boolean_result(&registry,"brep.intersect.section",input,&mut caller,&mut born,&mut free);
                let faces=section.get("faces").and_then(Value::as_dictionary).unwrap();
                assert!(faces.get("1").is_none());
                if fact.contains_key("ringAreas"){
                    let handle=read_geometry(faces,"0").unwrap();
                    let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
                    mesh.validate_component_references().unwrap();
                    let labels=&mesh.component_references["face"];
                    assert_eq!(labels.len(),1);
                    let label=labels.iter().next().unwrap().parse::<u64>().unwrap();
                    session.with_kernel_read(|kernel|{
                        let body=kernel.representation();
                        let (_,record)=body.faces.iter().find(|(_,record)|record.label.0==label).unwrap();
                        assert_eq!(record.inners.len(),1);
                        Ok(())
                    }).unwrap();
                    assert!((preview_area_volume(&mesh).0-fact["area"].as_f64().unwrap()).abs()<1e-5);
                }
                let input=Dictionary::new().insert("geometry",faces.get("0").unwrap().clone());
                let measured=retained_boolean_result(&registry,"brep.measure.area",input,&mut caller,&mut born,&mut free);
                assert!((read_channel_number(&measured,"area").unwrap()-fact["area"].as_f64().unwrap()).abs()<1e-6);
            }
        }
        let classify=if kind=="clip"{&resolved["classify"]}else{&row["classify"]};
        if let Some(probes)=classify.as_array(){
            for probe in probes{
                let query=Dictionary::new().insert("solid",solid.clone())
                    .insert("point",Value::Dictionary(point_dictionary(fixture_xyz(&probe[0]))));
                let actual=retained_boolean_result(&registry,"brep.measure.classify",query,&mut caller,&mut born,&mut free);
                let positive=probe[1].as_f64().unwrap();
                assert_eq!(read_channel_number(&actual,"classification").unwrap(),if kind=="clip"&&positive!=2.{1.-positive}else{positive});
            }
        }
        caller.retire(stock());
    }
}
