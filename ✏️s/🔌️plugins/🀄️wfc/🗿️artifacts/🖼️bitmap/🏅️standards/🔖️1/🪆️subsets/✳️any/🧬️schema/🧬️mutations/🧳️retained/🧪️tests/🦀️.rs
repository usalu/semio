//! 🧪️ Portable bitmap decisions agree with independent classifications and the cold reference.
use super::*;
#[path="../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📽️projection/👁️post/🧪️testing/🦀️.rs"]
mod post_fixture;
use post_fixture::PostFixture;
use protocol::{ColdMutation,DiffAlgebra,RetainedDiffOwner,RetainedDiffPreflightStep,RetainedMutationPreparation};
use protocol::{RetainedDone as Done,RetainedReply as Reply};
use semio_framework_value::{OriginalRetirementFailure,RetainedWorkPoll,retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_clone_source_testing::{FixtureSource,drive::{drive,drive_reply}};
const CASES:&str=include_str!("../../../../🧪️tests/🧫️fixtures/🧳️retained/🔣️.json");
const UNITS:u64=1<<20;
const BODY:usize=32768;
/// 🪙️ The deepest grant the finite fixture Source stock funds.
const FIXTURE_DEPTH:usize=1024;
type Source<T>=FixtureSource<T>;
fn source<T:RetireOwned+Sync+'static>(value:T)->Source<T>{FixtureSource::from_owner(value)}
fn grant()->RetainedCloneGrant{let mut g=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation;g.maximum_release_bytes=g.maximum_capacity_bytes;g.maximum_depth=g.maximum_depth.min(FIXTURE_DEPTH);g}
fn drain<T:RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).map_err(|(e,_)|e).unwrap();while !owner.terminal_is_empty(){let d=owner.next_demand(BODY).unwrap();let units=owner.next_computational_work(BODY).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert!(drive(g,units,|effect|owner.step(BODY,effect)).result.unwrap_or_else(|_|panic!("original retirement turn refused")).progress().fits(g));}}
/// 🎟️ Drives one funded preparation turn of `decide`, `inverse` or close inside one actual original action.
fn turn(g:RetainedCloneGrant,call:impl for<'r> FnMut(&mut RetainedAllocationEffect<'_>,Reply<'r>,bool)->Result<Done<'r>,ValueError>)->RetainedCloneStep{
 drive_reply::<RetainedCloneStep,OriginalRetirementFailure>(g,UNITS,|answer|match answer{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},call).result.unwrap_or_else(|_|panic!("original preparation turn refused"))
}
fn close(p:&mut Preparation){
 let issuer:std::sync::Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<M>>=std::sync::Arc::new(semio_framework_value::OwnedValueRetirementFactory::<M>::default());let c=RetainedMutationCloseContext{mutations:&issuer};p.begin_close();
 for _ in 0..100000{if p.terminal_is_empty(){return;}let q=p.close_demand(c,BODY).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:q.copy_bytes.max(BODY),maximum_capacity_bytes:q.capacity_bytes.max(16<<20),maximum_release_bytes:q.release_bytes.max(16<<20),maximum_depth:q.depth.max(FIXTURE_DEPTH),..grant()};let s=turn(g,|effect,reply,recover|p.close_reply(c,None,BODY,effect,reply,recover));assert!(s.progress().fits(g));}
 panic!("semantic preparation closure did not settle");
}
fn decode<T:semio_framework_value::FromValue>(value:&serde_json::Value)->T{semio_s_artifact_wfc_bitmap_snapshot::standards::v1::subsets::any::io::text::snapshot::bitmap_json_decode(&value.to_string()).unwrap()}
fn decision(p:&mut Preparation,m:&Source<M>,base:&Source<BitmapSnapshot>,cancel:Option<usize>)->Option<(bool,BitmapDiff,Vec<protocol::MutationMessage>,usize)>{
 let g=grant();for at in 0..100000{if cancel==Some(at){return None;}let q=p.decision_demand(m.borrow(),base.borrow(),BODY,UNITS).unwrap();assert!(protocol::retained_mutation_demand_fits(q,g));let s=turn(g,|effect,reply,recover|p.decide_reply(m.borrow(),base.borrow(),BODY,UNITS,effect,reply,recover));assert!(s.progress().fits(g));if matches!(s,RetainedCloneStep::Complete(_)){return Some(match p.take_decision(){RetainedMutationDecision::Admitted(d,rows)=>(true,d,rows,at+1),RetainedMutationDecision::Refused(rows)=>(false,BitmapDiff::default(),rows,at+1)});}}
 panic!("bounded fixture decision did not settle");
}
fn inverse_turn(p:&mut Preparation,m:&Source<M>,base:&Source<BitmapSnapshot>,ds:&Source<BitmapDiff>)->RetainedCloneStep{
 let g=grant();let q=p.inverse_demand(m.borrow(),base.borrow(),ds.borrow(),BODY,UNITS).unwrap();assert!(protocol::retained_mutation_demand_fits(q,g));
 let s=turn(g,|effect,reply,recover|p.inverse_reply(m.borrow(),base.borrow(),ds.borrow(),BODY,UNITS,effect,reply,recover));assert!(s.progress().fits(g));s
}

type Owner=RetainedDiffOwner<BitmapSnapshot,BitmapDiff>;
fn preflight(owner:&mut Owner,diff:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:RetainedCloneRef<'_,BitmapSnapshot>,g:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>)->RetainedDiffPreflightStep{
 drive_reply::<RetainedDiffPreflightStep,OriginalRetirementFailure>(g,UNITS,|answer|match answer{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply:Reply<'_,RetainedDiffPreflightStep>,recover|->Result<Done<'_,RetainedDiffPreflightStep>,ValueError>{if recover{protocol::recover_preflight_retained_diff_into(owner,diff,original,post,BODY,UNITS,effect,&mut *receiving,reply)}else{protocol::preflight_retained_diff_into(owner,diff,original,post,BODY,UNITS,effect,&mut *receiving,reply)}}).result.unwrap_or_else(|_|panic!("original central preflight refused"))
}
fn commit(owner:&mut Owner,diff:RetainedCloneRef<'_,BitmapDiff>,original:RetainedCloneRef<'_,BitmapSnapshot>,post:&mut PostFixture<BitmapSnapshot>,g:RetainedCloneGrant)->RetainedCloneStep{
 drive(g,UNITS,|effect|RetainedWorkPoll::Complete(protocol::commit_retained_diff(owner,diff,original,&mut *post,effect))).result.unwrap_or_else(|_|panic!("original central commit refused"))
}
fn close_once(owner:&mut Owner,g:RetainedCloneGrant)->RetainedCloneStep{
 drive_reply::<RetainedCloneStep,OriginalRetirementFailure>(g,UNITS,|answer|match answer{Ok(step)=>step.progress(),Err(failure)=>failure.retained_progress()},|effect,reply:Reply<'_>,recover|->Result<Done<'_>,ValueError>{if recover{owner.recover_close_into(BODY,effect,reply)}else{owner.close_into(BODY,effect,reply)}}).result.unwrap_or_else(|_|panic!("original central close refused"))
}
fn apply_retained_post(original:&BitmapSnapshot,diff:BitmapDiff)->BitmapSnapshot{
 let base=source(original.clone());let mut post=PostFixture::new(original.clone());let ds=source(diff);let g=grant();
 let mut owner=protocol::begin_retained_diff::<BitmapSnapshot,BitmapDiff>(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}).unwrap().0;
 let mut callback=|_|true;let mut receiving=semio_framework_value::NativeDecodeControl::new(16<<20,&mut callback);let mut ready=false;
 for _ in 0..100000{
  let q=owner.preflight_demand(ds.borrow(),base.borrow(),post.borrow(),BODY,UNITS).unwrap();assert!(protocol::retained_mutation_demand_fits(q,g));
  let step=preflight(&mut owner,ds.borrow(),base.borrow(),post.borrow(),g,&mut receiving);assert!(step.progress().fits(g));assert_eq!(post.borrow().get(),original);
  match step{RetainedDiffPreflightStep::Admitted(_,rows)=>{drain(rows);ready=true;break;},RetainedDiffPreflightStep::Refused(_,_)=>panic!("an admitted semantic delta must apply"),_=>{}}
 }
 assert!(ready);
 for _ in 0..100000{let q=owner.commit_demand(ds.borrow(),base.borrow(),post.mutable_read(),BODY).unwrap();assert!(protocol::retained_mutation_demand_fits(q,g));if matches!(commit(&mut owner,ds.borrow(),base.borrow(),&mut post,g),RetainedCloneStep::Complete(_)){break;}}
 owner.begin_close();while !owner.terminal_is_empty(){let q=owner.close_demand(BODY).unwrap();assert!(protocol::retained_mutation_demand_fits(q,g));assert!(close_once(&mut owner,g).progress().fits(g));}
 post.clone()
}

#[test]
fn every_portable_decision_and_inverse_matches_its_independent_contract(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let mut variants=std::collections::BTreeSet::new();
 for case in fixture["cases"].as_array().unwrap(){
  let plain=case.get("base").unwrap_or(&fixture["base"]);let original:BitmapSnapshot=decode(plain);let mutation:M=decode(&case["mutation"]);variants.insert(case["mutation"].as_object().unwrap().keys().next().unwrap().clone());
  let cold=mutation.diff(&original);let inverse=mutation.inverse(&original).unwrap();let expected_codes:Vec<&str>=case["expectedCodes"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect();
  assert_eq!(cold.messages().iter().map(|m|m.code.0.as_str()).collect::<Vec<_>>(),expected_codes,"{}",case["id"]);
  let m=source(mutation);let base=source(original.clone());let mut p=Preparation::new();let(admitted,d,rows,_)=decision(&mut p,&m,&base,None).unwrap();
  assert_eq!(&d,cold.diff(),"{} diff",case["id"]);assert_eq!(rows,cold.messages(),"{} diagnostics",case["id"]);
  let class=if !admitted{"refused"}else if d.is_empty(){"noop"}else{"applied"};assert_eq!(class,case["classification"].as_str().unwrap(),"{}",case["id"]);assert_eq!(base.borrow().get(),&original,"decision never writes its source");
  if admitted{
   let expected=protocol::apply_mutation(&original,m.borrow().get()).expect("test-only cold post oracle").0;assert_eq!(apply_retained_post(&original,d.clone()),expected,"{} complete retained post",case["id"]);
   let ds=source(d);let mut ready=false;for _ in 0..100000{if matches!(inverse_turn(&mut p,&m,&base,&ds),RetainedCloneStep::Complete(_)){ready=true;break;}}assert!(ready);assert_eq!(p.take_inverse(),inverse,"{} inverse order",case["id"]);
  }
  close(&mut p);
 }
 assert_eq!(variants.len(),13);
}
#[test]
fn every_pin_assignment_decision_frontier_remains_cancelable(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let case=fixture["cases"].as_array().unwrap().iter().find(|c|c["mutation"].get("PinSolution").is_some()&&c["classification"]=="applied").unwrap();let plain=case.get("base").unwrap_or(&fixture["base"]);let original:BitmapSnapshot=decode(plain);let mutation:M=decode(&case["mutation"]);
 let m=source(mutation.clone());let base=source(original.clone());let mut p=Preparation::new();let(_,_,_,turns)=decision(&mut p,&m,&base,None).unwrap();close(&mut p);
 for cancel in 0..turns{let m=source(mutation.clone());let base=source(original.clone());let mut p=Preparation::new();assert!(decision(&mut p,&m,&base,Some(cancel)).is_none());assert_eq!(base.borrow().get(),&original);close(&mut p);assert!(p.terminal_is_empty());}
}
#[test]
fn every_pin_assignment_inverse_frontier_remains_cancelable(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let case=fixture["cases"].as_array().unwrap().iter().find(|c|c["mutation"].get("PinSolution").is_some()&&c["classification"]=="applied").unwrap();let plain=case.get("base").unwrap_or(&fixture["base"]);let original:BitmapSnapshot=decode(plain);let mutation:M=decode(&case["mutation"]);
 let m=source(mutation.clone());let base=source(original.clone());let mut p=Preparation::new();let(_,d,_,_)=decision(&mut p,&m,&base,None).unwrap();let ds=source(d.clone());let mut turns=0;loop{turns+=1;if matches!(inverse_turn(&mut p,&m,&base,&ds),RetainedCloneStep::Complete(_)){break;}assert!(turns<100000);}close(&mut p);
 for cancel in 0..=turns{let m=source(mutation.clone());let base=source(original.clone());let mut p=Preparation::new();let(_,d,_,_)=decision(&mut p,&m,&base,None).unwrap();let ds=source(d);for _ in 0..cancel{inverse_turn(&mut p,&m,&base,&ds);}assert_eq!(base.borrow().get(),&original);close(&mut p);assert!(p.terminal_is_empty());}
}
#[test]
fn an_unfunded_decision_neither_allocates_nor_reads_a_variable_payload(){
 let fixture:serde_json::Value=serde_json::from_str(CASES).unwrap();let case=&fixture["cases"][0];let m=source(decode::<M>(&case["mutation"]));let base=source(decode::<BitmapSnapshot>(&fixture["base"]));let mut p=Preparation::new();
 let g=RetainedCloneGrant{maximum_copy_bytes:0,maximum_capacity_bytes:0,..grant()};let s=turn(g,|effect,reply,recover|p.decide_reply(m.borrow(),base.borrow(),0,UNITS,effect,reply,recover));
 assert!(matches!(s,RetainedCloneStep::Progress(_)));assert_eq!(s.progress(),RetainedCloneProgress::default());assert_eq!(p.phase,0);assert_eq!(p.capacity(),0);close(&mut p);
}
