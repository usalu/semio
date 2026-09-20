/** 🔬️ WR1 — drives the ONE `client-e2e` row that hangs (`artifact_create` with a real plugin
 * artifact kind) against the staged `semio-os-mcp` binary, with no client budget, printing the pid
 * so the stuck process can be `sample`d and every stderr line as it arrives. The gate's own client
 * gives up at 240 000 ms and kills the server, which destroys the evidence; this does not. */
import { spawn } from "node:child_process";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const binary = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp");
const folder = mkdtempSync(join(tmpdir(), "wr1-open-probe-"));
const kind = process.argv[2] ?? "s.draw.drawing";

const child = spawn(binary, ["stdio", "--folder", folder, "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control"], { stdio: ["pipe", "pipe", "pipe"] });
console.log(`[probe] pid=${child.pid} binary=${binary}`);
console.log(`[probe] folder=${folder} kind=${kind}`);

const pending = new Map();
let next = 1;
let buffer = "";
child.stdout.on("data", (chunk) => {
  buffer += chunk.toString();
  let index = buffer.indexOf("\n");
  while (index >= 0) {
    const line = buffer.slice(0, index).trim();
    buffer = buffer.slice(index + 1);
    if (line) {
      const envelope = JSON.parse(line);
      const waiter = pending.get(envelope.id);
      if (waiter) {
        pending.delete(envelope.id);
        waiter(envelope);
      }
    }
    index = buffer.indexOf("\n");
  }
});
child.stderr.on("data", (chunk) => {
  for (const line of chunk.toString().split("\n")) if (line.trim()) console.log(`[stderr] ${line.trim()}`);
});

function request(method, params) {
  const id = next++;
  const started = Date.now();
  const answered = new Promise((resolve) => pending.set(id, resolve));
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  return answered.then((envelope) => {
    console.log(`[probe] ${method} answered in ${Date.now() - started} ms`);
    return envelope;
  });
}

await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wr1-open-probe", version: "0" } });
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
const created = await request("tools/call", { name: "artifact_create", arguments: { artifactId: `wr1-open-probe-${Date.now()}`, kind } });
console.log(`[probe] artifact_create -> ${JSON.stringify(created.result ?? created.error).slice(0, 600)}`);
child.kill();
process.exit(0);
