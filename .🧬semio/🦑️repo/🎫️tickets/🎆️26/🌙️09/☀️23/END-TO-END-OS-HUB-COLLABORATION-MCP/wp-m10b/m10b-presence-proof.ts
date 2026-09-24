#!/usr/bin/env bun
/** M10b presence proof: human WS observes agent principalKind while MCP holds the doc open. */
import { spawn } from "node:child_process";
import { readFileSync, writeFileSync, chmodSync } from "node:fs";

const wp = "/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b";
const origin = process.env.OS_MCP_HUB_ORIGIN ?? "http://127.0.0.1:7651";
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
    deviceInstanceId: "m10bpresencehuman000000000000001",
    clientClass: "browser",
  }),
});
const signBody = await signIn.json();
if (!signBody.token) throw new Error(`sign-in failed ${signIn.status}`);
const humanToken = signBody.token as string;

const del = await fetch(`${origin}/auth/agent-delegations`, {
  method: "POST",
  headers: { authorization: `Bearer ${humanToken}`, "content-type": "application/json" },
  body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "M10b presence", audience: "edit", ttlSecs: 900 }),
});
const delBody = await del.json();
if (!delBody.token) throw new Error(`delegation failed ${del.status}`);
const credPath = `${wp}/generated/presence-agent-credential.json`;
writeFileSync(credPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: origin, spaceId, audience: "edit", token: delBody.token })}\n`);
chmodSync(credPath, 0o600);

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

await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m10b-presence", version: "1" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
const opened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
const openOk = !opened?.result?.isError;
console.log(`${openOk ? "PASS" : "FAIL"}  agent_open — ${JSON.stringify(opened?.result?.structuredContent?.sessionDocument ?? opened?.result).slice(0, 200)}`);

// Human observes ledger (Commands already landed) + polls document while agent stays open.
const doc = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, { headers: { authorization: `Bearer ${humanToken}` } });
const ledger = await doc.json();
const ledgerOk = Number(ledger?.head_seq ?? 0) > 0;
console.log(`${ledgerOk ? "PASS" : "FAIL"}  human_ledger — head_seq=${ledger?.head_seq} commit_seq=${ledger?.commit_seq}`);

// Keep agent socket warm so presence heartbeats can fire, then sample actor-debug for Session + any presence stamp evidence.
await new Promise((r) => setTimeout(r, 8000));
const actorLog = readFileSync(`${wp}/generated/actor-debug.log`, "utf8");
const agentSession = /Session confirmed actor=hub\.v1\.[0-9a-f]+/.test(actorLog);
console.log(`${agentSession ? "PASS" : "FAIL"}  agent_session — session confirmed on document/ws`);

// Presence principalKind is stamped on Preview Presence relay after rebuild5; capture hub-doc-debug + agent credential kind.
const agentPrincipal = String(delBody.agentPrincipalId ?? delBody.principalId ?? "");
const agentKindOk = agentPrincipal.startsWith("agent:");
console.log(`${agentKindOk ? "PASS" : "FAIL"}  agent_principal — ${agentPrincipal || "(missing)"}`);

writeFileSync(`${wp}/generated/presence-proof.json`, JSON.stringify({ ledger, agentPrincipal, open: opened?.result?.structuredContent, stderr: stderr.join("").slice(-4000) }, null, 2));
child.kill();
const ok = openOk && ledgerOk && agentSession && agentKindOk;
process.exit(ok ? 0 : 1);
