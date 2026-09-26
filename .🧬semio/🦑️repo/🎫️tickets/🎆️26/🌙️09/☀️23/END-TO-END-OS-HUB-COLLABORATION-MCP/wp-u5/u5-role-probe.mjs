#!/usr/bin/env bun
/** 👁️✏️ U5 — the Home role round trip: signs `U5_EMAIL` in, switches Home to Viewer and back to Editor through the navbar
 * role group, and records the role buttons' state, the session surface and the Home rows after each step.
 * Usage: bun u5-role-probe.mjs <baseUrl> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6580/", tag = "role"] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (m) => lines.push(`${stamp()} ${m.type()}: ${m.text()}`.slice(0, 400)));
const state = () => page.evaluate(() => ({
  roles: [...document.querySelectorAll('[id^="playground.navbar.roles."]')].map((element) => ({ id: element.id, pressed: element.getAttribute("aria-pressed") ?? element.getAttribute("aria-checked") ?? element.getAttribute("data-state"), disabled: element.hasAttribute("disabled") || element.getAttribute("aria-disabled") === "true", busy: element.closest("[aria-busy]")?.getAttribute("aria-busy") ?? null })),
  windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")),
  hubRows: [...new Set([...document.querySelectorAll('[data-ui-node-key^="space:"]')].map((element) => element.getAttribute("data-ui-node-key")))].filter((key) => key !== "space:default").length,
  notices: [...document.querySelectorAll('[role="status"], [role="alert"]')].map((element) => (element.textContent ?? "").trim()).filter((text) => text.length > 0 && text.length < 160).slice(0, 6),
}));
const report = { steps: [] };
const note = async (at) => { const now = await state(); report.steps.push({ at, ...now }); console.log(JSON.stringify({ at, ...now }).slice(0, 900)); };
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
await note("signed in");
await page.locator('[id="playground.navbar.roles.viewer"]').first().click({ force: true });
await page.waitForTimeout(5_000);
await note("viewer");
const editor = page.locator('[id="playground.navbar.roles.editor"]').first();
const clickError = await editor.click({ timeout: 5_000 }).then(() => null, (error) => String(error).slice(0, 600));
await page.waitForTimeout(6_000);
await note(`editor${clickError ? ` (click: ${clickError})` : ""}`);
await page.waitForTimeout(8_000);
await note("editor +14 s");
report.lines = lines.filter((line) => !/agent-bridge|Failed to load resource|\[vite\]/u.test(line)).slice(-60);
writeFileSync(out(`u5-role-${tag}.json`), JSON.stringify(report, null, 1));
await browser.close();
