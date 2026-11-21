fn shell_source_face(session:&Session,stock:&semio_framework_3d_brep_engine::GeometryHandle,index:usize)->Dictionary{
    use semio_s_spatial_kernel_semio_session::geometry_dict;
    session.with_kernel(|kernel|{
        let id=kernel.geometry_solid(stock).unwrap();
        let body=kernel.representation();
        let shell=body.solids.get(id).unwrap().outer;
        let face=body.shells.get(shell).unwrap().faces[index];
        let label=body.faces.get(face).unwrap().label;
        let handle=kernel.handle_for_label(label).unwrap();
        geometry_dict(kernel,&handle)
    }).unwrap()
}

#[test]
fn original_cylindrical_shell_keeps_native_cap_selection_and_exact_wall_topology(){
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-cylinder-shell.json")).unwrap();
    let spec=&fixture["stock"];
    let angle=spec["rotateY"].as_f64().unwrap();
    let (sin,cos)=angle.sin_cos();
    let origin=fixture_xyz(&spec["origin"]);
    let operator=fixture["operator"].as_str().unwrap();
    let cases=fixture["cases"].as_array().unwrap();
    let refused=fixture["refused"].as_array().unwrap();
    for (index,row) in cases.iter().chain(refused).enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:99610+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let raw=Dictionary::new().insert("radius",number(spec["radius"].as_f64().unwrap()))
            .insert("height",number(row["height"].as_f64().unwrap()));
        let raw=retained_boolean_result(&registry,"brep.prim3d.cylinder",raw,&mut caller,&mut born,&mut free);
        let rotation=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("axis",Value::Dictionary(vector_dictionary([0.,1.,0.]))).insert("angle",number(angle));
        let rotation=retained_boolean_result(&registry,"brep.xform.rotate",rotation,&mut caller,&mut born,&mut free);
        let translation=Dictionary::new().insert("geometry",rotation.get("geometryOut").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(origin)));
        let placed=retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
        let stock_handle=read_geometry(&placed,"geometryOut").unwrap();
        let input=Dictionary::new().insert("geometry",placed.get("geometryOut").unwrap().clone())
            .insert("thickness",number(row["thickness"].as_f64().unwrap_or_else(||fixture["thickness"].as_f64().unwrap())));
        let selected=row["open"].as_array().unwrap().iter().enumerate().fold(Dictionary::with_schema("list"),|list,(slot,key)| {
            let face=match key.as_str().unwrap(){
                "side"=>shell_source_face(&session,&stock_handle,0),
                "bottom"=>shell_source_face(&session,&stock_handle,1),
                "top"=>shell_source_face(&session,&stock_handle,2),
                "foreign"=>shell_source_face(&session,&read_geometry(&raw,"solid").unwrap(),1),
                _=>unreachable!(),
            };
            list.insert(slot.to_string(),Value::Dictionary(face))
        });
        let input=input.insert("openFaces",Value::Dictionary(selected));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.faces.len(),body.edges.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        if index==0{
            let mut cancelled=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..1024{
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                    Ok(())
                });
                assert_eq!(counts()[0],before[0]);
                if counts()[1]>before[1]{break;}
            }
            assert!(counts()[1]>before[1]);
            cancelled.as_mut().unwrap().cancel();
            close_live_job(&mut cancelled,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let mut job=Some(admit_live_job(&registry,operator,input));
        let mut finished=None;
        for _ in 0..1500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                match job.as_mut().unwrap().step(cx){
                    Ok((neural_engine::OperatorJobStep::Working(_),receipt))=>assert!(receipt.fits(grant)),
                    Ok((neural_engine::OperatorJobStep::Done(value),_))=>finished=Some(Some(neural_engine::ColdOwner::new(value))),
                    Err(_)=>finished=Some(None),
                    _=>panic!("unexpected cylindrical Shell step"),
                }
                Ok(())
            });
            if finished.is_some(){break;}
            assert_eq!(counts()[0],before[0]);
        }
        close_live_job(&mut job,&mut caller,&mut born,&mut free);
        let result=finished.expect("cylindrical Shell should reach a terminal state");
        if index>=cases.len(){
            assert!(result.is_none(),"refused stock unexpectedly produced a cylindrical shell");
            assert_eq!(counts(),before);
            caller.retire(stock());
            continue;
        }
        let result=result.unwrap();
        assert_eq!(counts()[0],before[0]+1);
        let handle=read_geometry(&result,"solid").unwrap();
        session.with_kernel_read(|kernel|{
            let id=kernel.geometry_solid(&handle).unwrap();
            let body=kernel.representation();
            let solid=body.solids.get(id).unwrap();
            assert_eq!(1+solid.inners.len(),row["shells"].as_u64().unwrap() as usize);
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            let mut euler=reach.vertices.len() as i64-reach.edges.len() as i64+reach.faces.len() as i64;
            let mut cylinders=0;
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                euler-=face.inners.len() as i64;
                match body.surfaces.get(face.surface).unwrap(){
                    Surface::Cylinder{..}=>cylinders+=1,
                    Surface::Plane{..}=>{},
                    _=>panic!("cylindrical shell unexpectedly published a different surface"),
                }
            }
            assert_eq!(cylinders,2);
            assert_eq!(euler,row["euler"].as_i64().unwrap());
            for &edge in &reach.edges{assert_eq!(body.edge_coedges(edge).len(),2);}
            Ok(())
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        let stock_mesh=run_funded_preview(&session,identity,stock_handle.as_str(),&mut caller,&mut born,&mut free);
        let actual=mesh.aabb();
        let expected=stock_mesh.aabb();
        for (a,b) in actual.0.into_iter().chain(actual.1).zip(expected.0.into_iter().chain(expected.1)){
            assert!((a-b).abs()<0.12);
        }
        let solid=result.get("solid").unwrap();
        for (name,op) in [("volume","brep.measure.volume"),("area","brep.measure.area")]{
            let measured=retained_boolean_result(&registry,op,Dictionary::new().insert("geometry",solid.clone()),&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&measured,name).unwrap()-row[name].as_f64().unwrap()).abs()<1e-10);
        }
        for probe in row["classify"].as_array().unwrap(){
            let [x,y,z]=fixture_xyz(&probe[0]);
            let world=[origin[0]+cos*x+sin*z,origin[1]+y,origin[2]-sin*x+cos*z];
            let query=Dictionary::new().insert("solid",solid.clone()).insert("point",Value::Dictionary(point_dictionary(world)));
            let measured=retained_boolean_result(&registry,"brep.measure.classify",query,&mut caller,&mut born,&mut free);
            assert_eq!(read_channel_number(&measured,"classification").unwrap(),probe[1].as_f64().unwrap());
        }
        caller.retire(stock());
    }
}
