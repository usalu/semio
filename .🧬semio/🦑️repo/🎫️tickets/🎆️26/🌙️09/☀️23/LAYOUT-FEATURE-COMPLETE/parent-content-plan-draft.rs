//! 🧭️ Original parent content replacement resolves its target and the inherited override rows it displaces.

use crate::{LayoutMutationSource, LayoutMutationView};
use crate::document_equality::LayoutDocumentEquality;
use crate::original_field::close_binding;
use crate::source::set_parent_page_content::SetParentPageContent;
use crate::target::{LayoutTargetLookup, LayoutTargetSource};
use semio_framework_value::{OriginalRetirementCloseLoan, RetainedAllocationEffect, RetirementDemand, RetirementPollExit, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, retained_clone_grant_admits};
use semio_s_artifact_layout_layout_model::{Frame, ParentPage, snapshot::LayoutSnapshot};
use std::mem::{ManuallyDrop, size_of};

const OVERHEAD: usize = 1024;
const KEY_BYTES: usize = 256;
const PARENT_ROWS: usize = 1;
const FRAME_ROWS: usize = 2;

fn invalid() -> ValueError { ValueError::invariant("parent content replacement lost original source custody") }
fn fixed() -> RetirementDemand { RetirementDemand {copy_bytes:OVERHEAD, depth:1, ..Default::default()} }
fn nested(d: RetirementDemand) -> Result<RetirementDemand, ValueError> {
    Ok(RetirementDemand {copy_bytes:d.copy_bytes.checked_add(OVERHEAD).ok_or_else(invalid)?, depth:d.depth.checked_add(1).ok_or_else(invalid)?, ..d})
}
fn child(g: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant {maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(OVERHEAD), maximum_depth:g.maximum_depth.saturating_sub(1), ..g}
}
fn paid(mut p: RetainedCloneProgress) -> RetainedCloneProgress {
    p.copied_items=p.copied_items.max(1);
    p.copied_bytes=p.copied_bytes.saturating_add(OVERHEAD);
    p
}
fn child_error(e: ValueError) -> ValueError { let p=paid(e.retained_progress()); e.with_retained_progress(p) }
fn payload(m: &impl LayoutMutationSource) -> Result<&SetParentPageContent, ValueError> {
    match m.view() { LayoutMutationView::SetParentPageContent(value)=>Ok(value), _=>Err(invalid()) }
}

struct Parents<'a>(&'a [ParentPage]);
impl LayoutTargetSource for Parents<'_> {
    fn identity(&self)->usize { PARENT_ROWS }
    fn count(&self)->usize { self.0.len() }
    fn key(&self,row:usize)->&str { &self.0[row].id }
}
struct Frames<'a>(&'a [Frame]);
impl LayoutTargetSource for Frames<'_> {
    fn identity(&self)->usize { FRAME_ROWS }
    fn count(&self)->usize { self.0.len() }
    fn key(&self,row:usize)->&str { self.0[row].id() }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Phase {
    #[default]
    BindMutation,
    BindBase,
    Locate,
    CompareLayers,
    CompareFrames,
    FindInheritingPage,
    InspectPageOverrides,
    ReservePages,
    ReserveRemovals,
    Complete,
}

#[derive(Clone, Copy)]
pub struct InheritedPageContent {
    pub page: usize,
    first_removal: usize,
    removal_count: usize,
}

#[derive(Default)]
pub struct ParentContentPlan {
    mutation: Option<RetainedCloneBinding>,
    base: Option<RetainedCloneBinding>,
    equality: LayoutDocumentEquality,
    lookup: LayoutTargetLookup,
    pages: ManuallyDrop<Vec<InheritedPageContent>>,
    removals: ManuallyDrop<Vec<usize>>,
    phase: Phase,
    parent: Option<usize>,
    row: usize,
    entry: usize,
    offset: usize,
    second_key: bool,
    counting: bool,
    page_count: usize,
    removal_count: usize,
    first_removal: usize,
    unchanged: bool,
    closing: bool,
}

impl ParentContentPlan {
    pub fn parent(&self) -> Option<usize> { self.parent }
    pub fn unchanged(&self) -> bool { self.unchanged }
    pub fn ready(&self) -> bool { self.phase==Phase::Complete && !self.closing }
    pub fn pages(&self) -> &[InheritedPageContent] { &self.pages }
    pub fn removed_overrides(&self, page: InheritedPageContent) -> &[usize] {
        &self.removals[page.first_removal..page.first_removal+page.removal_count]
    }

    pub fn verify<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>)->Result<(),ValueError>{
        if self.closing{return Err(invalid())}
        payload(m.get())?;
        m.check_binding(&self.mutation)?;
        b.check_binding(&self.base)?;
        Ok(())
    }

    fn frames<'a,M:LayoutMutationSource>(&self,m:RetainedCloneRef<'a,M>,b:RetainedCloneRef<'a,LayoutSnapshot>)->Result<(RetainedCloneRef<'a,Vec<Frame>>,RetainedCloneRef<'a,Vec<Frame>>),ValueError>{
        let parent=self.parent.ok_or_else(invalid)?;
        Ok((m.project(FRAME_ROWS,|m|&payload(m).expect("verified replacement").frames),b.project(PARENT_ROWS,|b|&b.parent_pages[parent].frames)))
    }

    pub fn demand<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>)->Result<RetirementDemand,ValueError>{
        self.verify(m,b)?;
        match self.phase{
            Phase::BindMutation=>nested(m.binding_demand(&self.mutation)?.unwrap_or_default()),
            Phase::BindBase=>nested(b.binding_demand(&self.base)?.unwrap_or_default()),
            Phase::Locate|Phase::InspectPageOverrides=>nested(self.lookup.demand()),
            Phase::CompareFrames=>{let(left,right)=self.frames(m,b)?;nested(self.equality.demand(left,right)?)},
            Phase::ReservePages=>reserve_demand::<InheritedPageContent>(self.page_count),
            Phase::ReserveRemovals=>reserve_demand::<usize>(self.removal_count),
            Phase::Complete=>Ok(Default::default()),
            _=>Ok(fixed()),
        }
    }

    pub fn work_demand<M:LayoutMutationSource>(&self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>)->Result<u64,ValueError>{
        self.verify(m,b)?;
        match self.phase{
            Phase::BindMutation=>m.binding_work_demand(&self.mutation),
            Phase::BindBase=>b.binding_work_demand(&self.base),
            Phase::CompareFrames=>{let(left,right)=self.frames(m,b)?;self.equality.work_demand(left,right)},
            Phase::Complete=>Ok(0),
            _=>Ok(1),
        }
    }

    pub fn advance<M:LayoutMutationSource>(&mut self,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>{
        let grant=effect.grant();
        let demand=self.demand(m,b)?;
        if self.ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        if !retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<self.work_demand(m,b)?{
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let proposal=payload(m.get())?;
        let base=b.get();
        let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        if !matches!(self.phase,Phase::BindMutation|Phase::BindBase|Phase::CompareFrames)&&!effect.performed(1)? {return Err(invalid())}
        match self.phase{
            Phase::BindMutation=>{
                p=paid(m.bind(&mut self.mutation,&mut effect.nested(child(grant))).result.map_err(child_error)?.unwrap_or_default());
                if self.mutation.is_some(){self.phase=Phase::BindBase}
            }
            Phase::BindBase=>{
                p=paid(b.bind(&mut self.base,&mut effect.nested(child(grant))).result.map_err(child_error)?.unwrap_or_default());
                if self.base.is_some(){self.phase=Phase::Locate}
            }
            Phase::Locate=>{
                p=paid(self.lookup.advance(&Parents(&base.parent_pages),PARENT_ROWS,&proposal.id,child(grant)).map_err(child_error)?.progress());
                if let Some(found)=self.lookup.result(){
                    self.parent=found;
                    let Some(at)=found else{self.phase=Phase::Complete;return Ok(RetainedCloneStep::Complete(p))};
                    if base.parent_pages[at].layers.len()!=proposal.layers.len(){self.begin_scan()}else{self.phase=Phase::CompareLayers}
                }
            }
            Phase::CompareLayers=>{
                if self.row==proposal.layers.len(){
                    self.phase=Phase::CompareFrames;
                }else{
                    let old=&base.parent_pages[self.parent.ok_or_else(invalid)?].layers[self.row];
                    let next=&proposal.layers[self.row];
                    let (left,right)=if self.second_key{(old.name.as_str(),next.name.as_str())}else{(old.id.as_str(),next.id.as_str())};
                    if old.visible!=next.visible||old.locked!=next.locked{self.begin_scan()}
                    else if let Some(same)=compare_key(left,right,&mut self.offset){
                        if !same{self.begin_scan()}
                        else{if self.second_key{self.row+=1}self.second_key=!self.second_key}
                    }
                }
            }
            Phase::CompareFrames=>{
                let(left,right)=self.frames(m,b)?;
                p=paid(self.equality.advance(left,right,&mut effect.nested(child(grant))).map_err(child_error)?.progress());
                if let Some(same)=self.equality.result(){
                    if same{self.unchanged=true;self.phase=Phase::Complete}else{self.begin_scan()}
                }
            }
            Phase::FindInheritingPage=>{
                if self.row==base.pages.len(){
                    if self.counting {self.phase=Phase::ReservePages}
                    else{if self.page_count!=self.pages.len()||self.removal_count!=self.removals.len(){return Err(invalid().with_retained_progress(p))}self.phase=Phase::Complete}
                }else if let Some(parent)=base.pages[self.row].parent_page_id.as_deref(){
                    if let Some(same)=compare_key(parent,&proposal.id,&mut self.offset){
                        if same{
                            self.first_removal=self.removals.len();
                            self.phase=Phase::InspectPageOverrides;
                        }else{self.next_page()}
                    }
                }else{self.next_page()}
            }
            Phase::InspectPageOverrides=>{
                let entries=&base.pages[self.row].overrides;
                if self.entry==entries.len(){
                    if self.counting{self.page_count=self.page_count.checked_add(1).ok_or_else(invalid)?}
                    else{
                        if self.pages.len()==self.pages.capacity(){return Err(invalid().with_retained_progress(p))}
                        self.pages.push(InheritedPageContent{page:self.row,first_removal:self.first_removal,removal_count:self.removals.len()-self.first_removal});
                    }
                    self.next_page();
                }else{
                    p=paid(self.lookup.advance(&Frames(&proposal.frames),FRAME_ROWS,&entries[self.entry].object_id,child(grant)).map_err(child_error)?.progress());
                    if let Some(found)=self.lookup.result(){
                        if found.is_none(){
                            if self.counting{self.removal_count=self.removal_count.checked_add(1).ok_or_else(invalid)?}
                            else{if self.removals.len()==self.removals.capacity(){return Err(invalid().with_retained_progress(p))}self.removals.push(self.entry)}
                        }
                        self.entry+=1;
                        self.lookup=Default::default();
                    }
                }
            }
            Phase::ReservePages=>{reserve_rows(&mut self.pages,self.page_count,effect,&mut p)?;self.phase=Phase::ReserveRemovals}
            Phase::ReserveRemovals=>{
                reserve_rows(&mut self.removals,self.removal_count,effect,&mut p)?;
                self.counting=false;
                self.row=0;
                self.phase=Phase::FindInheritingPage;
            }
            Phase::Complete=>unreachable!(),
        }
        Ok(if self.ready(){RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)})
    }

    fn begin_scan(&mut self){
        self.row=0;
        self.offset=0;
        self.counting=true;
        self.lookup=Default::default();
        self.phase=Phase::FindInheritingPage;
    }

    fn next_page(&mut self){
        self.row+=1;
        self.entry=0;
        self.offset=0;
        self.phase=Phase::FindInheritingPage;
    }

    pub fn begin_close(&mut self){ self.closing=true;self.lookup.begin_close();self.equality.begin_close() }
    fn alias(&self)->&Option<RetainedCloneBinding>{if self.base.is_some(){&self.base}else{&self.mutation}}
    fn inner(body:usize)->usize{body.saturating_sub(OVERHEAD)}
    fn local_close_pending(&self)->bool{self.pages.capacity()!=0||self.removals.capacity()!=0}
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.closing{return Err(invalid())}
        if !self.equality.terminal_is_empty(){return nested(self.equality.close_demand(Self::inner(body))?)}
        if !self.lookup.terminal_is_empty(){return nested(self.lookup.close_demand())}
        if self.local_close_pending(){
            let release_bytes=if self.removals.capacity()>0{if self.removals.is_empty(){self.removals.capacity()*size_of::<usize>()}else{0}}
                else if self.pages.is_empty(){self.pages.capacity()*size_of::<InheritedPageContent>()}else{0};
            return Ok(RetirementDemand{release_bytes,..fixed()});
        }
        nested(RetainedCloneBinding::close_demand(self.alias(),Self::inner(body))?)
    }
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.equality.terminal_is_empty(){return self.equality.close_work_demand(Self::inner(body))}
        if !self.lookup.terminal_is_empty()||self.local_close_pending(){return Ok(1)}
        RetainedCloneBinding::close_work_demand(self.alias(),Self::inner(body))
    }
    pub fn requires_original_control(&self)->bool{
        self.equality.terminal_is_empty()&&self.lookup.terminal_is_empty()&&!self.local_close_pending()&&RetainedCloneBinding::requires_original_control(self.alias())
    }
    pub fn close_step(&mut self,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let grant=effect.grant();
        let demand=self.close_demand(body)?;
        if !recover&&(!retained_clone_grant_admits(grant,demand)||effect.remaining_work_units()?<self.close_work_demand(body)?){return Ok(RetainedCloneStep::Progress(Default::default()))}
        let mut child_effect=effect.nested(child(grant));
        let progress=if !self.equality.terminal_is_empty(){
            paid(self.equality.close_step(Self::inner(body),&mut child_effect).map_err(child_error)?.progress())
        }else if !self.lookup.terminal_is_empty(){
            if !child_effect.performed(1)?{return Err(invalid().into())}
            paid(self.lookup.close_step(child(grant)).progress())
        }else if self.local_close_pending(){
            if !child_effect.performed(1)?{return Err(invalid().into())}
            if self.removals.capacity()>0{
                if self.removals.pop().is_none(){*self.removals=Vec::new()}
            }else if self.pages.pop().is_none(){*self.pages=Vec::new()}
            RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}
        }else{
            let alias=if self.base.is_some(){&mut self.base}else{&mut self.mutation};
            paid(close_binding(alias,loan,Self::inner(body),&mut child_effect)?.progress())
        };
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    pub fn terminal_is_empty(&self)->bool{
        self.closing&&self.equality.terminal_is_empty()&&self.lookup.terminal_is_empty()&&!self.local_close_pending()&&self.mutation.is_none()&&self.base.is_none()
    }
}

impl Drop for ParentContentPlan {
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"parent content plan requires funded retirement")}
}

fn compare_key(left:&str,right:&str,offset:&mut usize)->Option<bool>{
    if left.len()!=right.len(){*offset=0;return Some(false)}
    let end=left.len().min(offset.saturating_add(KEY_BYTES));
    if left.as_bytes()[*offset..end]!=right.as_bytes()[*offset..end]{*offset=0;return Some(false)}
    if end==left.len(){*offset=0;Some(true)}else{*offset=end;None}
}

fn reserve_demand<T>(count:usize)->Result<RetirementDemand,ValueError>{
    Ok(RetirementDemand{capacity_bytes:count.checked_mul(size_of::<T>()).ok_or_else(invalid)?,..fixed()})
}

fn reserve_rows<T>(rows:&mut Vec<T>,count:usize,effect:&mut RetainedAllocationEffect<'_>,p:&mut RetainedCloneProgress)->Result<(),ValueError>{
    if count==0{return Ok(())}
    if rows.capacity()!=0||effect.actual().is_some(){return Err(invalid().with_retained_progress(*p))}
    *effect.actual()=Some(0);
    let allocated=rows.try_reserve_exact(count);
    p.retained_capacity_bytes=rows.capacity().checked_mul(size_of::<T>()).ok_or_else(invalid)?;
    *effect.actual()=Some(p.retained_capacity_bytes);
    if allocated.is_err()||p.retained_capacity_bytes>effect.grant().maximum_capacity_bytes{return Err(invalid().with_retained_progress(*p))}
    Ok(())
}
