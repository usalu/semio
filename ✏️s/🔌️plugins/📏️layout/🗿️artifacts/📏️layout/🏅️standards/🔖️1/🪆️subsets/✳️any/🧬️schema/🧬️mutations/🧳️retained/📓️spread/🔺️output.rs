//! 📓️ Original affected spread memberships prepare and transfer one String owner per funded turn.
use crate::MutationPayload;
use semio_s_artifact_layout_layout_mutations_decisions::LayoutMutationView;
use semio_s_artifact_layout_layout_mutations_decisions::set_page_spread::SetPageSpread;
use semio_s_artifact_layout_layout_model::{Spread,snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_diff::*;
use crate::original_field::{OriginalField,close_binding};
use semio_os_kernel::{RetainedFlight as Flight,retained_routed};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneRef,RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits}};
use semio_s_artifact_layout_layout_mutations_decisions::target::{LayoutTargetSource,LayoutTargetLookup};
use std::marker::PhantomData;
use std::mem::{ManuallyDrop,size_of};
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original spread output custody is invalid")}
fn fixed()->RetirementDemand{RetirementDemand{copy_bytes:2048+size_of::<LayoutDiff>()*2,depth:1,..Default::default()}}
fn nested(mut d:RetirementDemand)->RetirementDemand{d.copy_bytes+=fixed().copy_bytes;d.depth+=1;d}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed().copy_bytes,maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=fixed().copy_bytes;p}
fn refused(e:ValueError)->ValueError{let p=receipt(e.retained_progress());e.with_retained_progress(p)}
fn payload<M:MutationPayload<SetPageSpread>>(m:&M)->Result<&SetPageSpread,ValueError>{if let LayoutMutationView::SetPageSpread(p)=m.view(){Ok(p)}else{Err(fault())}}
struct Spreads<'a>(&'a[Spread]);
impl LayoutTargetSource for Spreads<'_>{fn identity(&self)->usize{741}fn count(&self)->usize{self.0.len()}fn key(&self,i:usize)->&str{&self.0[i].id}}
struct Ids<'a>(&'a[String]);
impl LayoutTargetSource for Ids<'_>{fn identity(&self)->usize{742}fn count(&self)->usize{self.0.len()}fn key(&self,i:usize)->&str{&self.0[i]}}
fn list<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,index:usize)->RetainedCloneRef<'a,Vec<String>>{b.project(8,|b|&b.spreads).project(index,move|rows|&rows[index]).project(1,|s|&s.page_ids)}
fn text<'a>(b:RetainedCloneRef<'a,LayoutSnapshot>,at:[usize;2],old:usize,index:usize)->RetainedCloneRef<'a,String>{if index==2{b.project(9,|b|&b.pages).project(at[0],move|rows|&rows[at[0]]).project(0,|p|&p.id)}else{let ordinal=if index==0{at[1]}else{old};b.project(8,|b|&b.spreads).project(ordinal,move|rows|&rows[ordinal]).project(0,|s|&s.id)}}
pub struct SpreadOutput<M:MutationPayload<SetPageSpread>>{mutation:PhantomData<fn() -> M>,m:Option<RetainedCloneBinding>,b:Option<RetainedCloneBinding>,lookup:[LayoutTargetLookup;3],lists:[OriginalField<Vec<String>>;2],texts:[OriginalField<String>;3],held:[ManuallyDrop<Option<Vec<String>>>;2],strings:[ManuallyDrop<Option<String>>;3],retirement:RetainedCloneClose,flight:Flight,at:[usize;2],old:usize,positions:[Option<usize>;2],destination:usize,index:usize,phase:u8,headers:[usize;4],closing:bool}
impl<M:MutationPayload<SetPageSpread>> Default for SpreadOutput<M>{fn default()->Self{Self{mutation:PhantomData,m:None,b:None,lookup:std::array::from_fn(|_|Default::default()),lists:std::array::from_fn(|_|Default::default()),texts:std::array::from_fn(|_|Default::default()),held:std::array::from_fn(|_|ManuallyDrop::new(None)),strings:std::array::from_fn(|_|ManuallyDrop::new(None)),retirement:Default::default(),flight:Default::default(),at:[0;2],old:0,positions:[None;2],destination:0,index:0,phase:0,headers:[0;4],closing:false}}}
impl<M:MutationPayload<SetPageSpread>> SpreadOutput<M>{
 pub fn coordinates(&self)->[usize;2]{self.at}
 pub fn ready(&self)->bool{self.phase==14&&!self.closing}
 pub fn demand(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{m.check_binding(&self.m)?;b.check_binding(&self.b)?;payload(m.get())?;if self.closing||self.m.is_some()&&self.at!=at{return Err(fault())}Ok(match self.phase{0=>nested(m.binding_demand(&self.m)?.unwrap_or_default()),1=>nested(b.binding_demand(&self.b)?.unwrap_or_default()),2..=4=>nested(self.lookup[(self.phase-2)as usize].demand()),5|6=>{let i=(self.phase-5)as usize;if i==1&&self.old==at[1]{fixed()}else if self.lists[i].value().is_some(){let mut d=nested(self.lists[i].exchange_demand()?);d.copy_bytes+=size_of::<Option<Vec<String>>>()*2;d}else{nested(self.lists[i].demand(list(b,if i==0{at[1]}else{self.old}),body.min(256),maximum.saturating_sub(1))?)}},7..=9=>{let i=(self.phase-7)as usize;if i==1&&self.old==at[1]{fixed()}else if self.texts[i].value().is_some(){let mut d=nested(self.texts[i].exchange_demand()?);d.copy_bytes+=size_of::<Option<String>>()*2;d}else{nested(self.texts[i].demand(text(b,at,self.old,i),body.min(256),maximum.saturating_sub(1))?)}},10=>{let cross=self.old!=at[1];let target=b.get().spreads[at[1]].page_ids.len()+usize::from(cross);let previous=if cross{b.get().spreads[self.old].page_ids.len()-1}else{0};RetirementDemand{capacity_bytes:size_of::<LayoutPagesModification>()+size_of::<LayoutSpreadsModification>()*if cross{2}else{1}+(target+previous)*size_of::<String>(),..fixed()}},11..=14=>fixed(),_=>return Err(fault())})}
 pub fn embedded_birth_work()->Result<u64,ValueError>{
    let lists=OriginalField::<Vec<String>>::embedded_birth_work()?.checked_mul(2).ok_or_else(fault)?;
    let texts=OriginalField::<String>::embedded_birth_work()?.checked_mul(3).ok_or_else(fault)?;
    lists.checked_add(texts).ok_or_else(fault)
 }
 pub fn work_demand(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,maximum:u64)->Result<u64,ValueError>{
    let at=self.at;
    let body=body.min(256);
    let limit=maximum.saturating_sub(1);
    let work=match self.phase{
        0=>m.binding_work_demand(&self.m)?,
        1=>b.binding_work_demand(&self.b)?,
        5|6=>{
            let i=(self.phase-5)as usize;
            if i==1&&self.old==at[1]{0}else if self.lists[i].value().is_some(){1}else{
                self.lists[i].work_demand(list(b,if i==0{at[1]}else{self.old}),body,limit)?
            }
        },
        7..=9=>{
            let i=(self.phase-7)as usize;
            if i==1&&self.old==at[1]{0}else if self.texts[i].value().is_some(){1}else{
                self.texts[i].work_demand(text(b,at,self.old,i),body,limit)?
            }
        },
        _=>0,
    };
    work.checked_add(1).ok_or_else(fault)
 }
 pub fn advance(&mut self,prepared:&mut Vec<M>,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,at:[usize;2],d:&mut LayoutDiff,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{let g=effect.grant();let demand=if recover{fixed()}else{self.demand(m,b,at,body,maximum)?};if !recover{if !retained_clone_grant_admits(g,demand)||self.work_demand(m,b,body,maximum)?>maximum.min(effect.remaining_work_units()?){return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(fault().into())}}let mut paid=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};if matches!(self.phase,11..=13){let pages=d.pages.as_ref().ok_or_else(fault)?;let spreads=d.spreads.as_ref().ok_or_else(fault)?;if pages.modified.len()!=1||spreads.modified.len()!=if self.old!=at[1]{2}else{1}||pages.modified.as_ptr()as usize!=self.headers[0]||spreads.modified.as_ptr()as usize!=self.headers[1]||spreads.modified[0].patch.page_ids.as_ref().ok_or_else(fault)?.as_ptr()as usize!=self.headers[2]||self.old!=at[1]&&spreads.modified[1].patch.page_ids.as_ref().ok_or_else(fault)?.as_ptr()as usize!=self.headers[3]{return Err(fault().with_retained_progress(paid).into())}}match self.phase{
 0=>{self.at=at;paid=receipt(m.bind(&mut self.m,&mut effect.nested(child(g))).result.map_err(refused)?.unwrap_or_default());self.phase=1},1=>{paid=receipt(b.bind(&mut self.b,&mut effect.nested(child(g))).result.map_err(refused)?.unwrap_or_default());self.phase=2},
 2=>{let target=&b.get().pages[at[0]].spread_id;let step=self.lookup[0].advance(&Spreads(&b.get().spreads),2,target,child(g)).map_err(refused)?;paid=receipt(step.progress());if let Some(found)=self.lookup[0].result(){self.old=found.ok_or_else(fault)?;self.phase=3}},
 3|4=>{let i=(self.phase-3)as usize;let ordinal=if i==0{at[1]}else{self.old};let step=self.lookup[i+1].advance(&Ids(&b.get().spreads[ordinal].page_ids),self.phase as usize,&payload(m.get())?.id,child(g)).map_err(refused)?;paid=receipt(step.progress());if let Some(found)=self.lookup[i+1].result(){self.positions[i]=found;if i==0{let extent=b.get().spreads[at[1]].page_ids.len()-usize::from(found.is_some());self.destination=payload(m.get())?.index.or(found).unwrap_or(extent);if self.destination>extent{return Err(fault().with_retained_progress(paid).into())}}else if found.is_none(){return Err(fault().with_retained_progress(paid).into())}self.phase+=1}},
 5|6=>{let i=(self.phase-5)as usize;if i==1&&self.old==at[1]{self.phase+=1}else if self.lists[i].value().is_some(){let(value,p)=self.lists[i].take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed().copy_bytes-size_of::<Option<Vec<String>>>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;paid=receipt(p);paid.copied_bytes+=size_of::<Option<Vec<String>>>()*2;*self.held[i]=value;self.phase+=1}else{paid=receipt(self.lists[i].advance(list(b,if i==0{at[1]}else{self.old}),body.min(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())}},
 7..=9=>{let i=(self.phase-7)as usize;if i==1&&self.old==at[1]{self.phase+=1}else if self.texts[i].value().is_some(){let(value,p)=self.texts[i].take(&mut effect.nested(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed().copy_bytes-size_of::<Option<String>>()*2,maximum_depth:g.maximum_depth-1,..g})).map_err(refused)?;paid=receipt(p);paid.copied_bytes+=size_of::<Option<String>>()*2;*self.strings[i]=value;self.phase+=1}else{paid=receipt(self.texts[i].advance(text(b,at,self.old,i),body.min(256),maximum.saturating_sub(1),&mut effect.nested(child(g)),recover)?.progress())}},
 10=>{if d.pages.is_some()||d.spreads.is_some(){return Err(fault().with_retained_progress(paid).into())}if effect.actual().is_some(){return Err(fault().into())}*effect.actual()=Some(0);self.phase=254;d.pages=Some(LayoutPagesDelta::default());d.spreads=Some(LayoutSpreadsDelta::default());let pages=&mut d.pages.as_mut().unwrap().modified;let spreads=&mut d.spreads.as_mut().unwrap().modified;let cross=self.old!=at[1];let mut failed=pages.try_reserve_exact(1).is_err();failed|=spreads.try_reserve_exact(if cross{2}else{1}).is_err();paid.retained_capacity_bytes=pages.capacity()*size_of::<LayoutPagesModification>()+spreads.capacity()*size_of::<LayoutSpreadsModification>();if !failed{pages.push(LayoutPagesModification{id:String::new(),patch:PagePatch::default()});for i in 0..if cross{2}else{1}{spreads.push(LayoutSpreadsModification{id:String::new(),patch:SpreadPatch{page_ids:Some(Vec::new()),..Default::default()}});let rows=spreads[i].patch.page_ids.as_mut().unwrap();let len=if i==0{b.get().spreads[at[1]].page_ids.len()+usize::from(cross)}else{b.get().spreads[self.old].page_ids.len()-1};failed|=rows.try_reserve_exact(len).is_err();paid.retained_capacity_bytes+=rows.capacity()*size_of::<String>();self.headers[i+2]=rows.as_ptr()as usize;}}self.headers[0]=pages.as_ptr()as usize;self.headers[1]=spreads.as_ptr()as usize;*effect.actual()=Some(paid.retained_capacity_bytes);if failed||paid.retained_capacity_bytes>g.maximum_capacity_bytes{return Err(fault().with_retained_progress(paid).into())}self.phase=11},
 11=>{let Some(p)=prepared.last_mut().and_then(MutationPayload::payload_mut)else{return Err(fault().with_retained_progress(paid).into())};let page=&mut d.pages.as_mut().unwrap().modified[0];page.id=std::mem::take(&mut p.id);page.patch.spread_id=Some(std::mem::take(&mut p.spread_id));let rows=&mut d.spreads.as_mut().unwrap().modified;rows[0].id=self.strings[0].take().ok_or_else(fault)?;if self.old!=at[1]{rows[1].id=self.strings[1].take().ok_or_else(fault)?}self.index=0;self.phase=12},
 12=>{let rows=d.spreads.as_mut().unwrap().modified[0].patch.page_ids.as_mut().unwrap();let count=b.get().spreads[at[1]].page_ids.len()+usize::from(self.old!=at[1]);if self.index==count{self.index=0;self.phase=13}else{let value=if self.index==self.destination{self.strings[2].take().ok_or_else(fault)?}else{let mut index=self.index-usize::from(self.index>self.destination);if self.positions[0].is_some_and(|position|index>=position){index+=1}std::mem::take(&mut self.held[0].as_mut().ok_or_else(fault)?[index])};rows.push(value);self.index+=1}},
 13=>{if self.old==at[1]{self.phase=14}else{let source=self.held[1].as_mut().ok_or_else(fault)?;if self.index==source.len(){self.phase=14}else{if Some(self.index)!=self.positions[1]{let value=std::mem::take(&mut source[self.index]);d.spreads.as_mut().unwrap().modified[1].patch.page_ids.as_mut().unwrap().push(value)}self.index+=1}}},14=>{},_=>return Err(fault().into())}Ok(if self.ready(){RetainedCloneStep::Complete(paid)}else{RetainedCloneStep::Progress(paid)})}
 pub fn begin_close(&mut self){self.closing=true;for f in &mut self.lists{f.begin_close()}for f in &mut self.texts{f.begin_close()}for c in &mut self.lookup{c.begin_close()}}
 pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(fault())}for f in &self.lists{if !f.terminal_is_empty(){return Ok(nested(f.close_demand(body.min(256))?))}}for f in &self.texts{if !f.terminal_is_empty(){return Ok(nested(f.close_demand(body.min(256))?))}}for c in &self.lookup{if !c.terminal_is_empty(){return Ok(nested(c.close_demand()))}}if !self.retirement.is_empty(){return Ok(nested(self.retirement.next_owner_demands::<Vec<String>>(false,&None,body.min(256))?))}for v in &self.held{if v.is_some(){return Ok(nested(self.retirement.next_owner_demands::<Vec<String>>(true,&None,body.min(256))?))}}for s in &self.strings{if s.is_some(){return Ok(nested(self.retirement.next_owner_demands::<String>(true,&None,body.min(256))?))}}Ok(nested(RetainedCloneBinding::close_demand(if self.b.is_some(){&self.b}else{&self.m},body.min(256))?))}
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
    let inner=body.min(256);
    for field in &self.lists{if !field.terminal_is_empty(){return field.close_work_demand(inner)}}
    for field in &self.texts{if !field.terminal_is_empty(){return field.close_work_demand(inner)}}
    if self.lookup.iter().any(|owner|!owner.terminal_is_empty()){return Ok(1)}
    if !self.retirement.is_empty(){return self.retirement.next_owner_work::<Vec<String>>(false,&None,inner)}
    if self.held.iter().any(|owner|owner.is_some())||self.strings.iter().any(|owner|owner.is_some()){return Ok(1)}
    RetainedCloneBinding::close_work_demand(if self.b.is_some(){&self.b}else{&self.m},inner)
 }
 fn selected_original(&self,birth:bool)->bool{
    for field in &self.lists{if !field.terminal_is_empty(){return if birth{field.requires_original_birth()}else{field.requires_original_control()}}}
    for field in &self.texts{if !field.terminal_is_empty(){return if birth{field.requires_original_birth()}else{field.requires_original_control()}}}
    if self.lookup.iter().any(|owner|!owner.terminal_is_empty()){return false}
    if !self.retirement.is_empty(){return if birth{self.retirement.requires_original_birth()}else{self.retirement.requires_original_control()}}
    if self.held.iter().any(|owner|owner.is_some())||self.strings.iter().any(|owner|owner.is_some()){return false}
    let binding=if self.b.is_some(){&self.b}else{&self.m};
    if birth{RetainedCloneBinding::requires_original_birth(binding)}else{RetainedCloneBinding::requires_original_control(binding)}
 }
 pub fn requires_original_birth(&self)->bool{self.flight.idle()&&self.selected_original(true)}
 pub fn requires_original_control(&self)->bool{self.flight.loaned().unwrap_or_else(||self.selected_original(false))}
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    let grant=child(grant);
    for field in &mut self.lists{if !field.terminal_is_empty(){return field.close_original_birth_step(original,grant)}}
    for field in &mut self.texts{if !field.terminal_is_empty(){return field.close_original_birth_step(original,grant)}}
    if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}
    RetainedCloneBinding::close_original_birth_step(if self.b.is_some(){&mut self.b}else{&mut self.m},original,grant)
 }
 pub fn close_step(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
    let grant=effect.grant();
    if !recover&&(!retained_clone_grant_admits(grant,self.close_demand(body)?)||effect.remaining_work_units()?<self.close_work_demand(body)?){
        return Ok(RetainedCloneStep::Progress(Default::default()))
    }
    if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
    let body=body.min(256);
    let mut child_effect=effect.nested(child(grant));
    for field in &mut self.lists{if !field.terminal_is_empty(){return field.close_step(original,body,&mut child_effect,recover).map(|step|RetainedCloneStep::Progress(receipt(step.progress())))}}
    for field in &mut self.texts{if !field.terminal_is_empty(){return field.close_step(original,body,&mut child_effect,recover).map(|step|RetainedCloneStep::Progress(receipt(step.progress())))}}
    for cursor in &mut self.lookup{if !cursor.terminal_is_empty(){if !child_effect.performed(1)?{return Err(fault().into())}return Ok(RetainedCloneStep::Progress(receipt(cursor.close_step(child(grant)).progress())))}}
    if !self.retirement.is_empty()||!self.flight.idle(){
        let loaned=self.requires_original_control();
        let loan=if loaned{Some(original.as_deref_mut().ok_or_else(fault)?)}else{None};
        let effect=&mut child_effect;
        let mut work=0;
        let step=retained_routed!(self.flight,effect,recover,1,body,0,loan,work,
            |r|if loaned{self.retirement.step_original_into(original.as_deref_mut().ok_or_else(fault)?,body,effect,r)}else{self.retirement.step_into(body,effect,r)},
            if loaned{self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(fault)?,body,effect,r)}else{self.retirement.recover_step_into(body,effect,r)})?;
        let _=work;
        return Ok(RetainedCloneStep::Progress(receipt(step.progress())))
    }
    for slot in &mut self.held{
        if let Some(step)=self.retirement.begin_granted(slot,&mut child_effect).result?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())))}
    }
    for slot in &mut self.strings{
        if let Some(step)=self.retirement.begin_granted(slot,&mut child_effect).result?{return Ok(RetainedCloneStep::Progress(receipt(step.progress())))}
    }
    close_binding(if self.b.is_some(){&mut self.b}else{&mut self.m},original,body,&mut child_effect).map(|step|RetainedCloneStep::Progress(receipt(step.progress())))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.lists.iter().all(OriginalField::terminal_is_empty)&&self.texts.iter().all(OriginalField::terminal_is_empty)&&self.lookup.iter().all(LayoutTargetLookup::terminal_is_empty)&&self.held.iter().all(|v|v.is_none())&&self.strings.iter().all(|v|v.is_none())&&self.retirement.is_empty()&&self.m.is_none()&&self.b.is_none()&&self.flight.idle()}
}
impl<M:MutationPayload<SetPageSpread>> Drop for SpreadOutput<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"spread output requires original funded closure")}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
