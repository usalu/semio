use semio_framework_value_clone_source_testing::FixtureSource;
use super::run_original_test_turn;
use semio_os_kernel::{MutationMessage,MutationMessageLedgerRetirement};
use crate::LayoutMutation;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_s_artifact_layout_layout_mutations_decisions::batch_message::*;
use semio_framework_diagnostic::Severity;

use semio_framework_value::RetireOwned;
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn decode<T:semio_framework_value::FromValue>(v:&serde_json::Value)->T{semio_framework_pack_json::from_json_str(&v.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn source<T:RetireOwned+Sync+'static>(v:T)->FixtureSource<T>{FixtureSource::from_owner(v)}
fn close_source<T:RetireOwned+Sync+'static>(s:&mut FixtureSource<T>){while !s.terminal_is_empty(){s.close_once().unwrap();}}
fn literal(value:&str)->&'static str{match value{"targets must not repeat \""=>"targets must not repeat \"","\""=>"\""," of "=>" of "," target(s) skipped (not on this page): "=>" target(s) skipped (not on this page): "," target(s) skipped (locked): "=>" target(s) skipped (locked): ",", "=>", ","The transform moves no frame."=>"The transform moves no frame.","None of the "=>"None of the "," target(s) is an unlocked frame of page \""=>" target(s) is an unlocked frame of page \"","\"."=>"\".","A drag offset must be finite."=>"A drag offset must be finite.",_=>panic!("unregistered finite diagnostic literal")}}
struct Source<'a>{row:&'a serde_json::Value,m:RetainedCloneRef<'a,LayoutMutation>,b:RetainedCloneRef<'a,LayoutSnapshot>}
impl Source<'_>{fn original(&self,index:usize)->RetainedCloneRef<'_,String>{self.m.project(74,|m|match m{LayoutMutation::DragFrames(p)=>&p.targets,_=>unreachable!()}).project(index,move|rows|&rows[index])}}
impl DynamicMessageSource for Source<'_>{type Mutation=LayoutMutation;
 fn mutation(&self)->RetainedCloneRef<'_,LayoutMutation>{self.m}fn baseline(&self)->RetainedCloneRef<'_,LayoutSnapshot>{self.b}fn identity(&self)->usize{1}
 fn level(&self)->Severity{match self.row["level"].as_str().unwrap(){"warning"=>Severity::Warning,"error"=>Severity::Error,"fatal"=>Severity::Fatal,_=>unreachable!()}}
 fn code(&self)->&'static str{match self.row["code"].as_str().unwrap(){"mutation.partial"=>"mutation.partial","mutation.no-op"=>"mutation.no-op","mutation.invariant"=>"mutation.invariant","mutation.target-missing"=>"mutation.target-missing",_=>unreachable!()}}
 fn piece_count(&self)->usize{self.row["pieces"].as_array().unwrap().len()}
 fn piece(&self,index:usize)->MessagePiece<'_>{let row=&self.row["pieces"][index];if let Some(index)=row["escapedOriginal"].as_u64(){MessagePiece::EscapedOriginal(self.original(index as usize))}else if let Some(index)=row["original"].as_u64(){MessagePiece::Original(self.original(index as usize))}else if let Some(value)=row["number"].as_u64(){MessagePiece::Unsigned(value as usize)}else{MessagePiece::Literal(literal(row["literal"].as_str().unwrap()))}}
 fn target_count(&self)->usize{self.row["targets"].as_array().unwrap().len()}fn target(&self,index:usize)->RetainedCloneRef<'_,String>{self.original(self.row["targets"][index].as_u64().unwrap()as usize)}
}

fn close(owner:&mut DynamicMessagePreparation){owner.begin_close();for _ in 0..100000{if owner.terminal_is_empty(){return}let d=owner.close_demand(256).unwrap();assert_eq!(run_original_test_turn(Default::default(),RetainedCloneStep::progress,|effect,recover|owner.close_step(None,256,effect,recover)).progress(),Default::default());run_original_test_turn(grant(d),RetainedCloneStep::progress,|effect,recover|owner.close_step(None,256,effect,recover));}panic!("dynamic original diagnostic close stalled")}
fn close_rows(rows:Vec<MutationMessage>){let mut ledger=MutationMessageLedgerRetirement::new(String::new(),rows);while !ledger.terminal_is_empty(){let d=RetirementDemand{copy_bytes:ledger.next_copy_byte_demand(),capacity_bytes:ledger.next_capacity_byte_demand(256).unwrap(),release_bytes:ledger.next_release_byte_demand().unwrap(),depth:ledger.next_depth_demand().unwrap()};ledger.close_step(grant(d)).unwrap();}}
#[test]
fn canonical_dynamic_rows_copy_only_original_fields_with_exact_native_receipts(){
 let fixture:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🖼️frame/👥️batch/⚠️message/🧪️tests/🔣️.json")).unwrap();let common:serde_json::Value=serde_json::from_str(&crate::test_fixture_text("../../🧳️retained/🎚️simple/🧪️tests/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let event=LayoutMutation::DragFrames(crate::drag_frames::DragFrames{page_id:"fixture".into(),targets:decode(&row["original"]),dx:0.0,dy:0.0});
  for cut in std::iter::once(usize::MAX).chain(fixture["cuts"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize)){
   let mut mutation=source(event.clone());let mut base=source(decode::<LayoutSnapshot>(&common["base"]));let mut owner=DynamicMessagePreparation::default();
   for turn in 0..cut{if owner.ready(){break}assert!(turn<100000);let input=Source{row,m:mutation.borrow(),b:base.borrow()};let d=owner.demand(&input,256,1<<17).unwrap();let g=grant(d);assert_eq!(run_original_test_turn(Default::default(),RetainedCloneStep::progress,|effect,recover|owner.advance(&input,256,1<<17,effect,recover)).progress(),Default::default());for short in[RetainedCloneGrant{maximum_copy_bytes:d.copy_bytes-1,..g},RetainedCloneGrant{maximum_capacity_bytes:d.capacity_bytes.saturating_sub(1),..g},RetainedCloneGrant{maximum_depth:d.depth-1,..g}]{if short==g{continue}assert_eq!(run_original_test_turn(short,RetainedCloneStep::progress,|effect,recover|owner.advance(&input,256,1<<17,effect,recover)).progress(),Default::default());}run_original_test_turn(g,RetainedCloneStep::progress,|effect,recover|owner.advance(&input,256,1<<17,effect,recover));}
   if cut==usize::MAX{assert!(owner.ready());let d=owner.take_demand();assert!(owner.take(Default::default()).unwrap().is_none());let(rows,p)=owner.take(grant(d)).unwrap().unwrap();assert!(p.fits(grant(d)));assert_eq!(rows.len(),1);assert_eq!(rows[0].message,row["expected"]["message"].as_str().unwrap());assert_eq!(rows[0].target,decode::<Vec<String>>(&row["expected"]["target"]));assert_eq!(rows[0].level,Source{row,m:mutation.borrow(),b:base.borrow()}.level());close_rows(rows);}
   assert_eq!(mutation.borrow().get(),&event);close(&mut owner);close_source(&mut mutation);close_source(&mut base);
  }
 }
}
