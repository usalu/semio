#[test]
fn original_oriented_box_shell_preserves_caps_and_closed_material_in_the_authored_frame(){
    use semio_os_flow_geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3};
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-oriented-box-shell.json")).unwrap();
    let spec=&fixture["stock"];
    let size=fixture_xyz(&spec["size"]);
    let origin=fixture_xyz(&spec["origin"]);
    let yaw=spec["rotateY"].as_f64().unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    let cases=fixture["cases"].as_array().unwrap();
    let refused=fixture["refused"].as_array().unwrap();
    let directions=["-x","+x","-y","+y","-z","+z"];
    for (index,row) in cases.iter().chain(refused).enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:100100+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let input=["width","depth","height"].into_iter().zip(size)
            .fold(Dictionary::new(),|input,(key,span)|input.insert(key,number(span)));
        let raw=retained_boolean_result(&registry,"brep.prim3d.box",input,&mut caller,&mut born,&mut free);
        let pitch=row["rotateX"].as_f64().unwrap_or(0.37);
        let mirror=row["mirror"].as_bool().unwrap_or(true);
        let placed=placed_finish_box(&registry,raw.get("solid").unwrap(),origin,[yaw,pitch],mirror,&mut caller,&mut born,&mut free);
        let source=read_geometry(&placed,"geometryOut").unwrap();
        let (s,c)=yaw.sin_cos();
        let (sp,cp)=pitch.sin_cos();
        let sign=if mirror{-1.}else{1.};
        let basis=[Vec3::new(sign*c,sp*s,-cp*s),Vec3::new(0.,cp,sp),Vec3::new(sign*s,-sp*c,cp*c)];
        let lift=|[x,y,z]:[f64;3]|Pnt3::from_array(origin)+basis[0]*x+basis[1]*y+basis[2]*z;
        let open=row["open"].as_array().unwrap();
        let foreign=row["foreign"].as_bool().unwrap_or(false);
        let selected=open.iter().enumerate().fold(Dictionary::with_schema("list"),|list,(slot,side)|{
            let index=directions.iter().position(|&name|Some(name)==side.as_str()).unwrap();
            let normal=basis[index/2]*if index%2==0{-1.}else{1.};
            let value=if foreign{
                shell_source_face(&session,&read_geometry(&raw,"solid").unwrap(),1)
            }else{
                let ordinal=session.with_kernel_read(|kernel|{
                    let body=kernel.representation();
                    let stock=body.solids.get(kernel.geometry_solid(&source).unwrap()).unwrap();
                    let outer=body.shells.get(stock.outer).unwrap();
                    Ok(outer.faces.iter().position(|&id|{
                        let face=body.faces.get(id).unwrap();
                        let Some(Surface::Plane{frame})=body.surfaces.get(face.surface)else{return false;};
                        let n=if face.flipped{-frame.z}else{frame.z};
                        (n-normal).norm()<1e-10
                    }).unwrap())
                }).unwrap();
                shell_source_face(&session,&source,ordinal)
            };
            list.insert(slot.to_string(),Value::Dictionary(value))
        });
        let thickness=row["thickness"].as_f64().unwrap_or_else(||fixture["thickness"].as_f64().unwrap());
        let input=Dictionary::new().insert("geometry",placed.get("geometryOut").unwrap().clone())
            .insert("thickness",number(thickness)).insert("openFaces",Value::Dictionary(selected));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        if index>=cases.len(){
            let mut job=Some(admit_live_job(&registry,operator,input));
            let mut rejected=false;
            for _ in 0..96{
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    rejected=job.as_mut().unwrap().step(cx).is_err();
                    Ok(())
                });
                if rejected{break;}
            }
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert!(rejected);
            assert_eq!(counts(),before);
            caller.retire(stock());
            continue;
        }
        if index==0||open.len()==2{
            let mut job=Some(admit_live_job(&registry,operator,input.clone()));
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
        let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&result,"solid").unwrap();
        assert_eq!(counts()[0],before[0]+1);
        session.with_kernel_read(|kernel|{
            assert!(kernel.geometry_solid(&source).is_ok());
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let stock=body.solids.get(id).unwrap();
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            assert_eq!(1+stock.inners.len(),row["shells"].as_u64().unwrap() as usize);
            let euler=reach.vertices.len() as i64-reach.edges.len() as i64+reach.faces.len() as i64;
            assert_eq!(euler,row["euler"].as_i64().unwrap());
            for &face in &reach.faces{
                let face=body.faces.get(face).unwrap();
                let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{panic!("invalid shell support");};
                assert_eq!(basis.into_iter().filter(|axis|axis.dot(frame.z).abs()>1e-10).count(),1);
            }
            for &edge in &reach.edges{
                let uses=body.edge_coedges(edge);
                assert_eq!(uses.len(),2);
                assert_ne!(body.coedges.get(uses[0]).unwrap().forward,body.coedges.get(uses[1]).unwrap().forward);
            }
            Ok(())
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        let (min,max)=mesh.aabb();
        for (side,actual) in [min,max].into_iter().enumerate(){
            assert!(actual.into_iter().zip(fixture_xyz(&row["bounds"][side])).all(|(a,b)|(f64::from(a)-b).abs()<1e-6));
        }
        let (area,volume)=preview_area_volume(&mesh);
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-4);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-4);
        let solid=result.get("solid").unwrap();
        for (name,op) in [("area","brep.measure.area"),("volume","brep.measure.volume")]{
            let input=Dictionary::new().insert("geometry",solid.clone());
            let query=retained_boolean_result(&registry,op,input,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&query,name).unwrap()-row[name].as_f64().unwrap()).abs()<1e-10);
        }
        for probe in row["classify"].as_array().unwrap(){
            let point=lift(fixture_xyz(&probe[0])).to_array();
            let input=Dictionary::new().insert("solid",solid.clone()).insert("point",Value::Dictionary(point_dictionary(point)));
            let query=retained_boolean_result(&registry,"brep.measure.classify",input,&mut caller,&mut born,&mut free);
            assert_eq!(read_channel_number(&query,"classification").unwrap(),probe[1].as_f64().unwrap());
        }
        caller.retire(stock());
    }
}
