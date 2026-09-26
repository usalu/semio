#!/usr/bin/env bun
/** 🤖️ U5 — G10 S4's hub-pane path on a clean profile: hard-load `/spaces/<id>`, sign in through the hub workspace, mint an
 * agent delegation and press "Set up MCP client", sampling every 250 ms the MCP phase, the pane's bound space, the hub
 * connection id, whether the workspace element was remounted, the path and the session line; then reopens the workspace
 * from the palette and records what it says. Usage: bun u5-agent-install-probe.mjs <baseUrl> <spaceId> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl, spaceId, tag] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
const t0 = Date.now();
const at = () => ((Date.now() - t0) / 1000).toFixed(2);
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") lines.push(`${at()} ${m.type()}: ${m.text()}`.slice(0, 300)); });
const report = { samples: [], steps: [] };
const state = () => page.evaluate(() => {
  const section = document.querySelector("[data-semio-hub-workspace]");
  if (section && !section.__u5) section.__u5 = Math.random().toString(16).slice(2, 8);
  return {
    path: location.pathname,
    workspace: section?.getAttribute("data-semio-hub-workspace") ?? null,
    element: section?.__u5 ?? null,
    mcp: document.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp") ?? null,
    space: document.querySelector("[data-semio-hub-agent-space]")?.getAttribute("data-semio-hub-agent-space") ?? null,
    signedOutLine: /Not signed in to a hub|Nicht bei einem Hub angemeldet/u.test(section?.textContent ?? ""),
    rows: section ? section.querySelectorAll("li[data-space-id]").length : 0,
    windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).slice(0, 3),
  };
});
const sample = async (label) => {
  const now = await state();
  const key = JSON.stringify(now);
  if (report.samples.at(-1)?.key !== key) report.samples.push({ at: at(), label, key, ...now });
};
const step = async (name) => { const now = await state(); report.steps.push({ at: at(), name, ...now }); console.log(JSON.stringify({ at: at(), name, ...now })); };
await page.goto(new URL(`spaces/${spaceId}`, baseUrl).toString(), { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.documentElement.getAttribute("data-semio-os-ready") === "s", undefined, { timeout: 300_000 });
await step("loaded");
await page.locator("[data-semio-hub-sign-in]").first().click();
const workspace = page.locator("[data-semio-hub-workspace]");
await workspace.waitFor({ state: "visible", timeout: 30_000 });
for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
await workspace.locator('input[type="email"]').fill(process.env.U5_EMAIL);
await workspace.locator('input[type="password"]').fill(process.env.U5_PASSWORD);
await workspace.locator('form:has(input[type="password"]) button[type="submit"]').first().click();
const signedAt = Date.now();
while (Date.now() - signedAt < 20_000) { await sample("after sign-in"); await page.waitForTimeout(250); }
await step("20 s after sign-in");
await page.locator("input[data-element-alias='os.hub.agent.name']").fill(`U5 ${tag}`).catch((error) => report.steps.push({ name: "no agent name input", error: String(error).slice(0, 200) }));
await page.locator('[id="os.hub.agent.create"]').click({ timeout: 10_000 }).catch((error) => report.steps.push({ name: "no create", error: String(error).slice(0, 200) }));
await page.locator('[data-semio-hub-agent-mcp="idle"]').waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
await step("delegation minted");
await page.locator('[id="os.hub.agent.mcpInstall"]').click({ timeout: 10_000 }).catch((error) => report.steps.push({ name: "no install", error: String(error).slice(0, 200) }));
const installAt = Date.now();
while (Date.now() - installAt < 20_000) { await sample("after install"); await page.waitForTimeout(250); }
await step("20 s after install");
await page.screenshot({ path: out(`u5-agent-install-${tag}.png`) });
report.lines = lines.filter((line) => !/agent-bridge|Failed to load resource|WebSocket connection/u.test(line)).slice(-40);
writeFileSync(out(`u5-agent-install-${tag}.json`), JSON.stringify(report, null, 1));
for (const row of report.samples) console.log(`${row.at} ${row.label} ${row.key}`.slice(0, 300));
await browser.close();
