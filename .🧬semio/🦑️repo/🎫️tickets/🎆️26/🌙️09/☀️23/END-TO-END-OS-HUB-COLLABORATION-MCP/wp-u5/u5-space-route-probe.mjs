#!/usr/bin/env bun
/** 🧭️ U5 — a hard load of `/spaces/<id>` on a signed-in shell must land in that space: signs `U5_EMAIL` in, reads a
 * space id from Home, then loads `/spaces/<id>` N times and records, per attempt, the URL, the windows the canvas holds,
 * whether the Space app mounted (its `s-space-create-artifact` button) and every refusal/fault line.
 * Usage: bun u5-space-route-probe.mjs <baseUrl> <tag> [attempts] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6580/", tag = "route", attemptsText = "3"] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (m) => lines.push(`${stamp()} ${m.type()}: ${m.text()}`.slice(0, 500)));
page.on("response", (response) => { if (response.status() >= 400) lines.push(`${stamp()} http ${response.status()} ${response.url().replace(/^https?:\/\/[^/]+/u, "").slice(0, 200)}`); });
page.on("pageerror", (e) => lines.push(`${stamp()} pageerror: ${String(e)}`.slice(0, 500)));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await page.waitForTimeout(3_000);
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
await page.waitForFunction(() => [...document.querySelectorAll('[data-ui-node-key^="space:"]')].some((row) => row.getAttribute("data-ui-node-key") !== "space:default"), undefined, { timeout: 60_000 }).catch(() => undefined);
const spaceId = await page.evaluate(() => [...document.querySelectorAll('[data-ui-node-key^="space:"]')].map((row) => row.getAttribute("data-ui-node-key").slice("space:".length)).find((id) => id !== "default") ?? null);
const report = { baseUrl, spaceId, attempts: [] };
for (let attempt = 1; attempt <= Number(attemptsText); attempt += 1) {
  const from = lines.length;
  const started = Date.now();
  await page.goto(new URL(`spaces/${spaceId}`, baseUrl).toString(), { waitUntil: "domcontentloaded" });
  const mounted = await page.locator('[data-ui-node-key="s-space-create-artifact"]').first().waitFor({ state: "attached", timeout: 90_000 }).then(() => true).catch(() => false);
  await page.waitForTimeout(1_500);
  const state = await page.evaluate(() => ({ url: location.pathname, windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")), homeRows: document.querySelectorAll('[data-ui-node-key^="space:"]').length, cards: [...document.querySelectorAll("[data-window-id]")].map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").slice(0, 160)).filter((text) => /fault|revoked|failed|refus/iu.test(text)) }));
  const row = { attempt, mounted, ms: Date.now() - started, ...state, faults: lines.slice(from).filter((line) => /revoked|intake-rejected|refus|fault|pageerror|failed/iu.test(line) && !/404|typed-operation slots/u.test(line)).slice(-8), log: lines.slice(from).filter((line) => !/agent-bridge|Failed to load resource|\[vite\]/u.test(line)).slice(-120) };
  report.attempts.push(row);
  console.log(JSON.stringify({ ...row, log: undefined }).slice(0, 900));
  await page.screenshot({ path: out(`u5-space-route-${tag}-${attempt}.png`) });
}
writeFileSync(out(`u5-space-route-${tag}.json`), JSON.stringify(report, null, 1));
const consoleErrors = lines.filter((line) => / error: |pageerror:| http [45]\d\d /u.test(line));
console.log(`console errors: ${consoleErrors.length}`);
for (const line of consoleErrors) console.log(line.slice(0, 300));
await browser.close();
