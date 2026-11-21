use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, RetirementDemand, ValueError};

/// 🧭️ Advances one allocation-free validation or projection step of the borrowed snapshot.
pub fn advance_structured_preparation(
    copied: usize,
    preparation: impl FnOnce() -> Result<bool, ValueError>,
    cx: &mut StepContext<'_>,
) -> Result<Option<Result<bool, ValueError>>, OriginalTurnFailure> {
    let demand = RetirementDemand { copy_bytes: copied, depth: 1, ..Default::default() };
    if cx.should_yield() || !cx.retained_work_can_enter(demand) {
        return Ok(None);
    }
    let Some((result, performed)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| {
        if !effect.performed(1)? {
            return Ok::<_, ValueError>(None);
        }
        Ok(Some(preparation()))
    })? else { return Ok(None) };
    let result = result?;
    if performed != 0 {
        cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() })?;
        cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
    }
    Ok(result)
}
