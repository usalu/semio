//! 🪞️ A transparent newtype forwards every turn to its inner field's cursor and only moves the finished inner value into its wrapper.
use crate::receiving::owned::*;
use crate::FieldValue;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,retirement::RetireOwned};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
/// 🧩️ Validation borrows the finished inner value and wrapping is one pure constructor, so a refusal leaves the inner value in its paid retirement slot.
pub trait TransparentField:RetireOwned+Sized+Send+'static{type Inner:DslOwnedReceivingField;fn check(_:&Self::Inner)->Result<(),ValueError>{Ok(())}fn wrap(inner:Self::Inner)->Self;}
pub struct TransparentReceiving<W:TransparentField>{inner:OwnedSlot<W::Inner>,output:OwnedSlot<W>,child:OwnedReceivingSlot<W::Inner>,phase:u8,closing:bool,failed:bool}
impl<W:TransparentField>Default for TransparentReceiving<W>{fn default()->Self{Self{inner:Default::default(),output:Default::default(),child:OwnedReceivingSlot::new(),phase:0,closing:false,failed:false}}}
impl<W:TransparentField>TransparentReceiving<W>{
 fn selected(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  if self.failed||self.closing{return RetainedWorkOutcome{result:Err(invariant().into()),work:0}}
  if self.phase==4{return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  let d=match self.demands(source,body){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e.into()),work:0}};
  if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}
  match self.phase{
   0 if !self.child.is_ready()=>return self.child.step(source,native,body,child_grant(g)),
   1 if self.inner.value.is_none()=>return normal(self.child.take_into(&mut self.inner.value,child_grant(g))),
   2 if !self.child.terminal_is_empty()=>return post_child_close(self.child.close_step(child_grant(g)),native,g),
   _=>{}
  }
  if let Err(e)=native.checkpoint(){return RetainedWorkOutcome{result:Err(e.into()),work:0}}
  let result=(||{match self.phase{
   0=>self.phase=1,
   1=>self.phase=2,
   2=>self.phase=3,
   3=>{W::check(self.inner.value.as_ref().ok_or_else(invariant)?)?;let inner=self.inner.value.take().ok_or_else(invariant)?;*self.output.value=Some(W::wrap(inner));self.phase=4;},
   _=>return Err(invariant())}Ok(receipt(d))})();
  RetainedWorkOutcome{result:result.map_err(|error:ValueError|OriginalReceivingPrimary::from(error.with_retained_progress(receipt(metadata::<()>())))),work:1}
 }
}
impl<W:TransparentField>OwnedFieldReceiving<W>for TransparentReceiving<W>{
 fn demands(&self,source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.failed||self.closing{return Err(invariant())}match self.phase{0 if !self.child.is_ready()=>self.child.demands(source,body).and_then(nested),1 if self.inner.value.is_none()=>nested(nested(metadata::<()>())?),2 if !self.child.terminal_is_empty()=>self.child.close_demands(body).and_then(nested),4=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.selected(source,native,body,g);if turn.result.is_err()&&turn.work>0{self.failed=true}turn}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==4&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<W>,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if !self.is_ready(){return RetainedWorkOutcome{result:Err(invariant()),work:0}}move_into(&mut self.output.value,output,g)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.child.terminal_is_empty(){return self.child.close_demands(body).and_then(nested)}if !self.inner.terminal_is_empty(){return self.inner.close_demands(body)}self.output.close_demands(body)}
 fn close_step(&mut self,g:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetainedWorkOutcome{result:Err(e),work:0}};if !funds(g,d){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}if !self.closing{self.closing=true;return RetainedWorkOutcome{result:Ok(receipt(d)),work:1}}if !self.child.terminal_is_empty(){return self.child.close_step(child_grant(g))}if !self.inner.terminal_is_empty(){return self.inner.close_step(g)}self.output.close_step(g)}
 fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.inner.terminal_is_empty()&&self.output.terminal_is_empty()}
}
/// 🧬️ One declaration gives a transparent newtype over a cursor-received field both receiving families.
#[macro_export]
macro_rules! dsl_transparent_receiving{

 ($name:ty,$inner:ty,$check:expr,$wrap:expr)=>{
  impl $crate::transparent::TransparentField for $name{type Inner=$inner;fn check(inner:&$inner)->Result<(),$crate::receiving::owned::ValueError>{let check:fn(&$inner)->Result<(),$crate::receiving::owned::ValueError>=$check;check(inner)}fn wrap(inner:$inner)->Self{let wrap:fn($inner)->Self=$wrap;wrap(inner)}}
  impl $crate::receiving::owned::DslOwnedReceivingField for $name{type Cursor=$crate::transparent::TransparentReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl $crate::DslReceivingField for $name{fn receive_value(value:&$crate::FieldValue,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::owned::RetainedWorkOutcome<Self,$crate::receiving::NativeSnapshotFailure>{let outcome=<$inner as $crate::DslReceivingField>::receive_value(value,owner);let work=outcome.work;match outcome.result{Ok(inner)=>match <$name as $crate::transparent::TransparentField>::check(&inner){Ok(())=>$crate::receiving::owned::RetainedWorkOutcome{result:Ok(<$name as $crate::transparent::TransparentField>::wrap(inner)),work},Err(error)=>$crate::receiving::owned::RetainedWorkOutcome{result:Err(error.into()),work}},Err(error)=>$crate::receiving::owned::RetainedWorkOutcome{result:Err(error),work}}}}
 };
 ($name:ty,$inner:ty)=>{
  impl $crate::transparent::TransparentField for $name{type Inner=$inner;fn wrap(inner:$inner)->Self{Self(inner)}}
  impl $crate::receiving::owned::DslOwnedReceivingField for $name{type Cursor=$crate::transparent::TransparentReceiving<Self>;fn receiving_cursor()->Self::Cursor{::core::default::Default::default()}}
  impl $crate::DslReceivingField for $name{fn receive_value(value:&$crate::FieldValue,owner:&mut $crate::receiving::NativeSnapshotDecodeOwner<'_,'_>)->$crate::receiving::owned::RetainedWorkOutcome<Self,$crate::receiving::NativeSnapshotFailure>{$crate::receiving::owned::map_outcome(<$inner as $crate::DslReceivingField>::receive_value(value,owner),Self)}}
 };
}
