//! ↩️ A deleted page's exact original row and prior ordered spread position fund its inverse.
use crate::{LayoutMutation, create_page::CreatePage, set_page_spread::SetPageSpread};
use semio_framework_value::{
    OriginalRetirementCloseLoan, OriginalRetirementPoll, RetainedAllocationEffect, RetirementDemand,
    RetirementPollExit, ValueError, retained_clone::{
        RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, retained_clone_grant_admits,
    }
};
use semio_os_kernel::RetainedMutationCloseContext;
use semio_s_artifact_layout_layout_model::{Page, snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_mutations_decisions::page_deletion::DeletePageCoordinates;
use super::super::original_field::{MutationRows, OriginalField, close_rows, rows_work_demand};
use std::mem::{ManuallyDrop, size_of};

#[derive(Default, Clone, Copy)]
enum Phase { #[default] Select, Reserve, PageId, SpreadId, Page, Ready, Taken, Refused }

#[derive(Default)]
pub struct PageDeletionInverse {
    phase: Phase,
    coordinates: Option<DeletePageCoordinates>,
    source: Option<(u64, usize, usize, usize)>,
    restore_order: bool,
    page: OriginalField<Page>,
    ids: [OriginalField<String>; 2],
    rows: ManuallyDrop<Option<Vec<LayoutMutation>>>,
    retirement: Option<MutationRows<LayoutMutation>>,
    closing: bool,
}

fn invalid() -> ValueError { ValueError::invariant("deleted page inverse changed original custody") }

const METADATA: usize = 256 + 2 * size_of::<LayoutMutation>();
fn fixed() -> RetirementDemand { RetirementDemand { copy_bytes: METADATA, depth: 1, ..Default::default() } }
fn nested(mut demand: RetirementDemand) -> RetirementDemand {
    demand.copy_bytes += METADATA;
    demand.depth += 1;
    demand
}
fn child(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(METADATA), maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}
fn receipt(mut paid: RetainedCloneProgress) -> RetainedCloneProgress {
    paid.copied_bytes += METADATA;
    paid.copied_items = paid.copied_items.max(1);
    paid
}

fn page_source(base: RetainedCloneRef<'_, LayoutSnapshot>, at: usize) -> RetainedCloneRef<'_, Page> {
    base.project(9, |base| &base.pages).project(at, move |pages| &pages[at])
}

fn text_source(base: RetainedCloneRef<'_, LayoutSnapshot>, at: usize, field: usize) -> RetainedCloneRef<'_, String> {
    let page = page_source(base, at);
    if field == 0 { page.project(0, |page| &page.id) } else { page.project(2, |page| &page.spread_id) }
}

impl PageDeletionInverse {
    pub fn embedded_birth_work() -> Result<u64, ValueError> {
        let strings = OriginalField::<String>::embedded_birth_work()?.checked_mul(2).ok_or_else(invalid)?;
        strings.checked_add(OriginalField::<Page>::embedded_birth_work()?).ok_or_else(invalid)
    }

    fn validate_source(&self, base: RetainedCloneRef<'_, LayoutSnapshot>, at: DeletePageCoordinates) -> Result<(), ValueError> {
        if self.closing || self.coordinates.is_some_and(|selected| selected != at) || self.source.is_some_and(|identity| identity != base.reference_identity()) {
            return Err(invalid());
        }
        Ok(())
    }

    pub fn demand(&self, base: RetainedCloneRef<'_, LayoutSnapshot>, at: DeletePageCoordinates, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError> {
        self.validate_source(base, at)?;
        let body = body.min(256);
        let maximum = maximum.saturating_sub(1);
        Ok(match self.phase {
            Phase::Reserve => RetirementDemand { capacity_bytes: size_of::<LayoutMutation>() * (1 + usize::from(self.restore_order)), ..fixed() },
            Phase::PageId | Phase::SpreadId => {
                let which = usize::from(matches!(self.phase, Phase::SpreadId));
                let field = &self.ids[which];
                nested(if field.value().is_some() { field.exchange_demand()? }
                    else { field.demand(text_source(base, at.page, which), body, maximum)? })
            }
            Phase::Page => nested(if self.page.value().is_some() { self.page.exchange_demand()? }
                else { self.page.demand(page_source(base, at.page), body, maximum)? }),
            Phase::Refused | Phase::Taken => return Err(invalid()),
            _ => fixed(),
        })
    }

    pub fn work_demand(&self, base: RetainedCloneRef<'_, LayoutSnapshot>, at: DeletePageCoordinates, body: usize, maximum: u64) -> Result<u64, ValueError> {
        self.validate_source(base, at)?;
        let body = body.min(256);
        let maximum = maximum.saturating_sub(1);
        let child = match self.phase {
            Phase::PageId | Phase::SpreadId => {
                let which = usize::from(matches!(self.phase, Phase::SpreadId));
                let field = &self.ids[which];
                if field.value().is_some() { 1 } else { field.work_demand(text_source(base, at.page, which), body, maximum)? }
            }
            Phase::Page => {
                if self.page.value().is_some() { 1 } else { self.page.work_demand(page_source(base, at.page), body, maximum)? }
            }
            _ => 0,
        };
        child.checked_add(1).ok_or_else(invalid)
    }

    pub fn advance(&mut self, base: RetainedCloneRef<'_, LayoutSnapshot>, at: DeletePageCoordinates, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        self.validate_source(base, at)?;
        if !recover {
            if !retained_clone_grant_admits(grant, self.demand(base, at, body, maximum)?) ||
                self.work_demand(base, at, body, maximum)? > maximum.min(effect.remaining_work_units()?) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if !effect.performed(1)? { return Err(invalid().into()); }
        }
        let mut paid = receipt(Default::default());
        let body = body.min(256);
        let maximum = maximum.saturating_sub(1);
        match self.phase {
            Phase::Select if !recover => {
                let source = base.get();
                let spread = source.spreads.get(at.spread).ok_or_else(invalid)?;
                if source.pages.get(at.page).is_none() || at.member.is_some_and(|index| index >= spread.page_ids.len()) {
                    return Err(invalid().with_retained_progress(paid).into());
                }
                self.source = Some(base.reference_identity());
                self.coordinates = Some(at);
                self.restore_order = at.member != spread.page_ids.len().checked_sub(1);
                self.phase = Phase::Reserve;
            }
            Phase::Reserve if !recover => {
                if effect.actual().is_some() { return Err(invalid().into()); }
                self.phase = Phase::Refused;
                let rows = self.rows.insert(Vec::new());
                let reserve = rows.try_reserve_exact(1 + usize::from(self.restore_order));
                paid.retained_capacity_bytes = rows.capacity() * size_of::<LayoutMutation>();
                *effect.actual() = Some(paid.retained_capacity_bytes);
                if reserve.is_err() || paid.retained_capacity_bytes > grant.maximum_capacity_bytes {
                    return Err(invalid().with_retained_progress(paid).into());
                }
                self.phase = if self.restore_order { Phase::PageId } else { Phase::Page };
            }
            Phase::PageId | Phase::SpreadId => {
                let which = usize::from(matches!(self.phase, Phase::SpreadId));
                let field = &mut self.ids[which];
                let effect = &mut effect.nested(child(grant));
                if !recover && field.value().is_some() {
                    let (value, progress) = field.take(effect)?;
                    let value = value.ok_or_else(invalid)?;
                    paid = receipt(progress);
                    let rows = self.rows.as_mut().ok_or_else(invalid)?;
                    if which == 0 {
                        rows.push(LayoutMutation::SetPageSpread(SetPageSpread { id: value, spread_id: String::new(), index: at.member }));
                        self.phase = Phase::SpreadId;
                    } else {
                        let Some(LayoutMutation::SetPageSpread(command)) = rows.first_mut() else { return Err(invalid().into()); };
                        command.spread_id = value;
                        self.phase = Phase::Page;
                    }
                } else {
                    paid = receipt(field.advance(text_source(base, at.page, which), body, maximum, effect, recover)?.progress());
                }
            }
            Phase::Page => {
                let effect = &mut effect.nested(child(grant));
                if !recover && self.page.value().is_some() {
                    let (page, progress) = self.page.take(effect)?;
                    paid = receipt(progress);
                    self.rows.as_mut().ok_or_else(invalid)?.push(LayoutMutation::CreatePage(CreatePage { page: page.ok_or_else(invalid)?, index: Some(at.page) }));
                    self.phase = Phase::Ready;
                    return Ok(RetainedCloneStep::Complete(paid));
                }
                paid = receipt(self.page.advance(page_source(base, at.page), body, maximum, effect, recover)?.progress());
            }
            _ => return Err(invalid().with_retained_progress(paid).into()),
        }
        Ok(RetainedCloneStep::Progress(paid))
    }

    pub fn take_demand(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: 2 * size_of::<Option<Vec<LayoutMutation>>>(), depth: 1, ..Default::default() }
    }

    pub fn take(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Vec<LayoutMutation>>, RetainedCloneProgress), ValueError> {
        if self.closing || !matches!(self.phase, Phase::Ready) { return Err(invalid()); }
        let demand = self.take_demand();
        if !retained_clone_grant_admits(grant, demand) { return Ok((None, Default::default())); }
        self.phase = Phase::Taken;
        Ok((self.rows.take(), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        self.source = None;
        self.coordinates = None;
        self.page.begin_close();
        self.ids.iter_mut().for_each(OriginalField::begin_close);
    }

    pub fn close_demand(&self, context: RetainedMutationCloseContext<'_, LayoutMutation>, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.page.terminal_is_empty() { return self.page.close_demand(body); }
        if let Some(field) = self.ids.iter().find(|field| !field.terminal_is_empty()) { return field.close_demand(body); }
        if let Some(owner) = self.retirement.as_ref() {
            return if owner.terminal_is_empty() {
                Ok(RetirementDemand { copy_bytes: 2 * size_of::<Option<MutationRows<LayoutMutation>>>(), depth: 1, ..Default::default() })
            } else { owner.demands(context.mutations, body) };
        }
        Ok(if self.rows.is_some() { MutationRows::<LayoutMutation>::constructor_demand() } else { Default::default() })
    }

    pub fn close_work_demand(&self, context: RetainedMutationCloseContext<'_, LayoutMutation>, body: usize) -> Result<u64, ValueError> {
        if !self.page.terminal_is_empty() { return self.page.close_work_demand(body); }
        if let Some(field) = self.ids.iter().find(|field| !field.terminal_is_empty()) { return field.close_work_demand(body); }
        rows_work_demand(&self.retirement, self.rows.is_some(), context, body)
    }

    pub fn requires_original_birth(&self) -> bool {
        if !self.page.terminal_is_empty() { return self.page.requires_original_birth(); }
        if let Some(field) = self.ids.iter().find(|field| !field.terminal_is_empty()) { return field.requires_original_birth(); }
        self.retirement.as_ref().is_some_and(|owner| owner.requires_original_birth())
    }

    pub fn requires_original_control(&self) -> bool {
        if !self.page.terminal_is_empty() { return self.page.requires_original_control(); }
        if let Some(field) = self.ids.iter().find(|field| !field.terminal_is_empty()) { return field.requires_original_control(); }
        self.retirement.as_ref().is_some_and(|owner| owner.requires_original_control())
    }

    pub fn close_original_birth_step(&mut self, original: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        if !self.page.terminal_is_empty() { return self.page.close_original_birth_step(original, grant); }
        if let Some(field) = self.ids.iter_mut().find(|field| !field.terminal_is_empty()) { return field.close_original_birth_step(original, grant); }
        self.retirement.as_mut().expect("selected original inverse rows").close_original_birth_step(original, grant)
    }

    pub fn close_step(&mut self, context: RetainedMutationCloseContext<'_, LayoutMutation>, original: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        if !self.closing { return Err(invalid().into()); }
        if !recover && (!retained_clone_grant_admits(effect.grant(), self.close_demand(context, body)?) ||
            effect.remaining_work_units()? < self.close_work_demand(context, body)?) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if !self.page.terminal_is_empty() { return self.page.close_step(original, body, effect, recover); }
        if let Some(field) = self.ids.iter_mut().find(|field| !field.terminal_is_empty()) { return field.close_step(original, body, effect, recover); }
        close_rows(&mut self.retirement, &mut self.rows, context, original, body, effect)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.page.terminal_is_empty() && self.ids.iter().all(OriginalField::terminal_is_empty)
            && self.rows.is_none() && self.retirement.is_none()
    }
}

impl Drop for PageDeletionInverse {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "deleted page inverse requires funded retirement");
    }
}
