#!/usr/bin/env bun
/** 👀️ G4 outcome proof: a human holds a live document socket on a hub note while an agent, over the semio MCP (`--hub` delegated session), commits `addBlock`; the human's socket must receive the agent's `Commands` relay and the ledger head must advance. Usage: bun g4-agent-edit-human-sees.ts <hubOrigin> */
import { spawn } from "node:child_process";
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { decodePresencePeer, decodeServerFrame, encodeClientFrame } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { parseSocketGrantReceiptV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";
import { requireMcpBinary } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const ORIGIN = process.argv[2] ?? "http://127.0.0.1:7830";
const WS = ORIGIN.replace(/^http/u, "ws");
const OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-g4/generated";
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(1)}s`;
const rows: Array<{ step: string; ok: boolean; detail: string }> = [];
const row = (step: string, ok: boolean, detail: string) => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  [${at()}] ${step} — ${detail}`);
};
const hub = async (method: string, path: string, token?: string, body?: unknown) => {
  const response = await fetch(`${ORIGIN}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body: JSON.stringify(body) }), signal: AbortSignal.timeout(60_000) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};

const signIn = await hub("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: "g4humanobserver00000000000000001", clientClass: "browser" });
const human = String(signIn.json?.token ?? "");
row("human signs in", signIn.status === 200 && human.length > 0, `HTTP ${signIn.status}`);
const spaces = await hub("GET", "/directory/spaces", human);
const spaceId = String((spaces.json ?? []).map((entry: any) => entry?.space?.id).filter(Boolean).at(-1) ?? "");
const delegation = await hub("POST", "/auth/agent-delegations", human, { schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "G4 Claude agent", audience: "edit", ttlSecs: 900 });
row("human delegates an agent", delegation.status === 201, `space=${spaceId} agent=${delegation.json?.agentPrincipalId}`);
const credentialPath = join(mkdtempSync(join(tmpdir(), "semio-g4-agent-")), "agent-credential.json");
writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: ORIGIN, spaceId, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
chmodSync(credentialPath, 0o600);

const child = spawn(requireMcpBinary("/Users/ueli/Documents/semio"), ["stdio", "--hub", ORIGIN, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
let stderr = "";
child.stderr.on("data", (chunk: Buffer) => (stderr += chunk.toString("utf8")));
let buffer = "";
const pending = new Map<number, (message: any) => void>();
child.stdout.on("data", (chunk: Buffer) => {
  buffer += chunk.toString("utf8");
  for (let newline = buffer.indexOf("\n"); newline >= 0; newline = buffer.indexOf("\n")) {
    const line = buffer.slice(0, newline).trim();
    buffer = buffer.slice(newline + 1);
    if (!line) continue;
    const message = JSON.parse(line);
    pending.get(message.id)?.(message);
    pending.delete(message.id);
  }
});
let nextId = 1;
const request = (method: string, params: unknown) =>
  new Promise<any>((resolve) => {
    const id = nextId++;
    pending.set(id, resolve);
    child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  });
const call = async (name: string, args: unknown) => (await request("tools/call", { name, arguments: args })).result ?? {};

let socket: WebSocket | undefined;
try {
  await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g4-agent", version: "1" } });
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
  const listed = await request("resources/read", { uri: "semio://workspace/artifacts" });
  const artifacts = JSON.parse(String(listed.result?.contents?.[0]?.text ?? "{}")).artifacts ?? [];
  const note = artifacts.find((entry: any) => String(entry?.schema ?? entry?.artifactSchema ?? entry?.descriptor?.artifactSchema ?? "").startsWith("note")) ?? artifacts[0];
  const documentId = String(note?.scope?.documentId ?? "");
  const opened = await call("artifact_open", { artifactId: documentId });
  const surfaceId = String(opened.structuredContent?.sessionDocument?.surfaceId ?? "");
  row("agent opens the hub document over the semio MCP", opened.isError !== true && surfaceId.length > 0, `document=${documentId} kind=${opened.structuredContent?.kind} surface=${surfaceId} writePath=${opened.structuredContent?.sessionDocument?.writePath}`);

  const before = await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, human);
  const plan = await hub("POST", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, human, { schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: surfaceId, clientInstanceId: "g4-human-observer" });
  const grant = await hub("POST", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`, human, { schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.json?.receipt });
  const receipt = parseSocketGrantReceiptV1(grant.json);
  const frames: any[] = [];
  const url = `${WS}/scopes/${encodeURIComponent(`${spaceId}/${documentId}`)}/document/ws?surface=${encodeURIComponent(plan.json?.surface?.surfaceId ?? surfaceId)}`;
  socket = new WebSocket(url, ["semio.session.v1", human]);
  socket.binaryType = "arraybuffer";
  socket.addEventListener("message", (event) => {
    if (event.data instanceof ArrayBuffer) frames.push(decodeServerFrame(new Uint8Array(event.data)).frame);
  });
  for (let tick = 0; tick < 200 && socket.readyState === WebSocket.CONNECTING; tick++) await new Promise((resolve) => setTimeout(resolve, 50));
  const hash = [...(String(plan.json?.artifact?.packSchemaHash ?? "").match(/../gu) ?? [])].map((pair) => Number.parseInt(pair, 16));
  socket.send(encodeClientFrame({ SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: plan.json?.artifact?.schema, pack_schema_hash: hash, resume_token: null, frontier: null } }, "command"));
  const waitFor = async (predicate: (frame: any) => boolean, ms: number) => {
    const deadline = Date.now() + ms;
    while (Date.now() < deadline) {
      const hit = frames.find(predicate);
      if (hit) return hit;
      await new Promise((resolve) => setTimeout(resolve, 50));
    }
    return undefined;
  };
  const session = await waitFor((frame) => "Session" in frame, 20_000);
  row("human holds a live document socket", Boolean(session), `plan=${plan.status} grant=${grant.status} humanActor=${receipt.actorId} session=${JSON.stringify(session?.Session ?? null)}`);

  const search = await call("capabilities_search", { query: "add a block to the note", kind: ["mutation"] });
  const capabilityId = String((search.structuredContent?.results ?? []).find((hit: any) => String(hit.capabilityId ?? hit.id).endsWith(".addBlock"))?.capabilityId ?? "note.s.note.note@1/*#editor.addBlock");
  const framesBeforeInvoke = frames.length;
  const prepared = await call("action_prepare", { capabilityId, input: { kind: "text" } });
  const invoked = await call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
  row("agent commits addBlock over the semio MCP", invoked.isError !== true && invoked.structuredContent?.status === "SUCCEEDED", `${capabilityId} status=${invoked.structuredContent?.status} ${invoked.isError ? JSON.stringify(invoked.structuredContent).slice(0, 200) : ""}`);

  const relayed = await waitFor((frame) => "Commands" in frame && frames.indexOf(frame) >= framesBeforeInvoke, 60_000);
  const envelopes = (relayed?.Commands?.envelopes ?? []) as any[];
  const actors = envelopes.map((envelope) => String(envelope?.actor ?? envelope?.actor_id ?? ""));
  row("the human's socket receives the agent's Commands relay", Boolean(relayed) && actors.length > 0 && actors.every((actor) => actor !== receipt.actorId), `envelopes=${envelopes.length} actors=${JSON.stringify(actors)} humanActor=${receipt.actorId}`);
  console.log(`INFO  relayed envelope document_id=${JSON.stringify(envelopes.map((envelope) => envelope?.document_id ?? envelope?.documentId))} batch=${relayed?.Commands?.batch_id} keys=${JSON.stringify(Object.keys(envelopes[0] ?? {}))}`);
  await new Promise((resolve) => setTimeout(resolve, 3_000));
  const reopened = await call("artifact_open", { artifactId: documentId });
  console.log(`INFO  agent sessionDocument after commit ${JSON.stringify(reopened.structuredContent?.sessionDocument ?? null)}`);
  console.log(`INFO  human frame kinds ${JSON.stringify(frames.map((frame) => Object.keys(frame)[0]))}`);
  const presence = frames.filter((frame) => "Presence" in frame).flatMap((frame) => (frame.Presence.peers as number[][]).map((peer) => decodePresencePeer(Uint8Array.from(peer), [0])));
  console.log(`INFO  presence peers seen by the human: ${JSON.stringify(presence.map((peer: any) => ({ actor: peer.actor, label: peer.label, principalKind: peer.principalKind ?? peer.principal_kind })))}`);
  let after = await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, human);
  for (let poll = 0; poll < 30 && Number(after.json?.head_seq ?? 0) <= Number(before.json?.head_seq ?? 0); poll++) {
    await new Promise((resolve) => setTimeout(resolve, 1_000));
    after = await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, human);
  }
  console.log(`INFO  ledger before=${before.text.slice(0, 300)} after=${after.text.slice(0, 300)}`);
  row("the human reads the advanced ledger head", Number(after.json?.head_seq ?? 0) > Number(before.json?.head_seq ?? 0), `head_seq ${before.json?.head_seq}→${after.json?.head_seq} commit_seq ${before.json?.commit_seq}→${after.json?.commit_seq}`);
  writeFileSync(join(OUT, "g4-agent-edit-human-sees.json"), JSON.stringify({ rows, documentId, spaceId, frames: frames.map((frame) => Object.keys(frame)[0]), relayed, presence, stderrTail: stderr.slice(-4000) }, null, 2));
} catch (error) {
  row("proof", false, String(error));
} finally {
  socket?.close();
  child.kill();
  await hub("DELETE", `/auth/agent-delegations/${encodeURIComponent(String(delegation.json?.delegationId ?? ""))}`, human).catch(() => undefined);
  const red = rows.filter((entry) => !entry.ok).length;
  console.log(`g4-agent-edit-human-sees: ${rows.length - red}/${rows.length} rows green against ${ORIGIN}`);
  process.exit(red === 0 ? 0 : 1);
}
