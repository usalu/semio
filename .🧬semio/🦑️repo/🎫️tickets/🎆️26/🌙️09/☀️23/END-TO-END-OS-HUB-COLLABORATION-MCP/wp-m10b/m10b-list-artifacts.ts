#!/usr/bin/env bun
/** ✍️ M10b — list hub artifacts via MCP, then run the write-path chain with a real documentId. */
import { spawn } from "node:child_process";
import { writeFileSync } from "node:fs";

const [origin, spaceId, credentialPath, outPath] = process.argv.slice(2);
const binary = process.env.M10_MCP_BINARY!;
const child = spawn(binary, ["stdio", "--hub", origin, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"], { stdio: ["pipe", "pipe", "pipe"] });
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
await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m10b-list", version: "1" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
const listed = await call("resources/read", { uri: "semio://workspace/artifacts" });
const text = listed?.result?.contents?.[0]?.text ?? "{}";
writeFileSync(outPath, text + "\n");
const parsed = JSON.parse(text);
const arts = parsed.artifacts ?? [];
console.log(JSON.stringify({ count: arts.length, sample: arts[0], ids: arts.map((a: any) => a?.scope?.documentId ?? a?.artifactId ?? a?.id) }, null, 2));
child.kill();
