#!/usr/bin/env bun
/** 🤖️ U5 — G10 S4's hub-pane path on a clean profile: hard-load `/spaces/<id>` signed out, sign in through the hub workspace,
 * wait until the agent pane is bound to the space, mint an agent delegation, press "Set up MCP client" and wait for its
 * terminal phase, then close the workspace and reopen it the way the badge/palette does and record what it says. Every
 * 250 ms it samples the MCP phase, the pane's bound space, the path, the route-admission notice, the session line and the
 * mounted windows; every request to the serve origin is timed, so a request that queues behind held-open connections shows.
 * Usage: bun u5-agent-install-probe.mjs <baseUrl> <spaceId> <tag>   (env U5_EMAIL / U5_PASSWORD) */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl, spaceId, tag] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const origin = new URL(baseUrl).origin;
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
const t0 = Date.now();
const at = () => ((Date.now() - t0) / 1000).toFixed(2);
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") lines.push(`${at()} ${m.type()}: ${m.text()}`.slice(0, 300)); });
const open = new Map();
const requests = [];
page.on("request", (request) => {
  if (!request.url().startsWith(origin)) return;
  open.set(request, { at: Number(at()), url: request.url().slice(origin.length, origin.length + 120), type: request.resourceType(), method: request.method() });
});
const settle = (request, outcome) => {
  const entry = open.get(request);
  if (!entry) return;
  open.delete(request);
  requests.push({ ...entry, outcome, ms: Math.round((Number(at()) - entry.at) * 1000) });
};
page.on("requestfinished", (request) => settle(request, "finished"));
page.on("requestfailed", (request) => settle(request, `failed ${request.failure()?.errorText ?? ""}`));
const report = { samples: [], steps: [] };
const state = () => page.evaluate(() => {
  const section = document.querySelector("[data-semio-hub-workspace]");
  return {
    path: location.pathname,
    mcp: document.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp") ?? null,
    space: document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space") ?? null,
    admission: document.querySelector("[data-semio-route-admission]")?.getAttribute("data-semio-route-admission") ?? null,
    signedOutLine: /Not signed in to a hub|Nicht bei einem Hub angemeldet/u.test(section?.textContent ?? ""),
    rows: section ? section.querySelectorAll("li[data-space-id]").length : 0,
    delegations: section ? section.querySelectorAll("li[data-delegation-id]").length : 0,
    windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).slice(0, 3).join(","),
  };
});
const sample = async (label) => {
  const now = await state();
  const key = JSON.stringify(now);
  if (report.samples.at(-1)?.key !== key) report.samples.push({ at: at(), label, key });
};
const step = async (name, extra = {}) => { const now = await state(); const row = { at: at(), name, ...now, ...extra }; report.steps.push(row); console.log(JSON.stringify(row)); };
const openStreams = () => [...open.values()].filter((entry) => entry.type === "eventsource" || entry.type === "fetch" || entry.type === "xhr").map((entry) => `${entry.type} ${entry.method} ${entry.url} since ${entry.at}`);
const sampleUntil = async (label, done, budgetMs) => {
  const started = Date.now();
  while (Date.now() - started < budgetMs) {
    await sample(label);
    if (await done()) return true;
    await page.waitForTimeout(250);
  }
  return false;
};

await page.goto(new URL(`spaces/${spaceId}`, baseUrl).toString(), { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") === "s", undefined, { timeout: 300_000 });
await sampleUntil("loaded", async () => false, 3_000);
await step("loaded signed out");
await page.locator("[data-semio-hub-sign-in]").first().click();
const workspace = page.locator("[data-semio-hub-workspace]");
await workspace.waitFor({ state: "visible", timeout: 30_000 });
for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
await workspace.locator('input[type="email"]').fill(process.env.U5_EMAIL);
await workspace.locator('input[type="password"]').fill(process.env.U5_PASSWORD);
await workspace.locator('form:has(input[type="password"]) button[type="submit"]').first().click();
const signedAt = Date.now();
const bound = await sampleUntil("after sign-in", async () => (await state()).space === spaceId, 90_000);
await step("pane bound to the space", { bound, seconds: ((Date.now() - signedAt) / 1000).toFixed(1) });
await sampleUntil("settling", async () => false, 8_000);
await step("8 s later");
await sampleUntil("waiting for the create form", async () => (await page.locator("input[data-element-alias='os.hub.agent.name']").count()) > 0, 180_000);
await page.locator("input[data-element-alias='os.hub.agent.name']").fill(`U5 ${tag}`).catch((error) => report.steps.push({ name: "no agent name input", error: String(error).slice(0, 200) }));
await page.locator('[id="os.hub.agent.create"]').click({ timeout: 10_000 }).catch((error) => report.steps.push({ name: "no create", error: String(error).slice(0, 200) }));
await page.locator('[data-semio-hub-agent-mcp="idle"]').waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
await step("delegation minted", { openRequests: openStreams() });
await page.locator('[id="os.hub.agent.mcpInstall"]').click({ timeout: 10_000 }).catch((error) => report.steps.push({ name: "no install", error: String(error).slice(0, 200) }));
const installAt = Date.now();
const terminal = await sampleUntil("after install", async () => ["ready", "failed", "unavailable"].includes((await state()).mcp ?? ""), 30_000);
await step("install settled", { terminal, seconds: ((Date.now() - installAt) / 1000).toFixed(2), install: requests.filter((row) => row.url.startsWith("/__semio/agent-credentials")), openRequests: openStreams() });
await page.locator('[id="os.hub.signIn.cancel"]').click({ timeout: 10_000 }).catch(() => undefined);
await sampleUntil("closed", async () => false, 3_000);
await step("workspace closed");
await page.locator("[data-semio-hub-sign-in]").first().click().catch(async () => {
  await page.evaluate(() => { history.pushState(null, "", "/hub"); dispatchEvent(new PopStateEvent("popstate")); });
});
await sampleUntil("reopened", async () => (await state()).delegations > 0, 150_000);
await step("workspace reopened");
await page.screenshot({ path: out(`u5-agent-install-${tag}.png`) });
report.requests = requests.filter((row) => row.ms > 2_000 || row.url.startsWith("/__semio/agent-credentials"));
report.stillOpen = openStreams();
report.lines = lines.filter((line) => !/Failed to load resource|WebSocket connection/u.test(line)).slice(-40);
writeFileSync(out(`u5-agent-install-${tag}.json`), JSON.stringify(report, null, 1));
for (const row of report.samples) console.log(`${row.at} ${row.label} ${row.key}`.slice(0, 320));
await browser.close();
