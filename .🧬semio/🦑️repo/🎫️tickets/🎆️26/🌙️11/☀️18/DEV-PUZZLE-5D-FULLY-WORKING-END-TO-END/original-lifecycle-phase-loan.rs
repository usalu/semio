use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand};
use semio_plugin_ownership::bounded_ownership::PluginLifecycleStep;
#[cfg(not(canonical_lifecycle))]
use semio_plugin_ownership::bounded_ownership::original_lifecycle_step;
#[cfg(canonical_lifecycle)]
use scoped_lifecycle::lifecycle::original_lifecycle_step;

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 4,
        maximum_copy_bytes: 64,
        maximum_capacity_bytes: 64,
        maximum_release_bytes: 64,
        maximum_depth: 4,
    }
}

#[test]
fn a_phase_copy_loan_protects_the_owner_before_its_effect() {
    let mut sequence = 0;
    let mut ledger = RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(16, 100, grant()), root_cancel_token(), || Some(1), &mut sequence, &mut ledger);
    let phase = RetainedCloneGrant { maximum_copy_bytes: 3, ..grant() };
    let quote = RetirementDemand { copy_bytes: 4, depth: 1, ..Default::default() };
    let mut owner = Some([1, 2, 3, 4]);
    let step = original_lifecycle_step(&mut cx, phase, RetirementDemand::default(), |same, _| {
        if !same.retained_work_can_enter(quote) {
            return Ok((PluginLifecycleStep::Progress(Default::default()), false));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 4, ..Default::default() };
        same.consume_retained(progress)?;
        owner.take().unwrap();
        Ok((PluginLifecycleStep::Progress(progress), false))
    });
    assert_eq!(owner, Some([1, 2, 3, 4]), "phase refusal arrived after the retained owner changed");
    assert_eq!(cx.retained_progress(), RetainedCloneProgress::default());
    assert_eq!(step.unwrap(), PluginLifecycleStep::Progress(Default::default()));
}

#[test]
fn an_explicitly_offered_two_item_phase_can_post_both_items() {
    let mut sequence = 0;
    let mut ledger = RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(16, 100, grant()), root_cancel_token(), || Some(1), &mut sequence, &mut ledger);
    let phase = RetainedCloneGrant { maximum_items: 2, ..grant() };
    let progress = RetainedCloneProgress { copied_items: 2, ..Default::default() };
    let step = original_lifecycle_step(&mut cx, phase, RetirementDemand::default(), |same, _| {
        same.consume_retained(progress)?;
        Ok((PluginLifecycleStep::Progress(progress), false))
    });
    assert_eq!(step.unwrap(), PluginLifecycleStep::Progress(progress));
    assert_eq!(cx.retained_progress(), progress);
}

#[test]
fn a_one_item_phase_refuses_a_second_debit_without_hiding_observed_spend() {
    let mut sequence = 0;
    let mut ledger = RetainedCloneProgress::default();
    let mut cx = StepContext::new(OperationId(1), Generation(1), StepBudget::new(16, 100, grant()), root_cancel_token(), || Some(1), &mut sequence, &mut ledger);
    let phase = RetainedCloneGrant { maximum_items: 1, ..grant() };
    let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
    let step = original_lifecycle_step(&mut cx, phase, RetirementDemand::default(), |same, selected| {
        assert_eq!(same.retained_grant().maximum_items, selected.maximum_items);
        same.consume_retained(item)?;
        assert!(same.consume_retained(item).is_err());
        Ok((PluginLifecycleStep::Progress(item), false))
    });
    assert!(step.is_err());
    assert_eq!(cx.retained_progress().copied_items, 2);
}
