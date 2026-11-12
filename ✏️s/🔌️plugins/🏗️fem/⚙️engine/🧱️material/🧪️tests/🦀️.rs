use super::FemMaterial;
use super::FemMaterialValue;
use super::NATIVE_MATERIAL_DIMENSION;
#[path="../📋️table/🧪️tests/🦀️.rs"]mod table;
#[path="../🩹️patch/🧪️tests/🦀️.rs"]mod patch;
#[path="../../../../../../🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]pub(crate)mod driver;
use semio_framework_value::{DslValue, FromValue, ToValue, RetainedAllocationEffect, RetirementDemand, ValueError, retirement::RetireOwned, retained_clone::{OriginalEffectReply, OriginalEffectReplyComplete, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep, RetainedWorkPoll}};
use semio_framework_value_native_owned::{native_decoding::NativeReceive, owned::NativeReceiver};
use semio_framework_value_testing_admission::{EffectBank, session_driven, session_open};
use semio_framework_value_snapshot_projection::snapshot_projection::{ProjectionField, SnapshotProjection};
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
pub(crate) const BODY:usize=4096;
pub(crate) const MAXIMUM:u64=1<<20;
pub(crate) type Call<'t,F>=dyn for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,OriginalEffectReply<'r,RetainedCloneStep,F>,bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,F>,ValueError>+'t;
thread_local!{static BANK:std::cell::RefCell<EffectBank>=std::cell::RefCell::new(driver::bank());}
pub(crate) fn with_bank<R>(run:impl FnOnce(&mut EffectBank)->R)->R{BANK.with(|bank|run(&mut bank.borrow_mut()))}
pub(crate) fn source<T:RetireOwned+Sync+'static>(value:T)->RetainedCloneSource<T>{with_bank(|bank|driver::admit(bank,value))}
pub(crate) fn close_source<T:RetireOwned+Sync+'static>(value:RetainedCloneSource<T>){with_bank(|bank|driver::close_source(value,bank))}
pub(crate) fn drain<T:RetireOwned>(value:T){with_bank(|bank|driver::retire(value,bank))}
pub(crate) fn exact(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
pub(crate) fn denied(d:RetirementDemand)->Vec<RetainedCloneGrant>{let g=exact(d);[(true,RetainedCloneGrant{maximum_items:0,..g}),(d.copy_bytes>0,RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes.saturating_sub(1),..g}),(d.capacity_bytes>0,RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g}),(d.release_bytes>0,RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..g}),(d.depth>0,RetainedCloneGrant{maximum_depth:d.depth.saturating_sub(1),..g})].into_iter().filter_map(|(active,g)|active.then_some(g)).collect()}
pub(crate) fn drive<F:Send+std::fmt::Debug+'static>(g:RetainedCloneGrant,work:u64,call:&mut Call<'_,F>)->(Option<RetainedCloneStep>,(usize,usize)){
 let(poll,heap)=with_bank(|bank|{session_open(bank);let((poll,_),heap)=observe(||session_driven::<F>(bank,g,work.max(1),|effect,reply,recover|call(effect,reply,recover)));(poll,heap)});
 match poll{RetainedWorkPoll::Complete(turn)=>(Some(turn.result.unwrap_or_else(|error|panic!("{error:?}"))),heap),RetainedWorkPoll::Pending{..}=>(None,heap)}
}
pub(crate) fn paid<F:Send+std::fmt::Debug+'static>(d:RetirementDemand,work:u64,call:&mut Call<'_,F>)->RetainedCloneStep{
 for g in denied(d){let(step,heap)=drive::<F>(g,work,call);assert_eq!(heap,(0,0));assert_eq!(step.map(|s|s.progress()).unwrap_or_default(),Default::default());}
 let g=exact(d);for _ in 0..1024{let(step,heap)=drive::<F>(g,work,call);if let Some(step)=step{let p=step.progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));with_bank(|bank|bank.post(p,g,heap));return step;}}
 panic!("material funded turn stayed pending");
}
pub(crate) fn turns<T:NativeReceive<I>,I:RetireOwned+Sync+'static>(input:impl Fn()->I)->u64{
 let original=source(input());let mut receiver=T::receiver();let mut trace=driver::Trace::default();
 with_bank(|bank|{if let Ok(Some(output))=driver::receive_traced(&mut receiver,original.borrow(),bank,BODY,None,None,&mut trace){driver::retire(output,bank);}driver::close(&mut receiver,bank,BODY);});
 drop(receiver);close_source(original);trace.advance.len() as u64
}
pub(crate) fn close_projection<T:SnapshotProjection>(mut field:ProjectionField<T>){
 for _ in 0..100000{
  if field.terminal_is_empty(){return;}
  let started=field.close_started();let(d,work)=if started{(field.close_demands(BODY).unwrap(),field.close_work_demand(BODY).unwrap())}else{(field.begin_close_demand().unwrap(),field.begin_close_work_demand().unwrap())};
  paid::<semio_framework_value::OriginalRetirementFailure>(d,work,&mut|effect,reply,recover|match(started,recover){(false,false)=>field.begin_close_into(effect,reply),(false,true)=>field.recover_begin_close_into(effect,reply),(true,false)=>field.close_into(BODY,effect,reply),(true,true)=>field.recover_close_into(BODY,effect,reply)});
 }
 panic!("projection failed its funded close");
}
fn receive<I:RetireOwned+Sync+'static>(input:impl Fn()->I,expected:&serde_json::Value)where FemMaterial:NativeReceive<I>{
 let mut finished=false;
 for cut in 0..=turns::<FemMaterial,I>(&input)+1{
  let original=source(input());let mut receiver=<FemMaterial as NativeReceive<I>>::receiver();
  with_bank(|bank|driver::none(driver::receive_cut(&mut receiver,original.borrow(),bank,BODY,Some(cut))).unwrap());
  if receiver.output_ready(){
   finished=true;let material=with_bank(|bank|driver::receive_cut(&mut receiver,original.borrow(),bank,BODY,None)).unwrap().expect("uncut receive publishes");
   assert_eq!(material.id.as_str(),expected["id"].as_str().unwrap());assert_eq!(material.name.as_str(),expected["name"].as_str().unwrap());let value=material.to_value();for key in ["e","g","nu","rho"]{if expected.get(key).is_some(){assert_eq!(value.get(key).unwrap().as_f64().unwrap().to_bits(),expected[key].as_f64().unwrap().to_bits());}}drain(value);drain(material);
  }
  with_bank(|bank|driver::close(&mut receiver,bank,BODY));assert!(receiver.terminal_is_empty());drop(receiver);close_source(original);
  if finished{break;}
 }assert!(finished);
}
#[test]
fn native_material_receives_canonical_and_authored_original_values(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let row=&law["accepted"].as_array().unwrap().iter().find(|case|case["dimension"].as_u64()==Some(NATIVE_MATERIAL_DIMENSION as u64)).unwrap()["row"];
 receive(||DslValue::from(row),row);receive(||FemMaterialValue::from_value(DslValue::from(row)).unwrap(),row);
}
#[test]
fn native_material_receives_actual_declared_text_ordinals(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let row=&law["accepted"].as_array().unwrap().iter().find(|case|case["dimension"].as_u64()==Some(NATIVE_MATERIAL_DIMENSION as u64)).unwrap()["row"];
 receive(||{let mut record=semio_framework_dsl_record::RecordValue::default();let keys: &[&str]=if NATIVE_MATERIAL_DIMENSION==2{&["id","name","e","nu","rho"]}else{&["id","name","e","g","nu","rho"]};for(ordinal,key)in keys.iter().enumerate(){record.fields.insert(u16::try_from(ordinal).unwrap(),if let Some(text)=row[key].as_str(){semio_framework_dsl_record::FieldValue::Text(text.to_owned())}else{semio_framework_dsl_record::FieldValue::Float(row[key].as_f64().unwrap())});}record},row);
}
#[test]
fn native_material_refuses_invalid_parts_without_dropping_partial_fields(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let mut refused=0;
 for case in law["refused"].as_array().unwrap().iter().filter(|case|case["dimension"].as_u64()==Some(NATIVE_MATERIAL_DIMENSION as u64)){
  let original=source(DslValue::from(&case["row"]));let mut receiver=<FemMaterial as NativeReceive<DslValue>>::receiver();
  assert!(with_bank(|bank|driver::receive_cut(&mut receiver,original.borrow(),bank,BODY,None)).is_err());assert!(!receiver.output_ready());
  assert_eq!(original.borrow().get(),&DslValue::from(&case["row"]));
  with_bank(|bank|driver::close(&mut receiver,bank,BODY));assert!(receiver.terminal_is_empty());drop(receiver);close_source(original);refused+=1;
 }assert!(refused>0);
}
