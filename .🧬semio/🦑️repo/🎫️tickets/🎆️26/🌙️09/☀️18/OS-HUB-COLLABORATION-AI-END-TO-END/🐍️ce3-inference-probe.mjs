// 💡️ Drives `inference_run` on the real `semio-os-mcp` binary and prints the UNTRUNCATED reply —
// the client-e2e gate slices its detail at 300 chars, which hides the ActionBus factory's own
// rejection text (slice CE3).
import { spawn } from "node:child_process";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const repo = "/Users/ueli/Documents/semio";
const bin = join(repo, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp");
const folder = mkdtempSync(join(tmpdir(), "ce3-inference-"));
const child = spawn(bin, ["stdio", "--folder", folder, "--scopes", "workspace.read,artifact.write,inference.execute", "--no-bridge"], { cwd: repo, stdio: ["pipe", "pipe", "pipe"] });
let buffer = "";
const pending = new Map();
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let index;
  while ((index = buffer.indexOf("\n")) >= 0) {
    const line = buffer.slice(0, index).trim();
    buffer = buffer.slice(index + 1);
    if (!line) continue;
    let envelope; try { envelope = JSON.parse(line); } catch { continue; }
    if (envelope.id !== undefined && pending.has(envelope.id)) { pending.get(envelope.id)(envelope); pending.delete(envelope.id); }
  }
});
child.stderr.on("data", (chunk) => process.stderr.write(`[stderr] ${chunk}`));
let next = 1;
const request = (method, params) => new Promise((resolve, reject) => {
  const id = next++;
  pending.set(id, resolve);
  setTimeout(() => { if (pending.delete(id)) reject(new Error(`${method} timed out`)); }, 300_000).unref?.();
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
});
await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "ce3-probe", version: "0" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized", params: {} })}\n`);
const run = await request("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", cancellationId: "ce3-probe-cancel" } });
console.log(JSON.stringify(run, null, 2));
child.kill();
