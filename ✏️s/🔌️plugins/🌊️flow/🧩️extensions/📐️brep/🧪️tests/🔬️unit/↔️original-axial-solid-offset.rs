#[test]
fn original_axial_solid_offset_displaces_exact_cylinder_and_cone_supports(){
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,read_xyz,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-axial-solid-offset.json")).unwrap();
    let spec=&fixture["stock"];
    let angle=spec["rotateY"].as_f64().unwrap();
    let (s,c)=angle.sin_cos();
    let origin=fixture_xyz(&spec["origin"]);
    let world=|[x,y,z]:[f64;3]|[origin[0]+c*x+s*z,origin[1]+y,origin[2]-s*x+c*z];
    let n=[s,0.,c];
    let cases=fixture["cases"].as_array().unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    for (index,row) in cases.iter().chain(fixture["refused"].as_array().unwrap()).enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:99800+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let kind=row["kind"].as_str().unwrap();
        let make=if kind=="cylinder"{"brep.prim3d.cylinder"}else{"brep.prim3d.cone"};
        let input=Dictionary::new().insert("radius",number(spec["radius"].as_f64().unwrap()))
            .insert("height",number(spec["height"].as_f64().unwrap()));
        let raw=retained_boolean_result(&registry,make,input,&mut caller,&mut born,&mut free);
        let input=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("axis",Value::Dictionary(vector_dictionary([0.,1.,0.]))).insert("angle",number(angle));
        let tilted=retained_boolean_result(&registry,"brep.xform.rotate",input,&mut caller,&mut born,&mut free);
        let input=Dictionary::new().insert("geometry",tilted.get("geometryOut").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(origin)));
        let placed=retained_boolean_result(&registry,"brep.xform.translate",input,&mut caller,&mut born,&mut free);
        let mut source=placed.get("geometryOut").unwrap().clone();
        if kind=="frustum"{
            let cut=row["cut"].as_f64().unwrap_or_else(||spec["cut"].as_f64().unwrap());
            let input=Dictionary::new().insert("solid",source)
                .insert("planeOrigin",Value::Dictionary(point_dictionary(world([0.,0.,cut]))))
                .insert("planeNormal",Value::Dictionary(vector_dictionary(n)));
            let output=retained_boolean_result(&registry,"brep.intersect.split",input,&mut caller,&mut born,&mut free);
            source=output.get("negative").unwrap().clone();
        }
        let input=Dictionary::new().insert("geometry",source).insert("distance",number(row["distance"].as_f64().unwrap()));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.faces.len(),body.edges.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        if index==4{
            let mut cancelled=Some(admit_live_job(&registry,operator,input.clone()));
            for _ in 0..1000{
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
        if index>=cases.len(){
            let mut refused=Some(admit_live_job(&registry,operator,input));
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                assert!(refused.as_mut().unwrap().step(cx).is_err());
                Ok(())
            });
            close_live_job(&mut refused,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
            caller.retire(stock());
            continue;
        }
        let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        assert_eq!(counts()[0],before[0]+1);
        let handle=read_geometry(&result,"solid").unwrap();
        session.with_kernel_read(|kernel|{
            let id=kernel.geometry_solid(&handle).unwrap();
            let body=kernel.representation();
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            let cap_count=if kind=="cone"{1}else{2};
            assert_eq!(reach.faces.len(),1+cap_count);
            let mut side=0;
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                match body.surfaces.get(face.surface).unwrap(){
                    Surface::Plane{..}=>{},
                    Surface::Cylinder{..} if kind=="cylinder"=>side+=1,
                    Surface::Cone{..} if kind!="cylinder"=>side+=1,
                    _=>panic!("offset changed the exact surface family"),
                }
            }
            assert_eq!(side,1);
            let mut rims=Vec::new();
            for &id in &reach.edges{
                let edge=body.edges.get(id).unwrap();
                assert_eq!(body.edge_coedges(id).len(),2);
                if let Curve3::Circle{frame,radius}=body.curves3.get(edge.curve).unwrap(){
                    let point=frame.origin.to_array();
                    let z=(0..3).map(|i|(point[i]-origin[i])*n[i]).sum::<f64>();
                    rims.push((z,*radius));
                }
            }
            rims.sort_unstable_by(|a,b|a.0.total_cmp(&b.0));
            assert_eq!(rims.len(),cap_count);
            for (i,(level,radius)) in rims.into_iter().enumerate(){
                assert!((level-row["levels"][i].as_f64().unwrap()).abs()<1e-10);
                assert!((radius-row["radii"][i].as_f64().unwrap()).abs()<1e-10);
            }
            Ok(())
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        let (min,max)=mesh.aabb();
        for (i,actual) in [min,max].into_iter().enumerate(){
            assert!(actual.into_iter().zip(fixture_xyz(&row["bounds"][i])).all(|(a,b)|(f64::from(a)-b).abs()<0.09));
        }
        let solid=result.get("solid").unwrap();
        for (key,op) in [("volume","brep.measure.volume"),("area","brep.measure.area"),("centroid","brep.measure.centerOfMass")]{
            let input=Dictionary::new().insert("geometry",solid.clone());
            let measured=retained_boolean_result(&registry,op,input,&mut caller,&mut born,&mut free);
            if key=="centroid"{
                assert!(read_xyz(&measured,"center").unwrap().into_iter().zip(fixture_xyz(&row[key])).all(|(a,b)|(a-b).abs()<1e-10));
            }else{
                assert!((read_channel_number(&measured,key).unwrap()-row[key].as_f64().unwrap()).abs()<1e-10);
            }
        }
        for probe in row["classify"].as_array().unwrap(){
            let input=Dictionary::new().insert("solid",solid.clone())
                .insert("point",Value::Dictionary(point_dictionary(world(fixture_xyz(&probe[0])))));
            let value=retained_boolean_result(&registry,"brep.measure.classify",input,&mut caller,&mut born,&mut free);
            assert_eq!(read_channel_number(&value,"classification").unwrap(),probe[1].as_f64().unwrap());
        }
        caller.retire(stock());
    }
}
