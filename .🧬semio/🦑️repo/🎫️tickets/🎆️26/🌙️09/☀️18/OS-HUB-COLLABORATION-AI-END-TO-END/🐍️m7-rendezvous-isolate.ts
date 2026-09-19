#!/usr/bin/env bun
/** 🛰️ Slice M7 — isolates ONE question: after a `.mcp.json`-verbatim stdio gateway starts against a
 * live dev session's rendezvous, does `GET /__semio/agent-bridge` on that dev session answer 200?
 *
 * The full loop probe scored this red without separating "the gateway never published" from "the dev
 * server never served it", so this walks both halves at once: the offers directory on disk and the
 * endpoint over loopback, every 500 ms, with the gateway's stderr interleaved.
 *
 * `S_AGENT_BRIDGE_DIR=<root> bun 🐍️m7-rendezvous-isolate.ts`
 */
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PORT = process.env.SEMIO_M7_PORT ?? "6080";
const OFFER_URL = `http://127.0.0.1:${PORT}/__semio/agent-bridge`;
const ROOT = process.env.S_AGENT_BRIDGE_DIR ?? join(import.meta.dir, "🗑️generated", "m7-rendezvous");
const BUDGET_MS = Number(process.env.SEMIO_M7_BUDGET_MS ?? 120_000);

const entry = (JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, { command: string; args: string[] }> }).mcpServers.semio!;
console.log(`root=${ROOT}`);
console.log(`sessions=${existsSync(join(ROOT, "sessions")) ? readdirSync(join(ROOT, "sessions")).join(",") : "(missing)"}`);
console.log(`endpoint before gateway: ${await fetch(OFFER_URL).then((r) => r.status).catch(() => 0)}`);

const t0 = Date.now();
const child = Bun.spawn([entry.command, ...entry.args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env, S_AGENT_BRIDGE_DIR: ROOT } });
void (async () => {
  const decoder = new TextDecoder();
  for await (const chunk of child.stderr) for (const line of decoder.decode(chunk).split("\n")) if (line.trim() && !line.includes("[mcp registry] skipping")) console.log(`  ${Date.now() - t0}ms stderr ${line.trim().slice(0, 220)}`);
})();
void (async () => {
  const decoder = new TextDecoder();
  for await (const chunk of child.stdout) console.log(`  ${Date.now() - t0}ms stdout ${decoder.decode(chunk).trim().slice(0, 220)}`);
})();

child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-06-18", capabilities: { roots: { listChanged: true }, elicitation: {} }, clientInfo: { name: "m7-rendezvous-isolate", version: "1" } } })}\n`);
void child.stdin.flush();

let served = false;
while (Date.now() - t0 < BUDGET_MS && !served) {
  const offersDir = join(ROOT, "offers");
  const onDisk = existsSync(offersDir) ? readdirSync(offersDir) : [];
  const status = await fetch(OFFER_URL).then((r) => r.status).catch(() => 0);
  if (onDisk.length || status === 200) console.log(`  ${Date.now() - t0}ms disk=[${onDisk.join(",")}] endpoint=${status}`);
  if (status === 200) {
    console.log(`  offer body: ${await fetch(OFFER_URL).then((r) => r.text())}`);
    for (const name of onDisk) console.log(`  ${name}: ${readFileSync(join(offersDir, name), "utf8").replace(/\s+/g, " ")}`);
    served = true;
  }
  await new Promise((resolve) => setTimeout(resolve, 500));
}
console.log(served ? `SERVED after ${Date.now() - t0}ms` : `NEVER SERVED within ${BUDGET_MS}ms`);
child.kill();
process.exit(served ? 0 : 1);
