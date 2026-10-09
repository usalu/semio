#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🎯️dag-input/🦀️.rs"]
pub mod model;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🦀️.rs"]
pub mod codec;
pub mod infinite {pub mod board {pub mod schema {pub use crate::model as dag_input;}}}
#[cfg(test)]
mod tests {
use super::{model::*,codec::*};
#[test]
fn original_nodes_and_edges_borrow_native_json_under_independent_admission(){
 use super::model::output::{DagOutputCandidates,DagChannelView,DagRefusalView};use super::codec::output::{DagOutputPreparation,DagOutputKind,DagJsonSource};use semio_framework_value::retained_clone::RetainedCloneGrant;
 struct Source(Vec<String>);impl DagOutputCandidates for Source{fn node_candidate_count(&self)->usize{self.0.len()+1}fn node_candidate_id(&self,index:usize)->Option<&str>{self.0.get(index).map(String::as_str)}fn edge_candidate_count(&self)->usize{self.0.len()+1}fn edge_candidate_id(&self,index:usize)->Option<&str>{self.0.get(index).map(String::as_str)}fn channel_candidate_count(&self)->usize{0}fn channel_candidate(&self,_:usize)->Option<DagChannelView<'_>>{None}fn handle_candidate_identity(&self,_:usize)->Option<&str>{None}fn hovered_channel_view(&self)->Option<DagChannelView<'_>>{None}fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>{None}}
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🎯️selected-json.json")).unwrap();let admission:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧫️fixtures/🔣️.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(admission["retainedGrant"].clone()).unwrap();let mut turns=0;
 for row in fixture["cases"].as_array().unwrap(){for kind in [DagOutputKind::Nodes,DagOutputKind::Edges]{let source=Source(serde_json::from_value(row["ids"].clone()).unwrap());let original=source.0.iter().map(|text|text.as_ptr()).collect::<Vec<_>>();let mut prepared=DagOutputPreparation::new(kind);let mut observe=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new_retained(&mut observe);while !prepared.step(&source,1,&mut control,grant).unwrap(){turns+=1;}let mut writer=semio_framework_pack_json::JsonBorrowedWriteCursor::new();let output=loop{turns+=1;let view=prepared.view(&source);if let Some(output)=writer.step(&DagJsonSource{source:&view,kind},1,&mut control,grant).unwrap(){break output}};assert_eq!(output,serde_json::to_string(&source.0).unwrap());assert_eq!(original,source.0.iter().map(|text|text.as_ptr()).collect::<Vec<_>>());
  fn retire<T:semio_framework_value::retirement::RetireOwned>(value:T){use semio_framework_value::retirement::{admit_owned_retirement,owned_retirement_birth_bytes};let(mut owner,_)=admit_owned_retirement(value,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:owned_retirement_birth_bytes::<T>(),maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("native array controlled custody"));while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()}).unwrap();}drop(owner);}retire(prepared);retire(writer);
 }}println!("[DEBUG] original node and edge JSON arrays cases=6 retainedOneItemTurns={turns} originalPointers=true independentSerde=true invalidRanksSkipped=true exactControlledClose=true");
}
#[test]
fn original_dag_output_census_and_writer_borrow_each_rank_under_one_grant(){
 use super::model::output::{DagOutputCandidates,DagChannelView,DagRefusalView};use super::codec::output::{DagOutputPreparation,DagOutputKind,DagJsonSource};use semio_framework_value::retirement::{RetireOwned,admit_owned_retirement,owned_retirement_birth_bytes};use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneStep};
 struct Source(Vec<String>);impl DagOutputCandidates for Source{
  fn node_candidate_count(&self)->usize{self.0.len()+1}fn node_candidate_id(&self,index:usize)->Option<&str>{self.0.get(index).map(String::as_str)}fn edge_candidate_count(&self)->usize{1}fn edge_candidate_id(&self,_:usize)->Option<&str>{None}fn channel_candidate_count(&self)->usize{1}fn channel_candidate(&self,_:usize)->Option<DagChannelView<'_>>{None}fn handle_candidate_identity(&self,_:usize)->Option<&str>{None}fn hovered_channel_view(&self)->Option<DagChannelView<'_>>{None}fn wire_refusal_view(&self)->Option<DagRefusalView<'_>>{None}
 }
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🎯️selected-json.json")).unwrap();let grant:RetainedCloneGrant=serde_json::from_value(serde_json::from_str::<serde_json::Value>(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧫️fixtures/🔣️.json")).unwrap()["retainedGrant"].clone()).unwrap();let mut grants=0;
 for row in fixture["cases"].as_array().unwrap(){let source=Source(serde_json::from_value(row["ids"].clone()).unwrap());let pointers:Vec<_>=source.0.iter().map(|text|text.as_ptr()).collect();let mut prepared=DagOutputPreparation::new(DagOutputKind::Selection);let mut observe=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new_retained(&mut observe);assert!(!prepared.step(&source,0,&mut control,grant).unwrap());assert_eq!(control.owned_bytes(),0);while !prepared.step(&source,1,&mut control,grant).unwrap(){grants+=1;}let before=control.owned_bytes();let mut writer=semio_framework_pack_json::JsonBorrowedWriteCursor::new();let output=loop{grants+=1;let view=prepared.view(&source);if let Some(output)=writer.step(&DagJsonSource{source:&view,kind:DagOutputKind::Selection},1,&mut control,grant).unwrap(){break output}};let expected=serde_json::to_string(&DagSelectionDomains{nodes:source.0.clone(),edges:vec![],handles:vec![]}).unwrap();assert_eq!(output,expected);assert!(control.owned_bytes()>=before);assert_eq!(pointers,source.0.iter().map(|text|text.as_ptr()).collect::<Vec<_>>());
  fn retire<T:RetireOwned>(value:T){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:owned_retirement_birth_bytes::<T>(),maximum_release_bytes:0,maximum_depth:1};let(mut owner,_)=admit_owned_retirement(value,grant).unwrap_or_else(|_|panic!("actual borrowed output retirement admission"));while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let capacity=owner.next_capacity_byte_demand(copy).unwrap();let release=owner.next_release_byte_demand().unwrap();let depth=owner.next_depth_demand().unwrap();match owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth}).unwrap(){RetainedCloneStep::Progress(_)|RetainedCloneStep::Complete(_)=>{}}}drop(owner);}
  retire(prepared);retire(writer);
 }
 println!("[DEBUG] original DAG bounded output census cases=3 singleUnitGrants={grants} invalidRanksSkipped=true independentSerdeBytes=true sourcePointersPreserved=true cumulativeOwnership=true exactControlledRetirement=true");
}
#[test]
fn retained_numeric_host_preparation_keeps_original_order_and_each_granted_transition(){
 use semio_framework_value::numeric_scratch::{NumericIndex,NumericInsertCursor};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🧮️scratch/🧪️tests/📜️fixtures/🔣️.json")).unwrap();
 let mut index=NumericIndex::<i64,i64>::new();let mut oracle=std::collections::BTreeMap::new();let mut transitions=0;let mut allocations=0;
 for row in fixture["operations"].as_array().unwrap().iter().filter(|row|row["kind"]=="set"){
  let key=row["key"].as_i64().unwrap();let value=row["value"].as_i64().unwrap();let mut cursor=NumericInsertCursor::new(key,value);let mut observe=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(1048576,&mut observe);
  while !cursor.step(&mut index,1,&mut control).unwrap(){transitions+=1;}allocations+=control.owned_bytes();assert_eq!(cursor.take_previous(),oracle.insert(key,value));assert_eq!(index.len(),oracle.len());
  for(rank,(key,value))in oracle.iter().enumerate(){assert_eq!(index.entry_at_rank(rank),Some((key,value)));}
 }
 let before=index.len();let mut cursor=NumericInsertCursor::new(1000,1000);let mut observe=|_|false;assert!(cursor.step(&mut index,1,&mut semio_framework_value::NativeDecodeControl::new(1048576,&mut observe)).is_err());assert_eq!(index.len(),before);
 let mut credits=0;let mut released=0;let mut close_turns=0;for _ in 0..fixture["maximumCloseTurns"].as_u64().unwrap(){if index.terminal_is_empty(){break}let demand=index.next_close_release_byte_demand().unwrap();let admitted=(fixture["closeReleaseBytes"].as_u64().unwrap()as usize).min(demand.saturating_sub(credits));credits+=admitted;if credits<demand{close_turns+=1;continue}let progress=index.close_copy_step(1,credits).unwrap();assert!(progress.released_allocation_bytes<=credits);if progress.progressed{credits-=progress.released_allocation_bytes;}released+=progress.released_allocation_bytes;close_turns+=1;}assert!(index.terminal_is_empty());assert_eq!(credits,0);assert_eq!(released,allocations);
 println!("[DEBUG] numeric backing retirement oneByteGrant=true closeTurns={close_turns} exactAllocations={allocations} exactReleases={released} physicalTerminal=true");
 println!("[DEBUG] retained numeric actual host storage transitions={transitions} rankOrder=true independentBTreeMap=true cancellationBeforeMutation=true");
}
use semio_framework_value::{DslValue,NativeDecodeControl,NativeEncodeControl,ToValue,FromValue,ValueError};
use serde_json::Value;
type OracleStatuses=std::collections::BTreeMap<String,DagNodeEvaluationStatus>;
fn input(kind:&str,source:&str,control:&mut NativeDecodeControl<'_>)->Result<DslValue,ValueError>{match kind {
 "selection"=>decode_dag_selection_json(source,control).map(|value|value.to_value()),
 "channels"=>decode_dag_channels_json(source,control).map(|value|value.to_value()),
 "statuses"=>decode_dag_node_statuses_json(source,control).map(|value|{let output=value.to_value();DagNodeStatuses::retire_decoded(value);output}),
 "hover"=>decode_dag_hover_json(source,control).map(|value|value.to_value()),
 _=>panic!("unknown neutral family"),
}}
fn oracle(kind:&str,source:&str)->Value {match kind {
 "selection"=>serde_json::to_value(serde_json::from_str::<DagSelectionDomains>(source).unwrap()).unwrap(),
 "channels"=>serde_json::to_value(serde_json::from_str::<Vec<DagChannelRef>>(source).unwrap()).unwrap(),
 "statuses"=>serde_json::to_value(serde_json::from_str::<OracleStatuses>(source).unwrap()).unwrap(),
 "hover"=>serde_json::to_value(serde_json::from_str::<DagHoverFacts>(source).unwrap()).unwrap(),
 _=>panic!("unknown neutral family"),
}}
#[test]
fn actual_dag_input_admission_and_publication_agree_with_neutral_and_serde() {
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧫️fixtures/🔣️.json")).unwrap();let mut observations=0;
 for case in fixture["accepted"].as_array().unwrap(){let source=serde_json::to_string(&case["value"]).unwrap();let original=source.as_ptr();let kind=case["kind"].as_str().unwrap();
  let mut observe=|_|{observations+=1;true};let mut control=NativeDecodeControl::new(1048576,&mut observe);let actual=input(kind,&source,&mut control).unwrap();assert_eq!(Value::from(&actual),oracle(kind,&source));assert_eq!(source.as_ptr(),original);
  let mut publish=|_|true;let mut control=NativeEncodeControl::new(1048576,&mut publish);let encoded=match kind {"selection"=>encode_dag_selection_json(&serde_json::from_str(&source).unwrap(),&mut control).unwrap(),"channels"=>encode_dag_channels_json(&serde_json::from_str(&source).unwrap(),&mut control).unwrap(),"hover"=>encode_dag_hover_json(&serde_json::from_str(&source).unwrap(),&mut control).unwrap(),_=>continue};assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(),case["value"]);
 }
 for case in fixture["refused"].as_array().unwrap(){let source=serde_json::to_string(&case["value"]).unwrap();let mut observe=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut observe);assert!(input(case["kind"].as_str().unwrap(),&source,&mut control).is_err());}
 #[derive(serde::Deserialize)]struct StatusSample{n:DagNodeEvaluationStatus}
 for case in fixture["rawRefused"].as_array().unwrap(){let source=case["text"].as_str().unwrap();let kind=case["kind"].as_str().unwrap();let mut observe=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut observe);assert!(input(kind,source,&mut control).is_err());match kind{"selection"=>assert!(serde_json::from_str::<DagSelectionDomains>(source).is_err()),"hover"=>assert!(serde_json::from_str::<DagHoverFacts>(source).is_err()),_=>assert!(serde_json::from_str::<StatusSample>(source).is_err())}}
 println!("[DEBUG] Actual DAG IO: accepted={} refused={} rawDuplicateRefusals={} independentSerde=true progressObservations={observations} sourcePointersPreserved=true",fixture["accepted"].as_array().unwrap().len(),fixture["refused"].as_array().unwrap().len(),fixture["rawRefused"].as_array().unwrap().len());
}
#[test]
fn actual_dag_input_cancellation_and_zero_ceiling_preserve_borrowed_input() {
 let source=r#"{"nodes":["node雪"],"edges":[],"handles":[]}"#.to_owned();let pointer=source.as_ptr();let mut baseline=0;let mut observe=|_|{baseline+=1;true};assert!(decode_dag_selection_json(&source,&mut NativeDecodeControl::new(1048576,&mut observe)).is_ok());
 for stop in 0..baseline {let mut seen=0;let mut observe=|_|{let accepted=seen!=stop;seen+=1;accepted};let error=decode_dag_selection_json(&source,&mut NativeDecodeControl::new(1048576,&mut observe)).unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(source.as_ptr(),pointer);}
 let mut observe=|_|true;let error=decode_dag_selection_json(&source,&mut NativeDecodeControl::new(0,&mut observe)).unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(source.as_ptr(),pointer);
 let value=DagSelectionDomains{nodes:vec!["n".into()],edges:vec![],handles:vec![]};let mut observe=|_|true;assert!(encode_dag_selection_json(&value,&mut NativeEncodeControl::new(0,&mut observe)).is_err());assert_eq!(value.nodes,["n"]);
 println!("[DEBUG] Actual DAG IO cancellation: boundaries={baseline} zeroCeiling=true borrowedInputPreserved=true outputRefusalPreservesFacts=true");
}

#[test]
fn actual_retained_selection_io_preserves_original_neutral_bytes_and_every_refusal_frontier(){
 use crate::codec::selection::{DagSelectionJsonCursor,DagSelectionTextGrant,DagSelectionTextFault,DagSelectionTextStep,DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES};
 struct Source(Vec<String>);
 impl DagSelectionSource for Source {
  fn selection_candidate_count(&self,_:DagSelectionDomain)->usize{self.0.len()}
  fn selection_candidate_id(&self,_:DagSelectionDomain,index:usize)->Option<&str>{self.0.get(index).map(String::as_str)}
 }
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🎯️selected-json.json")).unwrap();let mut grants=0usize;let mut refusals=0usize;
 for row in fixture["cases"].as_array().unwrap(){let source=Source(serde_json::from_value(row["ids"].clone()).unwrap());let pointers:Vec<_>=source.0.iter().map(|s|s.as_ptr()).collect();let expected=serde_json::to_vec(&source.0).unwrap();let mut observe=|_|true;assert_eq!(encode_dag_node_ids_json(&source.0,&mut NativeEncodeControl::new(1048576,&mut observe)).unwrap().as_bytes(),expected);
 for domain in [DagSelectionDomain::Nodes,DagSelectionDomain::Edges]{let mut cursor=match domain{DagSelectionDomain::Nodes=>DagSelectionJsonCursor::default(),DagSelectionDomain::Edges=>DagSelectionJsonCursor::edges()};let grant=DagSelectionTextGrant{fuel:1,now_milliseconds:1,deadline_milliseconds:8,cancelled:false,interrupted:false};let mut output=Vec::new();let mut census=None;
 loop {for (bad,fault)in [(DagSelectionTextGrant{fuel:0,..grant},DagSelectionTextFault::NoFuel),(DagSelectionTextGrant{cancelled:true,..grant},DagSelectionTextFault::Cancelled),(DagSelectionTextGrant{interrupted:true,..grant},DagSelectionTextFault::Interrupted),(DagSelectionTextGrant{now_milliseconds:8,..grant},DagSelectionTextFault::Deadline)]{assert_eq!(cursor.step(&source,bad),Err(fault));refusals+=1;}grants+=1;match cursor.step(&source,grant).unwrap(){DagSelectionTextStep::Byte(byte)=>output.push(byte),DagSelectionTextStep::Census{bytes}=>census=Some(bytes),DagSelectionTextStep::Progress{..}=>{},DagSelectionTextStep::Complete=>break}}
 assert_eq!(output,expected);assert_eq!(census,Some(expected.len()));assert_eq!(source.0.iter().map(|s|s.as_ptr()).collect::<Vec<_>>(),pointers);assert_eq!(cursor.step(&source,grant),Ok(DagSelectionTextStep::Complete));}}
 let source=Source(vec!["x".repeat(DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES)]);let grant=DagSelectionTextGrant{fuel:1,now_milliseconds:1,deadline_milliseconds:8,cancelled:false,interrupted:false};let mut cursor=DagSelectionJsonCursor::default();loop{match cursor.step(&source,grant){Err(DagSelectionTextFault::Limit)=>break,Ok(DagSelectionTextStep::Progress{..})=>{},other=>panic!("oversized selection must refuse before output: {other:?}")}}
 println!("[DEBUG] Actual retained selection IO: originalCases=3 domains=2 singleUnitGrants={grants} guardRefusals={refusals} exactSerdeBytes=true sourcePointersPreserved=true ceilingBeforeOutput=true");
}

#[test]
fn typed_channel_direction_is_closed_before_physical_or_host_admission(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧫️fixtures/🔣️.json")).unwrap();
 let unknown=&fixture["refused"].as_array().unwrap().iter().find(|row|row["kind"]=="channels"&&row["value"][0]["direction"]=="side").unwrap()["value"][0];let raw=DslValue::from(unknown);let actual=DagChannelRef::from_value(raw).is_err();let oracle=serde_json::from_value::<DagChannelRef>(unknown.clone()).is_err();
 println!("[DEBUG] Direct typed channel closure: unknownDirectionRefused={actual} independentSerdeRefusal={oracle}");assert!(actual&&oracle,"direction must be declared enum before physical admission");
 for row in fixture["accepted"].as_array().unwrap().iter().filter(|row|row["kind"]=="channels"){for channel in row["value"].as_array().unwrap(){let raw=DslValue::from(channel);let typed=DagChannelRef::from_value(raw).unwrap();assert_eq!(Value::from(&typed.to_value()),channel.clone());assert_eq!(serde_json::to_value(typed).unwrap(),channel.clone());}}
}

#[test]
fn typed_dag_records_refuse_unknown_fields_before_host_admission(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧫️fixtures/🔣️.json")).unwrap();let mut checked=0;
 for row in fixture["refused"].as_array().unwrap(){let value=&row["value"];let kind=row["kind"].as_str().unwrap();let record=match kind{"selection"=>value,"channels"=>&value[0],"statuses"=>&value["n"],_=>continue};if record.get("extra").is_none(){continue}let raw=DslValue::from(value);
 let(actual,oracle)=match kind{"selection"=>(DagSelectionDomains::from_value(raw).is_err(),serde_json::from_value::<DagSelectionDomains>(value.clone()).is_err()),"channels"=>(Vec::<DagChannelRef>::from_value(raw).is_err(),serde_json::from_value::<Vec<DagChannelRef>>(value.clone()).is_err()),"statuses"=>({let mut observe=|_|true;DagNodeStatuses::from_value_controlled(&raw,&mut NativeDecodeControl::new(1048576,&mut observe)).is_err()},serde_json::from_value::<OracleStatuses>(value.clone()).is_err()),_=>unreachable!()};
 println!("[DEBUG] Typed DAG unknown field: family={kind} firstPartyRefused={actual} independentSerdeRefused={oracle}");assert!(actual&&oracle,"undeclared field must refuse in the typed model");checked+=1;
 }
 assert_eq!(checked,3);println!("[DEBUG] Typed DAG record closure: neutralFamilies={checked} independentSerde=true");
}

#[test]
fn hover_input_and_output_refuse_every_cancellation_before_projection_publication(){
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧫️fixtures/🔣️.json")).unwrap();let row=fixture["accepted"].as_array().unwrap().iter().rev().find(|row|row["kind"]=="hover").unwrap();let source=serde_json::to_string(&row["value"]).unwrap();let pointer=source.as_ptr();let mut reads=0;let mut observe=|_|{reads+=1;true};let facts=decode_dag_hover_json(&source,&mut NativeDecodeControl::new(1048576,&mut observe)).unwrap();
 for stop in 0..reads {let mut seen=0;let mut observe=|_|{let accepted=seen!=stop;seen+=1;accepted};assert_eq!(decode_dag_hover_json(&source,&mut NativeDecodeControl::new(1048576,&mut observe)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(source.as_ptr(),pointer);}
 let mut writes=0;let mut observe=|_|{writes+=1;true};let output=encode_dag_hover_json(&facts,&mut NativeEncodeControl::new(1048576,&mut observe)).unwrap();assert_eq!(serde_json::from_str::<Value>(&output).unwrap(),row["value"]);let before=facts.clone();
 for stop in 0..writes {let mut seen=0;let mut observe=|_|{let accepted=seen!=stop;seen+=1;accepted};assert_eq!(encode_dag_hover_json(&facts,&mut NativeEncodeControl::new(1048576,&mut observe)).unwrap_err().kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(facts,before);}
 let mut observe=|_|true;assert!(decode_dag_hover_json(&source,&mut NativeDecodeControl::new(0,&mut observe)).is_err());let mut publish=|_|true;assert!(encode_dag_hover_json(&facts,&mut NativeEncodeControl::new(0,&mut publish)).is_err());
 println!("[DEBUG] Hover projection authority: readCancelBoundaries={reads} writeCancelBoundaries={writes} inputPointerPreserved=true acceptedFactsPreserved=true zeroOwnershipRefused=true independentSerde=true");
}
}
