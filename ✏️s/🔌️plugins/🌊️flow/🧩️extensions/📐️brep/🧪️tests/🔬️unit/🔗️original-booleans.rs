//! 🔗️ Original certified analytic Boolean solids and retained preview.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");
include!("🐚️original-cylinder-shell.rs");
include!("↔️original-axial-solid-offset.rs");
include!("🧰️original-box-placement.rs");
include!("🧰️original-oriented-finish.rs");
include!("🐚️original-oriented-box-shell.rs");

#[test]
fn original_partial_rotated_box_booleans_publish_one_closed_prism() {
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️original-rotated-box-sets.json")).unwrap();
    for (index,row) in fixture["cases"].as_array().unwrap().iter().enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=flow_extension_sdk::geometry::GeometryWorkIdentity{operation:99120+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(17),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let mut stock_box=|spec:&serde_json::Value|{
            let input=["width","depth","height"].into_iter().zip(fixture_xyz(spec))
                .fold(Dictionary::new(),|input,(key,size)|input.insert(key,number(size)));
            let box_=retained_boolean_result(&registry,"brep.prim3d.box",input,&mut caller,&mut born,&mut free);
            box_.get("solid").unwrap().clone()
        };
        let a=stock_box(&fixture["stock"]);
        let unplaced=stock_box(&row["tool"]);
        let rotation=Dictionary::new().insert("geometry",unplaced)
            .insert("angle",number(row["angleDegrees"].as_f64().unwrap().to_radians()))
            .insert("axis",Value::Dictionary(vector_dictionary([0.,0.,1.])));
        let rotated=retained_boolean_result(&registry,"brep.xform.rotate",rotation,&mut caller,&mut born,&mut free);
        let translation=Dictionary::new().insert("geometry",rotated.get("geometryOut").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&row["translation"]))));
        let tool=retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
        let b=tool.get("geometryOut").unwrap();
        let count=||session.with_kernel_read(|kernel|Ok(kernel.live_handles().len())).unwrap();
        for (operator,operation) in [("brep.bool.cut","difference"),("brep.bool.fuse","union")]{
            let fact=&row["operations"][operation];
            let input=Dictionary::new().insert("a",a.clone()).insert("b",b.clone());
            let before=count();
            let components=fact["components"].as_u64().unwrap() as usize;
            if components>1 {
                let solids=||session.with_kernel_read(|kernel|Ok(kernel.representation().solids.len())).unwrap();
                let original=solids();
                let grant=Grant{maximum_items:1,..stock()};
                let mut job=Some(admit_live_job(&registry,operator,input.clone()));
                for _ in 0..1500 {
                    run(&mut caller,grant,&mut born,&mut free,|cx|{
                        let (step,receipt)=job.as_mut().unwrap().step(cx).unwrap();
                        assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                        Ok(())
                    });
                    assert_eq!(count(),before);
                    if solids()>original{break;}
                }
                assert_eq!(solids(),original+1);
                job.as_mut().unwrap().cancel();
                close_live_job(&mut job,&mut caller,&mut born,&mut free);
                assert_eq!((count(),solids()),(before,original));
            }
            let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
            assert_eq!(count(),before+1);
            let handle=read_geometry(&result,"solid").unwrap();
            if let Some(pieces)=fact["pieces"].as_array(){
                assert_eq!(components,pieces.len());
                let input=Dictionary::new().insert("compound",result.get("solid").unwrap().clone());
                let exploded=retained_boolean_result(&registry,"brep.topology.explode",input,&mut caller,&mut born,&mut free);
                let Value::Dictionary(pieces_out)=exploded.get("solids").unwrap() else{panic!("exploded component list missing");};
                let mut measured=Vec::new();
                for index in 0..components {
                    let member=pieces_out.get(&index.to_string()).unwrap();
                    let values=["volume","area"].map(|quantity|{
                        let measure=if quantity=="volume"{"brep.measure.volume"}else{"brep.measure.area"};
                        let query=Dictionary::new().insert("geometry",member.clone());
                        read_channel_number(&retained_boolean_result(&registry,measure,query,&mut caller,&mut born,&mut free),quantity).unwrap()
                    });
                    let wrapped=Dictionary::new().insert("solid",member.clone());
                    let child=read_geometry(&wrapped,"solid").unwrap();
                    let preview=run_funded_preview(&session,identity,child.as_str(),&mut caller,&mut born,&mut free);
                    let (area,volume)=preview_area_volume(&preview);
                    assert!((values[0]-volume).abs()<1e-5 && (values[1]-area).abs()<1e-5);
                    measured.push(values);
                }
                measured.sort_unstable_by(|a,b|a[0].total_cmp(&b[0]));
                for (actual,expected) in measured.iter().zip(pieces){
                    for i in 0..2{assert!((actual[i]-expected[i].as_f64().unwrap()).abs()<1e-9);}
                }
            }
            let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
            mesh.validate_component_references().unwrap();
            let (area,volume)=preview_area_volume(&mesh);
            for (op,key,preview) in [("brep.measure.area","area",area),("brep.measure.volume","volume",volume)]{
                let query=Dictionary::new().insert("geometry",result.get("solid").unwrap().clone());
                let measured=retained_boolean_result(&registry,op,query,&mut caller,&mut born,&mut free);
                let expected=fact[key].as_f64().unwrap();
                assert!((read_channel_number(&measured,key).unwrap()-expected).abs()<1e-9);
                assert!((preview-expected).abs()<1e-5);
            }
        }
        caller.retire(stock());
    }
}

#[test]
fn original_intersect_preserves_authored_planes_of_rotated_shared_axis_boxes() {
    use semio_s_spatial_kernel_semio_session::{point_dictionary, read_channel_number, read_geometry, vector_dictionary};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-rotated-box-intersection.json")).unwrap();
    let rows = fixture["cases"].as_array().unwrap().iter().chain(fixture["empty"].as_array().unwrap());
    for (index, row) in rows.enumerate() {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity = flow_extension_sdk::geometry::GeometryWorkIdentity { operation: 99067 + index as u64, generation: 17 };
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(identity.operation), Generation(17), stock(), || Some(1000), 1500, 131072);
        let (mut born, mut free) = (0, 0);
        let [a, b] = ["a", "b"].map(|port| {
            let spec = &row[port];
            let input = ["width", "depth", "height"].into_iter().zip(fixture_xyz(&spec["size"]))
                .fold(Dictionary::new(), |input, (key, size)| input.insert(key, number(size)));
            let placed = retained_boolean_result(&registry, "brep.prim3d.box", input, &mut caller, &mut born, &mut free);
            let mut solid = placed.get("solid").unwrap().clone();
            let angle = spec["angle"].as_f64().unwrap();
            if angle != 0.0 {
                let input = Dictionary::new().insert("geometry", solid).insert("angle", number(angle))
                    .insert("axis", Value::Dictionary(vector_dictionary([0., 0., 1.])));
                let rotated = retained_boolean_result(&registry, "brep.xform.rotate", input, &mut caller, &mut born, &mut free);
                solid = rotated.get("geometryOut").unwrap().clone();
            }
            let input = Dictionary::new().insert("geometry", solid)
                .insert("offset", Value::Dictionary(vector_dictionary(fixture_xyz(&spec["origin"]))));
            let moved = retained_boolean_result(&registry, "brep.xform.translate", input, &mut caller, &mut born, &mut free);
            moved.get("geometryOut").unwrap().clone()
        });
        let operator = row["operator"].as_str().unwrap_or_else(|| fixture["operator"].as_str().unwrap());
        let input = Dictionary::new().insert("a", a).insert("b", b.clone());
        let classify = |shape: &Value, rows: &serde_json::Value, caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize| {
            for probe in rows.as_array().unwrap() {
                let query = Dictionary::new().insert("solid",shape.clone())
                    .insert("point",Value::Dictionary(point_dictionary(fixture_xyz(&probe[0]))));
                let actual = retained_boolean_result(&registry,"brep.measure.classify",query,caller,born,free);
                assert_eq!(read_channel_number(&actual,"classification").unwrap(),probe[1].as_f64().unwrap());
            }
        };
        if row.get("classify").is_some() { classify(&b,&row["classify"]["operandB"],&mut caller,&mut born,&mut free); }
        let counts = || session.with_kernel_read(|kernel| {
            let body = kernel.representation();
            Ok([kernel.live_handles().len(), body.vertices.len(), body.edges.len(), body.faces.len(), body.solids.len()])
        }).unwrap();
        let before = counts();
        let expected = row["corners"].as_u64().map(|n| n as usize);
        let grant = Grant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        if expected.is_some() {
            let mut aborted = Some(admit_live_job(&registry, operator, input.clone()));
            for _ in 0..1300 {
                run(&mut caller, grant, &mut born, &mut free, |cx| {
                    let (step, receipt) = aborted.as_mut().unwrap().step(cx).unwrap();
                    assert!(matches!(step, neural_engine::OperatorJobStep::Working(_)) && receipt.fits(grant));
                    Ok(())
                });
                assert_eq!(counts()[0], before[0]);
                if counts()[3] > before[3] { break; }
            }
            assert!(counts()[3] > before[3]);
            aborted.as_mut().unwrap().cancel();
            close_live_job(&mut aborted, &mut caller, &mut born, &mut free);
            assert_eq!(counts(), before);
        }
        let mut job = Some(admit_live_job(&registry, operator, input));
        let mut output = None;
        for _ in 0..1800 {
            run(&mut caller, grant, &mut born, &mut free, |cx| {
                match job.as_mut().unwrap().step(cx) {
                    Ok((neural_engine::OperatorJobStep::Working(_),receipt)) => assert!(receipt.fits(grant)),
                    Ok((neural_engine::OperatorJobStep::Done(value),_)) => output = Some(Ok(value)),
                    Err(error) => output = Some(Err(format!("{error:?}"))),
                    _ => panic!("unexpected rotated-box intersection progress"),
                }
                Ok(())
            });
            if output.is_some() { break; }
            assert_eq!(counts()[0], before[0]);
        }
        close_live_job(&mut job, &mut caller, &mut born, &mut free);
        let output = output.expect("rotated box intersection did not finish");
        if let Some(corners) = expected {
            let output = neural_engine::ColdOwner::new(output.unwrap());
            assert_eq!(counts(), [before[0]+1, before[1]+2*corners, before[2]+3*corners, before[3]+corners+2, before[4]+1]);
            if row.get("classify").is_some() { classify(output.get("solid").unwrap(),&row["classify"]["result"],&mut caller,&mut born,&mut free); }
            let handle = read_geometry(&output, "solid").unwrap();
            let mesh = run_funded_preview(&session, identity, handle.as_str(), &mut caller, &mut born, &mut free);
            mesh.validate_component_references().unwrap();
            assert_eq!(mesh.component_references["face"].len(), corners+2);
            let (min,max) = mesh.aabb();
            for (side,actual) in [min,max].into_iter().enumerate() {
                assert!(actual.into_iter().zip(fixture_xyz(&row["bounds"][side])).all(|(a,b)|(f64::from(a)-b).abs()<1e-6));
            }
            if let Some(sections)=row["sections"].as_array(){
                for section in sections{
                    let input=Dictionary::new().insert("solid",output.get("solid").unwrap().clone())
                        .insert("planeOrigin",Value::Dictionary(point_dictionary(fixture_xyz(&section["origin"]))))
                        .insert("planeNormal",Value::Dictionary(vector_dictionary(fixture_xyz(&section["normal"]))));
                    let live=counts()[0];
                    if let Some(vertices)=section["vertices"].as_u64(){
                        let mut job=Some(admit_live_job(&registry,"brep.intersect.section",input));
                        let grant=Grant{maximum_items:1,maximum_copy_bytes:160,..stock()};
                        let mut result=None;
                        for _ in 0..1000{
                            run(&mut caller,grant,&mut born,&mut free,|cx|{
                                if let neural_engine::OperatorJobStep::Done(value)=job.as_mut().unwrap().step(cx).unwrap().0{
                                    result=Some(neural_engine::ColdOwner::new(value));
                                }
                                Ok(())
                            });
                            if result.is_some(){break;}
                            assert_eq!(counts()[0],live);
                        }
                        close_live_job(&mut job,&mut caller,&mut born,&mut free);
                        let result=result.unwrap();
                        let faces=result.get("faces").and_then(Value::as_dictionary).unwrap();
                        let key=read_geometry(faces,"0").unwrap();
                        assert_eq!(counts()[0],live+1);
                        let mesh=run_funded_preview(&session,identity,key.as_str(),&mut caller,&mut born,&mut free);
                        mesh.validate_component_references().unwrap();
                        assert_eq!(mesh.component_references["edge"].len() as u64,vertices);
                        let (min,max)=mesh.aabb();
                        for (side,actual) in [min,max].into_iter().enumerate(){
                            assert!(actual.into_iter().zip(fixture_xyz(&section["bounds"][side])).all(|(a,b)|(f64::from(a)-b).abs()<1e-6));
                        }
                        let input=Dictionary::new().insert("geometry",faces.get("0").unwrap().clone());
                        let measured=retained_boolean_result(&registry,"brep.measure.area",input,&mut caller,&mut born,&mut free);
                        assert!((read_channel_number(&measured,"area").unwrap()-section["area"].as_f64().unwrap()).abs()<1e-10);
                    }else{
                        let mut job=Some(admit_live_job(&registry,"brep.intersect.section",input));
                        let mut failed=false;
                        for _ in 0..50{
                            run(&mut caller,stock(),&mut born,&mut free,|cx|{
                                failed=job.as_mut().unwrap().step(cx).is_err();
                                Ok(())
                            });
                            if failed{break;}
                        }
                        close_live_job(&mut job,&mut caller,&mut born,&mut free);
                        assert!(failed);
                        assert_eq!(counts()[0],live);
                    }
                }
            }
            for (op, key) in [("brep.measure.volume", "volume"),("brep.measure.area","area")] {
                let input = Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
                let measured = retained_boolean_result(&registry, op, input, &mut caller, &mut born, &mut free);
                assert!((read_channel_number(&measured,key).unwrap()-row[key].as_f64().unwrap()).abs()<1e-10);
            }
        } else {
            assert!(output.err().expect("a contact must publish no solid").contains("empty"));
            assert_eq!(counts(), before);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_axial_booleans_publish_exact_solids_and_retire_cancelled_work() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,read_xyz,text_dictionary,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json")).unwrap();
    let rows=corpus["cases"].as_array().unwrap().iter().filter(|row|row["name"].as_str().unwrap().starts_with("coaxial ")||row.get("axialBore").is_some()||row.get("eccentricShoulders").is_some());
    for (index,row) in rows.enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98831+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let mut place=|recipe:&serde_json::Value|{
            let dims=&recipe["of"];
            let input=Dictionary::new().insert("radius",number(dims["radius"].as_f64().unwrap()))
                .insert("height",number(dims["height"].as_f64().unwrap()));
            let raw=retained_boolean_result(&registry,"brep.prim3d.cylinder",input,&mut caller,&mut born,&mut free);
            let translation=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&recipe["by"]))));
            retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free)
        };
        let stock_solid=place(&row["inputs"]["a"]);
        let tool=place(&row["inputs"]["b"]);
        let args=Dictionary::new().insert("a",stock_solid.get("geometryOut").unwrap().clone())
            .insert("b",tool.get("geometryOut").unwrap().clone());
        let is_blind=row["name"].as_str().unwrap().starts_with("coaxial blind bore")||row["axialBore"]=="blind";
        let is_cavity=row["name"].as_str().unwrap().starts_with("coaxial enclosed")||row["axialBore"]=="cavity";
        let is_step=row["kind"]=="brep.boolean.fuse";
        let double_step=row["name"].as_str().unwrap().starts_with("coaxial double shoulder")||row["eccentricShoulders"]==2;
        let operator=if is_step{"brep.bool.fuse"}else{"brep.bool.cut"};
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len(),body.shells.len()])
        }).unwrap();
        let before=counts();
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let mut draft=Some(admit_live_job(&registry,operator,args.clone()));
        for _ in 0..500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            if if is_cavity{counts()[5]==before[5]+2}else{counts()[2]>before[2]}{break;}
        }
        assert!(counts()[2]>before[2]);assert_eq!(counts()[0],before[0]);
        if is_cavity{assert_eq!(counts()[5],before[5]+2);}
        draft.as_mut().unwrap().cancel();
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let output=retained_boolean_result(&registry,operator,args,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"solid").unwrap();
        let stock_bottom=row["inputs"]["a"]["by"][2].as_f64().unwrap();
        let [tool_x,tool_y,tool_bottom]=fixture_xyz(&row["inputs"]["b"]["by"]);
        let tool_top=tool_bottom+row["inputs"]["b"]["of"]["height"].as_f64().unwrap();
        let floor=if tool_bottom>stock_bottom{tool_bottom}else{tool_top};
        let r_a=row["inputs"]["a"]["of"]["radius"].as_f64().unwrap();
        let r_b=row["inputs"]["b"]["of"]["radius"].as_f64().unwrap();
        let (inner,outer)=(r_a.min(r_b),r_a.max(r_b));
        let (large,small)=if r_a>r_b{("a","b")}else{("b","a")};
        let large=&row["inputs"][large];
        let small_bottom=row["inputs"][small]["by"][2].as_f64().unwrap();
        let [large_x,large_y,large_bottom]=fixture_xyz(&large["by"]);
        let [small_x,small_y,_]=fixture_xyz(&row["inputs"][small]["by"]);
        let step_below=small_bottom<large_bottom;
        let large_top=large_bottom+large["of"]["height"].as_f64().unwrap();
        let shoulder=if step_below{large_bottom}else{large_top};
        let (labels,selected_patches)=session.with_kernel_read(|kernel|{
            for input in [&stock_solid,&tool]{
                assert!(kernel.label_of(&read_geometry(input,"geometryOut").unwrap()).is_some());
            }
            let body=kernel.representation();
            let root=kernel.geometry_solid(&handle).unwrap();
            let native=body.solids.get(root).unwrap();
            assert_eq!(native.inners.len(),usize::from(is_cavity));
            if is_cavity{
                assert_eq!(body.shells.get(native.outer).unwrap().faces.len(),3);
                assert_eq!(body.shells.get(native.inners[0]).unwrap().faces.len(),3);
            }
            let reach=body.reachable_from(&[EntityRef::Solid(root)]);
            let expected=if double_step{(6,9,7)}else{(4,6,4+usize::from(is_blind||is_step)+2*usize::from(is_cavity))};
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),expected);
            let mut faces=Vec::new();
            let mut radii=Vec::new();
            let mut annuli=0;
            let mut selected_patches=Vec::new();
            let mut shoulders=[false;2];
            let mut ceiling_found=false;
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                faces.push(face.label.0.to_string());
                match body.surfaces.get(face.surface).unwrap(){
                    Surface::Cylinder{frame,radius}=>{
                        if is_step{
                            let (x,y)=if *radius==inner{(small_x,small_y)}else{(large_x,large_y)};
                            assert!((frame.origin.x-x).hypot(frame.origin.y-y)<1e-12);
                        }else if face.flipped{
                            assert!((frame.origin.x-tool_x).hypot(frame.origin.y-tool_y)<1e-12);
                        }
                        radii.push((*radius,face.flipped));
                    }
                    Surface::Plane{frame}=>{
                        if !face.inners.is_empty(){
                            assert_eq!(face.inners.len(),1);annuli+=1;
                            if is_step{
                                let lower=frame.origin.z==large_bottom;
                                assert_eq!(frame.origin.z,if double_step{if lower{large_bottom}else{large_top}}else{shoulder});
                                assert_eq!(frame.z.z,if lower{-1.}else{1.});
                                shoulders[usize::from(!lower)]=true;
                                selected_patches.push(face.label.0.to_string());
                            }
                        }
                        if (is_blind||is_cavity)&&frame.origin.z==floor {
                            assert!((frame.origin.x-tool_x).hypot(frame.origin.y-tool_y)<1e-12);
                            assert!(face.inners.is_empty());
                            assert_eq!(frame.z.z,if tool_bottom>stock_bottom{1.}else{-1.});
                            selected_patches.push(face.label.0.to_string());
                        }
                        if is_cavity&&frame.origin.z==tool_top{
                            assert!((frame.origin.x-tool_x).hypot(frame.origin.y-tool_y)<1e-12);
                            assert!(face.inners.is_empty());
                            assert_eq!(frame.z.z,-1.);
                            ceiling_found=true;
                        }
                    },
                    _=>panic!("expected two exact cylinder walls and annular caps"),
                }
            }
            radii.sort_by(|a,b|a.0.total_cmp(&b.0));
            let expected=if double_step{vec![(inner,false),(inner,false),(outer,false)]}else{vec![(inner,!is_step),(outer,false)]};
            assert_eq!(radii,expected);
            assert_eq!(annuli,2-usize::from(is_blind||is_step)-2*usize::from(is_cavity)+usize::from(double_step));
            assert_eq!(selected_patches.len(),usize::from(is_blind||is_cavity||is_step)+usize::from(double_step));
            if double_step{assert_eq!(shoulders,[true,true]);}
            assert_eq!(ceiling_found,is_cavity);
            assert!(reach.edges.iter().all(|&id| body.edge_coedges(id).len()==2));
            Ok((faces,selected_patches))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(labels.iter().all(|key|mesh.component_references["face"].contains(key)));
        let expected=&row["outputs"]["shape"];
        assert!((preview_area_volume(&mesh).0-expected["area"].as_f64().unwrap()).abs()<4.);
        let query=||Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
        for (op,key) in [("brep.measure.volume","volume"),("brep.measure.area","area")]{
            let measure=retained_boolean_result(&registry,op,query(),&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&measure,key).unwrap()-expected[key].as_f64().unwrap()).abs()<1e-10);
        }
        let measure=retained_boolean_result(&registry,"brep.measure.centerOfMass",query(),&mut caller,&mut born,&mut free);
        assert!(read_xyz(&measure,"center").unwrap().into_iter().zip(fixture_xyz(&expected["centroid"]))
            .all(|(a,b)|(a-b).abs()<1e-10));
        for label in selected_patches{
            let text=|value|Value::Dictionary(text_dictionary(value));
            let pick=Dictionary::new().insert("brep",output.get("solid").unwrap().clone())
                .insert("faceLabels",text(serde_json::to_string(&[&label]).unwrap()))
                .insert("edgeLabels",text("[]".to_owned())).insert("sourceHandle",text(String::new()));
            let selection=retained_boolean_result(&registry,"brep.brep",pick,&mut caller,&mut born,&mut free);
            let faces=selection.get("selectedFaces").and_then(Value::as_dictionary).unwrap();
            let floor=read_geometry(faces,"0").unwrap();
            let patch=run_funded_preview(&session,identity,floor.as_str(),&mut caller,&mut born,&mut free);
            patch.validate_component_references().unwrap();
            assert_eq!(patch.component_references["face"],[label]);
            let area=std::f64::consts::PI*if is_step{outer.powi(2)-inner.powi(2)}else{inner.powi(2)};
            assert!((preview_area_volume(&patch).0-area).abs()<0.35);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_sphere_cut_preserves_operands_and_handles_cavity_empty_and_disjoint_results() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json")).unwrap();
    let rows=fixture["cases"].as_array().unwrap().iter().filter(|row|{
        let name=row["name"].as_str().unwrap();
        ["concentric sphere cut","eccentric sphere cut","disjoint sphere cut"].into_iter().any(|prefix|name.starts_with(prefix))
    });
    for (i,row) in rows.enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98901+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let [stock_solid,tool]=["a","b"].map(|side|{
            let recipe=&row["inputs"][side];
            let raw=retained_boolean_result(&registry,"brep.prim3d.sphere",Dictionary::new().insert("radius",number(recipe["of"]["radius"].as_f64().unwrap())),&mut caller,&mut born,&mut free);
            let translate=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&recipe["by"]))));
            retained_boolean_result(&registry,"brep.xform.translate",translate,&mut caller,&mut born,&mut free)
        });
        let input=Dictionary::new().insert("a",stock_solid.get("geometryOut").unwrap().clone())
            .insert("b",tool.get("geometryOut").unwrap().clone());
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.shells.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let operator="brep.bool.cut";
        let empty=row.get("fault").is_some();
        let cavity=!empty&&!row["name"].as_str().unwrap().starts_with("disjoint sphere cut");
        let centers=["a","b"].map(|side|fixture_xyz(&row["inputs"][side]["by"]));
        let radii=["a","b"].map(|side|row["inputs"][side]["of"]["radius"].as_f64().unwrap());
        if empty||cavity{
            let mut job=Some(admit_live_job(&registry,operator,input.clone()));
            let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
            let mut refused=false;
            for _ in 0..500{
                run(&mut caller,grant,&mut born,&mut free,|cx|{
                    match job.as_mut().unwrap().step(cx){
                        Err(_)=>refused=true,
                        Ok((step,receipt))=>assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant)),
                    }
                    Ok(())
                });
                if refused||counts()[1]==before[1]+2{break;}
            }
            assert_eq!(refused,empty);
            if cavity{assert_eq!(counts()[1],before[1]+2);job.as_mut().unwrap().cancel();}
            assert_eq!(counts()[0],before[0]);
            close_live_job(&mut job,&mut caller,&mut born,&mut free);
            assert_eq!(counts(),before);
            if empty{caller.retire(stock());continue;}
        }
        let output=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&output,"solid").unwrap();
        let labels=session.with_kernel_read(|kernel|{
            for source in [&stock_solid,&tool]{
                assert!(kernel.label_of(&read_geometry(source,"geometryOut").unwrap()).is_some());
            }
            let body=kernel.representation();
            let solid=body.solids.get(kernel.geometry_solid(&handle).unwrap()).unwrap();
            assert_eq!(solid.inners.len(),usize::from(cavity));
            let mut labels=Vec::new();
            for (inside,shell) in std::iter::once(solid.outer).chain(solid.inners.iter().copied()).enumerate(){
                let faces=&body.shells.get(shell).unwrap().faces;
                assert_eq!(faces.len(),1);
                let face=body.faces.get(faces[0]).unwrap();
                let Surface::Sphere{frame,radius}=body.surfaces.get(face.surface).unwrap()else{panic!("expected sphere cut support");};
                let expected=row["inputs"][if inside==0{"a"}else{"b"}]["of"]["radius"].as_f64().unwrap();
                assert_eq!(*radius,expected);
                assert_eq!(frame.origin.to_array(),centers[inside]);
                assert_eq!(face.flipped,inside==1);
                labels.push(face.label.0.to_string());
            }
            Ok(labels)
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(labels.iter().all(|key|mesh.component_references["face"].contains(key)));
        let mut seen=[false;2];
        for (p,n) in mesh.positions.chunks_exact(3).zip(mesh.normals.chunks_exact(3)){
            let point=Pnt3::new(p[0] as f64,p[1] as f64,p[2] as f64);
            let normal=Pnt3::new(n[0] as f64,n[1] as f64,n[2] as f64);
            let offsets=centers.map(|center|point-Pnt3::from_array(center));
            let inside=cavity&&(offsets[1].norm()-radii[1]).abs()<1e-5;
            let side=usize::from(inside);
            assert!((offsets[side].norm()-radii[side]).abs()<1e-5);
            assert_eq!(offsets[side].dot(normal)<0.,inside);
            seen[side]=true;
        }
        assert_eq!(seen,[true,cavity]);
        let args=Dictionary::new().insert("geometry",output.get("solid").unwrap().clone());
        let volume=retained_boolean_result(&registry,"brep.measure.volume",args,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-row["outputs"]["shape"]["volume"].as_f64().unwrap()).abs()<1e-9);
        caller.retire(stock());
    }
}

#[test]
fn original_overlapping_sphere_booleans_share_a_signed_native_rim_and_previews() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_framework_3d_brep_vector::Pnt3;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json")).unwrap();
    let rows=fixture["cases"].as_array().unwrap().iter().filter(|row|row["name"].as_str().unwrap().starts_with("overlapping sphere"));
    for (i,row) in rows.enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98911+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let radii=["a","b"].map(|slot|row["inputs"][slot]["of"]["radius"].as_f64().unwrap());
        let centers=["a","b"].map(|slot|fixture_xyz(&row["inputs"][slot]["by"]));
        let mut input=Dictionary::new();
        for (index,slot) in ["a","b"].into_iter().enumerate(){
            let sphere=retained_boolean_result(&registry,"brep.prim3d.sphere",Dictionary::new().insert("radius",number(radii[index])),&mut caller,&mut born,&mut free);
            let translate=Dictionary::new().insert("geometry",sphere.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary(centers[index])));
            let placed=retained_boolean_result(&registry,"brep.xform.translate",translate,&mut caller,&mut born,&mut free);
            input=input.insert(slot,placed.get("geometryOut").unwrap().clone());
        }
        let operator=match row["kind"].as_str().unwrap(){
            "brep.boolean.fuse"=>"brep.bool.fuse","brep.boolean.intersect"=>"brep.bool.intersect",_=>"brep.bool.cut",
        };
        let cut=operator=="brep.bool.cut";
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.faces.len(),body.coedges.len(),body.edges.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for _ in 0..500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));Ok(())
            });
            if counts()[1]==before[1]+2{break;}
        }
        assert_eq!(counts()[1],before[1]+2);assert_eq!(counts()[0],before[0]);
        draft.as_mut().unwrap().cancel();
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&result,"solid").unwrap();
        let (labels,rim_label)=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let solid=body.solids.get(id).unwrap();
            assert!(solid.inners.is_empty());
            let shell=body.shells.get(solid.outer).unwrap();
            assert_eq!(shell.faces.len(),2);
            let mut labels=Vec::new();
            for (i,&face) in shell.faces.iter().enumerate(){
                let face=body.faces.get(face).unwrap();
                let Surface::Sphere{frame,radius}=body.surfaces.get(face.surface).unwrap()else{panic!("exact cap expected");};
                assert_eq!(frame.origin.to_array(),centers[i]);
                assert_eq!(*radius,radii[i]);
                assert_eq!(face.flipped,cut&&i==1);
                labels.push(face.label.0.to_string());
            }
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),(3,5,2));
            let mut rims=reach.edges.iter().filter_map(|&id|{
                let edge=body.edges.get(id).unwrap();
                let Curve3::Circle{frame,radius}=body.curves3.get(edge.curve).unwrap()else{return None;};
                (edge.range==(0.,std::f64::consts::TAU)).then_some((id,edge,frame,radius))
            });
            let (id,rim,frame,radius)=rims.next().unwrap();
            assert!(rims.next().is_none());
            let expected=row["rim"].as_array().unwrap();
            assert!(frame.origin.to_array().iter().zip(expected).all(|(a,b)|(a-b.as_f64().unwrap()).abs()<1e-12));
            assert!((*radius-expected[3].as_f64().unwrap()).abs()<1e-12);
            let coedges=body.edge_coedges(id);
            assert_eq!(coedges.len(),2);
            let direction=|id|{
                let usage=body.coedges.get(id).unwrap();
                let face=body.faces.get(body.loops.get(usage.loop_id).unwrap().face).unwrap();
                let surface=body.surfaces.get(face.surface).unwrap();
                let trim=body.curves2.get(usage.pcurve.unwrap()).unwrap();
                assert_eq!(usage.prange,rim.range);
                for t in [0.,1.,std::f64::consts::TAU]{
                    let uv=trim.eval(t);
                    assert!(surface.eval(uv.x,uv.y).distance(body.curves3.get(rim.curve).unwrap().eval(t))<1e-12);
                }
                usage.forward!=face.flipped
            };
            assert_ne!(direction(coedges[0]),direction(coedges[1]));
            Ok((labels,rim.label.0.to_string()))
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert_eq!(mesh.component_references["face"].len(),2);
        assert!(labels.iter().all(|label|mesh.component_references["face"].contains(label)));
        assert!(mesh.component_references["edge"].contains(&rim_label));
        let mut seen=[false;2];
        for (p,n) in mesh.positions.chunks_exact(3).zip(mesh.normals.chunks_exact(3)){
            let point=Pnt3::new(p[0] as f64,p[1] as f64,p[2] as f64);
            let normal=Pnt3::new(n[0] as f64,n[1] as f64,n[2] as f64);
            let offsets=centers.map(|center|point-Pnt3::from_array(center));
            let on=[0,1].map(|i|(offsets[i].norm()-radii[i]).abs()<1e-5);
            assert!(on[0]||on[1]);
            if on[0]!=on[1]{
                let i=usize::from(on[1]);
                assert_eq!(offsets[i].dot(normal)<0.,cut&&i==1);
                seen[i]=true;
            }
        }
        assert_eq!(seen,[true,true]);
        let query=Dictionary::new().insert("geometry",result.get("solid").unwrap().clone());
        let volume=retained_boolean_result(&registry,"brep.measure.volume",query,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-row["outputs"]["shape"]["volume"].as_f64().unwrap()).abs()<1e-9);
        caller.retire(stock());
    }
}

#[test]
fn original_sphere_clips_keep_exact_spherical_bands_and_planar_disks(){
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_curve::Curve3;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,read_xyz,vector_dictionary};
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json")).unwrap();
    let rows=corpus["cases"].as_array().unwrap().iter().filter(|row|row.get("sphericalDisk").is_some()||row.get("sphericalSlab").is_some());
    for (index,row) in rows.enumerate(){
        let (disk_rows,centroid) = if let Some(cap)=row.get("sphericalDisk"){
            (vec![cap],fixture_xyz(&cap["centroid"]))
        }else{
            let band=&row["sphericalSlab"];
            (band["disks"].as_array().unwrap().iter().collect(),fixture_xyz(&band["centroid"]))
        };
        let disks=disk_rows.into_iter().map(|disk|
            (fixture_xyz(&disk["center"]),fixture_xyz(&disk["normal"]),disk["radius"].as_f64().unwrap())
        ).collect::<Vec<_>>();
        let face_count=disks.len()+1;
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:99271+index as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let mut input=Dictionary::new();
        for slot in ["a","b"]{
            let recipe=&row["inputs"][slot];
            let spec=&recipe["of"];
            let (operator,args)=if spec["recipe"]=="sphere"{
                ("brep.prim3d.sphere",Dictionary::new().insert("radius",number(spec["radius"].as_f64().unwrap())))
            }else if spec["recipe"]=="cylinder"{
                let args=Dictionary::new().insert("radius",number(spec["radius"].as_f64().unwrap()))
                    .insert("height",number(spec["height"].as_f64().unwrap()));
                ("brep.prim3d.cylinder",args)
            }else{
                let args=["width","depth","height"].into_iter().enumerate().fold(Dictionary::new(),|args,(i,key)|
                    args.insert(key,number(spec["size"][i].as_f64().unwrap())));
                ("brep.prim3d.box",args)
            };
            let raw=retained_boolean_result(&registry,operator,args,&mut caller,&mut born,&mut free);
            let args=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&recipe["by"]))));
            let placed=retained_boolean_result(&registry,"brep.xform.translate",args,&mut caller,&mut born,&mut free);
            input=input.insert(slot,placed.get("geometryOut").unwrap().clone());
        }
        let operator=if row["kind"]=="brep.boolean.cut"{"brep.bool.cut"}else{"brep.bool.intersect"};
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.faces.len(),body.edges.len(),body.coedges.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for _ in 0..500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=draft.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));
                Ok(())
            });
            if counts()[1]==before[1]+face_count{break;}
        }
        assert_eq!(counts()[1],before[1]+face_count);
        assert_eq!(counts()[0],before[0]);
        draft.as_mut().unwrap().cancel();
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&result,"solid").unwrap();
        let (labels,rim_labels)=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),(2,3,face_count));
            let labels=reach.faces.iter().map(|&id|{
                let face=body.faces.get(id).unwrap();
                assert!(!face.flipped);
                match body.surfaces.get(face.surface).unwrap(){
                    Surface::Plane{frame}=>assert!(disks.iter().any(|(center,normal,_)|
                        frame.origin.to_array()==*center&&frame.z.to_array()==*normal)),
                    Surface::Sphere{..}=>{},
                    _=>panic!("unexpected sphere clip support"),
                }
                face.label.0.to_string()
            }).collect::<Vec<_>>();
            let rims=reach.edges.iter().filter_map(|&id|{
                let edge=body.edges.get(id).unwrap();
                let Curve3::Circle{radius,frame}=body.curves3.get(edge.curve).unwrap()else{return None;};
                (edge.range==(0.,std::f64::consts::TAU)).then_some((id,edge,frame,*radius))
            }).collect::<Vec<_>>();
            assert_eq!(rims.len(),disks.len());
            for (center,_,radius) in &disks{
                let &(id,edge,_,_)=rims.iter().find(|(_,_,frame,actual)|
                    frame.origin.to_array()==*center&&(actual-radius).abs()<1e-12).unwrap();
                let coedges=body.edge_coedges(id);
                assert_eq!(coedges.len(),2);
                let forward=|id|{
                    let trim=body.coedges.get(id).unwrap();
                    let face=body.faces.get(body.loops.get(trim.loop_id).unwrap().face).unwrap();
                    let support=body.surfaces.get(face.surface).unwrap();
                    let pcurve=body.curves2.get(trim.pcurve.unwrap()).unwrap();
                    let curve=body.curves3.get(edge.curve).unwrap();
                    assert_eq!(trim.prange,edge.range);
                    for t in [0.,1.,std::f64::consts::TAU]{
                        let uv=pcurve.eval(t);
                        assert!(support.eval(uv.x,uv.y).distance(curve.eval(t))<1e-12);
                    }
                    trim.forward
                };
                assert_ne!(forward(coedges[0]),forward(coedges[1]));
            }
            Ok((labels,rims.iter().map(|(_,edge,_,_)|edge.label.0.to_string()).collect::<Vec<_>>()))
        }).unwrap();
        let preview=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        preview.validate_component_references().unwrap();
        assert!(labels.iter().all(|label|preview.component_references["face"].contains(label)));
        assert!(rim_labels.iter().all(|label|preview.component_references["edge"].contains(label)));
        for (center,normal,_) in &disks{
            assert!(preview.positions.chunks_exact(3).zip(preview.normals.chunks_exact(3)).any(|(p,n)|
                (0..3).all(|i|(p[i] as f64-center[i]).abs()<1e-6&&(n[i] as f64-normal[i]).abs()<1e-6)));
        }
        let query=||Dictionary::new().insert("geometry",result.get("solid").unwrap().clone());
        for (op,key) in [("brep.measure.area","area"),("brep.measure.volume","volume")]{
            let measure=retained_boolean_result(&registry,op,query(),&mut caller,&mut born,&mut free);
            let expected=row["outputs"]["shape"][key].as_f64().unwrap();
            assert!((read_channel_number(&measure,key).unwrap()-expected).abs()<1e-10);
        }
        let mass=retained_boolean_result(&registry,"brep.measure.centerOfMass",query(),&mut caller,&mut born,&mut free);
        assert!(read_xyz(&mass,"center").unwrap().into_iter().zip(centroid).all(|(a,b)|(a-b).abs()<1e-10));
        caller.retire(stock());
    }
}

#[test]
fn original_mixed_and_parallel_cylinder_booleans_keep_exact_axial_profiles() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json")).unwrap();
    let rows=fixture["cases"].as_array().unwrap().iter().filter(|row|row["name"].as_str().unwrap().starts_with("box-cylinder ")||row["name"].as_str().unwrap().starts_with("parallel-cylinder "));
    for (i,row) in rows.enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98921+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let mut input=Dictionary::new();
        for slot in ["a","b"]{
            let recipe=&row["inputs"][slot];
            let spec=&recipe["of"];
            let (operator,parameters)=if spec["recipe"]=="box"{
                let dims=["width","depth","height"].into_iter().enumerate()
                    .fold(Dictionary::new(),|args,(i,k)|args.insert(k,number(spec["size"][i].as_f64().unwrap())));
                ("brep.prim3d.box",dims)
            }else{
                let dims=Dictionary::new().insert("radius",number(spec["radius"].as_f64().unwrap()))
                    .insert("height",number(spec["height"].as_f64().unwrap()));
                ("brep.prim3d.cylinder",dims)
            };
            let raw=retained_boolean_result(&registry,operator,parameters,&mut caller,&mut born,&mut free);
            let translation=Dictionary::new().insert("geometry",raw.get("solid").unwrap().clone())
                .insert("offset",Value::Dictionary(vector_dictionary(fixture_xyz(&recipe["by"]))));
            let placed=retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
            input=input.insert(slot,placed.get("geometryOut").unwrap().clone());
        }
        let operator=match row["kind"].as_str().unwrap(){
            "brep.boolean.fuse"=>"brep.bool.fuse","brep.boolean.intersect"=>"brep.bool.intersect",_=>"brep.bool.cut",
        };
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.vertices.len(),body.edges.len(),body.faces.len(),body.solids.len()])
        }).unwrap();
        let before=counts();
        let mut draft=Some(admit_live_job(&registry,operator,input.clone()));
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        let mut refused=false;
        for _ in 0..500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                match draft.as_mut().unwrap().step(cx){
                    Err(_)=>refused=true,
                    Ok((step,receipt))=>assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant)),
                }
                Ok(())
            });
            if refused||counts()[2]>before[2]{break;}
        }
        assert_eq!(refused,row.get("fault").is_some());
        assert_eq!(counts()[0],before[0]);
        if !refused{assert!(counts()[2]>before[2]);draft.as_mut().unwrap().cancel();}
        close_live_job(&mut draft,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);
        if refused{caller.retire(stock());continue;}
        let result=retained_boolean_result(&registry,operator,input,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&result,"solid").unwrap();
        let circular=row["retainedProfile"]=="cylinder";
        let members=row["retainedArcPrism"]["members"].as_u64().unwrap_or(0) as usize;
        let labels=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let solid=body.solids.get(id).unwrap();
            assert!(solid.inners.is_empty());
            let faces=&body.shells.get(solid.outer).unwrap().faces;
            assert_eq!(faces.len(),if members>0{members+2}else if circular{3}else{6});
            let cylinders=faces.iter().filter(|&&id|{
                let face=body.faces.get(id).unwrap();
                matches!(body.surfaces.get(face.surface),Some(Surface::Cylinder{..}))
            }).count();
            assert_eq!(cylinders,row["retainedArcPrism"]["arcs"].as_u64().map(|arcs|arcs as usize).unwrap_or(usize::from(circular)));
            if members>0{
                assert!(faces.iter().all(|&face|body.faces.get(face).unwrap().inners.is_empty()));
                let inward=faces.iter().filter(|&&face|body.faces.get(face).unwrap().flipped).count();
                assert_eq!(inward,row["retainedArcPrism"]["inward"].as_u64().unwrap_or(0) as usize);
                let reach=body.reachable_from(&[EntityRef::Solid(id)]);
                assert_eq!((reach.vertices.len(),reach.edges.len()),(2*members,3*members));
                assert!(reach.edges.iter().all(|&edge|body.edge_coedges(edge).len()==2));
            }
            Ok(faces.iter().map(|&id|body.faces.get(id).unwrap().label.0.to_string()).collect::<Vec<_>>())
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(labels.iter().all(|label|mesh.component_references["face"].contains(label)));
        let output=&row["outputs"]["shape"];
        if row["retainedArcPrism"]["inward"]==1{
            let tool=&row["inputs"]["b"];
            let [x,y,_]=fixture_xyz(&tool["by"]);
            let radius=tool["of"]["radius"].as_f64().unwrap();
            assert!(mesh.positions.chunks_exact(3).zip(mesh.normals.chunks_exact(3)).any(|(p,n)|{
                let dx=p[0] as f64-x;
                let dy=p[1] as f64-y;
                (dx.hypot(dy)-radius).abs()<1e-5
                    &&(dx*n[0] as f64+dy*n[1] as f64) < -0.99*radius
            }));
        }
        for axis in 0..3{
            let coordinates=mesh.positions.chunks_exact(3).map(|p|p[axis] as f64);
            let (lo,hi)=coordinates.fold((f64::INFINITY,f64::NEG_INFINITY),|(a,b),n|(a.min(n),b.max(n)));
            assert!((lo-output["bbox"][0][axis].as_f64().unwrap()).abs()<0.045);
            assert!((hi-output["bbox"][1][axis].as_f64().unwrap()).abs()<0.045);
        }
        if members>0{
            let area=output["area"].as_f64().unwrap();
            assert!((preview_area_volume(&mesh).0-area).abs()<0.24_f64.max(0.025*area));
        }
        for (operator,field) in [("brep.measure.area","area"),("brep.measure.volume","volume")]{
            let query=Dictionary::new().insert("geometry",result.get("solid").unwrap().clone());
            let measure=retained_boolean_result(&registry,operator,query,&mut caller,&mut born,&mut free);
            assert!((read_channel_number(&measure,field).unwrap()-output[field].as_f64().unwrap()).abs()<1e-9);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_box_bores_publish_exact_blind_floors_and_closed_void_shells() {
    use flow_extension_sdk::geometry::GeometryWorkIdentity;
    use semio_framework_3d_brep_representation::topology::EntityRef;
    use semio_framework_3d_brep_surface::Surface;
    use semio_s_spatial_kernel_semio_session::{read_channel_number,read_geometry,vector_dictionary};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/💡️inferences/📐️geometry/🔗️brep-boolean/🔣️.json")).unwrap();
    let rows=fixture["cases"].as_array().unwrap().iter().filter(|row|row["name"].as_str().unwrap().starts_with("box bore "));
    for (i,row) in rows.enumerate(){
        let session=Session::new();
        let registry=neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let identity=GeometryWorkIdentity{operation:98931+i as u64,generation:17};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(identity.operation),Generation(identity.generation),stock(),||Some(1000),1500,131072);
        let (mut born,mut free)=(0,0);
        let size=&row["inputs"]["a"]["size"];
        let args=["width","depth","height"].into_iter().enumerate()
            .fold(Dictionary::new(),|d,(i,k)|d.insert(k,number(size[i].as_f64().unwrap())));
        let box_=retained_boolean_result(&registry,"brep.prim3d.box",args,&mut caller,&mut born,&mut free);
        let spec=&row["inputs"]["b"]["of"];
        let args=Dictionary::new().insert("radius",number(spec["radius"].as_f64().unwrap())).insert("height",number(spec["height"].as_f64().unwrap()));
        let cylinder=retained_boolean_result(&registry,"brep.prim3d.cylinder",args,&mut caller,&mut born,&mut free);
        let tool_origin=fixture_xyz(&row["inputs"]["b"]["by"]);
        let translation=Dictionary::new().insert("geometry",cylinder.get("solid").unwrap().clone())
            .insert("offset",Value::Dictionary(vector_dictionary(tool_origin)));
        let tool=retained_boolean_result(&registry,"brep.xform.translate",translation,&mut caller,&mut born,&mut free);
        let args=Dictionary::new().insert("a",box_.get("solid").unwrap().clone()).insert("b",tool.get("geometryOut").unwrap().clone());
        let [lo,hi]=[0,1].map(|j|row["boreSpan"][j].as_f64().unwrap());
        let top=size[2].as_f64().unwrap();
        let closed=lo>0.&&hi<top;
        let shell_count=1+usize::from(closed);
        let counts=||session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            Ok([kernel.live_handles().len(),body.shells.len(),body.solids.len(),body.faces.len(),body.edges.len()])
        }).unwrap();
        let before=counts();
        let mut job=Some(admit_live_job(&registry,"brep.bool.cut",args.clone()));
        let grant=Grant{maximum_items:1,maximum_copy_bytes:128,..stock()};
        for _ in 0..500{
            run(&mut caller,grant,&mut born,&mut free,|cx|{
                let (step,receipt)=job.as_mut().unwrap().step(cx).unwrap();
                assert!(matches!(step,neural_engine::OperatorJobStep::Working(_))&&receipt.fits(grant));Ok(())
            });
            if counts()[1]==before[1]+shell_count{break;}
        }
        assert_eq!(counts()[1],before[1]+shell_count);assert_eq!(counts()[0],before[0]);
        job.as_mut().unwrap().cancel();
        close_live_job(&mut job,&mut caller,&mut born,&mut free);
        assert_eq!(counts(),before);

        let result=retained_boolean_result(&registry,"brep.bool.cut",args,&mut caller,&mut born,&mut free);
        let handle=read_geometry(&result,"solid").unwrap();
        let labels=session.with_kernel_read(|kernel|{
            let body=kernel.representation();
            let id=kernel.geometry_solid(&handle).unwrap();
            let solid=body.solids.get(id).unwrap();
            assert_eq!(solid.inners.len(),usize::from(closed));
            assert_eq!(body.shells.get(solid.outer).unwrap().faces.len(),if closed{6}else{8});
            if closed{assert_eq!(body.shells.get(solid.inners[0]).unwrap().faces.len(),3);}
            let reach=body.reachable_from(&[EntityRef::Solid(id)]);
            assert_eq!((reach.vertices.len(),reach.edges.len(),reach.faces.len()),(10,15,8+usize::from(closed)));
            let mut floor_count=0;
            let mut hole_count=0;
            let mut labels=Vec::new();
            for &id in &reach.faces{
                let face=body.faces.get(id).unwrap();
                labels.push(face.label.0.to_string());
                match body.surfaces.get(face.surface).unwrap(){
                    Surface::Cylinder{frame,radius}=>{
                        assert_eq!(frame.origin.to_array(),[tool_origin[0],tool_origin[1],lo]);
                        assert_eq!(*radius,spec["radius"].as_f64().unwrap());
                        assert!(face.flipped);
                    }
                    Surface::Plane{frame}=>{
                        hole_count+=face.inners.len();
                        let z=frame.origin.z;
                        if z>0.&&z<top{
                            assert!(face.inners.is_empty());
                            let direction=if face.flipped{-frame.z}else{frame.z};
                            assert_eq!(direction.z,if z==lo{1.}else{-1.});
                            assert!(z==lo||z==hi);
                            floor_count+=1;
                        }
                    }
                    _=>panic!("expected planar and cylindrical bore support"),
                }
            }
            assert_eq!(hole_count,usize::from(!closed));
            assert_eq!(floor_count,1+usize::from(closed));
            assert!(reach.edges.iter().all(|&id|body.edge_coedges(id).len()==2));
            Ok(labels)
        }).unwrap();
        let mesh=run_funded_preview(&session,identity,handle.as_str(),&mut caller,&mut born,&mut free);
        mesh.validate_component_references().unwrap();
        assert!(labels.iter().all(|label|mesh.component_references["face"].contains(label)));
        for (level,normal) in [(lo,1.0_f32),(hi,-1.0_f32)]{
            if level>0.&&level<top{
                assert!(mesh.positions.chunks_exact(3).zip(mesh.normals.chunks_exact(3)).any(|(p,n)|
                    (p[0] as f64-tool_origin[0]).abs()<1e-5&&(p[1] as f64-tool_origin[1]).abs()<1e-5
                    &&(p[2] as f64-level).abs()<1e-5&&n[2]==normal));
            }
        }
        let output=&row["outputs"]["shape"];
        assert!((preview_area_volume(&mesh).0-output["area"].as_f64().unwrap()).abs()<0.2);
        let args=Dictionary::new().insert("geometry",result.get("solid").unwrap().clone());
        let volume=retained_boolean_result(&registry,"brep.measure.volume",args,&mut caller,&mut born,&mut free);
        assert!((read_channel_number(&volume,"volume").unwrap()-output["volume"].as_f64().unwrap()).abs()<1e-9);
        caller.retire(stock());
    }
}
