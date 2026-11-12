//! 🖼️ Original evaluation scenes retain one immutable child root and one outgoing owner.
use super::super::*;
use semio_plugin::ArtifactView;
use crate::retained::{close_with_effect,poll_outcome,begin_close_owner};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneClose,retained_clone_grant_admits};
use semio_framework_job::{OriginalSourceBinding,OriginalTurnFailure,StepContext};
use std::mem::{size_of,ManuallyDrop};
type Scene=crate::editor::flow::host::FlowComposedScene;
type Children=semio_plugin::ChildContentView;
type Context<A>=semio_plugin::app::ArtifactOwnedToolJobContext<semio_plugin::EditorApp<A>>;
#[derive(Clone,Copy,PartialEq,Eq)]
struct Key{instance:u32,revision:[u8;32],generation:u64}
impl Key{
 fn matches(self,children:&Children,other:Self,other_children:&Children)->bool{self==other&&children.same_owner(other_children)}
 fn original<A:crate::FlowApp>(context:&Context<A>)->Self{Self{instance:context.app_instance_id,revision:context.canonical_base_revision,generation:context.provisional_generation()}}
 fn view(doc:&ArtifactView<'_,FlowSnapshot>)->Option<Self>{let operation=doc.render_operation_ref()?;Some(Self{instance:operation.app_instance_id,revision:operation.canonical_base_revision,generation:doc.provisional_generation()})}
}
struct Owners{scene:Option<Scene>,children:Option<Children>,outgoing_scene:Option<Scene>,outgoing_children:Option<Children>}
pub(super) struct FlowSceneCache{owners:ManuallyDrop<Owners>,retirement:RetainedCloneClose,key:Option<Key>,instance:Option<u32>,closing:bool}
impl FlowSceneCache{
 pub(super) fn new()->Self{Self{owners:ManuallyDrop::new(Owners{scene:None,children:None,outgoing_scene:None,outgoing_children:None}),retirement:Default::default(),key:None,instance:None,closing:false}}
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original Flow scene cache source or ownership changed")}
 fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
 pub(super) fn publish_demands()->RetirementDemand{Self::fixed(2*size_of::<Owners>()+2*size_of::<Key>()+2*size_of::<Children>())}
 pub(super) fn publish_original<A:crate::FlowApp>(&mut self,context:&Context<A>,binding:OriginalSourceBinding,scene:&mut Option<Scene>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing||!context.provisional().is_empty()||!binding.matches(cx)||self.instance.is_some_and(|instance|instance!=context.app_instance_id)||scene.is_none(){return Err(Self::invalid().into())}
  if self.pending()||cx.should_yield()||cx.is_cancelled(){return Ok(false)}
  let demand=Self::publish_demands();if !retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(false)}
  self.owners.outgoing_scene=self.owners.scene.take();self.owners.outgoing_children=self.owners.children.take();
  self.owners.children=Some(context.children.as_ref().clone());self.owners.scene=scene.take();self.key=Some(Key::original(context));self.instance=Some(context.app_instance_id);
  cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
 }
 pub(super) fn read<'a>(&'a self,doc:&ArtifactView<'_,FlowSnapshot>)->Option<&'a semio_framework_artifact_flow_flow::FlowHostSnapshot>{
  if self.closing||!self.key?.matches(self.owners.children.as_ref()?,Key::view(doc)?,&doc.children){return None}
  self.owners.scene.as_ref().map(Scene::source)
 }
 pub(super) fn pending(&self)->bool{!self.retirement.is_empty()||self.owners.outgoing_scene.is_some()||self.owners.outgoing_children.is_some()}
 pub(super) fn begin_close(&mut self){self.closing=true}
 pub(super) fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
  if !self.retirement.is_empty(){return self.retirement.demands(copy)}
  macro_rules! owner{($field:ident,$ty:ty)=>{if self.owners.$field.is_some(){return self.retirement.next_owner_demands::<$ty>(true,&None,copy)}}}
  owner!(outgoing_scene,Scene);owner!(outgoing_children,Children);
  if self.closing{owner!(scene,Scene);owner!(children,Children);if self.key.is_some()||self.instance.is_some(){return Ok(Self::fixed(size_of::<Option<Key>>()+size_of::<Option<u32>>()))}}
  Ok(Default::default())
 }
 /// 🧹️ Funds one close step of the outgoing owners inside the lifecycle turn's own effect.
 pub(super) fn advance(&mut self,cx:&mut StepContext<'_>)->Result<(),OriginalTurnFailure>{
  let d=self.demands(cx.retained_grant().maximum_copy_bytes)?;cx.with_original_retirement(|cx|close_with_effect(cx,d,|g,effect|self.advance_granted(g,effect)))?;Ok(())
 }
 fn advance_granted(&mut self,g:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome{
  if !self.retirement.is_empty(){return poll_outcome(self.retirement.step_granted(g.maximum_copy_bytes,effect))}
  macro_rules! owner{($field:ident)=>{if self.owners.$field.is_some(){return begin_close_owner(&mut self.retirement,&mut self.owners.$field,effect)}}}
  owner!(outgoing_scene);owner!(outgoing_children);
  if self.closing{owner!(scene);owner!(children);if self.key.is_some()||self.instance.is_some(){let demand=Self::fixed(size_of::<Option<Key>>()+size_of::<Option<u32>>());if !retained_clone_grant_admits(g,demand){return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.key=None;self.instance=None;return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})),work:1}}}
  RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}
 }
 pub(super) fn terminal_is_empty(&self)->bool{self.closing&&!self.pending()&&self.owners.scene.is_none()&&self.owners.children.is_none()&&self.key.is_none()&&self.instance.is_none()}
}
impl Drop for FlowSceneCache{fn drop(&mut self){assert!(self.terminal_is_empty(),"Flow cache must retire its original scene and child root");unsafe{ManuallyDrop::drop(&mut self.owners)}}}

#[cfg(test)]
#[path="🧪️.rs"]mod laws;
