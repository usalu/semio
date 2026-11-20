//! 🧵️ Original operator retirement and the published native guest path.

use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_effect_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkPoll};
use semio_framework_value::retirement::controlled::ControlledRetirement;

include!("🧰️original-support.rs");

#[test]
fn actual_operator_jobs_close_under_original_one_fuel_512_items_and_cancellation(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️original-close.json")).unwrap();
 for name in corpus["cases"].as_array().unwrap(){for cancel in [None,Some(0),Some(1),Some(3)]{
  let((job,registry,mut session),a,r)=observe(||fixture(name.as_str().unwrap()));let original=a-r;let mut job=Some(job);let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock(),||Some(1000),1500,32768);let(mut born,mut free)=(0,0);let mut calls=0;
  for turn in 0..200{calls=turn+1;if cancel==Some(turn){caller.cancel().cancel_now()}let Some(owner)=job.as_ref()else{break};let d=if owner.terminal_is_empty(){RetirementDemand{release_bytes:std::mem::size_of_val(&**owner),depth:1,..Default::default()}}else{demand(&**owner,stock().maximum_copy_bytes)};
   run(&mut caller,Grant{maximum_items:0,..stock()},&mut born,&mut free,|cx|{let step=close_original_effect_step(cx,d,|effect|job.as_mut().unwrap().close_step(effect))?;assert_eq!(step.progress(),Progress::default());Ok(())});
   run(&mut caller,Grant{maximum_items:512,..stock()},&mut born,&mut free,|cx|drive_close(&mut job,d,cx));
   if job.is_none(){break}
  }
  let within_opportunity=job.is_none();
  for _ in 0..100000{let Some(owner)=job.as_ref()else{break};let d=if owner.terminal_is_empty(){RetirementDemand{release_bytes:std::mem::size_of_val(&**owner),depth:1,..Default::default()}}else{demand(&**owner,stock().maximum_copy_bytes)};run(&mut caller,stock(),&mut born,&mut free,|cx|drive_close(&mut job,d,cx));}
  let mut registry=ControlledRetirement::new(registry).unwrap_or_else(|_|unreachable!("the actual registry has typed physical retirement"));for _ in 0..100000{if registry.terminal_is_empty(){break}let d=registry.selected_demand(stock().maximum_copy_bytes).unwrap();run(&mut caller,stock(),&mut born,&mut free,|cx|{close_original_effect_step(cx,d,|effect|registry.step(effect.grant().maximum_copy_bytes,effect))?;Ok(())});}let registry_closed=registry.terminal_is_empty();drop(registry);
  for _ in 0..100000{if session.terminal_is_empty(){break}let d=RetirementDemand{copy_bytes:session.next_close_copy_byte_demand().unwrap(),capacity_bytes:session.next_close_capacity_byte_demand(stock().maximum_copy_bytes).unwrap(),release_bytes:session.next_close_release_byte_demand().unwrap(),depth:session.next_close_depth_demand().unwrap()};run(&mut caller,stock(),&mut born,&mut free,|cx|{close_original_effect_step(cx,d,|effect|session.close_original_step(effect))?;Ok(())});}let session_closed=session.terminal_is_empty();drop(session);caller.retire(stock());assert!(job.is_none()&&registry_closed&&session_closed);assert_eq!(original+born,free);assert!(within_opportunity,"actual {name} exceeded the original 200 calls: {calls}");
 }}
}

#[test]
fn box_publishes_native_solid_and_retires_interrupted_topology() {
    let case: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solids.json")).unwrap();
    let dimensions: [f64; 3] = std::array::from_fn(|i| case["dimensions"][i].as_f64().unwrap());
    let volume = case["volume"].as_f64().unwrap();
    let face_count = case["faceCount"].as_u64().unwrap() as usize;
    for cancelled in [false, true] {
        let session = Session::new();
        let registry = neural_engine::ColdOwner::new(semio_s_plugin_flow_brep::module_registry(&session));
        let input = ["width", "depth", "height"].into_iter().zip(dimensions)
            .fold(Dictionary::new(), |input, (name, value)| input.insert(name, number(value)));
        let mut job = Some(admit_live_job(&registry, "brep.prim3d.box", input));
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(97149), Generation(17), stock(), || Some(1000), 1500, 32768);
        let (mut born, mut free) = (0, 0);
        let mut output = None;
        let mut topology_turns = 0;
        for _ in 0..1000 {
            let owner = job.as_ref().unwrap();
            if cancelled && topology_turns == 12 { caller.cancel().cancel_now(); }
            let granter = Grant {
                maximum_items: 1,
                maximum_copy_bytes: owner.next_step_copy_byte_demand().unwrap().max(64),
                ..stock()
            };
            let mut stopped = false;
            run(&mut caller, granter, &mut born, &mut free, |cx| {
                let (step, _) = job.as_mut().unwrap().step(cx).unwrap();
                match step {
                    neural_engine::OperatorJobStep::Done(result) => output = Some(result),
                    neural_engine::OperatorJobStep::Cancelled(_) => stopped = true,
                    neural_engine::OperatorJobStep::Working(progress) => {
                        if progress.phase == "solid-primitive-topology" { topology_turns += 1; }
                    }
                }
                Ok(())
            });
            if output.is_some() || stopped { break; }
            assert!(session.with_kernel_read(|kernel| Ok(kernel.live_handles().is_empty())).unwrap());
        }
        assert!(topology_turns >= 12);
        if cancelled {
            assert!(output.is_none());
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
            session.with_kernel_read(|kernel| {
                assert!(kernel.live_handles().is_empty());
                let body = kernel.representation();
                assert!(body.vertices.is_empty() && body.edges.is_empty() && body.faces.is_empty() && body.solids.is_empty());
                Ok(())
            }).unwrap();
        } else {
            let output = neural_engine::ColdOwner::new(output.expect("retained Box publishes a solid"));
            let handle = semio_s_spatial_kernel_semio_session::read_geometry(&output, "solid").unwrap();
            session.with_kernel_read(|kernel| {
                let map = |error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error);
                let solid = kernel.geometry_solid(&handle).map_err(map)?;
                assert_eq!(kernel.representation().solid_faces(solid).len(), face_count);
                assert!((kernel.volume_sync(&handle).map_err(map)? - volume).abs() < 1e-9);
                Ok(())
            }).unwrap();
            close_live_job(&mut job, &mut caller, &mut born, &mut free);
        }
        caller.retire(stock());
    }
}

#[test]
fn original_box_guest_evaluates_and_tessellates_the_published_solid() {
    let case: serde_json::Value = serde_json::from_str(include_str!("🔣️original-solids.json")).unwrap();
    let dimensions = case["dimensions"].as_array().unwrap();
    let number = |value: &serde_json::Value| serde_json::json!({ "$schema": "number", "value": value });
    let input = serde_json::json!({
        "width": number(&dimensions[0]), "depth": number(&dimensions[1]), "height": number(&dimensions[2])
    });
    let initial = serde_json::to_vec(&serde_json::json!({
        "neuronId": "native.box", "operatorId": "brep.prim3d.box", "nodeHash": 45057,
        "inputJson": input.to_string(), "roundUnits": 1
    })).unwrap();
    let resume = serde_json::to_vec(&serde_json::json!({
        "neuronId": "native.box", "operatorId": "brep.prim3d.box", "nodeHash": 45057,
        "inputJson": "", "resume": true, "roundUnits": 1
    })).unwrap();
    let mut owner = semio_s_plugin_flow_brep::extension_guest::bundle();
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97161), Generation(17), stock(), || Some(1000), 1500, 1_000_000);
    let (mut born, mut free) = (0, 0);
    let mut result = invoke_box_guest(&mut owner, "evaluate", &initial, &mut caller, &mut born, &mut free);
    for _ in 0..2000 {
        if result["done"] == true { break; }
        result = invoke_box_guest(&mut owner, "evaluate", &resume, &mut caller, &mut born, &mut free);
    }
    assert_eq!(result["done"], true);
    let solid = &result["output"]["solid"];
    assert_eq!(solid["kind"], "solid");
    let request = serde_json::to_vec(&serde_json::json!({
        "handle": solid["handle"].as_str().unwrap(), "tolerance": 0.05, "nodeHash": 45057,
        "windowId": "native.box.preview", "windowKindId": "flow"
    })).unwrap();
    let preview = invoke_box_guest(&mut owner, "tessellate", &request, &mut caller, &mut born, &mut free);
    assert_eq!(preview["done"], true);
    let mesh = &preview["mesh"];
    let scalars = mesh["positions"].as_array().unwrap();
    let vector = |index: &serde_json::Value| {
        let at = index.as_u64().unwrap() as usize * 3;
        semio_framework_3d_brep_vector::Vec3::new(scalars[at].as_f64().unwrap(), scalars[at+1].as_f64().unwrap(), scalars[at+2].as_f64().unwrap())
    };
    let indices = mesh["indices"].as_array().unwrap();
    assert_eq!(indices.len() / 3, 2 * case["faceCount"].as_u64().unwrap() as usize);
    let signed: f64 = indices.chunks_exact(3).map(|triangle| vector(&triangle[0]).dot(vector(&triangle[1]).cross(vector(&triangle[2]))) / 6.0).sum();
    let half = |axis: usize| dimensions[axis].as_f64().unwrap() as f32 / 2.0;
    let oracle = parry3d::shape::Shape::mass_properties(
        &parry3d::shape::Cuboid::new(parry3d::math::Vector::new(half(0), half(1), half(2))), 1.0,
    ).mass();
    assert_eq!(f64::from(oracle), case["volume"].as_f64().unwrap());
    assert!((signed - f64::from(oracle)).abs() < 1e-8);
    assert_eq!(mesh["componentReferences"]["face"].as_array().unwrap().len(), 6);
    assert_eq!(mesh["componentReferences"]["edge"].as_array().unwrap().len(), 12);
    assert_eq!(mesh["componentReferences"]["vertex"].as_array().unwrap().len(), 8);
    owner.begin_close();
    for _ in 0..100_000 {
        if owner.terminal_is_empty() { break; }
        let demand = owner.retirement_demands(64).unwrap();
        let grant = Grant {
            maximum_items: 1, maximum_copy_bytes: demand.copy_bytes.max(64), maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth,
        };
        run(&mut caller, grant, &mut born, &mut free, |cx| { owner.close_step(cx)?; Ok(()) });
    }
    assert!(owner.terminal_is_empty());
    caller.retire(stock());
}

fn fixture(name:&str)->(Box<dyn OperatorJob>,Option<Registry>,SessionCapture){
 let session=Session::new();
 if name.starts_with("mesh-"){let mut job=semio_s_plugin_flow_brep_mesh::MeshOperatorJob::output(semio_framework_3d::mesh::HalfedgeMesh::empty()).unwrap();if name=="mesh-held-fault"{job.fault=Some(neural_engine::EvalError::InvalidInput("original semantic fault雪".repeat(17)))}return(Box::new(job),None,SessionCapture::owned(session))}
 let registry=semio_s_plugin_flow_brep::module_registry(&session);let primitive=registry.dispatch_cold("brep.prim3d.box",Dictionary::new().insert("width",number(1.0)).insert("depth",number(1.0)).insert("height",number(1.0))).unwrap();let geometry=primitive.get("solid").unwrap().as_dictionary().unwrap().clone();let mut input=Some(Dictionary::new().insert("a",Value::Dictionary(geometry.clone())).insert("b",Value::Dictionary(geometry)));drop(primitive);let mut cursor=OperatorPlanCursor::new();let mut job=None;
 for _ in 0..100000{match registry.dispatch_job("brep.bool.fuse",&mut input,&mut cursor,stock()).unwrap(){OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(value))=>{job=Some(value);break},OperatorPlanStep::Working=>{},_=>panic!("the actual Boolean operator admits its actual retained job")}}
 (job.unwrap(),Some(registry),SessionCapture::owned(session))
}

fn invoke_box_guest(owner: &mut semio_plugin::ExtensionBundle, capability: &str, source: &[u8],
    caller: &mut OriginalJobFixtureCaller, born: &mut usize, free: &mut usize,
) -> serde_json::Value {
    let mut payload = None;
    for _ in 0..100_000 {
        run(caller, stock(), born, free, |cx| {
            let turn = owner.invoke(capability, source, cx).unwrap();
            assert!(turn.refusal.is_none(), "{capability} refused its funded invocation");
            payload = turn.payload;
            Ok(())
        });
        if payload.is_some() { break; }
    }
    let payload = payload.expect("guest completes one canonical Pack response");
    let output = pack::record::decode_value_record_body_exact(&payload, 1, &pack::PackLimits::default()).unwrap();
    serde_json::to_value(output).unwrap()
}
