#!/usr/bin/env bun
/** 🔎️ GJ1 — which channel does a bare `.mcp.json` semio session resolve, and is a shell attached? */
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
const req = (method: string, params: unknown) => { const id = nextId++; const p = new Promise<Record<string, any>>((r) => pending.set(id, r)); child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`); void child.stdin.flush(); return Promise.race([p, new Promise<Record<string, any>>((_, rej) => setTimeout(() => rej(new Error(`${method} timed out`)), 120000))]); };
try {
  await req("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "gj1-context-probe", version: "1" } });
  const ctx = await req("tools/call", { name: "context_resolve", arguments: {} });
  console.log(JSON.stringify(ctx.result?.structuredContent ?? ctx, null, 1));
} finally { child.kill(); }
