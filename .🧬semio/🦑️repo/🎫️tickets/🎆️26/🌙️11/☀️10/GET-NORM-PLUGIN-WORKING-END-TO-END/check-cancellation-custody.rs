//! 🪢 Verify live and returned cancellation leases under the same original actor.
use semio_framework_actor_retained_context::OriginalActorContext;
use semio_framework_actor_retained_turn::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_job::{Generation, OperationId, RevisionId, StepBudget};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
use semio_plugin_operation::{ArtifactDocumentAuthority, ToolCancellationIssuer, ToolOperationKey,
    ToolCancellationLeaseRetirement, original_lease_cursor_admit, original_lease_cursor_release};

fn clock() -> Option<u64> { Some(1) }

fn main() {
    let mask = RetainedCloneGrant {
        maximum_items: 64, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 16_777_216,
        maximum_release_bytes: 1_048_576, maximum_depth: 64,
    };
    let mut input = RetainedTurnInput {
        operation: 1, generation: 1, epoch: 1,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: 1_000_000, fuel: 1 << 23 },
        grant: RetainedCloneGrant {
            maximum_items: 16384, maximum_copy_bytes: 1 << 25, maximum_capacity_bytes: 1 << 26,
            maximum_release_bytes: 1 << 26, maximum_depth: 64,
        },
    };
    let mut actor = OriginalActorContext::new((), input).unwrap();
    while {
        let budget = StepBudget::new(8192, input.work.absolute_deadline_us, input.grant);
        let ready = actor.admit_step(input, budget, clock).unwrap().ready;
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
        !ready
    } {}
    actor.take_input().unwrap();
    let mut issuer = ToolCancellationIssuer::empty();
    let key = ToolOperationKey { app_instance_id: 1, document: ArtifactDocumentAuthority(9),
        operation_id: OperationId(2), base_revision: RevisionId(1), generation: Generation(1) };
    let mut lease = None;
    let mut close: Option<Box<ToolCancellationLeaseRetirement>> = None;
    let mut published = false;
    let mut keyed = false;
    for _ in 0..1000 {
        actor.begin_dispatch(input).unwrap();
        let mut observed = RetainedCloneProgress::default();
        let budget = StepBudget::new(8192, input.work.absolute_deadline_us, input.grant);
        let mut cx = actor.borrow_context(input, budget, &mut observed, clock).unwrap();
        cx.with_retained_loan(mask, |cx| {
            if !issuer.ready() {
                issuer.admit_step(cx.original_source_parent()?, cx)?;
            } else if !published {
                assert!(!issuer.live_cleanup_awaits_custody());
                lease = if keyed { issuer.begin_keyed(&key, cx)? } else { issuer.begin(&key, cx)? };
                if lease.is_some() {
                    let before = cx.retained_progress();
                    let fuel = cx.fuel_remaining();
                    for _ in 0..257 {
                        assert!(issuer.live_cleanup_awaits_custody());
                        assert!(!issuer.cleanup_slot(0, cx, true)?);
                    }
                    assert_eq!(cx.retained_progress(), before);
                    assert_eq!(cx.fuel_remaining(), fuel);
                    published = true;
                }
            } else if let Some(owner) = close.as_mut().filter(|owner| !owner.terminal_is_empty()) {
                let handle = issuer.handle().unwrap();
                let demand = owner.retirement_demands(handle, mask.maximum_copy_bytes)?;
                semio_framework_job::close_original_effect_step(cx, demand, |effect| owner.close_step(handle, effect))?;
            } else {
                let progress = if close.is_some() { original_lease_cursor_release(&mut close, cx.retained_grant())? }
                    else { let (owner, progress) = original_lease_cursor_admit(&mut lease, cx.retained_grant())?; close = owner; progress };
                cx.consume_retained(progress)?;
                cx.consume_fuel(progress.copied_items as u64);
                if lease.is_none() && close.is_none() {
                    assert_eq!(issuer.active_operation_count(), 0);
                    assert!(!issuer.live_cleanup_awaits_custody());
                    if keyed {
                        println!("DISPLAY original keyed and unkeyed leases waited through 257 observations and released");
                        std::process::exit(0);
                    }
                    keyed = true;
                    published = false;
                }
            }
            Ok(())
        }).unwrap();
        drop(cx);
        input.epoch += 1;
        input.work = actor.remaining_work().unwrap();
        input.grant = actor.remaining_grant().unwrap();
    }
    panic!("original lease did not return");
}
