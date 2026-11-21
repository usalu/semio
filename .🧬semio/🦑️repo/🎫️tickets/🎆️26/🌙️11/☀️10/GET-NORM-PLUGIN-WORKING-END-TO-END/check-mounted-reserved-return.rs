//! 🧾 Exercises reserved return effects across the actual mounted lifecycle wrappers.

use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget, StepContext};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand};
use semio_plugin_mounted_owner::{MountedOwnerIdentityV1, MountedOwnerPhaseV1, MountedOwnerPolicyV1, MountedOwnerTurnV1};
use semio_plugin_ownership::bounded_ownership::{original_lifecycle_step, PluginLifecycleFailure, PluginLifecycleStep};

fn main() {
    let cases: Vec<serde_json::Value> = serde_json::from_str(include_str!(env!("SEMIO_RECEIPT_FIXTURE"))).unwrap();
    for (index, row) in cases.into_iter().enumerate() {
        let grant: RetainedCloneGrant = serde_json::from_value(row["grant"].clone()).unwrap();
        let fresh: RetainedCloneProgress = serde_json::from_value(row["fresh"].clone()).unwrap();
        let reserved: RetainedCloneProgress = serde_json::from_value(row["reserved"].clone()).unwrap();
        let total = fresh.checked_add(reserved).unwrap();
        for (mode, nested, redeems) in [("outer", false, true), ("nested-control", true, false), ("composed", true, true), ("nested", true, true)] {
            let identity = MountedOwnerIdentityV1 { instance_id: 7, operation: OperationId(99), generation: Generation(1) };
            let mut sequence = 0;
            let mut posted = RetainedCloneProgress::default();
            let mut cx = StepContext::new(identity.operation, identity.generation, StepBudget::new(10, 100, grant),
                root_cancel_token(), || Some(1), &mut sequence, &mut posted);
            let policy = MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant };
            let mut owner = MountedOwnerTurnV1::admit(7, identity, MountedOwnerPhaseV1::Close, policy, &mut cx).unwrap().unwrap();
            let spent = |cx: &mut StepContext<'_>| -> Result<(), PluginLifecycleFailure> {
                cx.consume_retained(fresh)?;
                cx.consume_fuel(1);
                if redeems { cx.receive_original_reserved_return(reserved, 1)?; }
                Ok(())
            };
            let produce = |cx: &mut StepContext<'_>, loan| -> Result<PluginLifecycleStep, PluginLifecycleFailure> {
                if nested {
                    let step = original_lifecycle_step(cx, loan, RetirementDemand::default(), |cx, _| {
                        spent(cx)?;
                        Ok((PluginLifecycleStep::Progress(fresh), false))
                    })?;
                    if mode == "composed" {
                        Ok(PluginLifecycleStep::Progress(step.progress().unwrap().checked_add(reserved)?))
                    } else {
                        Ok(step)
                    }
                } else {
                    spent(cx)?;
                    Ok(PluginLifecycleStep::Progress(total))
                }
            };
            #[cfg(mounted_receipt_current)]
            let result = owner.advance_lifecycle(PluginLifecycleStep::with_progress, produce);
            #[cfg(not(mounted_receipt_current))]
            let result = owner.advance_lifecycle(produce);
            let actual = owner.context().retained_progress();
            let redeemed = owner.context().reserved_return_progress();
            assert_eq!(actual, if redeems { total } else { fresh });
            assert_eq!(redeemed, if redeems { reserved } else { RetainedCloneProgress::default() });
            let step = result.unwrap_or_else(|error| panic!(
                "fixture {index}, {mode}: mounted lifecycle rejected its receipt {actual:?}: {error:?}"
            ));
            assert_eq!(step.progress(), Some(actual));
            assert_eq!(owner.grant(), serde_json::from_value(row["remaining"].clone()).unwrap());
            println!("[DEBUG] fixture {index}, {mode}: accepted {actual:?}, reserved {redeemed:?}");
        }
    }
}
