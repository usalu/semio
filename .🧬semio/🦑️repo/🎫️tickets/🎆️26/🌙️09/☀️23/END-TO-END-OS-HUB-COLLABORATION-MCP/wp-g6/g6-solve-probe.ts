#!/usr/bin/env bun
/** ⏱️ G6 over-MCP throughput probe: artifact_create s.wfc.bitmap → inference_run on that artifact (the client-e2e genesis solve), timed end to end, with the relay's stderr lines streamed; G6_CANCEL_AFTER_MS sends notifications/cancelled mid-solve. */
import { spawn } from "node:child_process";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createInterface } from "node:readline";
import { execFileSync } from "node:child_process";

const RUN = process.env.G6_RUN ?? "solve";
const BIN = process.env.SEMIO_OS_MCP_BIN ?? "/Users/ueli/Documents/semio/.tmp-ticket/wp-g6/target/debug/semio-os-mcp";
const BUDGET_MS = Number(process.env.G6_BUDGET_MS ?? 600_000);
const CANCEL_AFTER_MS = Number(process.env.G6_CANCEL_AFTER_MS ?? 0);
const CANCEL_AT_PROGRESS = Number(process.env.G6_CANCEL_AT_PROGRESS ?? 0);
let cancelArmed: ((progress: number) => void) | null = null;
let cancelSentAt = 0;
const OUT = join(import.meta.dir, "generated");
const SCOPES = "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write";
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(2)}s`;
const log: string[] = [];
const say = (line: string) => {
  log.push(`[${at()}] ${line}`);
  console.log(`[${at()}] ${line}`);
};

const child = spawn(BIN, ["stdio", "--folder", mkdtempSync(join(tmpdir(), "semio-g6-probe-")), "--no-bridge", "--scopes", SCOPES], { stdio: ["pipe", "pipe", "pipe"] });
createInterface({ input: child.stderr }).on("line", (line) => {
  if (line.includes("[DEBUG] g6") || /error|panic|fault/i.test(line)) say(`stderr ${line.slice(0, 400)}`);
});
const pending = new Map<number, (msg: any) => void>();
createInterface({ input: child.stdout }).on("line", (line) => {
  const msg = JSON.parse(line);
  if (typeof msg.id === "number" && pending.has(msg.id)) {
    pending.get(msg.id)!(msg);
    pending.delete(msg.id);
  } else if (msg.method === "notifications/progress") {
    say(`progress ${JSON.stringify(msg.params).slice(0, 160)}`);
    cancelArmed?.(Number(msg.params?.progress));
  }
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
const cpuSeconds = (): number => {
  const text = execFileSync("ps", ["-o", "time=", "-p", String(child.pid)]).toString().trim();
  const parts = text.split(":").map(Number);
  return parts.reduce((total, part) => total * 60 + part, 0);
};
const threadSeconds = (): number[] => {
  const rows = execFileSync("ps", ["-M", "-p", String(child.pid)]).toString().trim().split("\n").slice(1);
  const clock = (text: string) => text.split(":").map(Number).reduce((total, part) => total * 60 + part, 0);
  return rows.map((row) => {
    const fields = row.trim().split(/\s+/);
    const times = fields.filter((field) => /^\d+:\d+\.\d+$/.test(field));
    return times.length >= 2 ? clock(times[times.length - 2]) + clock(times[times.length - 1]) : 0;
  });
};
const call = async (name: string, args: unknown) => {
  const msg = await request("tools/call", { name, arguments: args }).reply;
  return msg.result ?? { isError: true, structuredContent: msg.error };
};

let ok = false;
try {
  await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g6-probe", version: "1" } }).reply;
  child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized", params: {} })}\n`);
  const artifactId = `g6-wfc-${Date.now().toString(36)}`;
  const created = await call("artifact_create", { artifactId, kind: "s.wfc.bitmap" });
  say(`artifact_create ${created?.isError ? "FAIL" : "ok"} ${JSON.stringify(created?.structuredContent).slice(0, 160)}`);
  for (let repeat = Number(process.env.G6_REPEAT ?? 1); repeat > 1; repeat--) {
    const warmToken = `g6-warm-${repeat}-${Date.now().toString(36)}`;
    const warmStarted = Date.now();
    const warm = await request("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", artifactId, cancellationId: warmToken }, _meta: { progressToken: warmToken } }, BUDGET_MS).reply;
    say(`inference_run (repeat) ${warm.error ? "FAILED" : "answered"} in ${Date.now() - warmStarted} ms status=${warm.result?.structuredContent?.status}`);
  }
  const token = `g6-${Date.now().toString(36)}`;
  const cpuBefore = cpuSeconds();
  const threadsBefore = threadSeconds();
  const started = Date.now();
  const ran = request("tools/call", { name: "inference_run", arguments: { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", artifactId, cancellationId: token }, _meta: { progressToken: token } }, BUDGET_MS);
  const sendCancel = () => {
    cancelSentAt = Date.now();
    say(`sending notifications/cancelled requestId=${ran.id}`);
    child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/cancelled", params: { requestId: ran.id, reason: "g6 probe" } })}\n`);
  };
  if (CANCEL_AT_PROGRESS > 0) {
    cancelArmed = (progress) => {
      if (Math.abs(progress - CANCEL_AT_PROGRESS) < 1e-6 && cancelSentAt === 0) setTimeout(sendCancel, Number(process.env.G6_CANCEL_DELAY_MS ?? 0));
    };
  }
  if (CANCEL_AFTER_MS > 0) {
    setTimeout(sendCancel, CANCEL_AFTER_MS);
  }
  const reply = await ran.reply;
  const elapsed = Date.now() - started;
  const threadDeltas = threadSeconds().map((seconds, index) => seconds - (threadsBefore[index] ?? 0)).sort((a, b) => b - a);
  say(`inference_run per-thread cpu deltas (s, largest first): ${threadDeltas.slice(0, 4).map((value) => value.toFixed(2)).join(", ")}`);
  say(`inference_run process cpu ${(cpuSeconds() - cpuBefore).toFixed(2)} s over ${(elapsed / 1000).toFixed(2)} s wall`);
  if (cancelSentAt > 0) say(`cancel answered ${Date.now() - cancelSentAt} ms after notifications/cancelled`);
  const result = reply.error ?? reply.result?.structuredContent;
  ok = !reply.error && reply.result?.isError !== true;
  say(`inference_run ${ok ? "answered" : "FAILED"} in ${elapsed} ms status=${result?.status ?? result?.state ?? "-"} complete=${result?.complete} error=${JSON.stringify(result?.error ?? result?.code ?? null)} — ${JSON.stringify({ ...result, payload: undefined }).slice(0, 400)}`);
  const ping = await request("ping", {}, 10_000).reply;
  say(`ping ${ping.error ? "FAIL" : "ok"}`);
} finally {
  child.kill();
  writeFileSync(join(OUT, `g6-${RUN}.txt`), `${log.join("\n")}\n`);
}
process.exit(ok ? 0 : 1);
