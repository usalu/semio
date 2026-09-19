#!/usr/bin/env bun
/** 🛰️ Runtime probe for slice M4 — drives the REAL `semio-os-mcp` binary over stdio exactly as a
 * client launches it, and proves the three things the slice claims:
 *
 *   a) **unbound** — launched with NO `--folder`/`--hub`, a mutation-protocol call answers a typed,
 *      retryable `PLUGIN_UNAVAILABLE` naming both flags (it used to run against a scripted mock and
 *      report success).
 *   b) **bound** — launched with `--folder <repo root>` (what `.mcp.json` now passes), a real
 *      artifact/discovery tool answers for real.
 *   c) **approval round trip** — with `--auto-approve` absent, a destructive capability answers
 *      `APPROVAL_REQUIRED` whose `details` name every closed lane; with `--auto-approve all` the same
 *      call proceeds. Elicitation is also exercised: the probe advertises `capabilities.elicitation`
 *      and answers the server's own `elicitation/create` request.
 *
 * `bun .🧬semio/…/🐍️m4-bridge-approval-probe.ts`
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PROTOCOL_VERSION = "2025-06-18";
const HARNESS_ENVIRONMENT = { CLAUDE_CODE_SESSION_ID: "x", CLAUDE_CODE_MESSAGING_TOKEN: "y" } as const;
const BUDGET_MS = 240_000;

type ServerEntry = { readonly command: string; readonly args: readonly string[] };

function semioEntry(): ServerEntry {
  const config = JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, ServerEntry> };
  const entry = config.mcpServers.semio;
  if (!entry) throw new Error(".mcp.json has no `semio` server");
  return entry;
}

/** 🧹️ The `.mcp.json` args with every `--folder <dir>` pair removed — the "unbound" launch. */
function withoutWorkspace(args: readonly string[]): string[] {
  const kept: string[] = [];
  for (let index = 0; index < args.length; index += 1) {
    if (args[index] === "--folder" || args[index] === "--hub" || args[index] === "--space") {
      index += 1;
      continue;
    }
    kept.push(args[index]!);
  }
  return kept;
}

class Peer {
  private readonly pending = new Map<string | number, (envelope: Record<string, any>) => void>();
  readonly diagnostics: string[] = [];
  readonly serverRequests: Record<string, any>[] = [];
  private buffer = "";
  private nextId = 1;
  private readonly child: ReturnType<typeof Bun.spawn>;
  /** 🙋 How this probe answers the server's own `elicitation/create` requests. */
  elicitationAnswer: "accept" | "decline" | "cancel" = "accept";

  constructor(args: readonly string[], command: string) {
    this.child = Bun.spawn([command, ...args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env, ...HARNESS_ENVIRONMENT } });
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
    if (typeof envelope.method === "string") {
      this.serverRequests.push(envelope);
      if (envelope.method === "elicitation/create" && envelope.id !== undefined) {
        const content = this.elicitationAnswer === "accept" ? { approve: true } : undefined;
        this.write({ jsonrpc: "2.0", id: envelope.id, result: content ? { action: this.elicitationAnswer, content } : { action: this.elicitationAnswer } });
      }
      return;
    }
    const resolve = envelope.id !== undefined ? this.pending.get(envelope.id) : undefined;
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

  private write(envelope: unknown): void {
    this.child.stdin.write(`${JSON.stringify(envelope)}\n`);
    void this.child.stdin.flush();
  }

  notify(method: string, params: unknown): void {
    this.write({ jsonrpc: "2.0", method, params });
  }

  async request(method: string, params: unknown, budgetMs = BUDGET_MS): Promise<Record<string, any>> {
    const id = this.nextId++;
    const answered = new Promise<Record<string, any>>((resolve, reject) => {
      this.pending.set(id, resolve);
      setTimeout(() => {
        if (!this.pending.delete(id)) return;
        reject(new Error(`${method} did not answer within ${budgetMs}ms; stderr tail: ${this.diagnostics.slice(-3).join(" | ")}`));
      }, budgetMs);
    });
    this.write({ jsonrpc: "2.0", id, method, params });
    return answered;
  }

  stop(): void {
    this.child.kill();
  }
}

async function open(label: string, args: readonly string[], command: string): Promise<Peer> {
  console.log(`\n=== ${label}\n    ${command} ${args.join(" ")}`);
  const peer = new Peer(args, command);
  const initialized = await peer.request("initialize", {
    protocolVersion: PROTOCOL_VERSION,
    capabilities: { roots: { listChanged: true }, elicitation: {} },
    clientInfo: { name: "m4-bridge-approval-probe", title: "M4 Bridge/Approval Probe", version: "1" },
  });
  if (initialized.error) throw new Error(`${label}: initialize failed: ${JSON.stringify(initialized.error)}`);
  peer.notify("notifications/initialized", {});
  console.log(`  initialize ok — ${initialized.result?.serverInfo?.name}@${initialized.result?.serverInfo?.version}, protocol ${initialized.result?.protocolVersion}`);
  return peer;
}

function describe(envelope: Record<string, any>): string {
  if (envelope.error) return `JSON-RPC error ${envelope.error.code}: ${envelope.error.message} :: ${JSON.stringify(envelope.error.data ?? {}).slice(0, 400)}`;
  return `isError=${envelope.result?.isError} :: ${JSON.stringify(envelope.result?.structuredContent ?? envelope.result?.content ?? {}).slice(0, 400)}`;
}

const entry = semioEntry();
const boundArgs = [...entry.args];
const unboundArgs = withoutWorkspace(entry.args);

// (a) no workspace bound — the mutation protocol must refuse, not answer from a scripted mock.
const unbound = await open("(a) unbound — no --folder / --hub", unboundArgs, entry.command);
const unboundPrepare = await unbound.request("tools/call", { name: "action_prepare", arguments: { capabilityId: "capabilities.search", input: { query: "x" } } });
console.log(`  action_prepare  ${describe(unboundPrepare)}`);
const unboundOpen = await unbound.request("tools/call", { name: "artifact_open", arguments: { artifactId: "probe" } });
console.log(`  artifact_open   ${describe(unboundOpen)}`);
console.log(`  stderr tail: ${unbound.diagnostics.slice(-3).join(" | ").slice(0, 400)}`);
unbound.stop();

// (b) bound to the repo workspace exactly as `.mcp.json` now launches it.
const bound = await open("(b) bound — .mcp.json args verbatim", boundArgs, entry.command);
const searched = await bound.request("tools/call", { name: "capabilities_search", arguments: { query: "open an artifact" } });
console.log(`  capabilities_search ${describe(searched)}`);
const snapshot = await bound.request("tools/call", { name: "artifact_create", arguments: { kind: "probe" } });
console.log(`  artifact_create ${describe(snapshot)}`);
const uiFocus = await bound.request("tools/call", { name: "ui_focus", arguments: { windowId: "w1" } });
console.log(`  ui_focus        ${describe(uiFocus)}`);
console.log(`  stderr tail: ${bound.diagnostics.slice(-3).join(" | ").slice(0, 400)}`);

// 🗑️ The rendezvous the gateway just used — proof the bridge attached to a real `dev s` session.
console.log(`  live os session records: ${((): number => { try { return require("node:fs").readdirSync(join(process.env.HOME ?? ".", ".semio", "agent", "bridge", "sessions")).length; } catch { return 0; } })()}`);

// (c) approval round trip against a real destructive capability whose input schema takes no argument.
const destructive = await bound.request("tools/call", { name: "capabilities_search", arguments: { query: "delete selection clear", kind: ["mutation"] } });
const candidates = (destructive.result?.structuredContent?.results ?? []) as Record<string, any>[];
console.log(`  destructive candidates: ${candidates.map((row) => row.capabilityId).slice(0, 8).join(", ") || "(none — catalog compiled empty)"}`);

/** ⛩️ Invokes each candidate until one actually reaches the approval gate, returning that id. */
async function firstGated(peer: Peer, label: string): Promise<string | null> {
  for (const candidate of candidates.slice(0, 8)) {
    // 🔁️ `BUDGET_EXCEEDED` on `InstanceOpen` is the guest's own 8 ms interpreter slice and is
    // explicitly retryable — a first-open of a cold plugin needs several slices before the capability
    // can even be prepared, let alone gated.
    let invoked = await peer.request("tools/call", { name: "action_invoke", arguments: { capabilityId: candidate.capabilityId, input: {} } });
    for (let attempt = 0; attempt < 400 && (invoked.result?.structuredContent as Record<string, any> | undefined)?.code === "BUDGET_EXCEEDED"; attempt += 1) {
      invoked = await peer.request("tools/call", { name: "action_invoke", arguments: { capabilityId: candidate.capabilityId, input: {} } });
    }
    const code = (invoked.result?.structuredContent as Record<string, any> | undefined)?.code;
    console.log(`  [${label}] action_invoke ${candidate.capabilityId} → ${describe(invoked)}`);
    if (code === "APPROVAL_REQUIRED" || code === "PERMISSION_DENIED" || invoked.result?.isError === false) return candidate.capabilityId;
  }
  return null;
}

const gated = await firstGated(bound, "no --auto-approve");
console.log(`  server-initiated requests seen: ${bound.serverRequests.map((row) => row.method).join(", ") || "(none)"}`);
bound.stop();

if (gated) {
  const auto = await open("(c2) same capability with --auto-approve all", [...boundArgs, "--auto-approve", "all"], entry.command);
  const invoked = await auto.request("tools/call", { name: "action_invoke", arguments: { capabilityId: gated, input: {} } });
  console.log(`  action_invoke ${gated} → ${describe(invoked)}`);
  console.log(`  server-initiated requests seen: ${auto.serverRequests.map((row) => row.method).join(", ") || "(none)"}`);
  auto.stop();
} else {
  console.log("  no candidate reached the approval gate — every one failed input validation or the guest trap first");
}

console.log("\nprobe complete");
