#!/usr/bin/env bun
import { spawn } from "node:child_process";
import { readFileSync, writeFileSync, chmodSync } from "node:fs";
const wp = "/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b";
const origin = "http://127.0.0.1:7681";
const spaceId = readFileSync(`${wp}/generated/space-id.txt`, "utf8").trim();
const documentId = readFileSync(`${wp}/generated/document-id.txt`, "utf8").trim();
const binary = `${wp}/links/mcp-rust/dist/build/semio-os-mcp`;
const signIn = await fetch(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: "m10bconnectonly000000000000000", clientClass: "browser" }) });
const signBody = await signIn.json();
const humanToken = signBody.token as string;
const del = await fetch(`${origin}/auth/agent-delegations`, { method: "POST", headers: { authorization: `Bearer ${humanToken}`, "content-type": "application/json" }, body: JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: "M10b connect", audience: "edit", ttlSecs: 900 }) });
const delBody = await del.json();
const credPath = `${wp}/generated/agent-credential.json`;
writeFileSync(credPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: origin, spaceId, audience: "edit", token: delBody.token })}\n`);
chmodSync(credPath, 0o600);
writeFileSync(`${wp}/generated/actor-debug.log`, "");
const stderrChunks=[];
const child = spawn(binary, ["stdio", "--hub", origin, "--space", spaceId, "--credential-file", credPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
let buffer = "";
const pending = new Map<number, (m: any) => void>();
child.stderr.on("data", (c) => stderrChunks.push(c.toString()));
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let i: number;
  while ((i = buffer.indexOf("\n")) >= 0) {
    const line = buffer.slice(0, i).trim();
    buffer = buffer.slice(i + 1);
    if (!line) continue;
    try {
      const message = JSON.parse(line);
      const resolve = pending.get(message.id);
      if (resolve) { pending.delete(message.id); resolve(message); }
    } catch {}
  }
});
let nextId = 1;
const call = (method: string, params: unknown) => new Promise<any>((resolve) => {
  const id = nextId++;
  pending.set(id, resolve);
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
});
await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m10b-connect", version: "1" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
const opened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
console.log("open", opened?.result?.structuredContent?.sessionDocument?.writePath);
for (let t = 0; t < 20; t++) {
  await new Promise((r) => setTimeout(r, 1000));
  const log = readFileSync(`${wp}/generated/actor-debug.log`, "utf8");
  console.log(`t=${t+1}s logLines=${log.split("\n").filter(Boolean).length}`);
  if (log.includes("finish_connect OK") || log.includes("finish_connect Err") || log.includes("websocket up")) break;
}
console.log("--- log ---\n" + readFileSync(`${wp}/generated/actor-debug.log`, "utf8"));
child.kill();
