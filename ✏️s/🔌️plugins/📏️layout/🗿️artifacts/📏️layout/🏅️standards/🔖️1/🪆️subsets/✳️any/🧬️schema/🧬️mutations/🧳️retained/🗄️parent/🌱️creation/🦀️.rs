//! 🌱️ Original parent payloads prepare derived cache owners before sparse insertion.
use crate::{LayoutMutationSource,LayoutMutationView as LayoutMutation};
use semio_s_artifact_layout_layout_model::{ParentPage,snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_diff::{LayoutDiff,LayoutParentPagesDelta,LayoutParentPageInsertion,retained::apply::page_cache::{PageCachePreparation,PageLayerCacheSource}};
use crate::original_field::close_binding;
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use std::mem::size_of;
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original parent creation output custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:1024+size_of::<LayoutParentPageInsertion>()*2+size_of::<ParentPage>()*2,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=fixed().copy_bytes;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(fixed().copy_bytes),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=fixed().copy_bytes;p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn parent(m:&impl LayoutMutationSource)->Result<&ParentPage,ValueError>{if let LayoutMutation::CreateParentPage(p)=m.view(){Ok(&p.page)}else{Err(fault())}}
struct Source<'a>(&'a ParentPage);
impl PageLayerCacheSource for Source<'_>{fn identity(&self)->usize{721}fn layer_count(&self)->usize{self.0.layers.len()}fn layer_id(&self,i:usize)->&str{&self.0.layers[i].id}fn frame_count(&self)->usize{self.0.frames.len()}fn frame_id(&self,i:usize)->&str{self.0.frames[i].id()}fn frame_layer_id(&self,i:usize)->&str{self.0.frames[i].layer_id()}}
fn vacant()->ParentPage{ParentPage{id:String::new(),name:String::new(),width:0.0,height:0.0,layer_ids:Vec::new(),layers:Vec::new(),frames:Vec::new()}}
/// 🧳 Cache strings and original aliases stay owned here; partial diff and authored payload owners stay with the caller.
pub struct ParentCreationOutput{m:Option<RetainedCloneBinding>,b:Option<RetainedCloneBinding>,cache:PageCachePreparation,phase:u8,header:usize,closing:bool}
impl Default for ParentCreationOutput{fn default()->Self{Self{m:None,b:None,cache:Default::default(),phase:0,header:0,closing:false}}}
impl ParentCreationOutput{
 pub fn ready(&self)->bool{self.phase==4&&!self.closing}
 pub fn demand<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,_body:usize)->Result<RetirementDemand,ValueError>{if self.closing{return Err(fault())}let proposed=parent(m.get())?;let md=m.binding_demand(&self.m)?;let bd=b.binding_demand(&self.b)?;Ok(if self.m.is_none(){nested(md.unwrap_or_default())}else if self.b.is_none(){nested(bd.unwrap_or_default())}else{match self.phase{0=>nested(self.cache.demand(&Source(proposed))?),1=>RetirementDemand{capacity_bytes:size_of::<LayoutParentPageInsertion>(),..fixed()},2=>nested(self.cache.install_demand()?),3|4=>fixed(),_=>return Err(fault())}})}
 pub fn work_demand<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,_body:usize,_maximum:u64)->Result<u64,ValueError>{
    if self.m.is_none(){return m.binding_work_demand(&self.m)}
    if self.b.is_none(){return b.binding_work_demand(&self.b)}
    if self.phase==0{return self.cache.work_demand(&Source(parent(m.get())?))}
    Ok(1)
 }
 pub fn advance<M:LayoutMutationSource>(&mut self,prepared:&mut Vec<M>,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,d:&mut LayoutDiff,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{let g=effect.grant();if recover{return Err(fault().into())}let demand=self.demand(m,b,body)?;if !retained_clone_grant_admits(g,demand)||self.work_demand(m,b,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.m.is_none(){return Ok(RetainedCloneStep::Progress(receipt(m.bind(&mut self.m,&mut effect.nested(child(g))).result.map_err(refused)?.unwrap_or_default())))}if self.b.is_none(){return Ok(RetainedCloneStep::Progress(receipt(b.bind(&mut self.b,&mut effect.nested(child(g))).result.map_err(refused)?.unwrap_or_default())))}if self.phase!=0&&!effect.performed(1)?{return Err(fault().into())}let mut paid=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};if matches!(self.phase,2|3)&&!d.parent_pages.as_ref().is_some_and(|delta|delta.inserted.is_empty()&&delta.inserted.capacity()>=1&&delta.inserted.as_ptr()as usize==self.header){return Err(fault().with_retained_progress(paid).into())}match self.phase{0=>{let step=self.cache.advance(&Source(parent(m.get())?),&mut effect.nested(child(g))).map_err(refused)?;paid=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1}},1=>{if d.parent_pages.is_some(){return Err(fault().with_retained_progress(paid).into())}if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);self.phase=254;d.parent_pages=Some(LayoutParentPagesDelta::default());let rows=&mut d.parent_pages.as_mut().unwrap().inserted;let result=rows.try_reserve_exact(1);paid.retained_capacity_bytes=rows.capacity()*size_of::<LayoutParentPageInsertion>();*effect.actual()=Some(paid.retained_capacity_bytes);if result.is_err()||paid.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(paid).into())}self.header=rows.as_ptr()as usize;self.phase=2},2=>{let Some(p)=prepared.last_mut().and_then(LayoutMutationSource::parent_creation_mut)else{return Err(fault().with_retained_progress(paid).into())};let step=self.cache.install_prepared_parent(&mut p.page,child(g)).map_err(refused)?;paid=receipt(step.progress());if matches!(step,RetainedCloneStep::Complete(_)){self.phase=3}},3=>{let Some(p)=prepared.last_mut().and_then(LayoutMutationSource::parent_creation_mut)else{return Err(fault().with_retained_progress(paid).into())};let page=std::mem::replace(&mut p.page,vacant());d.parent_pages.as_mut().unwrap().inserted.push(LayoutParentPageInsertion{index:p.index.unwrap_or(b.get().parent_pages.len()),row:page});self.phase=4},4=>{},_=>return Err(fault().into())}Ok(if self.ready(){RetainedCloneStep::Complete(paid)}else{RetainedCloneStep::Progress(paid)})}
 pub fn begin_close(&mut self){self.closing=true;self.cache.begin_close()}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(fault())}if !self.cache.terminal_is_empty(){return Ok(nested(self.cache.close_demand(body.saturating_sub(fixed().copy_bytes))?))}if self.m.is_some(){return Ok(nested(RetainedCloneBinding::close_demand(&self.m,body.saturating_sub(fixed().copy_bytes))?))}if self.b.is_some(){return Ok(nested(RetainedCloneBinding::close_demand(&self.b,body.saturating_sub(fixed().copy_bytes))?))}Ok(fixed())}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
    let inner=body.saturating_sub(fixed().copy_bytes);
    if !self.cache.terminal_is_empty(){return self.cache.close_work_demand(inner)}
    RetainedCloneBinding::close_work_demand(if self.m.is_some(){&self.m}else{&self.b},inner)
 }
 pub fn requires_original_birth(&self)->bool{
    if !self.cache.terminal_is_empty(){return self.cache.requires_original_birth()}
    RetainedCloneBinding::requires_original_birth(if self.m.is_some(){&self.m}else{&self.b})
 }
 pub fn requires_original_control(&self)->bool{
    if !self.cache.terminal_is_empty(){return self.cache.requires_original_control()}
    RetainedCloneBinding::requires_original_control(if self.m.is_some(){&self.m}else{&self.b})
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.cache.terminal_is_empty(){return self.cache.close_original_birth_step(original,child(grant))}
    RetainedCloneBinding::close_original_birth_step(if self.m.is_some(){&mut self.m}else{&mut self.b},original,child(grant))
 }
 pub fn close_step(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    if !recover&&!retained_clone_grant_admits(g,self.close_demand(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
    if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
    let inner=body.saturating_sub(fixed().copy_bytes);
    let mut effect=effect.nested(child(g));
    let step=if !self.cache.terminal_is_empty(){self.cache.close_step(original,inner,&mut effect,recover,&mut 0)?}
        else{close_binding(if self.m.is_some(){&mut self.m}else{&mut self.b},original,inner,&mut effect)?};
    Ok(RetainedCloneStep::Progress(receipt(step.progress())))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.m.is_none()&&self.b.is_none()&&self.cache.terminal_is_empty()}}
impl Drop for ParentCreationOutput{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original parent output needs funded alias and displaced cache closure")}}
