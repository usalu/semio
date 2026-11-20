//! 📖️ Retained parent content ownership and sparse override displacement.

use crate::original_field::OriginalField;
use semio_framework_value::{OriginalRetirementCloseLoan, OriginalRetirementPoll, RetainedAllocationEffect, RetireOwned, RetirementDemand, RetirementPollExit, ValueError};
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneClose, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, retained_clone_grant_admits};
use semio_os_kernel::RetainedFlight;
use semio_s_artifact_layout_layout_diff::retained::apply::page_cache::close_owned;
use std::mem::{ManuallyDrop, size_of};

pub use semio_s_artifact_layout_layout_mutations_decisions::parent_content::ParentContentPlan;

const SHELL: usize = 3072;
const BODY: usize = 256;

fn invalid() -> ValueError {
    ValueError::invariant("retained parent content lost its original field or plan")
}

fn fixed() -> RetirementDemand {
    RetirementDemand { copy_bytes: SHELL, depth: 1, ..Default::default() }
}

fn nested(mut demand: RetirementDemand) -> RetirementDemand {
    demand.copy_bytes += SHELL;
    demand.depth += 1;
    demand
}

fn child(grant: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(SHELL), maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant }
}

fn paid(mut progress: RetainedCloneProgress) -> RetainedCloneProgress {
    progress.copied_items = progress.copied_items.max(1);
    progress.copied_bytes += SHELL;
    progress
}

fn reserve<T>(rows: &mut Vec<T>, count: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
    if count == 0 { return Ok(paid(Default::default())); }
    if rows.capacity() != 0 || effect.actual().is_some() { return Err(invalid()); }
    *effect.actual() = Some(0);
    let result = rows.try_reserve_exact(count);
    let retained_capacity_bytes = rows.capacity() * size_of::<T>();
    *effect.actual() = Some(retained_capacity_bytes);
    let progress = paid(RetainedCloneProgress { retained_capacity_bytes, ..Default::default() });
    if result.is_err() || retained_capacity_bytes > effect.grant().maximum_capacity_bytes {
        return Err(invalid().with_retained_progress(progress));
    }
    Ok(progress)
}

fn capacity<T>(count: usize) -> Result<RetirementDemand, ValueError> {
    let capacity_bytes = count.checked_mul(size_of::<T>()).ok_or_else(invalid)?;
    Ok(RetirementDemand { capacity_bytes, ..fixed() })
}

struct Field<T: RetainedClone> {
    owner: OriginalField<T>,
    recycling: bool,
}

impl<T: RetainedClone> Default for Field<T> {
    fn default() -> Self { Self { owner: Default::default(), recycling: false } }
}

impl<T: RetainedClone> Field<T> {
    fn demand(&self, source: RetainedCloneRef<'_, T>, body: usize, maximum: u64) -> Result<RetirementDemand, ValueError> {
        if self.recycling { return self.recycle_demand(body); }
        self.owner.value().map_or_else(|| self.owner.demand(source, body, maximum), |_| self.owner.exchange_demand())
    }

    fn work(&self, source: RetainedCloneRef<'_, T>, body: usize, maximum: u64) -> Result<u64, ValueError> {
        if self.recycling { return self.recycle_work(body); }
        self.owner.value().map_or_else(|| self.owner.work_demand(source, body, maximum), |_| Ok(1))
    }

    fn step(&mut self, source: RetainedCloneRef<'_, T>, body: usize, maximum: u64, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<(Option<T>, RetainedCloneProgress), RetirementPollExit> {
        if self.recycling { return self.recycle(body, effect, recover).map(|progress| (None, progress)); }
        if !recover && self.owner.value().is_some() {
            let (value, progress) = self.owner.take(effect)?;
            self.owner.begin_close();
            self.recycling = true;
            return Ok((value, progress));
        }
        Ok((None, self.owner.advance(source, body, maximum, effect, recover)?.progress()))
    }

    fn recycle_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.owner.terminal_is_empty() {
            Ok(RetirementDemand { copy_bytes: 2 * size_of::<Self>(), depth: 1, ..Default::default() })
        } else {
            self.owner.close_demand(body)
        }
    }

    fn recycle_work(&self, body: usize) -> Result<u64, ValueError> {
        if self.owner.terminal_is_empty() { OriginalField::<T>::embedded_birth_work() } else { self.owner.close_work_demand(body) }
    }

    fn recycle(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneProgress, RetirementPollExit> {
        if self.owner.terminal_is_empty() {
            let demand = self.recycle_demand(body)?;
            if recover || !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(self.recycle_work(body)?)? {
                return Err(invalid().into());
            }
            *self = Self::default();
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() });
        }
        Ok(self.owner.close_step(None, body, effect, recover)?.progress())
    }
}

struct Saved<T: RetireOwned> {
    value: ManuallyDrop<Option<T>>,
    close: RetainedCloneClose,
    flight: RetainedFlight,
}

impl<T: RetireOwned> Saved<T> {
    fn empty_shell() -> Self {
        Self { value: ManuallyDrop::new(None), close: Default::default(), flight: Default::default() }
    }
    fn start(&mut self, value: T) -> Result<&mut T, ValueError> {
        if !self.empty() { return Err(invalid()); }
        Ok(self.value.insert(value))
    }
    fn get_mut(&mut self) -> Result<&mut T, ValueError> { self.value.as_mut().ok_or_else(invalid) }
    fn take_demand() -> RetirementDemand { RetirementDemand { copy_bytes: 2 * size_of::<Option<T>>(), depth: 1, ..Default::default() } }
    fn take(&mut self, grant: RetainedCloneGrant) -> Result<(Option<T>, RetainedCloneProgress), ValueError> {
        let demand = Self::take_demand();
        if !retained_clone_grant_admits(grant, demand) { return Ok((None, Default::default())); }
        Ok((Some(self.value.take().ok_or_else(invalid)?), RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }
    fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.close.next_owner_demands::<T>(self.value.is_some(), &None, body) }
    fn work(&self, body: usize) -> Result<u64, ValueError> { self.close.next_owner_work::<T>(self.value.is_some(), &None, body) }
    fn birth(&self) -> bool { self.flight.idle() && self.close.requires_original_birth() }
    fn control(&self) -> bool { self.flight.loaned().unwrap_or_else(|| self.close.requires_original_control()) }
    fn birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        self.close.step_original_birth(loan, grant)
    }
    fn step(&mut self, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        close_owned(&mut self.close, &mut self.value, &mut self.flight, loan, body, effect, recover, &mut 0)
    }
    fn empty(&self) -> bool { self.value.is_none() && self.close.is_empty() && self.flight.idle() }
}

trait Close {
    fn close_begin(&mut self);
    fn empty(&self) -> bool;
    fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn work(&self, body: usize) -> Result<u64, ValueError>;
    fn birth(&self) -> bool;
    fn control(&self) -> bool;
    fn birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll;
    fn step(&mut self, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit>;
}

impl<T: RetainedClone> Close for Field<T> {
    fn close_begin(&mut self) { self.owner.begin_close(); }
    fn empty(&self) -> bool { self.owner.terminal_is_empty() }
    fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.owner.close_demand(body) }
    fn work(&self, body: usize) -> Result<u64, ValueError> { self.owner.close_work_demand(body) }
    fn birth(&self) -> bool { self.owner.requires_original_birth() }
    fn control(&self) -> bool { self.owner.requires_original_control() }
    fn birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll { self.owner.close_original_birth_step(loan, grant) }
    fn step(&mut self, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> { self.owner.close_step(loan, body, effect, recover) }
}

impl<T: RetireOwned> Close for Saved<T> {
    fn close_begin(&mut self) {}
    fn empty(&self) -> bool { self.empty() }
    fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.demand(body) }
    fn work(&self, body: usize) -> Result<u64, ValueError> { self.work(body) }
    fn birth(&self) -> bool { self.birth() }
    fn control(&self) -> bool { self.control() }
    fn birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll { self.birth_step(loan, grant) }
    fn step(&mut self, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> { self.step(loan, body, effect, recover) }
}

impl Close for semio_s_artifact_layout_layout_diff::retained::apply::page_cache::PageCachePreparation {
    fn close_begin(&mut self) { self.begin_close(); }
    fn empty(&self) -> bool { self.terminal_is_empty() }
    fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.close_demand(body) }
    fn work(&self, body: usize) -> Result<u64, ValueError> { self.close_work_demand(body) }
    fn birth(&self) -> bool { self.requires_original_birth() }
    fn control(&self) -> bool { self.requires_original_control() }
    fn birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll { self.close_original_birth_step(loan, grant) }
    fn step(&mut self, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> { self.close_step(loan, body, effect, recover, &mut 0) }
}

impl<T: Close> Close for Option<T> {
    fn close_begin(&mut self) { if let Some(owner) = self { owner.close_begin(); } }
    fn empty(&self) -> bool { self.as_ref().is_none_or(Close::empty) }
    fn demand(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.as_ref().ok_or_else(invalid)?.demand(body) }
    fn work(&self, body: usize) -> Result<u64, ValueError> { self.as_ref().ok_or_else(invalid)?.work(body) }
    fn birth(&self) -> bool { self.as_ref().is_some_and(Close::birth) }
    fn control(&self) -> bool { self.as_ref().is_some_and(Close::control) }
    fn birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
        self.as_mut().expect("selected optional parent owner").birth_step(loan, grant)
    }
    fn step(&mut self, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
        self.as_mut().ok_or_else(invalid)?.step(loan, body, effect, recover)
    }
}

macro_rules! retirement {
    ($($field:ident),+ $(,)?) => {
        fn owners(&self) -> [&dyn Close; [$(stringify!($field)),+].len()] { [$(&self.$field),+] }
        fn owners_mut(&mut self) -> [&mut dyn Close; [$(stringify!($field)),+].len()] { [$(&mut self.$field),+] }
        pub fn begin_close(&mut self) {
            self.closing = true;
            for owner in self.owners_mut() { owner.close_begin(); }
        }
        pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
            self.owners().into_iter().find(|owner| !owner.empty())
                .map_or(Ok(Default::default()), |owner| owner.demand(body.min(BODY)).map(nested))
        }
        pub fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
            self.owners().into_iter().find(|owner| !owner.empty())
                .map_or(Ok(0), |owner| owner.work(body.min(BODY)))
        }
        pub fn requires_original_birth(&self) -> bool {
            self.owners().into_iter().find(|owner| !owner.empty()).is_some_and(Close::birth)
        }
        pub fn requires_original_control(&self) -> bool {
            self.owners().into_iter().find(|owner| !owner.empty()).is_some_and(Close::control)
        }
        pub fn close_original_birth_step(&mut self, loan: &mut OriginalRetirementCloseLoan<'_>, grant: RetainedCloneGrant) -> OriginalRetirementPoll {
            self.owners_mut().into_iter().find(|owner| !owner.empty()).expect("selected original parent content owner").birth_step(loan, child(grant))
        }
        pub fn close_step(&mut self, loan: Option<&mut OriginalRetirementCloseLoan<'_>>, body: usize, effect: &mut RetainedAllocationEffect<'_>, recover: bool) -> Result<RetainedCloneStep, RetirementPollExit> {
            if !self.closing { return Err(invalid().into()); }
            if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
            let grant = effect.grant();
            if !recover && (!retained_clone_grant_admits(grant, self.close_demand(body)?) || effect.remaining_work_units()? < self.close_work_demand(body)?) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            let owner = self.owners_mut().into_iter().find(|owner| !owner.empty()).ok_or_else(invalid)?;
            let progress = owner.step(loan, body.min(BODY), &mut effect.nested(child(grant)), recover)?.progress();
            Ok(RetainedCloneStep::Progress(paid(progress)))
        }
        pub fn terminal_is_empty(&self) -> bool { self.closing && self.owners().into_iter().all(Close::empty) }
    };
}
#[path = "🔺️output.rs"]
mod output;
#[path = "↩️inverse.rs"]
mod inverse;
pub use inverse::ParentContentInverse;
pub use output::ParentContentOutput;
