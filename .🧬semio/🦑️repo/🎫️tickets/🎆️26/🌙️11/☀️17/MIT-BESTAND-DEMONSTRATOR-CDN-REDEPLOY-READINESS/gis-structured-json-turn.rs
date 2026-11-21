use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonWriteSource};
use semio_framework_value::{NativeEncodeControl, ValueError, ValueRefusalKind};

/// 🪙️ Advances one paid JSON frontier action over the mounted job's original Source.
pub fn advance_structured_json(
    cursor: &mut JsonBorrowedWriteCursor,
    source: &impl JsonWriteSource,
    output: &mut Option<Vec<u8>>,
    cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    if output.is_some() {
        return Ok(true);
    }
    let mut grant = cx.retained_grant();
    let output_copy = size_of::<Option<Vec<u8>>>();
    if cx.should_yield() || grant.maximum_copy_bytes < output_copy {
        return Ok(false);
    }
    grant.maximum_copy_bytes -= output_copy;
    let next = cursor.normal_step_demands_with_copy(source, grant.maximum_copy_bytes)?;
    let total = cx.original_total_retained_grant();
    if next.copy_bytes > total.maximum_copy_bytes.saturating_sub(output_copy)
        || next.capacity_bytes > total.maximum_capacity_bytes
        || next.depth > total.maximum_depth
    {
        return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "structured JSON cannot fund its next source phase").into());
    }
    let Some((result, performed)) = cx.with_original_source_effect(grant, |cx, effect| {
        let total = effect.remaining_work_units()?;
        if total == 0 {
            return Ok::<_, ValueError>((Ok(false), Default::default()));
        }
        let mut live = |_| !cx.should_yield();
        let mut native = NativeEncodeControl::new_retained(&mut live);
        let result = cursor.step_with_demand(source, 1, &mut native, effect.grant());
        let mut progress = cursor.normal_step_progress();
        if progress.retained_capacity_bytes != 0 {
            *effect.actual() = Some(progress.retained_capacity_bytes);
        }
        if cursor.last_turn_work() != 0 && !effect.performed(cursor.last_turn_work())? {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "structured JSON exceeded its admitted work"));
        }
        let result = result.map(|(ready, _)| {
            if let Some(bytes) = ready {
                *output = Some(bytes.into_bytes());
                progress.copied_bytes += output_copy;
            }
            output.is_some()
        });
        Ok((result, progress))
    })? else { return Ok(false) };
    let (body, progress) = result?;
    cx.consume_retained(progress)?;
    cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
    Ok(body?)
}
