#!/usr/bin/env bun
/** ⚙️ U5 — Settings while a spawned program owns the canvas: spawns `<pluginId>`, opens the shell's Settings, and records
 * what it offers (tabs, a language control), the page's body size and every console error, with screenshots; then picks
 * German when a language control exists. Usage: bun u5-settings-focus-probe.mjs <baseUrl> <tag> <pluginId> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { click, dismissIntroduction, spawnProgram } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, tag, pluginId] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = [];
const stamp = () => new Date().toISOString().slice(11, 23);
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning") lines.push(`${stamp()} ${m.type()}: ${m.text()}`.slice(0, 500)); });
page.on("pageerror", (e) => lines.push(`${stamp()} pageerror: ${String(e)}`.slice(0, 500)));
const report = { steps: [] };
const state = () => page.evaluate(() => ({
  bodyChildren: document.body.children.length,
  windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")),
  settingsTabs: [...document.querySelectorAll('[role="tab"]')].map((element) => (element.textContent ?? "").trim()).filter((text) => text.length > 0 && text.length < 40).slice(0, 24),
  languageControls: [...document.querySelectorAll('select, [role="combobox"]')].map((element) => (element.textContent ?? element.getAttribute("aria-label") ?? "").trim().slice(0, 60)).filter((text) => /english|deutsch|german|englisch/iu.test(text)),
  lang: document.documentElement.lang,
}));
const note = async (at) => { const now = await state(); report.steps.push({ at, ...now }); console.log(JSON.stringify({ at, ...now }).slice(0, 700)); await page.screenshot({ path: out(`u5-settings-${tag}-${report.steps.length}.png`) }); };
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const spawned = await spawnProgram(page, pluginId);
report.spawned = spawned;
await page.waitForTimeout(2_000);
await note("spawned");
await click(page, '[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], [id="framework.settings"]');
await page.waitForTimeout(3_000);
await note("settings opened");
report.generalCandidates = await page.evaluate(() => [...document.querySelectorAll("[data-tab-id], [role=tab], button")].filter((element) => /general|allgemein/iu.test(element.textContent ?? "") || /general/iu.test(element.getAttribute("data-tab-id") ?? "")).map((element) => ({ tag: element.tagName, id: element.id, tab: element.getAttribute("data-tab-id"), role: element.getAttribute("role"), text: (element.textContent ?? "").trim().slice(0, 40) })).slice(0, 8));
console.log(JSON.stringify(report.generalCandidates));
await page.locator('[data-tab-id*="general" i]').first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(2_000);
await note("general tab");
const language = page.locator('[role="tab"], [role="button"], button').filter({ hasText: /^(language|sprache)$/iu }).first();
if ((await language.count()) > 0) {
  await language.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  await note("language tab");
}
report.lines = lines.filter((line) => !/agent-bridge|Failed to load resource|WebSocket connection/u.test(line));
writeFileSync(out(`u5-settings-${tag}.json`), JSON.stringify(report, null, 1));
for (const line of report.lines.slice(-15)) console.log(line.slice(0, 300));
await browser.close();
