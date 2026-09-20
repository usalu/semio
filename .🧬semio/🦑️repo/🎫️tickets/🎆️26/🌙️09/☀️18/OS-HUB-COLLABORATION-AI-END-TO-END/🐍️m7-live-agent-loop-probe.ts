#!/usr/bin/env bun
/** 🛰️ Slice M7 — the LIVE agent loop, end to end, with a real MCP stdio client on one side and a
 * real React `dev` session in a real browser on the other. Nothing here is mocked: the gateway is
 * the shipped `semio-os-mcp` binary launched exactly as `.mcp.json` launches it, the shell is a
 * running vite dev session, and the only thing between them is the rendezvous M4 built and the
 * dialling seam M7 closed.
 *
 * The transcript it proves, in order:
 *   (a) the React shell dials the gateway and agent presence appears in the shell;
 *   (b) a tool call shows as a running row and then as a result;
 *   (c) U1's cancel button cancels an in-flight call;
 *   (d) `ui_reveal` / `ui_focus` visibly act on the live shell;
 *   (e) a destructive capability raises the approval affordance; Approve Once lets it through, Deny
 *       returns the typed refusal, and a silent shell returns the typed timeout.
 *
 * `bun .🧬semio/…/🐍️m7-live-agent-loop-probe.ts`
 * env: SEMIO_M7_PORT (default 6080), SEMIO_M7_PLUGIN (default note), SEMIO_M7_HEADLESS=0 to watch.
 */
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { chromium, type Page } from "playwright";

const REPO_ROOT = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const PROTOCOL_VERSION = "2025-06-18";
const PORT = process.env.SEMIO_M7_PORT ?? "6080";
const PLUGIN = process.env.SEMIO_M7_PLUGIN ?? "note";
const URL = process.env.SEMIO_M7_URL ?? `http://127.0.0.1:${PORT}/?plugin=${PLUGIN}`;
const OFFER_URL = `http://127.0.0.1:${PORT}/__semio/agent-bridge`;
const BUDGET_MS = 240_000;

type ServerEntry = { readonly command: string; readonly args: readonly string[] };

const outcomes: { step: string; ok: boolean; detail: string }[] = [];
function record(step: string, ok: boolean, detail: string): void {
  outcomes.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"} ${step} :: ${detail}`);
}

function semioEntry(): ServerEntry {
  const config = JSON.parse(readFileSync(join(REPO_ROOT, ".mcp.json"), "utf8")) as { mcpServers: Record<string, ServerEntry> };
  const entry = config.mcpServers.semio;
  if (!entry) throw new Error(".mcp.json has no `semio` server");
  return entry;
}

/** 🔌️ One live `semio-os-mcp` stdio connection, driven exactly as a real MCP client drives it. */
class Peer {
  private readonly pending = new Map<string | number, (envelope: Record<string, any>) => void>();
  readonly diagnostics: string[] = [];
  readonly serverRequests: Record<string, any>[] = [];
  private buffer = "";
  private nextId = 1;
  private readonly child: ReturnType<typeof Bun.spawn>;
  /** 🙋 How this probe answers `elicitation/create`: `silent` answers nothing at all, which is what
   * proves the new wall-clock timeout at runtime. */
  elicitationAnswer: "accept" | "decline" | "cancel" | "silent" = "accept";

  constructor(args: readonly string[], command: string, environment: Record<string, string> = {}) {
    this.child = Bun.spawn([command, ...args], { cwd: REPO_ROOT, stdin: "pipe", stdout: "pipe", stderr: "pipe", env: { ...process.env, ...environment } });
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
      if (envelope.method === "elicitation/create" && envelope.id !== undefined && this.elicitationAnswer !== "silent") {
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

  call(name: string, args: unknown, budgetMs = BUDGET_MS): Promise<Record<string, any>> {
    return this.request("tools/call", { name, arguments: args }, budgetMs);
  }

  stop(): void {
    this.child.kill();
  }
}

async function open(label: string, args: readonly string[], command: string, elicitation: boolean, environment: Record<string, string> = {}): Promise<Peer> {
  console.log(`\n=== ${label}\n    ${command} ${args.join(" ")}`);
  const peer = new Peer(args, command, environment);
  const capabilities: Record<string, unknown> = { roots: { listChanged: true } };
  if (elicitation) capabilities.elicitation = {};
  const initialized = await peer.request("initialize", { protocolVersion: PROTOCOL_VERSION, capabilities, clientInfo: { name: "m7-live-agent-loop-probe", title: "M7 Live Agent Loop Probe", version: "1" } });
  if (initialized.error) throw new Error(`${label}: initialize failed: ${JSON.stringify(initialized.error)}`);
  peer.notify("notifications/initialized", {});
  return peer;
}

function structured(envelope: Record<string, any>): Record<string, any> {
  return (envelope.result?.structuredContent ?? {}) as Record<string, any>;
}

function describe(envelope: Record<string, any>): string {
  if (envelope.error) return `JSON-RPC error ${envelope.error.code}: ${envelope.error.message}`;
  return `isError=${envelope.result?.isError} :: ${JSON.stringify(envelope.result?.structuredContent ?? envelope.result?.content ?? {}).slice(0, 320)}`;
}

const sleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms));

/** ⏳️ Polls `probe` until it answers truthy or the budget runs out — the only wait shape this probe
 * uses, so nothing ever sleeps longer than the thing it is waiting for. */
async function until<T>(label: string, budgetMs: number, intervalMs: number, probe: () => Promise<T | null>): Promise<T | null> {
  const deadline = Date.now() + budgetMs;
  while (Date.now() < deadline) {
    const value = await probe();
    if (value) return value;
    await sleep(intervalMs);
  }
  console.log(`  (timed out waiting for ${label} after ${budgetMs}ms)`);
  return null;
}

/** 🖥️ Everything this probe reads out of the live shell, in one evaluate. */
function readShell(page: Page) {
  return page.evaluate(() => {
    const entries = [...document.querySelectorAll("[data-semio-agent-chat-entry]")].map((element) => ({
      kind: element.getAttribute("data-semio-agent-chat-entry"),
      state: element.getAttribute("data-agent-chat-state"),
      id: element.id,
      text: (element as HTMLElement).innerText.replace(/\s+/g, " ").trim().slice(0, 240),
    }));
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      shellError: document.documentElement.getAttribute("data-semio-os-error"),
      presenceTone: document.querySelector("[data-semio-agent-presence-tone]")?.getAttribute("data-semio-agent-presence-tone") ?? null,
      presenceText: (document.querySelector("[data-semio-agent-presence-tone]") as HTMLElement | null)?.innerText?.trim() ?? null,
      chatPanel: document.querySelector("[data-semio-agent-chat-panel]") !== null,
      entries,
      cancelButtons: [...document.querySelectorAll("[data-semio-agent-chat-cancel]")].map((element) => element.getAttribute("data-semio-agent-chat-cancel")),
      approvalGroups: [...document.querySelectorAll("[data-semio-agent-chat-approval]")].map((element) => element.getAttribute("data-semio-agent-chat-approval")),
      panelsVisible: [...document.querySelectorAll("[data-panel-anchor]")].map((element) => `${element.getAttribute("data-panel-anchor")}:${element.getAttribute("data-panel-visible")}`),
      activeWindowId: document.querySelector("[data-active-window-id]")?.getAttribute("data-active-window-id") ?? null,
      windowIds: [...new Set([...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")))],
    };
  });
}

// ───────────────────────────── the run ─────────────────────────────
const entry = semioEntry();
const consoleLines: string[] = [];
const t0 = Date.now();

const browser = await chromium.launch({ headless: process.env.SEMIO_M7_HEADLESS !== "0", args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (message) => consoleLines.push(`${Date.now() - t0} ${message.type()} ${message.text().slice(0, 1500)}`));
page.on("pageerror", (error) => consoleLines.push(`${Date.now() - t0} pageerror ${String(error).slice(0, 1500)}`));

let gateway: Peer | null = null;
try {
  // 0. The gateway must start AFTER the dev session published its record: discovery runs once, at
  //    gateway start (M4 §5.4).
  const offerBefore = await fetch(OFFER_URL).then((response) => response.status).catch(() => 0);
  gateway = await open("gateway — .mcp.json args verbatim", entry.args, entry.command, true);
  const offer = await until("the gateway's offer on the dev server", 60_000, 500, async () => {
    const response = await fetch(OFFER_URL).catch(() => null);
    if (!response || response.status !== 200) return null;
    return (await response.json()) as { url: string; admissionProof: string; pid: number };
  });
  record("0 rendezvous", offer !== null, offer ? `offer before gateway=${offerBefore}, after=200, url=${offer.url}, proof=${String(offer.admissionProof).slice(0, 8)}…, pid=${offer.pid}` : `the gateway published no offer (endpoint was ${offerBefore} before)`);

  // (a) the shell dials, and agent presence appears.
  await page.goto(URL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  // 🪧️ `data-semio-os-ready` carries the BOOTED PLUGIN'S ID (`note`), not `"true"` — reading it as a
  // boolean is how this probe's first run scored a perfectly healthy 10 s boot as a 240 s failure.
  const booted = await until("shell boot", 240_000, 1000, async () => {
    const view = await readShell(page);
    return view.ready || view.shellError ? view : null;
  });
  record("boot", Boolean(booted?.ready) && !booted?.shellError, booted ? `ready=${booted.ready} error=${booted.shellError} windows=${booted.windowIds.length} surfaces=${booted.windowIds.join(",")}` : "the shell never reached data-semio-os-ready");

  // The gateway's own witness that a shell attached: `ui_focus` stops answering the
  // "no shell is attached to /bridge yet" refusal the moment the socket registers.
  const attached = await until("the shell to register on /bridge", 90_000, 1000, async () => {
    const focused = await gateway!.call("ui_focus", {});
    return structured(focused).code === undefined ? focused : null;
  });
  record("(a) shell dials /bridge", attached !== null, attached ? `ui_focus answered ${describe(attached)}` : "ui_focus never stopped answering PLUGIN_UNAVAILABLE — no shell registered");

  // (d) ui_reveal opens the chat panel in the live shell — both the visible act AND the way this
  //     probe gets the conversation on screen.
  const revealed = await gateway.call("ui_reveal", { anchor: "right", path: ["framework.chat"] });
  const chatVisible = await until("the chat panel", 30_000, 500, async () => {
    const view = await readShell(page);
    return view.chatPanel ? view : null;
  });
  record("(d) ui_reveal acts on the live shell", chatVisible !== null, `${describe(revealed)}; chat panel in DOM=${chatVisible !== null}, presence=${chatVisible?.presenceTone ?? "(none)"}`);

  const presence = await until("agent presence", 20_000, 500, async () => {
    const view = await readShell(page);
    return view.presenceTone ? view : null;
  });
  record("(a) agent presence renders", presence !== null, presence ? `tone=${presence.presenceTone} text=${JSON.stringify(presence.presenceText)}` : "no [data-semio-agent-presence-tone] in the shell");

  const windowId = presence?.windowIds.find((id) => id) ?? null;
  const focusedOne = windowId ? await gateway.call("ui_focus", { windowId }) : null;
  record("(d) ui_focus acts on the live shell", focusedOne !== null && structured(focusedOne).ok === true, focusedOne ? `${describe(focusedOne)} (windowId=${windowId})` : "the shell reported no window id to focus");

  // (b) a tool call shows as a running row, then as a result.
  const running = gateway.call("capabilities_search", { query: "note block" });
  const sawRunning = await until("a running tool-call row", 20_000, 100, async () => {
    const view = await readShell(page);
    const row = view.entries.find((row) => row.kind === "toolCall" && row.state === "running");
    return row ?? null;
  });
  const settledCall = await running;
  const sawResult = await until("the settled tool-call row", 30_000, 200, async () => {
    const view = await readShell(page);
    const row = view.entries.find((row) => row.kind === "toolCall" && (row.state === "ok" || row.state === "failed"));
    return row ?? null;
  });
  record("(b) tool call → running row → result", sawRunning !== null && sawResult !== null, `running=${JSON.stringify(sawRunning)} settled=${JSON.stringify(sawResult)} call=${describe(settledCall)}`);

  // 🎯️ The mutation candidates, read off the LIVE catalog — needed by (c) as well as (e), because a
  //    gateway-owned read verb settles in milliseconds and never gives the cancel affordance a frame
  //    to exist in. A plugin `action_invoke` stays in flight for seconds, which is the honest shape of
  //    "an in-flight call" anyway.
  const found = await gateway.call("capabilities_search", { query: "delete selection clear", kind: ["mutation"] });
  const candidates = ((structured(found).results ?? []) as Record<string, any>[]).map((row) => String(row.capabilityId));
  const gateTarget = candidates.find((id) => id.startsWith(`${PLUGIN}.`)) ?? candidates[0] ?? null;

  // (c) U1's cancel button cancels an in-flight call.
  const longCall = gateTarget
    ? gateway.call("action_invoke", { capabilityId: gateTarget, input: {} }).catch((error) => ({ error: { message: String(error) } }) as Record<string, any>)
    : Promise.resolve({ error: { message: "no mutation capability to keep in flight" } } as Record<string, any>);
  const cancelTarget = await until("a cancellable row", 30_000, 100, async () => {
    const view = await readShell(page);
    return view.cancelButtons.find((id) => id) ?? null;
  });
  let cancelDetail = "no cancellable row ever appeared";
  let cancelOk = false;
  if (cancelTarget) {
    await page.locator(`[data-semio-agent-chat-cancel="${cancelTarget}"]`).first().click({ timeout: 10_000 }).catch((error) => consoleLines.push(`cancel click: ${String(error).slice(0, 200)}`));
    const cancelling = await until("the cancelling row", 20_000, 100, async () => {
      const view = await readShell(page);
      const row = view.entries.find((row) => row.id.endsWith(cancelTarget));
      return row && row.state !== "running" ? row : null;
    });
    const settled = await longCall;
    cancelOk = cancelling !== null;
    cancelDetail = `row=${JSON.stringify(cancelling)} call=${describe(settled as Record<string, any>)}`;
  } else {
    await longCall;
  }
  record("(c) cancel button cancels an in-flight call", cancelOk, cancelDetail);

  // (e) a destructive capability raises the approval affordance in the panel.
  console.log(`  destructive candidates: ${candidates.slice(0, 8).join(", ") || "(none)"}`);

  /** ⛩️ Invokes `capabilityId` without awaiting, decides the approval in the SHELL with `decision`,
   * and returns the gateway's own answer. `BUDGET_EXCEEDED` is the guest's own 8 ms interpreter
   * slice and is explicitly retryable, so a cold first open is retried rather than scored. */
  async function decideInShell(capabilityId: string, decision: "once" | "deny"): Promise<{ answer: Record<string, any>; sawAffordance: boolean }> {
    let sawAffordance = false;
    for (let attempt = 0; attempt < 200; attempt += 1) {
      const invoked = gateway!.call("action_invoke", { capabilityId, input: {} });
      const approvalId = await until("the approval affordance", 15_000, 150, async () => {
        const view = await readShell(page);
        return view.approvalGroups.find((id) => id) ?? null;
      });
      if (approvalId) {
        sawAffordance = true;
        await page.locator(`[id="framework.chat.approval.${decision}.${approvalId}"]`).first().click({ timeout: 10_000 }).catch((error) => consoleLines.push(`approval click: ${String(error).slice(0, 200)}`));
      }
      const answer = await invoked;
      if (structured(answer).code !== "BUDGET_EXCEEDED") return { answer, sawAffordance };
    }
    return { answer: { result: { isError: true, structuredContent: { code: "BUDGET_EXCEEDED" } } }, sawAffordance };
  }

  // 🎯️ Gate a capability of the plugin THIS session is running: every other plugin's `.wasm` in the
  // shared target dir was linked before A2's `-zstack-size` fix and traps in `InstanceOpen` long
  // before the approval gate, which would score A2's open defect as an M7 failure.
  if (gateTarget === null) {
    record("(e) approval affordance", false, "the live catalog exposed no destructive capability to gate");
  } else {
    console.log(`  gating ${gateTarget}`);
    const once = await decideInShell(gateTarget, "once");
    record("(e1) Approve Once lets it through", once.sawAffordance && structured(once.answer).code !== "APPROVAL_REQUIRED", `affordance=${once.sawAffordance} answer=${describe(once.answer)}`);
    const denied = await decideInShell(gateTarget, "deny");
    record("(e2) Deny returns the typed refusal", structured(denied.answer).code === "PERMISSION_DENIED", `affordance=${denied.sawAffordance} answer=${describe(denied.answer)}`);

    // ⏱️ Slice M7's new elicitation deadline, at runtime: a client that advertises `elicitation` and
    // then never answers must get the typed timeout — not an approval, and not a wedged agent.
    // 🏝️ Its own empty rendezvous, so the SHELL lane is closed by construction and the only budget
    // this step spends is the elicitation deadline itself.
    const isolated = join(import.meta.dir, "🗑️generated", "m7-rendezvous-empty");
    mkdirSync(join(isolated, "sessions"), { recursive: true });
    const silent = await open("(e3) elicitation timeout — a client that never answers", entry.args, entry.command, true, { S_AGENT_BRIDGE_DIR: isolated });
    silent.elicitationAnswer = "silent";
    const timedOut = await silent.call("action_invoke", { capabilityId: gateTarget, input: {} });
    const timeoutDetails = JSON.stringify(structured(timedOut).details ?? {});
    record("(e3) a silent client returns the typed timeout", structured(timedOut).code === "APPROVAL_REQUIRED" && timeoutDetails.includes("did not answer the elicitation in time"), `${describe(timedOut)} details=${timeoutDetails.slice(0, 300)}`);
    silent.stop();
  }

  console.log(`\n  gateway stderr tail: ${gateway.diagnostics.slice(-6).join(" | ").slice(0, 1200)}`);
  console.log(`  server-initiated requests seen: ${gateway.serverRequests.map((row) => row.method).join(", ") || "(none)"}`);
} finally {
  gateway?.stop();
  await page.screenshot({ path: join(import.meta.dir, "🗑️generated", "m7-live-shell.png"), fullPage: false }).catch(() => undefined);
  await browser.close();
}

console.log("\n=== console faults");
for (const line of consoleLines.filter((line) => /error|pageerror|refused|trap|unreachable/i.test(line) && !/favicon|\[vite\]|DevTools|WebGL/i.test(line)).slice(0, 25)) console.log(`  ${line}`);

console.log("\n=== transcript");
for (const outcome of outcomes) console.log(`  ${outcome.ok ? "PASS" : "FAIL"}  ${outcome.step}`);
const failed = outcomes.filter((outcome) => !outcome.ok).length;
console.log(`\n${outcomes.length - failed}/${outcomes.length} steps passed`);
process.exit(failed === 0 ? 0 : 1);
