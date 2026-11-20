//! 🧪️ Genuine Draw issuance, independent serde witnesses and exact original allocator receipts exercise every cancellation cut.
use crate::*;
fn g(g:VectorSceneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:g.maximum_source_items,maximum_copy_bytes:g.maximum_copy_bytes,maximum_capacity_bytes:g.maximum_capacity_bytes,maximum_release_bytes:g.maximum_release_bytes,maximum_depth:g.maximum_depth}}
use semio_framework_value::{OriginalRetirementFailure,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value_clone_source_testing::{FixtureSource,drive::drive};
use semio_s_artifact_draw_drawing_model::{snapshot::DrawingSnapshot,validation::DRAWING_ADMITTER,vector_source::DRAWING_VECTOR_SOURCE};

fn grant()->VectorSceneGrant{VectorSceneGrant{maximum_source_items:4096,maximum_copy_bytes:16*1024*1024,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:256}}
fn kind(node:&VectorNodeRef<'_>)->&'static str{match &node.content{VectorContentRef::Group{..}=>"group",VectorContentRef::Path(_)=>"path",VectorContentRef::Shape(_)=>"shape",VectorContentRef::Text{..}=>"text",VectorContentRef::Image{..}=>"image",VectorContentRef::Boolean{..}=>"boolean",VectorContentRef::Trace{..}=>"trace"}}
fn progress(cursor:&DrawingPlanCursor)->RetainedCloneProgress{let p=cursor.last_receipt();RetainedCloneProgress{copied_items:p.source_items,copied_bytes:p.copied_bytes,retained_capacity_bytes:p.retained_capacity_bytes,released_bytes:p.released_bytes}}
fn advance<'a>(cursor:&mut DrawingPlanCursor,read:&ChildAdmissionRead<'a>,grant:VectorSceneGrant)->DrawingPlanTurn<'a>{
 let mut output=None;
 drive(g(grant),256,|effect|{
  let start=effect.performed_work();let result=cursor.advance(read,effect).map(|step|{output=Some(step);RetainedCloneStep::Progress(progress(cursor))}).map_err(OriginalRetirementFailure::from);
  RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()-start})
 }).result.unwrap_or_else(|error|panic!("Drawing plan refused: {:?}",error.primary()));
 output.unwrap()
}
fn close_once(cursor:&mut DrawingPlanCursor,grant:VectorSceneGrant)->bool{
 let mut advanced=false;
 drive(g(grant),256,|effect|{
  let start=effect.performed_work();let result=cursor.close_step(effect).map(|step|{advanced=step;RetainedCloneStep::Progress(progress(cursor))}).map_err(OriginalRetirementFailure::from);
  RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:effect.performed_work()-start})
 }).result.unwrap_or_else(|error|panic!("Drawing plan close refused: {:?}",error.primary()));
 advanced
}
fn close(cursor:&mut DrawingPlanCursor){
 cursor.begin_close();
 for _ in 0..10000{
  if cursor.terminal_is_empty(){return}
  let d=cursor.close_demand(grant().maximum_copy_bytes).unwrap();
  if d.copy_bytes>0{assert!(!close_once(cursor,VectorSceneGrant{maximum_copy_bytes:d.copy_bytes-1,..grant()}));assert_eq!(cursor.last_receipt(),Default::default())}
  close_once(cursor,grant());
 }
 panic!("Drawing cancellation did not finish")
}
#[test]
fn original_graph_plans_all_kinds_without_source_copy_and_closes_every_cut(){let law:serde_json::Value=serde_json::from_str(include_str!("🧵️graph.json")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/✅️validation/🧪️.json")).unwrap();for cut in [Some(0),Some(1),Some(2),Some(3),Some(4),Some(5),Some(9),Some(17),Some(31),Some(63),None]{let document:DrawingSnapshot=serde_json::from_value(fixture["base"].clone()).unwrap();let mut source=FixtureSource::from_owner(document);let read=ChildAdmissionRead{source:semio_framework_io_ports::erase_child_admission_source(source.borrow(),0x4348494c),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE};let mut cursor=DrawingPlanCursor::new(DrawingSourceAddress::Document,DrawingPlacement::Canvas,LayoutBounds{x:2.0,y:3.0,width:200.0,height:100.0,rotation:0.0});let mut kinds=Vec::new();let mut samples=0;let mut complete=false;for turn in 0..10000{if cut.is_some_and(|cut|turn>=cut){break}let demand=cursor.next_demand(&read,grant().maximum_copy_bytes).unwrap();for denied in [VectorSceneGrant{maximum_source_items:0,..grant()},VectorSceneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant()},VectorSceneGrant{maximum_depth:0,..grant()}]{let step=advance(&mut cursor,&read,denied);assert!(matches!(step,DrawingPlanTurn::Progress));assert_eq!(cursor.last_receipt(),Default::default());}let step=advance(&mut cursor,&read,grant());match step{DrawingPlanTurn::MeasureText(text)=>{assert_eq!(text.font.family,"shareTechMono");assert_eq!(text.font.size,12.0);cursor.include_text_extent(text.index,Some(DrawingBounds{left:0.0,top:-12.0,right:15.0,bottom:1.0})).unwrap()},DrawingPlanTurn::Semantic(DrawingSemanticEvent::Node{node,..})=>kinds.push(kind(&node)),DrawingPlanTurn::Semantic(DrawingSemanticEvent::Sample{value,..})=>{assert_eq!(value,[0,64,128,255]);samples+=1},DrawingPlanTurn::Done=>{complete=true;break},_=>{}}}if cut.is_none(){assert!(complete);let expected:Vec<&str>=law["cases"][0]["kinds"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect();assert_eq!(kinds,expected);assert_eq!(samples,1)}close(&mut cursor);drop(cursor);while !source.terminal_is_empty(){source.close_once().unwrap();}}}

#[path="🦀️.rs"]mod authored;
