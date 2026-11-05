//! 🧪️ Shared generation folds match neutral SQL states and the actual product central apply.
use super::{GenerationPlayState,FormGeneration};
use super::retained_generation_fold::{GenerationFoldInput,GenerationFoldOutcome,GenerationFoldWork};
use super::retained_scratch_testing::ScratchCaller;
use super::retained_host_scratch::HostScratchDomain;
use semio_framework_job::close_original_retained_step;
fn state(value:&serde_json::Value)->GenerationPlayState{semio_framework_pack_json::from_json_str(&value.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
fn input(row:&serde_json::Value)->GenerationFoldInput{
 let text=|key:&str|row[key].as_str().unwrap().to_owned();let optional=|key:&str|row[key].as_str().map(str::to_owned);
 match row["kind"].as_str().unwrap(){
  "add"=>GenerationFoldInput::Add{generation:semio_framework_pack_json::from_json_str::<FormGeneration>(&row["generation"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap(),index:row["index"].as_u64().map(|index|index as usize)},
  "remove"=>GenerationFoldInput::Remove{id:text("id")},"rename"=>GenerationFoldInput::Rename{id:text("id"),name:text("name")},
  "setValue"=>GenerationFoldInput::SetValue{id:text("id"),key:text("key"),value:row["value"].clone().into()},
  "select"=>GenerationFoldInput::Select{id:optional("id")},"preview"=>GenerationFoldInput::Preview{text:optional("text")},_=>panic!("unknown closed generation event")
 }
}
fn wire(domain:HostScratchDomain,row:&serde_json::Value)->Option<serde_json::Value>{
 let kind=row["kind"].as_str().unwrap();let mut payload=serde_json::Map::new();
 let variant=match kind{
  "add"=>{payload.insert("generation".into(),row["generation"].clone());payload.insert("index".into(),row["index"].clone());"CreateGeneration"},
  "remove"=>{payload.insert("id".into(),row["id"].clone());"DeleteGeneration"},
  "rename"=>{payload.insert("id".into(),row["id"].clone());payload.insert(if domain==HostScratchDomain::Generation2d{"name"}else{"newName"}.into(),row["name"].clone());"RenameGeneration"},
  "setValue"=>{payload.insert("id".into(),row["id"].clone());payload.insert("questionId".into(),row["key"].clone());payload.insert(if domain==HostScratchDomain::Generation2d{"value"}else{"newValue"}.into(),row["value"].clone());"ChangeGenerationValue"},
  "select"=>{payload.insert("generationId".into(),row["id"].clone());"SelectGeneration"},
  "preview"=>{if domain==HostScratchDomain::Generation2d{return None}payload.insert("text".into(),row["text"].clone());"ChangeGenerationPreview"},_=>panic!("unknown closed generation wire")
 };
 if domain==HostScratchDomain::Generation2d{let mut object=serde_json::Map::new();object.insert(variant.into(),serde_json::Value::Object(payload));Some(serde_json::Value::Object(object))}else{let mut tag=variant[..1].to_lowercase();tag.push_str(&variant[1..]);payload.insert("mutation".into(),tag.into());Some(serde_json::Value::Object(payload))}
}
fn discard(value:GenerationPlayState){<GenerationPlayState as semio_framework_value::FromValue>::retire_decoded(value)}
#[test]
fn original_generation_fold_matches_central_apply_with_one_fuel_frames_and_full_physical_close(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let domain=super::host_scratch_test_domain();
 for row in corpus["cases"].as_array().unwrap(){
  let expected=state(&row["after"]);if let Some(wire)=wire(domain,&row["mutation"]){let original=state(&row["before"]);let(next,accepted)=super::generation_fold_test_original(&original,&wire);assert_eq!(next,expected,"{}",row["name"]);assert_eq!(accepted,matches!(row["outcome"].as_str().unwrap(),"applied"|"unchanged"));discard(original);discard(next)}discard(expected);
  for profile in [1usize,512]{for cut in [None,Some(0usize),Some(1),Some(2)]{
   let((mut source,edit),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(state(&row["before"]),input(&row["mutation"])));assert!(!seed.overflowed);let mut caller=ScratchCaller::new(&policy,profile,seed.requested_bytes-seed.released_bytes);let mut work=caller.turn(|cx|Ok(GenerationFoldWork::new(edit,cx)));let mut ready=false;
   for opportunity in 0..16384{if cut==Some(opportunity){caller.cancel();let cancelled=caller.turn(|cx|work.step(&mut source,cx));assert!(!cancelled);break}ready=caller.turn(|cx|work.step(&mut source,cx));if ready{break}}
   let body_calls=caller.calls;let actual=state(&row["after"]);let same=source==actual;discard(actual);let outcome=work.outcome().map(GenerationFoldOutcome::code);work.begin_close();
   for _ in 0..16384{if work.terminal_is_empty(){break}caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}
   assert!(work.terminal_is_empty());drop(work);caller.close(source);let total_calls=caller.finish();if cut.is_none(){assert!(ready,"{} did not become ready (body {}, all {})",row["name"],body_calls,total_calls);assert!(same,"{}",row["name"]);assert_eq!(outcome,Some(row["outcome"].as_str().unwrap()))}
  }}
 }
}

#[test]
fn generation_fold_rejects_a_distinct_original_caller_with_the_same_labels(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let stock:semio_framework_value::RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
 let((mut source,edit),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(state(&corpus["cases"][0]["before"]),GenerationFoldInput::Select{id:None}));let mut caller=ScratchCaller::new(&policy,1,seed.requested_bytes-seed.released_bytes);let mut work=caller.turn(|cx|Ok(GenerationFoldWork::new(edit,cx)));
 let mut foreign=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(policy["operation"].as_u64().unwrap()),semio_framework_job::Generation(policy["generation"].as_u64().unwrap()),stock,super::retained_scratch_testing::clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let fuel=foreign.fuel_remaining();let before=foreign.cumulative_receipt();
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||foreign.turn(stock,|cx|work.step(&mut source,cx)));let refused=result.is_err();let zero=foreign.fuel_remaining()==fuel&&foreign.cumulative_receipt()==before&&(heap.requested_bytes,heap.released_bytes)==(0,0);foreign.retire(stock);
 let unchanged=source.selected_generation_id.as_deref()==Some("g1")&&work.outcome().is_none();work.begin_close();while !work.terminal_is_empty(){caller.turn(|cx|cx.with_original_retirement(|same|{let d=work.next_close_demands(same.retained_grant().maximum_copy_bytes)?;close_original_retained_step(same,d,|g|work.close_step(g))?;Ok(())}));}drop(work);caller.close(source);caller.finish();assert!(refused&&zero&&unchanged);
}
