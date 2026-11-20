use crate::post_fixture::PostFixture;
use semio_s_artifact_layout_layout_diff::retained::collection::RetainedCollection;
use std::mem::size_of;
use semio_framework_value::{OriginalRetirementCloseLoan,RetainedAllocationEffect,RetirementDemand,RetirementPollExit,ValueError,OriginalRetirementPoll,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_os_kernel::ApplyCapability;
use crate::{PageLayersDelta,PageLayersModification,PageLayerRemoval,PageLayerInsertion,PageLayerRelocation,LayerPatch};
use semio_s_artifact_layout_layout_model::Layer;
use semio_os_kernel as protocol;
use semio_s_artifact_layout_layout_diff::retained::setters::OwnedPatch;
use protocol::{RetainedDiffApply,RetainedDiffPreflightAdvance,MutationMessage};
use semio_framework_value::{NativeDecodeControl,OriginalRetirementFailure,OriginalRetirementOutcome,RetainedWorkPoll};
use semio_framework_value_clone_source_testing::{FixtureSource,drive::{drive,drive_reply}};
use protocol::{RetainedDiffOwner,RetainedDiffPreflightStep,RetainedReply as Reply,RetainedDone as Done,retained_conclude,retained_settle};
const UNITS:u64=1<<20;
#[derive(Clone,Default,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue,semio_framework_value::RetireOwned,semio_framework_value::RetainedClone)]
struct TestDiff{delta:PageLayersDelta}
impl protocol::MutationDiff<Vec<Layer>> for TestDiff{}
impl protocol::RetainedDiff<Vec<Layer>> for TestDiff{type Apply=TestApply;}
struct TestApply{inner:RetainedCollection<Layer,PageLayersDelta>,membership:bool,index:usize,field:usize}
fn selected(d:RetainedCloneRef<'_,TestDiff>)->RetainedCloneRef<'_,PageLayersDelta>{d.project(0,|diff|&diff.delta)}
impl RetainedDiffApply<Vec<Layer>,TestDiff> for TestApply{
 fn new()->Self{Self{inner:RetainedCollection::new(),membership:false,index:0,field:0}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,TestDiff>,_:RetainedCloneRef<'_,Vec<Layer>>,post:RetainedCloneRef<'_,Vec<Layer>>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
    self.inner.preflight_demand(selected(d),post.get(),body,maximum)
 }
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,TestDiff>,_:RetainedCloneRef<'_,Vec<Layer>>,post:RetainedCloneRef<'_,Vec<Layer>>,body:usize,maximum:u64)->Result<u64,ValueError>{
    self.inner.preflight_work_demand(selected(d),post.get(),body,maximum)
 }
 fn preflight_reply<'r>(&mut self,d:RetainedCloneRef<'_,TestDiff>,_:RetainedCloneRef<'_,Vec<Layer>>,post:RetainedCloneRef<'_,Vec<Layer>>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,receiving:&mut NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{
    let base=effect.performed_work();
    let mut work=0;
    let result=(||{
        receiving.checkpoint()?;
        let step=self.inner.preflight(selected(d),post.get(),body,maximum,effect,recover,&mut work)?;
        assert!(step.error.is_none(),"positive collection witness must admit");
        Ok(if step.done{RetainedDiffPreflightAdvance::Ready(step.progress,true)}else{RetainedDiffPreflightAdvance::Progress(step.progress)})
    })();
    retained_conclude(effect,base,result,work,RetainedDiffPreflightAdvance::progress,reply)
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,_:RetainedCloneRef<'_,TestDiff>,_:RetainedCloneRef<'_,Vec<Layer>>,_:&Vec<Layer>,_:usize)->Result<RetirementDemand,ValueError>{
    if !self.membership{return self.inner.membership_commit_demand()}
    if self.index==self.inner.modification_targets().unwrap().len(){return Ok(RetirementDemand{copy_bytes:128,depth:1,..Default::default()})}
    self.inner.staged().unwrap().modified[self.index].patch.field_demand(self.field)
 }
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,TestDiff>,b:RetainedCloneRef<'_,Vec<Layer>>,post:&Vec<Layer>,body:usize)->Result<u64,ValueError>{
    self.commit_demand(d,b,post,body)?;
    Ok(1)
 }
 fn commit(&mut self,d:RetainedCloneRef<'_,TestDiff>,b:RetainedCloneRef<'_,Vec<Layer>>,post:&mut Vec<Layer>,effect:&mut RetainedAllocationEffect<'_>,capability:ApplyCapability)->OriginalRetirementOutcome{
    let base=effect.performed_work();
    let result=(||{
        if !self.membership{
            let step=self.inner.commit_membership(post,effect,capability)?;
            if matches!(step,RetainedCloneStep::Complete(_)){self.membership=true}
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let grant=effect.grant();
        let demand=self.commit_demand(d,b,post,grant.maximum_copy_bytes)?;
        if !protocol::retained_mutation_demand_fits(demand,grant)||effect.remaining_work_units()?==0{
            return Ok(RetainedCloneStep::Progress(Default::default()))
        }
        if !effect.performed(1)?{return Err(ValueError::invariant("collection law exceeded admitted commit work"))}
        let targets=self.inner.modification_targets().unwrap();
        if self.index==targets.len(){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:128,..Default::default()}))}
        let target=targets[self.index];
        let patch=&mut self.inner.staged_mut(capability)?.modified[self.index].patch;
        let step=patch.exchange_field(&mut post[target],self.field,grant,capability)?;
        self.field+=1;
        if self.field==<LayerPatch as OwnedPatch<Layer>>::FIELD_COUNT{self.field=0;self.index+=1}
        Ok(RetainedCloneStep::Progress(step.progress()))
    })().map_err(OriginalRetirementFailure::from);
    let work=effect.performed_work().saturating_sub(base);
    retained_settle(effect,base,result,work)
 }
 fn begin_close(&mut self){self.inner.begin_close()}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.inner.close_demand(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.inner.close_work_demand(body)}
 fn requires_original_birth(&self)->bool{self.inner.requires_original_birth()}
 fn requires_original_control(&self)->bool{self.inner.requires_original_control()}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    self.inner.close_original_birth_step(original,grant)
 }
 fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
    let base=effect.performed_work();
    let mut work=0;
    let result=self.inner.close_step(original,body,effect,recover,&mut work);
    retained_conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn terminal_is_empty(&self)->bool{self.inner.terminal_is_empty()}
}
fn grant(d:RetirementDemand)->RetainedCloneGrant{
    RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}
}
fn preflight(owner:&mut RetainedDiffOwner<Vec<Layer>,TestDiff>,diff:RetainedCloneRef<'_,TestDiff>,original:RetainedCloneRef<'_,Vec<Layer>>,post:RetainedCloneRef<'_,Vec<Layer>>,body:usize,grant:RetainedCloneGrant,receiving:&mut NativeDecodeControl<'_>)->RetainedDiffPreflightStep{
    drive_reply(grant,UNITS,|answer|answer.as_ref().map_or_else(OriginalRetirementFailure::retained_progress,|step:&RetainedDiffPreflightStep|step.progress()),
        |effect,reply,recover|{
            if recover{protocol::recover_preflight_retained_diff_into(owner,diff,original,post,body,UNITS,effect,receiving,reply)}
            else{protocol::preflight_retained_diff_into(owner,diff,original,post,body,UNITS,effect,receiving,reply)}
        },
    ).result.unwrap()
}
fn commit(owner:&mut RetainedDiffOwner<Vec<Layer>,TestDiff>,diff:RetainedCloneRef<'_,TestDiff>,original:RetainedCloneRef<'_,Vec<Layer>>,post:&mut PostFixture<Vec<Layer>>,grant:RetainedCloneGrant)->RetainedCloneStep{
    drive(grant,UNITS,|effect|RetainedWorkPoll::Complete(protocol::commit_retained_diff(owner,diff,original,&mut *post,effect))).result.unwrap()
}
fn close(owner:&mut RetainedDiffOwner<Vec<Layer>,TestDiff>){
    owner.begin_close();
    for _ in 0..100_000{
        if owner.terminal_is_empty(){return}
        let demand=owner.close_demand(4096).unwrap();
        let mut close=|grant|{
            drive_reply(grant,UNITS,|answer|answer.as_ref().map_or_else(OriginalRetirementFailure::retained_progress,|step:&RetainedCloneStep|step.progress()),
                |effect,reply,recover|if recover{owner.recover_close_into(4096,effect,reply)}else{owner.close_into(4096,effect,reply)},
            ).result.unwrap()
        };
        assert_eq!(close(Default::default()).progress(),Default::default());
        close(grant(demand));
    }
    panic!("real collection central close stalled")
}
fn layer(id:&str)->Layer{Layer{id:id.into(),name:format!("name:{id}"),visible:true,locked:false,object_ids:vec![format!("untouched:{id}")]}}
fn input(row:&serde_json::Value)->(Vec<Layer>,TestDiff){let held=row["held"].as_array().unwrap().iter().map(|id|layer(id.as_str().unwrap())).collect();let d=&row["delta"];let delta=PageLayersDelta{removed:d["removed"].as_array().unwrap().iter().map(|row|PageLayerRemoval{id:row["id"].as_str().unwrap().into(),index:row["index"].as_u64().unwrap() as usize}).collect(),inserted:d["inserted"].as_array().unwrap().iter().map(|row|PageLayerInsertion{index:row["index"].as_u64().unwrap() as usize,row:layer(row["id"].as_str().unwrap())}).collect(),moved:d["moved"].as_array().unwrap().iter().map(|row|PageLayerRelocation{id:row["id"].as_str().unwrap().into(),from:row["from"].as_u64().unwrap() as usize,to:row["to"].as_u64().unwrap() as usize}).collect(),modified:d["modified"].as_array().unwrap().iter().enumerate().map(|(i,row)|PageLayersModification{id:row["id"].as_str().unwrap().into(),patch:LayerPatch{name:Some(format!("authored:{i}")),..Default::default()}}).collect()};(held,TestDiff{delta})}
#[test]
fn genuine_central_sparse_collection_keeps_sources_and_untouched_live_allocations(){let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🧳️retained/📋️keyed-plan/🧪️fixtures/🔣️.json")).unwrap();for row in corpus["cases"].as_array().unwrap(){if row.get("error").is_some(){continue}for body in[1024,4096]{let(post,event)=input(row);let mut post=PostFixture::new(post);let pointers:Vec<_>=post.iter().map(|row|row.object_ids[0].as_ptr()).collect();let original=post.clone();let base=FixtureSource::from_owner(original.clone());let event_copy=event.clone();let diff=FixtureSource::from_owner(event);let(mut owner,_)=protocol::begin_retained_diff::<Vec<Layer>,TestDiff>(grant(protocol::retained_diff_birth_demand::<Vec<Layer>,TestDiff>())).unwrap();let mut callback=|_|true;let mut receiving=semio_framework_value::NativeDecodeControl::new(1<<30,&mut callback);for turn in 0..100000{let d=owner.preflight_demand(diff.borrow(),base.borrow(),post.borrow(),body,UNITS).unwrap();assert_eq!(preflight(&mut owner,diff.borrow(),base.borrow(),post.borrow(),body,Default::default(),&mut receiving).progress(),Default::default());let step=preflight(&mut owner,diff.borrow(),base.borrow(),post.borrow(),body,grant(d),&mut receiving);assert!(step.progress().fits(grant(d)));assert_eq!(post,original);if matches!(step,protocol::RetainedDiffPreflightStep::Admitted(..)){break}assert!(turn<99999)}for turn in 0..100000{let d=owner.commit_demand(diff.borrow(),base.borrow(),&post,body).unwrap();assert_eq!(commit(&mut owner,diff.borrow(),base.borrow(),&mut post,Default::default()).progress(),Default::default());let step=commit(&mut owner,diff.borrow(),base.borrow(),&mut post,grant(d));assert!(step.progress().fits(grant(d)));if matches!(step,RetainedCloneStep::Complete(_)){break}assert!(turn<99999)}for(at,origin)in post.iter().zip(row["origins"].as_array().unwrap()){let expected=if origin[0]=="held"{row["held"][origin[1].as_u64().unwrap() as usize].as_str().unwrap()}else{row["delta"]["inserted"][origin[1].as_u64().unwrap() as usize]["id"].as_str().unwrap()};assert_eq!(at.id,expected);if origin[0]=="held"{assert_eq!(at.object_ids[0].as_ptr(),pointers[origin[1].as_u64().unwrap() as usize])}}for(index,target)in row["modTargets"].as_array().unwrap().iter().enumerate(){let target=target.as_u64().unwrap() as usize;if !row["modTargets"].as_array().unwrap()[index+1..].contains(&serde_json::json!(target)){assert_eq!(post[target].name,format!("authored:{index}"))}}assert_eq!(base.borrow().get(),&original);assert_eq!(diff.borrow().get().delta,event_copy.delta);close(&mut owner);drop(diff);drop(base);drop(FixtureSource::from_owner(post.into_inner()));}}}
