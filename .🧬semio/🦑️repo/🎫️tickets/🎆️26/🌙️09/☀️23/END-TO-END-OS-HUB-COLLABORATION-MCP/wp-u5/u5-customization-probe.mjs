#!/usr/bin/env bun
/** 🎨️ U5 — does customization survive a reload (same device) and reach a second device (fresh browser context, same hub
 * user)? Device A signs in, sets appearance, language, a keybinding override, saves a named window layout and changes the
 * window arrangement; the probe reads each back after a reload and on device B. `--list` only enumerates the palette's
 * customization commands. Usage: bun u5-customization-probe.mjs <baseUrl> <tag> [--list] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { click, dismissIntroduction, openPalette } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, tag, ...flags] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const report = { steps: [] };

async function boot(context) {
  const page = await context.newPage();
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
  await dismissIntroduction(page);
  await page.waitForTimeout(2_000);
  return page;
}

async function signIn(page) {
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
  await page.waitForTimeout(1_000);
}

async function paletteItems(page, query) {
  await openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.fill(query);
  await page.waitForTimeout(1_200);
  const items = await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((element) => ({ id: element.getAttribute("data-command-item-id"), text: (element.textContent ?? "").trim().slice(0, 60) })));
  return items;
}

async function runCommand(page, query, id) {
  const items = await paletteItems(page, query);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
  const present = await item.count();
  if (present) await item.click({ force: true });
  else await page.keyboard.press("Escape");
  await page.waitForTimeout(1_500);
  return { query, id, present, candidates: items.slice(0, 8) };
}

const SAVED_LAYOUT = "U5 saved layout";
const KEY_CHORD = "Control+Alt+KeyK";

async function openSettingsTab(page, tabId) {
  if ((await page.locator(`[id="${tabId}"]`).count()) === 0) await click(page, '[id="os.openSettings"], [data-slot="navbar"] [id*="settings" i], [id="framework.settings"]');
  await page.waitForTimeout(1_500);
  await page.locator(`[id="${tabId}"]`).first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_200);
}

async function pickSelect(page, triggerId, optionText) {
  await page.locator(`button[id="${triggerId}"], [role="combobox"][id="${triggerId}"]`).first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(700);
  const option = page.locator('[role="option"]').filter({ hasText: optionText }).first();
  const present = await option.count();
  if (present) await option.click({ force: true }).catch(() => undefined);
  else await page.keyboard.press("Escape").catch(() => undefined);
  await page.waitForTimeout(1_200);
  return present;
}

async function customize(page) {
  const done = {};
  await click(page, '[id="framework.panelToggle.display"], button:has-text("Display")');
  await page.waitForTimeout(1_500);
  await page.locator('[id="framework.display.layout"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(800);
  await page.locator('[id="framework.display.layout.save"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(800);
  const label = page.locator('[id="framework.display.saveLabel"]').first();
  done.layoutInput = await label.count();
  if (done.layoutInput) {
    await label.fill(SAVED_LAYOUT);
    await page.locator('[id="framework.display.save"]').first().click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_000);
  }
  await openSettingsTab(page, "framework.settings.keybindings");
  const capture = page.locator('[id^="framework.settings.keybindings.capture."]').first();
  done.keybindingControl = (await capture.getAttribute("id").catch(() => null))?.slice("framework.settings.keybindings.capture.".length) ?? null;
  if (done.keybindingControl) {
    const controlId = done.keybindingControl;
    const button = page.locator(`[id="framework.settings.keybindings.capture.${controlId}"]`).first();
    await button.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(600);
    done.captureState = (await button.textContent().catch(() => ""))?.trim().slice(0, 40) ?? "";
    await button.focus().catch(() => undefined);
    await page.keyboard.press(KEY_CHORD).catch(() => undefined);
    await page.waitForTimeout(1_000);
  }
  await openSettingsTab(page, "framework.settings.general");
  done.appearance = await pickSelect(page, "framework.settings.appearance", /^(Dark|Dunkel)$/u);
  done.language = await pickSelect(page, "framework.settings.language", /^(Deutsch|German)$/u);
  await page.waitForTimeout(2_000);
  await click(page, 'button:has-text("Verlauf"), button:has-text("History")');
  await page.waitForTimeout(2_000);
  done.historyPanel = await page.evaluate(() => document.querySelectorAll('[id^="framework.history."]').length);
  return done;
}

async function readCustomization(page, keybindingControl) {
  const base = await read(page);
  const historyPanelNodes = await page.evaluate(() => document.querySelectorAll('[id^="framework.history."]').length);
  await openSettingsTab(page, "framework.settings.keybindings");
  const keybinding = keybindingControl === null ? null : await page.evaluate((controlId) => (document.querySelector(`[id="framework.settings.keybindings.${controlId}"]`)?.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120), keybindingControl);
  await click(page, '[id="framework.panelToggle.display"], button:has-text("Display"), button:has-text("Anzeige")');
  await page.waitForTimeout(1_200);
  await page.locator('[id="framework.display.layout"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(800);
  await page.locator('[id="framework.display.layout.group.saved"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(600);
  const savedLayout = await page.evaluate((name) => (document.body.innerText ?? "").includes(name), SAVED_LAYOUT);
  return { ...base, historyPanelNodes, keybinding, savedLayout };
}

const read = (page) => page.evaluate(() => ({
  appearance: [...document.querySelectorAll("[data-appearance], [data-ui-appearance], [data-color-scheme]")].slice(0, 3).map((element) => `${element.tagName.toLowerCase()}:${element.getAttribute("data-appearance") ?? element.getAttribute("data-ui-appearance") ?? element.getAttribute("data-color-scheme")}`).join(",") || `html.class=${document.documentElement.className.slice(0, 60)}`,
  background: getComputedStyle(document.body).backgroundColor,
  colorScheme: getComputedStyle(document.documentElement).colorScheme,
  lang: document.documentElement.lang,
  windows: [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")),
  storageKeys: Object.keys(localStorage).filter((key) => /pref|layout|config/iu.test(key)).slice(0, 12),
  dockUi: (() => { try { const apps = JSON.parse(localStorage.getItem("semio.os.config") ?? "{}").dockUi?.apps ?? {}; return Object.fromEntries(Object.entries(apps).map(([app, value]) => [app, Object.fromEntries(Object.entries(value?.anchors ?? {}).map(([anchor, entry]) => [anchor, `${entry?.visible ? "visible" : "hidden"}:${(entry?.path ?? []).join("/")}`]))])); } catch { return null; } })(),
  openPanels: ["framework.display.layout", "framework.settings.general", "framework.history.commands"].filter((id) => document.querySelector(`[id="${id}"]`) !== null),
}));

const contextA = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const a = await boot(contextA);
if (flags.includes("--list")) {
  for (const query of ["dark", "light", "appearance", "Deutsch", "language", "layout", "keybinding", "shortcut", "theme"]) console.log(JSON.stringify({ query, items: await paletteItems(a, query) }).slice(0, 900)), await a.keyboard.press("Escape");
  await browser.close();
  process.exit(0);
}
await signIn(a);
report.steps.push({ at: "A before", ...(await read(a)) });
const done = await customize(a);
report.steps.push({ at: "A customized", done, ...(await readCustomization(a, done.keybindingControl)) });
await a.screenshot({ path: out(`u5-customization-${tag}-a.png`) });
await a.reload({ waitUntil: "commit" });
await a.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await a.waitForTimeout(6_000);
report.steps.push({ at: "A reloaded", ...(await readCustomization(a, done.keybindingControl)) });
await a.screenshot({ path: out(`u5-customization-${tag}-a-reloaded.png`) });
const contextB = await browser.newContext({ viewport: { width: 1440, height: 900 } });
const b = await boot(contextB);
await signIn(b);
await b.waitForTimeout(4_000);
report.steps.push({ at: "B signed in (second device)", ...(await readCustomization(b, done.keybindingControl)) });
await b.screenshot({ path: out(`u5-customization-${tag}-b.png`) });
writeFileSync(out(`u5-customization-${tag}.json`), JSON.stringify(report, null, 1));
for (const step of report.steps) console.log(JSON.stringify(step).slice(0, 600));
await browser.close();
