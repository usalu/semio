#!/usr/bin/env bun
/** 🔬️ AP1 — what the SHELL route answers for a mutation that is not a no-op.
 *
 * The live-agent gate drives the whole (f) chain with the destructive verb it found for (e), and
 * `note…editor.deleteSelection` emits nothing when nothing is selected (its own `handle` returns
 * `Emit::default()`), so `(f3) action_invoke changes the head` can never move a head. This probe
 * separates the two questions by driving an UNCONDITIONAL mutation (`addBlock`) through the same
 * gateway, and prints both history reads plus the snapshot bytes on either side of the invoke.
 *
 * It needs a live shell that answers `/__semio/agent-bridge`; it does NOT drive a browser, so the
 * shell must already be dialled in (run it while the gate's own session is up, or against a serve
 * whose page is open). Env: S_OS_MCP_LIVE_SHELL_URL, S_AGENT_BRIDGE_DIR, S_OS_MCP_PROBE_VERB,
 * S_OS_MCP_PROBE_INPUT (the verb's own JSON argument object).
 */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import { chromium } from "playwright";

const REPO_ROOT = join(dirname(fileURLToPath(new URL(import.meta.url))), "..", "..", "..", "..", "..", "..", "..");
const PLUGIN = process.env.S_OS_MCP_LIVE_PLUGIN ?? "note";
const ORIGIN = process.env.S_OS_MCP_LIVE_SHELL_URL ?? "http://127.0.0.1:6080";
const VERB = process.env.S_OS_MCP_PROBE_VERB ?? "addBlock";
const INPUT = JSON.parse(process.env.S_OS_MCP_PROBE_INPUT ?? "{}") as Record<string, unknown>;

type Entry = { readonly command: string; readonly args: readonly string[] };
const entry = (JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, Entry> }).mcpServers.semio;

class Peer {
  private readonly pending = new Map<number, (envelope: Record<string, any>) => void>();
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
  call(name: string, args: unknown): Promise<Record<string, any>> {
    return this.request("tools/call", { name, arguments: args });
  }
  stop(): void {
    this.child.kill();
  }
}

const structured = (envelope: Record<string, any>): Record<string, any> => (envelope.result?.structuredContent ?? {}) as Record<string, any>;
const show = (label: string, envelope: Record<string, any>): void => console.log(`${label} :: isError=${envelope.result?.isError} ${JSON.stringify(structured(envelope)).slice(0, 420)}`);
/** 🧬️ A snapshot is only interesting by its WHOLE pack: a truncated preview hides a change that
 * lands past the cut, so the identity of two reads is decided on a digest of every byte. */
const digest = (envelope: Record<string, any>): string => {
  const pack = String(structured(envelope).packBase64 ?? "");
  const hash = new Bun.CryptoHasher("sha256");
  hash.update(pack);
  return `packBytes=${structured(envelope).packBytes ?? "?"} sprBytes=${structured(envelope).sprBytes ?? "?"} chars=${pack.length} sha256=${hash.digest("hex").slice(0, 16)}`;
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const peer = new Peer();
try {
  await peer.request("initialize", { protocolVersion: "2025-06-18", capabilities: { roots: { listChanged: true }, elicitation: {} }, clientInfo: { name: "ap1-shell-route-probe", title: "AP1", version: "1" } });
  peer.request("notifications/initialized", {});
  await page.goto(`${ORIGIN}/?plugin=${PLUGIN}`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  for (let attempt = 0; attempt < 240; attempt += 1) {
    const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"));
    if (ready) break;
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  for (let attempt = 0; attempt < 90; attempt += 1) {
    if (structured(await peer.call("ui_focus", {})).code === undefined) break;
    await new Promise((resolve) => setTimeout(resolve, 1000));
  }
  show("context_resolve", await peer.call("context_resolve", {}));
  const found = await peer.call("capabilities_search", { query: VERB, kind: ["mutation"] });
  const rows = (structured(found).results ?? []) as Record<string, any>[];
  const target = rows.map((row) => String(row.capabilityId)).find((id) => id.startsWith(`${PLUGIN}.`) && id.endsWith(`.${VERB}`)) ?? null;
  console.log(`target=${target}`);
  if (!target) throw new Error(`no ${PLUGIN} capability ends in .${VERB}; candidates: ${rows.map((row) => row.capabilityId).join(", ").slice(0, 300)}`);
  const artifactId = `ap1-${Date.now().toString(36)}`;
  const kind = /^[^.]+\.([^@]+)@/.exec(target)?.[1] ?? "";
  show("artifact_create", await peer.call("artifact_create", { artifactId, kind }));
  show("artifact_open", await peer.call("artifact_open", { artifactId }));
  const before = await peer.call("artifact_snapshot", { artifactId });
  console.log(`snapshot BEFORE :: ${digest(before)}`);
  const prepared = await peer.call("action_prepare", { capabilityId: target, input: INPUT });
  show("action_prepare", prepared);
  show("action_invoke", await peer.call("action_invoke", { preparedActionHandle: String(structured(prepared).preparedHandle ?? "") }));
  show("artifact_open AFTER(1st)", await peer.call("artifact_open", { artifactId }));
  const after = await peer.call("artifact_snapshot", { artifactId });
  console.log(`snapshot AFTER  :: ${digest(after)}`);
  console.log(`snapshot CHANGED=${digest(before) !== digest(after)}`);
  // ⏳️ A second read three seconds later separates "the snapshot reads the wrong store" from "the
  // snapshot reads the right store before it has settled".
  await new Promise((resolve) => setTimeout(resolve, 3000));
  const settled = await peer.call("artifact_snapshot", { artifactId });
  console.log(`snapshot SETTLED:: ${digest(settled)} changed=${digest(before) !== digest(settled)}`);
  show("artifact_open AFTER", await peer.call("artifact_open", { artifactId }));
} finally {
  peer.stop();
  await browser.close();
}
