//! 🧪️ Portable bitmap decisions agree with independent classifications and the cold reference.
use super::*;
use protocol::{ColdMutation,DiffAlgebra,RetainedDiffApply};
use semio_framework_value::{retirement::RetireOwned,retained_clone::RetainedCloneSource};
const CASES:&str=include_str!("../../../../🧪️tests/🧫️fixtures/🧳️retained/🔣️.json");
struct Source<T:RetireOwned+Sync+'static>(RetainedCloneSource<T>);
impl<T:RetireOwned+Sync+'static> Source<T>{
 fn new(value:T)->Self{Self(semio_framework_value::testing::admit_owned_source(value,()).0)}
 fn borrow(&self)->RetainedCloneRef<'_,T>{self.0.try_borrow().unwrap()}
 fn close(&mut self){for _ in 0..100000{if self.0.terminal_is_empty(){return;}let q=self.0.next_close_demand(32768).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:q.copy_bytes.max(32768),maximum_capacity_bytes:q.capacity_bytes,maximum_release_bytes:q.release_bytes,maximum_depth:q.depth};let s=self.0.close_step(g).result.unwrap();assert!(s.progress().fits(g));}panic!("original source closure did not settle");}
}
impl<T:RetireOwned+Sync+'static> Drop for Source<T>{fn drop(&mut self){assert!(std::thread::panicking()||self.0.terminal_is_empty());}}
fn grant()->RetainedCloneGrant{let mut g=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation;g.maximum_release_bytes=g.maximum_capacity_bytes;g}
fn close(p:&mut Preparation){let issuer=semio_framework_value::OwnedValueRetirementFactory::<M>::default();let c=RetainedMutationCloseContext{mutations:&issuer};p.begin_close();for _ in 0..100000{if p.terminal_is_empty(){return;}let q=p.close_demand(c,32768).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:q.copy_bytes.max(32768),maximum_capacity_bytes:q.capacity_bytes.max(16<<20),maximum_release_bytes:q.release_bytes.max(16<<20),maximum_depth:q.depth.max(4096),..grant()};let s=p.close_step(c,g).unwrap();assert!(s.progress().fits(g));}panic!("semantic preparation closure did not settle");}
fn decode<T:semio_framework_value::FromValue>(value:&serde_json::Value)->T{semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn decision(p:&mut Preparation,m:&Source<M>,base:&Source<BitmapSnapshot>,cancel:Option<usize>)->Option<(bool,BitmapDiff,Vec<protocol::MutationMessage>,usize)>{
 let g=grant();for turn in 0..100000{if cancel==Some(turn){return None;}let q=p.decision_demand(m.borrow(),base.borrow(),g.maximum_copy_bytes).unwrap();assert!(fits(q,g));let s=p.decide(m.borrow(),base.borrow(),g.maximum_copy_bytes,g).unwrap();assert!(s.progress().fits(g));if matches!(s,RetainedMutationAdvance::Ready(_)){return Some(match p.take_decision(){RetainedMutationDecision::Admitted(d,rows)=>(true,d,rows,turn+1),RetainedMutationDecision::Refused(rows)=>(false,BitmapDiff::default(),rows,turn+1)});}}panic!("bounded fixture decision did not settle");
}

fn apply_retained_post(original:&BitmapSnapshot,diff:BitmapDiff)->BitmapSnapshot{
 use semio_framework_value::retained_clone::RetainedCloneSourceTake;
 let mut base=Source::new(original.clone());let mut post=Source::new(original.clone());let mut ds=Source::new(diff);let mut plan=crate::diff::BitmapApply::new();let g=grant();
 let mut accept=|_|true;let mut receiving=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(16<<20,&mut accept);let mut ready=false;
 for _ in 0..100000{
  let q=plan.preflight_demand(ds.borrow(),base.borrow(),post.borrow(),g.maximum_copy_bytes).unwrap();assert!(fits(q,g));
  let step=protocol::preflight_retained_diff(&mut plan,ds.borrow(),base.borrow(),post.borrow(),g.maximum_copy_bytes,g,&mut receiving).unwrap();assert!(step.progress().fits(g));assert_eq!(post.borrow().get(),original);
  match step{protocol::RetainedDiffPreflightStep::Admitted(_,_)=>{ready=true;break;},protocol::RetainedDiffPreflightStep::Refused(_,_)=>panic!("an admitted semantic delta must apply"),_=>{}}
 }
 assert!(ready);let mut actual=None;
 for _ in 0..100000{match post.0.take_authority(g).result.unwrap(){RetainedCloneSourceTake::Pending(_)=>{},RetainedCloneSourceTake::Ready(value,_)=>{actual=Some(value);break;}}}
 let mut actual=actual.expect("retained preflight releases the mutable post authority");let q=plan.commit_demand(ds.borrow(),base.borrow(),&actual,g.maximum_copy_bytes).unwrap();assert!(fits(q,g));
 assert!(matches!(protocol::commit_retained_diff(&mut plan,ds.borrow(),base.borrow(),&mut actual,g).unwrap(),RetainedCloneStep::Complete(_)));
 plan.begin_close();for _ in 0..100000{if plan.terminal_is_empty(){break;}let q=plan.close_demand(g.maximum_copy_bytes).unwrap();assert!(fits(q,g));let step=plan.close_step(g).unwrap();assert!(step.progress().fits(g));}assert!(plan.terminal_is_empty());ds.close();base.close();post.close();actual
}

#[test]
fn every_portable_decision_and_inverse_matches_its_independent_contract(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let mut variants=std::collections::BTreeSet::new();
 for case in fixture["cases"].as_array().unwrap(){
  let plain=case.get("base").unwrap_or(&fixture["base"]);let original:BitmapSnapshot=decode(plain);let mutation:M=decode(&case["mutation"]);variants.insert(case["mutation"].as_object().unwrap().keys().next().unwrap().clone());
  let cold=mutation.diff(&original);let inverse=mutation.inverse(&original).unwrap();let expected_codes:Vec<&str>=case["expectedCodes"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect();
  assert_eq!(cold.messages().iter().map(|m|m.code.0.as_str()).collect::<Vec<_>>(),expected_codes,"{}",case["id"]);
  let mut m=Source::new(mutation);let mut base=Source::new(original.clone());let mut p=Preparation::new();let(admitted,d,rows,_)=decision(&mut p,&m,&base,None).unwrap();
  assert_eq!(&d,cold.diff(),"{} diff",case["id"]);assert_eq!(rows,cold.messages(),"{} diagnostics",case["id"]);
  let class=if !admitted{"refused"}else if d.is_empty(){"noop"}else{"applied"};assert_eq!(class,case["classification"].as_str().unwrap(),"{}",case["id"]);assert_eq!(base.borrow().get(),&original,"decision never writes its source");
  if admitted{let expected=protocol::apply_mutation(&original,m.borrow().get()).expect("test-only cold post oracle").0;assert_eq!(apply_retained_post(&original,d.clone()),expected,"{} complete retained post",case["id"]);let mut ds=Source::new(d);let g=grant();let mut ready=false;for _ in 0..100000{let q=p.inverse_demand(m.borrow(),base.borrow(),ds.borrow(),g.maximum_copy_bytes).unwrap();assert!(fits(q,g));let s=p.inverse(m.borrow(),base.borrow(),ds.borrow(),g.maximum_copy_bytes,g).unwrap();assert!(s.progress().fits(g));if matches!(s,RetainedMutationAdvance::Ready(_)){ready=true;break;}}assert!(ready);assert_eq!(p.take_inverse(),inverse,"{} inverse order",case["id"]);ds.close();}
  close(&mut p);m.close();base.close();
 }
 assert_eq!(variants.len(),13);
}
#[test]
fn every_pin_assignment_decision_frontier_remains_cancelable(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let case=fixture["cases"].as_array().unwrap().iter().find(|c|c["mutation"].get("PinSolution").is_some()&&c["classification"]=="applied").unwrap();let plain=case.get("base").unwrap_or(&fixture["base"]);let original:BitmapSnapshot=decode(plain);let mutation:M=decode(&case["mutation"]);
 let mut m=Source::new(mutation.clone());let mut base=Source::new(original.clone());let mut p=Preparation::new();let(_,_,_,turns)=decision(&mut p,&m,&base,None).unwrap();close(&mut p);m.close();base.close();
 for cancel in 0..turns{let mut m=Source::new(mutation.clone());let mut base=Source::new(original.clone());let mut p=Preparation::new();assert!(decision(&mut p,&m,&base,Some(cancel)).is_none());assert_eq!(base.borrow().get(),&original);close(&mut p);assert!(p.terminal_is_empty());m.close();base.close();}
}

#[test]
fn every_pin_assignment_inverse_frontier_remains_cancelable(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let case=fixture["cases"].as_array().unwrap().iter().find(|c|c["mutation"].get("PinSolution").is_some()&&c["classification"]=="applied").unwrap();let plain=case.get("base").unwrap_or(&fixture["base"]);let original:BitmapSnapshot=decode(plain);let mutation:M=decode(&case["mutation"]);let g=grant();
 let mut m=Source::new(mutation.clone());let mut base=Source::new(original.clone());let mut p=Preparation::new();let(_,d,_,_)=decision(&mut p,&m,&base,None).unwrap();let mut ds=Source::new(d);let mut turns=0;loop{turns+=1;let s=p.inverse(m.borrow(),base.borrow(),ds.borrow(),g.maximum_copy_bytes,g).unwrap();assert!(s.progress().fits(g));if matches!(s,RetainedMutationAdvance::Ready(_)){break;}assert!(turns<100000);}close(&mut p);ds.close();m.close();base.close();
 for cancel in 0..=turns{let mut m=Source::new(mutation.clone());let mut base=Source::new(original.clone());let mut p=Preparation::new();let(_,d,_,_)=decision(&mut p,&m,&base,None).unwrap();let mut ds=Source::new(d);for _ in 0..cancel{let s=p.inverse(m.borrow(),base.borrow(),ds.borrow(),g.maximum_copy_bytes,g).unwrap();assert!(s.progress().fits(g));}assert_eq!(base.borrow().get(),&original);close(&mut p);assert!(p.terminal_is_empty());ds.close();m.close();base.close();}
}
#[test]
fn an_unfunded_decision_neither_allocates_nor_reads_a_variable_payload(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let case=&fixture["cases"][0];let mut m=Source::new(decode::<M>(&case["mutation"]));let mut base=Source::new(decode::<BitmapSnapshot>(&fixture["base"]));let mut p=Preparation::new();let g=RetainedCloneGrant{maximum_copy_bytes:0,maximum_capacity_bytes:0,..grant()};let s=p.decide(m.borrow(),base.borrow(),0,g).unwrap();assert!(matches!(s,RetainedMutationAdvance::Progress(_)));assert_eq!(s.progress(),RetainedCloneProgress::default());assert_eq!(p.phase,0);assert_eq!(p.capacity(),0);close(&mut p);m.close();base.close();
}
