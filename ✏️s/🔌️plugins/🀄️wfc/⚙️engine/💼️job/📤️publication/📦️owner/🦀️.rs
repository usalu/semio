//! 📦️ Publication slots return inner custody before paying their separate physical header removal.
use super::Publication;
use semio_framework_job::{InteractiveJobCloseStep, OriginalTurnFailure, RetainedCloneProgress, RetainedCloneStep, StepContext};
use semio_framework_value::{OriginalRetirementFailure, RetirementDemand, RetainedWorkOutcome, ValueError};

/// 📏️ Quotes inner retirement or the exact remaining publication box allocation.
pub fn publication_retirement_demands(slot: &Option<Box<Publication>>) -> Result<RetirementDemand, ValueError> {
    match slot {
        None => Ok(Default::default()),
        Some(publication) if publication.terminal_is_empty() => Ok(RetirementDemand { release_bytes: std::mem::size_of::<Publication>(), depth: 1, ..Default::default() }),
        Some(publication) => publication.retirement_demands(),
    }
}

/// 🧹️ Closes one original inner turn or returns its already-empty box on a distinct paid turn.
pub fn close_publication_step(slot: &mut Option<Box<Publication>>, cx: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    let Some(publication) = slot.as_mut() else {
        return Ok(InteractiveJobCloseStep::Complete { progress: Default::default() });
    };
    if !publication.terminal_is_empty() {
        let demand = publication.retirement_demands()?;
        return semio_framework_job::close_original_retained_effect_step(cx, demand, |effect| publication.close_step(effect).result.map_err(OriginalRetirementFailure::from));
    }
    let demand = publication_retirement_demands(slot)?;
    semio_framework_job::close_original_retained_step(cx, demand, |_| {
        drop(slot.take());
        RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })), work: 1 }
    })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
