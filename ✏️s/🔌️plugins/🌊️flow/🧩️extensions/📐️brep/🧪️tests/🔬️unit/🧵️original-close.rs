//! 🧪️ Actual Boolean and Mesh jobs close through the original one-fuel caller and independent physical stock.
use neural_engine::{Atom,Dictionary,Value,OperatorJob,OperatorPlanCursor,OperatorPlanStep,OperatorPlanAdmission,Registry};
use semio_s_spatial_kernel_semio_session::{Session,SessionCapture};
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_retained_step};
use semio_framework_value::{RetainedCloneGrant as Grant,RetainedCloneProgress as Progress,RetainedCloneStep as Step,RetirementDemand,RetainedWorkOutcome};
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn observe<T>(body:impl FnOnce()->T)->(T,usize,usize){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(body);(value,heap.requested_bytes,heap.released_bytes)}
fn stock()->Grant{Grant{maximum_items:65536,maximum_copy_bytes:128<<20,maximum_capacity_bytes:256<<20,maximum_release_bytes:512<<20,maximum_depth:4096}}
fn run(caller:&mut OriginalJobFixtureCaller,g:Grant,born:&mut usize,free:&mut usize,body:impl FnOnce(&mut StepContext<'_>)->Result<(),OriginalTurnFailure>){let(_,a,r)=observe(||caller.turn(g,|cx|cx.with_original_retirement(|cx|cx.with_original_work_loan(1,1500,body))).unwrap());let p=caller.receipt();assert!(p.fits(g));assert_eq!((a,r),(p.retained_capacity_bytes,p.released_bytes));*born+=a;*free+=r;}
fn number(value:f64)->Value{Value::Dictionary(Dictionary::with_schema("number").insert("value",Value::Atom(Atom::Decimal(value))))}
fn fixture(name:&str)->(Box<dyn OperatorJob>,Option<Registry>,SessionCapture){
 let session=Session::new();
 if name.starts_with("mesh-"){let mut job=semio_s_plugin_flow_brep_mesh::MeshOperatorJob::output(semio_framework_3d::mesh::HalfedgeMesh::empty()).unwrap();if name=="mesh-held-fault"{job.fault=Some(neural_engine::EvalError::InvalidInput("original semantic fault雪".repeat(17)))}return(Box::new(job),None,SessionCapture::owned(session))}
 let registry=semio_s_plugin_flow_brep::module_registry(&session);let primitive=registry.dispatch_cold("brep.prim3d.box",Dictionary::new().insert("width",number(1.0)).insert("depth",number(1.0)).insert("height",number(1.0))).unwrap();let geometry=primitive.get("solid").unwrap().as_dictionary().unwrap().clone();let mut input=Some(Dictionary::new().insert("a",Value::Dictionary(geometry.clone())).insert("b",Value::Dictionary(geometry)));drop(primitive);let mut cursor=OperatorPlanCursor::new();let mut job=None;
 for _ in 0..100000{match registry.dispatch_job("brep.bool.fuse",&mut input,&mut cursor,stock()).unwrap(){OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(value))=>{job=Some(value);break},OperatorPlanStep::Working=>{},_=>panic!("the actual Boolean operator admits its actual retained job")}}
 (job.unwrap(),Some(registry),SessionCapture::owned(session))
}
fn demand(job:&dyn OperatorJob,copy:usize)->RetirementDemand{RetirementDemand{copy_bytes:job.next_close_copy_byte_demand().unwrap(),capacity_bytes:job.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:job.next_close_release_byte_demand().unwrap(),depth:job.next_close_depth_demand().unwrap()}}
fn drive_close(job:&mut Option<Box<dyn OperatorJob>>,d:RetirementDemand,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
 if !cx.retained_work_can_enter(d){return Ok(())}if job.as_ref().unwrap().terminal_is_empty(){let bytes=std::mem::size_of_val(&**job.as_ref().unwrap());drop(job.take());cx.consume_fuel(1);cx.consume_retained(Progress{copied_items:1,released_bytes:bytes,..Default::default()})?;}else{close_original_retained_step(cx,d,|g|{let turn=job.as_mut().unwrap().close_step(g);assert!(turn.work<=1);turn})?;}Ok(())
}
#[test]
fn actual_operator_jobs_close_under_original_one_fuel_512_items_and_cancellation(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️original-close.json")).unwrap();
 for name in corpus["cases"].as_array().unwrap(){for cancel in [None,Some(0),Some(1),Some(3)]{
  let((job,registry,mut session),a,r)=observe(||fixture(name.as_str().unwrap()));let original=a-r;let mut job=Some(job);let mut caller=OriginalJobFixtureCaller::admit(OperationId(97146),Generation(17),stock(),||Some(1000),1500,32768);let(mut born,mut free)=(0,0);let mut calls=0;
  for turn in 0..200{calls=turn+1;if cancel==Some(turn){caller.cancel().cancel_now()}let Some(owner)=job.as_ref()else{break};let d=if owner.terminal_is_empty(){RetirementDemand{release_bytes:std::mem::size_of_val(&**owner),depth:1,..Default::default()}}else{demand(&**owner,stock().maximum_copy_bytes)};
   run(&mut caller,Grant{maximum_items:0,..stock()},&mut born,&mut free,|cx|{let turn=job.as_mut().unwrap().close_step(cx.retained_grant());assert_eq!(turn.work,0);assert_eq!(turn.result?.progress(),Progress::default());Ok(())});
   run(&mut caller,Grant{maximum_items:512,..stock()},&mut born,&mut free,|cx|drive_close(&mut job,d,cx));
   if job.is_none(){break}
  }
  let within_opportunity=job.is_none();
  for _ in 0..100000{let Some(owner)=job.as_ref()else{break};let d=if owner.terminal_is_empty(){RetirementDemand{release_bytes:std::mem::size_of_val(&**owner),depth:1,..Default::default()}}else{demand(&**owner,stock().maximum_copy_bytes)};run(&mut caller,stock(),&mut born,&mut free,|cx|drive_close(&mut job,d,cx));}
  let mut registry=ControlledRetirement::new(registry).unwrap_or_else(|_|unreachable!("the actual registry has typed physical retirement"));for _ in 0..100000{if registry.terminal_is_empty(){break}let d=registry.selected_demand(stock().maximum_copy_bytes).unwrap();run(&mut caller,stock(),&mut born,&mut free,|cx|{close_original_retained_step(cx,d,|g|{let result=registry.step(g);RetainedWorkOutcome{result,work:registry.last_turn_work()}})?;Ok(())});}let registry_closed=registry.terminal_is_empty();drop(registry);
  for _ in 0..100000{if session.terminal_is_empty(){break}let d=RetirementDemand{copy_bytes:session.next_close_copy_byte_demand().unwrap(),capacity_bytes:session.next_close_capacity_byte_demand(stock().maximum_copy_bytes).unwrap(),release_bytes:session.next_close_release_byte_demand().unwrap(),depth:session.next_close_depth_demand().unwrap()};run(&mut caller,stock(),&mut born,&mut free,|cx|{close_original_retained_step(cx,d,|g|session.close_original_step(g))?;Ok(())});}let session_closed=session.terminal_is_empty();drop(session);caller.retire(stock());assert!(job.is_none()&&registry_closed&&session_closed);assert_eq!(original+born,free);assert!(within_opportunity,"actual {name} exceeded the original 200 calls: {calls}");
 }}
}
