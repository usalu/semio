use semio_framework::kernel::{
    ActivationEvent, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime,
    ActorInstanceOpenRequest, AppInstanceId, Budget, CommandIngressStatus, CommandPageCursor,
    Event, FixedCommandPage, OriginalJobBudget, TurnResult,
};
use semio_framework_actor::{OriginalActorTurnWork, RetainedTurnInput};
use semio_framework_value::{FromValue, RetainedCloneGrant, RetainedCloneProgress};
use semio_hub_fem::FemApps;
use semio_plugin::plugin_runtime::{PluginRuntime, install_plugin_bundle_result};
use semio_plugin::reactor::{original_actor_context::OriginalActorGuestEntry, poll_kernel};
use std::mem::ManuallyDrop;

const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 8192,
    maximum_copy_bytes: 8 << 20,
    maximum_capacity_bytes: 8 << 20,
    maximum_release_bytes: 8 << 20,
    maximum_depth: 128,
};

const WORK: u64 = 16_384;

fn clock() -> Option<u64> { Some(1) }

fn next_input(epoch: &mut u64) -> RetainedTurnInput {
    *epoch += 1;
    RetainedTurnInput {
        operation: 1,
        generation: 1,
        epoch: *epoch,
        work: OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: u64::MAX, fuel: WORK },
        grant: GRANT,
    }
}

fn turn(
    runtime: &PluginRuntime<FemApps>,
    actor: &mut OriginalActorGuestEntry,
    epoch: &mut u64,
    event: &mut Option<Event>,
    page: &mut Option<(CommandPageCursor, FixedCommandPage)>,
) -> TurnResult {
    let input = next_input(epoch);
    let policy = OriginalJobBudget { input, retained_policy: GRANT, fuel: WORK, deadline_us: u64::MAX, cancelled: false };
    let budget = Budget::from_retained(input, WORK, 16, 65_536, 16);
    let mut spent = RetainedCloneProgress::default();
    let call = actor.dispatch_value_step(input, policy.step_budget(), &mut spent, clock, |context| {
        semio_plugin::__async::poll::resolve_ready(poll_kernel(runtime, event, page, None, budget, &mut semio_plugin::artifact_app_laws::fixture_identity(), context))
    });
    call.input.return_original(call.receipt_spent, call.work_units_spent).expect("original receipt validation");
    call.result.unwrap_or_else(|failure| panic!("actor epoch {}: {failure:?}; retained cause: {:?}", *epoch, actor.original_job_refusal()))
}

fn word(value: &serde_json::Value, field: &str) -> u64 {
    let value = &value[field];
    value.as_u64().or_else(|| value.as_str().and_then(|text| text.parse().ok())).unwrap()
}

fn command_page(row: &serde_json::Value) -> (CommandPageCursor, FixedCommandPage) {
    let fields = &row["cursor"];
    let word = |field| word(fields, field);
    let cursor = CommandPageCursor {
        owner: word("owner"), generation: word("generation"),
        command_index: word("commandIndex") as _, command_count: word("commandCount") as _,
        instance: word("instance") as _, seq: word("seq"), kind: word("kind") as _,
        page_index: word("pageIndex") as _, page_count: word("pageCount") as _,
        item_count: word("itemCount") as _, metadata: word("metadata") as _,
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
            request: ActorInstanceOpenRequest {
                activation_generation: word(payload, "activationGeneration"), instance_id: word(payload, "instance") as _, request_sequence: word(payload, "requestSequence"),
            },
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
            Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: ActorInstanceLifecycleReceipt::Captured {
                lifetime: ActorInstanceLifetime {
                    activation_generation: word(lifetime, "activationGeneration"), instance_id: word(lifetime, "instanceId") as _, guest_lifetime: word(lifetime, "guestLifetime"),
                },
                request_sequence: word(receipt, "requestSequence"),
            } })
        }
        other => panic!("unexpected browser event {other}"),
    }
}

fn main() {
    let runtime = ManuallyDrop::new(PluginRuntime::<FemApps>::new(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT).unwrap());
    let mut actor = ManuallyDrop::new(OriginalActorGuestEntry::new());
    let mut epoch = 0;
    let mut ready = false;
    for _ in 0..256 {
        let input = next_input(&mut epoch);
        let budget = OriginalJobBudget { input, retained_policy: GRANT, fuel: WORK, deadline_us: u64::MAX, cancelled: false };
        let call = actor.admit_plugin_step(budget, clock, |_| {
            install_plugin_bundle_result(&runtime, semio_hub_fem::plugin());
            Ok(())
        });
        call.input.return_original(call.receipt_spent, call.work_units_spent).unwrap();
        if call.result.expect("actual guest admission").ready {
            ready = true;
            break;
        }
    }
    assert!(ready, "guest context admission remained pending");
    let transcript: serde_json::Value = serde_json::from_str(include_str!("../browser-seq7.json")).unwrap();
    let mut captured = None;
    for row in transcript.as_array().unwrap() {
        if let Some(input) = row["events"].as_array().unwrap().first() {
            let event = browser_event(input);
            if let Event::InstanceLifecycleAck(ack) = &event { assert_eq!(captured, Some(ack.receipt)); }
            let opening = matches!(event, Event::InstanceOpen { .. });
            let mut event = Some(event);
            for _ in 0..16_384 {
                let result = turn(&runtime, &mut actor, &mut epoch, &mut event, &mut None);
                captured = result.lifecycle_receipt.or(captured);
                if event.is_none() && (!opening || captured.is_some()) { break; }
            }
            assert!(event.is_none(), "event {} remained pending", input["kind"]);
            continue;
        }
        let (cursor, bytes) = command_page(row);
        let mut page = Some((cursor, bytes));
        let mut complete = false;
        for _ in 0..16_384 {
            let result = turn(&runtime, &mut actor, &mut epoch, &mut None, &mut page);
            match &result.command_ingress {
                CommandIngressStatus::CommandComplete(done) if done.seq == cursor.seq => { complete = true; break; }
                CommandIngressStatus::Fault { fault, .. } => panic!("command {} epoch {epoch}: {}", cursor.seq, String::from_utf8_lossy(fault.as_slice())),
                _ => {}
            }
        }
        assert!(complete, "command {} did not complete at actor epoch {epoch}", cursor.seq);
        println!("Native FEM 3D command {} reached command completion at epoch {epoch}", cursor.seq);
    }
}
