#!/usr/bin/env bun
/** GJ3 — prove s.wfc.bitmap.solve returns a RESULT via MCP after the inference pool-pump fix. */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { ensureMcpBinary, spawnRawMcp } from "./links/mcp/🟦️.ts";

const HERE = dirname(fileURLToPath(import.meta.url));
function repoRootFrom(start: string): string {
  let dir = start;
  for (let i = 0; i < 12; i++) {
    if (existsSync(join(dir, "Cargo.toml")) && existsSync(join(dir, ".mcp.json")) && existsSync(join(dir, "AGENTS.md"))) return dir;
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  throw new Error(`repo root not found from ${start}`);
}
const REPO = repoRootFrom(HERE);
const OUT = join(HERE, "generated");
mkdirSync(OUT, { recursive: true });
const BUDGET_MS = Number(process.env.GJ3_BUDGET_MS ?? 120_000);
const T0 = Date.now();
const at = () => `${((Date.now() - T0) / 1000).toFixed(1)}s`;
const SCOPES = "workspace.read,artifact.write,inference.execute,ui.observe,ui.control,conversation.write";

const indices = Array.from({ length: 16 }, (_, cell) => cell % 2);
const pixels = Buffer.from(Uint8Array.from(indices)).toString("base64");
const snapshot = {
  schema: "s.wfc.bitmap",
  seed: 11,
  input: {
    width: 4,
    height: 4,
    palette: [
      { r: 0, g: 0, b: 0, a: 255 },
      { r: 255, g: 255, b: 255, a: 255 },
    ],
    pixels,
  },
  output: { width: 6, height: 4, periodic: true },
  model: { patternSize: 2, symmetry: 1, periodicInput: true },
  pinned: [],
};

console.error(`[gj3] repo=${REPO} budgetMs=${BUDGET_MS}`);
const bin = ensureMcpBinary(REPO);
const folder = join(OUT, "wfc-workspace");
mkdirSync(folder, { recursive: true });
const mcp = spawnRawMcp(bin, ["stdio", "--folder", folder, "--no-bridge", "--scopes", SCOPES]);
const lines: string[] = [];
const log = (s: string) => {
  console.log(s);
  lines.push(s);
};

try {
  await mcp.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "gj3-solve-probe", version: "1" } }, BUDGET_MS);
  mcp.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized", params: {} }));
  const ctx = await mcp.request("tools/call", { name: "context_resolve", arguments: {} }, BUDGET_MS);
  const channel = (ctx as any).result?.structuredContent?.channel ?? (ctx as any).structuredContent?.channel;
  log(`[${at()}] channel=${channel}`);
  const progress: number[] = [];
  const onNotify = (msg: any) => {
    if (msg?.method === "notifications/progress") {
      const p = msg?.params?.progress;
      if (typeof p === "number") progress.push(p);
      log(`[${at()}] progress=${p}`);
    }
  };
  (mcp as any).onNotification?.(onNotify);
  const ran = await mcp.request(
    "tools/call",
    {
      name: "inference_run",
      arguments: {
        artifactKind: "s.wfc.bitmap",
        inferenceSchema: "s.wfc.bitmap.solve",
        pluginId: "wfc",
        payload: { snapshot },
        cancellationId: `gj3-${Date.now().toString(36)}`,
      },
      _meta: { progressToken: "gj3-solve" },
    },
    BUDGET_MS,
  );
  const result = (ran as any).result ?? ran;
  log(`[${at()}] inference_run isError=${result.isError}`);
  const structured = result.structuredContent ?? result;
  const text = JSON.stringify(structured);
  log(text.length > 8000 ? `${text.slice(0, 8000)}…(${text.length} B)` : text);
  const ok = result.isError !== true && text.includes("s.wfc.bitmap");
  writeFileSync(
    join(OUT, "gj3-wfc-solve.json"),
    JSON.stringify({ ok, elapsedMs: Date.now() - T0, progress, ran, stderr: mcp.stderrText?.()?.slice?.(0, 8000) ?? "" }, null, 2),
  );
  if (!ok) {
    log(`[${at()}] FAIL: no solved bitmap payload`);
    process.exitCode = 1;
  } else {
    log(`[${at()}] PASS solved bitmap via MCP`);
  }
} catch (error) {
  log(`[${at()}] FAILED ${(error as Error).message}`);
  writeFileSync(join(OUT, "gj3-wfc-solve-error.txt"), String(error));
  process.exitCode = 1;
} finally {
  writeFileSync(join(OUT, "gj3-wfc-solve-run.txt"), lines.join("\n") + "\n");
  try {
    mcp.kill?.();
  } catch {}
}
