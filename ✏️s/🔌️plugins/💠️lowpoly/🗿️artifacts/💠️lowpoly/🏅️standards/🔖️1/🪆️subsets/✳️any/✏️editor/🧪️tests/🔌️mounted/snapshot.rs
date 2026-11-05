//! 📥️ Public snapshot JSON ingress preserves painted bytes and exposes one complete host load.
use super::{context,effects::{collect,collect_with_faults}};
use semio_framework_plugin::{PluginApp,artifact_app_laws};
use semio_framework::kernel::Effect;
use semio_framework_value::DslValue;
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use pack::snapshot::ArtifactPack;
use serde_json::Value;

fn source(fixture:&Value)->String{
 let recipe=&fixture["recipe"];let rgba:Vec<u8>=serde_json::from_value(recipe["rgba"].clone()).unwrap();
 let pixels=rgba.repeat(recipe["width"].as_u64().unwrap()as usize*recipe["height"].as_u64().unwrap()as usize);
 serde_json::to_string(&serde_json::json!({"schema":"lowpoly.document","objects":[{"id":recipe["object"]["id"],"name":recipe["object"]["name"],"transform":{"position":[0,0,0],"rotation":[0,0,0],"scale":[1,1,1]},"smoothShading":false,"mesh":null,"meshContent":"","meshState":null,"paintLayers":[{"name":"Color","visible":true,"opacity":1,"blendMode":"normal","pixels":base64_codec::base64_standard_encode(pixels)}]}]})).unwrap()
}

#[semio_framework_async_macros::async_test]
async fn public_snapshot_json_imports_accept_painted_and_exact_limit_inputs_and_refuse_one_extra_byte(){
 let fixture:Value=serde_json::from_str(include_str!("snapshot.json")).unwrap();let json=source(&fixture);
 assert_eq!(json.len(),fixture["expected"]["jsonBytes"].as_u64().unwrap()as usize);
 assert!(json.len()>fixture["limits"]["ordinaryBytes"].as_u64().unwrap()as usize);
 let maximum=fixture["limits"]["jsonBytes"].as_u64().unwrap()as usize;
 for action in fixture["commands"].as_array().unwrap(){
  for length in[json.len(),maximum]{
   let text=format!("{json}{}"," ".repeat(length-json.len()));let mut app=context::app_with_registry().await;
   let before=app.snapshot().unwrap();let history=app.take_history_patch().await.unwrap();let edits=context::committed_edits(&mut app).await;
   let arguments=DslValue::object([("json".to_string(),DslValue::String(text))]);
   artifact_app_laws::dispatch_fixture_action(&mut app.0,action.as_str().unwrap(),Some(arguments),&context::action_meta(),&mut artifact_app_laws::fixture_identity(),app.1.cx(),semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).await.expect("painted snapshot admitted through public original producer");
   let effects=collect(&mut app).await;assert_eq!(effects.len(),fixture["expected"]["loadEffects"].as_u64().unwrap()as usize);
   let Effect::LoadDocument{pack,..}=&effects[0]else{panic!("one complete host document load")};let loaded=LowpolySnapshot::decode_pack(pack).unwrap();
   assert_eq!(loaded.objects.len(),1);assert_eq!(loaded.objects[0].id,fixture["recipe"]["object"]["id"].as_str().unwrap());
   let pixels=&loaded.objects[0].paint_layers[0].pixels;let rgba:Vec<u8>=serde_json::from_value(fixture["recipe"]["rgba"].clone()).unwrap();
   assert_eq!(pixels.len(),fixture["expected"]["pixelBytes"].as_u64().unwrap()as usize);assert!(pixels.chunks_exact(4).all(|pixel|pixel==rgba.as_slice()));
   assert_eq!(app.snapshot().unwrap(),before,"capturing a host effect does not apply it");assert_eq!(context::committed_edits(&mut app).await,edits);assert_eq!(app.take_history_patch().await.unwrap(),history);
  }
  let mut app=context::app_with_registry().await;let before=app.snapshot().unwrap();let edits=context::committed_edits(&mut app).await;
  let text=format!("{json}{}"," ".repeat(maximum+1-json.len()));let arguments=DslValue::object([("json".to_string(),DslValue::String(text))]);
  let immediate=artifact_app_laws::dispatch_fixture_action(&mut app.0,action.as_str().unwrap(),Some(arguments),&context::action_meta(),&mut artifact_app_laws::fixture_identity(),app.1.cx(),semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).await;
  let(effects,mut faults)=collect_with_faults(&mut app).await;if let Err(fault)=immediate{faults.push(fault.code.0);}
  assert!(!faults.is_empty(),"one over-budget byte must refuse");assert!(faults.iter().all(|code|code=="lowpoly.snapshot-json.capacity"),"unrelated refusal is not a budget proof: {faults:?}");
  assert!(effects.is_empty());assert_eq!(app.snapshot().unwrap(),before);assert_eq!(context::committed_edits(&mut app).await,edits);
 }
}
