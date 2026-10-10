//! 📸️ Exact Flow snapshot and local scene sources retain their defining paid close frontiers.
use crate::{FlowSnapshot,FlowWorkingScene};
use std::{mem::ManuallyDrop,sync::Arc};
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement,shared::SharedControlledRetirement}};

#[derive(semio_framework_value::RetireOwned)]
struct FlowSnapshotSources{schema:String,child_id:String,target:semio_framework_artifact_reference::ArtifactRef,scene:Option<SharedControlledRetirement<FlowWorkingScene>>}

impl RetireOwned for FlowWorkingScene{
 fn retirement(self)->Box<dyn RetirementCursor>{let(widgets,synapses,layout)=self.into_parts();semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(widgets),semio_framework_value::retirement::deferred(synapses),semio_framework_value::retirement::deferred(layout)])}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.widgets),semio_framework_value::retirement::deferred_birth_bytes_for(&self.synapses),semio_framework_value::retirement::deferred_birth_bytes_for(&self.layout)])}
 fn controlled_retirement_supported()->bool{true}
}

struct FlowSnapshotRetirement{source:ManuallyDrop<Option<FlowSnapshot>>,owners:ManuallyDrop<Option<ControlledRetirement<FlowSnapshotSources>>>}
impl FlowSnapshotRetirement{
 fn new(source:FlowSnapshot)->Self{Self{source:ManuallyDrop::new(Some(source)),owners:ManuallyDrop::new(None)}}
 fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.owners.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()})}return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original Flow snapshot depth overflow"))?})}Ok(RetirementDemand{depth:usize::from(self.source.is_some()),..Default::default()})}
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let empty=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}let d=self.demands(grant.maximum_copy_bytes)?;if grant.maximum_items==0||grant.maximum_copy_bytes<d.copy_bytes||grant.maximum_capacity_bytes<d.capacity_bytes||grant.maximum_release_bytes<d.release_bytes||grant.maximum_depth<d.depth{return Ok(RetainedCloneStep::Progress(empty))}
  if let Some(owner)=self.owners.as_mut(){if owner.terminal_is_empty(){self.owners.take();return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}return owner.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()))}
  let FlowSnapshot{schema,content}=self.source.take().unwrap();match content.into_typed_retained_parts::<FlowWorkingScene>(){Ok(parts)=>{*self.owners=Some(ControlledRetirement::new(FlowSnapshotSources{schema,child_id:parts.child_id,target:parts.target,scene:parts.local_owner.map(SharedControlledRetirement::lease)}).unwrap_or_else(|_|unreachable!("original Flow snapshot sources define full closure")));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))},Err(content)=>{*self.source=Some(FlowSnapshot{schema,content});Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original Flow snapshot local root has a different concrete type"))}}
 }
}
impl RetirementCursor for FlowSnapshotRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(p))if p==Default::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(p)|RetainedCloneStep::Complete(p))=>RetirementStep::Progress(p)}}
 fn terminal_is_empty(&self)->bool{self.source.is_none()&&self.owners.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.demands(copy).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demands(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl RetireOwned for FlowSnapshot{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(FlowSnapshotRetirement::new(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<FlowSnapshotRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl Drop for FlowSnapshotRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original Flow snapshot abandoned local root custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.owners);}}}}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct SnapshotRetirementFactory;
impl store::SnapshotRetirementFactory<FlowSnapshot> for SnapshotRetirementFactory{
 fn retirement_birth_bytes(&self,_source:&Arc<FlowSnapshot>)->usize{semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<FlowSnapshot>()}
 fn retire(&self,source:Arc<FlowSnapshot>,grant:RetainedCloneGrant)->Result<(Box<dyn store::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<FlowSnapshot>)>{semio_framework_value::retirement::shared::admit_shared_retirement(source,grant,true)}
}
impl store::ArtifactOwnedValueRetirementFactory<FlowSnapshot> for SnapshotRetirementFactory{
 fn retirement_birth_bytes(&self,_source:&FlowSnapshot)->usize{semio_framework_value::retirement::controlled::controlled_retirement_birth_bytes::<FlowSnapshot>()}
 fn retire_owned(&self,source:FlowSnapshot,grant:RetainedCloneGrant)->Result<(Box<dyn store::ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,FlowSnapshot)>{semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(source,grant).map(|(owner,p)|(owner as Box<dyn store::ErasedSnapshotRetirement>,p))}
}
