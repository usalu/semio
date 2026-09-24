#!/usr/bin/env bun
/** M10b short ledger proof with file-based auth. */
import { spawn } from "node:child_process";
import { readFileSync, writeFileSync, chmodSync } from "node:fs";

const wp = "/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b";
const origin = "http://127.0.0.1:7651";
const spaceId = readFileSync(`${wp}/generated/space-id.txt`, "utf8").trim();
const documentId = readFileSync(`${wp}/generated/document-id.txt`, "utf8").trim();
const binary = `${wp}/links/mcp-rust/dist/build/semio-os-mcp`;

const signIn = await fetch(`${origin}/auth/sessions`, {
  method: "POST",
  headers: { "content-type": "application/json" },
  body: JSON.stringify({
    schema: "semio.hub.auth.credential-sign-in/v1",
    email: "user1@semio.dev",
    password: "gm1-local-dev-pass-1",
    deviceInstanceId: "m10bledgerwait0000000000000004",
    clientClass: "browser",
  }),
});
const signBody = await signIn.json();
if (!signBody.token) throw new Error(`sign-in failed ${signIn.status} ${JSON.stringify(signBody)}`);
const humanToken = signBody.token as string;
writeFileSync(`${wp}/generated/human-token.txt`, humanToken);

const del = await fetch(`${origin}/auth/agent-delegations`, {
  method: "POST",
  headers: { authorization: `Bearer ${humanToken}`, "content-type": "application/json" },
  body: JSON.stringify({
    schema: "semio.hub.auth.agent-delegation-create/v1",
    spaceId,
    agentLabel: "M10b ledger3",
    audience: "edit",
    ttlSecs: 900,
  }),
});
const delBody = await del.json();
if (!delBody.token) throw new Error(`delegation failed ${del.status} ${JSON.stringify(delBody)}`);
const credPath = `${wp}/generated/agent-credential.json`;
writeFileSync(credPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: origin, spaceId, audience: "edit", token: delBody.token })}\n`);
chmodSync(credPath, 0o600);

writeFileSync(`${wp}/generated/actor-debug.log`, "");

const child = spawn(binary, ["stdio", "--hub", origin, "--space", spaceId, "--credential-file", credPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
const stderr: string[] = [];
child.stderr.on("data", (c) => stderr.push(c.toString()));
let buffer = "";
const pending = new Map<number, (m: any) => void>();
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let i: number;
  while ((i = buffer.indexOf("\n")) >= 0) {
    const line = buffer.slice(0, i).trim();
    buffer = buffer.slice(i + 1);
    if (!line) continue;
    let message: any;
    try { message = JSON.parse(line); } catch { continue; }
    const resolve = pending.get(message.id);
    if (resolve) { pending.delete(message.id); resolve(message); }
  }
});
let nextId = 1;
const call = (method: string, params: unknown) => new Promise<any>((resolve) => {
  const id = nextId++;
  pending.set(id, resolve);
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
});
const row = (ok: boolean, step: string, detail: string) => console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);

await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m10b-ledger", version: "1" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
const opened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
const open = opened?.result?.structuredContent;
row(!opened?.result?.isError, "open", `writePath=${open?.sessionDocument?.writePath} surface=${open?.sessionDocument?.surfaceId}`);
await new Promise((r) => setTimeout(r, 15000));
const searched = await call("tools/call", { name: "capabilities_search", arguments: { query: "add", kind: "mutation" } });
const hits = (searched?.result?.structuredContent?.results ?? []) as any[];
const verb = hits.find((h) => h.artifactKind === open?.artifactKind) ?? hits[0];
const capabilityId = String(verb?.id ?? verb?.capabilityId ?? "");
const prepared = await call("tools/call", { name: "action_prepare", arguments: { capabilityId, input: {} } });
const handle = prepared?.result?.structuredContent?.preparedHandle;
row(!prepared?.result?.isError && Boolean(handle), "prepare", `handle=${handle}`);
const invoked = await call("tools/call", { name: "action_invoke", arguments: { preparedActionHandle: handle } });
row(!invoked?.result?.isError && invoked?.result?.structuredContent?.status === "SUCCEEDED", "invoke", JSON.stringify(invoked?.result?.structuredContent).slice(0, 200));
await new Promise((r) => setTimeout(r, 5000));
const reopened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
row(Number(reopened?.result?.structuredContent?.sessionDocument?.relayedBatches ?? 0) > 0, "relayed", `batches=${reopened?.result?.structuredContent?.sessionDocument?.relayedBatches}`);

const deadline = Date.now() + 60_000;
let ledger: any = {};
let moved = false;
while (Date.now() < deadline) {
  const response = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, { headers: { authorization: `Bearer ${humanToken}` } });
  ledger = await response.json().catch(() => ({ status: response.status }));
  console.log(`INFO  ledger status=${response.status} head_seq=${ledger?.head_seq} commit_seq=${ledger?.commit_seq}`);
  if (Number(ledger?.head_seq ?? 0) > 0 || Number(ledger?.commit_seq ?? 0) > 0) { moved = true; break; }
  await new Promise((r) => setTimeout(r, 2000));
}
row(moved, "ledger", `head_seq=${ledger?.head_seq} commit_seq=${ledger?.commit_seq}`);
writeFileSync(`${wp}/generated/ledger-wait4.json`, JSON.stringify({ ledger, stderr: stderr.join("").slice(-8000) }, null, 2));
console.log("--- actor-debug.log ---");
console.log(readFileSync(`${wp}/generated/actor-debug.log`, "utf8").slice(0, 8000));
child.kill();
process.exit(moved ? 0 : 1);
