#!/usr/bin/env bun
/** 🤝️ G10 live proof of outcome 4b + G-P2-2: a human holds a hub note open in the React `s` shell; an AI
 * agent, as its own delegated principal over the stdio semio MCP, opens the same document and commits an
 * edit; the human sees the agent in the roster as an agent (declared principal kind, localized) and sees
 * the edit land live, without a reload; the hub ledger advances.
 * usage: bun g10-agent-edit-human-sees.ts <shellOrigin> <hubOrigin> <en|de> [captureDir] */
import { chmodSync, mkdtempSync, writeFileSync } from "node:fs";
import { randomBytes } from "node:crypto";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { chromium, type Page } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { requireMcpBinary, spawnRawMcp } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const [SHELL = "http://127.0.0.1:6530", HUB = "http://127.0.0.1:7800", LOCALE = "en", OUT = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-g10-logs"] = process.argv.slice(2);
const AGENT_WORD = LOCALE === "de" ? "KI-Agent" : "AI agent";
const EMAIL = process.env.G10_HUMAN_EMAIL ?? "user2@semio.dev";
const PASSWORD = process.env.G10_HUMAN_PASSWORD ?? "gm1-local-dev-pass-2";
const rows: { step: string; ok: boolean; detail: string }[] = [];
const row = (step: string, ok: boolean, detail: string) => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);
};
const pause = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};

/** 🪞️ What the human's shell shows: the history ledger, the roster with each peer's declared kind, the sync pill. */
const readShell = (page: Page) =>
  page.evaluate(() => {
    const text = (el: Element | null) => ((el as HTMLElement | null)?.innerText ?? "").replace(/\s+/g, " ").trim();
    const peers = [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')];
    return {
      ledger: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => text(el).slice(0, 96)),
      peers: peers.map((el) => ({ id: el.getAttribute("data-row-id"), kind: el.getAttribute("data-presence-kind"), label: el.getAttribute("aria-label") ?? text(el) })),
      syncPill: text(document.querySelector('[id="s-sync-status"]')),
    };
  });

const signIn = await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `g10human${randomBytes(12).toString("hex")}`, clientClass: "browser" }));
const token = String(signIn.json?.token ?? "");
const spaceName = `G10 agent edit ${LOCALE} ${randomBytes(3).toString("hex")}`;
await hub("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))));
const spaceId = String((await hub("GET", "/directory/spaces", token)).json?.find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await hub("GET", creations, token);
const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
const OPEN = process.env.G10_OPEN ?? "home";
const requestId = randomBytes(16).toString("hex");
let creation = OPEN === "space" ? { phase: "shell" } : (await hub("POST", creations, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: `Agent edit ${LOCALE}` })))).json;
for (const deadline = Date.now() + 1_800_000; ["accepted", "preparing"].includes(creation?.phase) && Date.now() < deadline; ) {
  await pause(1_000);
  creation = (await hub("GET", `${creations}/${requestId}`, token)).json;
}
let documentId = String(creation?.ready?.artifactId ?? "");
row(`0 setup: ${EMAIL} owns a fresh space${OPEN === "space" ? "" : " holding a fresh note"} (hub authorities)`, spaceId.length > 0 && (OPEN === "space" || documentId.length > 0), `space=${spaceId} kind=${kind?.kindId} phase=${creation?.phase} document=${documentId}`);
const documentsOf = async () => [...new Set([...String((await hub("GET", "/directory/events?after=0", token)).text).matchAll(/"document\.indexed"[^}]*?"spaceId":"([^"]+)","documentId":"([^"]+)"/gu)].filter((match) => match[1] === spaceId).map((match) => match[2]))];
const headOf = async () => Number((await hub("GET", `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}`, token)).json?.head_seq ?? -1);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: LOCALE === "de" ? "de-DE" : "en-US" })).newPage();
const lines: string[] = [];
const sockets: { url: string; closed: boolean }[] = [];
page.on("console", (message) => lines.push(`${message.type()} ${message.text().slice(0, 400)}`));
page.on("response", (response) => {
  if (response.status() >= 400) lines.push(`http ${response.status()} ${response.request().method()} ${response.url().slice(0, 240)}`);
});
page.on("websocket", (ws) => {
  const entry = { url: ws.url(), closed: false };
  sockets.push(entry);
  ws.on("close", () => (entry.closed = true));
});
let agent: ReturnType<typeof spawnRawMcp> | undefined;
let delegationId = "";
try {
  const REMOTE = OPEN === "remote";
  await page.goto(REMOTE ? `${SHELL}/?plugin=note` : `${SHELL}/`, { waitUntil: "domcontentloaded", timeout: 180_000 });
  if (REMOTE) await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") !== null, undefined, { timeout: 300_000 });
  else await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(EMAIL);
  await form.locator('input[type="password"]').fill(PASSWORD);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  if (OPEN === "space") {
    const spaceButton = page.locator(`[data-semio-hub-workspace] li[data-space-id="${spaceId}"] button`).first();
    await spaceButton.waitFor({ state: "attached", timeout: 90_000 });
    await spaceButton.click({ force: true });
    await page.waitForFunction((id) => window.location.pathname.includes(`/spaces/${id}`) && [...document.querySelectorAll("[data-window-id]")].some((el) => el.getAttribute("data-window-id") !== "s-home-main"), spaceId, { timeout: 180_000 });
    const known = new Set(await documentsOf());
    for (let tick = 0; tick < 30 && !(await page.locator('[data-slot="window-action-pane"] [id="action.createArtifact"]').count()); tick += 1) {
      for (const id of await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].filter((toggle) => toggle.closest('[data-folded="true"]') !== null).map((toggle) => toggle.id))) await page.locator(`[id="${id}"]`).first().click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(1_000);
    }
    await page.locator('[data-slot="window-action-pane"] [id="action.createArtifact"]').first().click({ force: true });
    await page.waitForTimeout(1_500);
    await page.locator('[data-slot="window-action-pane"] [id$=".arg.name"]:is(input,textarea), [data-slot="window-action-pane"] [id$=".arg.name"] :is(input,textarea)').first().fill(`Agent edit ${LOCALE} (shell)`);
    await page.locator('[data-slot="window-action-pane"] [id$=".arg.kindChoice"]').first().click({ force: true });
    await page.waitForTimeout(1_000);
    const kindIndex = ((catalog.json?.kinds ?? []) as any[]).findIndex((entry) => entry?.kindId === kind?.kindId);
    await page.locator('[role="option"]').nth(kindIndex).click({ force: true });
    await page.locator('[id$=".action.createArtifact.execute"]').first().click({ force: true });
    for (const deadline = Date.now() + 600_000; !documentId && Date.now() < deadline; ) {
      documentId = (await documentsOf()).find((id) => !known.has(id)) ?? "";
      if (!documentId) await pause(1_000);
    }
    row("1s the human creates a note in the space from the shell's own create action, and it opens", documentId.length > 0, `document=${documentId || "<none>"} kind=${kind?.kindId} option=${kindIndex}`);
  } else {
    await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
    await form.waitFor({ state: "hidden", timeout: 15_000 });
  }
  const openRow = async (prefix: string, id: string) => {
    await page.locator(`[data-ui-node-key="${prefix}:${id}"]`).first().waitFor({ state: "attached", timeout: 180_000 });
    const buttons = page.locator(`[data-ui-node-key="${prefix}:${id}"] button`);
    for (let index = 0; index < (await buttons.count()); index += 1) {
      const name = `${(await buttons.nth(index).getAttribute("aria-label")) ?? ""} ${(await buttons.nth(index).textContent()) ?? ""}`;
      if (/open|öffnen/iu.test(name)) {
        await buttons.nth(index).focus();
        await buttons.nth(index).press("Enter");
        return;
      }
    }
    throw new Error(`row ${prefix}:${id} offers no open action`);
  };
  if (OPEN === "space") {
  } else if (REMOTE) {
    const clickOnce = async (selector: string) => ((await page.locator(selector).count()) ? page.locator(selector).first().click({ force: true, timeout: 8_000 }).then(() => true, () => false) : false);
    for (let attempt = 0; attempt < 6 && !(await page.locator('[id="framework.sync.remote.path"]').count()); attempt += 1) {
      if (!(await clickOnce('[id="framework.sync.remote"]')) && !(await clickOnce('[id="ui.utilities.group.sync"]'))) await clickOnce('[data-slot="panel-tab-button"][id="s-sync-status"], [id="s-sync-status"]');
      await page.waitForTimeout(1_500);
    }
    const input = page.locator('[id="framework.sync.remote.path"]');
    await input.fill(`${HUB.replace(/^https?:\/\//u, "")}/${spaceId}/${documentId}`);
    await page.waitForTimeout(400);
    await input.locator("xpath=ancestor::*[@data-slot='popover-content'][1]").locator('button:has([data-icon="link"])').first().click({ force: true, timeout: 8_000 });
    await page.waitForTimeout(3_000);
    await page.keyboard.press("Escape").catch(() => undefined);
  } else {
  await openRow("space", spaceId);
  await page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "visible", timeout: 180_000 });
  const apiRow = await page.locator(`[data-ui-node-key="artifact:${documentId}"]`).first().waitFor({ state: "attached", timeout: 60_000 }).then(() => true, () => false);
  row("1a the Space table lists the note created through the hub's own creation route", apiRow, apiRow ? documentId : `no artifact:${documentId} row within 60 s — the human creates one in the Space app instead`);
  if (!apiRow) {
    const before = new Set(await page.locator('[data-ui-node-key^="artifact:"]').evaluateAll((els) => els.map((el) => el.getAttribute("data-ui-node-key"))));
    const create = page.locator('[data-ui-node-key="s-space-create-artifact"]').first();
    await create.focus();
    await create.press("Enter");
    const dialog = page.locator('[role="dialog"][data-slot="dialog-content"]');
    await dialog.waitFor({ state: "visible", timeout: 15_000 });
    await page.locator('[id="name"]').fill(`Agent edit ${LOCALE} (shell)`);
    await page.locator('[id="kindChoice"]').click();
    await page.locator(`[role="option"][data-value*='"kindId":"${String(kind?.kindId ?? "")}"']`).first().click();
    await page.locator('[id="ui.dialog.submit"]').click();
    await dialog.waitFor({ state: "hidden", timeout: 20_000 });
    let created = "";
    for (const deadline = Date.now() + 600_000; !created && Date.now() < deadline; ) {
      for (const key of await page.locator('[data-ui-node-key^="artifact:"]').evaluateAll((els) => els.map((el) => el.getAttribute("data-ui-node-key")))) if (key && !before.has(key)) created = key.slice("artifact:".length);
      if (!created) await pause(500);
    }
    row("1b the human creates a note in the Space app", created.length > 0, `document=${created || "<none>"}`);
    documentId = created;
  }
  await openRow("artifact", documentId);
  }
  let live = await readShell(page);
  for (let tick = 0; tick < 90 && !(live.syncPill && !/detached|connecting|backoff|getrennt|verbinde|erneut/iu.test(live.syncPill) && sockets.some((ws) => ws.url.includes("/document/ws") && !ws.closed)); tick += 1) {
    await pause(2_000);
    live = await readShell(page);
  }
  row("1 the human signs in through the shell and holds the hub note open live", sockets.some((ws) => ws.url.includes("/document/ws") && !ws.closed), `sync="${live.syncPill}" ledger=${live.ledger.length} peers=${JSON.stringify(live.peers)}`);
  const before = await readShell(page);
  const headBefore = await headOf();

  const label = `Claude G10 ${LOCALE}`;
  const delegation = await hub("POST", "/auth/agent-delegations", token, JSON.stringify({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId, agentLabel: label, audience: "edit", ttlSecs: 1800 }));
  delegationId = String(delegation.json?.delegationId ?? "");
  const credentialPath = join(mkdtempSync(join(tmpdir(), "g10-agent-edit-")), "agent-credential.json");
  writeFileSync(credentialPath, `${JSON.stringify({ schema: "semio.hub.agent-credential/v1", hubOrigin: HUB, spaceId, audience: "edit", token: delegation.json?.token })}\n`, { mode: 0o600 });
  chmodSync(credentialPath, 0o600);
  process.env.S_AGENT_BRIDGE_DIR = mkdtempSync(join(tmpdir(), "g10-agent-edit-bridge-"));
  agent = spawnRawMcp(requireMcpBinary("/Users/ueli/Documents/semio"), ["stdio", "--hub", HUB, "--space", spaceId, "--credential-file", credentialPath, "--scopes", "workspace.read,artifact.write", "--no-bridge"]);
  await agent.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g10-agent-edit", version: "1" } }, 600_000);
  agent.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }));
  const call = async (name: string, args: Record<string, unknown>) => ((await agent!.request("tools/call", { name, arguments: args }, 600_000)).result ?? {}) as { isError?: boolean; structuredContent?: any };
  const context = await call("context_resolve", {});
  const opened = await call("artifact_open", { artifactId: documentId });
  row("2 the agent is its own delegated principal and opens the same document", opened.isError !== true && String(context.structuredContent?.principal ?? "") === `agent:${delegationId}`, `principal=${context.structuredContent?.principal} writePath=${opened.structuredContent?.sessionDocument?.writePath ?? "?"} ${opened.isError ? JSON.stringify(opened.structuredContent).slice(0, 240) : ""}`);

  let roster = await readShell(page);
  for (let tick = 0; tick < 30 && !roster.peers.some((peer) => peer.kind === "agent"); tick += 1) {
    await pause(1_000);
    roster = await readShell(page);
  }
  const agentPeer = roster.peers.find((peer) => peer.kind === "agent");
  row(`3 the human's roster shows the agent AS an agent (${LOCALE})`, Boolean(agentPeer) && String(agentPeer?.label ?? "").includes(AGENT_WORD) && String(agentPeer?.label ?? "").includes(label), `peers=${JSON.stringify(roster.peers)}`);

  const search = await call("capabilities_search", { query: "add a block", kind: ["mutation"] });
  const capabilityId = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId ?? hit.id)).find((id) => id.endsWith(".addBlock")) ?? "");
  const prepared = await call("action_prepare", { capabilityId, input: { kind: "text" } });
  const invoked = await call("action_invoke", { preparedActionHandle: prepared.structuredContent?.preparedHandle });
  row("4 the agent commits an edit through the semio MCP", invoked.structuredContent?.status === "SUCCEEDED", `${capabilityId} status=${invoked.structuredContent?.status ?? JSON.stringify(invoked.structuredContent ?? prepared.structuredContent).slice(0, 240)}`);

  let after = await readShell(page);
  const t0 = Date.now();
  for (let tick = 0; tick < 60 && after.ledger.length <= before.ledger.length; tick += 1) {
    await pause(1_000);
    after = await readShell(page);
  }
  row("5 the human sees the agent's edit land live, without a reload", after.ledger.length > before.ledger.length, `ledger ${before.ledger.length}→${after.ledger.length} after ${Date.now() - t0} ms; newest=${JSON.stringify(after.ledger.slice(-2))}`);
  let headAfter = await headOf();
  for (let tick = 0; tick < 30 && headAfter <= headBefore; tick += 1) {
    await pause(1_000);
    headAfter = await headOf();
  }
  row("6 the hub ledger advanced with the agent's commit", headAfter > headBefore, `head_seq ${headBefore}→${headAfter}`);
  await page.screenshot({ path: join(OUT, `g10-agent-edit-human-sees-${LOCALE}.png`) });
} catch (error) {
  row("run", false, error instanceof Error ? error.stack ?? error.message : String(error));
  await page.screenshot({ path: join(OUT, `g10-agent-edit-human-sees-${LOCALE}-error.png`) }).catch(() => undefined);
} finally {
  await agent?.close().catch(() => undefined);
  if (delegationId) await hub("DELETE", `/auth/agent-delegations/${encodeURIComponent(delegationId)}`, token).catch(() => undefined);
  writeFileSync(join(OUT, `g10-agent-edit-human-sees-${LOCALE}-console.txt`), lines.slice(-400).join("\n"));
  await browser.close();
}
const red = rows.filter((entry) => !entry.ok).length;
console.log(`g10-agent-edit-human-sees (${LOCALE}): ${rows.length - red}/${rows.length} rows green`);
process.exit(red === 0 ? 0 : 1);
