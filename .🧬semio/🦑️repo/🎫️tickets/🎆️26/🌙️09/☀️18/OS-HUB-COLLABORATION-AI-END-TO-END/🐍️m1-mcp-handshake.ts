#!/usr/bin/env bun
/** 🤝️ End-to-end handshake probe for both `.mcp.json` servers (slice M1).
 *
 * Spawns each server exactly as Claude Code does — the literal `command`/`args` out of `.mcp.json`,
 * with the CURRENT environment plus two Claude-Code harness variables (`CLAUDE_CODE_SESSION_ID`,
 * `CLAUDE_CODE_MESSAGING_TOKEN`) that the old `semio-os-mcp` process-entry seal rejected — then runs
 * `initialize` → `notifications/initialized` → `tools/list` → `resources/list` over newline-delimited
 * JSON-RPC and prints the surface. For the `semio` gateway it additionally binds a real headless
 * workspace with `--folder <repo root>` and calls one real read tool (`capabilities_search`).
 *
 * `bun .🧬semio/…/🐍️m1-mcp-handshake.ts`
 */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PROTOCOL_VERSION = "2025-06-18";
const HARNESS_ENVIRONMENT = { CLAUDE_CODE_SESSION_ID: "x", CLAUDE_CODE_MESSAGING_TOKEN: "y" } as const;
const BOOT_BUDGET_MS = 180_000;

type ServerEntry = { readonly command: string; readonly args: readonly string[] };

function mcpServers(): Record<string, ServerEntry> {
  const config = JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, ServerEntry> };
  return config.mcpServers;
}

class Peer {
  private readonly pending = new Map<number, (envelope: Record<string, any>) => void>();
  private readonly diagnostics: string[] = [];
  private buffer = "";
  private nextId = 1;
  private readonly child: ReturnType<typeof Bun.spawn>;

  constructor(entry: ServerEntry, extraArgs: readonly string[]) {
    this.child = Bun.spawn([entry.command, ...entry.args, ...extraArgs], {
      cwd: REPO_ROOT,
      stdin: "pipe",
      stdout: "pipe",
      stderr: "pipe",
      env: { ...process.env, ...HARNESS_ENVIRONMENT },
    });
    void this.pump();
    void this.collectDiagnostics();
  }

  private async pump(): Promise<void> {
    const decoder = new TextDecoder();
    for await (const chunk of this.child.stdout) {
      this.buffer += decoder.decode(chunk);
      let newline = this.buffer.indexOf("\n");
      while (newline >= 0) {
        const line = this.buffer.slice(0, newline).trim();
        this.buffer = this.buffer.slice(newline + 1);
        if (line) this.deliver(line);
        newline = this.buffer.indexOf("\n");
      }
    }
  }

  private deliver(line: string): void {
    let envelope: Record<string, any>;
    try {
      envelope = JSON.parse(line);
    } catch {
      this.diagnostics.push(`unparseable stdout line: ${line.slice(0, 200)}`);
      return;
    }
    const resolve = typeof envelope.id === "number" ? this.pending.get(envelope.id) : undefined;
    if (resolve) {
      this.pending.delete(envelope.id);
      resolve(envelope);
    }
  }

  private async collectDiagnostics(): Promise<void> {
    const decoder = new TextDecoder();
    for await (const chunk of this.child.stderr) {
      for (const line of decoder.decode(chunk).split("\n")) if (line.trim()) this.diagnostics.push(line.trim());
    }
  }

  notify(method: string, params: unknown): void {
    this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method, params })}\n`);
    void this.child.stdin.flush();
  }

  async request(method: string, params: unknown, budgetMs = BOOT_BUDGET_MS): Promise<Record<string, any>> {
    const id = this.nextId++;
    const answered = new Promise<Record<string, any>>((resolve, reject) => {
      this.pending.set(id, resolve);
      setTimeout(() => {
        if (!this.pending.delete(id)) return;
        reject(new Error(`${method} did not answer within ${budgetMs}ms; last diagnostics: ${this.diagnostics.slice(-3).join(" | ")}`));
      }, budgetMs);
    });
    this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
    await this.child.stdin.flush();
    return answered;
  }

  stop(): void {
    this.child.kill();
  }

  diagnosticsTail(count: number): readonly string[] {
    return this.diagnostics.slice(-count);
  }
}

function names(rows: unknown, key: string): string[] {
  return Array.isArray(rows) ? rows.map((row) => String((row as Record<string, unknown>)[key])).sort() : [];
}

async function handshake(label: string, entry: ServerEntry, extraArgs: readonly string[]): Promise<Peer> {
  console.log(`\n=== ${label} — ${entry.command} ${[...entry.args, ...extraArgs].join(" ")}`);
  const peer = new Peer(entry, extraArgs);
  const initialized = await peer.request("initialize", {
    protocolVersion: PROTOCOL_VERSION,
    capabilities: { roots: { listChanged: true }, sampling: {}, elicitation: {} },
    clientInfo: { name: "m1-mcp-handshake", title: "M1 MCP Handshake", version: "1" },
  });
  if (initialized.error) throw new Error(`${label}: initialize failed: ${JSON.stringify(initialized.error)}`);
  console.log(`  initialize      ok — server=${initialized.result?.serverInfo?.name}@${initialized.result?.serverInfo?.version} protocol=${initialized.result?.protocolVersion}`);
  peer.notify("notifications/initialized", {});
  const tools = await peer.request("tools/list", {});
  if (tools.error) throw new Error(`${label}: tools/list failed: ${JSON.stringify(tools.error)}`);
  const toolNames = names(tools.result?.tools, "name");
  console.log(`  tools/list      ok — ${toolNames.length} tools: ${toolNames.join(", ")}`);
  const resources = await peer.request("resources/list", {});
  if (resources.error) throw new Error(`${label}: resources/list failed: ${JSON.stringify(resources.error)}`);
  const resourceUris = names(resources.result?.resources, "uri");
  console.log(`  resources/list  ok — ${resourceUris.length} resources: ${resourceUris.join(", ")}`);
  return peer;
}

const servers = mcpServers();
const repoPeer = await handshake("repo", servers.repo!, []);
repoPeer.stop();

const osPeer = await handshake("semio (os, headless workspace bound to the repo root)", servers.semio!, ["--folder", REPO_ROOT]);
const searched = await osPeer.request("tools/call", { name: "capabilities_search", arguments: { query: "open an artifact" } });
if (searched.error) throw new Error(`semio: capabilities_search failed at the protocol level: ${JSON.stringify(searched.error)}`);
const hits = (searched.result?.structuredContent?.hits ?? []) as Record<string, unknown>[];
console.log(`  capabilities_search ok — isError=${searched.result?.isError} hits=${hits.length}`);
for (const hit of hits.slice(0, 5)) console.log(`    · ${hit.capabilityId ?? hit.id} — ${String(hit.title ?? hit.summary ?? "").slice(0, 80)}`);
const workspace = await osPeer.request("resources/read", { uri: "semio://workspace" });
if (workspace.error) throw new Error(`semio: resources/read semio://workspace failed: ${JSON.stringify(workspace.error)}`);
console.log(`  resources/read semio://workspace ok — ${String(workspace.result?.contents?.[0]?.text ?? "").slice(0, 300)}`);
console.log(`  stderr tail: ${osPeer.diagnosticsTail(2).join(" | ").slice(0, 300)}`);
osPeer.stop();

console.log(`\nboth .mcp.json servers answered initialize + tools/list + resources/list (repo root ${dirname(join(REPO_ROOT, "x"))})`);
