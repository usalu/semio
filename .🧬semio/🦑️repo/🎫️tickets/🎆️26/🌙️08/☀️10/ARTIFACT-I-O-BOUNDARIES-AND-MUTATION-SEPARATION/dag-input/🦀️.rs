#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🧬️schema/🎯️dag-input/🦀️.rs"]
pub mod model;
#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🦀️.rs"]
pub mod codec;
pub mod infinite {pub mod board {pub mod schema {pub use crate::model as dag_input;}}}
#[cfg(test)]
mod tests {
use super::{model::*,codec::*};
use semio_framework_value::{DslValue,NativeDecodeControl,NativeEncodeControl,ToValue,ValueError};
use serde_json::Value;
fn input(kind:&str,source:&str,control:&mut NativeDecodeControl<'_>)->Result<DslValue,ValueError>{match kind {
 "selection"=>decode_dag_selection_json(source,control).map(|value|value.to_value()),
 "channels"=>decode_dag_channels_json(source,control).map(|value|value.to_value()),
 "statuses"=>decode_dag_node_statuses_json(source,control).map(|value|value.to_value()),
 _=>panic!("unknown neutral family"),
}}
fn oracle(kind:&str,source:&str)->Value {match kind {
 "selection"=>serde_json::to_value(serde_json::from_str::<DagSelectionDomains>(source).unwrap()).unwrap(),
 "channels"=>serde_json::to_value(serde_json::from_str::<Vec<DagChannelRef>>(source).unwrap()).unwrap(),
 "statuses"=>serde_json::to_value(serde_json::from_str::<DagNodeStatuses>(source).unwrap()).unwrap(),
 _=>panic!("unknown neutral family"),
}}
#[test]
fn actual_dag_input_admission_and_publication_agree_with_neutral_and_serde() {
 let fixture:Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧫️fixtures/🔣️.json")).unwrap();let mut observations=0;
 for case in fixture["accepted"].as_array().unwrap(){let source=serde_json::to_string(&case["value"]).unwrap();let original=source.as_ptr();let kind=case["kind"].as_str().unwrap();
  let mut observe=|_|{observations+=1;true};let mut control=NativeDecodeControl::new(1048576,&mut observe);let actual=input(kind,&source,&mut control).unwrap();assert_eq!(Value::from(&actual),oracle(kind,&source));assert_eq!(source.as_ptr(),original);
  let mut publish=|_|true;let mut control=NativeEncodeControl::new(1048576,&mut publish);let encoded=match kind {"selection"=>encode_dag_selection_json(&serde_json::from_str(&source).unwrap(),&mut control).unwrap(),"channels"=>encode_dag_channels_json(&serde_json::from_str(&source).unwrap(),&mut control).unwrap(),_=>continue};assert_eq!(serde_json::from_str::<Value>(&encoded).unwrap(),case["value"]);
 }
 for case in fixture["refused"].as_array().unwrap(){let source=serde_json::to_string(&case["value"]).unwrap();let mut observe=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut observe);assert!(input(case["kind"].as_str().unwrap(),&source,&mut control).is_err());}
 #[derive(serde::Deserialize)]struct StatusSample{n:DagNodeEvaluationStatus}
 for case in fixture["rawRefused"].as_array().unwrap(){let source=case["text"].as_str().unwrap();let kind=case["kind"].as_str().unwrap();let mut observe=|_|true;let mut control=NativeDecodeControl::new(1048576,&mut observe);assert!(input(kind,source,&mut control).is_err());if kind=="selection"{assert!(serde_json::from_str::<DagSelectionDomains>(source).is_err());}else{assert!(serde_json::from_str::<StatusSample>(source).is_err());}}
 println!("[DEBUG] Actual DAG IO: accepted=6 refused=12 rawDuplicateRefusals=2 independentSerde=true progressObservations={observations} sourcePointersPreserved=true");
}
#[test]
fn actual_dag_input_cancellation_and_zero_ceiling_preserve_borrowed_input() {
 let source=r#"{"nodes":["node雪"],"edges":[],"handles":[]}"#.to_owned();let pointer=source.as_ptr();let mut baseline=0;let mut observe=|_|{baseline+=1;true};assert!(decode_dag_selection_json(&source,&mut NativeDecodeControl::new(1048576,&mut observe)).is_ok());
 for stop in 0..baseline {let mut seen=0;let mut observe=|_|{let accepted=seen!=stop;seen+=1;accepted};let error=decode_dag_selection_json(&source,&mut NativeDecodeControl::new(1048576,&mut observe)).unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(source.as_ptr(),pointer);}
 let mut observe=|_|true;let error=decode_dag_selection_json(&source,&mut NativeDecodeControl::new(0,&mut observe)).unwrap_err();assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::OwnershipLimit);assert_eq!(source.as_ptr(),pointer);
 let value=DagSelectionDomains{nodes:vec!["n".into()],edges:vec![],handles:vec![]};let mut observe=|_|true;assert!(encode_dag_selection_json(&value,&mut NativeEncodeControl::new(0,&mut observe)).is_err());assert_eq!(value.nodes,["n"]);
 println!("[DEBUG] Actual DAG IO cancellation: boundaries={baseline} zeroCeiling=true borrowedInputPreserved=true outputRefusalPreservesFacts=true");
}
}
