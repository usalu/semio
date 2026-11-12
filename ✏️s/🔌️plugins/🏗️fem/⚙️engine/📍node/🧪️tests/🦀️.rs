//! 🧪️ Actual native node values share original identity and return partial receiving custody under granted closure.
use super::FemNode;
use super::FemNodeValue;
use super::DIMENSION;
use semio_framework_value::{DslValue, FromValue, OriginalRetirementFailure, RetainedWorkPoll, RetirementDemand, retirement::RetireOwned, retained_clone::{RetainedCloneGrant, RetainedCloneRef}};
use semio_framework_value_snapshot_projection::snapshot_projection::ProjectionField;
use semio_framework_value_native_owned::{native_decoding::NativeReceive, owned::NativeReceiver};
use semio_framework_value_testing_admission::{EffectBank, session_driven, session_open};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
#[path="../../../../../../🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
const BODY:usize=4096;
const MAXIMUM:u64=1<<20;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn settle(bank:&mut EffectBank,g:RetainedCloneGrant,heap:(usize,usize),poll:RetainedWorkPoll<semio_framework_value::retained_clone::RetainedCloneStep,OriginalRetirementFailure>){if let RetainedWorkPoll::Complete(turn)=poll{let step=turn.result.unwrap_or_else(|_|panic!("node projection turn refused"));bank.post(step.progress(),g,heap);}}
fn project(original:RetainedCloneRef<'_,FemNode>,bank:&mut EffectBank)->ProjectionField<FemNode>{
 let mut field=ProjectionField::<FemNode>::default();session_open(bank);
 for _ in 0..100000{if field.ready(){break;}let g=grant(field.demands(original,BODY,MAXIMUM).unwrap());let work=field.advance_work_demand(original,BODY,MAXIMUM).unwrap();let((poll,_),heap)=observe(||session_driven::<OriginalRetirementFailure>(bank,g,work.max(1),|effect,reply,recover|if recover{field.recover_advance_into(original,BODY,MAXIMUM,effect,reply)}else{field.advance_into(original,BODY,MAXIMUM,effect,reply)}));settle(bank,g,heap,poll);}
 assert!(field.ready(),"node projection stalled");field
}
fn close_projection(mut field:ProjectionField<FemNode>,bank:&mut EffectBank){
 for _ in 0..100000{
  if field.terminal_is_empty(){return;}
  let(d,work)=if field.close_started(){(field.close_demands(BODY).unwrap(),field.close_work_demand(BODY).unwrap())}else{(field.begin_close_demand().unwrap(),field.begin_close_work_demand().unwrap())};let g=grant(d);
  let((poll,_),heap)=observe(||session_driven::<OriginalRetirementFailure>(bank,g,work.max(1),|effect,reply,recover|match(field.close_started(),recover){(false,false)=>field.begin_close_into(effect,reply),(false,true)=>field.recover_begin_close_into(effect,reply),(true,false)=>field.close_into(BODY,effect,reply),(true,true)=>field.recover_close_into(BODY,effect,reply)}));settle(bank,g,heap,poll);

 }
 panic!("node projection failed its funded close");
}
fn run<I:RetireOwned+Sync+'static>(input:impl Fn()->I,expected:&serde_json::Value,accepted:bool)where FemNode:NativeReceive<I>{
 let mut completed=false;let mut refused=false;
 let cuts={let mut bank=driver::bank();let original=driver::admit(&mut bank,input());let mut receiver=<FemNode as NativeReceive<I>>::receiver();let mut trace=driver::Trace::default();if let Ok(Some(output))=driver::receive_traced(&mut receiver,original.borrow(),&mut bank,BODY,None,None,&mut trace){driver::retire(output,&mut bank);}driver::close(&mut receiver,&mut bank,BODY);drop(receiver);driver::close_source(original,&mut bank);
  let total=trace.advance.len() as u64;let mut cuts=std::collections::BTreeSet::from([0u64,1,2]);let mut seen=std::collections::HashSet::new();for index in 1..trace.shapes.len(){if seen.insert((trace.shapes[index-1],trace.shapes[index])){cuts.extend([index as u64,index as u64+1])}}cuts.extend([total.saturating_sub(1),total,total+1]);cuts};
 for cut in cuts{
  let mut bank=driver::bank();let original=driver::admit(&mut bank,input());let foreign=driver::admit(&mut bank,input());
  let mut receiver=<FemNode as NativeReceive<I>>::receiver();
  match driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,Some(cut)){Ok(published)=>assert!(published.is_none()),Err(_)=>{assert!(!accepted);refused=true;}}
  if !refused{
   if receiver.output_ready(){
    assert!(accepted);let output=driver::receive_cut(&mut receiver,original.borrow(),&mut bank,BODY,None).unwrap().expect("uncut receive publishes");
    assert_eq!(output.id.as_str(),expected["id"].as_str().unwrap());assert_eq!(output.x.to_bits(),expected["x"].as_f64().unwrap().to_bits());assert_eq!(output.y.to_bits(),expected["y"].as_f64().unwrap().to_bits());completed=true;
    let row=driver::admit(&mut bank,output);let pointer=std::ptr::from_ref(row.borrow().get().id.get());
    let field=project(row.borrow(),&mut bank);assert_eq!(std::ptr::from_ref(field.value().unwrap().id.get()),pointer);close_projection(field,&mut bank);
    driver::close_source(row,&mut bank);
   }else if cut>0{assert!(receiver.demands(foreign.borrow(),BODY,MAXIMUM).is_err());}
  }
  driver::close(&mut receiver,&mut bank,BODY);assert!(receiver.terminal_is_empty());drop(receiver);driver::close_source(original,&mut bank);driver::close_source(foreign,&mut bank);
  if completed||refused{break;}
 }
 assert_eq!(completed,accepted);assert_eq!(refused,!accepted);
}
#[test]
fn native_node_original_receiving_projection_and_every_cancelled_frontier(){let f:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for row in f["cases"].as_array().unwrap().iter().filter(|row|row["dimension"].as_u64()==Some(DIMENSION as u64)){let accepted=row["accepted"].as_bool().unwrap();run(||DslValue::from(&row["row"]),&row["row"],accepted);if accepted{run(||FemNodeValue::from_value(DslValue::from(&row["row"])).unwrap(),&row["row"],true);}}}
