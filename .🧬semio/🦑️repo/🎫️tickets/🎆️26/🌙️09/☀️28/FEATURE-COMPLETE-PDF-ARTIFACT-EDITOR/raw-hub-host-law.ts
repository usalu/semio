/** 🧪️ Verifies original group retention, retry, acknowledgement and cross-author Hub delivery in the actual host source. */
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { parseDocumentBackboneTransportMessage, encodeBackboneMessage } from "./🗑️generated/raw-spr-transport/message.ts";
import { encodeServerFrame } from "@semio-tech/framework-replication";
import { artifacts, type ArtifactState } from "./🗑️generated/raw-hub-atomic/host-fixture.ts";
import { acknowledgeAtomicHubGroup, deliverAtomicHubPacket, flushPendingAtomicHubGroup, relayAtomicHubPart, retireAtomicHubSocket } from "./🗑️generated/raw-hub-atomic/host.ts";

class Socket {
  readyState = WebSocket.OPEN;
  sent: Uint8Array[] = [];
  send(bytes: Uint8Array) { this.sent.push(bytes); }
  close() { this.readyState = WebSocket.CLOSED; }
}
const validScope = "fixture-document";
const fresh = (actor: string): ArtifactState => ({
  config: { documentId: validScope },
  actor, hubActorReady: true, closed: false, executionTargetLease: { fields: () => ({ grant: { write: true } }) },
  runtimeKey: actor, openClientInstanceId: "fixture-runtime", socket: new Socket(), nextBatchId: 82, pendingSocketActorId: null,
  pendingBatches: new Map(), pendingDocumentBackboneBytes: 0, pendingDocumentBackboneMessages: 0, frontier: null,
  received: [], events: [], recoveries: 0, requiredTailFrontier: null, artifactBootstrap: null, artifactRebootstrapRequired: false,
  browserActorReservation: null,
});
const parts = Array.from({ length: 8 }, (_, i) => new Uint8Array(readFileSync(new URL("./🗑️generated/raw-spr-transport/" + i + ".part", import.meta.url))));
const author = fresh("fixture-actor");
artifacts.set(author.runtimeKey, author);
const initial = author.socket!;
for (const [index, envelopes] of parts.entries()) {
  const message = encodeBackboneMessage({ kind: "mutations", envelopes });
  const selected = parseDocumentBackboneTransportMessage(message);
  assert.equal(selected.kind, "atomic-part");
  if (selected.kind !== "atomic-part") throw new Error("actual staged selector refused independent part");
  await relayAtomicHubPart(author, selected);
  assert.equal(initial.sent.length, index === 7 ? 8 : 0);
  assert.equal(author.pendingDocumentBackboneMessages, index + 1);
}
assert.ok(author.pendingDocumentBackboneBytes > 473_683);
assert.equal(acknowledgeAtomicHubGroup(author, 81, [{ Applied: { outcome: "Accepted" } }]), false);
retireAtomicHubSocket(author, initial as unknown as WebSocket);
const retry = new Socket();
author.socket = retry;
assert.equal(flushPendingAtomicHubGroup(author), true);
assert.equal(retry.sent.length, 8);
for (const [index, packet] of retry.sent.entries()) {
  assert.equal(new DataView(packet.buffer).getBigUint64(2, true), 83n);
  assert.deepEqual(packet.subarray(10), parts[index]);
}
assert.equal(acknowledgeAtomicHubGroup(author, 83, [{ Applied: { outcome: "Accepted" } }]), true);
assert.equal(acknowledgeAtomicHubGroup(author, 83, [{ Applied: { outcome: "Accepted" } }]), false);
assert.equal(author.pendingDocumentBackboneMessages, 0);
assert.equal(author.pendingDocumentBackboneBytes, 0);
assert.equal(author.events.length, 1);

const peer = fresh("fixture-peer");
artifacts.set(peer.runtimeKey, peer);
peer.browserActorReservation = { receiveBackbone: async message => { peer.received.push(message); } };
const metadata = encodeServerFrame({ Ack: { batch_id: 0, stages: [], frontier: {
  document_id: validScope, head_edit_ordinal: 15, last_commit_seq: 1, head_edit_id: "edit-png-82", chain_hash: Array(32).fill(19),
} } }, "command");
const serverPackets = parts.map((part, index) => {
  const prefix = index === 0 ? metadata : new Uint8Array();
  const packet = new Uint8Array(4 + prefix.length + part.length);
  packet.set([0, 16, prefix.length & 255, prefix.length >> 8]);
  packet.set(prefix, 4);
  packet.set(part, 4 + prefix.length);
  return packet;
});
for (const [index, packet] of serverPackets.entries()) {
  await deliverAtomicHubPacket(peer, peer.socket as unknown as WebSocket, packet);
  assert.equal(peer.received.length, index === 7 ? 8 : 0);
  assert.equal(peer.frontier !== null, index === 7);
}
assert.equal(peer.frontier?.head_edit_id, "edit-png-82");
assert.equal(peer.recoveries, 0);
console.log("[DEBUG] The actual host retained one group across reconnect, settled ownership once, and offered all cross-author pages only after verification");

const waiting = fresh("fixture-awaiting");
artifacts.set(waiting.runtimeKey, waiting);
let offered = 0;
waiting.browserActorReservation = { receiveBackbone: async () => {
  if (++offered === 8) throw new Error("atomic document delivery is ahead of its causal frontier");
} };
const awaiting = waiting.socket!;
for (const packet of serverPackets.slice(0, -1)) {
  assert.equal(await deliverAtomicHubPacket(waiting, awaiting as unknown as WebSocket, packet), true);
}
await assert.rejects(deliverAtomicHubPacket(waiting, awaiting as unknown as WebSocket, serverPackets.at(-1)!), /ahead of its causal frontier/);
assert.equal(offered, 8);
assert.equal(waiting.frontier, null);
assert.equal(waiting.events.length, 0);
assert.equal(awaiting.readyState, WebSocket.CLOSED);
console.log("[DEBUG] A refused Store causal frontier did not advance or acknowledge this authenticated incoming group");
