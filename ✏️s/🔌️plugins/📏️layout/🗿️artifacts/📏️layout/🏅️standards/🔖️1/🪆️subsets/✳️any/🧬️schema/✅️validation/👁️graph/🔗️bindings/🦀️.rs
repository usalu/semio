//! 🔗️ Final semantic preflight holds actual live-post and authored original leases before any borrowed reader continuation exists.
use crate::snapshot::LayoutSnapshot;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
#[derive(Default)]pub struct FinalGraphSourceBindings{live:Option<RetainedCloneBinding>,authored:Option<RetainedCloneBinding>,stage:u8,closing:bool}
impl FinalGraphSourceBindings{
 const OVERHEAD:usize=256;
 fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout final graph original source binding state is invalid")}
 fn nested(d:RetirementDemand)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:d.copy_bytes.checked_add(Self::OVERHEAD).ok_or_else(Self::invalid)?,depth:d.depth.checked_add(1).ok_or_else(Self::invalid)?,..d})}
 fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(Self::OVERHEAD),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
 fn paid(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(Self::OVERHEAD);p}
 fn error(error:ValueError)->ValueError{let p=Self::paid(error.retained_progress());error.with_retained_progress(p)}
 pub fn new()->Self{Default::default()}
 pub fn ready(&self)->bool{self.stage==2&&!self.closing}
 pub fn verify<D>(&self,live_post:RetainedCloneRef<'_,LayoutSnapshot>,authored:RetainedCloneRef<'_,D>)->Result<(),ValueError>{live_post.binding_demand(&self.live)?;authored.binding_demand(&self.authored)?;Ok(())}
 pub fn demand<D>(&self,live_post:RetainedCloneRef<'_,LayoutSnapshot>,authored:RetainedCloneRef<'_,D>,_:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(Self::invalid())}self.verify(live_post,authored)?;match self.stage{0=>Self::nested(live_post.binding_demand(&self.live)?.unwrap_or_default()),1=>Self::nested(authored.binding_demand(&self.authored)?.unwrap_or_default()),2=>Ok(Default::default()),_=>Err(Self::invalid())}}
 pub fn advance<D>(&mut self,live_post:RetainedCloneRef<'_,LayoutSnapshot>,authored:RetainedCloneRef<'_,D>,body:usize,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.demand(live_post,authored,body)?;if self.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let p=match self.stage{0=>live_post.bind(&mut self.live,Self::child(g)),1=>authored.bind(&mut self.authored,Self::child(g)),_=>return Err(Self::invalid())}.map_err(Self::error)?.unwrap_or_default();if self.stage==0&&self.live.is_some(){self.stage=1}else if self.stage==1&&self.authored.is_some(){self.stage=2}let p=Self::paid(p);Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})}
 pub fn begin_close(&mut self){self.closing=true}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.terminal_is_empty(){return Ok(Default::default())}Self::nested(RetainedCloneBinding::close_demand(if self.authored.is_some(){&self.authored}else{&self.live},body.saturating_sub(Self::OVERHEAD))?)}
 pub fn close_step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(Self::invalid())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}let held=if self.authored.is_some(){&mut self.authored}else{&mut self.live};let p=RetainedCloneBinding::close_one(held,Self::child(g)).map_err(Self::error)?.progress();if self.live.is_none(){self.stage=0}Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(Self::paid(p))}else{RetainedCloneStep::Progress(Self::paid(p))})}
 pub fn terminal_is_empty(&self)->bool{self.live.is_none()&&self.authored.is_none()}
}
impl Drop for FinalGraphSourceBindings{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Final graph original leases require funded close")}}
#[cfg(test)]#[path="🧪️tests.rs"]mod tests;
