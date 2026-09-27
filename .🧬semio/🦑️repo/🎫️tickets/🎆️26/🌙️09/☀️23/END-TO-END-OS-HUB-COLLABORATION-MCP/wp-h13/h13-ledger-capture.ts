#!/usr/bin/env bun
/** 📼️ H13 item 4: captures what a hub Check In folds — the document's active checkpoint pair (pack, spr) and its committed
 * ledger tail as one `os_spr::encode_envelopes` stream (`writeVecEnvelope`, the Rust `encode_envelopes` twin) — so the
 * refusal replays natively with a backtrace. Reads the tail from a fresh document socket (hello with no frontier).
 * usage: bun h13-ledger-capture.ts <origin> <spaceId> <documentId> <outDir>   (env C12_USER1_PASSWORD) */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { hubProbeCall, hubProbeOpenPlan, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
import { decodeCanonicalCheckpointPairV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { decodeServerFrame, encodeClientFrame, writeVecEnvelope } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { parseDocumentSocketGrantReceiptV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";

const [origin, spaceId, documentId, outDir] = process.argv.slice(2) as [string, string, string, string];
mkdirSync(outDir, { recursive: true });
const token = await hubProbeSignIn(origin, "user1@semio.dev", process.env.C12_USER1_PASSWORD!, "h13-ledger-capture");
const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`;
const pairAnswer = await hubProbeCall(origin, "GET", `${scope}/active-checkpoint/pair`, token, undefined, "application/vnd.semio.canonical-checkpoint-pair.v1");
if (pairAnswer.status !== 200) throw new Error(`pair ${pairAnswer.status}`);
const pair = decodeCanonicalCheckpointPairV1(pairAnswer.bytes);
writeFileSync(join(outDir, "pack.bin"), pair.packBytes);
writeFileSync(join(outDir, "spr.bin"), pair.sprBytes);
const plan = await hubProbeOpenPlan(origin, token, spaceId, documentId, "h13-ledger-capture");
const grant = await hubProbeCall(origin, "POST", `${scope}/socket-grants`, token, JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json.receipt }));
parseDocumentSocketGrantReceiptV1(grant.json);
const socket = new WebSocket(`${origin.replace(/^http/u, "ws")}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json.surface.surfaceId)}`, ["semio.session.v1", token]);
socket.binaryType = "arraybuffer";
const envelopes: any[] = [];
let welcome: any = null;
let lastFrameAt = Date.now();
socket.addEventListener("message", (event) => {
  if (!(event.data instanceof ArrayBuffer)) return;
  const frame: any = decodeServerFrame(new Uint8Array(event.data)).frame;
  lastFrameAt = Date.now();
  if ("Welcome" in frame) welcome = frame.Welcome;
  if ("Commands" in frame) envelopes.push(...frame.Commands.envelopes);
});
for (let tick = 0; tick < 400 && socket.readyState === WebSocket.CONNECTING; tick += 1) await Bun.sleep(50);
const packSchemaHash = [...(String(plan.json.artifact.packSchemaHash).match(/../gu) ?? [])].map((hex) => Number.parseInt(hex, 16));
socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
while (welcome === null || Date.now() - lastFrameAt < 3_000) await Bun.sleep(200);
socket.close(1000, "capture");
const stream: number[] = [];
writeVecEnvelope(stream, envelopes);
writeFileSync(join(outDir, "envelopes.bin"), Uint8Array.from(stream));
const summary = {
  baseline: pair.baselineFrontier,
  packBytes: pair.packBytes.length,
  sprBytes: pair.sprBytes.length,
  welcome: JSON.parse(JSON.stringify(welcome, (_key, value) => (typeof value === "bigint" ? value.toString() : value))),
  envelopes: envelopes.map((envelope) => ({ id: envelope.mutation_id, actor: envelope.actor, deps: envelope.dependencies, observed: envelope.observed, target: envelope.target, diffSchema: envelope.diff.schema, diffBytes: envelope.diff.payload.length, inverseBytes: envelope.inverse.payload.length })),
  streamBytes: stream.length,
};
writeFileSync(join(outDir, "summary.json"), `${JSON.stringify(summary, null, 2)}\n`);
console.log(JSON.stringify({ ...summary, welcome: undefined, envelopes: summary.envelopes.length }));
process.exit(0);
