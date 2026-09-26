#!/usr/bin/env bun
/** 🔗️ U5 — G10 S4's two hub-overlay symptoms: signs `U5_EMAIL` in, enters a space, then opens "Open Hub and Spaces" from
 * the palette (at rest, and immediately after a hard route change while that space route is still in flight) and
 * records whether the `/hub` overlay shows and which space its agent pane names. Usage: bun u5-hub-overlay-probe.mjs <baseUrl> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dismissIntroduction, openPalette } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, tag] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") lines.push(`${stamp()} ${m.type()}: ${m.text()}`.slice(0, 400)); });
page.on("pageerror", (e) => lines.push(`${stamp()} pageerror: ${String(e)}`.slice(0, 400)));
const report = { steps: [] };
const overlayState = () => page.evaluate(() => {
  const overlay = document.querySelector("[data-semio-hub-workspace]");
  const text = (overlay?.textContent ?? "").replace(/\s+/gu, " ");
  return { path: location.pathname, overlay: overlay !== null, noSpace: /Open a space to manage|Öffne einen Space/u.test(text), spaceWindows: [...document.querySelectorAll('[data-ui-node-key="s-space-create-artifact"]')].length };
});
const note = async (at, extra = {}) => { const now = { at, ...extra, ...(await overlayState()) }; report.steps.push(now); console.log(JSON.stringify(now)); };
const openHubFromPalette = async () => {
  await openPalette(page);
  await page.locator("[role='dialog'] [data-slot='command-input']").first().fill("hub");
  await page.waitForTimeout(800);
  const item = page.locator('[data-slot="command-item"]').filter({ hasText: /Hub/u }).first();
  const id = await item.getAttribute("data-command-item-id").catch(() => null);
  await item.click({ force: true }).catch(() => undefined);
  return id;
};
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true });
const form = page.locator("[data-semio-hub-workspace]");
await form.waitFor({ state: "visible", timeout: 60_000 });
for (const skip of await page.getByRole("button", { name: /^(Skip|Überspringen)$/u }).all()) await skip.click({ force: true }).catch(() => undefined);
await form.locator('input[type="email"]').fill(process.env.U5_EMAIL ?? "bo@example.org");
await form.locator('input[type="password"]').fill(process.env.U5_PASSWORD ?? "correct horse battery staple");
await form.locator('[id="os.hub.signIn.submit"]').click({ force: true });
await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
await page.waitForTimeout(2_000);
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
await page.waitForFunction(() => [...document.querySelectorAll('[data-ui-node-key^="space:"]')].filter((row) => row.getAttribute("data-ui-node-key") !== "space:default").length >= 2, undefined, { timeout: 60_000 }).catch(() => undefined);
const spaceIds = await page.evaluate(() => [...new Set([...document.querySelectorAll('[data-ui-node-key^="space:"]')].map((row) => row.getAttribute("data-ui-node-key").slice("space:".length)))].filter((id) => id !== "default").slice(0, 2));
report.spaceIds = spaceIds;
await page.goto(new URL(`spaces/${spaceIds[0]}`, baseUrl).toString(), { waitUntil: "domcontentloaded" });
await page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "attached", timeout: 90_000 }).catch(() => undefined);
await page.waitForTimeout(2_000);
await note("in space A");
const commandId = await openHubFromPalette();
await page.waitForTimeout(1_500);
await note("palette hub at rest", { commandId });
await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
await page.waitForTimeout(2_000);
await note("overlay closed");
await page.evaluate((id) => { history.pushState({}, "", `/spaces/${id}`); dispatchEvent(new PopStateEvent("popstate")); }, spaceIds[1]);
await page.waitForTimeout(150);
await openHubFromPalette();
await page.waitForTimeout(1_500);
await note("palette hub while space B route in flight");
await page.waitForTimeout(8_000);
await note("palette hub +8 s");
await page.screenshot({ path: out(`u5-hub-overlay-${tag}.png`) });
report.lines = lines.filter((line) => !/agent-bridge|Failed to load resource|WebSocket connection/u.test(line)).slice(-40);
writeFileSync(out(`u5-hub-overlay-${tag}.json`), JSON.stringify(report, null, 1));
for (const line of report.lines.slice(-10)) console.log(line.slice(0, 300));
await browser.close();
