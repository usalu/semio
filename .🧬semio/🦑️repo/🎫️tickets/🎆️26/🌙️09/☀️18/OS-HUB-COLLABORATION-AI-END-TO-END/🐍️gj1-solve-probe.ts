#!/usr/bin/env bun
/** 🧩️ GJ1 — drive `s.wfc.bitmap.solve` to a RESULT and print every progress row with its wall
 * offset, so a long run is distinguishable from a spin. Payload: the crate's own 4×4 stripes
 * fixture, stated by hand (WI1 §7.2's carrier-free path). */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
const REPO_ROOT = join(dirname(fileURLToPath(new URL(import.meta.url))), "..", "..", "..", "..", "..", "..", "..");
type Entry = { readonly command: string; readonly args: readonly string[] };
const entry = (JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, Entry> }).mcpServers.semio;
const BUDGET_MS = Number(process.env.GJ1_BUDGET_MS ?? 1_500_000);
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(1)}s`;
const pending = new Map<number, (e: Record<string, any>) => void>();
let buffer = "", nextId = 1;
const child = Bun.spawn([entry.command, ...entry.args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env } });
const stderrPath = process.env.GJ1_STDERR;
if (stderrPath) { void (async () => { const sink = Bun.file(stderrPath).writer(); const d = new TextDecoder(); for await (const c of child.stderr) { sink.write(d.decode(c)); sink.flush(); } })(); }
void (async () => { const d = new TextDecoder(); for await (const c of child.stdout) { buffer += d.decode(c); let n = buffer.indexOf("\n"); while (n >= 0) { const line = buffer.slice(0, n).trim(); buffer = buffer.slice(n + 1); if (line) { const e = JSON.parse(line); if (typeof e.method === "string") { if (e.method === "notifications/progress") { const p = e.params ?? {}; console.log(`  [${at()}] progress ${p.progress ?? "?"}/${p.total ?? "?"} ${p.message ?? ""}`); } } else { pending.get(e.id)?.(e); pending.delete(e.id); } } n = buffer.indexOf("\n"); } } })();
const req = (method: string, params: unknown) => { const id = nextId++; const p = new Promise<Record<string, any>>((r) => pending.set(id, r)); child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`); void child.stdin.flush(); return Promise.race([p, new Promise<Record<string, any>>((_, rej) => setTimeout(() => rej(new Error(`${method} timed out after ${BUDGET_MS}ms`)), BUDGET_MS))]); };
const call = (name: string, args: unknown, meta?: unknown) => req("tools/call", { name, arguments: args, ...(meta ? { _meta: meta } : {}) });

const indices = Array.from({ length: 16 }, (_, cell) => cell % 2);
const pixels = Buffer.from(Uint8Array.from(indices)).toString("base64");
const snapshot = {
  schema: "s.wfc.bitmap",
  seed: 11,
  input: { width: 4, height: 4, palette: [{ r: 0, g: 0, b: 0, a: 255 }, { r: 255, g: 255, b: 255, a: 255 }], pixels },
  output: { width: 6, height: 4, periodic: true },
  model: { patternSize: 2, symmetry: 1, periodicInput: true },
  pinned: [],
};
try {
  await req("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "gj1-solve-probe", version: "1" } });
  const ctx = await call("context_resolve", {});
  console.log(`[${at()}] channel=${(ctx.result?.structuredContent as any)?.channel}`);
  const ran = await call(
    "inference_run",
    { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", payload: { snapshot }, cancellationId: `gj1-${Date.now().toString(36)}`, ...(process.env.GJ1_WORK_UNITS ? { workUnits: Number(process.env.GJ1_WORK_UNITS) } : {}) },
    { progressToken: "gj1-solve" },
  );
  console.log(`[${at()}] inference_run isError=${ran.result?.isError}`);
  const structured = ran.result?.structuredContent;
  const text = JSON.stringify(structured ?? ran.result ?? ran);
  console.log(text.length > 6000 ? `${text.slice(0, 6000)}…(${text.length} B)` : text);
} catch (error) {
  console.log(`[${at()}] FAILED ${(error as Error).message}`);
} finally { child.kill(); }
