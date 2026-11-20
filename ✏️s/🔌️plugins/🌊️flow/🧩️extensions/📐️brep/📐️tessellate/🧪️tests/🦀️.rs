//! 🧪️ Two native retained callers share the original tessellation lane.

use super::*;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_plugin::ExtensionInvokeStep;

fn stock() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 65_536, maximum_copy_bytes: 1 << 23, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 }
}

fn turn(resources: &TessellationResources, session: &SessionCapture, bytes: &[u8], caller: &mut OriginalJobFixtureCaller, grant: RetainedCloneGrant, waiting: bool) -> ExtensionInvokeStep {
    let result = caller.refreshed_turn(grant, |cx| {
        let fuel = cx.fuel_remaining();
        let result = resources.invoke(session, bytes, cx).unwrap();
        if waiting { assert_eq!(cx.fuel_remaining(), fuel); }
        Ok(result)
    }).unwrap();
    assert_eq!(result.retained_progress, caller.business_receipt());
    assert!(result.retained_progress.fits(grant));
    result
}

#[test]
fn original_tessellation_waiter_retries_without_spending_or_stealing_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️interleaved-tessellation.json")).unwrap();
    let requests = fixture["requests"].as_array().unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let session = Session::new();
        let handles = session.with_kernel(|kernel| requests.iter().map(|request| {
            let [start, end]: [[f64;3];2] = serde_json::from_value(request["line"].clone()).unwrap();
            kernel.line_curve_sync(start, end).map_err(|error| semio_s_spatial_kernel_semio_session::map_kernel_error(&error))
        }).collect::<Result<Vec<_>, _>>()).unwrap();
        let capture = session.capture();
        let inputs: Vec<Vec<u8>> = requests.iter().zip(&handles).map(|(request, handle)| {
            serde_json::to_vec(&serde_json::json!({
                "handle": handle.as_str(), "tolerance": fixture["tolerance"],
                "nodeHash": request["nodeHash"], "windowId": request["windowId"], "windowKindId": request["windowKindId"]
            })).unwrap()
        }).collect();
        let resources = TessellationResources::new();
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 128, ..stock() };
        let first = OriginalJobFixtureCaller::admit(OperationId(8710), Generation(7), stock(), || Some(1000), 1500, 1_000_000);
        let second = OriginalJobFixtureCaller::admit(OperationId(8711), Generation(7), stock(), || Some(1000), 1500, 1_000_000);
        let mut callers = [first, second];
        let mut done = [false; 2];
        let mut resumed = [false; 2];
        let mut cancelled_active = false;
        assert_eq!(turn(&resources, &capture, &inputs[0], &mut callers[0], grant, false).retained_progress, item());
        let cancelled = case["cancel"].as_u64().map(|value| value as usize);
        if cancelled == Some(1) { callers[1].cancel().cancel_now(); }
        let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
        let parked = turn(&resources, &capture, &inputs[1], &mut callers[1], denied, true);
        assert_eq!(parked.retained_progress, Default::default());
        assert!(parked.refusal.is_none() && parked.payload.is_none());
        for _ in 0..20_000 {
            for i in 0..2 {
                if done[i] { continue; }
                if i == 0 && cancelled == Some(0) && !cancelled_active {
                    let state = resources.cursor.lock().unwrap();
                    if state.as_ref().is_some_and(|source| source.query_active) {
                        callers[0].cancel().cancel_now();
                        cancelled_active = true;
                    }
                }
                let result = turn(&resources, &capture, &inputs[i], &mut callers[i], grant, i == 1 && !done[0]);
                if i == 1 && !done[0] {
                    assert_eq!(result.retained_progress, Default::default());
                    assert!(result.payload.is_none());
                    if cancelled != Some(1) { assert!(result.refusal.is_none()); }
                }
                if let Some(failure) = result.refusal {
                    assert_eq!(cancelled, Some(i), "unexpected original refusal {failure:?}");
                    assert_eq!(failure.primary_value().unwrap().kind, ValueRefusalKind::Canceled);
                    done[i] = true;
                } else if let Some(payload) = result.payload {
                    assert_ne!(cancelled, Some(i));
                    for expected in [handles[i].as_str(), requests[i]["windowId"].as_str().unwrap(), requests[i]["windowKindId"].as_str().unwrap()] {
                        assert!(payload.windows(expected.len()).any(|bytes| bytes == expected.as_bytes()));
                    }
                    done[i] = true;
                } else if result.retained_progress != RetainedCloneProgress::default() {
                    resumed[i] = true;
                }
            }
            if done == [true;2] { break; }
        }
        assert_eq!(done, [true;2]);
        assert!(resumed[0]);
        assert_eq!(resumed[1], cancelled != Some(1));
        assert_eq!(cancelled_active, cancelled == Some(0));
        assert!(resources.terminal_is_empty());
        for caller in callers { caller.retire(stock()); }
    }
}
