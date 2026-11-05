//! 🧪️ Corpus versions and genuine immutable child-root aliases define cache hits.
use super::*;
#[test]
fn retained_scene_cache_versions_and_real_child_roots_match_the_neutral_corpus(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:16*1024*1024,maximum_release_bytes:16*1024*1024,maximum_depth:4096};
 for row in corpus["cases"].as_array().unwrap(){
  let key=|value:&serde_json::Value|Key{instance:value["instance"].as_u64().unwrap().try_into().unwrap(),revision:[u8::from_str_radix(value["revision"].as_str().unwrap(),16).unwrap();32],generation:value["generation"].as_u64().unwrap()};
  let mut left=Some(Children{root:(!row["stored"]["root"].is_null()).then(||std::sync::Arc::new(semio_plugin::ChildContentRoot::default()))});
  let mut right=Some(Children{root:if row["query"]["root"].is_null(){None}else if row["stored"]["root"]==row["query"]["root"]{left.as_ref().unwrap().root.clone()}else{Some(std::sync::Arc::new(semio_plugin::ChildContentRoot::default()))}});
  let actual=key(&row["stored"]).matches(left.as_ref().unwrap(),key(&row["query"]),right.as_ref().unwrap());
  let expected=row["matches"].as_bool().unwrap();let mut retirement=RetainedCloneClose::default();let mut turns=0;
  while left.is_some()||right.is_some()||!retirement.is_empty(){
   let turn=if !retirement.is_empty(){retirement.step_granted(grant)}else{let turn=retirement.begin_granted(if left.is_some(){&mut left}else{&mut right},grant);RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}};
   assert!(turn.work<=1);assert!(turn.result.unwrap().progress().fits(grant));turns+=1;assert!(turns<=200);
  }
  assert_eq!(actual,expected,"{}",row["name"]);
 }
 let mut cache=FlowSceneCache::new();cache.begin_close();assert!(cache.terminal_is_empty());assert_eq!(cache.advance(grant).work,0);
}
