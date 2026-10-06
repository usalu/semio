//! 🧪️ The same closed graph scene schedules exercise owned decoding and independent serde values.
use super::*;
use semio_framework_value::{DslValue, NativeDecodeControl};
use pack::intrinsic::IntrinsicFormat;
use semio_framework_dsl_record::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};

pub(super) fn graph_scene_encoded(value: DslValue, format: IntrinsicFormat) -> Vec<u8> {
 let spec=RecordSpec::new(None,RecordLayout::Lines,vec![FieldSpec::new(1,"value",Shape::Value)]);
 let record=RecordValue { fields: [(1,FieldValue::Value(value))].into_iter().collect() };
 match format { IntrinsicFormat::Body=>pack::record::encode_record_body(&spec,&record,&Default::default()).unwrap(),IntrinsicFormat::Document=>pack::record::encode_document(&spec,&record,&Default::default()).unwrap() }
}

#[test]
fn canonical_graph_scene_corpus_matches_the_closed_serde_oracle_in_both_declared_frames() {
 let corpus: Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap() {
  let oracle=serde_json::from_value::<NodeGraphScenePayload>(row["input"].clone());
  assert_eq!(oracle.is_ok(),row["accepted"].as_bool().unwrap(),"{} serde",row["id"]);
  for format in [IntrinsicFormat::Body,IntrinsicFormat::Document] {
   let bytes=graph_scene_encoded(DslValue::from(&row["input"]),format);
   let mut calls=0;let mut progress=|_|{calls+=1;true};let mut control=NativeDecodeControl::new(16_777_216,&mut progress);
   let actual=scene::decode(&bytes,format,&Default::default(),&mut control);
   assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap(),"{} {format:?}",row["id"]);
   if let Ok(scene)=actual {assert_eq!(serde_json::to_value(scene.payload).unwrap(),serde_json::to_value(oracle.as_ref().unwrap()).unwrap(),"{} exact independent projection",row["id"]);}
   drop(control);assert!(calls>0);
  }
 }
 eprintln!("[DEBUG] canonical Graph fourteen shared schedules agree with serde_json across Body and Document");
}

#[test]
fn canonical_graph_scene_refuses_implicit_frames_trailing_bytes_revocation_and_owned_budget_exhaustion() {
 let input=serde_json::json!({"nodes":[{"id":"bounded".repeat(1024)}]});
 for format in [IntrinsicFormat::Body,IntrinsicFormat::Document] {
  let bytes=graph_scene_encoded(DslValue::from(&input),format);
  let other=match format {IntrinsicFormat::Body=>IntrinsicFormat::Document,IntrinsicFormat::Document=>IntrinsicFormat::Body};
  let mut accept=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accept);
  assert!(scene::decode(&bytes,other,&Default::default(),&mut control).is_err());
  let mut cancel=|_|false;let mut control=NativeDecodeControl::new(16_777_216,&mut cancel);
  assert!(scene::decode(&bytes,format,&Default::default(),&mut control).is_err());
  let mut accept=|_|true;let mut control=NativeDecodeControl::new(1,&mut accept);
  assert!(scene::decode(&bytes,format,&Default::default(),&mut control).is_err());assert!(control.owned_bytes()<=1);
 }
 let mut trailing=graph_scene_encoded(DslValue::from(&input),IntrinsicFormat::Body);trailing.push(0);
 let mut accept=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accept);
 assert!(scene::decode(&trailing,IntrinsicFormat::Body,&Default::default(),&mut control).is_err());
 assert!(scene::from_json(r#"{"nodes":[],"nodes":[]}"#,&mut control).is_err());
}

#[test]
fn canonical_graph_scene_commit_keeps_opaque_text_and_original_projection_behavior() {
 let input=r#"{"nodes":[{"id":"a","outputs":[{"id":"out"}]}],"viewport":{"x":11,"y":22,"zoom":3},"controlsJson":"pk:opaque","capabilitiesJson":"cap"}"#;
 let mut accept=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accept);
 let scene=scene::from_json(input,&mut control).unwrap();let mut host=GraphHost::default();host.synchronize_scene(scene).unwrap();
 assert_eq!(host.dag.host_snapshot.nodes.iter().map(|node|node.id.as_str()).collect::<Vec<_>>(),vec!["a"]);
 assert_eq!(host.viewport(),Viewport2d {x:11.0,y:22.0,zoom:3.0});assert_eq!(host.controls_json,"pk:opaque");assert_eq!(host.capabilities_json,"cap");
}
