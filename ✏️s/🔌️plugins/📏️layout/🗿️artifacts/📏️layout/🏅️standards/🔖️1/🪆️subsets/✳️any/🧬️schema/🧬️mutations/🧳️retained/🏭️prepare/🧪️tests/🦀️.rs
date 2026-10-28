use super::*;
use semio_framework_value::{retirement::{RetireOwned,OwnedValueRetirementFactory},retained_clone::RetainedCloneSource};
use protocol::{PagedOneItemEdit,PagedOneItemEditStep};
use semio_framework_value::snapshot_projection::post::testing::PostFixture;
fn issued_progress(step:&PagedOneItemEditStep<LayoutMutation>)->RetainedCloneProgress{match step{PagedOneItemEditStep::Progress(p)|PagedOneItemEditStep::Complete(p,_)|PagedOneItemEditStep::CompleteRows(p,_)|PagedOneItemEditStep::CompleteOutcome(p,..)|PagedOneItemEditStep::RefusedOutcome(p,_)=>*p}}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn source<T:RetireOwned+Sync+'static>(value:T)->RetainedCloneSource<T>{match RetainedCloneSource::admit_owned(value,(),RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:4096}){Ok((owner,_))=>owner,Err((error,_,_))=>panic!("{error}")}}
fn close_source<T:RetireOwned+Sync+'static>(owner:&mut RetainedCloneSource<T>){while !owner.terminal_is_empty(){let d=RetirementDemand{copy_bytes:owner.next_close_copy_byte_demand().unwrap(),capacity_bytes:owner.next_close_capacity_byte_demand(4096).unwrap(),release_bytes:owner.next_close_release_byte_demand().unwrap(),depth:owner.next_close_depth_demand().unwrap()};owner.close_step(grant(d)).unwrap();}}
fn close_messages(rows:Vec<MutationMessage>){let mut ledger=MutationMessageLedgerRetirement::new(String::new(),rows);while !ledger.terminal_is_empty(){let d=semio_framework_value::ErasedSnapshotRetirement::next_demand(&ledger,4096).unwrap();ledger.close_step(grant(d)).unwrap();}}
fn close_inverse(rows:Vec<LayoutMutation>,factory:&OwnedValueRetirementFactory<LayoutMutation>){let d=semio_framework_value_factory::vector::FactoryVectorRetirement::<LayoutMutation>::constructor_demand();let(mut owner,_)=semio_framework_value_factory::vector::FactoryVectorRetirement::admit(rows,grant(d)).unwrap_or_else(|(error,_)|panic!("{error}"));while !owner.terminal_is_empty(){let d=owner.demands(factory,4096).unwrap();owner.close_step(factory,grant(d)).unwrap();}}
fn close(owner:&mut protocol::MutationApplyEdit<LayoutSnapshot,LayoutMutation>,factory:&OwnedValueRetirementFactory<LayoutMutation>){owner.begin_close();for _ in 0..100000{if owner.terminal_is_empty(){return;}let d=owner.close_demands(factory,4096).unwrap();assert_eq!(owner.close_step(factory,RetainedCloneGrant::default()).unwrap().progress(),Default::default());owner.close_step(factory,grant(d)).unwrap();}panic!("Layout genuine issuer close stalled");}
fn decode<T:semio_framework_value::FromValue>(v:&serde_json::Value)->T{semio_framework_pack_json::from_json_str(&v.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn observe_issued(g:RetainedCloneGrant,f:impl FnOnce()->PagedOneItemEditStep<LayoutMutation>)->PagedOneItemEditStep<LayoutMutation>{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(f);let p=issued_progress(&step);assert!(!heap.overflowed);assert_eq!(heap.requested_bytes,p.retained_capacity_bytes);assert_eq!(heap.released_bytes,p.released_bytes);assert!(p.fits(g));step}
fn advance_once(owner:&mut protocol::MutationApplyEdit<LayoutSnapshot,LayoutMutation>,base:semio_framework_value::retained_clone::RetainedCloneRef<'_,LayoutSnapshot>,mutation:semio_framework_value::retained_clone::RetainedCloneRef<'_,LayoutMutation>,post:&mut PostFixture<LayoutSnapshot>,body:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>)->PagedOneItemEditStep<LayoutMutation>{
 let mode=owner.post_mode();
 if let Some(d)=post.turn_demand(mode,body).unwrap(){
  assert_eq!(post.prepare_turn(mode,body,Default::default()).unwrap(),Some(Default::default()));
  let p=post.prepare_turn(mode,body,grant(d)).unwrap().expect("quoted original post custody");
  assert!(p.fits(grant(d)));
  return PagedOneItemEditStep::Progress(p);
 }
 let d=owner.advance_demands(base,mutation,post.read_mode(mode).unwrap(),body).unwrap();
 let short=observe_issued(Default::default(),||owner.advance(base,mutation,post.access(mode).unwrap(),body,Default::default(),control).unwrap());
 assert_eq!(issued_progress(&short),Default::default());
 let step=observe_issued(grant(d),||owner.advance(base,mutation,post.access(mode).unwrap(),body,grant(d),control).unwrap());
 assert!(issued_progress(&step).fits(grant(d)));
 step
}
fn execute(event:&LayoutMutation,original:LayoutSnapshot,initial_post:LayoutSnapshot,body:usize)->(LayoutSnapshot,Vec<LayoutMutation>,Vec<MutationMessage>,bool){
 let mut post=PostFixture::new(initial_post);
 let mut base=source(original.clone());
 let mut mutation=source(event.clone());
 let factory=OwnedValueRetirementFactory::<LayoutMutation>::default();
 let mut owner=protocol::MutationApplyEdit::<LayoutSnapshot,LayoutMutation>::default();
 let mut callback=|_|true;
 let mut control=semio_framework_value::NativeDecodeControl::new(1<<30,&mut callback);
 let mut answer=None;
 for _ in 0..1000000{
  match advance_once(&mut owner,base.borrow(),mutation.borrow(),&mut post,body,&mut control){
   PagedOneItemEditStep::Progress(_)=>{},
   PagedOneItemEditStep::CompleteOutcome(_,inverse,messages)=>{answer=Some((inverse,messages,true));break},
   PagedOneItemEditStep::RefusedOutcome(_,messages)=>{answer=Some((Vec::new(),messages,false));break},
   _=>panic!("unexpected original issuer result"),
  }
 }
 let(inverse,messages,accepted)=answer.expect("Layout original retained issuer progress");
 assert_eq!(base.borrow().get(),&original);
 assert_eq!(mutation.borrow().get(),event);
 close(&mut owner,&factory);
 close_source(&mut mutation);
 close_source(&mut base);
 (post.into_inner(),inverse,messages,accepted)
}
#[test]
fn genuine_generic_issuer_decides_preflights_inverts_commits_and_executes_issued_inverse(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/⚙️semantic/🔣️.json")).unwrap();
 let extra:serde_json::Value=serde_json::from_str(include_str!("../../🎚️simple/🧪️tests/🔣️.json")).unwrap();
 let mut checked=0;
 for row in corpus["cases"].as_array().unwrap().iter().chain(extra["commands"].as_array().unwrap()){
  let event:LayoutMutation=decode(&row["mutation"]);
  if super::super::simple::SimpleKind::of(&event).is_none()&&!super::super::frame::batch_prepare::selected(&event){continue}
  for body in[1024,4096,65536]{
   let original:LayoutSnapshot=decode(row.get("before").unwrap_or(&extra["base"]));
   let expected:LayoutSnapshot=decode(row.get("afterLivePost").unwrap_or(&row["after"]));
   let initial_post:LayoutSnapshot=decode(row.get("livePost").unwrap_or(row.get("before").unwrap_or(&extra["base"])));
   let(post,mut inverse,messages,accepted)=execute(&event,original.clone(),initial_post,body);
   assert_eq!(post,expected,"{}",row["name"]);
   assert!(!row["accepted"].as_bool().unwrap()||accepted,"{}: accepted original retained command",row["name"]);
   assert_eq!(inverse.is_empty(),!row["accepted"].as_bool().unwrap());
   if row.get("inverseKinds").is_some(){
    if row["mutation"].get("CreateLayer").is_some(){assert!(matches!(&inverse[0],LayoutMutation::UpdateLayer(_)));assert!(matches!(&inverse[1],LayoutMutation::CreateLayer(_)));}
    else{assert!(matches!(&inverse[0],LayoutMutation::UpdateCharacterStyle(_)));assert!(matches!(&inverse[1],LayoutMutation::CreateCharacterStyle(_)));}
   }
   if let Some(expected)=row.get("messages"){assert_eq!(messages.len(),expected.as_array().unwrap().len(),"{}: exact retained diagnostic count",row["name"]);for(actual,expected)in messages.iter().zip(expected.as_array().unwrap()){let level=match actual.level{semio_framework_diagnostic::Severity::Warning=>"warning",semio_framework_diagnostic::Severity::Error=>"error",semio_framework_diagnostic::Severity::Fatal=>"fatal",_=>panic!("unexpected retained severity")};assert_eq!(level,expected["level"].as_str().unwrap());assert_eq!(actual.code.0,expected["code"].as_str().unwrap());assert_eq!(actual.message,expected["message"].as_str().unwrap());assert_eq!(actual.target,decode::<Vec<String>>(&expected["target"]));assert_eq!(actual.op_index,None)}}
   let mut restored=post;
   let factory=OwnedValueRetirementFactory::<LayoutMutation>::default();
   for back in inverse.iter().rev(){
    let(next,issued_back,back_messages,back_accepted)=execute(back,restored.clone(),restored,body);
    assert!(back_accepted,"{}: actual retained inverse refused",row["name"]);
    close_messages(back_messages);
    close_inverse(issued_back,&factory);
    restored=next;
   }
   let expected_undo=row.get("afterUndo").map(decode::<LayoutSnapshot>).unwrap_or(original);
   assert_eq!(restored,expected_undo,"{}: actual retained inverse restores original affected fields",row["name"]);
   close_messages(messages);
   close_inverse(std::mem::take(&mut inverse),&factory);
   let mut output=source(restored);close_source(&mut output);
  }
  checked+=1;
 }
 assert_eq!(checked,241);
}
#[test]
fn ordinary_layout_cancellation_closes_real_semantic_diff_and_inverse_frontiers(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/⚙️semantic/🔣️.json")).unwrap();
 let extra:serde_json::Value=serde_json::from_str(include_str!("../../🎚️simple/🧪️tests/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap().iter().take(1).chain(extra["commands"].as_array().unwrap().iter().filter(|row|(["SetPageGuides","SetStoryRuns","DeleteLink","DeleteStory","DeleteParagraphStyle","DeleteCharacterStyle","DeleteSpread","CreateLayer","CreateFrame","DeleteFrame","ChangeDataFields","SetPageParent","UpdateTextFrame","DeleteParentPage","SetPageBackgroundDrawing","CreateParentPage","SetPageSpread","DragFrames","RotateFrames","ScaleFrames","SetPageOverrides","ReplaceDocument"].iter().any(|kind|row["mutation"].get(*kind).is_some()))&&row["accepted"]==true)){
  for cut in[0,1,5,50,100,250,1000]{
   let original:LayoutSnapshot=decode(row.get("before").unwrap_or(&extra["base"]));
   let event:LayoutMutation=decode(&row["mutation"]);
   let mut post=PostFixture::new(original.clone());
   let mut base=source(original.clone());let mut mutation=source(event.clone());
   let factory=OwnedValueRetirementFactory::<LayoutMutation>::default();
   let mut owner=protocol::MutationApplyEdit::<LayoutSnapshot,LayoutMutation>::default();
   let mut callback=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(1<<30,&mut callback);
   for _ in 0..cut{
    match advance_once(&mut owner,base.borrow(),mutation.borrow(),&mut post,4096,&mut control){
     PagedOneItemEditStep::Progress(_)=>{},
     PagedOneItemEditStep::CompleteOutcome(_,inverse,rows)=>{close_messages(rows);close_inverse(inverse,&factory);break},
     PagedOneItemEditStep::RefusedOutcome(_,rows)=>{close_messages(rows);break},
     _=>panic!("unexpected retained cancellation frontier"),
    }
   }
   assert_eq!(base.borrow().get(),&original);assert_eq!(mutation.borrow().get(),&event);
   close(&mut owner,&factory);close_source(&mut mutation);close_source(&mut base);
   let mut output=source(post.into_inner());close_source(&mut output);
  }
 }
}

#[test]
fn shared_reference_refusals_keep_error_severity(){let corpus:serde_json::Value=serde_json::from_str(include_str!("../../🎚️simple/🧪️tests/🔣️.json")).unwrap();let mut checked=0;for row in corpus["commands"].as_array().unwrap(){if row["errorLevel"]=="error"{assert!(matches!(error_level(row["error"].as_str().unwrap()),semio_framework_diagnostic::Severity::Error));checked+=1}}assert_eq!(checked,5);assert!(matches!(error_level("mutation.invariant"),semio_framework_diagnostic::Severity::Fatal));}
