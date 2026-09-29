# ⚖️ Section G: the language-agnostic backbone parity fixture pins what reaches the STORE (`storeMessages`) for a hub frame, an
# accepted and a refused batch — the Rust native actor and the TS worker replay it (Rust twin + TS twin of one law).
PARITY_FIXTURE = STORE_DIR + "🔄️sync/⚖️parity/🧫️fixtures/🔣️.json"
PARITY_SCHEMA = STORE_DIR + "🔄️sync/⚖️parity/🧬️schema/🔣️.json"
PARITY_TS = STORE_DIR + "🔄️sync/🧪️tests/🔬️backbone-parity/🟦️.ts"
PARITY_RS = STORE_DIR + "🔄️sync/🧪️tests/🔬️backbone-parity/🦀️.rs"


def json_edit(rel, mutate, label):
    text = read(rel)
    if text is None:
        problems.append((label, "missing file"))
        return
    data = json.loads(text)
    if json.dumps(data, indent=2, ensure_ascii=False) + "\n" != text:
        problems.append((label, "not canonical two-space json"))
        return
    if not mutate(data):
        plan.append(f"present {label}")
        return
    files[rel] = json.dumps(data, indent=2, ensure_ascii=False) + "\n"
    plan.append(f"edit    {label}")


def frontier(ordinal, head, byte):
    return {"documentId": "parity-doc", "headEditOrdinal": ordinal, "headEditId": head, "lastCommitSeq": ordinal, "chainHashByte": byte}


HUB_ORDER_SCENARIOS = [
    {
        "id": "hub-order-frame-runs",
        "title": "A Hub Frame Reaches The Store Run By Run In The Hub's Order",
        "documentId": "parity-doc",
        "spaceId": "parity-space",
        "actor": "local-actor",
        "steps": [
            {"op": "localDispatch", "dispatch": {"kind": "queueMutation", "mutationId": "own-1", "n": 1}},
            {"op": "localDispatch", "dispatch": {"kind": "connectSocket", "actor": "hub.v1." + "f" * 64}},
            {"op": "serverFrame", "frame": {"kind": "welcome", "sessionId": "s9", "resumeToken": "resume-9", "frontier": frontier(0, "genesis", 1), "bootstrap": "none"}},
            {"op": "serverFrame", "frame": {"kind": "session", "actor": "hub.v1." + "f" * 64, "color": 4}},
            {"op": "expect", "expect": {"outboxMutationIds": [], "pendingBatchCount": 1, "storeMessages": []}},
            {"op": "localDispatch", "dispatch": {"kind": "failConnection"}},
            {"op": "localDispatch", "dispatch": {"kind": "connectSocket", "actor": "hub.v1." + "f" * 64}},
            {"op": "serverFrame", "frame": {"kind": "welcome", "sessionId": "s10", "resumeToken": "resume-10", "frontier": frontier(3, "remote-2", 4), "bootstrap": "tail"}},
            {"op": "serverFrame", "frame": {"kind": "commands", "origin": "hub.catch-up", "envelopes": [{"mutationId": "remote-1", "n": 2}, {"mutationId": "own-1", "n": 1}, {"mutationId": "remote-2", "n": 3}], "frontier": frontier(3, "remote-2", 4)}},
            {"op": "expect", "expect": {"outboxMutationIds": [], "pendingBatchCount": 0, "frontierEditId": "remote-2", "storeMessages": ["sequenced:remote-1", "committed:own-1", "sequenced:remote-2"]}},
        ],
    },
    {
        "id": "hub-order-ack-commits",
        "title": "An Accepted Batch Reaches The Store As The Hub's Commit",
        "documentId": "parity-doc",
        "spaceId": "parity-space",
        "actor": "local-actor",
        "steps": [
            {"op": "localDispatch", "dispatch": {"kind": "queueMutation", "mutationId": "own-2", "n": 4}},
            {"op": "localDispatch", "dispatch": {"kind": "connectSocket", "actor": "hub.v1." + "9" * 64}},
            {"op": "serverFrame", "frame": {"kind": "welcome", "sessionId": "s11", "resumeToken": "resume-11", "frontier": frontier(0, "genesis", 1), "bootstrap": "none"}},
            {"op": "serverFrame", "frame": {"kind": "session", "actor": "hub.v1." + "9" * 64, "color": 1}},
            {"op": "serverFrame", "frame": {"kind": "ack", "batchId": 0, "outcome": "accepted", "frontier": frontier(1, "own-2", 2)}},
            {"op": "expect", "expect": {"pendingBatchCount": 0, "commandOutcome": "accepted", "storeMessages": ["committed:own-2"]}},
        ],
    },
]
STORE_EXPECTATIONS = {
    "remote-commands-order": ["sequenced:remote-1,remote-2"],
    "ack-rejected": ["committed:reject-1", "sequenced:reject-1~undo"],
}


def add_hub_order_scenarios(data):
    changed = False
    known = {scenario["id"] for scenario in data["scenarios"]}
    for scenario in HUB_ORDER_SCENARIOS:
        if scenario["id"] not in known:
            data["scenarios"].append(scenario)
            changed = True
    for scenario in data["scenarios"]:
        messages = STORE_EXPECTATIONS.get(scenario["id"])
        if messages is None:
            continue
        final = [step for step in scenario["steps"] if step["op"] == "expect"][-1]["expect"]
        if final.get("storeMessages") != messages:
            final["storeMessages"] = messages
            changed = True
    return changed


def add_store_messages_expectation(data):
    properties = data["$defs"]["Expectation"]["properties"]
    if "storeMessages" in properties:
        return False
    properties["storeMessages"] = {"type": "array", "items": {"type": "string", "pattern": "^(mutations|sequenced|committed|ack):"}}
    return True


json_edit(PARITY_FIXTURE, add_hub_order_scenarios, "parity fixture hub-order scenarios and store expectations")
json_edit(PARITY_SCHEMA, add_store_messages_expectation, "parity schema store expectation")

edit(PARITY_TS, """import { encodeClientFrame, encodePresencePeer, type MutationEnvelope, type ServerFrame, type WireFrontierSummary, type WireMutationEnvelope } from "@semio-tech/framework-replication";
import type { ArtifactActorConfig } from "../../../../../🟦️.ts";""", """import { decodeDocumentBackboneEnvelopeBatchExact, encodeClientFrame, encodePresencePeer, type MutationEnvelope, type ServerFrame, type WireFrontierSummary, type WireMutationEnvelope } from "@semio-tech/framework-replication";
import { decodeBackboneMessage, type ArtifactActorConfig } from "../../../../../🟦️.ts";""", "parity ts imports the store message codec")
edit(PARITY_TS, """function clientFrameKind(bytes: Uint8Array, decodeClientFrame: BackboneWorkerTestDependencies["decodeClientFrame"]): string {""", """/** 🏷️ `kind:id,id` of one store-bound message — the fixture's `storeMessages` vocabulary, shared with the Rust twin. */
function storeMessageLabel(bytes: Uint8Array | readonly number[]): string {
  const message = decodeBackboneMessage(bytes instanceof Uint8Array ? bytes : Uint8Array.from(bytes));
  if (message.kind === "ack" || message.kind === "committed") return `${message.kind}:${message.opIds.join(",")}`;
  if (message.kind === "mutations" || message.kind === "sequenced") return `${message.kind}:${decodeDocumentBackboneEnvelopeBatchExact(message.envelopes).map((envelope) => envelope.mutation_id).join(",")}`;
  return message.kind;
}

function clientFrameKind(bytes: Uint8Array, decodeClientFrame: BackboneWorkerTestDependencies["decodeClientFrame"]): string {""", "parity ts store message label")
edit(PARITY_TS, """      const observedEvents: string[] = [];
""", """      const observedEvents: string[] = [];
      const storeMessages: string[] = [];
""", "parity ts records store messages")
edit(PARITY_TS, """          observedEvents.push(event.kind);
""", """          observedEvents.push(event.kind);
          if (event.kind === "documentBackbone") storeMessages.push(storeMessageLabel((event as unknown as { message: Uint8Array | readonly number[] }).message));
""", "parity ts labels each store message")
edit(PARITY_TS, """            const commandBatch = "Commands" in frame && frame.Commands.envelopes.length > 0 ? new Uint8Array([1, 2, 3]) : null;""",
     """            const commandBatch = "Commands" in frame && frame.Commands.envelopes.length > 0 ? dependencies.extractServerCommandsDocumentBackboneBatchExact(dependencies.encodeServerFrame(frame, "command")) : null;""", "parity ts hands the worker the frame's exact batch")
edit(PARITY_TS, """            if (typeof expect.commandOutcome === "string") vitest.expect(lastOutcome).toBe(expect.commandOutcome);""", """            if (typeof expect.commandOutcome === "string") vitest.expect(lastOutcome).toBe(expect.commandOutcome);
            if (Array.isArray(expect.storeMessages)) vitest.expect(storeMessages).toEqual(expect.storeMessages);""", "parity ts asserts store messages")

edit(PARITY_RS, """use crate::os_spr::wire::RebootstrapRequired;""", """use crate::os_spr::wire::RebootstrapRequired;
use crate::os_store::{Backbone, BackboneMessage};""", "parity rs imports the store backbone")
edit(PARITY_RS, """fn remote_kind(state: &RemoteState) -> &'static str {""", """/// 🏷️ `kind:id,id` of one store-bound message — the fixture's `storeMessages` vocabulary, shared with the TS twin.
fn store_message_label(message: &BackboneMessage) -> String {
    let ids = |envelopes: &[u8]| crate::os_spr::decode_envelopes(envelopes).expect("store-bound batch decodes").iter().map(|envelope| envelope.mutation_id.0.clone()).collect::<Vec<_>>().join(",");
    match message {
        BackboneMessage::Ack { op_ids } => format!("ack:{}", op_ids.join(",")),
        BackboneMessage::Committed { op_ids } => format!("committed:{}", op_ids.join(",")),
        BackboneMessage::Mutations { envelopes } => format!("mutations:{}", ids(envelopes)),
        BackboneMessage::Sequenced { envelopes } => format!("sequenced:{}", ids(envelopes)),
        BackboneMessage::Genesis { .. } => "genesis".into(),
        BackboneMessage::Member { .. } => "member".into(),
    }
}

fn remote_kind(state: &RemoteState) -> &'static str {""", "parity rs store message label")
edit(PARITY_RS, """    socket: Option<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>,
    document_id: String,
}""", """    socket: Option<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>,
    document_id: String,
    store: ChannelBackbone,
    store_messages: Vec<String>,
}""", "parity rs keeps the store end")
edit(PARITY_RS, """        let (_, remote) = ChannelBackbone::pair("backbone-parity").await;""", """        let (store, remote) = ChannelBackbone::pair("backbone-parity").await;""", "parity rs opens the store end")
edit(PARITY_RS, """        Self { actor, events, observed_events: Vec::new(), outgoing: Vec::new(), last_outcome: None, socket: None, document_id: document_id.into() }
    }""", """        Self { actor, events, observed_events: Vec::new(), outgoing: Vec::new(), last_outcome: None, socket: None, document_id: document_id.into(), store, store_messages: Vec::new() }
    }

    async fn drain_store(&mut self) {
        for message in self.store.receive().await.expect("store inbound drains") {
            self.store_messages.push(store_message_label(&message));
        }
    }""", "parity rs drains the store end")
edit(PARITY_RS, """        self.actor.inject_hub_frame(server).await;
        self.drain_events();
        self.pump_outgoing().await;
    }""", """        self.actor.inject_hub_frame(server).await;
        self.drain_events();
        self.drain_store().await;
        self.pump_outgoing().await;
    }""", "parity rs records store messages after a frame")
edit(PARITY_RS, """            other => panic!("unsupported dispatch {other}"),
        }
        self.drain_events();
        self.pump_outgoing().await;
    }""", """            other => panic!("unsupported dispatch {other}"),
        }
        self.drain_events();
        self.drain_store().await;
        self.pump_outgoing().await;
    }""", "parity rs records store messages after a dispatch")
edit(PARITY_RS, """        if let Some(outcome) = expect.get("commandOutcome").and_then(|value| value.as_str()) {
            assert_eq!(self.last_outcome.as_deref(), Some(outcome), "commandOutcome");
        }""", """        if let Some(outcome) = expect.get("commandOutcome").and_then(|value| value.as_str()) {
            assert_eq!(self.last_outcome.as_deref(), Some(outcome), "commandOutcome");
        }
        if let Some(rows) = expect.get("storeMessages").and_then(|value| value.as_array()) {
            let expected: Vec<String> = rows.iter().map(|row| row.as_str().expect("store message").into()).collect();
            assert_eq!(self.store_messages, expected, "storeMessages");
        }""", "parity rs asserts store messages")
