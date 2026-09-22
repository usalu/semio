/** ⏱️ Splits the (e3) hang of `live-agent-loop-check` into its two candidate costs (slice CE2):
 * how long a cold `semio-os-mcp` gateway takes to answer `initialize` with the whole 59-descriptor
 * catalog behind it, and how long a destructive `action_invoke` then takes to come back with the
 * typed elicitation timeout when the client advertises `elicitation` and never answers. The gate
 * itself only reports "did not answer within 240000ms", which cannot tell the two apart. */
import { spawn } from "node:child_process";
import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const BIN = join(REPO, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp");
const CAPABILITY = process.argv[2] ?? "note.s.note.note@1/*#editor.deleteSelection";
const child = spawn(BIN, ["stdio", "--folder", REPO, "--scopes", "workspace.read,artifact.write,inference.execute,ui.observe,ui.control"], {
  stdio: ["pipe", "pipe", "pipe"],
  env: { ...process.env, S_AGENT_BRIDGE_DIR: mkdtempSync(join(tmpdir(), "ce2-silent-")) },
});
let buffer = "";
const pending = new Map<number, (value: Record<string, unknown>) => void>();
child.stdout.on("data", (chunk: Buffer) => {
  buffer += chunk.toString("utf8");
  for (let index = buffer.indexOf("\n"); index >= 0; index = buffer.indexOf("\n")) {
    const line = buffer.slice(0, index).trim();
    buffer = buffer.slice(index + 1);
    if (!line) continue;
    const message = JSON.parse(line) as { id?: number; method?: string };
    if (typeof message.id === "number" && pending.has(message.id)) {
      pending.get(message.id)!(message as Record<string, unknown>);
      pending.delete(message.id);
      continue;
    }
    if (message.method) console.log(`  [${(Date.now() - started) / 1000}s] server → ${message.method}`);
  }
});
child.stderr.on("data", (chunk: Buffer) => {
  for (const line of chunk.toString("utf8").split("\n")) if (line.trim()) console.log(`  stderr: ${line.slice(0, 160)}`);
});
const started = Date.now();
let nextId = 1;
function call(method: string, params: Record<string, unknown>): Promise<Record<string, unknown>> {
  const id = nextId++;
  return new Promise((resolve) => {
    pending.set(id, resolve);
    child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  });
}
const t0 = Date.now();
await call("initialize", { protocolVersion: "2025-06-18", capabilities: { elicitation: {} }, clientInfo: { name: "ce2-timing", version: "0" } });
console.log(`initialize answered in ${((Date.now() - t0) / 1000).toFixed(1)}s`);
child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
const t1 = Date.now();
console.log(`invoking ${CAPABILITY} and answering NOTHING …`);
const answer = await call("tools/call", { name: "action_invoke", arguments: { capabilityId: CAPABILITY, input: {} } });
console.log(`action_invoke answered in ${((Date.now() - t1) / 1000).toFixed(1)}s`);
console.log(JSON.stringify(answer).slice(0, 700));
child.kill();
process.exit(0);
