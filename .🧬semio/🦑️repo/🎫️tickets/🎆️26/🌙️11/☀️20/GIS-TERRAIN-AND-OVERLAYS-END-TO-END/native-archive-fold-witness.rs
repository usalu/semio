use semio_framework_job::{Generation, OperationId, StepContext};
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::{RetainedCloneGrant, RetirementDemand, ValueError};
use semio_os_spr::{HistoryShape, decode_document_archive_bytes, decode_history};
use std::future::Future;
use std::sync::Arc;

include!("fold-quote.rs");

fn poll_to_completion<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..16_384 {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
    panic!("archive input decode exceeded its turn bound");
}

fn trace(bytes: &[u8]) {
    let archive = poll_to_completion(decode_document_archive_bytes(bytes)).unwrap();
    let history = Arc::new(poll_to_completion(decode_history(&archive.parent_spr, &semio_os_spr::DecodeOptions::default())).unwrap());
    let copy = std::env::args().nth(2).unwrap().parse::<usize>().unwrap();
    let unit = RetainedCloneGrant {
        maximum_items: 16, maximum_copy_bytes: copy, maximum_capacity_bytes: 16_777_216,
        maximum_release_bytes: 1_048_576, maximum_depth: 64,
    };
    let total = RetainedCloneGrant {
        maximum_items: 8_000_000, maximum_copy_bytes: 2_022_000_000,
        maximum_capacity_bytes: 500_000_000, maximum_release_bytes: 500_000_000, maximum_depth: 64,
    };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(111), Generation(1), total, || Some(0), u64::MAX, 1 << 24);
    let mut decoder = caller.admitted_decoder();
    let setup = RetainedCloneGrant { maximum_items: 256, maximum_copy_bytes: 1_048_576, ..unit };
    let mut ready = false;
    for _ in 0..64 {
        ready = caller.refreshed_turn_with_decoder(setup, &mut decoder, |cx| cx.with_native_receiving(&mut None, |_, _| Ok(()))).unwrap();
        if ready { break; }
    }
    assert!(ready, "fixture receiving prerequisites must be paid");
    let mut job: Option<FoldJob> = None;
    let mut output = None;
    const TURNS: usize = 65_536;
    for turn in 1usize..=TURNS {
        let detailed = turn < 8 || turn.is_power_of_two();
        let outcome = caller.refreshed_turn_with_decoder(unit, &mut decoder, |cx| {
            let mut scope = None;
            if job.is_none() {
                let mut runner = Some(|control| {
                    let history = history.clone();
                    async move { history.fold_controlled(HistoryShape::Document, &control).await }
                });
                let demand = FoldJob::original_birth_demand(runner.as_ref().unwrap())?;
                let quote = fold_receiving_quote(cx, demand, 1).expect("birth quote fits");
                cx.with_native_receiving_quoted(&mut scope, quote, |native, _| {
                    let observed = FoldJob::admit_original(&mut runner, &mut job, native);
                    eprintln!("[DEBUG] birth demand={demand:?} work={} progress={:?} result={:?} refusal={:?}", observed.work, observed.progress, observed.result, observed.receipt_refusal);
                    assert!(matches!(observed.result, Some(Ok(true))) && observed.receipt_refusal.is_none());
                    Ok(())
                })?;
            } else {
                let current = job.as_mut().unwrap();
                let Some((body, grant, work)) = fold_step_quote(cx, current)? else {
                    if detailed { eprintln!("[DEBUG] turn={turn} no quote demand={:?} completed={}", current.original_step_demand(4096), current.completed()); }
                    return Ok(false);
                };
                cx.with_native_receiving_quoted(&mut scope, (grant, work), |native, _| {
                    if current.original_output_ready() {
                        let observed = current.take_original_output(&mut output, native);
                        eprintln!("[DEBUG] output turn={turn} work={} progress={:?} result={:?} refusal={:?}", observed.work, observed.progress, observed.result, observed.receipt_refusal);
                        assert!(matches!(observed.result, Some(Ok(true))) && observed.receipt_refusal.is_none());
                    } else {
                        assert!(!current.original_rejection_ready());
                        let observed = current.step_original(native, body);
                        if detailed {
                            eprintln!("[DEBUG] turn={turn} body={body} grant={grant:?} work={} progress={:?} completed={} result={:?} refusal={:?}",
                                observed.work, observed.progress, current.completed(), observed.result.as_ref().map(|result| result.as_ref().map(|_| ())), observed.receipt_refusal);
                        }
                        assert!(observed.result.as_ref().is_none_or(Result::is_ok) && observed.receipt_refusal.is_none());
                    }
                    Ok(())
                })?;
            }
            if let Some(scope) = scope {
                assert!(scope.source_refusal.is_none() && scope.call.is_ok());
                cx.settle_receiving_scope()?;
            }
            Ok(output.is_some())
        });
        if outcome.unwrap() {
            assert!(job.as_ref().unwrap().terminal_is_empty());
            drop(output);
            for _ in 0..128 {
                if decoder.terminal_is_empty() { break; }
                caller.refreshed_turn(setup, |cx| { decoder.close_on_source(cx)?; Ok(()) }).unwrap();
            }
            assert!(decoder.terminal_is_empty());
            caller.retire(unit);
            return;
        }
    }
    eprintln!("[DEBUG] {TURNS} turns did not complete; final count {}", job.as_ref().unwrap().completed());
    std::mem::forget(job);
    std::mem::forget(decoder);
    std::mem::forget(caller);
    std::process::exit(2);
}

fn main() {
    let bytes = std::fs::read(std::env::args_os().nth(1).unwrap()).unwrap();
    std::thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || trace(&bytes)).unwrap().join().unwrap();
}
