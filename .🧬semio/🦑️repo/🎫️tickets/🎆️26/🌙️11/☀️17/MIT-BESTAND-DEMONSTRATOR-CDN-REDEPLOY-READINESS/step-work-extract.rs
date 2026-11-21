#[inline(never)]
fn settle_original_effect_work(
    work: &mut OriginalWork<'_>,
    action: &mut Option<OriginalWorkActionIdentity>,
    receipt: &mut Option<OriginalWorkPerformedReceipt>,
    proof: &mut Option<OriginalWorkActionProof>,
) -> Result<u128, ValueError> {
    let performed = OriginalWorkSink::observed_work(receipt.as_ref().ok_or_else(|| ValueError::invariant("original effect receipt absent"))?);
    work.claim_performed_for_action(*action, receipt)?;
    if !work.performed_seen_for_action(*action)? {
        work.cancel_for_action(action)?;
    } else {
        let mut partial = None;
        if !work.settle_partial_for_action_into(*action, &mut partial)? {
            return Err(ValueError::invariant("original effect partial receipt is pending"));
        }
        let partial = partial.ok_or_else(|| ValueError::invariant("original effect partial outcome absent"))?;
        if partial.total() != performed || partial.source_refusal().is_some() {
            return Err(ValueError::invariant("original effect partial outcome differs from its performed work"));
        }
        let accepted = work.commit_for_action(performed, *action, proof)?;
        work.finish_for_action(proof, action)?;
        if !accepted {
            return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "original effect performed more work than its ceiling"));
        }
    }
    Ok(performed)
}

#[inline(never)]
fn recover_original_effect_work(
    work: &mut OriginalWork<'_>,
    action: &mut Option<OriginalWorkActionIdentity>,
    receipt: &mut Option<OriginalWorkPerformedReceipt>,
    proof: &mut Option<OriginalWorkActionProof>,
) {
    if receipt.is_some() {
        let _ = work.claim_performed_for_action(*action, receipt);
    }
    if action.is_none() {
        return;
    }
    if proof.is_some() {
        let _ = work.finish_for_action(proof, action);
    } else if work.performed_seen_for_action(*action).unwrap_or(false) {
        if work.partial_pending_for_action(*action).unwrap_or(false) {
            let mut outcome = None;
            let _ = work.settle_partial_for_action_into(*action, &mut outcome);
        }
        if let Ok(total) = work.performed_total_for_action(*action) {
            let _ = work.commit_for_action(total, *action, proof);
            let _ = work.finish_for_action(proof, action);
        }
    } else {
        let _ = work.cancel_for_action(action);
    }
}
