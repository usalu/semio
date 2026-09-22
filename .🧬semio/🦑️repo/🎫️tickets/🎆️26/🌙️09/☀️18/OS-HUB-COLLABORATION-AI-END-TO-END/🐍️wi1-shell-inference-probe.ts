#!/usr/bin/env bun
/** 🔬️ WI1 — a USER runs an inference on an artifact, in the LIVE `s` shell.
 *
 * Drives the identical chain a human's agent does: the `.mcp.json` `semio` server (stdio), dialled
 * into the running shell, creates an `s.wfc.bitmap` artifact, opens it so the human sees it, then
 * calls `inference_run` with `artifactId` and `_meta.progressToken` and records every
 * `notifications/progress` row that comes back. Screenshots the shell before and after, so the
 * capture shows the artifact on screen and the agent panel carrying the run.
 *
 * Env: S_OS_MCP_LIVE_SHELL_URL (default http://127.0.0.1:6200), S_WI1_PLUGIN (default wfc),
 *      S_WI1_KIND (default s.wfc.bitmap), S_WI1_SCHEMA (default s.wfc.bitmap.solve).
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { chromium } from "playwright";

const TICKET = dirname(fileURLToPath(new URL(import.meta.url)));
const REPO_ROOT = join(TICKET, "..", "..", "..", "..", "..", "..", "..");
const GEN = join(TICKET, "🗑️generated");
const ORIGIN = process.env.S_OS_MCP_LIVE_SHELL_URL ?? "http://127.0.0.1:6200";
const PLUGIN = process.env.S_WI1_PLUGIN ?? "wfc";
const KIND = process.env.S_WI1_KIND ?? "s.wfc.bitmap";
const SCHEMA = process.env.S_WI1_SCHEMA ?? "s.wfc.bitmap.solve";

type Entry = { readonly command: string; readonly args: readonly string[] };
const entry = (JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, Entry> }).mcpServers.semio;

class Peer {
  private readonly pending = new Map<number, (envelope: Record<string, any>) => void>();
  readonly notifications: Record<string, any>[] = [];
  private buffer = "";
  private nextId = 1;
  private readonly child;
  constructor() {
    this.child = Bun.spawn([entry.command, ...entry.args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env } });
    void this.pump();
  }
  private async pump(): Promise<void> {
    const decoder = new TextDecoder();
    for await (const chunk of this.child.stdout) {
      this.buffer += decoder.decode(chunk);
      let newline = this.buffer.indexOf("\n");
      while (newline >= 0) {
        const line = this.buffer.slice(0, newline).trim();
        this.buffer = this.buffer.slice(newline + 1);
        if (line) {
          const envelope = JSON.parse(line) as Record<string, any>;
          if (typeof envelope.method === "string") {
            this.notifications.push(envelope);
            if (envelope.method === "elicitation/create" && envelope.id !== undefined) this.write({ jsonrpc: "2.0", id: envelope.id, result: { action: "accept", content: { approve: true } } });
          } else {
            const resolve = this.pending.get(envelope.id as number);
            if (resolve) {
              this.pending.delete(envelope.id as number);
              resolve(envelope);
            }
          }
        }
        newline = this.buffer.indexOf("\n");
      }
    }
  }
  private write(envelope: unknown): void {
    this.child.stdin.write(`${JSON.stringify(envelope)}\n`);
    void this.child.stdin.flush();
  }
  request(method: string, params: unknown): Promise<Record<string, any>> {
    const id = this.nextId++;
    const answered = new Promise<Record<string, any>>((resolve) => this.pending.set(id, resolve));
    this.write({ jsonrpc: "2.0", id, method, params });
    return answered;
  }
  call(name: string, args: unknown, meta?: unknown): Promise<Record<string, any>> {
    return this.request("tools/call", meta === undefined ? { name, arguments: args } : { name, arguments: args, _meta: meta });
  }
  stop(): void {
    this.child.kill();
  }
}

const structured = (envelope: Record<string, any>): Record<string, any> => (envelope.result?.structuredContent ?? {}) as Record<string, any>;
const show = (label: string, envelope: Record<string, any>): void => console.log(`${label} :: isError=${envelope.result?.isError} ${JSON.stringify(structured(envelope)).slice(0, 600)}`);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const peer = new Peer();
try {
  await peer.request("initialize", { protocolVersion: "2025-06-18", capabilities: { roots: { listChanged: true }, elicitation: {} }, clientInfo: { name: "wi1-shell-inference-probe", title: "WI1", version: "1" } });
  peer.request("notifications/initialized", {});
  await page.goto(`${ORIGIN}/?plugin=${PLUGIN}`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  for (let attempt = 0; attempt < 300; attempt += 1) {
    const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
    if (ready) break;
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  console.log(`shell ready=${await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))}`);
  for (let attempt = 0; attempt < 120; attempt += 1) {
    if (structured(await peer.call("ui_focus", {})).code === undefined) break;
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  show("context_resolve", await peer.call("context_resolve", {}));

  // 📜️ What the agent can READ about this inference before it sends anything.
  const listed = await peer.call("inference_list", {});
  const declared = ((structured(listed).declared ?? []) as Record<string, any>[]).find((row) => row.artifactKind === KIND && row.inferenceSchema === SCHEMA);
  console.log(`inference_list :: ${(structured(listed).declared ?? []).length} declared; pinned row payload=${JSON.stringify(declared?.payload ?? null).slice(0, 700)}`);

  const artifactId = `wi1-${Date.now().toString(36)}`;
  show("artifact_create", await peer.call("artifact_create", { artifactId, kind: KIND }));
  show("artifact_open", await peer.call("artifact_open", { artifactId }));
  await new Promise((resolve) => setTimeout(resolve, 2000));
  await page.screenshot({ path: join(GEN, "wi1-shell-before-inference.png"), fullPage: false });

  const progressToken = `wi1-${Date.now().toString(36)}`;
  const before = peer.notifications.length;
  const started = Date.now();
  const ran = await peer.call("inference_run", { artifactKind: KIND, inferenceSchema: SCHEMA, pluginId: PLUGIN, artifactId, cancellationId: `${artifactId}-cancel` }, { progressToken });
  const seconds = Math.round((Date.now() - started) / 1000);
  const progress = peer.notifications.slice(before).filter((envelope) => envelope.method === "notifications/progress" && envelope.params?.progressToken === progressToken);
  console.log(`inference_run :: ${seconds}s isError=${ran.result?.isError} progressRows=${progress.length}`);
  for (const row of progress) console.log(`  progress ${row.params?.progress} ${row.params?.message ?? ""}`);
  show("inference_run", ran);
  const jobId = String(structured(ran).jobId ?? (structured(ran).details as Record<string, any> | undefined)?.jobId ?? "");
  if (jobId) show("job_get", await peer.call("job_get", { jobId }));
  await new Promise((resolve) => setTimeout(resolve, 2000));
  await page.screenshot({ path: join(GEN, "wi1-shell-after-inference.png"), fullPage: false });
  console.log(`screenshots: 🗑️generated/wi1-shell-{before,after}-inference.png`);
} finally {
  peer.stop();
  await browser.close();
}
