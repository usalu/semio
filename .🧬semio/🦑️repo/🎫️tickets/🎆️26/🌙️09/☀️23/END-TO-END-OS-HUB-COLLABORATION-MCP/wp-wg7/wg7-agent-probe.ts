#!/usr/bin/env bun
/** 🤖️ WG7 (S12-3) — a delegated AI agent, as its own principal over the stdio semio MCP (`mcp__semio__*` surface), edits a hub note
 * while a human holds it open in the wasm32 BROWSER wgpu shell: the human's roster shows the agent AS an agent (en/de badge) and the
 * agent's block lands in the human's Artifact panel without a reload; the hub ledger advances. The wgpu twin of G10's React 4b.
 * Usage: bun wg7-agent-probe.ts <shellUrl> <hubOrigin> <spaceId> <documentId> <en|de> [tag] */
import { chmodSync, mkdirSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { join } from "node:path";
import { chromium, type Page } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { requireMcpBinary, spawnRawMcp } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";

const [SHELL = "http://127.0.0.1:6552/?plugin=note", HUB = "http://127.0.0.1:8050", SPACE = "", DOCUMENT = "", LOCALE = "en", TAG = "a1"] = process.argv.slice(2);
const OUT = join(import.meta.dir, "generated");
const DURABLE = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s12-wg7-agent";
mkdirSync(OUT, { recursive: true });
mkdirSync(DURABLE, { recursive: true, mode: 0o700 });
const HUMANS: Record<string, string> = { "user1@semio.dev": "gm1-local-dev-pass-1", "user2@semio.dev": "gm1-local-dev-pass-2" };
const EMAIL = process.env.WG7_HUMAN ?? "user1@semio.dev";
const PASSWORD = HUMANS[EMAIL] ?? "";
const AGENT_WORD = LOCALE === "de" ? "KI-Agent" : "AI agent";
const MIRROR = "#semio-wgpu-accessibility";
const rows: { step: string; ok: boolean; detail: string }[] = [];
const row = (step: string, ok: boolean, detail: string) => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail.slice(0, 500)}`);
};
const pause = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any;
  try { json = JSON.parse(text); } catch {}
  return { status: response.status, text, json };
};

type Node = { key: string; label?: string; role?: string; disabled?: boolean; valueText?: string; checked?: boolean };
const projection = async (page: Page): Promise<Node[]> => {
  const raw = await page.evaluate(async () => (await (globalThis as any).semioWgpuIntrospection?.dumpAccessibility?.()) ?? "");
  try { return (JSON.parse(raw).windows ?? []).flatMap((surface: any) => surface.nodes ?? []); } catch { return []; }
};
const keyEnding = (nodes: Node[], suffix: string) => nodes.find((node) => String(node.key).endsWith(suffix))?.key;
const awaitKey = async (page: Page, suffix: string, budgetMs = 40_000) => {
  for (const deadline = Date.now() + budgetMs; Date.now() < deadline; await pause(800)) {
    const key = keyEnding(await projection(page), suffix);
    if (key) return key;
  }
  return undefined;
};
const activate = async (page: Page, key: string | undefined, settleMs = 2500) => {
  if (!key) return "absent";
  const outcome = await page.evaluate(({ selector, nodeKey }) => {
    const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`) as HTMLElement | null;
    if (!element) return "absent";
    if ((element as HTMLButtonElement).disabled === true) return "disabled";
    element.focus();
    element.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    return "activated";
  }, { selector: MIRROR, nodeKey: key });
  await page.waitForTimeout(settleMs);
  return outcome;
};
const typeInto = async (page: Page, key: string | undefined, value: string, verify = false) => {
  if (!key) return false;
  for (const deadline = Date.now() + 15_000; Date.now() < deadline; ) {
    const applied = await page.evaluate(({ selector, nodeKey, text }) => {
      const element = document.querySelector(`${selector} [data-node-key="${nodeKey}"]`);
      if (!(element instanceof HTMLInputElement) && !(element instanceof HTMLTextAreaElement)) return false;
      element.focus();
      element.value = text;
      element.dispatchEvent(new Event("input", { bubbles: true }));
      element.dispatchEvent(new Event("change", { bubbles: true }));
      return true;
    }, { selector: MIRROR, nodeKey: key, text: value });
    await page.waitForTimeout(applied ? 1500 : 500);
    if (applied && (!verify || (await projection(page)).some((node) => node.key === key && node.valueText === value))) return true;
  }
  return false;
};
const blocks = async (page: Page) => (await projection(page)).filter((node) => String(node.key).startsWith("note-play-block:")).map((node) => node.label ?? "");
const footer = async (page: Page) => {
  const nodes = await projection(page);
  return { presence: nodes.find((node) => node.key === "s-presence-peers")?.label ?? "", sync: nodes.find((node) => node.key === "s-sync-status")?.label ?? "" };
};

const signIn = await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `wg7agent${randomBytes(12).toString("hex")}`, clientClass: "browser" }));
const token = String(signIn.json?.token ?? "");
const headOf = async () => Number((await hub("GET", `/spaces/${encodeURIComponent(SPACE)}/documents/${encodeURIComponent(DOCUMENT)}`, token)).json?.head_seq ?? -1);
row("0 the human holds a hub session for the space", token.length > 0, `status=${signIn.status}`);

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--enable-features=Vulkan,WebGPU", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: LOCALE === "de" ? "de-DE" : "en-US" })).newPage();
const lines: string[] = [];
page.on("console", (message) => lines.push(`${message.type()} ${message.text().slice(0, 400)}`));
let agent: ReturnType<typeof spawnRawMcp> | undefined;
let delegationId = "";
try {
  await page.goto(SHELL, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.waitForFunction(() => typeof (globalThis as any).semioWgpuIntrospection?.dumpStructure === "function", null, { timeout: 180_000 });
  await page.waitForTimeout(8000);
  await activate(page, "framework.hub.signIn", 2000);
  await typeInto(page, await awaitKey(page, "framework.hub.address"), HUB);
  for (let attempt = 0; attempt < 4 && (await activate(page, await awaitKey(page, "framework.hub.sign-in.add", 5000), 4000)) !== "activated"; attempt += 1) {}
  for (let attempt = 0; attempt < 4; attempt += 1) {
    const connection = (await projection(page)).find((node) => String(node.key).includes("sign-in.connection") && String(node.key).includes("remote:"));
    await activate(page, connection?.key, 2000);
    await typeInto(page, await awaitKey(page, "framework.hub.email", 5000), EMAIL);
    await typeInto(page, await awaitKey(page, "framework.hub.password", 5000), PASSWORD);
    if ((await activate(page, await awaitKey(page, "framework.hub.sign-in.submit", 5000), 12_000)) === "activated" && !(await awaitKey(page, "framework.hub.sign-in.submit", 3000))) break;
  }
  await activate(page, keyEnding(await projection(page), "framework.hub.close"), 1500);
  await activate(page, "s-sync-status", 2000);
  await activate(page, await awaitKey(page, "framework.sync.remote", 10_000), 2000);
  const path = await awaitKey(page, "framework.sync.remote.path", 10_000);
  await typeInto(page, path, `${HUB.replace(/^https?:\/\//u, "")}/${SPACE}/${DOCUMENT}`, true);
  for (let tick = 0; tick < 30 && (await projection(page)).find((node) => String(node.key).endsWith("framework.sync.attach"))?.disabled !== false; tick += 1) await pause(500);
  await activate(page, keyEnding(await projection(page), "framework.sync.attach"), 4000);
  let live = await footer(page);
  for (let tick = 0; tick < 60 && !/live|connected|persisted|verbunden|gespeichert/iu.test(live.sync); tick += 1) {
    await pause(2000);
    live = await footer(page);
  }
  row(`1 the human (wasm32 wgpu, ${LOCALE}) signed in and holds the hub note live`, /live|connected|persisted|verbunden|gespeichert/iu.test(live.sync), JSON.stringify(live));
  for (let attempt = 0; attempt < 3 && !keyEnding(await projection(page), "note-play-blocks.add.text"); attempt += 1) {
    if ((await projection(page)).find((node) => node.key === "framework.panel.artifact")?.checked !== true) await activate(page, "framework.panel.artifact", 3000);
    await awaitKey(page, "note-play-blocks.add.text", 10_000);
  }
  row("1b the human's Artifact panel lists the note's blocks", Boolean(keyEnding(await projection(page), "note-play-blocks.add.text")), JSON.stringify(await blocks(page)));
  const before = await blocks(page);
  const headBefore = await headOf();

  const label = `WG7 agent ${LOCALE}`;
  const delegation = await hub("POST", "/auth/agent-delegations", token, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId: SPACE, agentLabel: label, audience: "edit", ttlSecs: 1800 }));
  delegationId = String(delegation.json?.delegationId ?? "");
  const credentialPath = join(DURABLE, `agent-credential-${TAG}.json`);
  writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId: SPACE, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
  chmodSync(credentialPath, 0o600);
  process.env.S_AGENT_BRIDGE_DIR = join(DURABLE, "bridge");
  mkdirSync(process.env.S_AGENT_BRIDGE_DIR, { recursive: true, mode: 0o700 });
  agent = spawnRawMcp(requireMcpBinary("/Users/ueli/Documents/semio"), ["stdio", "--hub", HUB, "--space", SPACE, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"]);
  await agent.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "wg7-agent-probe", version: "1" } }, 600_000);
  agent.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }));
  const call = async (name: string, args: Record<string, unknown>) => ((await agent!.request("tools/call", { name, arguments: args }, 600_000)).result ?? {}) as { isError?: boolean; structuredContent?: any };
  const context = await call("context_resolve", {});
  const opened = await call("artifact_open", { artifactId: DOCUMENT });
  row("2 the agent is its own delegated principal and opens the same document", opened.isError !== true && String(context.structuredContent?.principal ?? "") === `agent:${delegationId}`, `principal=${context.structuredContent?.principal} ${opened.isError ? JSON.stringify(opened.structuredContent).slice(0, 240) : ""}`);
  let roster = await footer(page);
  for (let tick = 0; tick < 30 && !roster.presence.includes(AGENT_WORD); tick += 1) {
    await pause(1000);
    roster = await footer(page);
  }
  row(`3 the human's wgpu roster shows the agent AS an agent (${LOCALE})`, roster.presence.includes(AGENT_WORD) && roster.presence.includes(label), JSON.stringify(roster));
  const search = await call("capabilities_search", { query: "add a block", kind: ["mutation"] });
  const capabilityId = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId ?? hit.id)).find((id) => id.endsWith(".addBlock")) ?? "");
  const prepared = await call("action_prepare", { capabilityId, input: { kind: "text" } });
  const invoked = await call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
  row("4 the agent commits an edit through the semio MCP", invoked.structuredContent?.status === "SUCCEEDED", `${capabilityId} status=${invoked.structuredContent?.status ?? JSON.stringify(invoked.structuredContent).slice(0, 240)} prepared=${JSON.stringify({ isError: prepared.isError, content: prepared.structuredContent }).slice(0, 600)}`);
  let after = await blocks(page);
  const t0 = Date.now();
  for (let tick = 0; tick < 60 && after.length <= before.length; tick += 1) {
    await pause(1000);
    after = await blocks(page);
  }
  row("5 the human sees the agent's block land live in the wgpu shell, without a reload", after.length > before.length, `blocks ${before.length}→${after.length} after ${Date.now() - t0} ms`);
  let headAfter = await headOf();
  for (let tick = 0; tick < 30 && headAfter <= headBefore; tick += 1) {
    await pause(1000);
    headAfter = await headOf();
  }
  row("6 the hub ledger advanced with the agent's commit", headAfter > headBefore, `head_seq ${headBefore}→${headAfter}`);
  await page.screenshot({ path: join(OUT, `agent-${TAG}-${LOCALE}.png`) });
} catch (error) {
  row("run", false, error instanceof Error ? error.stack ?? error.message : String(error));
  await page.screenshot({ path: join(OUT, `agent-${TAG}-${LOCALE}-error.png`) }).catch(() => undefined);
} finally {
  await agent?.close().catch(() => undefined);
  if (delegationId) await hub("DELETE", `/auth/agent-delegations/${encodeURIComponent(delegationId)}`, token).catch(() => undefined);
  writeFileSync(join(OUT, `agent-${TAG}-${LOCALE}-console.txt`), lines.slice(-400).join("\n"));
  await browser.close();
}
const red = rows.filter((entry) => !entry.ok).length;
console.log(`RESULT ${rows.length - red}/${rows.length}`);
process.exit(red === 0 ? 0 : 1);
