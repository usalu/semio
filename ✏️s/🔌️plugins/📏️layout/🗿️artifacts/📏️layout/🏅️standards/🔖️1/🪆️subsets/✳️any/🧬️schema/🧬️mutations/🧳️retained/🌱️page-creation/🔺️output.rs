//! 🌱️ Page insertion or removal publishes together with the ordered spread membership.
use crate::{MutationPayload, original_field::OriginalField};
use semio_framework_value::{
    OriginalRetirementCloseLoan, OriginalRetirementPoll, RetainedAllocationEffect,
    RetirementDemand, RetirementPollExit, ValueError,
    retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, retained_clone_grant_admits},
};
use semio_os_kernel::{RetainedFlight, retained_routed};
use semio_s_artifact_layout_layout_diff::{LayoutDiff, LayoutPageInsertion, LayoutPageRemoval, LayoutPagesDelta, LayoutSpreadsDelta, LayoutSpreadsModification, SpreadPatch};
use semio_s_artifact_layout_layout_model::{Page, PageColumns, PageMargins, Spread, snapshot::LayoutSnapshot};
use semio_s_artifact_layout_layout_mutations_decisions::{
    LayoutMutationView, create_page::CreatePage, delete_page::DeletePage, page_deletion::DeletePageCoordinates,
};
use std::mem::{ManuallyDrop, size_of, take};

#[derive(Clone, Copy, Default)]
enum Phase {
    #[default]
    Select,
    Reserve,
    SpreadId,
    Members,
    CopyMembers,
    AppendId,
    PublishPage,
    Ready,
    Refused,
}

/// 📍 A validated page edit and its original spread position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageMembership {
    Create { page: usize, spread: usize },
    Delete(DeletePageCoordinates),
}

impl PageMembership {
    fn spread(self) -> usize {
        match self { Self::Create { spread, .. } => spread, Self::Delete(at) => at.spread }
    }

    fn member_count(self, count: usize) -> Result<usize, ValueError> {
        match self {
            Self::Create { .. } => count.checked_add(1),
            Self::Delete(at) => count.checked_sub(usize::from(at.member.is_some())),
        }.ok_or_else(invalid)
    }
}

#[derive(Default)]
pub struct PageMembershipOutput {
    phase: Phase,
    selection: Option<PageMembership>,
    sources: Option<[(u64, usize, usize, usize); 2]>,
    fields: [OriginalField<String>; 2],
    members: OriginalField<Vec<String>>,
    held_members: ManuallyDrop<Option<Vec<String>>>,
    member: usize,
    retirement: RetainedCloneClose,
    flight: RetainedFlight,
    closing: bool,
}

fn invalid() -> ValueError {
    ValueError::invariant("original page membership output lost its selected custody")
}

const METADATA: usize = 512 + 2 * size_of::<LayoutDiff>() + 2 * size_of::<Page>();

fn fixed() -> RetirementDemand {
    RetirementDemand { copy_bytes: METADATA, depth: 1, ..Default::default() }
}

fn nested(mut demand: RetirementDemand, move_bytes: usize) -> RetirementDemand {
    demand.copy_bytes += METADATA + move_bytes;
    demand.depth += 1;
    demand
}

fn child(grant: RetainedCloneGrant, move_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(METADATA + move_bytes),
        maximum_depth: grant.maximum_depth.saturating_sub(1),
        ..grant
    }
}

fn receipt(mut progress: RetainedCloneProgress, move_bytes: usize) -> RetainedCloneProgress {
    progress.copied_items = progress.copied_items.max(1);
    progress.copied_bytes += METADATA + move_bytes;
    progress
}

fn payload(mutation: &impl MutationPayload<CreatePage>) -> &CreatePage {
    let LayoutMutationView::CreatePage(payload) = mutation.view() else { unreachable!("selected create-page mutation") };
    payload
}

fn vacant_page() -> Page {
    Page {
        id: String::new(),
        name: String::new(),
        spread_id: String::new(),
        parent_page_id: None,
        width: 0.0,
        height: 0.0,
        margins: PageMargins { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 },
        columns: PageColumns { count: 0, gutter: 0.0 },
        guides: Vec::new(),
        layer_ids: Vec::new(),
        layers: Vec::new(),
        frames: Vec::new(),
        overrides: Vec::new(),
        background_drawing: None,
    }
}

impl PageMembershipOutput {
    pub fn embedded_birth_work() -> Result<u64, ValueError> {
        let strings = OriginalField::<String>::embedded_birth_work()?.checked_mul(2).ok_or_else(invalid)?;
        strings.checked_add(OriginalField::<Vec<String>>::embedded_birth_work()?).ok_or_else(invalid)
    }

    fn spread<'a>(&self, base: RetainedCloneRef<'a, LayoutSnapshot>) -> RetainedCloneRef<'a, Spread> {
        let row = self.selection.expect("selected page edit").spread();
        base.project(8, |base| &base.spreads).project(row, move |rows| &rows[row])
    }

    fn text<'a, M: MutationPayload<CreatePage>>(&self, index: usize, command: RetainedCloneRef<'a, M>, base: RetainedCloneRef<'a, LayoutSnapshot>) -> RetainedCloneRef<'a, String> {
        if index == 0 {
            self.spread(base).project(0, |spread| &spread.id)
        } else {
            command.project(0, |mutation| &payload(mutation).page).project(0, |page| &page.id)
        }
    }

    pub fn demand<M: MutationPayload<CreatePage>>(&self, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.sources.is_some_and(|sources| sources != [command.reference_identity(), base.reference_identity()]) {
            return Err(invalid());
        }
        let body = body.min(256);
        let maximum = maximum.saturating_sub(1);
        Ok(match self.phase {
            Phase::Reserve => {
                let selection = self.selection.ok_or_else(invalid)?;
                let count = selection.member_count(self.spread(base).get().page_ids.len())?;
                let page_row = match selection {
                    PageMembership::Create { .. } => size_of::<LayoutPageInsertion>(),
                    PageMembership::Delete(_) => size_of::<LayoutPageRemoval>(),
                };
                let capacity_bytes = count.checked_mul(size_of::<String>())
                    .and_then(|bytes| bytes.checked_add(page_row + size_of::<LayoutSpreadsModification>()))
                    .ok_or_else(invalid)?;
                RetirementDemand { capacity_bytes, ..fixed() }
            }
            Phase::SpreadId | Phase::AppendId => {
                let index = usize::from(matches!(self.phase, Phase::AppendId));
                let field = &self.fields[index];
                if field.value().is_some() {
                    nested(field.exchange_demand()?, size_of::<String>())
                } else {
                    nested(field.demand(self.text(index, command, base), body, maximum)?, 0)
                }
            }
            Phase::Members => {
                if self.members.value().is_some() {
                    nested(self.members.exchange_demand()?, size_of::<Option<Vec<String>>>())
                } else {
                    nested(self.members.demand(self.spread(base).project(1, |spread| &spread.page_ids), body, maximum)?, 0)
                }
            }
            Phase::Refused => return Err(invalid()),
            _ => fixed(),
        })
    }

    pub fn work_demand<M: MutationPayload<CreatePage>>(&self, command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, body: usize, maximum: u64) -> Result<u64, ValueError> {
        let body = body.min(256);
        let maximum = maximum.saturating_sub(1);
        let child_work = match self.phase {
            Phase::SpreadId | Phase::AppendId => {
                let index = usize::from(matches!(self.phase, Phase::AppendId));
                let field = &self.fields[index];
                if field.value().is_some() { 1 } else { field.work_demand(self.text(index, command, base), body, maximum)? }
            }
            Phase::Members => {
                if self.members.value().is_some() { 1 } else {
                    self.members.work_demand(self.spread(base).project(1, |spread| &spread.page_ids), body, maximum)?
                }
            }
            _ => 0,
        };
        child_work.checked_add(1).ok_or_else(invalid)
    }

    pub fn advance<M: MutationPayload<CreatePage> + MutationPayload<DeletePage>>(&mut self, prepared: &mut [M], command: RetainedCloneRef<'_, M>, base: RetainedCloneRef<'_, LayoutSnapshot>, selection: PageMembership, diff: &mut LayoutDiff, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        let grant = effect.grant();
        let mut progress = receipt(Default::default(), 0);
        let sources = [command.reference_identity(), base.reference_identity()];
        if self.closing || self.selection.is_some_and(|previous| previous != selection) || self.sources.is_some_and(|previous| previous != sources) {
            return Err(invalid().into());
        }
        if !recover {
            let demand = self.demand(command, base, body, maximum)?;
            if !retained_clone_grant_admits(grant, demand) || self.work_demand(command, base, body, maximum)? > maximum.min(effect.remaining_work_units()?) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if !effect.performed(1)? { return Err(invalid().into()); }
        }
        let body = body.min(256);
        let maximum = maximum.saturating_sub(1);
        match self.phase {
            Phase::Select if !recover => {
                let base = base.get();
                let spread = base.spreads.get(selection.spread()).ok_or_else(invalid)?;
                let valid = match (selection, command.get().view()) {
                    (PageMembership::Create { page, .. }, LayoutMutationView::CreatePage(_)) => page <= base.pages.len(),
                    (PageMembership::Delete(at), LayoutMutationView::DeletePage(_)) => {
                        at.page < base.pages.len() && at.member.is_none_or(|member| member < spread.page_ids.len())
                    }
                    _ => false,
                };
                if !valid {
                    return Err(invalid().with_retained_progress(progress).into());
                }
                self.selection = Some(selection);
                self.sources = Some(sources);
                self.phase = Phase::Reserve;
            }
            Phase::Reserve if !recover => {
                if diff.pages.is_some() || diff.spreads.is_some() || effect.actual().is_some() {
                    return Err(invalid().with_retained_progress(progress).into());
                }
                let count = selection.member_count(self.spread(base).get().page_ids.len())?;
                *effect.actual() = Some(0);
                self.phase = Phase::Refused;
                let pages = diff.pages.insert(LayoutPagesDelta::default());
                let spreads = &mut diff.spreads.insert(LayoutSpreadsDelta::default()).modified;
                let (mut failed, page_bytes) = match selection {
                    PageMembership::Create { .. } => (pages.inserted.try_reserve_exact(1).is_err(), pages.inserted.capacity() * size_of::<LayoutPageInsertion>()),
                    PageMembership::Delete(_) => (pages.removed.try_reserve_exact(1).is_err(), pages.removed.capacity() * size_of::<LayoutPageRemoval>()),
                };
                failed |= spreads.try_reserve_exact(1).is_err();
                let mut actual = page_bytes + spreads.capacity() * size_of::<LayoutSpreadsModification>();
                if !failed {
                    let mut page_ids = Vec::new();
                    failed = page_ids.try_reserve_exact(count).is_err();
                    actual += page_ids.capacity() * size_of::<String>();
                    spreads.push(LayoutSpreadsModification { id: String::new(), patch: SpreadPatch { name: None, page_ids: Some(page_ids) } });
                }
                progress.retained_capacity_bytes = actual;
                *effect.actual() = Some(actual);
                if failed || actual > grant.maximum_capacity_bytes { return Err(invalid().with_retained_progress(progress).into()); }
                self.phase = Phase::SpreadId;
            }
            Phase::SpreadId | Phase::AppendId => {
                let index = usize::from(matches!(self.phase, Phase::AppendId));
                let source = self.text(index, command, base);
                let field = &mut self.fields[index];
                if !recover && field.value().is_some() {
                    let moved = size_of::<String>();
                    let (value, paid) = field.take(&mut effect.nested(child(grant, moved)))?;
                    progress = receipt(paid, moved);
                    let value = value.ok_or_else(invalid)?;
                    let row = &mut diff.spreads.as_mut().ok_or_else(invalid)?.modified[0];
                    if index == 0 {
                        row.id = value;
                        self.phase = Phase::Members;
                    } else {
                        row.patch.page_ids.as_mut().ok_or_else(invalid)?.push(value);
                        self.phase = Phase::PublishPage;
                    }
                } else {
                    progress = receipt(field.advance(source, body, maximum, &mut effect.nested(child(grant, 0)), recover)?.progress(), 0);
                }
            }
            Phase::Members => {
                if !recover && self.members.value().is_some() {
                    let moved = size_of::<Option<Vec<String>>>();
                    let (members, paid) = self.members.take(&mut effect.nested(child(grant, moved)))?;
                    *self.held_members = members;
                    progress = receipt(paid, moved);
                    self.phase = Phase::CopyMembers;
                } else {
                    let source = self.spread(base).project(1, |spread| &spread.page_ids);
                    progress = receipt(self.members.advance(source, body, maximum, &mut effect.nested(child(grant, 0)), recover)?.progress(), 0);
                }
            }
            Phase::CopyMembers if !recover => {
                let held = self.held_members.as_mut().ok_or_else(invalid)?;
                if let Some(member) = held.get_mut(self.member) {
                    if !matches!(selection, PageMembership::Delete(at) if at.member == Some(self.member)) {
                        diff.spreads.as_mut().ok_or_else(invalid)?.modified[0].patch.page_ids.as_mut().ok_or_else(invalid)?.push(take(member));
                    }
                    self.member += 1;
                } else {
                    self.phase = match selection {
                        PageMembership::Create { .. } => Phase::AppendId,
                        PageMembership::Delete(_) => Phase::PublishPage,
                    };
                }
            }
            Phase::PublishPage if !recover => {
                let mutation = prepared.last_mut().ok_or_else(invalid)?;
                let pages = diff.pages.as_mut().ok_or_else(invalid)?;
                match selection {
                    PageMembership::Create { page, .. } => {
                        let payload = <M as MutationPayload<CreatePage>>::payload_mut(mutation).ok_or_else(invalid)?;
                        pages.inserted.push(LayoutPageInsertion { index: page, row: std::mem::replace(&mut payload.page, vacant_page()) });
                    }
                    PageMembership::Delete(at) => {
                        let payload = <M as MutationPayload<DeletePage>>::payload_mut(mutation).ok_or_else(invalid)?;
                        pages.removed.push(LayoutPageRemoval { id: take(&mut payload.id), index: at.page });
                    }
                }
                self.phase = Phase::Ready;
                return Ok(RetainedCloneStep::Complete(progress));
            }
            _ => return Err(invalid().with_retained_progress(progress).into()),
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
        self.fields.iter_mut().for_each(OriginalField::begin_close);
        self.members.begin_close();
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let body = body.min(256);
        let demand = if let Some(field) = self.fields.iter().find(|field| !field.terminal_is_empty()) {
            field.close_demand(body)?
        } else if !self.members.terminal_is_empty() {
            self.members.close_demand(body)?
        } else {
            self.retirement.next_owner_demands::<Vec<String>>(self.held_members.is_some(), &None, body)?
        };
        Ok(nested(demand, 0))
    }

    pub fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        let body = body.min(256);
        if let Some(field) = self.fields.iter().find(|field| !field.terminal_is_empty()) {
            return field.close_work_demand(body);
        }
        if !self.members.terminal_is_empty() { return self.members.close_work_demand(body); }
        self.retirement.next_owner_work::<Vec<String>>(self.held_members.is_some(), &None, body)
    }

    fn selected_original(&self, birth: bool) -> bool {
        if let Some(field) = self.fields.iter().find(|field| !field.terminal_is_empty()) {
            return if birth { field.requires_original_birth() } else { field.requires_original_control() };
        }
        if !self.members.terminal_is_empty() {
            return if birth { self.members.requires_original_birth() } else { self.members.requires_original_control() };
        }
        if birth { self.retirement.requires_original_birth() } else { self.retirement.requires_original_control() }
    }

    pub fn requires_original_birth(&self) -> bool {
        self.flight.idle() && self.selected_original(true)
    }

    pub fn requires_original_control(&self) -> bool {
        self.flight.loaned().unwrap_or_else(|| self.selected_original(false))
    }

    pub fn close_original_birth_step(&mut self, original: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        let grant = child(grant, 0);
        if let Some(field) = self.fields.iter_mut().find(|field| !field.terminal_is_empty()) {
            return field.close_original_birth_step(original, grant);
        }
        if !self.members.terminal_is_empty() { return self.members.close_original_birth_step(original, grant); }
        self.retirement.step_original_birth(original, grant)
    }

    pub fn close_step(&mut self, mut original: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let grant = effect.grant();
        if !self.closing { return Err(invalid().into()); }
        if !recover && (!retained_clone_grant_admits(grant, self.close_demand(body)?) || effect.remaining_work_units()? < self.close_work_demand(body)?) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let body = body.min(256);
        let effect = &mut effect.nested(child(grant, 0));
        let progress = if let Some(field) = self.fields.iter_mut().find(|field| !field.terminal_is_empty()) {
            field.close_step(original, body, effect, recover)?.progress()
        } else if !self.members.terminal_is_empty() {
            self.members.close_step(original, body, effect, recover)?.progress()
        } else if !self.retirement.is_empty() || !self.flight.idle() {
            let loaned = self.requires_original_control();
            let loan = if loaned { Some(original.as_deref_mut().ok_or_else(invalid)?) } else { None };
            let mut work = 0;
            let step = retained_routed!(self.flight, effect, recover, 1, body, 0, loan, work,
                |reply| if loaned { self.retirement.step_original_into(original.as_deref_mut().ok_or_else(invalid)?, body, effect, reply) } else { self.retirement.step_into(body, effect, reply) },
                if loaned { self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(invalid)?, body, effect, reply) } else { self.retirement.recover_step_into(body, effect, reply) })?;
            let _ = work;
            step.progress()
        } else {
            self.retirement.begin_granted(&mut self.held_members, effect).result?.ok_or_else(invalid)?.progress()
        };
        Ok(RetainedCloneStep::Progress(receipt(progress, 0)))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.fields.iter().all(OriginalField::terminal_is_empty) && self.members.terminal_is_empty()
            && self.held_members.is_none() && self.retirement.is_empty() && self.flight.idle()
    }
}

impl Drop for PageMembershipOutput {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original page membership requires funded retirement");
    }
}
