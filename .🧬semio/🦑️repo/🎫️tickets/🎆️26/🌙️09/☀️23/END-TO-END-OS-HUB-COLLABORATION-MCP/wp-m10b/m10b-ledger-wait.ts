#!/usr/bin/env bun
/** ✍️ M10b — open hub doc, invoke, wait for ledger head_seq with a long-lived gateway. */
import { spawn } from "node:child_process";
import { writeFileSync } from "node:fs";

const [origin, spaceId, credentialPath, documentId, humanToken] = process.argv.slice(2);
const binary = process.env.M10_MCP_BINARY!;
const child = spawn(binary, ["stdio", "--hub", origin, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
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
const searched = await call("tools/call", { name: "capabilities_search", arguments: { query: "add", kind: "mutation" } });
const hits = (searched?.result?.structuredContent?.results ?? []) as any[];
const verb = hits.find((h) => h.artifactKind === open?.artifactKind) ?? hits[0];
const capabilityId = String(verb?.id ?? verb?.capabilityId ?? "");
const args = Object.fromEntries((verb?.arguments ?? []).filter((a: any) => a?.required).map((a: any) => [a.id, a.default ?? (a?.schema?.kind === "number" ? 0 : a?.schema?.options?.[0]?.value ?? "point")]));
console.log("INFO  capability", capabilityId, "input", JSON.stringify(args));
const prepared = await call("tools/call", { name: "action_prepare", arguments: { capabilityId, input: args } });
const handle = prepared?.result?.structuredContent?.preparedHandle;
row(!prepared?.result?.isError && Boolean(handle), "prepare", prepared?.result?.isError ? JSON.stringify(prepared.result.structuredContent).slice(0, 300) : `handle=${handle}`);
const invoked = await call("tools/call", { name: "action_invoke", arguments: { preparedActionHandle: handle } });
row(!invoked?.result?.isError && invoked?.result?.structuredContent?.status === "SUCCEEDED", "invoke", JSON.stringify(invoked?.result?.structuredContent).slice(0, 300));
const reopened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
row(Number(reopened?.result?.structuredContent?.sessionDocument?.relayedBatches ?? 0) > 0, "relayed", `batches=${reopened?.result?.structuredContent?.sessionDocument?.relayedBatches}`);

const deadline = Date.now() + 300_000;
let ledger: any = {};
let moved = false;
while (Date.now() < deadline) {
  const response = await fetch(`${origin}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, { headers: { authorization: `Bearer ${humanToken}` } });
  ledger = await response.json().catch(() => ({}));
  console.log(`INFO  ledger head_seq=${ledger?.head_seq} commit_seq=${ledger?.commit_seq} epoch=${ledger?.epoch}`);
  if (Number(ledger?.head_seq ?? 0) > 0 || Number(ledger?.commit_seq ?? 0) > 0) { moved = true; break; }
  await new Promise((r) => setTimeout(r, 3000));
}
row(moved, "ledger", `head_seq=${ledger?.head_seq} commit_seq=${ledger?.commit_seq} epoch=${ledger?.epoch}`);
writeFileSync(process.env.M10B_LEDGER_OUT ?? "/tmp/m10b-ledger.json", JSON.stringify({ ledger, stderr: stderr.join("").slice(-4000) }, null, 2));
if (stderr.length) console.log("--- stderr tail ---\n" + stderr.join("").split("\n").slice(-30).join("\n"));
child.kill();
process.exit(moved ? 0 : 1);
