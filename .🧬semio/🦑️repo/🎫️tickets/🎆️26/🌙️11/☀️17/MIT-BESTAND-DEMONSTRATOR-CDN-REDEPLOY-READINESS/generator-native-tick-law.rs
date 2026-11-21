use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, StepContext};
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunState, ToolRunTick, ToolRunTraceOp, ToolRunTracePage, ToolRunTraceStore, ToolRunTraceSubject, ToolRunVerdict};
use semio_framework_tool_run_tick_wire::tick_receive::OriginalTickWireSource;
use semio_framework_value::retirement::controlled::ControlledRetirement;
use semio_framework_value::RetainedCloneGrant;
use native::OriginalToolRunNativeState;

struct Wire(Vec<u8>);

impl OriginalTickWireSource for Wire {
    fn identity(&self) -> usize { self.0.as_ptr() as usize }
    fn byte_len(&self) -> usize { self.0.len() }
    fn page(&self, index: usize) -> Option<&[u8]> { self.0.chunks(17).nth(index) }
}

#[test]
fn native_tick_receives_publishes_and_retires() {
    receive_publish_and_retire(false, None);
}

#[test]
fn native_tick_publishes_and_retires_an_appended_entity() {
    receive_publish_and_retire(true, None);
}

#[test]
fn native_tick_keeps_the_same_trace_pages_until_the_receiver_completes() {
    receive_publish_and_retire(false, Some(false));
}

#[test]
fn native_tick_closes_while_its_trace_receiver_is_pending() {
    receive_publish_and_retire(false, Some(true));
}

fn receive_publish_and_retire(append_entity: bool, cancel_trace: Option<bool>) {
    let grant = RetainedCloneGrant {
        maximum_items: 1 << 10,
        maximum_copy_bytes: 1 << 20,
        maximum_capacity_bytes: 1 << 20,
        maximum_release_bytes: 1 << 20,
        maximum_depth: 64,
    };
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 20,
        maximum_copy_bytes: 1 << 28,
        maximum_capacity_bytes: 1 << 28,
        maximum_release_bytes: 1 << 28,
        ..grant
    };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(876), Generation(1), stock, || Some(1), u64::MAX, 100_000);
    caller.admit_return_table();
    let mut decoder = caller.admitted_decoder();
    caller.retain_dispatch().unwrap();
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]);
    let mut state = OriginalToolRunNativeState::empty();
    let mut initialized = false;
    for _ in 0..1024 {
        if caller.refreshed_turn_with_decoder(grant, &mut decoder, |cx| state.initialize("generator", identity, ToolRunState::Starting, cx)).unwrap() {
            initialized = true;
            break;
        }
    }
    assert!(initialized);
    let trace_op = ToolRunTraceOp::Upsert { key: 13, verdict: ToolRunVerdict::Success, reason: 4,
                                           subject: ToolRunTraceSubject::Entity { entity: 7 } };
    let trace = cancel_trace.map_or_else(Vec::new, |_| {
        vec![ToolRunTracePage { identity, page: 0, ops: vec![trace_op] }]
    });
    let tick = ToolRunTick {
        identity, sequence: 1, progress: None, steps: Vec::new(), trace,
        append_ops: Vec::new(), append_entities: if append_entity { vec![7] } else { Vec::new() }, retract_to: None,
        payload: None, payload_update: None,
    };
    let source = Wire(tick.encode().unwrap());
    let mut published = false;
    let mut domain_calls = 0;
    let mut trace_calls = 0;
    let mut applied = 0;
    let mut trace_address = None;
    let mut trace_store = ToolRunTraceStore::new(identity);
    let mut receive = |state: &mut OriginalToolRunNativeState, cx: &mut StepContext<'_>| {
        state.receive_with_domain(&source, identity, ToolRunState::Running, |_, _| {
            domain_calls += 1;
            Ok(Some(0))
        }, |pages, cursor, cx| {
            assert_eq!(pages.len(), 1);
            assert_eq!(pages[0].ops.as_slice(), &[trace_op]);
            let address = pages.as_ptr();
            assert_eq!(*trace_address.get_or_insert(address), address);
            trace_calls += 1;
            let demand = trace_store.next_funded_apply_demand(cursor, pages)?;
            if !cx.retained_work_can_enter(demand) {
                return Ok(false);
            }
            let Some((step, performed)) = cx.with_original_source_effect(cx.retained_grant(), |_, effect| {
                trace_store.step_funded_apply(cursor, pages, effect)
            })? else { return Ok(false) };
            let step = step?;
            if let Some(op) = step.applied {
                assert_eq!(op, trace_op);
                applied += 1;
            }
            cx.consume_retained(step.progress)?;
            cx.consume_fuel(u64::try_from(performed).unwrap());
            Ok(step.complete)
        }, cx)
    };
    for step in 0..4096 {
        if caller.refreshed_turn_with_decoder(grant, &mut decoder, |cx| receive(&mut state, cx)).unwrap() {
            published = true;
            break;
        }
        if cancel_trace == Some(true) && state.fixture_diagnostic().0 == 6 {
            caller.refreshed_turn_with_decoder(grant, &mut decoder, |cx| receive(&mut state, cx)).unwrap();
            state.request_close();
            break;
        }
        if state.fixture_diagnostic().0 == 5
            && state.fixture_cursor_close_demand(grant.maximum_copy_bytes).is_none()
        {
            eprintln!("[DEBUG] native cursor empty at step {step}");
        }
    }
    if !published && cancel_trace != Some(true) {
        let diagnostic = (state.fixture_diagnostic(), state.fixture_cursor_close_demand(grant.maximum_copy_bytes),
                          caller.business_receipt());
        std::mem::forget((state, decoder, caller));
        panic!("[DEBUG] native phase, cursor, outcome, stepped, tick, sequence, ready, frame and receipt: {diagnostic:?}");
    }
    if published {
        let mut received = false;
        let denied = RetainedCloneGrant { maximum_items: 0, ..grant };
        assert!(!caller.turn(denied, |cx| state.acknowledge_received(&mut received, cx)).unwrap());
        assert!(!received);
        assert!(caller.refreshed_turn_with_decoder(grant, &mut decoder, |cx| receive(&mut state, cx)).unwrap());
        assert!(caller.turn(grant, |cx| state.acknowledge_received(&mut received, cx)).unwrap());
        assert!(received);
        let view = state.borrowed_view(identity, ToolRunState::Running).expect("published native presentation");
        assert_eq!(view.progress.sequence, 1);
        assert_eq!(view.provisional_entities.len(), usize::from(append_entity));
        if append_entity {
            assert_eq!(*view.provisional_entities.get_index(0).unwrap().0, 7);
        }
    } else {
        assert!(state.borrowed_view(identity, ToolRunState::Running).is_none());
    }
    assert_eq!(domain_calls, 1);
    match cancel_trace {
        Some(false) => {
            assert!(trace_calls > 1);
            assert_eq!(applied, 1);
            assert_eq!(trace_store.record(13).unwrap().verdict, ToolRunVerdict::Success);
        }
        Some(true) => {
            assert_eq!(trace_calls, 1);
            assert_eq!(applied, 0);
            assert!(trace_store.is_empty());
        }
        None => assert_eq!(trace_calls, 0),
    }
    for _ in 0..1024 {
        if state.terminal_is_empty() { break; }
        caller.refreshed_turn_with_decoder(grant, &mut decoder, |cx: &mut StepContext<'_>| cx.with_original_retirement(|cx| {
            let demand = state.next_close_demands(cx.retained_grant().maximum_copy_bytes)?;
            state.close_with(cx, demand).map(|_| ())
        })).unwrap();
    }
    assert!(state.terminal_is_empty());
    let mut trace_close = ControlledRetirement::new(trace_store).unwrap_or_else(|(error, _)| panic!("{error}"));
    for _ in 0..1024 {
        if trace_close.terminal_is_empty() { break; }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            let body = cx.retained_grant().maximum_copy_bytes;
            let demand = trace_close.selected_demand(body)?;
            semio_framework_job::close_original_effect_step(cx, demand, |effect| trace_close.step(body, effect)).map(|_| ())
        })).unwrap();
    }
    assert!(trace_close.terminal_is_empty());
    for _ in 0..1024 {
        if decoder.terminal_is_empty() { break; }
        caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
            decoder.close_on_source(cx).map(|_| ())
        })).unwrap();
    }
    assert!(decoder.terminal_is_empty());
    caller.retire(grant);
}
