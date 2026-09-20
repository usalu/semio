#!/usr/bin/env bun
/** 🧭️ MCP protocol-conformance probe for the `semio` gateway (slice M5br).
 *
 * Spawns the `semio` server exactly as `.mcp.json` declares it — the literal `command`/`args` a
 * Claude Desktop / Claude Code host uses, plus the two Claude-Code harness variables — and drives
 * the spec surface a real client depends on over newline-delimited JSON-RPC stdio:
 *
 *   1. protocol version negotiation (every supported version, an unsupported one, and none at all)
 *   2. cursor pagination on `tools/list` / `resources/list` / `resources/templates/list` /
 *      `prompts/list`, walked to exhaustion, plus a cursor this server never minted
 *   3. `resources/subscribe` validation and a real `notifications/resources/updated` push
 *   4. `_meta.progressToken` → `notifications/progress`, and `notifications/cancelled`
 *   5. `outputSchema` declared by every tool and `structuredContent` present on every OK result
 *   6. `isError: true` for a tool's own failure vs a JSON-RPC error for a protocol failure
 *
 * Every row prints PASS/FAIL with the wire evidence; the process exits non-zero on any red row.
 * `bun .🧬semio/…/🐍️m5br-conformance-probe.ts`
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const HARNESS_ENVIRONMENT = { CLAUDE_CODE_SESSION_ID: "m5br-conformance", CLAUDE_CODE_MESSAGING_TOKEN: "m5br-conformance" } as const;
const BOOT_BUDGET_MS = 240_000;

type ServerEntry = { readonly command: string; readonly args: readonly string[] };
type Envelope = Record<string, any>;

function semioServer(): ServerEntry {
  const config = JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, ServerEntry> };
  const entry = config.mcpServers.semio;
  if (!entry) throw new Error(".mcp.json declares no `semio` server");
  return entry;
}

class Peer {
  private readonly pending = new Map<number, (envelope: Envelope) => void>();
  private readonly diagnostics: string[] = [];
  private readonly notifications: Envelope[] = [];
  private buffer = "";
  private nextId = 1;
  private readonly child: ReturnType<typeof Bun.spawn>;

  constructor(entry: ServerEntry, extraArgs: readonly string[]) {
    this.child = Bun.spawn([entry.command, ...entry.args, ...extraArgs], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env, ...HARNESS_ENVIRONMENT } });
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
    let envelope: Envelope;
    try {
      envelope = JSON.parse(line);
    } catch {
      this.diagnostics.push(`unparseable stdout line: ${line.slice(0, 200)}`);
      return;
    }
    if (envelope.id === undefined || envelope.id === null) {
      this.notifications.push(envelope);
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
    for await (const chunk of this.child.stderr) for (const line of decoder.decode(chunk).split("\n")) if (line.trim()) this.diagnostics.push(line.trim());
  }

  notify(method: string, params: unknown): void {
    this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", method, params })}\n`);
    void this.child.stdin.flush();
  }

  async request(method: string, params: unknown, budgetMs = BOOT_BUDGET_MS): Promise<Envelope> {
    const id = this.nextId++;
    const answered = new Promise<Envelope>((resolve, reject) => {
      this.pending.set(id, resolve);
      setTimeout(() => {
        if (!this.pending.delete(id)) return;
        reject(new Error(`${method} did not answer within ${budgetMs}ms; diagnostics: ${this.diagnostics.slice(-3).join(" | ")}`));
      }, budgetMs);
    });
    this.child.stdin.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
    await this.child.stdin.flush();
    const envelope = await answered;
    return { ...envelope, requestId: id };
  }

  serverNotifications(): readonly Envelope[] {
    return this.notifications;
  }

  diagnosticsTail(count: number): readonly string[] {
    return this.diagnostics.slice(-count);
  }

  stop(): void {
    this.child.kill();
  }
}

const rows: { step: string; ok: boolean; detail: string }[] = [];
function record(step: string, ok: boolean, detail: string): void {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);
}

async function initialized(peer: Peer, protocolVersion: string | undefined): Promise<Envelope> {
  const parameters: Record<string, unknown> = { capabilities: { roots: { listChanged: true }, sampling: {}, elicitation: {} }, clientInfo: { name: "m5br-conformance", title: "M5br conformance", version: "1" } };
  if (protocolVersion !== undefined) parameters.protocolVersion = protocolVersion;
  const reply = await peer.request("initialize", parameters);
  if (!reply.error) peer.notify("notifications/initialized", {});
  return reply;
}

/** 📄️ Walks one cursor-paginated list exactly as a spec-strict client does. */
async function walk(peer: Peer, method: string, key: string, identify: (row: any) => string): Promise<{ ok: boolean; detail: string; pages: number; visited: string[] }> {
  const visited: string[] = [];
  const seen = new Set<string>();
  let cursor: string | undefined;
  let pages = 0;
  while (pages < 64) {
    const page = await peer.request(method, cursor === undefined ? {} : { cursor });
    if (page.error) return { ok: false, detail: `${method} page ${pages + 1}: ${JSON.stringify(page.error)}`, pages, visited };
    visited.push(...((page.result?.[key] ?? []) as unknown[]).map(identify));
    pages += 1;
    const next = page.result?.nextCursor as string | undefined;
    if (next === undefined || next === null) return { ok: true, detail: `${pages} page(s), ${visited.length} entry(ies)`, pages, visited };
    if (seen.has(next)) return { ok: false, detail: `${method} repeated cursor ${next} — the walk would never terminate`, pages, visited };
    seen.add(next);
    cursor = next;
  }
  return { ok: false, detail: `${method} did not finish within 64 pages`, pages, visited };
}

const entry = semioServer();
console.log(`probe: ${entry.command} ${entry.args.join(" ")} (cwd ${REPO_ROOT})`);

//#region 🧪️Negotiation
const SUPPORTED = ["2026-07-28", "2025-11-25", "2025-06-18"];
for (const version of SUPPORTED) {
  const peer = new Peer(entry, []);
  try {
    const reply = await initialized(peer, version);
    record(`initialize negotiates ${version}`, !reply.error && reply.result?.protocolVersion === version, reply.error ? JSON.stringify(reply.error) : `protocolVersion=${reply.result?.protocolVersion} server=${reply.result?.serverInfo?.name}@${reply.result?.serverInfo?.version}`);
  } finally {
    peer.stop();
  }
}
{
  const peer = new Peer(entry, []);
  try {
    const reply = await initialized(peer, "1999-01-01");
    const answered = reply.result?.protocolVersion as string | undefined;
    record("initialize answers an unsupported version with a supported one", !reply.error && Boolean(answered) && SUPPORTED.includes(answered!), reply.error ? JSON.stringify(reply.error) : `asked 1999-01-01, offered ${answered}`);
  } finally {
    peer.stop();
  }
}
//#endregion 🧪️Negotiation

const peer = new Peer(entry, []);
let exitCode = 0;
try {
  const handshake = await initialized(peer, "2025-06-18");
  const capabilities = handshake.result?.capabilities ?? {};
  record("initialize (the version Claude Code sends)", !handshake.error, handshake.error ? JSON.stringify(handshake.error) : `capabilities=${JSON.stringify(capabilities)}`);

  //#region 🧪️Pagination
  const unpagedTools = await peer.request("tools/list", {});
  const toolRows = (unpagedTools.result?.tools ?? []) as any[];
  const toolNames = toolRows.map((tool) => String(tool.name));
  const toolWalk = await walk(peer, "tools/list", "tools", (tool: any) => String(tool.name));
  record("tools/list pagination walk", toolWalk.ok && toolWalk.visited.join(" ") === toolNames.join(" "), toolWalk.ok ? `${toolWalk.detail}; order identical to the unpaged list (${toolNames.length} tools)` : toolWalk.detail);
  const foreign = await peer.request("tools/list", { cursor: "someone-elses-cursor" });
  record("tools/list rejects a cursor this server never minted", foreign.error?.code === -32602, foreign.error ? `code=${foreign.error.code} ${foreign.error.message}` : "a foreign cursor was ACCEPTED");
  const resourceWalk = await walk(peer, "resources/list", "resources", (resource: any) => String(resource.uri));
  const duplicates = resourceWalk.visited.filter((uri, index) => resourceWalk.visited.indexOf(uri) !== index);
  record("resources/list pagination + unique URIs", resourceWalk.ok && duplicates.length === 0, resourceWalk.ok ? `${resourceWalk.detail}, ${duplicates.length} duplicate URI(s)${duplicates.length ? `: ${duplicates.join(", ")}` : ""}` : resourceWalk.detail);
  const templateWalk = await walk(peer, "resources/templates/list", "resourceTemplates", (row: any) => String(row.uriTemplate ?? row.uri));
  record("resources/templates/list pagination walk", templateWalk.ok, templateWalk.detail);
  const promptWalk = await walk(peer, "prompts/list", "prompts", (row: any) => String(row.name));
  record("prompts/list pagination walk", promptWalk.ok, promptWalk.detail);
  //#endregion 🧪️Pagination

  //#region 🧪️StructuredOutput
  const withoutSchema = toolRows.filter((tool) => !tool.outputSchema).map((tool) => String(tool.name));
  record("every tool declares an outputSchema", withoutSchema.length === 0, withoutSchema.length === 0 ? `${toolRows.length} tools, all with an outputSchema` : `missing on: ${withoutSchema.join(", ")}`);
  const withoutInput = toolRows.filter((tool) => !tool.inputSchema).map((tool) => String(tool.name));
  record("every tool declares an inputSchema", withoutInput.length === 0, withoutInput.length === 0 ? `${toolRows.length} tools` : `missing on: ${withoutInput.join(", ")}`);
  //#endregion 🧪️StructuredOutput

  //#region 🧪️ErrorSurface
  const unknownTool = await peer.request("tools/call", { name: "there_is_no_such_tool", arguments: {} });
  record("an unknown tool is a JSON-RPC error, not a tool result", Boolean(unknownTool.error), unknownTool.error ? `code=${unknownTool.error.code} ${unknownTool.error.message}` : `answered a result: ${JSON.stringify(unknownTool.result).slice(0, 200)}`);
  const badArguments = await peer.request("tools/call", { name: "artifact_snapshot", arguments: { artifactId: "there-is-no-such-artifact" } });
  const isErrorResult = badArguments.result?.isError === true;
  record("a tool's own failure is isError:true inside a RESULT", isErrorResult && !badArguments.error, badArguments.error ? `answered a JSON-RPC error instead: code=${badArguments.error.code} ${badArguments.error.message}` : `isError=${badArguments.result?.isError} structured=${JSON.stringify(badArguments.result?.structuredContent).slice(0, 200)}`);
  const okCall = await peer.request("tools/call", { name: "capabilities_search", arguments: { query: "edit the document" } });
  record("a successful tool result carries structuredContent", !okCall.error && okCall.result?.isError !== true && okCall.result?.structuredContent !== undefined, okCall.error ? JSON.stringify(okCall.error) : `isError=${okCall.result?.isError} keys=${Object.keys(okCall.result?.structuredContent ?? {}).join(",")}`);
  //#endregion 🧪️ErrorSurface

  //#region 🧪️Subscriptions
  const refused = await peer.request("resources/subscribe", { uri: "semio://there-is-no-such-resource" });
  record("resources/subscribe refuses an unserveable URI", Boolean(refused.error), refused.error ? `code=${refused.error.code} ${refused.error.message.slice(0, 140)}` : "an unserveable URI was accepted into a silent forever-wait");
  const probeArtifactId = `m5br-${Date.now().toString(36)}`;
  const created = await peer.request("tools/call", { name: "artifact_create", arguments: { artifactId: probeArtifactId, kind: "os.agent.probe/v1" } });
  const artifactId = created.result?.structuredContent?.artifactId as string | undefined;
  record("artifact_create (the subscription subject)", !created.error && created.result?.isError !== true && Boolean(artifactId), created.error ? JSON.stringify(created.error) : `isError=${created.result?.isError} artifactId=${artifactId} ${JSON.stringify(created.result?.structuredContent).slice(0, 200)}`);
  if (artifactId) {
    const uri = `semio://artifact/${artifactId}`;
    const subscribed = await peer.request("resources/subscribe", { uri });
    record("resources/subscribe accepts a listed artifact URI", !subscribed.error, subscribed.error ? JSON.stringify(subscribed.error) : `subscribed to ${uri}`);
    const before = peer.serverNotifications().length;
    const validated = await peer.request("tools/call", { name: "artifact_validate", arguments: { artifactId } });
    const snapshot = await peer.request("tools/call", { name: "artifact_snapshot", arguments: { artifactId } });
    const reopened = await peer.request("tools/call", { name: "artifact_open", arguments: { artifactId } });
    const updates = peer.serverNotifications().slice(before).filter((envelope) => envelope.method === "notifications/resources/updated");
    const uris = updates.map((envelope) => String(envelope.params?.uri ?? ""));
    record("notifications/resources/updated names the subscribed URI", uris.includes(uri), `${updates.length} update(s): ${uris.join(", ") || "<none>"} (validate isError=${validated.result?.isError} snapshot isError=${snapshot.result?.isError} open isError=${reopened.result?.isError})`);
    const unsubscribed = await peer.request("resources/unsubscribe", { uri });
    record("resources/unsubscribe", !unsubscribed.error, unsubscribed.error ? JSON.stringify(unsubscribed.error) : `unsubscribed from ${uri}`);
  }
  //#endregion 🧪️Subscriptions

  //#region 🧪️ProgressAndCancellation
  const listed = await peer.request("tools/call", { name: "inference_list", arguments: {} });
  const services = (listed.result?.structuredContent?.declared ?? listed.result?.structuredContent?.services ?? []) as any[];
  record("inference_list", !listed.error && listed.result?.isError !== true, listed.error ? JSON.stringify(listed.error) : `isError=${listed.result?.isError} services=${services.length} ${JSON.stringify(listed.result?.structuredContent).slice(0, 200)}`);
  // 🛑️ Cancellation first, against a request this connection already completed: the spec forbids
  // answering a notification either way, so what is proven here is that it is accepted and the
  // server stays responsive.
  peer.notify("notifications/cancelled", { requestId: listed.requestId, reason: "m5br cancellation probe" });
  const alive = await peer.request("ping", {}, 60_000);
  record("notifications/cancelled is accepted and unanswered", !alive.error, alive.error ? `the server stopped answering after notifications/cancelled: ${JSON.stringify(alive.error)}` : "cancelled accepted with no response; ping still answers");

  // 📈️ The progress push, LAST, because `inference_run` drives a real plugin guest and can outlive
  // any client budget. The point is not that it finishes — it is that the rows arrive WHILE it
  // runs, which is the whole difference between a pushed progress stream and polling `job_get`.
  const service = services[0];
  if (service) {
    const progressToken = `m5br-${Date.now().toString(36)}`;
    const before = peer.serverNotifications().length;
    const started = Date.now();
    let ran: Envelope | undefined;
    try {
      ran = await peer.request("tools/call", { name: "inference_run", arguments: { artifactKind: service.artifactKind, inferenceSchema: service.inferenceSchema, pluginId: service.contributor || service.owner }, _meta: { progressToken } }, 120_000);
    } catch {
      ran = undefined;
    }
    const progress = peer.serverNotifications().slice(before).filter((envelope) => envelope.method === "notifications/progress" && envelope.params?.progressToken === progressToken);
    const verdict = ran === undefined ? `inference_run still running after ${Math.round((Date.now() - started) / 1000)}s` : `inference_run answered isError=${ran.result?.isError}`;
    record("notifications/progress carries the client's _meta.progressToken", progress.length > 0, progress.length > 0 ? `${progress.length} row(s) pushed mid-call, progress=${progress.map((envelope) => envelope.params?.progress).join("→")} (${verdict})` : `no progress row carried ${progressToken} (${verdict}) ${JSON.stringify(ran?.result?.structuredContent ?? ran?.error ?? {}).slice(0, 200)}`);
  } else {
    record("notifications/progress carries the client's _meta.progressToken", false, "no installed plugin declared an inference service, so no job-minting tool could be driven");
  }
  //#endregion 🧪️ProgressAndCancellation

  console.log(`\nstderr tail: ${peer.diagnosticsTail(3).join(" | ").slice(0, 400)}`);
} finally {
  const failed = rows.filter((row) => !row.ok);
  console.log(`\nm5br-conformance: ${rows.length - failed.length}/${rows.length} rows green.`);
  if (failed.length) console.log(`red: ${failed.map((row) => row.step).join(" | ")}`);
  exitCode = failed.length ? 1 : 0;
  peer.stop();
}
process.exit(exitCode);
