//! 🗑️ Bounded DeletePage resolution over retained original page, frame and spread sources.
use crate::delete_page::DeletePage;
use crate::target::{LayoutTargetLookup, LayoutTargetSource};
use semio_framework_value::{
    OriginalRetirementCloseLoan, OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand,
    RetirementPollExit, ValueError, retained_clone::{
        RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef,
        RetainedCloneStep, retained_clone_grant_admits,
    },
};
use semio_os_kernel::{RetainedFlight, retained_routed};
use semio_s_artifact_layout_layout_model::{Frame, Page, Spread, snapshot::LayoutSnapshot};

/// 📍 Original coordinates for sparse removal, ordered membership and inverse preparation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeletePageCoordinates {
    pub page: usize,
    pub spread: usize,
    pub member: Option<usize>,
}

/// 🚧 The recorded reason this particular page removal cannot be published.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeletePageRefusal {
    MissingPage,
    SurvivingThread,
    MissingSpread { page: usize },
}

impl DeletePageRefusal {
    /// 🔎 The diagnostic code used by the registered mutation.
    pub fn code(self) -> &'static str {
        match self {
            Self::MissingPage | Self::MissingSpread { .. } => "mutation.target-missing",
            Self::SurvivingThread => "mutation.invariant",
        }
    }
}

enum Keys<'a> {
    Pages(&'a [Page]),
    Spreads(&'a [Spread]),
    Members(&'a [String]),
    Frames(&'a [Frame]),
}

impl LayoutTargetSource for Keys<'_> {
    fn identity(&self) -> usize {
        match self {
            Self::Pages(rows) => rows.as_ptr() as usize,
            Self::Spreads(rows) => rows.as_ptr() as usize,
            Self::Members(rows) => rows.as_ptr() as usize,
            Self::Frames(rows) => rows.as_ptr() as usize,
        }
    }

    fn count(&self) -> usize {
        match self {
            Self::Pages(rows) => rows.len(),
            Self::Spreads(rows) => rows.len(),
            Self::Members(rows) => rows.len(),
            Self::Frames(rows) => rows.len(),
        }
    }

    fn key(&self, index: usize) -> &str {
        match self {
            Self::Pages(rows) => &rows[index].id,
            Self::Spreads(rows) => &rows[index].id,
            Self::Members(rows) => &rows[index],
            Self::Frames(rows) => rows[index].id(),
        }
    }
}

#[derive(Clone, Copy, Default)]
enum Phase {
    #[default]
    Page,
    Threads,
    Spread,
    Member,
}

type SourceIdentity = (u64, usize, usize, usize);
const SOURCE_WITNESS_COPY_BYTES: usize = std::mem::size_of::<Option<SourceIdentity>>();

/// 🧳 The plan owns source aliases and ordinal/progress fields only.
#[derive(Default)]
pub struct DeletePagePlan {
    mutation: Option<RetainedCloneBinding>,
    base: Option<RetainedCloneBinding>,
    base_identity: Option<SourceIdentity>,
    lookup: LayoutTargetLookup,
    phase: Phase,
    page: usize,
    spread: usize,
    surviving_page: usize,
    surviving_frame: usize,
    result: Option<Result<DeletePageCoordinates, DeletePageRefusal>>,
    closing: bool,
    close_flight: RetainedFlight,
}

fn invariant() -> ValueError {
    ValueError::invariant("original Layout DeletePage source or close changed")
}

impl DeletePagePlan {
    /// 🎯 Only a settled decision exposes its exact coordinates or refusal.
    pub fn result(&self) -> Option<Result<DeletePageCoordinates, DeletePageRefusal>> {
        self.result
    }

    pub fn demand(&self, mutation: RetainedCloneRef<'_, DeletePage>, base: RetainedCloneRef<'_, LayoutSnapshot>) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.base_identity.is_some_and(|expected| expected != base.reference_identity()) {
            return Err(invariant());
        }
        let mutation_demand = mutation.binding_demand(&self.mutation)?;
        let base_demand = base.binding_demand(&self.base)?;
        if let Some(mut demand) = mutation_demand {
            demand.copy_bytes += SOURCE_WITNESS_COPY_BYTES;
            return Ok(demand);
        }
        Ok(base_demand.unwrap_or_else(|| if self.result.is_none() { self.lookup.demand() } else { RetirementDemand::default() }))
    }

    pub fn work_demand(&self, mutation: RetainedCloneRef<'_, DeletePage>, base: RetainedCloneRef<'_, LayoutSnapshot>) -> Result<u64, ValueError> {
        self.demand(mutation, base)?;
        if self.mutation.is_none() { return mutation.binding_work_demand(&self.mutation)?.checked_add(1).ok_or_else(invariant); }
        if self.base.is_none() { return base.binding_work_demand(&self.base); }
        Ok(u64::from(self.result.is_none()))
    }

    pub fn advance(&mut self, mutation: RetainedCloneRef<'_, DeletePage>, base: RetainedCloneRef<'_, LayoutSnapshot>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.demand(mutation, base)?;
        if self.result.is_some() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let grant = effect.grant();
        if !retained_clone_grant_admits(grant, demand) || effect.remaining_work_units()? < self.work_demand(mutation, base)? {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if self.mutation.is_none() {
            let alias_grant = RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - SOURCE_WITNESS_COPY_BYTES, ..grant };
            let mut progress = mutation.bind(&mut self.mutation, &mut effect.nested(alias_grant)).result?.unwrap_or_default();
            if self.mutation.is_some() {
                if !effect.performed(1)? { return Err(invariant().with_retained_progress(progress)); }
                self.base_identity = Some(base.reference_identity());
                progress.copied_bytes += SOURCE_WITNESS_COPY_BYTES;
            }
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if self.base.is_none() {
            return base.bind(&mut self.base, effect).result.map(|progress| RetainedCloneStep::Progress(progress.unwrap_or_default()));
        }
        if !effect.performed(1)? { return Err(invariant()); }
        let base = base.get();
        let progress = match self.phase {
            Phase::Page => {
                let step = self.lookup.advance(&Keys::Pages(&base.pages), 0, &mutation.get().id, grant)?;
                match self.lookup.result() {
                    Some(Some(page)) => {
                        self.page = page;
                        self.go(if base.pages[page].frames.is_empty() { Phase::Spread } else { Phase::Threads });
                    }
                    Some(None) => self.result = Some(Err(DeletePageRefusal::MissingPage)),
                    None => {}
                }
                step.progress()
            }
            Phase::Threads => self.check_surviving_thread(base, grant)?,
            Phase::Spread => {
                let step = self.lookup.advance(&Keys::Spreads(&base.spreads), 2, &base.pages[self.page].spread_id, grant)?;
                match self.lookup.result() {
                    Some(Some(spread)) => { self.spread = spread; self.go(Phase::Member); }
                    Some(None) => self.result = Some(Err(DeletePageRefusal::MissingSpread { page: self.page })),
                    None => {}
                }
                step.progress()
            }
            Phase::Member => {
                let step = self.lookup.advance(&Keys::Members(&base.spreads[self.spread].page_ids), 3, &base.pages[self.page].id, grant)?;
                if let Some(member) = self.lookup.result() {
                    self.result = Some(Ok(DeletePageCoordinates { page: self.page, spread: self.spread, member }));
                }
                step.progress()
            }
        };
        Ok(if self.result.is_some() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    fn go(&mut self, phase: Phase) {
        self.phase = phase;
        self.lookup = LayoutTargetLookup::new();
    }

    fn check_surviving_thread(&mut self, base: &LayoutSnapshot, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let coordination = RetainedCloneProgress { copied_items: 1, copied_bytes: self.lookup.demand().copy_bytes, ..Default::default() };
        if self.surviving_page == base.pages.len() {
            self.go(Phase::Spread);
            return Ok(coordination);
        }
        let page = &base.pages[self.surviving_page];
        if self.surviving_page == self.page || self.surviving_frame == page.frames.len() {
            self.surviving_page += 1;
            self.surviving_frame = 0;
            return Ok(coordination);
        }
        let Frame::Text { thread_next: Some(target), .. } = &page.frames[self.surviving_frame] else {
            self.surviving_frame += 1;
            return Ok(coordination);
        };
        let step = self.lookup.advance(&Keys::Frames(&base.pages[self.page].frames), 1, target, grant)?;
        match self.lookup.result() {
            Some(Some(_)) => self.result = Some(Err(DeletePageRefusal::SurvivingThread)),
            Some(None) => { self.surviving_frame += 1; self.go(Phase::Threads); }
            None => {}
        }
        Ok(step.progress())
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        self.base_identity = None;
        self.result = None;
        self.lookup.begin_close();
    }

    fn closing_binding(&self) -> &Option<RetainedCloneBinding> {
        if self.base.is_some() { &self.base } else { &self.mutation }
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing { return Err(invariant()); }
        if !self.lookup.terminal_is_empty() { return Ok(self.lookup.close_demand()); }
        RetainedCloneBinding::close_demand(self.closing_binding(), body)
    }

    pub fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        if !self.closing { return Err(invariant()); }
        if !self.lookup.terminal_is_empty() { return Ok(1); }
        RetainedCloneBinding::close_work_demand(self.closing_binding(), body)
    }

    pub fn requires_original_birth(&self) -> bool {
        self.closing && self.lookup.terminal_is_empty() && self.close_flight.idle() && RetainedCloneBinding::requires_original_birth(self.closing_binding())
    }

    pub fn requires_original_control(&self) -> bool {
        self.close_flight.loaned().unwrap_or_else(|| self.closing && self.lookup.terminal_is_empty() && RetainedCloneBinding::requires_original_control(self.closing_binding()))
    }

    pub fn close_original_birth_step(&mut self, original: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        let owner = if self.base.is_some() { &mut self.base } else { &mut self.mutation };
        RetainedCloneBinding::close_original_birth_step(owner, original, grant)
    }

    pub fn close_step(&mut self, mut original: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        if !recover && (!retained_clone_grant_admits(grant, self.close_demand(body)?) || effect.remaining_work_units()? < self.close_work_demand(body)?) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let progress = if !self.lookup.terminal_is_empty() {
            if !effect.performed(1)? { return Err(invariant().into()); }
            self.lookup.close_step(grant).progress()
        } else {
            let loaned = self.requires_original_control();
            let loan = if loaned { Some(original.as_deref_mut().ok_or_else(invariant)?) } else { None };
            let owner = if self.base.is_some() { &mut self.base } else { &mut self.mutation };
            let mut work = 0;
            let step = retained_routed!(self.close_flight, effect, recover, 1, body, 0, loan, work,
                |reply| if loaned { RetainedCloneBinding::close_original_into(owner, original.as_deref_mut().ok_or_else(invariant)?, body, effect, reply) }
                else { RetainedCloneBinding::close_into(owner, body, effect, reply) },
                if loaned { RetainedCloneBinding::recover_close_original_into(owner, original.as_deref_mut().ok_or_else(invariant)?, body, effect, reply) }
                else { RetainedCloneBinding::recover_close_into(owner, body, effect, reply) }
            )?;
            let _ = work;
            step.progress()
        };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.lookup.terminal_is_empty() && self.base.is_none() && self.mutation.is_none() && self.close_flight.idle()
    }
}

