#[test]
fn original_oriented_box_fillet_keeps_the_selected_native_edge_and_reciprocal_trims(){
    check_oriented_box_finish(include_str!("🔣️original-oriented-fillet.json"),"radius");
}

#[test]
fn original_oriented_box_chamfer_keeps_the_exact_bevel_plane_and_reciprocal_trims(){
    check_oriented_box_finish(include_str!("🔣️original-oriented-chamfer.json"),"distance");
}

fn check_oriented_box_finish(fixture_text:&str,parameter:&str){
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,text_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(fixture_text).unwrap();
    let fillet=parameter=="radius";
    let spec=&fixture["stock"];
    let size=fixture_xyz(&spec["size"]);
    let origin=fixture_xyz(&spec["origin"]);
    let angle=spec["rotateY"].as_f64().unwrap();
    let (s,c)=angle.sin_cos();
    let cases=fixture["cases"].as_array().unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    for (index,row) in cases.iter().chain(fixture["refused"].as_array().unwrap()).enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:99920+index as u64+if fillet{0}else{32},generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let input=["width","depth","height"].into_iter().zip(size).fold(Dictionary::new(),|input,(key,span)|input.insert(key,number(span)));
        let raw=retained_boolean_result(&registry,"brep.prim3d.box",input,&mut caller,&mut born,&mut free);
        let pitch=row["rotateX"].as_f64().unwrap_or(0.);
        let (sp,cp)=pitch.sin_cos();
        let mirrored=row["mirror"].as_bool().unwrap_or(false);
        let placed=placed_finish_box(&registry,raw.get("solid").unwrap(),origin,[angle,pitch],mirrored,&mut caller,&mut born,&mut free);
        let original=read_geometry(&placed,"geometryOut").unwrap();
        let sign=if mirrored{-1.}else{1.};
        let lift=|[x,y,z]:[f64;3]|{
            let pitch_axis=-s*x+c*z;
            Pnt3::from_array([origin[0]+sign*(c*x+s*z),origin[1]+cp*y-sp*pitch_axis,origin[2]+sp*y+cp*pitch_axis])
        };
        let foreign=row["foreign"].as_bool().unwrap_or(false);
        let (selector_value,selector_handle)=if foreign{(raw.get("solid").unwrap(),read_geometry(&raw,"solid").unwrap())}
            else{(placed.get("geometryOut").unwrap(),original.clone())};
        let expected=row["edges"].as_array().unwrap();
        let labels=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&selector_handle).unwrap();
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            let labels=expected.iter().map(|edge|{
                let [a,b]=[&edge[0],&edge[1]].map(|json|{
                    let p=fixture_xyz(json);
                    if foreign{Pnt3::from_array(p)}else{lift(p)}
                });
                reach.edges.iter().find_map(|&id|{
                    let edge=body.edges.get(id)?;
                    let p=body.vertices.get(edge.v0)?.position;
                    let q=body.vertices.get(edge.v1)?.position;
                    let matches=(p.distance(a)<1e-10&&q.distance(b)<1e-10)||(p.distance(b)<1e-10&&q.distance(a)<1e-10);
                    matches.then_some(edge.label.0.to_string())
                }).unwrap()
            }).collect::<Vec<_>>();
            Ok(labels)
        }).unwrap();
        let pick=Dictionary::new().insert("brep",selector_value.clone())
            .insert("edgeLabels",Value::Dictionary(text_dictionary(serde_json::to_string(&labels).unwrap())))
            .insert("faceLabels",Value::Dictionary(text_dictionary("[]".to_owned())))
            .insert("sourceHandle",Value::Dictionary(text_dictionary(String::new())));
        let selection=retained_boolean_result(&registry,"brep.brep",pick,&mut caller,&mut born,&mut free);
        let amount=row[parameter].as_f64().unwrap_or_else(||fixture[parameter].as_f64().unwrap());
        let input=Dictionary::new().insert("geometry",placed.get("geometryOut").unwrap().clone())
            .insert("edges",selection.get("selectedEdges").unwrap().clone()).insert(parameter,number(amount));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.faces.len(),body.edges.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
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
        if index==0{
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
        let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        assert_eq!(counts()[0],before[0]+1);
        let handle=read_geometry(&result,"solid").unwrap();
        let axis=row["axis"].as_u64().unwrap() as usize;
        let direction=lift(std::array::from_fn(|i|if i==axis{1.}else{0.}))-lift([0.;3]);
        let source_corner=fixture_xyz(&expected[0][0]);
        let bevel_local=std::array::from_fn(|i|if i==axis{0.}else if source_corner[i]==0.{-1.}else{1.});
        let bevel_normal=(lift(bevel_local)-lift([0.;3])).normalized().unwrap();
        let (arc_labels,face_labels)=session.with_kernel_read(|kernel|{
            assert!(kernel.geometry_solid(&original).is_ok());
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            let n=4+expected.len();
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),(2*n,3*n,n+2));
            let mut arcs=Vec::new();
            for &id in &reach.edges{
                let edge=body.edges.get(id).unwrap();
                let uses=body.edge_coedges(id);
                assert_eq!(uses.len(),2);
                assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
                if let Curve3::Circle{radius:actual,..}=body.curves3.get(edge.curve).unwrap(){
                    assert_eq!(*actual,amount);
                    assert_eq!(edge.range,(0.,std::f64::consts::FRAC_PI_2));
                    arcs.push(edge.label.0.to_string());
                }
            }
            let mut faces=Vec::new();
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                match body.surfaces.get(face.surface).unwrap(){
                    Surface::Cylinder{frame,radius:actual} if fillet=>{
                        assert!(!face.flipped);
                        assert_eq!(*actual,amount);
                        assert!((frame.z-direction).norm()<1e-10);
                        faces.push(face.label.0.to_string());
                    }
                    Surface::Plane{frame}=>{
                        let normal=if face.flipped{-frame.z}else{frame.z};
                        if !fillet&&(normal-bevel_normal).norm()<1e-10{
                            let distance=(lift(source_corner)-frame.origin).dot(normal);
                            assert!((distance-amount*std::f64::consts::FRAC_1_SQRT_2).abs()<1e-10);
                            faces.push(face.label.0.to_string());
                        }
                    }
                    _=>panic!("selected finish published an unexpected analytic surface"),
                }
            }
            assert_eq!(arcs.len(),if fillet{2*expected.len()}else{0});
            assert_eq!(faces.len(),expected.len());
            Ok((arcs,faces))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(arc_labels.iter().all(|label|mesh.component_references["edge"].contains(label)));
        assert!(face_labels.iter().all(|label|mesh.component_references["face"].contains(label)));
        let (area,volume)=preview_area_volume(&mesh);
        let tolerance=if fillet{0.2}else{1e-4};
        assert!((area-row["area"].as_f64().unwrap()).abs()<tolerance);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<tolerance);
        if fillet{
            for (key,op) in [("volume","brep.measure.volume"),("area","brep.measure.area")]{
                let input=Dictionary::new().insert("geometry",result.get("solid").unwrap().clone());
                let measured=retained_boolean_result(&registry,op,input,&mut caller,&mut born,&mut free);
                assert!((read_channel_number(&measured,key).unwrap()-row[key].as_f64().unwrap()).abs()<1e-10);
            }
        }
        caller.retire(stock());
    }
}




#[test]
fn original_oriented_all_edge_box_chamfer_publishes_exact_bevel_and_corner_planes(){
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3};
    use semio_s_spatial_kernel_semio_session::read_geometry;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-box-all-edge-chamfer.json")).unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    for (index,row) in fixture["orientedCases"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:99985+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let size=fixture_xyz(&row["size"]);
        let origin=fixture_xyz(&row["origin"]);
        let angles=[row["rotateY"].as_f64().unwrap(),row["rotateX"].as_f64().unwrap()];
        let mirrored=row["mirror"].as_bool().unwrap();
        let input=["width","depth","height"].into_iter().zip(size)
            .fold(Dictionary::new(),|input,(key,span)|input.insert(key,number(span)));
        let raw=retained_boolean_result(&registry,"brep.prim3d.box",input,&mut caller,&mut born,&mut free);
        let placed=placed_finish_box(&registry,raw.get("solid").unwrap(),origin,angles,mirrored,&mut caller,&mut born,&mut free);
        let stock_handle=read_geometry(&placed,"geometryOut").unwrap();
        let input=|distance|Dictionary::new().insert("geometry",placed.get("geometryOut").unwrap().clone())
            .insert("distance",number(distance));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for refused in row["refused"].as_array().unwrap(){
            let mut job=Some(admit_live_job(&registry,operator,input(refused.as_f64().unwrap())));
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                assert!(job.as_mut().unwrap().step(cx).is_err());
                Ok(())
            });
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let distance=row["distance"].as_f64().unwrap();
        let mut cancelled=Some(admit_live_job(&registry,operator,input(distance)));
        for _ in 0..1000{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                Ok(())
            });
            assert_eq!(counts()[0],before[0]);
            if counts()[3]>before[3]{break;}
        }
        assert!(counts()[3]>before[3]);
        cancelled.as_mut().unwrap().cancel();
        close_live_job(&mut cancelled,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let result=retained_boolean_result(&registry,operator,input(distance),&mut caller,&mut born,&mut free);
        let handle=read_geometry(&result,"solid").unwrap();
        assert_eq!(counts(),[before[0]+1,before[1]+24,before[2]+48,before[3]+26,before[4]+1]);
        let (s,c)=angles[0].sin_cos();
        let (sp,cp)=angles[1].sin_cos();
        let sign=if mirrored{-1.}else{1.};
        let basis=[Vec3::new(sign*c,sp*s,-cp*s),Vec3::new(0.,cp,sp),Vec3::new(sign*s,-sp*c,cp*c)];
        let center=(0..3).fold(Pnt3::from_array(origin),|p,i|p+basis[i]*(size[i]*0.5));
        session.with_kernel_read(|kernel|{
            assert!(kernel.geometry_solid(&stock_handle).is_ok());
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            let mut planes=[0;3];
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{panic!("expected an exact plane");};
                assert!(!face.flipped);
                assert!((center-frame.origin).dot(frame.z)<0.);
                let local=basis.map(|axis|axis.dot(frame.z).abs());
                let used=local.into_iter().filter(|value|*value>1e-10).count();
                assert!(local.into_iter().filter(|value|*value>1e-10).all(|value|(value-(used as f64).sqrt().recip()).abs()<1e-10));
                planes[used-1]+=1;
            }
            assert_eq!(planes,[6,12,8]);
            for &id in &reach.edges{
                let edge=body.edges.get(id).unwrap();
                assert!(matches!(body.curves3.get(edge.curve),Some(Curve3::Line{..})));
                let uses=body.edge_coedges(id);
                assert_eq!(uses.len(),2);
                assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
            }
            Ok(())
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["face"].len(),26);
        assert_eq!(mesh.component_references["edge"].len(),48);
        let (min,max)=mesh.aabb();
        for (i,actual) in [min,max].into_iter().enumerate(){
            assert!(actual.into_iter().zip(fixture_xyz(&row["bounds"][i])).all(|(a,b)|(f64::from(a)-b).abs()<1e-6));
        }
        let (area,volume)=preview_area_volume(&mesh);
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-4);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-4);
        caller.retire(stock());
    }
}
