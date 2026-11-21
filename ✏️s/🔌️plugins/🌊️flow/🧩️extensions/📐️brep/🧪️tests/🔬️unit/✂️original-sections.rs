//! ✂️ Original analytic solid sections and selected face preview.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn original_oblique_box_split_publishes_two_exact_signed_planar_solids() {
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3};
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-oblique-box-split.json")).unwrap();
    let operator=fixture["operator"].as_str().unwrap();
    for (index,row) in fixture["cases"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=flow_extension_sdk::geometry::GeometryWorkIdentity{operation:99180+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let size=fixture_xyz(&row["size"]);
        let input=["width","depth","height"].into_iter().zip(size)
            .fold(Dictionary::new(),|input,(key,size)|input.insert(key,number(size)));
        let raw=run_live_query(&registry,"brep.prim3d.box",input,&mut caller,&mut born,&mut free);
        let angle=row["rotationDegrees"].as_f64().unwrap().to_radians();
        let rotation=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("angle",number(angle)).insert("axis",Value::Dictionary(vector_dictionary([0.,0.,1.])));
        let rotated=run_live_query(&registry,"brep.xform.rotate",rotation,&mut caller,&mut born,&mut free);
        let base=fixture_xyz(&row["origin"]);
        let translation=Dictionary::new().insert("geometry",rotated.get("geometryOut").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(base)));
        let stock_geometry=run_live_query(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
        let (s,c)=angle.sin_cos();
        let rotate=|[x,y,z]|[c*x-s*y,s*x+c*y,z];
        let p=rotate(fixture_xyz(&row["point"]));
        let origin=std::array::from_fn(|i|p[i]+base[i]);
        let normal=rotate(fixture_xyz(&row["normal"]));
        let input=|p,n|Dictionary::new().insert("solid",stock_geometry.get("geometryOut").unwrap().clone())
            .insert("planeOrigin",Value::Dictionary(point_dictionary(p)))
            .insert("planeNormal",Value::Dictionary(vector_dictionary(n)));
        let counts=||session.with_kernel_read(|kernel|{
            let b=kernel.representation();
            Ok([kernel.live_handles().len(),b.vertices.len(),b.edges.len(),b.faces.len(),b.solids.len()])
        }).unwrap();
        let before=counts();
        let mut cancelled=Some(admit_live_job(&registry,operator,input(origin,normal)));
        let grant=Grant{maximum_items:1,..stock()};
        for _ in 0..2500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=cancelled.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            assert_eq!(counts()[0],before[0]);
            if counts()[4]==before[4]+2{break;}
        }
        assert_eq!(counts()[4],before[4]+2);
        cancelled.as_mut().unwrap().cancel();
        close_live_job(&mut cancelled,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        for sign in [1.,-1.]{
            let before=counts();
            let normal=normal.map(|n|sign*n);
            let result=run_live_query(&registry,operator,input(origin,normal),&mut caller,&mut born,&mut free);
            assert_eq!((counts()[0],counts()[4]),(before[0]+2,before[4]+2));
            let mut total=0.;
            for (half,channel) in ["positive","negative"].into_iter().enumerate(){
                let expected=&row["halves"][if sign>0.{half}else{1-half}];
                let key=read_geometry(&result,channel).unwrap();
                let (center,cap)=session.with_kernel_read(|kernel|{
                    let body=kernel.representation();
                    let id=kernel.geometry_solid(&key).unwrap();
                    let reach=body.reachable_from(&[EntityRef::Solid(id)]);
                    let sum=reach.vertices.iter().map(|id|body.vertices.get(*id).unwrap().position.to_array())
                        .fold([0.;3],|a,p|std::array::from_fn(|i|a[i]+p[i]));
                    let center=sum.map(|p|p/reach.vertices.len() as f64);
                    let n=Vec3::from_array(normal).normalized().unwrap();
                    let direction=if half==0{-n}else{n};
                    let mut caps=reach.faces.iter().filter_map(|id|{
                        let face=body.faces.get(*id).unwrap();
                        let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap() else{panic!("Split published curved support");};
                        ((frame.z-direction).norm()<1e-10&&((frame.origin-Pnt3::from_array(origin)).dot(n)).abs()<1e-10).then_some(frame.origin.to_array())
                    });
                    let point=caps.next().unwrap();
                    assert!(caps.next().is_none());
                    Ok((center,point))
                }).unwrap();
                let mesh=run_funded_preview(&session,identity,key.as_str(),&mut caller,&mut born,&mut free);
                mesh.validate_component_references().unwrap();
                let (lo,hi)=mesh.aabb();
                for (axis,actual) in [lo,hi].into_iter().enumerate(){
                    assert!(actual.into_iter().zip(fixture_xyz(&expected["bounds"][axis])).all(|(a,b)|(f64::from(a)-b).abs()<1e-6));
                }
                for (op,name) in [("brep.measure.volume","volume"),("brep.measure.area","area")]{
                    let query=Dictionary::new().insert("geometry",result.get(channel).unwrap().clone());
                    let values=run_live_query(&registry,op,query,&mut caller,&mut born,&mut free);
                    let amount=read_channel_number(&values,name).unwrap();
                    assert!((amount-expected[name].as_f64().unwrap()).abs()<1e-9);
                    if name=="volume"{total+=amount;}
                }
                for (point,state) in [(center,0.),(cap,2.)]{
                    let query=Dictionary::new().insert("solid",result.get(channel).unwrap().clone())
                        .insert("point",Value::Dictionary(point_dictionary(point)));
                    let classified=run_live_query(&registry,"brep.measure.classify",query,&mut caller,&mut born,&mut free);
                    assert_eq!(read_channel_number(&classified,"classification").unwrap(),state);
                }
            }
            assert!((total-size.into_iter().product::<f64>()).abs()<1e-9);
        }
        if index==0{
            for row in fixture["refused"].as_array().unwrap(){
                let before=counts();
                let mut job=Some(admit_live_job(&registry,operator,input(fixture_xyz(&row["point"]),fixture_xyz(&row["normal"]))));
                run(&mut caller,stock(),&mut born,&mut free,|cx|{
                    let result=job.as_mut().unwrap().step(cx).err().expect("contact and external split must refuse");
                    assert!(format!("{result:?}").contains("InvalidValue"));
                    Ok(())
                });
                close_live_job(&mut job,&mut caller,&mut born,&mut free);
                assert_eq!(counts(),before);
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_section_publishes_exact_sphere_and_oblique_cylinder_faces() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::{Pnt3,Vec3};
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,read_channel_number,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).unwrap();
    for (index,row) in corpus["retainedConicSection"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98801+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let radius=row["radius"].as_f64().unwrap();
        let mut parameters=Dictionary::new().insert("radius",number(radius));
        let constructor=if let Some(height)=row["height"].as_f64(){
            parameters=parameters.insert("height",number(height));"brep.prim3d.cylinder"
        }else{"brep.prim3d.sphere"};
        let raw=retained_boolean_result(&registry,constructor,parameters,&mut caller,&mut born,&mut free);
        let place=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let source=retained_boolean_result(&registry,"brep.xform.translate",place,&mut caller,&mut born,&mut free);
        let source_handle=read_geometry(&source,"geometryOut").unwrap();
        let inputs=|origin,normal|Dictionary::new().insert("solid",source.get("geometryOut").unwrap().clone())
            .insert("planeOrigin",Value::Dictionary(point_dictionary(fixture_xyz(origin))))
            .insert("planeNormal",Value::Dictionary(vector_dictionary(fixture_xyz(normal))));
        let operator="brep.intersect.section";
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.curves3.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for refused in row["refusals"].as_array().unwrap(){
            let mut job=Some(admit_live_job(&registry,operator,inputs(&refused["origin"],&refused["normal"])));
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                assert!(job.as_mut().unwrap().step(cx).is_err());
                Ok(())
            });
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let authored=inputs(&row["planeOrigin"],&row["normal"]);
        let mut draft=Some(admit_live_job(&registry,operator,authored.clone()));
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
        let output=retained_boolean_result(&registry,operator,authored,&mut caller,&mut born,&mut free);
        let faces=output.get("faces").and_then(Value::as_dictionary).unwrap();
        let handle=read_geometry(faces,"0").unwrap();
        assert!(faces.get("1").is_none());
        let center=Pnt3::from_array(fixture_xyz(&row["center"]));
        let normal=Vec3::from_array(fixture_xyz(&row["normal"])).normalized().unwrap();
        let source_origin=Pnt3::from_array(fixture_xyz(&row["origin"]));
        let (face_label,edge_label)=session.with_kernel_read(|kernel|{
            assert!(kernel.label_of(&source_handle).is_some());
            let face_label=kernel.label_of(&handle).unwrap();
            let body=kernel.representation();
            let (_,face)=body.faces.iter().find(|(_,face)|face.label==face_label).unwrap();
            let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{panic!("expected section plane");};
            assert!(frame.origin.distance(center)<1e-12&&(frame.z-normal).norm()<1e-12);
            let ring=body.loops.get(face.outer.unwrap()).unwrap();
            let use_=body.coedges.get(ring.first).unwrap();
            let edge=body.edges.get(use_.edge).unwrap();
            assert_eq!(edge.v0,edge.v1);
            assert_eq!(edge.range,(0.,std::f64::consts::TAU));
            let curve=body.curves3.get(edge.curve).unwrap();
            let actual=match curve{
                Curve3::Circle{radius,..}=>[*radius;2],
                Curve3::Ellipse{major_radius,minor_radius,..}=>[*major_radius,*minor_radius],
                _=>panic!("expected exact conic trim"),
            };
            for (actual,expected) in actual.into_iter().zip(row["radii"].as_array().unwrap()){
                assert!((actual-expected.as_f64().unwrap()).abs()<1e-12);
            }
            for t in [0.,0.25,1.4,2.75,5.] {
                let p=curve.eval(t);
                let uv=body.curves2.get(use_.pcurve.unwrap()).unwrap().eval(t);
                assert!(frame.to_world(Pnt3::new(uv.x,uv.y,0.)).distance(p)<1e-12);
                assert!((p-center).dot(normal).abs()<1e-12);
                let relative=p-source_origin;
                let distance=if row["kind"]=="sphere"{relative.norm()}else{relative.x.hypot(relative.y)};
                assert!((distance-radius).abs()<1e-12);
            }
            Ok((face_label.0.to_string(),edge.label.0.to_string()))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["face"],[face_label]);
        assert_eq!(mesh.component_references["edge"],[edge_label]);
        let area=retained_boolean_result(&registry,"brep.measure.area",Dictionary::new().insert("geometry",faces.get("0").unwrap().clone()),&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&area,"area").unwrap()-row["area"].as_f64().unwrap()).abs()<1e-10);
        caller.retire(stock());
    }
}

#[test]
fn original_axial_cylinder_section_publishes_the_signed_rectangle_and_retires_its_draft() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).unwrap();
    let session=Session::new();
    let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(98811),Generation(17),stock(),||Some(1000),1500,131072);
    let (mut born,mut free)=(0,0);
    let operator="brep.intersect.section";
    let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
    for row in corpus["retainedAxialCylinderSection"].as_array().unwrap(){
        let args=Dictionary::new().insert("radius",number(row["radius"].as_f64().unwrap()))
            .insert("height",number(row["height"].as_f64().unwrap()));
        let raw=retained_boolean_result(&registry,"brep.prim3d.cylinder",args,&mut caller,&mut born,&mut free);
        let translation=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let source=retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
        let args=|p:&serde_json::Value,n:&serde_json::Value|Dictionary::new().insert("solid",source.get("geometryOut").unwrap().clone())
            .insert("planeOrigin",Value::Dictionary(point_dictionary(fixture_xyz(p))))
            .insert("planeNormal",Value::Dictionary(vector_dictionary(fixture_xyz(n))));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.curves3.len()])
        }).unwrap();
        let normals=row["normals"].as_array().unwrap();
        let before=counts();
        for p in row["refusedPlaneOrigins"].as_array().unwrap(){
            let mut job=Some(admit_live_job(&registry,operator,args(p,&normals[0])));
            run(&mut caller,grant,&mut born,&mut free,|cx|{assert!(job.as_mut().unwrap().step(cx).is_err());Ok(())});
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
        }
        let mut job=Some(admit_live_job(&registry,operator,args(&row["planeOrigin"],&normals[0])));
        for _ in 0..100 {
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (status,receipt)=job.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(status,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            if counts()[2]>before[2]{break;}
        }
        assert!(counts()[2]>before[2]);
        assert_eq!(counts()[0],before[0]);
        job.as_mut().unwrap().cancel();
        close_live_job(&mut job,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        for (i,n) in normals.iter().enumerate(){
            let output=retained_boolean_result(&registry,operator,args(&row["planeOrigin"],n),&mut caller,&mut born,&mut free);
            let faces=output.get("faces").and_then(Value::as_dictionary).unwrap();
            let handle=read_geometry(faces,"0").unwrap();
            assert!(faces.get("1").is_none());
            let identity=GeometryWorkIdentity{operation:98811,generation:17+i as u64};
            let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
            mesh.validate_component_references().unwrap();
            for (domain,count) in [("face",1),("edge",4),("vertex",4)]{
                assert_eq!(mesh.component_references[domain].len(),count);
            }
            let normal=fixture_xyz(n).map(|v|v as f32);
            assert!(mesh.normals.chunks_exact(3).all(|vector|vector==normal));
            let measure=Dictionary::new().insert("geometry",faces.get("0").unwrap().clone());
            let area=retained_boolean_result(&registry,"brep.measure.area",measure,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&area,"area").unwrap()-row["area"].as_f64().unwrap()).abs()<1e-10);
        }
    }
    caller.retire(stock());
}

#[test]
fn original_clipped_cylinder_section_publishes_mixed_trim_and_preview_labels() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).unwrap();
    for (i,row) in corpus["retainedClippedCylinderSection"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98821+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let args=Dictionary::new().insert("radius",number(row["radius"].as_f64().unwrap()))
            .insert("height",number(row["height"].as_f64().unwrap()));
        let raw=retained_boolean_result(&registry,"brep.prim3d.cylinder",args,&mut caller,&mut born,&mut free);
        let translate=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let solid=retained_boolean_result(&registry,"brep.xform.translate",translate,&mut caller,&mut born,&mut free);
        let args=Dictionary::new().insert("solid",solid.get("geometryOut").unwrap().clone())
            .insert("planeOrigin",Value::Dictionary(point_dictionary(fixture_xyz(&row["planeOrigin"]))))
            .insert("planeNormal",Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
        let measure=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.coedges.len(),body.faces.len(),body.curves3.len()])
        }).unwrap();
        let base=measure();
        let mut draft=Some(admit_live_job(&registry,"brep.intersect.section",args.clone()));
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for _ in 0..100 {
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            if measure()[2]>base[2]{break;}
        }
        assert!(measure()[2]>base[2]);assert_eq!(measure()[0],base[0]);
        draft.as_mut().unwrap().cancel();
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(measure(),base);
        let output=retained_boolean_result(&registry,"brep.intersect.section",args,&mut caller,&mut born,&mut free);
        let faces=output.get("faces").and_then(Value::as_dictionary).unwrap();
        assert!(faces.get("1").is_none());
        let handle=read_geometry(faces,"0").unwrap();
        let corners=row["capCorners"].as_array().unwrap();
        let expected_normal=Pnt3::from_array(fixture_xyz(&row["normal"])).normalized().unwrap();
        let (labels,area)=session.with_kernel_read(|kernel|{
            let label=kernel.label_of(&handle).unwrap();
            let body=kernel.representation();
            let (_,face)=body.faces.iter().find(|(_,face)|face.label==label).unwrap();
            let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{panic!("planar section expected");};
            assert!((frame.z-expected_normal).norm()<1e-12);
            let first=body.loops.get(face.outer.unwrap()).unwrap().first;
            let mut id=first;
            let mut twice_area=0.;
            let mut labels=Vec::new();
            for member in 0..corners.len(){
                let use_=body.coedges.get(id).unwrap();
                let edge=body.edges.get(use_.edge).unwrap();
                let next=body.coedges.get(use_.next).unwrap();
                assert_eq!(next.prev,id);
                assert_eq!(body.edges.get(next.edge).unwrap().v0,edge.v1);
                let point=body.vertices.get(edge.v0).unwrap().position;
                assert!(corners.iter().any(|corner|Pnt3::from_array(fixture_xyz(corner)).distance(point)<1e-12));
                assert!(use_.forward&&edge.range==use_.prange);
                let (a,b)=edge.range;
                let pcurve=body.curves2.get(use_.pcurve.unwrap()).unwrap();
                let curve=body.curves3.get(edge.curve).unwrap();
                for t in [a,(a+b)/2.,b]{
                    let uv=pcurve.eval(t);
                    assert!(frame.to_world(Pnt3::new(uv.x,uv.y,0.)).distance(curve.eval(t))<1e-12);
                }
                twice_area+=match curve{
                    Curve3::Ellipse{major_radius,minor_radius,..} if member%2==0=>major_radius*minor_radius*(b-a),
                    Curve3::Line{..} if member%2==1=>{let p=pcurve.eval(a);let q=pcurve.eval(b);p.x*q.y-p.y*q.x},
                    _=>panic!("expected alternating ellipse and line trim"),
                };
                labels.push(edge.label.0.to_string());
                id=use_.next;
            }
            assert_eq!(id,first);
            Ok((labels,twice_area/2.))
        }).unwrap();
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-10);
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(!mesh.indices.is_empty());
        assert!(mesh.normals.chunks_exact(3).all(|n|{
            let actual=Pnt3::new(n[0] as f64,n[1] as f64,n[2] as f64);
            (actual-expected_normal).norm()<1e-6
        }));
        assert_eq!(mesh.component_references["edge"].len(),labels.len());
        assert!(labels.iter().all(|label|mesh.component_references["edge"].contains(label)));
        assert_eq!(mesh.component_references["vertex"].len(),corners.len());
        caller.retire(stock());
    }
}

#[test]
fn original_cone_and_frustum_section_publish_funded_exact_contours() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_geometry,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/✂️brep-intersect/🔣️.json")).unwrap();
    let rows=["retainedConicalSections","retainedParabolicSections","retainedHyperbolicSections","retainedApexSections"]
        .into_iter().flat_map(|key|corpus[key].as_array().unwrap());
    for (i,row) in rows.enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98851+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [x,y,z]=fixture_xyz(&row["origin"]);
        let height=row["height"].as_f64().unwrap();
        let radius=|i:usize|row["radii"][i].as_f64().unwrap();
        let source=if row["kind"]=="cone"{
            let input=Dictionary::new().insert("radius",number(radius(0))).insert("height",number(height));
            let cone=retained_boolean_result(&registry,"brep.prim3d.cone",input,&mut caller,&mut born,&mut free);
            let place=Dictionary::new().insert("geometry",cone.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary([x,y,z])));
            retained_boolean_result(&registry,"brep.xform.translate",place,&mut caller,&mut born,&mut free).get("geometryOut").unwrap().clone()
        }else{
            let mut profiles=Dictionary::with_schema("list");
            for index in 0..2 {
                let input=Dictionary::new().insert("radius",number(radius(index)))
                    .insert("normal",Value::Dictionary(vector_dictionary([0.,0.,1.])))
                    .insert("center",Value::Dictionary(point_dictionary([x,y,z+height*index as f64])));
                let curve=retained_boolean_result(&registry,"brep.curve.circle",input,&mut caller,&mut born,&mut free);
                let input=Dictionary::new().insert("wire",curve.get("curve").unwrap().clone());
                let face=retained_boolean_result(&registry,"brep.util.faceFromWire",input,&mut caller,&mut born,&mut free);
                profiles=profiles.insert(index.to_string(),face.get("face").unwrap().clone());
            }
            let loft=Dictionary::new().insert("profiles",Value::Dictionary(profiles)).insert("smooth",number(0.));
            retained_boolean_result(&registry,"brep.sweep.loft",loft,&mut caller,&mut born,&mut free).get("solid").unwrap().clone()
        };
        let input=Dictionary::new().insert("solid",source)
            .insert("planeOrigin",Value::Dictionary(point_dictionary(fixture_xyz(&row["planeOrigin"]))))
            .insert("planeNormal",Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.edges.len(),body.faces.len(),body.curves3.len(),body.coedges.len()])
        }).unwrap();
        let before=counts();
        let operator="brep.intersect.section";
        let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for _ in 0..100{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (state,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(state,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            if counts()[1]>before[1]{break;}
        }
        assert!(counts()[1]>before[1]);assert_eq!(counts()[0],before[0]);
        draft.as_mut().unwrap().cancel();
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let output=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let faces=output.get("faces").and_then(Value::as_dictionary).unwrap();
        let handle=read_geometry(faces,"0").unwrap();
        let n=Pnt3::from_array(fixture_xyz(&row["normal"])).normalized().unwrap();
        let points=row.get("points").and_then(serde_json::Value::as_array);
        let corner_count=points.or_else(||row["capCorners"].as_array()).unwrap().len();
        let (labels,area)=session.with_kernel_read(|kernel|{
            let label=kernel.label_of(&handle).unwrap();
            let body=kernel.representation();
            let (_,face)=body.faces.iter().find(|(_,face)|face.label==label).unwrap();
            let Surface::Plane{frame}=body.surfaces.get(face.surface).unwrap()else{panic!("expected a section plane");};
            if let Some(center)=row.get("center"){
                assert!(frame.origin.distance(Pnt3::from_array(fixture_xyz(center)))<1e-10);
            }
            let oriented=if face.flipped{-frame.z}else{frame.z};
            assert!((oriented-n).norm()<1e-12);
            let first=body.loops.get(face.outer.unwrap()).unwrap().first;
            let mut next=first;
            let mut labels=Vec::new();
            let mut twice_area=0.;
            for member in 0..corner_count.max(1){
                let use_=body.coedges.get(next).unwrap();
                let edge=body.edges.get(use_.edge).unwrap();
                let curve=body.curves3.get(edge.curve).unwrap();
                let trim=body.curves2.get(use_.pcurve.unwrap()).unwrap();
                let (a,b)=edge.range;
                assert_eq!(edge.range,use_.prange);
                for t in [a,(a+b)*0.5,b]{
                    let uv=trim.eval(t);
                    assert!(frame.to_world(Pnt3::new(uv.x,uv.y,0.)).distance(curve.eval(t))<1e-10);
                }
                if let Some(points)=points{
                    let position=body.vertices.get(edge.v0).unwrap().position;
                    assert!(points.iter().any(|p|position.distance(Pnt3::from_array(fixture_xyz(p)))<1e-10));
                }
                let area=match curve{
                    Curve3::Circle{radius,..} if member==0&&corner_count==0=>radius*radius*(b-a),
                    Curve3::Ellipse{major_radius,minor_radius,..} if member%2==0=>major_radius*minor_radius*(b-a),
                    Curve3::Nurbs{knots,controls,weights} if member%2==0=>{
                        assert!(row.get("center").is_none());
                        assert_eq!(knots.degree,2);
                        assert_eq!(controls.len(),3);
                        assert!(weights.iter().all(|&w|w>0.0&&w.is_finite()));
                        let mut previous=trim.eval(a);
                        (1..=4096).map(|index|{
                            let current=trim.eval(a+(b-a)*f64::from(index)/4096.);
                            let term=previous.x*current.y-previous.y*current.x;
                            previous=current;term
                        }).sum()
                    }
                    Curve3::Line{..} if points.is_some()||member%2==1=>{let p=trim.eval(a);let q=trim.eval(b);p.x*q.y-p.y*q.x},
                    _=>panic!("unexpected conical section contour"),
                };
                twice_area+=if use_.forward{area}else{-area};
                labels.push(edge.label.0.to_string());
                next=use_.next;
            }
            assert_eq!(next,first);
            Ok((labels,twice_area*if face.flipped{-0.5}else{0.5}))
        }).unwrap();
        assert!((area-row["area"].as_f64().unwrap()).abs()<if row.get("center").is_some()||points.is_some(){1e-9}else{1e-4});
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(!mesh.indices.is_empty());
        assert!(mesh.normals.chunks_exact(3).all(|v|(Pnt3::new(v[0] as f64,v[1] as f64,v[2] as f64)-n).norm()<1e-6));
        assert!(labels.iter().all(|key|mesh.component_references["edge"].contains(key)));
        caller.retire(stock());
    }
}

#[test]
fn clipped_section_faces_keep_authored_arcs_in_closest_point_and_distance() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_xyz, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️mixed-section-distance.json")).unwrap();
    let coordinates = |row: &serde_json::Value| Value::Dictionary(point_dictionary(fixture_xyz(row)));
    for (index, row) in fixture["sections"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(99900 + index as u64), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let constructor = if row["kind"] == "cone" { "brep.prim3d.cone" } else { "brep.prim3d.cylinder" };
        let params = Dictionary::new().insert("radius", number(row["radius"].as_f64().unwrap())).insert("height", number(row["height"].as_f64().unwrap()));
        let solid = retained_boolean_result(&registry, constructor, params, &mut caller, &mut born, &mut free);
        let input = Dictionary::new().insert("geometry", solid.get("solid").unwrap().clone())
            .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let placed = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
        let input = Dictionary::new().insert("solid", placed.get("geometryOut").unwrap().clone())
            .insert("planeOrigin", coordinates(&row["planeOrigin"]))
            .insert("planeNormal", Value::Dictionary(vector_dictionary(fixture_xyz(&row["normal"]))));
        let section = retained_boolean_result(&registry, "brep.intersect.section", input, &mut caller, &mut born, &mut free);
        let faces = section.get("faces").and_then(Value::as_dictionary).unwrap();
        let face = faces.get("0").unwrap();
        assert!(faces.get("1").is_none());
        let input = Dictionary::new().insert("geometry", face.clone());
        if index == 0 {
            let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
            let mut area_job = Some(admit_live_job(&registry, "brep.measure.area", input.clone()));
            for _ in 0..3 {
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = area_job.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                    assert!(receipt.fits(grant));
                    Ok(())
                });
            }
            assert_eq!(area_job.as_ref().unwrap().progress().phase, "geometry-area-integrating");
            run(&mut caller, Grant { maximum_items: 0, ..grant }, &mut born, &mut free, |cx| {
                let (step, receipt) = area_job.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                assert_eq!(receipt, Progress::default());
                Ok(())
            });
            area_job.as_mut().unwrap().cancel();
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                assert!(matches!(area_job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                Ok(())
            });
            close_live_job(&mut area_job, &mut caller, &mut born, &mut free);
        }
        let measured = retained_boolean_result(&registry, "brep.measure.area", input, &mut caller, &mut born, &mut free);
        let area = read_channel_number(&measured, "area").unwrap();
        assert!((area - row["area"].as_f64().unwrap()).abs() < 1e-9, "{}: {area}", row["kind"]);

        for (query_index, query) in row["queries"].as_array().unwrap().iter().enumerate() {
            let target = coordinates(&query["target"]);
            let input = Dictionary::new().insert("point", target.clone());
            let vertex = run_live_query(&registry, "brep.util.vertex", input, &mut caller, &mut born, &mut free);
            let vertex = vertex.get("vertex").unwrap();
            let input = Dictionary::new().insert("geometry", face.clone()).insert("point", target);
            let nearest = run_live_query(&registry, "brep.measure.closestPoint", input, &mut caller, &mut born, &mut free);
            let expected = fixture_xyz(&query["point"]);
            let actual = read_xyz(&nearest, "pointOut").unwrap();
            assert!(actual.into_iter().zip(expected).all(|(a, b)| (a - b).abs() < 1e-8), "{}: {actual:?}", query["id"]);
            for (order, (a, b)) in [(face, vertex), (vertex, face)].into_iter().enumerate() {
                let input = Dictionary::new().insert("a", a.clone()).insert("b", b.clone());
                if query_index == 0 && order == 0 {
                    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
                    let mut job = Some(admit_live_job(&registry, "brep.measure.distance", input.clone()));
                    for _ in 0..4 {
                        run(&mut caller, grant, &mut born, &mut free, |cx| {
                            let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
                            assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)));
                            assert!(receipt.fits(grant));
                            Ok(())
                        });
                    }
                    assert_eq!(job.as_ref().unwrap().progress().phase, "planar-closest-point");
                    job.as_mut().unwrap().cancel();
                    run(&mut caller, grant, &mut born, &mut free, |cx| {
                        assert!(matches!(job.as_mut().unwrap().step(cx).unwrap().0, neural_engine::OperatorJobStep::Cancelled(_)));
                        Ok(())
                    });
                    close_live_job(&mut job, &mut caller, &mut born, &mut free);
                }
                let result = run_live_query(&registry, "brep.measure.distance", input, &mut caller, &mut born, &mut free);
                let distance = read_channel_number(&result, "distance").unwrap();
                assert!((distance - query["distance"].as_f64().unwrap()).abs() < 1e-8, "{}: {distance}", query["id"]);
            }
        }
        caller.retire_exact();
    }
}

#[test]
fn retained_spherical_properties_survive_boolean_publication_and_face_selection() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_xyz, text_dictionary, vector_dictionary};

    let corpus: serde_json::Value = serde_json::from_str(include_str!("🔣️spherical-measurement.json")).unwrap();
    let tolerance = corpus["relativeTolerance"].as_f64().unwrap();
    for (index, row) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(99940 + index as u64), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let [outer, inner]: [f64; 2] = serde_json::from_value(row["radii"].clone()).unwrap();
        let mut sphere = |index: usize| {
            let radius = [outer, inner][index];
            let created = retained_boolean_result(&registry, "brep.prim3d.sphere", Dictionary::new().insert("radius", number(radius)), &mut caller, &mut born, &mut free);
            let input = Dictionary::new().insert("geometry", created.get("solid").unwrap().clone())
                .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&row["centers"][index]))));
            retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free).get("geometryOut").unwrap().clone()
        };
        let a = sphere(0);
        let kind = row["kind"].as_str().unwrap();
        let (operator, input) = if kind == "shell" {
            let input = Dictionary::new().insert("geometry", a).insert("thickness", number(outer - inner))
                .insert("openFaces", Value::Dictionary(Dictionary::with_schema("list")));
            ("brep.solid.shell", input)
        } else {
            let operator = match kind {
                "cut" => "brep.bool.cut",
                "fuse" => "brep.bool.fuse",
                "intersect" => "brep.bool.intersect",
                _ => panic!("unexpected spherical operation {kind}"),
            };
            (operator, Dictionary::new().insert("a", a).insert("b", sphere(1)))
        };
        let result = retained_boolean_result(&registry, operator, input, &mut caller, &mut born, &mut free);
        let solid = result.get("solid").unwrap().clone();
        let query = || Dictionary::new().insert("geometry", solid.clone());
        for (operator, channel) in [("brep.measure.area", "area"), ("brep.measure.volume", "volume")] {
            let result = run_live_query(&registry, operator, query(), &mut caller, &mut born, &mut free);
            let actual = read_channel_number(&result, channel).unwrap();
            let expected = row[channel].as_f64().unwrap();
            assert!((actual - expected).abs() < tolerance * expected.abs().max(1.0), "{} {channel}: {actual}", row["id"]);
        }
        let result = run_live_query(&registry, "brep.measure.centerOfMass", query(), &mut caller, &mut born, &mut free);
        let center = read_xyz(&result, "center").unwrap();
        assert!(center.into_iter().zip(fixture_xyz(&row["center"])).all(|(a, b)| (a-b).abs() < tolerance), "{}: {center:?}", row["id"]);

        let input = Dictionary::new().insert("brep", solid)
            .insert("edgeLabels", Value::Dictionary(text_dictionary("[]")))
            .insert("faceLabels", Value::Dictionary(text_dictionary("[]")))
            .insert("sourceHandle", Value::Dictionary(text_dictionary("")));
        let result = retained_boolean_result(&registry, "brep.brep", input, &mut caller, &mut born, &mut free);
        let faces = result.get("face").and_then(Value::as_dictionary).unwrap();
        let mut actual = ["0", "1"].map(|index| {
            let input = Dictionary::new().insert("geometry", faces.get(index).unwrap().clone());
            let area = run_live_query(&registry, "brep.measure.area", input, &mut caller, &mut born, &mut free);
            read_channel_number(&area, "area").unwrap()
        });
        let mut expected: [f64; 2] = serde_json::from_value(row["faces"].clone()).unwrap();
        if let Some(probes) = row["nearest"].as_array() {
            for probe in probes {
                let expected_area = expected[probe["face"].as_u64().unwrap() as usize];
                let selected = if (actual[0] - expected_area).abs() < tolerance * expected_area { "0" } else { "1" };
                let face = faces.get(selected).unwrap().clone();
                let target = Value::Dictionary(point_dictionary(fixture_xyz(&probe["target"])));
                let input = Dictionary::new().insert("geometry", face.clone()).insert("point", target.clone());
                let projected = run_live_query(&registry, "brep.measure.closestPoint", input, &mut caller, &mut born, &mut free);
                let projected = read_xyz(&projected, "pointOut").unwrap();
                if probe["point"].is_array() {
                    assert!(projected.into_iter().zip(fixture_xyz(&probe["point"]))
                        .all(|(a,b)| (a-b).abs() < tolerance), "{}: {projected:?}", row["id"]);
                }
                let [x, y, z] = fixture_xyz(&probe["target"]);
                let distance = (projected[0] - x).hypot(projected[1] - y).hypot(projected[2] - z);
                assert!((distance - probe["distance"].as_f64().unwrap()).abs() < tolerance);
                let input = Dictionary::new().insert("point", target);
                let vertex = run_live_query(&registry, "brep.util.vertex", input, &mut caller, &mut born, &mut free);
                let vertex = vertex.get("vertex").unwrap();
                for (a, b) in [(&face, vertex), (vertex, &face)] {
                    let input = Dictionary::new().insert("a", a.clone()).insert("b", b.clone());
                    let result = run_live_query(&registry, "brep.measure.distance", input, &mut caller, &mut born, &mut free);
                    assert!((read_channel_number(&result, "distance").unwrap() - probe["distance"].as_f64().unwrap()).abs()
                        < tolerance, "{} {}", row["id"], probe);
                }
            }
        }
        actual.sort_by(f64::total_cmp);
        expected.sort_by(f64::total_cmp);
        assert!(actual.into_iter().zip(expected).all(|(a,b)| (a-b).abs() < tolerance * b), "{}: {actual:?}", row["id"]);
        assert!(faces.get("2").is_none());
        caller.retire_exact();
    }
}


#[test]
fn original_convert_to_nurbs_cuboid_preserves_trims_and_retires_cancelled_rows() {
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{point_dictionary,read_channel_number,read_geometry,read_text,vector_dictionary};

    let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️nurbs-cuboid.json")).unwrap();
    let operator=corpus["operator"].as_str().unwrap();
    let input=corpus["input"].as_str().unwrap();
    let output=corpus["output"].as_str().unwrap();
    let tolerance=corpus["tolerance"].as_f64().unwrap();
    for (idx,row) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(103480+idx as u64),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [w,d,h]=fixture_xyz(&row["size"]);
        let arguments=Dictionary::new().insert("width",number(w)).insert("depth",number(d)).insert("height",number(h));
        let initial=retained_boolean_result(&registry,"brep.prim3d.box",arguments,&mut caller,&mut born,&mut free);
        let target=Dictionary::new().insert("geometry",initial.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["origin"]))));
        let placed=retained_boolean_result(&registry,"brep.xform.translate",target,&mut caller,&mut born,&mut free);
        let source_handle=read_geometry(&placed,"geometryOut").unwrap();
        let original=placed.get("geometryOut").unwrap();
        let args=||Dictionary::new().insert(input,original.clone());
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.solids.len(),body.curves3.len(),body.surfaces.len()])
        }).unwrap();
        let before=counts();
        let mut cancelled=Some(admit_live_job(&registry,operator,args()));
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for _ in 0..55 {
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                assert!(matches!(cancelled.as_mut().unwrap().step(cx).unwrap().0,neural_engine::OperatorJobStep::Working(_)));
                Ok(())
            });
        }
        assert_eq!(counts()[0],before[0]);
        close_live_job(&mut cancelled,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);

        let converted=retained_boolean_result(&registry,operator,args(),&mut caller,&mut born,&mut free);
        let converted_handle=read_geometry(&converted,output).unwrap();
        let solid=converted.get(output).unwrap();
        let [a,b]=session.with_kernel_read(|kernel|Ok([kernel.label_of(&source_handle).unwrap(),kernel.label_of(&converted_handle).unwrap()])).unwrap();
        assert_ne!(a,b);
        session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let selected=|label|body.solids.iter().find(|(_,solid)|solid.label==label).unwrap().0;
            let original=body.solid_faces(selected(a));
            let converted=body.solid_faces(selected(b));
            assert_eq!([original.len(),converted.len()],[6;2]);
            for (source,result) in original.into_iter().zip(converted) {
                let face=|id|body.surfaces.get(body.faces.get(id).unwrap().surface).unwrap();
                let analytic=face(source);
                let spline=face(result);
                assert!(matches!(analytic,Surface::Plane{..}));
                assert!(matches!(spline,Surface::Nurbs{u_knots,v_knots,..} if u_knots.degree==1 && v_knots.degree==1));
                let edges=|id|body.face_coedges(id);
                for (old,new) in edges(source).into_iter().zip(edges(result)) {
                    let old=body.coedges.get(old).unwrap();
                    let new=body.coedges.get(new).unwrap();
                    assert_eq!(old.forward,new.forward);
                    assert_eq!(old.prange,new.prange);
                    let edge=body.edges.get(new.edge).unwrap();
                    let curve=body.curves3.get(edge.curve).unwrap();
                    assert!(matches!(curve,Curve3::Nurbs{knots,..} if knots.degree==1));
                    let trim=body.curves2.get(new.pcurve.unwrap()).unwrap();
                    for t in [0.0,0.375,1.0] {
                        let uv=trim.eval(t);
                        let on_surface=spline.eval(uv.x,uv.y);
                        assert!(on_surface.distance(analytic.eval(uv.x,uv.y))<tolerance);
                        assert!(on_surface.distance(curve.eval(t))<tolerance);
                    }
                }
            }
            let first=body.solid_faces(selected(b))[0];
            let edge=body.edges.get(body.coedges.get(body.face_coedges(first)[0]).unwrap().edge).unwrap();
            let point=body.curves3.get(edge.curve).unwrap().eval(row["edge"]["parameter"].as_f64().unwrap());
            assert!(point.distance(Pnt3::from_array(fixture_xyz(&row["edge"]["point"])))<tolerance);
            Ok(())
        }).unwrap();
        for (op,port) in [("brep.measure.area","area"),("brep.measure.volume","volume")] {
            let command=Dictionary::new().insert("geometry",solid.clone());
            let measured=run_live_query(&registry,op,command,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&measured,port).unwrap()-row[port].as_f64().unwrap()).abs()<tolerance);
        }
        let step = &corpus["stepExport"];
        let import = &corpus["stepImport"];
        let mut exchange = |operator, input| {
            let mut job = Some(admit_live_job(&registry, operator, input));
            let mut output = None;
            for _ in 0..10000 {
                let copy = job.as_ref().unwrap().next_step_copy_byte_demand().unwrap().max(128);
                run(&mut caller, Grant { maximum_copy_bytes: copy, ..grant }, &mut born, &mut free, |cx| {
                    if let neural_engine::OperatorJobStep::Done(value) = job.as_mut().unwrap().step(cx).unwrap().0 {
                        output = Some(value);
                    }
                    Ok(())
                });
                if output.is_some() { break; }
            }
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            neural_engine::ColdOwner::new(output.unwrap())
        };
        let export_port = step["output"].as_str().unwrap();
        let export = |value: &Value| Dictionary::new().insert("geometry", value.clone());
        let verify = |text: &str| {
            for (entity, count) in step["counts"].as_object().unwrap() {
                assert_eq!(text.lines().filter(|line| line.contains(&format!(" = {entity}("))).count(), count.as_u64().unwrap() as usize);
            }
        };
        let before_export = counts();
        let document = exchange(step["operator"].as_str().unwrap(), export(solid));
        verify(&read_text(&document, export_port).unwrap());
        assert_eq!(counts(), before_export);

        let restored = exchange(import["operator"].as_str().unwrap(),
            Dictionary::new().insert(import["input"].as_str().unwrap(), document.get(export_port).unwrap().clone()));
        let restored_port = import["output"].as_str().unwrap();
        let restored_handle = read_geometry(&restored, restored_port).unwrap();
        session.with_kernel_read(|kernel| {
            let original_label = kernel.label_of(&converted_handle).unwrap();
            let restored_label = kernel.label_of(&restored_handle).unwrap();
            assert_ne!(original_label, restored_label);
            let body = kernel.representation();
            let faces = |label| body.solid_faces(body.solids.iter().find(|(_, solid)| solid.label == label).unwrap().0);
            for (original, restored) in faces(original_label).into_iter().zip(faces(restored_label)) {
                let surface = |id| body.surfaces.get(body.faces.get(id).unwrap().surface).unwrap();
                assert_eq!(surface(original), surface(restored));
                let edge = |id| body.edges.get(body.coedges.get(id).unwrap().edge).unwrap();
                for (a, b) in body.face_coedges(original).into_iter().zip(body.face_coedges(restored)) {
                    let (a, b) = (edge(a), edge(b));
                    assert_eq!(a.range, b.range);
                    assert_eq!(body.curves3.get(a.curve), body.curves3.get(b.curve));
                }
            }
            Ok(())
        }).unwrap();
        let document = exchange(step["operator"].as_str().unwrap(), export(restored.get(restored_port).unwrap()));
        verify(&read_text(&document, export_port).unwrap());
        if idx==0 {
            let sphere=retained_boolean_result(&registry,"brep.prim3d.sphere",Dictionary::new().insert("radius",number(2.0)),&mut caller,&mut born,&mut free);
            let point=|xyz|Value::Dictionary(point_dictionary(xyz));
            let line=retained_boolean_result(&registry,"brep.curve.line",Dictionary::new().insert("start",point([0.0;3])).insert("end",point([1.0;3])),&mut caller,&mut born,&mut free);
            let snapshot=counts();
            for wrong in [sphere.get("solid").unwrap(),line.get("curve").unwrap()] {
                let mut denied=Some(admit_live_job(&registry,operator,Dictionary::new().insert(input,wrong.clone())));
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    assert!(denied.as_mut().unwrap().step(cx).is_err());
                    Ok(())
                });
                close_live_job(&mut denied,&mut caller,&mut born,&mut free);
            }
            assert_eq!(counts(),snapshot);
        }
        caller.retire_exact();
    }
}

#[test]
fn original_section_publishes_disconnected_faces_together_and_rolls_back_private_topology() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🧪️tests/🧫️fixtures/disconnected-planar-section.json")).unwrap();
    let session = Session::new();
    let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
    let identity = GeometryWorkIdentity { operation: 99812, generation: 17 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(17), stock(), || Some(1000), 1500, 131072);
    let (mut born, mut free) = (0, 0);
    let mut points = Dictionary::new().insert("$schema", Value::Atom(Atom::String("list".into())));
    for (index, row) in fixture["profile"].as_array().unwrap().iter().enumerate() {
        let axes = [row[0].as_f64().unwrap(), row[1].as_f64().unwrap(), 0.];
        points = points.insert(index.to_string(), Value::Dictionary(point_dictionary(axes)));
    }
    let profile = run_live_query(&registry, "brep.surf.planarFace", Dictionary::new().insert("points", Value::Dictionary(points)),
        &mut caller, &mut born, &mut free);
    let span = [0., 0., fixture["height"].as_f64().unwrap()];
    let solid = run_live_query(&registry, "brep.sweep.extrude", Dictionary::new().insert("face", profile.get("face").unwrap().clone())
        .insert("vector", Value::Dictionary(vector_dictionary(span))), &mut caller, &mut born, &mut free);
    let counts = || session.with_kernel_read(|kernel| {
        let body = kernel.representation();
        Ok([kernel.live_handles().len(), body.vertices.len(), body.edges.len(), body.faces.len(), body.curves3.len()])
    }).unwrap();
    let input = |sign| Dictionary::new().insert("solid", solid.get("solid").unwrap().clone())
        .insert("planeOrigin", Value::Dictionary(point_dictionary([0., fixture["cut"].as_f64().unwrap(), 0.])))
        .insert("planeNormal", Value::Dictionary(vector_dictionary([0., sign, 0.])));
    let operator = "brep.intersect.section";
    let before = counts();
    let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
    let mut job = Some(admit_live_job(&registry, operator, input(1.)));
    let staged = (0..1800).any(|_| {
        let mut staged = false;
        run(&mut caller, grant, &mut born, &mut free, |cx| {
            let (step, receipt) = job.as_mut().unwrap().step(cx).unwrap();
            let OperatorJobStep::Working(progress) = step else { panic!("private section opened early"); };
            staged = progress.phase == "section-staged-claim";
            assert!(receipt.fits(grant));
            Ok(())
        });
        assert_eq!(counts()[0], before[0]);
        staged
    });
    assert!(staged);
    assert_eq!(counts()[3], before[3] + 2);
    job.as_mut().unwrap().cancel();
    close_live_job(&mut job, &mut caller, &mut born, &mut free);
    assert_eq!(counts(), before);

    for sign in [-1., 1.] {
        let prior = counts();
        let payload = run_live_query(&registry, operator, input(sign), &mut caller, &mut born, &mut free);
        assert_eq!((counts()[0], counts()[3]), (prior[0] + 2, prior[3] + 2));
        let faces = payload.get("faces").and_then(Value::as_dictionary).unwrap();
        assert_eq!((faces.schema(), faces.len()), (Some("list"), 3));
        let mut areas = ["0", "1"].map(|key| {
            let handle = read_geometry(faces, key).unwrap();
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            let query = Dictionary::new().insert("geometry", faces.get(key).unwrap().clone());
            let area = run_live_query(&registry, "brep.measure.area", query, &mut caller, &mut born, &mut free);
            read_channel_number(&area, "area").unwrap()
        });
        areas.sort_by(f64::total_cmp);
        let expected = fixture["cases"][0]["faces"].as_array().unwrap();
        assert!(areas.iter().zip(expected).all(|(area, expected)| *area == expected["area"].as_f64().unwrap()));
    }
    caller.retire(stock());
}
