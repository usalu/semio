#!/usr/bin/env bun
/** 🪝️ S7 — WHICH queue is two dispatches behind, proved by an in-page hook.
 *
 * S6 §2 measured that the host's reading of a spawned program's work is two dispatches behind and
 * named three candidates: the typed-operation drain's re-arm, the per-spawned-instance completion
 * subscription S5 added, or the History/check-in projection. Reading the console buffer cannot tell
 * them apart (a buffer survives reloads — `feedback-browser-console-buffer-survives-reload`), so the
 * shell publishes a structured hook instead: `globalThis.__s7Hook` collects one record per
 * `dispatch-response`, per `spawned-completion` and per `applyHistoryPatch` admission decision, with
 * the CURSOR the projection held at that moment and whether the patch was admitted.
 *
 * Read the hook against the ledger the DOM shows after each step and the answer is unambiguous:
 *  - no `spawned-completion` record at all  → the drain re-arm or the subscription is the lag
 *  - records present but `applied:false` with `patchCursor <= currentCursor` → the PROJECTION is
 *
 * Usage: bun 🐍️s7-history-hook-probe.mjs <baseUrl> <pluginId> <verbId> [dispatches]
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6071/";
const pluginId = process.argv[3] ?? "draw";
const verbId = process.argv[4] ?? "addLayer";
const dispatches = Number(process.argv[5] ?? 4);
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const log = (...parts) => console.log("[s7hook]", ...parts);

const readHook = (page) => page.evaluate(() => (globalThis.__s7Hook ?? []).slice());
const readLedger = (page) =>
  page.evaluate(() => {
    const text = (element) => (element?.innerText ?? "").replace(/\s+/gu, " ").trim();
    const carrier = document.querySelector("[data-history-json]");
    return {
      rows: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => text(element).slice(0, 48)),
      checkin: text(document.querySelector("#s-checkin")),
      historyJson: carrier === null ? null : carrier.getAttribute("data-history-json"),
    };
  });
const windowIds = (page) => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));

async function awaitBeacon(page, deadline) {
  while (Date.now() < deadline) {
    const beacon = await page.evaluate(() => (document.documentElement.dataset.semioOsReady !== undefined ? `ready:${document.documentElement.dataset.semioOsReady}` : document.documentElement.dataset.semioOsError !== undefined ? `error:${document.documentElement.dataset.semioOsError}` : null));
    if (beacon !== null) return beacon;
    await page.waitForTimeout(1000);
  }
  return null;
}
async function dismissIntroduction(page) {
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) return;
    const skip = page.locator('[data-slot="introduction-veil"] button', { hasText: /skip/iu }).first();
    if ((await skip.count()) > 0) await skip.click({ force: true }).catch(() => undefined);
    else await page.keyboard.press("Escape").catch(() => undefined);
    await page.waitForTimeout(500);
  }
}
async function openPalette(page) {
  await dismissIntroduction(page);
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press("Meta+p");
  await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
}
async function signIn(page, email, password) {
  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) === 0) return "no sign-in badge";
  await badge.click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator("[data-semio-hub-workspace] button[aria-label]").first().click({ force: true }).catch(() => undefined);
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null, undefined, { timeout: 180_000 }).catch(() => undefined);
  await page.waitForTimeout(4_000);
  return null;
}
async function enterStudio(page) {
  await openPalette(page);
  const item = page.locator('[data-slot="command-item"]').filter({ hasText: /s\s*·\s*studio/iu }).first();
  if ((await item.count()) === 0) return "no studio palette entry";
  const before = await windowIds(page);
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + 180_000;
  while (Date.now() < deadline) {
    if ((await windowIds(page)).some((id) => !before.includes(id))) {
      await page.waitForTimeout(6_000);
      return null;
    }
    await page.waitForTimeout(500);
  }
  return "studio never opened";
}
async function spawnProgram(page, id) {
  const before = await windowIds(page);
  await openPalette(page);
  await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(id);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${id}"]`).first();
  await item.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  if ((await item.count()) === 0) return [];
  await item.click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + 60_000;
  while (Date.now() < deadline) {
    const opened = (await windowIds(page)).filter((entry) => !before.includes(entry));
    if (opened.length > 0) {
      await page.waitForTimeout(3_000);
      return opened;
    }
    await page.waitForTimeout(250);
  }
  return [];
}
const click = async (page, selector) => {
  const locator = page.locator(selector).first();
  if ((await locator.count()) === 0) return "absent";
  return locator.click({ force: true, timeout: 8_000 }).then(() => "ok").catch((error) => String(error).split("\n")[0].slice(0, 70));
};

const browser = await chromium.launch({ headless: process.env.S7_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const result = { baseUrl, pluginId, verbId, dispatches, steps: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  result.hookPresent = await page.evaluate(() => Array.isArray(globalThis.__s7Hook));
  if (process.env.S7_SIGN_IN_EMAIL) result.signIn = await signIn(page, process.env.S7_SIGN_IN_EMAIL, process.env.S7_SIGN_IN_PASSWORD ?? "");
  result.studio = await enterStudio(page);
  await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_500);
  result.windowIds = await spawnProgram(page, pluginId);
  log(`spawned ${JSON.stringify(result.windowIds)}`);
  const toggles = page.locator('[id$=".engagement.toggle"]');
  const toggleCount = await toggles.count();
  for (let index = 0; index < toggleCount; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  result.hookAtSpawn = await readHook(page);
  result.ledgerAtSpawn = await readLedger(page);
  for (let index = 0; index < dispatches; index += 1) {
    const hookBefore = (await readHook(page)).length;
    const row = await click(page, `[data-slot="window-action-pane"] [id="action.${verbId}"]`);
    await page.waitForTimeout(1_000);
    const execute = await click(page, `[data-slot="window-action-pane"] [id$=".action.${verbId}.execute"]`);
    await page.waitForTimeout(5_000);
    const hook = await readHook(page);
    const ledger = await readLedger(page);
    result.steps.push({ dispatch: index + 1, row, execute, hook: hook.slice(hookBefore), ledgerRows: ledger.rows.length, checkin: ledger.checkin, historyJson: ledger.historyJson });
    log(`dispatch ${index + 1}: rows=${ledger.rows.length} checkin=${JSON.stringify(ledger.checkin)} new-hook=${hook.length - hookBefore}`);
    for (const record of hook.slice(hookBefore)) log(`   ${JSON.stringify(record)}`);
  }
  await page.screenshot({ path: `${generated}s7-history-hook-${pluginId}.png` });
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  await browser.close();
}
writeFileSync(`${generated}s7-history-hook-${pluginId}.txt`, JSON.stringify(result, null, 2));
process.exit(result.fatal ? 1 : 0);
