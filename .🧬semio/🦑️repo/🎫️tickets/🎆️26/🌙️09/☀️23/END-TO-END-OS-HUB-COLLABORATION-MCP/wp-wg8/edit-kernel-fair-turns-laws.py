#!/usr/bin/env python3
"""WG8 s12: the fairness contract (schema + fixture `turnFairness` of `🧵️kernel-pool-future`) and its laws for the
native kernel's fair turns (`edit-kernel-fair-turns.py`)."""
import json
import pathlib

ENGINE = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine")
SCHEMA = ENGINE / "🧬️schema/🧵️kernel-pool-future/🔣️.json"
FIXTURE = ENGINE / "🧫️fixtures/🧵️kernel-pool-future/🔣️.json"
LAWS = ENGINE / "🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs"

schema = json.loads(SCHEMA.read_text())
if "turnFairness" not in schema["required"]:
    schema["required"].append("turnFairness")
schema["properties"]["turnFairness"] = {
    "description": "⚖️ How the native kernel shares its request loop while a guest turn is mid-flight: a slice is `sliceFuel`/`sliceWallMs`; after each slice of a preempted turn at most `requestsBetweenSlices` queued requests run first when the head may (`gaps`); only an explicit cancel ends a turn — the requester dropping its request (`abandonment`) or a realm close at the head. Ticket 26/09/23 WG8.",
    "type": "object",
    "additionalProperties": False,
    "required": ["sliceFuel", "sliceWallMs", "requestsBetweenSlices", "gaps", "abandonment"],
    "properties": {
        "sliceFuel": {"type": "integer", "minimum": 1},
        "sliceWallMs": {"type": "integer", "minimum": 1},
        "requestsBetweenSlices": {"type": "integer", "minimum": 1},
        "gaps": {
            "type": "array",
            "minItems": 5,
            "items": {
                "type": "object",
                "additionalProperties": False,
                "required": ["id", "head", "headInstance", "busy", "served", "cancels"],
                "properties": {
                    "id": {"type": "string", "minLength": 1},
                    "head": {"enum": ["exchangeCommands", "destroyApp", "closeRealm", None]},
                    "headInstance": {"type": "integer", "minimum": 0},
                    "busy": {"type": "integer", "minimum": 0},
                    "served": {"type": "boolean"},
                    "cancels": {"type": "boolean"},
                },
            },
        },
        "abandonment": {
            "type": "array",
            "minItems": 3,
            "items": {
                "type": "object",
                "additionalProperties": False,
                "required": ["id", "admitted", "delivered", "abandoned"],
                "properties": {"id": {"type": "string", "minLength": 1}, "admitted": {"type": "boolean"}, "delivered": {"type": "boolean"}, "abandoned": {"type": "boolean"}},
            },
        },
    },
}
SCHEMA.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")

fixture = json.loads(FIXTURE.read_text())
fixture["turnFairness"] = {
    "sliceFuel": 50000000,
    "sliceWallMs": 100,
    "requestsBetweenSlices": 1,
    "gaps": [
        {"id": "another-instances-commands-run-in-the-gap", "head": "exchangeCommands", "headInstance": 2, "busy": 1, "served": True, "cancels": False},
        {"id": "the-busy-instances-own-commands-wait-for-its-turn", "head": "exchangeCommands", "headInstance": 1, "busy": 1, "served": False, "cancels": False},
        {"id": "a-close-waits-for-a-whole-request-boundary", "head": "destroyApp", "headInstance": 2, "busy": 1, "served": False, "cancels": False},
        {"id": "a-realm-close-cancels-the-mid-flight-turn", "head": "closeRealm", "headInstance": 0, "busy": 1, "served": False, "cancels": True},
        {"id": "an-empty-queue-leaves-the-turn-alone", "head": None, "headInstance": 0, "busy": 1, "served": False, "cancels": False},
    ],
    "abandonment": [
        {"id": "dropped-while-queued-is-a-cancel", "admitted": True, "delivered": False, "abandoned": True},
        {"id": "dropped-after-its-outcome-is-not", "admitted": True, "delivered": True, "abandoned": False},
        {"id": "dropped-before-admission-has-nothing-to-cancel", "admitted": False, "delivered": False, "abandoned": False},
    ],
}
FIXTURE.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")

laws = LAWS.read_text()
ANCHOR = """    fn command_request(instance: u32, generation: u64, page_count: usize) -> KernelRequest {"""
assert laws.count(ANCHOR) == 1
NEW = r'''    fn turn_fairness() -> serde_json::Value {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧵️kernel-pool-future/🔣️.json")).expect("neutral kernel fairness contract");
        fixture["turnFairness"].clone()
    }

    /// ⚖️ A slice is the contract's fuel and wall, and one slice gap serves the contract's number of requests.
    #[test]
    fn the_turn_slice_and_its_gap_are_the_fairness_contracts() {
        let fairness = turn_fairness();
        assert_eq!(TURN_BUDGET.fuel, fairness["sliceFuel"].as_u64().expect("slice fuel"));
        assert_eq!(u64::from(TURN_BUDGET.deadline_ms), fairness["sliceWallMs"].as_u64().expect("slice wall"));
        assert_eq!(TURN_REQUESTS_BETWEEN_SLICES as u64, fairness["requestsBetweenSlices"].as_u64().expect("requests between slices"));
    }

    /// ⚖️ Between two slices of a turn mid-flight on `busy`, the head request runs only when it belongs to another
    /// instance and owns its outcome; a close waits for a whole-request boundary; a realm close at the head
    /// cancels the turn; FIFO and command credits are untouched by a head that may not run.
    #[test]
    fn a_slice_gap_serves_only_another_instances_head_request_and_a_realm_close_cancels() {
        for gap in turn_fairness()["gaps"].as_array().expect("gaps") {
            let queue = KernelRequestQueue::default();
            let busy = gap["busy"].as_u64().expect("busy") as u32;
            let instance = gap["headInstance"].as_u64().expect("head instance") as u32;
            let head = match gap["head"].as_str() {
                Some("exchangeCommands") => Some(command_request(instance, 7, 1)),
                Some("destroyApp") => Some(destroy_request(instance)),
                Some("closeRealm") => Some(KernelRequest::CloseRealm { owner: close_submission(&Arc::new(KernelCloseSubmissionRegistry::new()), instance, 3) }),
                _ => None,
            };
            if let Some(head) = head {
                queue.try_push(head, Arc::new(ResponseSlot::default()), None).unwrap_or_else(|_| panic!("fairness gap admission"));
            }
            let cancels = queue.head_is(|request| matches!(request, KernelRequest::CloseRealm { .. }));
            let served = queue.try_next_if(|request| kernel_request_interleavable(request, busy));
            assert_eq!((served.is_some(), cancels), (gap["served"].as_bool().unwrap(), gap["cancels"].as_bool().unwrap()), "{}", gap["id"]);
            if served.is_none() && gap["head"].is_string() {
                assert!(queue.try_next().is_some(), "{}: a head that may not run stays the head", gap["id"]);
            }
        }
    }

    /// 🛑️ A request is abandoned — its turn cancellable — exactly when its requester dropped the future after the
    /// queue admitted it and before its outcome arrived.
    #[test]
    fn a_kernel_request_is_abandoned_only_when_dropped_undelivered_after_admission() {
        for case in turn_fairness()["abandonment"].as_array().expect("abandonment") {
            let queue = Arc::new(KernelRequestQueue::default());
            let slot = Arc::new(ResponseSlot::default());
            let mut future = KernelFuture { slot: slot.clone(), request: Some(destroy_request(9)), queue: queue.clone(), finished: false };
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            if case["admitted"].as_bool().unwrap() {
                assert!(Pin::new(&mut future).poll(&mut context).is_pending(), "{}", case["id"]);
                assert!(queue.try_next().is_some(), "{}: admitted", case["id"]);
            }
            if case["delivered"].as_bool().unwrap() {
                slot.deliver(KernelOutcome::Created(Ok(9)));
                assert!(Pin::new(&mut future).poll(&mut context).is_ready(), "{}", case["id"]);
            }
            drop(future);
            assert_eq!(slot.abandoned(), case["abandoned"].as_bool().unwrap(), "{}", case["id"]);
        }
    }

'''
laws = laws.replace(ANCHOR, NEW + ANCHOR)
LAWS.write_text(laws)
print("fairness contract + laws applied")
