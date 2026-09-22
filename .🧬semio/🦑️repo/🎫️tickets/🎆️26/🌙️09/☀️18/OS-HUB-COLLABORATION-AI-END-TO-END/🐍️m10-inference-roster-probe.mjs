// 🧪️ M10 — reads the LIVE gateway's `inference_list` roster over a real stdio MCP session and
// prints every declared row, so the extension-contributed service is observed on the real install
// rather than only in a unit law.
import { spawn } from "node:child_process";

const binary = process.argv[2];
const folder = process.argv[3] ?? ".";
const child = spawn(binary, ["stdio", "--folder", folder, "--scopes", "workspace.read,artifact.write,inference.execute"], { stdio: ["pipe", "pipe", "pipe"] });
let buffer = "";
const pending = new Map();
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let index;
  while ((index = buffer.indexOf("\n")) >= 0) {
    const line = buffer.slice(0, index).trim();
    buffer = buffer.slice(index + 1);
    if (!line) continue;
    let message;
    try { message = JSON.parse(line); } catch { continue; }
    const resolve = pending.get(message.id);
    if (resolve) { pending.delete(message.id); resolve(message); }
  }
});
child.stderr.on("data", (chunk) => process.stderr.write(chunk));
setTimeout(() => { console.log("probe timed out"); child.kill();
process.exit(0); process.exit(2); }, 240_000).unref();
let nextId = 1;
const call = (method, params) => new Promise((resolve) => {
  const id = nextId++;
  pending.set(id, resolve);
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
});
await call("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "m10-roster-probe", version: "1" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
const listed = await call("tools/call", { name: "inference_list", arguments: {} });
const declared = listed?.result?.structuredContent?.declared ?? [];
console.log(`declared=${declared.length}`);
for (const row of declared) console.log(`${row.owner} | ${row.contributor} | ${row.artifactKind} | ${row.inferenceSchema}`);
child.kill();
process.exit(0);
