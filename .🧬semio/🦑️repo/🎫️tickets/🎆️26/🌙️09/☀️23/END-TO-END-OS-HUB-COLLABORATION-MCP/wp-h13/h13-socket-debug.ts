/** 🔬️ H13 one-off: opens a note as a fresh edit agent on a hub and prints every document-socket frame kind + the close. usage: bun h13-socket-debug.ts <hub> (env OS_HUB_PROBE_PASSWORD) */
import { randomBytes } from "node:crypto";
import { hubProbeCall, hubProbeCreateArtifact, hubProbeCreateSpace, hubProbeCreationCatalog, hubProbeOpenPlan, hubProbeSignIn } from "/Users/ueli/Documents/semio/🌎️hub/🤝️integration-harness/🟦️.ts";
import { decodeServerFrame, encodeClientFrame } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { parseDocumentSocketGrantReceiptV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";
const hub = process.argv[2]!;
const human = await hubProbeSignIn(hub, "user1@semio.dev", process.env.OS_HUB_PROBE_PASSWORD!, "h13dbg");
const spaceId = await hubProbeCreateSpace(hub, human, `H13 dbg ${Date.now()}`);
const catalog = await hubProbeCreationCatalog(hub, human, spaceId);
const kind = catalog.kinds.find((k) => k.schema.startsWith("note"))!;
const doc = await hubProbeCreateArtifact(hub, human, spaceId, catalog.generationId, kind.kindId, "dbg");
const created = await hubProbeCall(hub, "POST", "/auth/agent-delegations", human, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "dbg", audience: process.argv[3] ?? "edit", ttlSecs: 900 }));
const session = await hubProbeCall(hub, "POST", "/auth/agent-sessions", created.json.token, JSON.stringify({ schema: "semio.hub.auth.agent-session/v1", audience: process.argv[3] ?? "edit", agentInstanceId: `dbg.${randomBytes(4).toString("hex")}` }));
console.log("session", session.status);
const token = process.argv[4] === "human" ? human : session.json.token;
const scope = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(doc.artifactId)}`;
const plan = await hubProbeOpenPlan(hub, token, spaceId, doc.artifactId, "dbg");
const grant = await hubProbeCall(hub, "POST", `${scope}/socket-grants`, token, JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json.receipt }));
console.log("plan", plan.status, "grant", grant.status);
const granted = parseDocumentSocketGrantReceiptV1(grant.json);
const socket = new WebSocket(`${hub.replace(/^http/u, "ws")}/scopes/${encodeURIComponent(`${spaceId}/${doc.artifactId}`)}/document/ws?surface=${encodeURIComponent(plan.json.surface.surfaceId)}`, ["semio.session.v1", token]);
socket.binaryType = "arraybuffer";
const t0 = Date.now();
socket.addEventListener("open", () => {
  console.log(Date.now() - t0, "open");
  const packSchemaHash = [...(String(plan.json.artifact.packSchemaHash).match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
  socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json.artifact.schema, pack_schema_hash: packSchemaHash, resume_token: null, frontier: null } }, "command"));
});
socket.addEventListener("close", (event) => console.log(Date.now() - t0, "close", event.code, event.reason));
socket.addEventListener("message", (event) => {
  if (!(event.data instanceof ArrayBuffer)) { console.log(Date.now() - t0, "text", String(event.data).slice(0, 200)); return; }
  let frame: any;
  try { frame = decodeServerFrame(new Uint8Array(event.data)).frame; } catch (error) { console.log(Date.now() - t0, "undecodable", String(error).slice(0, 200)); return; }
  console.log(Date.now() - t0, "frame", typeof frame === "string" ? frame : Object.keys(frame)[0], JSON.stringify(frame, (_k, v) => (typeof v === "bigint" ? String(v) : v)).slice(0, 240));
  if ("Welcome" in frame) socket.send(encodeClientFrame({ Commands: { batch_id: 1, envelopes: [{ mutation_id: `dbg-${randomBytes(8).toString("hex")}`, document_id: doc.artifactId, actor: granted.actorId, dependencies: [], observed: null, target: [], diff: { schema: plan.json.artifact.schema, payload: Array.from(new TextEncoder().encode("dbg")) }, inverse: { schema: plan.json.artifact.schema, payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 } }] } }, "command"));
});
await Bun.sleep(8000);
process.exit(0);
