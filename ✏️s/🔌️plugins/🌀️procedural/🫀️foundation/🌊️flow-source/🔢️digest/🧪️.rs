//! 🧪️ Original preview hashes equal the existing FNV implementation and the authored neutral corpus.
use super::{mix,OFFSET,EPOCH};
use std::hash::Hasher;
#[test]
fn native_preview_link_digest_matches_neutral_parts_and_independent_fnv(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  let parts:Vec<&str>=row["parts"].as_array().unwrap().iter().map(|part|part.as_str().unwrap()).collect();
  let epoch=row["provisionalGeneration"].as_str().unwrap().parse::<u64>().unwrap();
  let expected=u64::from_str_radix(row["expected"].as_str().unwrap(),16).unwrap();
  let mut hash=OFFSET;let mut oracle=fnv::FnvHasher::default();
  for part in &parts{for byte in part.bytes().chain(std::iter::once(0x1f)){hash=mix(hash,byte)}oracle.write(part.as_bytes());oracle.write(&[0x1f]);}
  let epoch=epoch.wrapping_mul(EPOCH);
  assert_eq!(hash^epoch,expected);
  assert_eq!(oracle.finish()^epoch,expected);
  assert_eq!(semio_s_artifact_procedural_2d_preview_eval::preview_eval_digest(&parts)^epoch,expected);
 }
}

use super::PreviewLinkDigestWork;
use super::ProceduralFlowSourceWork;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation,StepContext,OriginalTurnFailure,close_original_retained_step};
use semio_os_kernel::os_store::snapshot_read as reads;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ErasedSnapshotRetirement,RetirementDemand};
use semio_framework_value::retirement::arc_custody;
use std::sync::Arc;
type Root=semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
fn clock()->Option<u64>{Some(1000)}
fn delta(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:b.copied_items-a.copied_items,copied_bytes:b.copied_bytes-a.copied_bytes,retained_capacity_bytes:b.retained_capacity_bytes-a.retained_capacity_bytes,released_bytes:b.released_bytes-a.released_bytes}}
fn turn<R>(caller:&mut OriginalJobFixtureCaller,stock:RetainedCloneGrant,decoder:&mut OriginalJobDecodeOwner,calls:&mut usize,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->R{
 *calls+=1;assert!(*calls<=200);
 caller.turn_with_decoder(stock,decoder,|cx|{
  let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||run(cx));let p=delta(before,cx.retained_progress());assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result
 }).unwrap()
}
fn fixture(root:&Root)->&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree{&root.host_snapshot}
fn generation(root:&Root)->&dyn semio_framework_pack_json::ArtifactCanonicalJsonTree{root.generation.as_state()}
macro_rules! close_work{($owner:ident,$caller:ident,$stock:ident,$decoder:ident,$calls:ident)=>{{
 $owner.begin_close();
 while !$owner.terminal_is_empty(){turn(&mut $caller,$stock,&mut $decoder,&mut $calls,|cx|cx.with_original_retirement(|cx|{
  for _ in 0..512{if $owner.terminal_is_empty()||cx.should_yield(){break}let d=$owner.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|$owner.close_step(g))?;if step.progress()==Default::default(){break}}Ok(())
 }));}
}}}
#[test]
fn original_preview_digest_keeps_exact_committed_sources_within_200_caller_turns(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
 for with_generation in [false,true]{for cancel_after in [None,Some(0usize),Some(1),Some(2)]{
  let root=Arc::new(Root::default());let selected="ä/東京".repeat(257);let provisional=19u64;
  let fixture_wire=semio_framework_pack_json::to_json_string(&root.host_snapshot);let generation_wire=semio_framework_pack_json::to_json_string(root.generation.as_state());
  let parts=if with_generation{vec![fixture_wire.as_str(),generation_wire.as_str(),selected.as_str()]}else{vec![fixture_wire.as_str()]};
  let expected=semio_s_artifact_procedural_2d_preview_eval::preview_eval_digest(&parts)^provisional.wrapping_mul(EPOCH);
  let mut oracle=fnv::FnvHasher::default();for part in parts{oracle.write(part.as_bytes());oracle.write(&[0x1f])}assert_eq!(expected,oracle.finish()^provisional.wrapping_mul(EPOCH));
  let registry=reads::SnapshotReadRegistryHandle::new();let revision=[73;32];let business=17;assert!(registry.publish_authority(business,revision));let lease=registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_|panic!("original preview issuer"));let read=reads::SnapshotRead::new(root,lease);let identity=read.lease_identity();let pointer=read.get()as*const Root;
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()),Generation(policy["generation"].as_u64().unwrap()),stock,clock,policy["deadlineUs"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let mut decoder=OriginalJobDecodeOwner::empty();let mut calls=0;
  let mut source=turn(&mut caller,stock,&mut decoder,&mut calls,|cx|Ok(ProceduralFlowSourceWork::new(read,|root|&root.host_snapshot,revision,business,false,cx)));
  let(mut work,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||PreviewLinkDigestWork::new(fixture,with_generation.then_some(generation as super::TreeProject<Root>),provisional));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut output=None;
  for opportunity in 0..200{if cancel_after==Some(opportunity){break}assert_eq!(source.original().unwrap().lease_identity(),identity);assert_eq!(source.original().unwrap().get()as*const Root,pointer);
   output=turn(&mut caller,stock,&mut decoder,&mut calls,|cx|work.step(&mut source,&selected,cx));if output.is_some(){break}
  }
  if cancel_after.is_none(){assert_eq!(output,Some(expected))}
  if let Some(output)=output{work.restore_output(output).unwrap();}
  work.begin_close();if !work.terminal_is_empty(){let d=work.next_close_demands(64).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if d.release_bytes>0=>denied.maximum_release_bytes-=1,4 if d.depth>0=>denied.maximum_depth-=1,_=>continue}let(outcome,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||work.close_step(denied));assert_eq!(outcome.work,0);assert_eq!(outcome.result.unwrap().progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}}
  close_work!(work,caller,stock,decoder,calls);close_work!(source,caller,stock,decoder,calls);
  let mut active=None;
  while registry.has_returned()||active.is_some(){
   turn(&mut caller,stock,&mut decoder,&mut calls,|cx|cx.with_original_retirement(|cx|{
    for _ in 0..512{
     if cx.should_yield(){break}
     let d=if let Some(owner)=active.as_ref(){reads::artifact_retirement_box_demands(owner,cx.retained_grant().maximum_copy_bytes)?}else if registry.has_returned(){RetirementDemand{capacity_bytes:arc_custody::shared_retirement_birth_bytes::<Root>(),depth:1,..Default::default()}}else{break};
     let step=close_original_retained_step(cx,d,|g|{
      if active.is_some(){reads::artifact_retirement_box_close_step(&mut active,g)}else{let outcome=registry.try_admit_one_returned::<Root,_>(g,|root,g|arc_custody::admit_shared_retirement(root,g,false));let result=outcome.result.map(|(owner,p)|{active=owner;RetainedCloneStep::Progress(p)}).map_err(|error|error.into_value_error());RetainedWorkOutcome{result,work:outcome.work}}
     })?;if step.progress()==Default::default(){break}
    }Ok(())
   }));
  }
  let mut issuer=reads::SnapshotReadRegistryAliasRetirement::new(registry);
  while !issuer.terminal_is_empty(){turn(&mut caller,stock,&mut decoder,&mut calls,|cx|cx.with_original_retirement(|cx|{for _ in 0..512{if issuer.terminal_is_empty()||cx.should_yield(){break}let d=issuer.next_demand(cx.retained_grant().maximum_copy_bytes)?;let step=close_original_retained_step(cx,d,|g|issuer.close_step(g))?;if step.progress()==Default::default(){break}}Ok(())}));}
  while !decoder.terminal_is_empty(){calls+=1;assert!(calls<=200);caller.turn(stock,|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;decoder.close_on_source(cx)?;Ok(())})).unwrap();}
  assert_eq!(caller.stock(),stock);assert!(caller.cumulative_receipt().fits(stock));caller.retire(stock);
 }}
}
