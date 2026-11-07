//! 🧪️ Genuine application reads seed native scratch fields and return every issuer lease under the original caller.
use super::retained_provisional_scene::ProvisionalSceneWork;
use super::retained_scratch_testing::ScratchCaller;
use semio_framework_value_snapshot_read as reads;
use semio_framework_value::{RetirementDemand,RetainedWorkOutcome,RetainedCloneStep};
use semio_framework_value::retirement::arc_custody;
use semio_framework_job::close_original_retained_step;
use semio_plugin::ArtifactApp;
use std::sync::Arc;
type App=super::ProvisionalSceneTestApp;
type Root=<App as ArtifactApp>::Snapshot;
fn close(work:&mut ProvisionalSceneWork<App,super::ProvisionalSceneTestFolds>,caller:&mut ScratchCaller){
 work.begin_close();for _ in 0..32768{if work.terminal_is_empty(){return}caller.turn(|cx|cx.with_original_retirement(|cx|{let d=work.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|work.close_step(g))?;Ok(())}));}panic!("scene owners did not close")
}
#[test]
fn original_procedural_scene_keeps_actual_read_and_closes_all_native_fields_with_one_fuel(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🧪️tests/🧫️fixtures/🎟️caller.json")).unwrap();
 for items in [1,512]{for base_stage in [false,true]{for cut in [None,Some(0),Some(1),Some(2),Some(7)]{
  let revision=[61;32];let business=23;
  let((registry,read),seed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
   let mut original=Root::default();original.host_snapshot.schema="source.源".into();let root=Arc::new(original);let registry=reads::SnapshotReadRegistryHandle::new();assert!(registry.publish_authority(business,revision));
   let lease=registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_|panic!("actual scene read issuer"));let read=reads::SnapshotRead::new(root,lease);(registry,read)
  });assert!(!seed.overflowed);
  let pointer=read.get()as*const Root;let identity=read.lease_identity();
  let expected_host=semio_framework_pack_json::to_json_string(&read.get().host_snapshot);let expected_generation=semio_framework_pack_json::to_json_string(read.get().generation.as_state());
  let mut caller=ScratchCaller::new(&policy,items,seed.requested_bytes.checked_sub(seed.released_bytes).unwrap());
  let mut work=caller.turn(|cx|Ok(ProvisionalSceneWork::<App,super::ProvisionalSceneTestFolds>::new(read,|root|&root.host_snapshot,|root|root.generation.as_state(),revision,business,cx)));
  let mut foreign=ScratchCaller::new(&policy,items,0);
  let failure=foreign.turn(|cx|Ok(work.step(cx).expect_err("same labels on a foreign real caller must refuse")));
  foreign.close(failure);foreign.finish();
  let mut turns=0;loop{
   assert_eq!(work.source_owner().original().unwrap().get()as*const Root,pointer);assert_eq!(work.source_owner().original().unwrap().lease_identity(),identity);
   if !base_stage&&cut==Some(turns){caller.cancel();let observed=caller.turn(|cx|{let canceled=cx.is_cancelled();let ready=work.step(cx)?;Ok(canceled&&!ready)});assert!(observed);break}
   let ready=caller.turn(|cx|work.step(cx));turns+=1;if ready{break}assert!(turns<32768)
  }
  let matched=if cut.is_none()||base_stage{semio_framework_pack_json::to_json_string(work.source().unwrap().unwrap())==expected_host&&semio_framework_pack_json::to_json_string(work.generation().unwrap())==expected_generation}else{true};
  if cut.is_none()||base_stage{
   let identity=work.source().unwrap().unwrap().widgets.as_ptr();
   let capacity=work.source().unwrap().unwrap().widgets.capacity();
   let mut held=None;
   let denied=caller.turn(|cx|{
    let mut g=cx.retained_grant();g.maximum_copy_bytes=0;
    let turn=work.source_owner_mut().take_scratch_original(&mut held,g);
    let ready=turn.result.as_ref().copied().unwrap_or(false);
    semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(|_|RetainedCloneStep::Progress(Default::default())),work:turn.work})?;
    Ok(ready)
   });assert!(!denied&&held.is_none()&&work.source().unwrap().is_some());
   let mut foreign=ScratchCaller::new(&policy,items,0);
   let failure=foreign.turn(|cx|Ok(work.source_owner_mut().take_scratch_into(&mut held,cx).expect_err("foreign scratch recipient")));
   foreign.close(failure);foreign.finish();assert!(held.is_none());
   assert!(caller.turn(|cx|work.source_owner_mut().take_scratch_into(&mut held,cx)));
   assert!(work.source().unwrap().is_none());assert_eq!(held.as_ref().unwrap().widgets.as_ptr(),identity);assert_eq!(held.as_ref().unwrap().widgets.capacity(),capacity);
   assert!(caller.turn(|cx|work.source_owner_mut().restore_scratch_from(&mut held,cx)));
   assert!(held.is_none());assert_eq!(work.source().unwrap().unwrap().widgets.as_ptr(),identity);
  }
  if base_stage{
   let mut base=caller.turn(|cx|Ok(super::retained_scene_base::OriginalSceneBaseWork::new(cx)));
   let mut turns=0;
   loop{
    if cut==Some(turns){caller.cancel();assert!(caller.turn(|cx|Ok(cx.is_cancelled()&&!base.step(work.source_owner_mut(),cx)?)));break}
    if caller.turn(|cx|base.step(work.source_owner_mut(),cx)){break}
    turns+=1;assert!(turns<32768);
   }
   if cut.is_none(){
    assert_eq!(semio_framework_pack_json::to_json_string(base.base().unwrap().unwrap()),expected_host);
    assert_eq!(semio_framework_pack_json::to_json_string(work.source().unwrap().unwrap()),expected_host);
    assert_ne!(base.base().unwrap().unwrap().schema.as_ptr(),work.source().unwrap().unwrap().schema.as_ptr());
   }
   base.begin_close();
   for _ in 0..32768{
    if base.terminal_is_empty(){break}
    caller.turn(|cx|cx.with_original_retirement(|cx|{let d=base.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;close_original_retained_step(cx,d,|g|base.close_step(g))?;Ok(())}));
   }
   assert!(base.terminal_is_empty());
  }
  close(&mut work,&mut caller);
  let mut active=None;
  for _ in 0..32768{
   if !registry.has_returned()&&active.is_none(){break}
   caller.turn(|cx|cx.with_original_retirement(|cx|{
    let d=if let Some(owner)=active.as_ref(){reads::artifact_retirement_box_demands(owner,cx.retained_grant().maximum_copy_bytes)?}else{RetirementDemand{capacity_bytes:arc_custody::shared_retirement_birth_bytes::<Root>(),depth:1,..Default::default()}};
    close_original_retained_step(cx,d,|g|{
     if active.is_some(){reads::artifact_retirement_box_close_step(&mut active,g)}else{let turn=registry.try_admit_one_returned::<Root,_>(g,|root,g|arc_custody::admit_shared_retirement(root,g,false));let result=turn.result.map(|(owner,p)|{active=owner;RetainedCloneStep::Progress(p)}).map_err(|error|error.into_value_error());RetainedWorkOutcome{result,work:turn.work}}
    })?;Ok(())
   }));
  }
  assert!(!registry.has_returned()&&active.is_none());
  let mut issuer=Some(reads::SnapshotReadRegistryAliasRetirement::new(registry));
  for _ in 0..32768{if issuer.is_none(){break}caller.turn(|cx|cx.with_original_retirement(|cx|{let d=reads::snapshot_registry_alias_demands(&issuer)?;close_original_retained_step(cx,d,|g|RetainedWorkOutcome{result:reads::snapshot_registry_alias_close_step(&mut issuer,g),work:1})?;Ok(())}));}
  assert!(issuer.is_none());caller.finish();assert!(matched);
 }}}
}
