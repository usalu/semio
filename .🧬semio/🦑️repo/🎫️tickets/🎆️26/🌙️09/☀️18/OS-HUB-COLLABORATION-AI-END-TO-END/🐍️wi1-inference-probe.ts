#!/usr/bin/env bun
/** 🔬️ WI1 — ONE `inference_run` against the real `.mcp.json` gateway, with the UNTRUNCATED reply.
 * `client-e2e` slices a tool error at 300 chars, which hides exactly the guest backtrace a trap
 * needs. Creates its own `s.wfc.bitmap` artifact, runs the inference on it, prints everything.
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const TICKET = dirname(fileURLToPath(new URL(import.meta.url)));
const REPO_ROOT = join(TICKET, "..", "..", "..", "..", "..", "..", "..");
const KIND = process.env.S_WI1_KIND ?? "s.wfc.bitmap";
const SCHEMA = process.env.S_WI1_SCHEMA ?? "s.wfc.bitmap.solve";
const PLUGIN = process.env.S_WI1_PLUGIN ?? "wfc";
const BUDGET = Number(process.env.S_WI1_BUDGET_MS ?? 600000);

type Entry = { readonly command: string; readonly args: readonly string[] };
const entry = (JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, Entry> }).mcpServers.semio;
const pending = new Map<number, (envelope: Record<string, any>) => void>();
const notifications: Record<string, any>[] = [];
let buffer = "";
let nextId = 1;
const child = Bun.spawn([entry.command, ...entry.args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env } });
void (async () => {
  const decoder = new TextDecoder();
  for await (const chunk of child.stdout) {
    buffer += decoder.decode(chunk);
    let newline = buffer.indexOf("\n");
    while (newline >= 0) {
      const line = buffer.slice(0, newline).trim();
      buffer = buffer.slice(newline + 1);
      if (line) {
        const envelope = JSON.parse(line) as Record<string, any>;
        if (typeof envelope.method === "string") notifications.push(envelope);
        else pending.get(envelope.id as number)?.(envelope), pending.delete(envelope.id as number);
      }
      newline = buffer.indexOf("\n");
    }
  }
})();
const request = (method: string, params: unknown, meta?: unknown): Promise<Record<string, any>> => {
  const id = nextId++;
  const answered = new Promise<Record<string, any>>((resolve) => pending.set(id, resolve));
  child.stdin.write(`${JSON.stringify(meta === undefined ? { jsonrpc: "2.0", id, method, params } : { jsonrpc: "2.0", id, method, params: { ...(params as object), _meta: meta } })}\n`);
  void child.stdin.flush();
  return Promise.race([answered, new Promise<Record<string, any>>((_, reject) => setTimeout(() => reject(new Error(`${method} did not answer within ${BUDGET}ms`)), BUDGET))]);
};
const call = (name: string, args: unknown, meta?: unknown) => request("tools/call", { name, arguments: args }, meta);
const structured = (envelope: Record<string, any>) => (envelope.result?.structuredContent ?? {}) as Record<string, any>;

try {
  await request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wi1-inference-probe", version: "1" } });
  const listed = await call("inference_list", {});
  const row = ((structured(listed).declared ?? []) as Record<string, any>[]).find((candidate) => candidate.artifactKind === KIND && candidate.inferenceSchema === SCHEMA);
  console.log(`inference_list: ${(structured(listed).declared ?? []).length} rows; contract = ${JSON.stringify(row?.payload ?? null).slice(0, 400)}`);
  const artifactId = `wi1-probe-${Date.now().toString(36)}`;
  const created = await call("artifact_create", { artifactId, kind: KIND });
  console.log(`artifact_create: isError=${created.result?.isError} ${JSON.stringify(structured(created)).slice(0, 200)}`);
  const snap = await call("artifact_snapshot", { artifactId });
  const pack = String(structured(snap).packBase64 ?? "");
  const spr = String(structured(snap).sprBase64 ?? "");
  console.log(`artifact_snapshot: packBytes=${structured(snap).packBytes} sprBytes=${structured(snap).sprBytes} packB64=${pack.length} sprB64=${spr.length}`);
  console.log(`PACK_B64 ${pack}`);
  console.log(`SPR_B64 ${spr}`);
  const token = `wi1-${Date.now().toString(36)}`;
  const before = notifications.length;
  const started = Date.now();
  const ran = await call("inference_run", { artifactKind: KIND, inferenceSchema: SCHEMA, pluginId: PLUGIN, artifactId, cancellationId: `${artifactId}-cancel` }, { progressToken: token });
  console.log(`inference_run: ${Math.round((Date.now() - started) / 1000)}s isError=${ran.result?.isError}`);
  for (const note of notifications.slice(before)) if (note.method === "notifications/progress") console.log(`  progress ${note.params?.progress} ${note.params?.message ?? ""}`);
  console.log(JSON.stringify(ran.result ?? ran, null, 2));
  const err = await new Response(child.stderr).text().catch(() => "");
  if (err.trim()) console.log(`--- gateway stderr ---\n${err.slice(-6000)}`);
} finally {
  child.kill();
}
