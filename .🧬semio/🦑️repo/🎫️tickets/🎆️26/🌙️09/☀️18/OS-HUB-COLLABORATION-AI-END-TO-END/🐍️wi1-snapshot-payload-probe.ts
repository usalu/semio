#!/usr/bin/env bun
/** 🔬️ WI1 — diagnostic: the SAME inference with a HAND-WRITTEN `snapshot` instead of the bound
 * `document`. Separates "the solve cannot run in this lane" from "the document carrier is broken".
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
const REPO_ROOT = join(dirname(fileURLToPath(new URL(import.meta.url))), "..", "..", "..", "..", "..", "..", "..");
type Entry = { readonly command: string; readonly args: readonly string[] };
const entry = (JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, Entry> }).mcpServers.semio;
const pending = new Map<number, (e: Record<string, any>) => void>();
let buffer = "", nextId = 1;
const child = Bun.spawn([entry.command, ...entry.args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env } });
void (async () => { const d = new TextDecoder(); for await (const c of child.stdout) { buffer += d.decode(c); let n = buffer.indexOf("\n"); while (n >= 0) { const line = buffer.slice(0, n).trim(); buffer = buffer.slice(n + 1); if (line) { const e = JSON.parse(line); if (typeof e.method !== "string") { pending.get(e.id)?.(e); pending.delete(e.id); } } n = buffer.indexOf("\n"); } } })();
const req = (method: string, params: unknown) => { const id = nextId++; const p = new Promise<Record<string, any>>((r) => pending.set(id, r)); child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`); void child.stdin.flush(); return Promise.race([p, new Promise<Record<string, any>>((_, rej) => setTimeout(() => rej(new Error(`${method} timed out`)), 300000))]); };
const call = (name: string, args: unknown) => req("tools/call", { name, arguments: args });

// 🖼️ A 4×4 two-colour vertical-stripe sample, collapsed into a 6×4 periodic output — the exact
// fixture the crate's own `stripes()` law uses, stated by hand so nothing is read from a document.
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
  await req("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wi1-snapshot-payload-probe", version: "1" } });
  const started = Date.now();
  const ran = await call("inference_run", { artifactKind: "s.wfc.bitmap", inferenceSchema: "s.wfc.bitmap.solve", pluginId: "wfc", payload: { snapshot }, cancellationId: `wi1-snap-${Date.now().toString(36)}` });
  console.log(`inference_run(snapshot): ${Math.round((Date.now() - started) / 1000)}s isError=${ran.result?.isError}`);
  console.log(JSON.stringify(ran.result?.structuredContent ?? ran, null, 2).slice(0, 2500));
} finally { child.kill(); }
