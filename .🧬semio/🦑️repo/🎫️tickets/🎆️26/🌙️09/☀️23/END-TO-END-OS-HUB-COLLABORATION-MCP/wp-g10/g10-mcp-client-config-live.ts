#!/usr/bin/env bun
/** 🔌️ G10 live proof of G-P2-3: a signed-in human turns an agent delegation into a working MCP client
 * configuration from inside the shell, a real MCP client started from exactly that configuration acts
 * in the space as the agent principal, and withdrawing the delegation in the same pane stops it.
 *
 * usage: bun g10-mcp-client-config-live.ts <shellOrigin> <hubOrigin> <en|de> <captureDir>
 * One human (user1) signs in through the shell's own hub sign-in; the space is created over the
 * hub's directory command route so the run owns a fresh one. */
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { randomBytes } from "node:crypto";
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
import { spawnRawMcp } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts";
import { createSpaceCommandV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🏘️spaces/🟦️.ts";

const [SHELL = "http://127.0.0.1:6532", HUB = "http://127.0.0.1:8030", LOCALE = "en", OUT = "/Users/ueli/Documents/semio/.tmp-ticket/wp-g10/generated"] = process.argv.slice(2);
const WORDS = {
  en: { install: "Set up MCP client", copy: "Copy MCP configuration", title: "Connect an AI client", create: "Create delegation", withdrawConfirm: "Withdraw access" },
  de: { install: "MCP-Client einrichten", copy: "MCP-Konfiguration kopieren", title: "KI-Client verbinden", create: "Zugang erstellen", withdrawConfirm: "Zugang zurückziehen" },
}[LOCALE === "de" ? "de" : "en"];
mkdirSync(OUT, { recursive: true });
const rows: { step: string; ok: boolean; detail: string }[] = [];
const row = (step: string, ok: boolean, detail: string): void => {
  rows.push({ step, ok, detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${step} — ${detail}`);
};
const hub = async (method: string, path: string, token?: string, body?: unknown, extra: Record<string, string> = {}) => {
  const response = await fetch(`${HUB}${path}`, { method, headers: { ...(body === undefined ? {} : { "content-type": "application/json" }), ...(token ? { authorization: `Bearer ${token}` } : {}), ...extra }, ...(body === undefined ? {} : { body: typeof body === "string" ? body : JSON.stringify(body) }) });
  const text = await response.text();
  let json: any;
  try {
    json = JSON.parse(text);
  } catch {}
  return { status: response.status, text, json };
};

const signIn = await hub("POST", "/auth/sessions", undefined, { schema: "semio.hub.auth.credential-sign-in/v1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1", deviceInstanceId: `g10mcpclient${randomBytes(10).toString("hex")}`, clientClass: "browser" });
const token = String(signIn.json?.token ?? "");
const spaceName = `G10 agents ${LOCALE} ${randomBytes(3).toString("hex")}`;
const created = await hub("POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(randomBytes(16).toString("hex"), createSpaceCommandV1(spaceName, "atelier", "private"))), { origin: HUB });
const spaceId = String((await hub("GET", "/directory/spaces", token)).json?.find((entry: any) => entry?.space?.name === spaceName)?.space?.id ?? "");
row("0 setup: user1 owns a fresh space", signIn.status === 200 && created.status === 202 && spaceId.length > 0, `sign-in ${signIn.status}, create-space ${created.status}, space=${spaceId}`);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 }, locale: LOCALE === "de" ? "de-DE" : "en-US" });
await context.grantPermissions(["clipboard-read", "clipboard-write"], { origin: SHELL });
const page = await context.newPage();
const lines: string[] = [];
page.on("console", (message) => lines.push(`${message.type()} ${message.text().slice(0, 400)}`));
const gateways: ReturnType<typeof spawnRawMcp>[] = [];
try {
  await page.goto(`${SHELL}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
  await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") === "s" && document.title.includes("space"), undefined, { timeout: 300_000 });
  await page.locator("[data-semio-hub-sign-in]").first().waitFor({ state: "visible", timeout: 60_000 });
  await page.locator("[data-semio-hub-sign-in]").first().click();
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 30_000 });
  for (let attempt = 0; attempt < 5; attempt += 1) {
    const veil = page.locator('[data-slot="introduction-veil"] button, [data-semio-hub-workspace] button', { hasText: /^\s*(skip|überspringen)\s*$/iu }).first();
    if ((await veil.count()) === 0) break;
    await veil.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(400);
  }
  await workspace.locator('input[type="email"]').fill("user1@semio.dev");
  await workspace.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
  await workspace.locator('form:has(input[type="password"]) button[type="submit"]').first().click();
  await page.waitForFunction(() => document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space")?.length, undefined, { timeout: 60_000 }).catch(() => undefined);
  const paneSpace = await page.locator("[data-semio-hub-agent-space]").first().getAttribute("data-semio-hub-agent-space").catch(() => null);
  row("1 the human signs in through the shell and the agent pane is bound to the open space", paneSpace === spaceId, `pane space=${paneSpace}`);

  const label = `Claude ${LOCALE} ${randomBytes(2).toString("hex")}`;
  await page.locator("input[data-element-alias='os.hub.agent.name']").fill(label);
  await page.locator('[id="os.hub.agent.create"]').click();
  await page.locator('[data-semio-hub-agent-mcp="idle"]').waitFor({ state: "visible", timeout: 60_000 });
  const block = page.locator("[data-semio-hub-agent-mcp]");
  const titleText = await block.innerText();
  const installName = await page.locator('[id="os.hub.agent.mcpInstall"]').getAttribute("aria-label");
  row(`2 a fresh delegation offers the MCP client set-up (${LOCALE})`, titleText.includes(WORDS.title) && installName === WORDS.install, `title=${titleText.split("\n")[0]} install="${installName}"`);

  await page.locator('[id="os.hub.agent.mcpInstall"]').click();
  await page.locator('[data-semio-hub-agent-mcp="ready"]').waitFor({ state: "visible", timeout: 30_000 });
  const configText = await page.locator("[data-semio-hub-agent-mcp-config]").innerText();
  await page.screenshot({ path: join(OUT, `g10-mcp-client-config-${LOCALE}.png`) });
  const config = JSON.parse(configText) as { mcpServers: Record<string, { type: string; command: string; args: string[] }> };
  const [serverName, entry] = Object.entries(config.mcpServers)[0] ?? ["", { type: "", command: "", args: [] }];
  const credentialPath = entry.args[entry.args.indexOf("--credential-file") + 1] ?? "";
  const mode = existsSync(credentialPath) ? (statSync(credentialPath).mode & 0o777).toString(8) : "missing";
  const tokenLeak = /delegation\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}/u.test(configText);
  row("3 the pane shows one complete stdio server entry naming an owner-only credential, never the token", Object.keys(config.mcpServers).length === 1 && entry.args.includes("--hub") && entry.args[entry.args.indexOf("--hub") + 1] === HUB && entry.args[entry.args.indexOf("--space") + 1] === spaceId && mode === "600" && !tokenLeak, `server=${serverName} command=${entry.command} credential=${credentialPath} mode=${mode} tokenInConfig=${tokenLeak}`);

  await page.locator('[id="os.hub.agent.mcpCopy"]').click();
  await page.locator('[data-semio-hub-agent-mcp="copied"]').waitFor({ state: "visible", timeout: 10_000 }).catch(() => undefined);
  const clipboard = await page.evaluate(() => navigator.clipboard.readText()).catch(() => "");
  const copyName = await page.locator('[id="os.hub.agent.mcpCopy"]').getAttribute("aria-label");
  row(`4 Copy puts exactly the shown configuration on the clipboard (${LOCALE})`, clipboard === configText || clipboard.trim() === configText.trim(), `clipboard=${clipboard.length}B shown=${configText.length}B copy="${copyName}"`);

  const beforeRevoke = join(mkdtempSync(join(tmpdir(), "g10-cred-")), "credential.json");
  copyFileSync(credentialPath, beforeRevoke);
  chmodSync(beforeRevoke, 0o600);
  process.env.S_AGENT_BRIDGE_DIR = mkdtempSync(join(tmpdir(), "g10-bridge-"));
  const startFromConfig = (args: readonly string[]) => {
    const proc = spawnRawMcp(entry.command, args);
    gateways.push(proc);
    return proc;
  };
  const agent = startFromConfig(entry.args);
  const initialized = await agent.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g10-mcp-client-config", version: "1" } }, 600_000);
  agent.writeRaw(JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" }));
  const context1 = await agent.request("tools/call", { name: "context_resolve", arguments: {} }, 120_000);
  const workspaceRead = await agent.request("resources/read", { uri: "semio://workspace" }, 120_000);
  const workspaceBody = String((workspaceRead.result as any)?.contents?.[0]?.text ?? "");
  const principal = String((context1.result as any)?.structuredContent?.principal ?? (context1.result as any)?.structuredContent?.principalId ?? "");
  row("5 a real MCP client started from exactly that entry is the agent principal in that space", initialized.error === undefined && workspaceBody.includes(spaceId) && principal.length > 0, `server=${(initialized.result as any)?.serverInfo?.name} principal=${principal} workspace=${workspaceBody.slice(0, 160)}`);
  await agent.close();

  await page.locator("li[data-delegation-id]", { hasText: label }).locator("button").first().click();
  await page.locator('[data-semio-hub-agent-revoke-confirm]').waitFor({ state: "visible", timeout: 10_000 });
  await page.locator("[data-semio-hub-agent-revoke-confirm] button").first().click();
  await page.waitForFunction((name: string) => [...document.querySelectorAll("li[data-delegation-id]")].some((li) => li.textContent?.includes(name) && li.getAttribute("data-delegation-state") === "revoked"), label, { timeout: 30_000 }).catch(() => undefined);
  for (let tick = 0; tick < 40 && existsSync(credentialPath); tick += 1) await page.waitForTimeout(250);
  row("6 withdrawing the delegation in the pane removes the installed credential", !existsSync(credentialPath), `installed file present=${existsSync(credentialPath)}`);

  const revokedArgs = entry.args.map((arg) => (arg === credentialPath ? beforeRevoke : arg));
  const stale = startFromConfig(revokedArgs);
  const refused = await stale.request("initialize", { protocolVersion: "2025-06-18", capabilities: {}, clientInfo: { name: "g10-mcp-client-config-revoked", version: "1" } }, 120_000).catch((error: unknown) => ({ error: { message: String(error) } }) as any);
  const stderr = stale.stderrText();
  row("7 a client holding the withdrawn credential is refused by the hub", refused.error !== undefined || /revoked|forbidden|403|delegation/iu.test(stderr), `initialize=${JSON.stringify(refused.error ?? refused.result ?? {}).slice(0, 160)} stderr=${stderr.slice(-240).replace(/\s+/gu, " ")}`);
  await stale.close();
} catch (error) {
  row("run", false, error instanceof Error ? error.stack ?? error.message : String(error));
  await page.screenshot({ path: join(OUT, `g10-mcp-client-config-${LOCALE}-error.png`) }).catch(() => undefined);
} finally {
  for (const proc of gateways) await proc.close().catch(() => undefined);
  writeFileSync(join(OUT, `g10-mcp-client-config-${LOCALE}-console.txt`), lines.slice(-400).join("\n"));
  await browser.close();
}
const red = rows.filter((entry) => !entry.ok);
console.log(`g10-mcp-client-config (${LOCALE}): ${rows.length - red.length}/${rows.length} rows green`);
process.exit(red.length === 0 ? 0 : 1);
