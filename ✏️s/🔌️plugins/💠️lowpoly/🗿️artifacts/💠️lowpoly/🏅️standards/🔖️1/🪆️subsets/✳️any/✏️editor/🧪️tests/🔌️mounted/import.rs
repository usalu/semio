//! 📥️ Public chunk staging and typed admission publish one complete mesh replacement.
use super::context;
use semio_framework_plugin::{PluginApp,artifact_app_laws};
use semio_framework::kernel::{Effect,import_payload_chunks,import_chunk_arguments};
use semio_s_artifact_lowpoly_lowpoly_model::snapshot::LowpolySnapshot;
use pack::snapshot::ArtifactPack;
use serde_json::Value;

use super::effects::collect;

#[semio_framework_async_macros::async_test]
async fn chunked_large_mesh_import_publishes_exactly_one_complete_load_document(){
 let fixture:Value=serde_json::from_str(include_str!("import.json")).unwrap();
 let raw=format!("{}{}",fixture["source"].as_str().unwrap(),fixture["paddingLine"].as_str().unwrap().repeat(fixture["paddingLines"].as_u64().unwrap()as usize));
 for form in fixture["forms"].as_array().unwrap(){
  let payload=if form=="dataUrl"{format!("data:model/obj;base64,{}",base64_codec::base64_standard_encode(raw.as_bytes()))}else{raw.clone()};
  assert!(payload.len()>16_384&&payload.len()<semio_s_artifact_lowpoly_lowpoly_commands::media::LOWPOLY_MESH_FILE_BYTES);
  let chunks=import_payload_chunks(&payload);assert!(chunks.len()>1);let mut app=context::app_with_registry().await;let before=app.snapshot().unwrap();let edits=context::committed_edits(&mut app).await;let mut effects=Vec::new();
  for chunk in &chunks{
   artifact_app_laws::dispatch_fixture_action(&mut app.0,fixture["action"].as_str().unwrap(),Some(import_chunk_arguments(fixture["name"].as_str().unwrap(),chunk,None)),&context::action_meta(),&mut artifact_app_laws::fixture_identity(),app.1.cx(),semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap();
   let published=collect(&mut app).await;if chunk.chunk+1<chunk.chunk_count{assert!(published.is_empty(),"partial input must not publish a replacement");}effects.extend(published);assert_eq!(app.snapshot().unwrap(),before);assert_eq!(context::committed_edits(&mut app).await,edits);
  }
  assert_eq!(effects.len(),fixture["expected"]["loadEffects"].as_u64().unwrap()as usize);let Effect::LoadDocument{pack,..}=&effects[0]else{panic!("expected one complete document load")};let loaded=LowpolySnapshot::decode_pack(pack).unwrap();assert_eq!(loaded.objects.len(),fixture["expected"]["objects"].as_u64().unwrap()as usize);let mesh=loaded.objects[0].mesh_state.as_ref().unwrap().clone().into_mesh().unwrap();assert_eq!(mesh.face_count(),fixture["expected"]["faces"].as_u64().unwrap()as usize);
  let mut positions:Vec<_>=(0..mesh.vertex_count()).map(|vertex|{let p=mesh.vertex_position(semio_framework_3d::mesh::VertexId(vertex as u32)).unwrap();[p.x() as f64,p.y() as f64,p.z() as f64]}).collect();positions.sort_by(|a,b|a.partial_cmp(b).unwrap());let mut expected:Vec<[f64;3]>=serde_json::from_value(fixture["expected"]["positions"].clone()).unwrap();expected.sort_by(|a,b|a.partial_cmp(b).unwrap());assert_eq!(positions,expected);
 }
}
