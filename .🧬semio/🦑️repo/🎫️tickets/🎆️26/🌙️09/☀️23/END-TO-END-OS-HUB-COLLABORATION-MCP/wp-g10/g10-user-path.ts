#!/usr/bin/env bun
/** 🧭️ G10 S4: the user path for an AI client, zero-touch and timed, from a CLEAN browser profile. A human signs in to the
 * `s` shell, opens their space, creates a note there, turns an agent delegation into an MCP client configuration with the
 * in-product "Set up MCP client" action, and a REAL MCP client — the official `@modelcontextprotocol/sdk` `Client` over
 * `StdioClientTransport`, started from exactly the shown `command`/`args` — connects, lists the tools, edits the note, and
 * asks for a destructive change the human approves in the shell. The human then withdraws the delegation in the same pane
 * and a client holding the withdrawn credential is refused.
 * usage: bun g10-user-path.ts <shellOrigin> <hubOrigin> <en|de> <captureDir> <email> <password>
 * `S_AGENT_BRIDGE_DIR` must be the rendezvous the serve publishes into (a user's default one needs no variable at all). */
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { Client } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/index.js";
import { StdioClientTransport } from "/Users/ueli/Documents/semio/node_modules/@modelcontextprotocol/sdk/dist/esm/client/stdio.js";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";
import { sealSpaceArtifactCreateV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts";

const [SHELL, HUB, LOCALE = "en", OUT, EMAIL, PASSWORD] = process.argv.slice(2);
if (!SHELL || !HUB || !OUT || !EMAIL || !PASSWORD) throw new Error("usage: bun g10-user-path.ts <shellOrigin> <hubOrigin> <en|de> <captureDir> <email> <password>");
const WORDS = {
  en: { install: "Set up MCP client", title: "Connect an AI client", once: "Approve Once" },
  de: { install: "MCP-Client einrichten", title: "KI-Client verbinden", once: "Einmal genehmigen" },
}[LOCALE === "de" ? "de" : "en"];
mkdirSync(OUT, { recursive: true });
const started = Date.now();
const seconds = () => ((Date.now() - started) / 1000).toFixed(1);
const rows: { step: string; ok: boolean; detail: string; at: string }[] = [];
const row = (step: string, ok: boolean, detail: string): void => {
  rows.push({ step, ok, detail, at: seconds() });
  console.log(`${ok ? "PASS" : "FAIL"}  [${seconds()} s] ${step} — ${detail}`);
};
const hub = async (method: string, path: string, token?: string, body?: string) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}) }, ...(body === undefined ? {} : { body }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};
const pause = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
/** 🧹️ Dismisses every first-run introduction tour the shell shows over the page, the way a user skips it. */
const skipTours = async (page: import("playwright").Page): Promise<void> => {
  for (let attempt = 0; attempt < 20; attempt += 1) {
    const skip = page.getByRole("button", { name: /^\s*(skip|überspringen)\s*$/iu }).first();
    if ((await skip.count()) === 0) return;
    await skip.click({ force: true }).catch(() => undefined);
    await pause(400);
  }
};

const setup = await hub("POST", "/auth/sessions", undefined, JSON.stringify({ schema: "semio.hub.auth.credential-sign-in/v1", email: EMAIL, password: PASSWORD, deviceInstanceId: `g10userpath${randomBytes(10).toString("hex")}`, clientClass: "browser" }));
const token = String(setup.json?.token ?? "");
const spaceName = `G10 user path ${LOCALE} ${randomBytes(3).toString("hex")}`;
await hub("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))));
const spaceId = String((await hub("GET", "/directory/spaces", token)).json?.find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
const creations = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
const catalog = await hub("GET", creations, token);
const kind = (catalog.json?.kinds ?? []).find((entry: any) => String(entry?.schema ?? "").startsWith("note"));
const requestId = randomBytes(16).toString("hex");
let creation = (await hub("POST", creations, token, JSON.stringify(sealSpaceArtifactCreateV1({ requestId, expectedCatalogGenerationId: String(catalog.json?.catalogGenerationId ?? ""), kindId: String(kind?.kindId ?? ""), name: `User path ${LOCALE}` })))).json;
for (const deadline = Date.now() + 1_800_000; ["accepted", "preparing"].includes(creation?.phase) && Date.now() < deadline; ) {
  await pause(1_000);
  creation = (await hub("GET", `${creations}/${requestId}`, token)).json;
}
const documentId = String(creation?.ready?.artifactId ?? "");
row("0 setup: the user owns a fresh space with one note (hub authorities)", spaceId.length > 0 && documentId.length > 0, `space=${spaceId} note=${documentId} kind=${kind?.kindId}`);
const pathStarted = Date.now();

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: LOCALE === "de" ? "de-DE" : "en-US" });
const page = await context.newPage();
const lines: string[] = [];
page.on("console", (message) => lines.push(`${message.type()} ${message.text().slice(0, 400)}`));
let client: Client | undefined;
try {
  await page.goto(`${SHELL}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") === "s", undefined, { timeout: 300_000 });
  await page.locator("[data-semio-hub-sign-in]").first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 30_000 });
  await workspace.locator('input[type="email"]').fill(EMAIL);
  await workspace.locator('input[type="password"]').fill(PASSWORD);
  await workspace.locator('form:has(input[type="password"]) button[type="submit"]').first().click();
  await page.waitForFunction((id) => document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space") === id, spaceId, { timeout: 90_000 });
  row("1 a clean profile signs in through the shell and lands in the space", true, `space=${spaceId}`);

  const label = `Client ${LOCALE} ${randomBytes(2).toString("hex")}`;
  await page.locator("input[data-element-alias='os.hub.agent.name']").fill(label);
  await page.locator('[id="os.hub.agent.create"]').click();
  await page.locator('[data-semio-hub-agent-mcp="idle"]').waitFor({ state: "visible", timeout: 60_000 });
  const installName = await page.locator('[id="os.hub.agent.mcpInstall"]').getAttribute("aria-label");
  await page.locator('[id="os.hub.agent.mcpInstall"]').click();
  const paneTrace: string[] = [];
  const tracePane = async (): Promise<void> => {
    const state = await page.evaluate(() => ({
      mcp: document.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp") ?? null,
      space: document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space") ?? null,
      alerts: [...document.querySelectorAll("[data-semio-hub-workspace] [role=alert], [data-semio-hub-workspace] [role=status]")].map((node) => node.textContent?.trim()).filter(Boolean).slice(0, 4),
    }));
    paneTrace.push(`${seconds()} s ${JSON.stringify(state)}`);
  };
  for (let tick = 0; tick < 30 && !(await page.locator('[data-semio-hub-agent-mcp="ready"]').isVisible()); tick += 1) {
    await tracePane();
    await page.waitForTimeout(1000);
  }
  if (!(await page.locator('[data-semio-hub-agent-mcp="ready"]').isVisible())) console.log(`INFO [${seconds()} s] pane after "${WORDS.install}": ${[...new Set(paneTrace.map((line) => line.replace(/^[0-9.]+ s /, "")))].join(" → ")}`);
  await page.locator('[data-semio-hub-agent-mcp="ready"]').waitFor({ state: "visible", timeout: 5_000 });
  const config = JSON.parse(await page.locator("[data-semio-hub-agent-mcp-config]").innerText()) as { mcpServers: Record<string, { command: string; args: string[] }> };
  const [serverName, entry] = Object.entries(config.mcpServers)[0]!;
  const credentialPath = entry.args[entry.args.indexOf("--credential-file") + 1] ?? "";
  row(`2 "${WORDS.install}" hands out one stdio entry bound to the hub space (${LOCALE})`, installName === WORDS.install && existsSync(credentialPath) && entry.args.includes(spaceId), `server=${serverName} command=${entry.command} credential=${credentialPath}`);
  await skipTours(page);

  const connectStarted = Date.now();
  client = new Client({ name: "g10-user-path", version: "1" }, { capabilities: {} });
  await client.connect(new StdioClientTransport({ command: entry.command, args: entry.args, env: { ...process.env } as Record<string, string>, stderr: "pipe" }), { timeout: 900_000 });
  const tools = await client.listTools(undefined, { timeout: 120_000 });
  const contextResolved: any = await client.callTool({ name: "context_resolve", arguments: {} }, undefined, { timeout: 120_000 });
  const principal = String(contextResolved.structuredContent?.principal ?? "");
  row("3 the official MCP SDK client connects from that entry and lists the tools", tools.tools.length === 28 && principal.startsWith("agent:"), `tools=${tools.tools.length} principal=${principal} channel=${contextResolved.structuredContent?.channel} connect=${((Date.now() - connectStarted) / 1000).toFixed(1)} s`);

  const opened: any = await client.callTool({ name: "artifact_open", arguments: { artifactId: documentId } }, undefined, { timeout: 600_000 });
  const search: any = await client.callTool({ name: "capabilities_search", arguments: { query: "add a block", kind: ["mutation"] } }, undefined, { timeout: 120_000 });
  const addBlock = String(((search.structuredContent?.results ?? []) as any[]).map((hit) => String(hit.capabilityId)).find((id) => id.endsWith(".addBlock")) ?? "");
  const edited: any = await client.callTool({ name: "action_invoke", arguments: { capabilityId: addBlock, input: { kind: "text" } } }, undefined, { timeout: 600_000 });
  row("4 the client edits the hub note", opened.isError !== true && edited.structuredContent?.status === "SUCCEEDED", `open=${opened.isError ? JSON.stringify(opened.structuredContent).slice(0, 200) : "ok"} ${addBlock} status=${edited.structuredContent?.status ?? JSON.stringify(edited.structuredContent).slice(0, 200)}`);

  const destructive = addBlock.replace(/\.addBlock$/u, ".deleteSelection");
  const before = new Set(await page.locator("[data-semio-agent-approval-id]").evaluateAll((els) => els.map((el) => el.getAttribute("data-semio-agent-approval-id"))));
  const pending = client.callTool({ name: "action_invoke", arguments: { capabilityId: destructive, input: {} } }, undefined, { timeout: 600_000 });
  let approvalId = "";
  for (let tick = 0; tick < 120 && !approvalId; tick += 1) {
    approvalId = (await page.locator("[data-semio-agent-approval-id]").evaluateAll((els) => els.map((el) => el.getAttribute("data-semio-agent-approval-id") ?? ""))).find((id) => id && !before.has(id)) ?? "";
    if (!approvalId) await pause(500);
  }
  const onceWord = approvalId ? ((await page.locator(`[id="framework.approvals.once.${approvalId}"]`).first().innerText().catch(() => "")) ?? "").trim() : "";
  await page.screenshot({ path: join(OUT, `g10-user-path-approval-${LOCALE}.png`) });
  if (approvalId) {
    await skipTours(page);
    const once = page.locator(`[id="framework.approvals.once.${approvalId}"]`).first();
    await once.waitFor({ state: "visible", timeout: 30_000 });
    await once.click({ timeout: 10_000 });
  }
  const decided: any = await pending;
  row(`5 the destructive request waits for the human, who approves it in the shell (${LOCALE})`, approvalId.length > 0 && onceWord === WORDS.once && decided.isError !== true, `approval=${approvalId || "<none>"} button="${onceWord}" answer=${JSON.stringify(decided.structuredContent ?? {}).slice(0, 200)}`);
  row("6 time from the clean profile's first page load to an approved agent edit", true, `${((Date.now() - pathStarted) / 1000).toFixed(1)} s`);

  const beforeRevoke = join(mkdtempSync(join(tmpdir(), "g10-user-path-cred-")), "credential.json");
  copyFileSync(credentialPath, beforeRevoke);
  chmodSync(beforeRevoke, 0o600);
  await skipTours(page);
  const paneSpaceNow = await page.locator("[data-semio-hub-agent-space]").first().getAttribute("data-semio-hub-agent-space").catch(() => null);
  if (paneSpaceNow !== spaceId) {
    console.log(`INFO [${seconds()} s] the agent pane is bound to ${paneSpaceNow ?? "no space"} at revoke time (path ${await page.evaluate(() => location.pathname)}); the user opens the space from the hub list to manage its agents`);
    await page.locator(`[data-semio-hub-workspace] li[data-space-id="${spaceId}"] button`).first().click({ force: true, timeout: 30_000 });
    await page.waitForFunction((id) => window.location.pathname.includes(`/spaces/${id}`), spaceId, { timeout: 120_000 });
    await skipTours(page);
    await page.locator('[data-slot="navbar"]').first().focus().catch(() => undefined);
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    const palette = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await palette.waitFor({ state: "visible", timeout: 15_000 });
    await palette.fill("hub");
    await page.locator('[data-slot="command-item"]').filter({ hasText: /open hub|hub.*öffnen/iu }).first().click({ timeout: 10_000 });
    await page.waitForFunction((id) => document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space") === id, spaceId, { timeout: 60_000 }).catch(() => undefined);
    console.log(`INFO [${seconds()} s] reopened the hub workspace from the palette: pane=${await page.locator("[data-semio-hub-agent-space]").first().getAttribute("data-semio-hub-agent-space").catch(() => null)} path=${await page.evaluate(() => location.pathname)}`);
    await skipTours(page);
  }
  await page.locator("li[data-delegation-id]", { hasText: label }).locator("button").first().click({ timeout: 60_000 });
  await page.locator("[data-semio-hub-agent-revoke-confirm]").waitFor({ state: "visible", timeout: 10_000 });
  await page.locator("[data-semio-hub-agent-revoke-confirm] button").first().click();
  for (let tick = 0; tick < 60 && existsSync(credentialPath); tick += 1) await pause(250);
  const staleClient = new Client({ name: "g10-user-path-revoked", version: "1" }, { capabilities: {} });
  const revokedArgs = entry.args.map((arg) => (arg === credentialPath ? beforeRevoke : arg));
  const refused = await staleClient.connect(new StdioClientTransport({ command: entry.command, args: revokedArgs, env: { ...process.env } as Record<string, string>, stderr: "pipe" }), { timeout: 120_000 }).then(
    () => "connected",
    (error: unknown) => `refused: ${String(error).slice(0, 200)}`,
  );
  await staleClient.close().catch(() => undefined);
  row("7 withdrawing in the pane removes the credential and the hub refuses a client holding a copy", !existsSync(credentialPath) && refused.startsWith("refused"), `installed file present=${existsSync(credentialPath)} copy=${refused}`);
  const afterRevoke: any = await client.callTool({ name: "action_invoke", arguments: { capabilityId: addBlock, input: { kind: "text" } } }, undefined, { timeout: 120_000 }).catch((error: unknown) => ({ isError: true, structuredContent: { message: String(error) } }));
  console.log(`INFO  [${seconds()} s] 7b the client that was already connected, after the withdrawal (audit G12-P2-1, H9/U5): ${afterRevoke.isError ? `refused ${JSON.stringify(afterRevoke.structuredContent).slice(0, 200)}` : `still edits: status=${afterRevoke.structuredContent?.status}`}`);
} catch (error) {
  row("run", false, error instanceof Error ? (error.stack ?? error.message) : String(error));
  await page.screenshot({ path: join(OUT, `g10-user-path-${LOCALE}-error.png`) }).catch(() => undefined);
} finally {
  await client?.close().catch(() => undefined);
  writeFileSync(join(OUT, `g10-user-path-${LOCALE}-console.txt`), lines.slice(-400).join("\n"));
  await browser.close();
}
const red = rows.filter((entry) => !entry.ok);
console.log(`g10-user-path (${LOCALE}): ${rows.length - red.length}/${rows.length} rows green in ${seconds()} s`);
process.exit(red.length === 0 ? 0 : 1);
