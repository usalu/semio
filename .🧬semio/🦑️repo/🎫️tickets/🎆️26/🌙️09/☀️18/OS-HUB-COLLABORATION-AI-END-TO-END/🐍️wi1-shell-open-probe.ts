#!/usr/bin/env bun
/** 🔬️ WI1 — which catalog verb opens a `🀄️wfc` surface in the live `s` shell? */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { chromium } from "playwright";
const REPO_ROOT = join(dirname(fileURLToPath(new URL(import.meta.url))), "..", "..", "..", "..", "..", "..", "..");
type Entry = { readonly command: string; readonly args: readonly string[] };
const entry = (JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, Entry> }).mcpServers.semio;
const pending = new Map<number, (e: Record<string, any>) => void>();
let buffer = "", nextId = 1;
const child = Bun.spawn([entry.command, ...entry.args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env } });
void (async () => { const d = new TextDecoder(); for await (const c of child.stdout) { buffer += d.decode(c); let n = buffer.indexOf("\n"); while (n >= 0) { const line = buffer.slice(0, n).trim(); buffer = buffer.slice(n + 1); if (line) { const e = JSON.parse(line); if (typeof e.method !== "string") { pending.get(e.id)?.(e); pending.delete(e.id); } } n = buffer.indexOf("\n"); } } })();
const req = (m: string, p: unknown) => { const id = nextId++; const pr = new Promise<Record<string, any>>((r) => pending.set(id, r)); child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method: m, params: p })}\n`); void child.stdin.flush(); return Promise.race([pr, new Promise<Record<string, any>>((_, rj) => setTimeout(() => rj(new Error(`${m} timed out`)), 180000))]); };
const call = (n: string, a: unknown) => req("tools/call", { name: n, arguments: a });
const S = (e: Record<string, any>) => (e.result?.structuredContent ?? {}) as Record<string, any>;
const b = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await b.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
try {
  await req("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wi1-shell-open-probe", version: "1" } });
  await page.goto("http://127.0.0.1:6200/", { waitUntil: "domcontentloaded", timeout: 300000 });
  for (let i = 0; i < 300; i++) { if (await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))) break; await new Promise(r => setTimeout(r, 1000)); }
  for (let i = 0; i < 120; i++) { if (S(await call("ui_focus", {})).code === undefined) break; await new Promise(r => setTimeout(r, 1000)); }
  for (const id of ["space.s.space.home@1/*#editor.createStudio", "space.s.space.studio@1/*#editor.spawnApp"]) {
    const described = await call("capabilities_describe", { capabilityId: id });
    console.log(`\n== ${id}\n${JSON.stringify(S(described)).slice(0, 1400)}`);
  }
} finally { child.kill(); await b.close(); }
