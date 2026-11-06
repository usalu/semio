//! 🧮️ Real EN 1990 public replay preparation pays nested owners under its original caller.
use semio_framework_job::{Generation,OperationId,OriginalJobDecodeOwner,OriginalJobFixtureCaller};
use semio_framework_os_store_one_item_batch::paged_one_item::ColdMutationApplyFactory;
use semio_os_store_replay_operation::{ArtifactMutationSource,ArtifactReplayPreparationContext,ArtifactReplayPreparationFactory,ArtifactReplayPreparationMode,ArtifactReplayPreparationRequest,ArtifactReplayPreparationStep};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,ErasedSnapshotRetirement,FromValue,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_snapshot_read::{SnapshotRead,SnapshotReadRegistryHandle};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use semio_s_artifact_norm_en1990_mutations::En1990Mutation;
use std::sync::Arc;

#[global_allocator]
static ALLOCATOR:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

trait Receipt{fn receipt(&self)->RetainedCloneProgress;}
impl Receipt for RetainedCloneStep{fn receipt(&self)->RetainedCloneProgress{self.progress()}}
impl Receipt for ArtifactReplayPreparationStep{fn receipt(&self)->RetainedCloneProgress{self.progress()}}
impl<T>Receipt for(T,RetainedCloneProgress){fn receipt(&self)->RetainedCloneProgress{self.1}}
impl<T>Receipt for Option<(T,RetainedCloneProgress)>{fn receipt(&self)->RetainedCloneProgress{self.as_ref().map_or(Default::default(),|(_,p)|*p)}}
impl Receipt for Option<RetainedCloneProgress>{fn receipt(&self)->RetainedCloneProgress{self.unwrap_or_default()}}
fn retained(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:2_000_000,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<28,maximum_release_bytes:1<<28,maximum_depth:4096}}
#[derive(Debug)]
struct Unpaid{turn:usize,actual:(usize,usize),reported:RetainedCloneProgress}
struct Probe{caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner,turn:usize,unpaid:Option<Unpaid>}
impl Probe{
 fn new()->Self{Self{caller:OriginalJobFixtureCaller::admit(OperationId(199007),Generation(1),stock(),||Some(1),u64::MAX,2_000_000),decoder:OriginalJobDecodeOwner::empty(),turn:0,unpaid:None}}
 fn run<T:Receipt>(&mut self,g:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>)->Result<T,ValueError>)->T{
  let mut measured=None;
  let result=self.caller.turn_with_decoder(g,&mut self.decoder,|cx|cx.with_original_work_loan(4096,cx.deadline_us(),|cx|{
   let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cx.with_native_receiving(run));
   let p=result.as_ref().map_or_else(ValueError::retained_progress,Receipt::receipt);measured=Some(((heap.requested_bytes,heap.released_bytes),p));
   let issued=cx.receiving_issued_grant().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original EN 1990 receiver omitted its issued grant"))?;cx.consume_receiving_retained(issued,p)?;result.map_err(Into::into)
  }));
  self.turn+=1;let(actual,p)=measured.expect("original operation entered its physical observation");
  if actual!=(p.retained_capacity_bytes,p.released_bytes)&&self.unpaid.is_none(){self.unpaid=Some(Unpaid{turn:self.turn,actual,reported:p});}
  result.unwrap_or_else(|failure|panic!("original EN 1990 caller refused: {failure:?}"))
 }
 fn retire<T:RetireOwned>(&mut self,value:T){
  let mut value=Some(value);let mut close=RetainedCloneClose::default();
  for _ in 0..100000{if value.is_none()&&close.is_empty(){return;}let d=close.next_owner_demands::<T>(value.is_some(),&None,4096).unwrap();let g=retained(d);
   self.run(g,|control|control.with_retirement(g,|_|if close.is_empty(){close.begin_granted(&mut value,g).result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default())))}else{close.step_granted(g).result}));
  }panic!("original EN 1990 fixture custody did not retire");
 }
 fn finish(mut self)->Option<Unpaid>{
  for _ in 0..100000{if self.decoder.terminal_is_empty(){let unpaid=self.unpaid.take();self.caller.retire(stock());return unpaid;}
   self.caller.turn(stock(),|cx|cx.with_original_retirement(|cx|{let d=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,d,|g|self.decoder.close_step(g))?;Ok(())})).unwrap();
  }panic!("original EN 1990 decoder did not retire");
 }
}
fn texts()->(&'static str,&'static str,&'static str){(
 semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌉️insert-bridge-sls/✅apply/🔣️.before.json"),
 semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌉️insert-bridge-sls/✅apply/🔣️.mutation.json"),
 semio_framework_test_fixtures::repo_text!("✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌉️insert-bridge-sls/✅apply/🔣️.after.json"))}
fn oracle(before:&str,mutation:&str,after:&str,policy:&serde_json::Value){
 use std::io::Write;
 let input=serde_json::json!({"before":serde_json::from_str::<serde_json::Value>(before).unwrap(),"mutation":serde_json::from_str::<serde_json::Value>(mutation).unwrap(),"policy":policy,"schema":serde_json::from_str::<serde_json::Value>(include_str!("🧬️schema/🔣️.json")).unwrap()});
 let mut child=std::process::Command::new("repo").args(["tools","exec","bun","--","-e",r#"import{Database}from'bun:sqlite';import Ajv from'ajv/dist/2020.js';const v=JSON.parse(await Bun.stdin.text());if(!new Ajv({strict:true}).compile(v.schema)(v.policy))throw Error('physical policy schema');const db=new Database(':memory:');db.exec('create table rows(pos integer primary key,payload text)');v.before.bridgeSls.forEach((row,i)=>db.query('insert into rows values(?,?)').run(i*2,JSON.stringify(row)));const at=v.mutation.index??v.before.bridgeSls.length;db.query('insert into rows values(?,?)').run(at*2-1,JSON.stringify(v.mutation.item));v.before.bridgeSls=db.query('select payload from rows order by pos').all().map(r=>JSON.parse(r.payload));console.log(JSON.stringify(v.before));"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
 child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success());assert_eq!(serde_json::from_slice::<En1990Snapshot>(&output.stdout).unwrap(),serde_json::from_str::<En1990Snapshot>(after).unwrap());
}

#[test]
fn en1990_public_factory_pays_nested_projection_and_semantics(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let(before,mutation,after)=texts();oracle(before,mutation,after,&policy);
 let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let bytes=std::hint::black_box(vec![0_u8;73]);std::hint::black_box(&bytes);drop(bytes);});assert_eq!((heap.requested_bytes,heap.released_bytes),(73,73));
 let maximum=policy["maximumTurns"].as_u64().unwrap()as usize;let body=policy["bodyBytes"].as_u64().unwrap()as usize;
 let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:policy["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:policy["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
 for index in policy["operationIndices"].as_array().unwrap(){for prefix in std::iter::once(usize::MAX).chain(policy["cancelAfter"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize)){
  let mut probe=Probe::new();let root=Arc::new(serde_json::from_str::<En1990Snapshot>(before).unwrap());let expected=serde_json::from_str::<En1990Snapshot>(after).unwrap();
  let parsed=semio_framework_pack_json::parse(mutation,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut original=Some(En1990Mutation::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).unwrap());
  let snapshots:Arc<dyn ArtifactOwnedValueRetirementFactory<En1990Snapshot>>=Arc::new(semio_framework_value::OwnedValueRetirementFactory::<En1990Snapshot>::default());
  let mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<En1990Mutation>>=Arc::new(semio_framework_value::OwnedValueRetirementFactory::<En1990Mutation>::default());
  let d=ArtifactMutationSource::<En1990Mutation>::constructor_demand();let(mut source,_)=probe.run(retained(d),|_|ArtifactMutationSource::admit(&mut original,&mutations,retained(d))).unwrap();let mut input=None;
  for _ in 0..maximum{let g=retained(source.demands(body).unwrap());if source.ready(){probe.run(g,|_|source.take(&mut input,g));break;}probe.run(g,|_|source.advance(g));}
  let registry=SnapshotReadRegistryHandle::new();let lease=registry.try_issue(Arc::clone(&root)).unwrap_or_else(|_|panic!("original EN 1990 read lease"));
  let context=ArtifactReplayPreparationContext{mode:ArtifactReplayPreparationMode::Prefix,operation_index:index.as_u64().unwrap()as u32,generation:1,base_revision:[7;32]};
  let request=ArtifactReplayPreparationRequest{base:SnapshotRead::new(Arc::clone(&root),lease),input:input.take().unwrap(),context,snapshots:Arc::clone(&snapshots),mutations:Arc::clone(&mutations)};
  let factory=ColdMutationApplyFactory::<En1990Snapshot,En1990Mutation>::default();let birth=retained(ArtifactReplayPreparationFactory::preparation_birth_demand(&factory));
  let(mut owner,_)=probe.run(birth,|control|{control.admit_turn_capacity(birth.maximum_capacity_bytes)?;control.charge(birth.maximum_capacity_bytes)?;ArtifactReplayPreparationFactory::begin(&factory,request,birth).map_err(|(error,_)|error)});
  let mut prepared=None;
  for turn in 0..maximum{if turn==prefix{break;}let before=owner.advance_demands(body).unwrap();let denied=semio_framework_os_kernel::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:0,maximum_copy_bytes:funded.maximum_copy_bytes,maximum_capacity_bytes:funded.maximum_capacity_bytes,maximum_release_bytes:funded.maximum_release_bytes,maximum_depth:funded.maximum_depth};
   let step=probe.run(funded,|control|owner.advance(body,denied,control));assert_eq!(step.progress(),Default::default());assert_eq!(owner.last_turn_work(),0);assert_eq!(owner.advance_demands(body).unwrap(),before);
   let grant=semio_framework_os_kernel::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:1,..denied};let step=probe.run(funded,|control|owner.advance(body,grant,control));if let ArtifactReplayPreparationStep::Prepared(_)=step{prepared=owner.take_prepared();break;}
  }
  if let Some(prepared)=prepared{assert!(prepared.apply_refusal.is_none()&&prepared.input_refusal.is_none());assert_eq!(prepared.next.as_deref(),Some(&expected));probe.retire(prepared.next);probe.retire(prepared.inverse);probe.retire(prepared.messages);}else{assert_ne!(prefix,usize::MAX,"actual EN 1990 public factory completes");}
  owner.cancel();owner.begin_close();for _ in 0..maximum{if owner.terminal_is_empty(){break;}let g=retained(owner.close_demands(body).unwrap());probe.run(g,|control|control.with_retirement(g,|same|owner.close_step(g,same)));}assert!(owner.terminal_is_empty());drop(owner);
  source.begin_close();for _ in 0..maximum{if source.terminal_is_empty(){break;}let g=retained(source.close_demand(body).unwrap());probe.run(g,|_|source.close_step(g));}assert!(source.terminal_is_empty());drop(source);
  let mut active:Option<Box<dyn ErasedSnapshotRetirement>>=None;
  for _ in 0..maximum{if !registry.has_returned()&&active.is_none(){break;}let d=if let Some(owner)=active.as_ref(){semio_framework_value_snapshot_read::artifact_retirement_box_demands(owner.as_ref(),body).unwrap()}else{registry.returned_admission_demands::<En1990Snapshot>(|_|RetirementDemand{capacity_bytes:semio_framework_value::factory::FactorySharedRetirement::<En1990Snapshot>::constructor_capacity_bytes(),depth:1,..Default::default()}).unwrap()};let g=retained(d);probe.run(g,|_|semio_framework_value_snapshot_read::advance_returned_snapshot_read(&registry,&mut active,&snapshots,g).result);}
  assert!(!registry.has_returned()&&active.is_none());probe.retire(root);probe.retire(semio_framework_value::FactoryAuthority::new(snapshots));probe.retire(semio_framework_value::FactoryAuthority::new(mutations));
  let unpaid=probe.finish();assert_eq!(unpaid.is_none(),policy["expected"]["heapMatchesReceipts"].as_bool().unwrap(),"actual EN 1990 nested heap was not physically funded: {unpaid:?}");
 }}
}
