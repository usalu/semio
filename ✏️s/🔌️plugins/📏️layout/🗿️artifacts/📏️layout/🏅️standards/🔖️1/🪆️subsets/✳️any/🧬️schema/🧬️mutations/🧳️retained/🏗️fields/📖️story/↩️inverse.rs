//! ↩️ The exact deleted span and the previous style runs undo the retained story splice.
use crate::{MutationPayload,original_field::OriginalField,story_output::runs};
use semio_s_artifact_layout_layout_mutations_decisions::{LayoutMutationSource,LayoutMutationView,edit_story::EditStory,set_story_runs::SetStoryRuns,story_plan::StorySplicePlan,story_text::OriginalTextPreparation,story_source::{SpliceTextSource,story_id}};
use semio_s_artifact_layout_layout_diff::retained::apply::page_cache::close_owned;
use semio_s_artifact_layout_layout_model::{snapshot::LayoutSnapshot,TextStyleRun};
use semio_os_kernel::RetainedFlight;
use semio_framework_value::{
    OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,
    RetireOwned,RetirementDemand,RetirementPollExit,ValueError,
    retained_clone::{RetainedCloneRef,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits},
};
use std::mem::{ManuallyDrop,size_of};

fn invalid()->ValueError{ValueError::invariant("Layout story inverse lost its original command")}
fn count(plan:&StorySplicePlan,b:&LayoutSnapshot)->usize{1+usize::from(!b.stories[plan.story].style_runs.is_empty())}
fn span<'a,M:LayoutMutationSource>(plan:&StorySplicePlan,m:RetainedCloneRef<'a,M>,b:RetainedCloneRef<'a,LayoutSnapshot>)->SpliceTextSource<'a,M>{
    SpliceTextSource{m,b,story:plan.story,range:plan.range(),inverse:true}
}
fn fixed<M>()->RetirementDemand{RetirementDemand{copy_bytes:4*size_of::<M>()+256,depth:1,..Default::default()}}

pub struct StoryInverse<M:MutationPayload<EditStory>+MutationPayload<SetStoryRuns>+RetireOwned>{
    text:OriginalTextPreparation,
    id:OriginalField<String>,
    runs:OriginalField<Vec<TextStyleRun>>,
    runs_id:OriginalField<String>,
    rows:ManuallyDrop<Option<Vec<M>>>,
    retirement:RetainedCloneClose,
    flight:RetainedFlight,
    phase:u8,
    closing:bool,
}

impl<M:MutationPayload<EditStory>+MutationPayload<SetStoryRuns>+RetireOwned> Default for StoryInverse<M>{
    fn default()->Self{Self{text:Default::default(),id:Default::default(),runs:Default::default(),runs_id:Default::default(),rows:ManuallyDrop::new(None),retirement:Default::default(),flight:Default::default(),phase:0,closing:false}}
}

impl<M:MutationPayload<EditStory>+MutationPayload<SetStoryRuns>+RetireOwned> StoryInverse<M>{
    pub fn birth_work()->Result<u64,ValueError>{Ok(2*OriginalField::<String>::embedded_birth_work()?+OriginalField::<Vec<TextStyleRun>>::embedded_birth_work()?)}
    pub fn demand(&self,plan:&StorySplicePlan,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{
        plan.verify(m,b)?;
        if !plan.ready()||plan.failure.is_some()||plan.unchanged||self.closing{return Err(invalid())}
        Ok(match self.phase{
            0=>RetirementDemand{capacity_bytes:count(plan,b.get())*size_of::<M>(),..fixed::<M>()},
            1=>self.text.demand(&span(plan,m,b))?,
            2=>self.id.demand(story_id(m),body,max)?,
            3 if count(plan,b.get())==2=>self.runs.demand(runs(b,plan.story),body,max)?,
            4=>self.id.exchange_demand()?,
            5=>self.text.take_demand(),
            6=>self.runs_id.demand(story_id(m),body,max)?,
            7=>self.runs.exchange_demand()?,
            8=>self.runs_id.exchange_demand()?,
            _=>fixed::<M>(),
        })
    }
    pub fn work_demand(&self,plan:&StorySplicePlan,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64)->Result<u64,ValueError>{
        match self.phase{
            1=>self.text.work_demand(&span(plan,m,b)),
            2=>self.id.work_demand(story_id(m),body,max),
            6=>self.runs_id.work_demand(story_id(m),body,max),
            3 if count(plan,b.get())==2=>self.runs.work_demand(runs(b,plan.story),body,max),
            _=>Ok(1),
        }
    }
    pub fn advance(&mut self,plan:&StorySplicePlan,m:RetainedCloneRef<'_,M>,b:RetainedCloneRef<'_,LayoutSnapshot>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant=effect.grant();
        let demand=if recover{Default::default()}else{self.demand(plan,m,b,body,max)?};
        if !recover&&(!retained_clone_grant_admits(grant,demand)||self.work_demand(plan,m,b,body,max)?>max.min(effect.remaining_work_units()?)){
            return Ok(RetainedCloneStep::Progress(Default::default()))
        }
        let mut progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        let LayoutMutationView::EditStory(command)=m.get().view()else{return Err(invalid().into())};
        match self.phase{
            0=>{
                effect.performed(1)?;*self.rows=Some(Vec::new());let rows=self.rows.as_mut().ok_or_else(invalid)?;
                *effect.actual()=Some(0);let attempted=rows.try_reserve_exact(count(plan,b.get()));
                progress.retained_capacity_bytes=rows.capacity()*size_of::<M>();
                *effect.actual()=Some(progress.retained_capacity_bytes);
                if attempted.is_err()||!progress.fits(grant){return Err(invalid().with_retained_progress(progress).into())}
                self.phase=1;
            }
            1=>{progress=self.text.advance(&span(plan,m,b),effect,recover)?.progress();if self.text.ready(){self.phase=2}}
            2=>{progress=self.id.advance(story_id(m),body,max,effect,recover)?.progress();if self.id.value().is_some(){self.phase=3}}
            3=>{
                if count(plan,b.get())==1{effect.performed(1)?;self.phase=4}
                else{progress=self.runs.advance(runs(b,plan.story),body,max,effect,recover)?.progress();if self.runs.value().is_some(){self.phase=4}}
            }
            4=>{
                let(id,paid)=self.id.take(effect)?;progress=paid;
                let row=EditStory{id:id.ok_or_else(invalid)?,offset:command.offset,delete:plan.inserted_scalars(),insert:String::new()};
                self.rows.as_mut().ok_or_else(invalid)?.push(M::from_payload(row));self.phase=5;
            }
            5=>{
                effect.performed(1)?;let(insert,paid)=self.text.take(grant)?;progress=paid;
                let rows=self.rows.as_mut().ok_or_else(invalid)?;
                let Some(row)=rows.first_mut().and_then(MutationPayload::<EditStory>::payload_mut)else{return Err(invalid().into())};
                row.insert=insert.ok_or_else(invalid)?;self.phase=if count(plan,b.get())==1{9}else{6};
            }
            6=>{progress=self.runs_id.advance(story_id(m),body,max,effect,recover)?.progress();if self.runs_id.value().is_some(){self.phase=7}}
            7=>{
                let(runs,paid)=self.runs.take(effect)?;progress=paid;
                self.rows.as_mut().ok_or_else(invalid)?.push(M::from_payload(SetStoryRuns{id:String::new(),runs:runs.ok_or_else(invalid)?}));self.phase=8;
            }
            8=>{
                let(id,paid)=self.runs_id.take(effect)?;progress=paid;let rows=self.rows.as_mut().ok_or_else(invalid)?;
                let Some(row)=rows.last_mut().and_then(MutationPayload::<SetStoryRuns>::payload_mut)else{return Err(invalid().into())};
                row.id=id.ok_or_else(invalid)?;rows.swap(0,1);self.phase=9;
            }
            _=>return Err(invalid().into())
        }
        Ok(if self.phase==9{RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    pub fn take_demand(&self)->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Vec<M>>(),depth:1,..Default::default()}}
    pub fn take(&mut self,grant:RetainedCloneGrant)->Result<(Option<Vec<M>>,RetainedCloneProgress),ValueError>{
        if self.phase!=9||self.closing{return Err(invalid())}
        let demand=self.take_demand();
        if !retained_clone_grant_admits(grant,demand){return Ok((None,Default::default()))}
        self.phase=10;
        Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
    }
    pub fn begin_close(&mut self){self.closing=true;self.text.begin_close();self.id.begin_close();self.runs.begin_close();self.runs_id.begin_close();}
    pub fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if !self.text.terminal_is_empty(){return self.text.close_demand(body)}
        if !self.id.terminal_is_empty(){return self.id.close_demand(body)}
        if !self.runs.terminal_is_empty(){return self.runs.close_demand(body)}
        if !self.runs_id.terminal_is_empty(){return self.runs_id.close_demand(body)}
        self.retirement.next_owner_demands::<Vec<M>>(self.rows.is_some(),&None,body)
    }
    pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
        if !self.text.terminal_is_empty(){return self.text.close_work_demand(body)}
        if !self.id.terminal_is_empty(){return self.id.close_work_demand(body)}
        if !self.runs.terminal_is_empty(){return self.runs.close_work_demand(body)}
        if !self.runs_id.terminal_is_empty(){return self.runs_id.close_work_demand(body)}
        self.retirement.next_owner_work::<Vec<M>>(self.rows.is_some(),&None,body)
    }
    pub fn requires_original_birth(&self)->bool{
        if !self.text.terminal_is_empty(){return self.text.requires_original_birth()}
        if !self.id.terminal_is_empty(){return self.id.requires_original_birth()}
        if !self.runs.terminal_is_empty(){return self.runs.requires_original_birth()}
        if !self.runs_id.terminal_is_empty(){return self.runs_id.requires_original_birth()}
        self.flight.idle()&&self.retirement.requires_original_birth()
    }
    pub fn requires_original_control(&self)->bool{
        if !self.text.terminal_is_empty(){return self.text.requires_original_control()}
        if !self.id.terminal_is_empty(){return self.id.requires_original_control()}
        if !self.runs.terminal_is_empty(){return self.runs.requires_original_control()}
        if !self.runs_id.terminal_is_empty(){return self.runs_id.requires_original_control()}
        self.flight.loaned().unwrap_or_else(||self.retirement.requires_original_control())
    }
    pub fn close_original_birth_step(&mut self,loan:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
        if !self.text.terminal_is_empty(){return self.text.close_original_birth_step(loan,grant)}
        if !self.id.terminal_is_empty(){return self.id.close_original_birth_step(loan,grant)}
        if !self.runs.terminal_is_empty(){return self.runs.close_original_birth_step(loan,grant)}
        if !self.runs_id.terminal_is_empty(){return self.runs_id.close_original_birth_step(loan,grant)}
        self.retirement.step_original_birth(loan,grant)
    }
    pub fn close_step(&mut self,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool)->Result<RetainedCloneStep,RetirementPollExit>{
        if !self.closing{return Err(invalid().into())}
        if !recover&&(!retained_clone_grant_admits(effect.grant(),self.close_demand(body)?)||self.close_work_demand(body)?>effect.remaining_work_units()?){
            return Ok(RetainedCloneStep::Progress(Default::default()))
        }
        if !self.text.terminal_is_empty(){return self.text.close_step(loan,body,effect,recover)}
        if !self.id.terminal_is_empty(){return self.id.close_step(loan,body,effect,recover)}
        if !self.runs.terminal_is_empty(){return self.runs.close_step(loan,body,effect,recover)}
        if !self.runs_id.terminal_is_empty(){return self.runs_id.close_step(loan,body,effect,recover)}
        let mut work=0;
        close_owned(&mut self.retirement,&mut self.rows,&mut self.flight,loan,body,effect,recover,&mut work)
    }
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.flight.idle()&&self.text.terminal_is_empty()&&self.id.terminal_is_empty()&&self.runs.terminal_is_empty()&&self.runs_id.terminal_is_empty()&&self.rows.is_none()&&self.retirement.is_empty()}
}
impl<M:MutationPayload<EditStory>+MutationPayload<SetStoryRuns>+RetireOwned> Drop for StoryInverse<M>{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original story inverse requires funded close")}
}
