#!/usr/bin/env bun
/** 🔬️ Slice S5 — a spawned foreign program now owns windows on the `s` canvas (S5 §4). This measures
 * what those windows actually EXPOSE: the Actions rail (folded by default, so its controls are not in
 * the DOM until it is opened), the utility bar, and every `[data-action-id]` the page carries — so the
 * mutate/undo/redo half is driven by a real affordance instead of a guessed selector.
 *
 * Usage: bun 🐍️s5-spawned-action-diagnose.mjs [uiOrigin] [pluginId]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const plugin = process.argv[3] ?? "draw";
const ACCOUNT = { email: process.env.S2_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S2_SIGN_IN_PASSWORD ?? "collab e2e first human phrase" };
const say = (...parts) => console.log("[s5]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const faults = [];
page.on("console", (message) => { const text = message.text(); if (/dropped|refus|reject|not-ui-safe|fault|undeclared/iu.test(text)) faults.push(text.slice(0, 320)); });
page.on("pageerror", (error) => faults.push(`pageerror ${String(error).slice(0, 220)}`));
const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s5-${name}.png`, import.meta.url)) }).catch(() => undefined);

const windowIds = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")));
const actionControls = () => page.evaluate(() => [...document.querySelectorAll("[data-action-id]")].map((element) => `${element.id || "(no id)"}|${element.getAttribute("data-action-id")}|${element.tagName}|${element.disabled === true ? "disabled" : "enabled"}`));
const railToggles = () => page.evaluate(() => [...document.querySelectorAll('[id*="actions"], [aria-label*="Action"], [data-slot="window-actions"], [data-slot="window-actions-toggle"]')].map((element) => `${element.id || "(no id)"}|${element.getAttribute("data-slot") ?? element.tagName}|${element.getAttribute("aria-expanded") ?? ""}|${(element.textContent ?? "").trim().slice(0, 24)}`).slice(0, 40));
const utilityIds = () => page.evaluate(() => [...document.querySelectorAll('[data-utility-id], [id*="utility"]')].map((element) => `${element.id || "(no id)"}|${element.getAttribute("data-utility-id") ?? ""}`).slice(0, 40));

const openPalette = async () => {
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press("Meta+p");
  await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
};
const spawn = async (filter, matcher) => {
  const before = await windowIds();
  await openPalette();
  await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(filter);
  await page.waitForTimeout(1_200);
  const item = matcher ? page.locator('[data-slot="command-item"]').filter({ hasText: matcher }).first() : page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${filter}"]`).first();
  if ((await item.count()) === 0) { await page.keyboard.press("Escape"); return null; }
  await item.click({ force: true }).catch(() => undefined);
  for (let waited = 0; waited < 60_000; waited += 400) {
    const opened = (await windowIds()).filter((id) => !before.includes(id));
    if (opened.length > 0) return opened;
    await page.waitForTimeout(400);
  }
  return [];
};

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined || document.documentElement.dataset.semioOsError !== undefined, undefined, { timeout: 300_000 }).catch(() => undefined);
  await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true, timeout: 60_000 }).catch(() => undefined);
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  await workspace.locator('input[type="email"]').fill(ACCOUNT.email);
  await workspace.locator('input[type="password"]').fill(ACCOUNT.password);
  await workspace.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForTimeout(12_000);
  say(`signed in; windows=${JSON.stringify(await windowIds())}`);

  say(`studio windows: ${JSON.stringify(await spawn("space", /s\s*·\s*studio/iu))}`);
  await page.waitForTimeout(4_000);
  const spawned = await spawn(plugin);
  say(`${plugin} windows: ${JSON.stringify(spawned)}`);
  await page.waitForTimeout(6_000);
  await shot(`spawned-${plugin}`);

  say(`action controls (folded): ${JSON.stringify(await actionControls())}`);
  say(`rail toggles: ${JSON.stringify(await railToggles())}`);
  say(`utility ids: ${JSON.stringify(await utilityIds())}`);
  say(`window element ids: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id^="framework.window"]')].map((element) => element.id).slice(0, 60)))}`);

  // 🎛️ Open every Actions rail the canvas shows — the rail is folded by default, so its controls are
  // not in the DOM at all until it is expanded.
  // 🎛️ The Actions rail lives in the window's ENGAGEMENT pane (`🪟️Window/🟦️.tsx:370-387`), whose
  // toggle is `framework.window.<segment>.engagement.toggle` — not an `.actions` id.
  const toggles = page.locator('[id$=".engagement.toggle"]');
  const toggleCount = await toggles.count();
  say(`actions toggles: ${toggleCount}`);
  for (let index = 0; index < toggleCount; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(3_000);
  say(`action controls (unfolded): ${JSON.stringify(await actionControls())}`);
  say(`tree ids: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id^="tree."], [id^="action."], [id^="section."]')].map((element) => element.id).slice(0, 60)))}`);
  say(`action-pane slots: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] *[id]')].map((element) => `${element.id}|${element.tagName}`).slice(0, 60)))}`);
  say(`window body slots: ${JSON.stringify(await page.evaluate(() => [...new Set([...document.querySelectorAll("[data-slot]")].map((element) => element.getAttribute("data-slot")))].slice(0, 80)))}`);
  await shot(`spawned-${plugin}-actions`);
  say(`body: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 900))}`);

  // 🖱️ Click a rail row and read EVERYTHING the shell says about it — a refusal is a `console.warn`,
  // which a probe filtering on `type() === "error"` never sees.
  const chatter = [];
  page.on("console", (message) => chatter.push(`${message.type()}: ${message.text().slice(0, 300)}`));
  const historyRow = (id) => page.evaluate((rowId) => { const element = document.getElementById(rowId); return element === null ? "absent" : `${element.getAttribute("aria-disabled") ?? "-"}|${element.className.includes("disabled") ? "class-disabled" : "enabled"}`; }, id);
  const engagement = () => page.evaluate(() => ([...document.querySelectorAll('[data-slot="engagement-status"]')].map((e) => (e.textContent ?? "").trim()).join(" | ").slice(0, 160)));
  say(`undo/redo before: ${await historyRow("action.undo")} / ${await historyRow("action.redo")} ; engagement ${JSON.stringify(await engagement())}`);

  // 🧾️ A staged verb (label ends with `…`) EXPANDS a form on the first click; the second control is
  // its Execute. Dump the expanded subtree so the executing control is named, not guessed.
  const staged = process.argv[4] ?? "addLayer";
  const stagedRow = page.locator(`[data-slot="window-action-pane"] [id="action.${staged}"]`).first();
  if ((await stagedRow.count()) > 0) {
    chatter.length = 0;
    await stagedRow.scrollIntoViewIfNeeded().catch(() => undefined);
    await stagedRow.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(2_000);
    say(`expanded ${staged}: ${JSON.stringify(await page.evaluate((verb) => [...document.querySelectorAll('[data-slot="window-action-pane"] *')].filter((element) => element.id?.startsWith(`action.${verb}`) || element.tagName === "BUTTON").map((element) => `${element.id || "(no id)"}|${element.tagName}|${(element.textContent ?? "").trim().slice(0, 20)}`).slice(0, 30), staged))}`);
    for (const line of chatter.slice(0, 8)) say(`   expand :: ${line}`);
    const execute = page.locator('[data-slot="window-action-pane"] button').filter({ hasText: /execute|ausführen|apply|add|run/iu }).first();
    say(`execute controls: ${await page.locator('[data-slot="window-action-pane"] button').count()}; match ${await execute.count()}`);
    if ((await execute.count()) > 0) {
      chatter.length = 0;
      const bodyBefore = await page.evaluate(() => ([...document.querySelectorAll('[data-slot="window-body"]')].map((e) => (e.textContent ?? "").replace(/\s+/gu, " ")).join(" || ").slice(0, 400)));
      await execute.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(8_000);
      say(`after execute: engagement ${JSON.stringify(await engagement())}; undo ${await historyRow("action.undo")}`);
      say(`window body: ${JSON.stringify(bodyBefore)} -> ${JSON.stringify(await page.evaluate(() => ([...document.querySelectorAll('[data-slot="window-body"]')].map((e) => (e.textContent ?? "").replace(/\s+/gu, " ")).join(" || ").slice(0, 400))))}`);
      for (const line of chatter.slice(0, 16)) say(`   execute :: ${line}`);
      // ⏪️ The undo/redo round trip, by the shell's own reserved chords.
      await page.keyboard.press("Meta+KeyZ");
      await page.waitForTimeout(4_000);
      say(`after undo: engagement ${JSON.stringify(await engagement())}`);
      await page.keyboard.press("Meta+Shift+KeyZ");
      await page.waitForTimeout(4_000);
      say(`after redo: engagement ${JSON.stringify(await engagement())}`);
    }
  }
  await shot(`spawned-${plugin}-after-verbs`);

  say("=== faults ===");
  for (const line of [...new Set(faults)].slice(0, 15)) say(`   ${line}`);
} finally {
  await browser.close();
}
