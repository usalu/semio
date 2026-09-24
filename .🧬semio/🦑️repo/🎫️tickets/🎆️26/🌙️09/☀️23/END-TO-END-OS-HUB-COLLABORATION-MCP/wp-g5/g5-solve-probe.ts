#!/usr/bin/env bun
/** ⏱️ G5 over-MCP throughput probe: artifact_create s.wfc.bitmap → inference_run on that artifact (the client-e2e genesis solve), timed end to end, with the relay's stderr lines streamed; G5_CANCEL_AFTER_MS sends notifications/cancelled mid-solve. */
import { spawn } from "node:child_process";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createInterface } from "node:readline";

const RUN = process.env.G5_RUN ?? "solve";
const BIN = process.env.SEMIO_OS_MCP_BIN ?? "/Users/ueli/Documents/semio/.tmp-ticket/wp-g5/target/debug/semio-os-mcp";
const BUDGET_MS = Number(process.env.G5_BUDGET_MS ?? 600_000);
const CANCEL_AFTER_MS = Number(process.env.G5_CANCEL_AFTER_MS ?? 0);
const OUT = join(import.meta.dir, "generated");
const SCOPES = "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write";
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(2)}s`;
const log: string[] = [];
const say = (line: string) => {
  log.push(`[${at()}] ${line}`);
  console.log(`[${at()}] ${line}`);
};

const child = spawn(BIN, ["stdio", "--folder", mkdtempSync(join(tmpdir(), "semio-g5-probe-")), "--no-bridge", "--scopes", SCOPES], { stdio: ["pipe", "pipe", "pipe"] });
createInterface({ input: child.stderr }).on("line", (line) => {
  if (line.includes("[DEBUG] g5") || /error|panic|fault/i.test(line)) say(`stderr ${line.slice(0, 400)}`);
});
const pending = new Map<number, (msg: any) => void>();
createInterface({ input: child.stdout }).on("line", (line) => {
  const msg = JSON.parse(line);
  if (typeof msg.id === "number" && pending.has(msg.id)) {
    pending.get(msg.id)!(msg);
    pending.delete(msg.id);
  } else if (msg.method === "notifications/progress") say(`progress ${JSON.stringify(msg.params).slice(0, 160)}`);
});
let nextId = 1;
const request = (method: string, params: unknown, timeoutMs = 900_000): { id: number; reply: Promise<any> } => {
  const id = nextId++;
  const reply = new Promise<any>((resolve) => {
    const timer = setTimeout(() => {
      pending.delete(id);
      resolve({ error: { code: -32000, message: `no reply within ${timeoutMs} ms` } });
    }, timeoutMs);
    pending.set(id, (msg) => {
      clearTimeout(timer);
      resolve(msg);
    });
  });
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  return { id, reply };
};
const call = async (name: string, args: unknown) => {
  const msg = await request("tools/call", { name, arguments: args }).reply;
  return msg.result ?? { isError: true, structuredContent: msg.error };
};

let ok = false;
try {
  await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g5-probe", version: "1" } }).reply;
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized", params: {} })}\n`);
  const artifactId = `g5-wfc-${Date.now().toString(36)}`;
  const created = await call("artifact_create", { artifactId, kind: "s.wfc.bitmap" });
  say(`artifact_create ${created?.isError ? "FAIL" : "ok"} ${JSON.stringify(created?.structuredContent).slice(0, 160)}`);
  for (let repeat = Number(process.env.G5_REPEAT ?? 1); repeat > 1; repeat--) {
    const warmToken = `g5-warm-${repeat}-${Date.now().toString(36)}`;
    const warmStarted = Date.now();
    const warm = await request("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", artifactId, cancellationId: warmToken }, _meta: { progressToken: warmToken } }, BUDGET_MS).reply;
    say(`inference_run (repeat) ${warm.error ? "FAILED" : "answered"} in ${Date.now() - warmStarted} ms status=${warm.result?.structuredContent?.status}`);
  }
  const token = `g5-${Date.now().toString(36)}`;
  const started = Date.now();
  const ran = request("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", artifactId, cancellationId: token }, _meta: { progressToken: token } }, BUDGET_MS);
  if (CANCEL_AFTER_MS > 0) {
    setTimeout(() => {
      say(`sending notifications/cancelled requestId=${ran.id}`);
      child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/cancelled", params: { requestId: ran.id, reason: "g5 probe" } })}\n`);
    }, CANCEL_AFTER_MS);
  }
  const reply = await ran.reply;
  const elapsed = Date.now() - started;
  const result = reply.error ?? reply.result?.structuredContent;
  ok = !reply.error && reply.result?.isError !== true;
  say(`inference_run ${ok ? "answered" : "FAILED"} in ${elapsed} ms status=${result?.status ?? result?.state ?? "-"} complete=${result?.complete} error=${JSON.stringify(result?.error ?? result?.code ?? null)} — ${JSON.stringify({ ...result, payload: undefined }).slice(0, 400)}`);
  const ping = await request("ping", {}, 10_000).reply;
  say(`ping ${ping.error ? "FAIL" : "ok"}`);
} finally {
  child.kill();
  writeFileSync(join(OUT, `g5-${RUN}.txt`), `${log.join("\n")}\n`);
}
process.exit(ok ? 0 : 1);
