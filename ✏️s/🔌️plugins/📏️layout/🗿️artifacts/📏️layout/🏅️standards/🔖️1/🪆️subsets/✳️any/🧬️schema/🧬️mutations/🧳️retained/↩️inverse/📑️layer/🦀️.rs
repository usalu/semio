//! 📑️ Layer inverses prepare only original identity/name fields and preserve nondefault flags.
use crate::LayoutMutation;
use super::super::simple::SimpleKind;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_os_kernel::RetainedMutationCloseContext;
use super::super::original_field::{OriginalField,close_rows,rows_work_demand};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_framework_value_factory::vector::FactoryVectorRetirement;
use std::mem::{size_of,ManuallyDrop};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Layout layer inverse original custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<LayoutMutation>()*4+512,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=256;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_bytes+=256;p.copied_items=p.copied_items.max(1);p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn remove(m:&LayoutMutation)->Result<bool,ValueError>{match m{LayoutMutation::CreateLayer(p)=>Ok(p.remove),_=>Err(fault())}}
fn extra(m:&LayoutMutation,b:&LayoutSnapshot,at:[usize;2])->Result<bool,ValueError>{Ok(remove(m)?&&{let layer=&b.pages[at[0]].layers[at[1]];!layer.visible||layer.locked})}
fn source<'a>(index:usize,m:RetainedCloneRef<'a,LayoutMutation>,b:RetainedCloneRef<'a,LayoutSnapshot>,at:[usize;2])->RetainedCloneRef<'a,String>{let field=index%3;if matches!(m.get(),LayoutMutation::CreateLayer(p)if p.remove){let page=b.project(9,|b|&b.pages).project(at[0],move|pages|&pages[at[0]]);if field==0{page.project(0,|p|&p.id)}else{page.project(1,|p|&p.layers).project(at[1],move|layers|&layers[at[1]]).project(field,move|layer|if field==1{&layer.id}else{&layer.name})}}else{m.project(2000+field,move|m|match m{LayoutMutation::CreateLayer(p)=>match field{0=>&p.page_id,1=>&p.id,_=>&p.name},_=>unreachable!("original layer metadata")})}}
/// 🧳 Six independent String preparations cover the exact one/two-event inverse without a Page clone.
pub struct LayerInverse{fields:[OriginalField<String>;6],rows:ManuallyDrop<Option<Vec<LayoutMutation>>>,retirement:Option<FactoryVectorRetirement<LayoutMutation>>,phase:u8,index:usize,closing:bool}
impl Default for LayerInverse{fn default()->Self{Self{fields:std::array::from_fn(|_|Default::default()),rows:ManuallyDrop::new(None),retirement:None,phase:0,index:0,closing:false}}}
impl LayerInverse{
 pub fn demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if self.closing||SimpleKind::of(m.get())!=Some(SimpleKind::CreateLayer){return Err(fault())}Ok(match self.phase{0=>RetirementDemand{capacity_bytes:size_of::<LayoutMutation>()*if extra(m.get(),b.get(),at)?{2}else{1},..fixed()},1|3=>if(self.phase==1&&self.index==3)||(self.phase==3&&(!extra(m.get(),b.get(),at)?||self.index==6))||self.fields[self.index].value().is_some(){fixed()}else{nested(self.fields[self.index].demand(source(self.index,m,b,at),body.saturating_sub(256),maximum.saturating_sub(1))?)},2|4=>if(self.phase==2&&self.index==3)||(self.phase==4&&self.index==6){fixed()}else{let mut d=nested(self.fields[self.index].exchange_demand()?);d.copy_bytes+=size_of::<LayoutMutation>()*2;d},5|6=>fixed(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{
    [OriginalField::<String>::embedded_birth_work()?,OriginalField::<String>::embedded_birth_work()?,OriginalField::<String>::embedded_birth_work()?,OriginalField::<String>::embedded_birth_work()?,OriginalField::<String>::embedded_birth_work()?,OriginalField::<String>::embedded_birth_work()?]
        .into_iter().try_fold(1u64,|work,child|work.checked_add(child).ok_or_else(fault))
 }
 pub fn work_demand(&self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],body:usize,maximum:u64)->Result<u64,ValueError>{
    let limit=maximum.saturating_sub(1);
    let selected=match self.phase{1|3 if !((self.phase==1&&self.index==3)||(self.phase==3&&(!extra(m.get(),b.get(),at)?||self.index==6)))&&self.fields[self.index].value().is_none()=>self.fields[self.index].work_demand(source(self.index,m,b,at),body.saturating_sub(256),limit)?,2|4=>1,_=>0};
    selected.checked_add(1).ok_or_else(fault)
 }
 pub fn advance(&mut self,m:RetainedCloneRef<'_,LayoutMutation>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let g=effect.grant();
    let d=if recover{fixed()}else{self.demand(m,b,at,body,maximum)?};
    if !recover{
        if !retained_clone_grant_admits(g,d)||self.work_demand(m,b,at,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !effect.performed(1)?{return Err(fault().into())}
    }
 let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};match self.phase{0=>{if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);self.phase=254;*self.rows=Some(Vec::new());let n=if extra(m.get(),b.get(),at)?{2}else{1};let rows=self.rows.as_mut().unwrap();let reserved=rows.try_reserve_exact(n);p.retained_capacity_bytes=rows.capacity()*size_of::<LayoutMutation>();*effect.actual()=Some(p.retained_capacity_bytes);if reserved.is_err()||p.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(p).into())}self.phase=1},1|3=>{if self.phase==1&&self.index==3{self.phase=2;self.index=0}else if self.phase==3&&!extra(m.get(),b.get(),at)?{self.phase=6}else if self.phase==3&&self.index==6{self.phase=4;self.index=3}else if self.fields[self.index].value().is_some(){self.index+=1}else{p=receipt(self.fields[self.index].advance(source(self.index,m,b,at),body.saturating_sub(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())}},2|4=>{if self.phase==2&&self.index==3{self.phase=3;self.index=3}else if self.phase==4&&self.index==6{self.phase=5}else{let(value,paid)=self.fields[self.index].take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-256-size_of::<LayoutMutation>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;p=receipt(paid);p.copied_bytes+=size_of::<LayoutMutation>()*2;let value=value.expect("original layer inverse String");if self.phase==2{if self.index==0{self.rows.as_mut().unwrap().push(LayoutMutation::CreateLayer(crate::create_layer::CreateLayer{page_id:value,id:String::new(),name:String::new(),index:remove(m.get())?.then_some(at[1]),remove:!remove(m.get())?}))}else{let LayoutMutation::CreateLayer(row)=&mut self.rows.as_mut().unwrap()[0]else{return Err(fault().into())};if self.index==1{row.id=value}else{row.name=value}}}else if self.index==3{let layer=&b.get().pages[at[0]].layers[at[1]];self.rows.as_mut().unwrap().push(LayoutMutation::UpdateLayer(crate::update_layer::UpdateLayer{page_id:value,layer_id:String::new(),name:String::new(),visible:layer.visible,locked:layer.locked}))}else{let LayoutMutation::UpdateLayer(row)=&mut self.rows.as_mut().unwrap()[1]else{return Err(fault().into())};if self.index==4{row.layer_id=value}else{row.name=value}}self.index+=1}},5=>{self.rows.as_mut().unwrap().swap(0,1);self.phase=6},6=>return Ok(RetainedCloneStep::Complete(p)),_=>return Err(fault().into())}if self.phase==6{Ok(RetainedCloneStep::Complete(p))}else{Ok(RetainedCloneStep::Progress(p))}}
 pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Vec<LayoutMutation>>>()*2,depth:1,..Default::default()}}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<LayoutMutation>>,RetainedCloneProgress),ValueError>{if self.phase!=6||self.closing{return Err(fault())}let d=self.take_demand();if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=7;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;for field in &mut self.fields{field.begin_close()}}
 pub fn close_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<RetirementDemand,ValueError>{for field in &self.fields{if !field.terminal_is_empty(){return field.close_demand(body)}}if let Some(owner)=self.retirement.as_ref(){return if owner.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<FactoryVectorRetirement<LayoutMutation>>>()*2+256,depth:1,..Default::default()})}else{owner.demands(c.mutations,body)}}if self.rows.is_some(){return Ok(FactoryVectorRetirement::<LayoutMutation>::constructor_demand())}Ok(Default::default())}
 pub fn close_work_demand(&self,c:RetainedMutationCloseContext<'_,LayoutMutation>,body:usize)->Result<u64,ValueError>{
    for field in &self.fields{if !field.terminal_is_empty(){return field.close_work_demand(body)}}

    rows_work_demand(&self.retirement,self.rows.is_some(),c,body)
 }
 pub fn requires_original_birth(&self)->bool{
    for field in &self.fields{if !field.terminal_is_empty(){return field.requires_original_birth()}}

    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_birth())
 }
 pub fn requires_original_control(&self)->bool{
    for field in &self.fields{if !field.terminal_is_empty(){return field.requires_original_control()}}

    self.retirement.as_ref().is_some_and(|owner|owner.requires_original_control())
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    for field in &mut self.fields{if !field.terminal_is_empty(){return field.close_original_birth_step(original,grant)}}

    self.retirement.as_mut().expect("selected Layout inverse retirement").close_original_birth_step(original,grant)
 }
 pub fn close_step(&mut self,c:RetainedMutationCloseContext<'_,LayoutMutation>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    if !self.closing{return Err(fault().into())}
    let grant=effect.grant();
    if !recover&&!retained_clone_grant_admits(grant,self.close_demand(c,body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
    for field in &mut self.fields{if !field.terminal_is_empty(){return field.close_step(original,body,effect,recover)}}

    close_rows(&mut self.retirement,&mut self.rows,c,original,body,effect)
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.iter().all(OriginalField::terminal_is_empty)&&self.rows.is_none()&&self.retirement.is_none()}}
impl Drop for LayerInverse{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Layout layer inverse needs original funded cleanup")}}
