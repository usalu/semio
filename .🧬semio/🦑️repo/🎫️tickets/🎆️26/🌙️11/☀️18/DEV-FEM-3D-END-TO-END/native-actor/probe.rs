use semio_framework::kernel::{
    ActivationEvent, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, AppInstanceId, Budget, CommandIngressStatus, CommandPageCursor, Effect, Event, FixedCommandPage, MessageEndpoint,
    OriginalJobBudget, PluginInstanceId, TurnResult,
};
use semio_framework_actor::typed_operation_page::TypedOperationResultPage;
use semio_framework_actor::{OriginalActorTurnWork, RetainedTurnInput, RetainedTurnReceipt};
use semio_framework_value::{FromValue, RetainedCloneGrant, RetainedCloneProgress};
use semio_hub_fem::FemApps;
use semio_plugin::plugin_runtime::{LAST_MAINTENANCE_STAGE, PluginRuntime, install_plugin_bundle_result};
use semio_plugin::reactor::{original_actor_context::OriginalActorGuestEntry, poll_kernel};
use std::mem::ManuallyDrop;
use std::sync::atomic::Ordering;

const MAINTENANCE_NOT_ENTERED: u64 = u64::MAX - 1;

const BROWSER_SOURCE: RetainedTurnInput = RetainedTurnInput {
    operation: 1,
    generation: 1,
    epoch: 0,
    work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: 3_608_101_800, fuel: 50_000_000 },
    grant: RetainedCloneGrant { maximum_items: 8 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 512 << 20, maximum_release_bytes: 512 << 20, maximum_depth: 64 },
};

fn clock() -> Option<u64> {
    Some(1)
}

fn next_budget(source: &mut RetainedTurnInput) -> OriginalJobBudget {
    source.epoch += 1;
    OriginalJobBudget { input: *source, retained_policy: source.grant, fuel: source.work.fuel, deadline_us: source.work.absolute_deadline_us, cancelled: false }
}

fn receive(source: &mut RetainedTurnInput, receipt: RetainedTurnReceipt) {
    receipt.validate_for(*source).expect("original receipt validation");
    source.grant = receipt.remaining;
    source.work.fuel = receipt.remaining_fuel;
}

fn turn(runtime: &PluginRuntime<FemApps>, actor: &mut OriginalActorGuestEntry, source: &mut RetainedTurnInput, event: &mut Option<Event>, page: &mut Option<(CommandPageCursor, FixedCommandPage)>) -> TurnResult {
    let policy = next_budget(source);
    let result = poll_issued(runtime, actor, policy, event, page);
    receive(source, result.retained_receipt);
    result
}

fn poll_issued(runtime: &PluginRuntime<FemApps>, actor: &mut OriginalActorGuestEntry, policy: OriginalJobBudget, event: &mut Option<Event>, page: &mut Option<(CommandPageCursor, FixedCommandPage)>) -> TurnResult {
    let input = policy.input;
    let epoch = input.epoch;
    let budget = Budget::from_retained(input, policy.fuel, 64, 1 << 20, 8);
    let mut spent = RetainedCloneProgress::default();
    let mut identity = semio_plugin::artifact_app_laws::fixture_identity();
    let trace = std::env::var_os("SEMIO_ACTOR_REPLAY_CLEANUP").is_some();
    if trace {
        LAST_MAINTENANCE_STAGE.store(MAINTENANCE_NOT_ENTERED, Ordering::Relaxed);
    }
    let call = actor.dispatch_value_step(input, policy.step_budget(), &mut spent, clock, |context| semio_plugin::plugin_runtime::drive_self_waking_ready(poll_kernel(runtime, event, page, None, budget, &mut identity, context)));
    if trace {
        trace_cleanup(runtime, epoch, call.result.is_err());
    }
    call.input.return_original(call.receipt_spent, call.work_units_spent).expect("aggregate actor receipt");
    let result = call.result.unwrap_or_else(|failure| {
        if let Some(cause) = actor.original_job_refusal() {
            eprintln!("[DEBUG] native FEM actor failure epoch={} primary={} receipt={:?} context={:?} work={:?}", epoch, cause.primary.message(), cause.receipt_refusal, cause.context_refusal, cause.work_refusal);
        }
        panic!("actor epoch {epoch}: {failure:?}")
    });
    result.retained_receipt.validate_for(input).expect("kernel receipt");
    result
}

fn trace_cleanup(runtime: &PluginRuntime<FemApps>, epoch: u64, failed: bool) {
    let instances = runtime.instances.borrow();
    let Some(cell) = instances.get(1) else { return };
    let Ok(mut pump) = cell.maintenance_pump.try_lock() else { return };
    let marker = LAST_MAINTENANCE_STAGE.load(Ordering::Relaxed);
    let stage = (marker != MAINTENANCE_NOT_ENTERED).then_some(marker);
    let stalled = cell.maintenance_stalled_steps.load(Ordering::Relaxed);
    let status = cell.maintenance_status.load(Ordering::Relaxed);
    let job = pump.session.as_mut().and_then(|session| session.checked_out_job_mut());
    let progress = job.and_then(|job| job.progress);
    if failed || stage.is_some() || (stalled != 0 && progress.is_some()) {
        eprintln!("[DEBUG] maintenance epoch={epoch} generic_stage={stage:?} stalled={stalled} status={status} outcome_pending={} resume_pending={} progress={progress:?}", pump.outcome_pending, pump.resume_pending,);
    }
}

fn word(value: &serde_json::Value, field: &str) -> u64 {
    let value = &value[field];
    value.as_u64().or_else(|| value.as_str().and_then(|text| text.parse().ok())).unwrap()
}

fn command_page(row: &serde_json::Value) -> (CommandPageCursor, FixedCommandPage) {
    let fields = &row["cursor"];
    let word = |field| word(fields, field);
    let cursor = CommandPageCursor {
        owner: word("owner"),
        generation: word("generation"),
        command_index: word("commandIndex") as _,
        command_count: word("commandCount") as _,
        instance: word("instance") as _,
        seq: word("seq"),
        kind: word("kind") as _,
        page_index: word("pageIndex") as _,
        page_count: word("pageCount") as _,
        item_count: word("itemCount") as _,
        metadata: word("metadata") as _,
    };
    let text = row["hex"].as_str().unwrap();
    let bytes = (0..text.len()).step_by(2).map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap()).collect::<Vec<_>>();
    (cursor, FixedCommandPage::try_copy_from(&bytes).unwrap())
}

fn browser_event(row: &serde_json::Value) -> Event {
    let payload = &row["payload"];
    let text = |field: &str| payload[field].as_str().unwrap().to_owned();
    match row["kind"].as_str().unwrap() {
        "activate" => Event::Activate { reason: ActivationEvent::OnArtifactKind { kind: payload["reason"]["val"].as_str().unwrap().into() } },
        "instance-open" => Event::InstanceOpen {
            request: ActorInstanceOpenRequest { activation_generation: word(payload, "activationGeneration"), instance_id: word(payload, "instance") as _, request_sequence: word(payload, "requestSequence") },
            app_id: AppInstanceId(text("appId")),
            actor: text("actor"),
            config: serde_json::from_value(payload["config"].clone()).unwrap(),
            assets: serde_json::from_value(payload["assets"].clone()).unwrap(),
            capabilities: Vec::new(),
            quotas: semio_os_kernel::pack_rt::decode_wire_value(&serde_json::from_value::<Vec<u8>>(payload["quotas"].clone()).unwrap()).ok().and_then(|value| FromValue::from_value(value).ok()).unwrap_or_default(),
        },
        "instance-lifecycle-ack" => {
            let receipt = &payload["receipt"];
            let lifetime = &receipt["lifetime"];
            Event::InstanceLifecycleAck(ActorInstanceLifecycleAck {
                receipt: ActorInstanceLifecycleReceipt::Captured {
                    lifetime: ActorInstanceLifetime { activation_generation: word(lifetime, "activationGeneration"), instance_id: word(lifetime, "instanceId") as _, guest_lifetime: word(lifetime, "guestLifetime") },
                    request_sequence: word(receipt, "requestSequence"),
                },
            })
        }
        "message" => {
            let source = &payload["source"];
            assert_eq!(source["tag"], "shell");
            Event::Message { source: MessageEndpoint::Shell { instance: PluginInstanceId(word(source, "val").to_string()) }, payload: serde_json::from_value(payload["payload"].clone()).unwrap() }
        }
        other => panic!("unexpected browser event {other}"),
    }
}

fn browser_grant(row: &serde_json::Value) -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: word(row, "maximumItems") as usize,
        maximum_copy_bytes: word(row, "maximumCopyBytes") as usize,
        maximum_capacity_bytes: word(row, "maximumCapacityBytes") as usize,
        maximum_release_bytes: word(row, "maximumReleaseBytes") as usize,
        maximum_depth: word(row, "maximumDepth") as usize,
    }
}

fn browser_budget(row: &serde_json::Value) -> OriginalJobBudget {
    let input = &row["input"];
    let work = &input["work"];
    OriginalJobBudget {
        input: RetainedTurnInput {
            operation: word(input, "operation"),
            generation: word(input, "generation"),
            epoch: word(input, "epoch"),
            work: OriginalActorTurnWork { clock_binding: word(work, "clockBinding"), absolute_deadline_us: word(work, "absoluteDeadlineUs"), fuel: word(work, "fuel") },
            grant: browser_grant(&input["grant"]),
        },
        retained_policy: browser_grant(&row["retainedPolicy"]),
        fuel: word(row, "fuel"),
        deadline_us: word(row, "deadlineUs"),
        cancelled: row["cancelled"].as_bool().unwrap(),
    }
}

fn replay_captured_polls(runtime: &PluginRuntime<FemApps>, actor: &mut OriginalActorGuestEntry, source: &mut RetainedTurnInput, capture: &str) {
    let lines = capture.lines().take_while(|line| !line.starts_with("[DEBUG] guest poll refusal "));
    let json = |line: &str| serde_json::from_str::<serde_json::Value>(line).expect("captured JSON row");
    let rows = lines.clone().filter_map(|line| line.strip_prefix("[DEBUG] guest grant ")).map(json);
    let inputs = lines.filter_map(|line| line.strip_prefix("[DEBUG] guest [DEBUG] command ")).map(json);
    let mut rows = rows.collect::<Vec<_>>();
    let mut inputs = inputs.into_iter();
    rows.sort_unstable_by_key(|row| word(&row["slice"]["input"], "epoch"));
    let mut messages = 0;
    let mut command_complete = false;
    let mut load_document = false;
    let mut published = std::collections::HashMap::<[u8; 25], u8>::new();
    let mut lifecycle = None;
    let mut last_epoch = 0;
    for row in rows {
        let mut policy = browser_budget(&row["slice"]);
        policy.input.grant = policy.input.grant.restricted_by(source.grant);
        policy.input.work.fuel = policy.input.work.fuel.min(source.work.fuel);
        policy.retained_policy = policy.retained_policy.restricted_by(policy.input.grant);
        policy.fuel = policy.fuel.min(policy.input.work.fuel);
        *source = policy.input;
        let epoch = policy.input.epoch;
        assert!(epoch > last_epoch, "duplicate captured poll at epoch {epoch}");
        last_epoch = epoch;
        let events = row["events"].as_array().unwrap();
        let mut event = None;
        let mut page = None;
        let mut ack_lane = None;
        if let Some(kind) = events.first() {
            let input = inputs.next().expect("captured event payload");
            let input = &input["events"][0];
            assert_eq!(&input["kind"], kind, "captured event at epoch {epoch}");
            event = Some(browser_event(input));
            if let Some(Event::InstanceLifecycleAck(ack)) = &event {
                assert_eq!(lifecycle, Some(ack.receipt));
            }
            if kind == "message" {
                messages += 1;
                let Some(Event::Message { payload, .. }) = &event else { unreachable!() };
                let token: &[u8; 25] = payload.strip_prefix(TypedOperationResultPage::RENDERER_ACK_MAGIC).expect("captured renderer ACK").try_into().unwrap();
                let lane = published.remove(token).unwrap_or_else(|| panic!("native actor had not emitted renderer receipt {} at epoch {epoch}", messages - 1));
                assert_eq!(lane, [6, 8, 10][messages - 1], "captured typed result lane at epoch {epoch}");
                ack_lane = Some(lane);
            }
            println!("Captured browser poll epoch {epoch} event {kind}");
        } else if row["command"].is_object() {
            let input = inputs.next().expect("captured command bytes");
            assert_eq!(input["cursor"], row["command"], "captured cursor at epoch {epoch}");
            page = Some(command_page(&input));
            println!("Captured browser poll epoch {epoch} command {}", input["cursor"]["seq"]);
        }
        let result = poll_issued(runtime, actor, policy, &mut event, &mut page);
        receive(source, result.retained_receipt);
        if let Some(lane) = ack_lane {
            assert!(event.is_none(), "typed result ACK remained unconsumed at epoch {epoch}");
            println!("Native FEM 3D accepted renderer ACK for lane {lane} at epoch {epoch}");
        }
        lifecycle = result.lifecycle_receipt.or(lifecycle);
        load_document |= result.effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. }));
        for effect in &result.effects {
            if let Effect::SendMessage { payload, .. } = effect {
                if let Some(body) = payload.strip_prefix(TypedOperationResultPage::RENDERER_PAGE_MAGIC) {
                    published.insert(body[..25].try_into().unwrap(), body[25]);
                }
            }
        }
        match &result.command_ingress {
            CommandIngressStatus::CommandComplete(done) if done.seq == 7 => {
                command_complete = true;
                println!("Native FEM 3D command 7 reached completion at actor epoch {epoch}");
            }
            CommandIngressStatus::Fault { cursor, fault } => panic!("command {} epoch {epoch}: {}", cursor.seq, String::from_utf8_lossy(fault.as_slice())),
            _ => {}
        }
    }
    assert_eq!(messages, 3);
    assert!(command_complete, "captured polls did not complete setActiveExample ingress");
    assert!(load_document, "captured polls did not publish LoadDocument");
    println!("Native replay accepted all captured polls through epoch {last_epoch}");
}

fn main() {
    let capture = std::env::args_os().nth(1);
    std::thread::Builder::new().stack_size(64 << 20).spawn(move || replay(capture)).expect("original actor replay thread").join().unwrap();
}

fn replay(capture: Option<std::ffi::OsString>) {
    let capture = capture.map(|path| std::fs::read_to_string(path).expect("browser poll capture"));
    let runtime = ManuallyDrop::new(PluginRuntime::<FemApps>::new(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).unwrap());
    let mut actor = ManuallyDrop::new(OriginalActorGuestEntry::new());
    let mut source = BROWSER_SOURCE;
    if let Some(capture) = &capture {
        let first = capture.lines().find_map(|line| line.strip_prefix("[DEBUG] guest grant ")).expect("first captured browser poll");
        let row = serde_json::from_str::<serde_json::Value>(first).expect("first captured budget");
        let input = browser_budget(&row["slice"]).input;
        source.operation = input.operation;
        source.generation = input.generation;
        source.work.clock_binding = input.work.clock_binding;
        source.work.absolute_deadline_us = input.work.absolute_deadline_us;
    }
    let mut ready = false;
    for _ in 0..256 {
        let budget = next_budget(&mut source);
        let call = actor.admit_plugin_step(budget, clock, |_| {
            install_plugin_bundle_result(&runtime, semio_hub_fem::plugin());
            Ok(())
        });
        receive(&mut source, call.input.return_original(call.receipt_spent, call.work_units_spent).unwrap());
        if call.result.expect("actual guest admission").ready {
            ready = true;
            break;
        }
    }
    assert!(ready, "guest context admission remained pending");
    if let Some(capture) = &capture {
        replay_captured_polls(&runtime, &mut actor, &mut source, capture);
        return;
    }
    let transcript: serde_json::Value = serde_json::from_str(include_str!("../browser-seq7.json")).unwrap();
    let mut captured = None;
    for row in transcript.as_array().unwrap() {
        if let Some(input) = row["events"].as_array().unwrap().first() {
            let event = browser_event(input);
            if let Event::InstanceLifecycleAck(ack) = &event {
                assert_eq!(captured, Some(ack.receipt));
            }
            let opening = matches!(event, Event::InstanceOpen { .. });
            let mut event = Some(event);
            for _ in 0..16_384 {
                let result = turn(&runtime, &mut actor, &mut source, &mut event, &mut None);
                captured = result.lifecycle_receipt.or(captured);
                if event.is_none() && (!opening || captured.is_some()) {
                    break;
                }
            }
            assert!(event.is_none() && (!opening || captured.is_some()), "event {} remained pending", input["kind"]);
            println!("Native FEM 3D event {} reached actor epoch {}", input["kind"], source.epoch);
            continue;
        }
        let (cursor, bytes) = command_page(row);
        let sequence = cursor.seq;
        let mut page = Some((cursor, bytes));
        let mut complete = false;
        let mut load_document = false;
        for _ in 0..16_384 {
            let result = turn(&runtime, &mut actor, &mut source, &mut None, &mut page);
            load_document |= result.effects.iter().any(|effect| matches!(effect, Effect::LoadDocument { .. }));
            match &result.command_ingress {
                CommandIngressStatus::CommandComplete(done) if done.seq == sequence => {
                    complete = true;
                    break;
                }
                CommandIngressStatus::Fault { fault, .. } => panic!("command {sequence} epoch {}: {}", source.epoch, String::from_utf8_lossy(fault.as_slice())),
                _ => {}
            }
        }
        let epoch = source.epoch;
        assert!(complete, "command {sequence} did not complete at actor epoch {epoch}");
        if sequence == 7 {
            assert!(load_document, "setActiveExample did not emit LoadDocument");
        }
        println!("Native FEM 3D command {sequence} reached command completion at epoch {epoch}");
    }
}
