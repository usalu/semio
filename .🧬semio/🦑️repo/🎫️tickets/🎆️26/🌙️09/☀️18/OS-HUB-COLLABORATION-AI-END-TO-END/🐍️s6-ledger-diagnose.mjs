#!/usr/bin/env bun
/** 🔬️ S6 — what witness does the REAL `s` host actually offer for a spawned program's mutation?
 *
 * The single-plugin playground probes on this ticket (`🐍️b3d-interaction-probe.mjs`) judge a mutation by
 * two things: the framework History panel's applied ledger rows and the uncommitted-edit count the
 * check-in button prints as `Check in (N)`. Inside `s` the button reads `Check In` with no count at all,
 * so half of that witness is missing and the sweep scored every kind `mutated: false`.
 *
 * This dumps everything a mutation could move, before the verb / after the verb / after undo / after
 * redo, so the sweep's oracle is chosen by measurement instead of by analogy.
 *
 * Usage: bun 🐍️s6-ledger-diagnose.mjs <baseUrl> <pluginId> <verbId>
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6071/";
const pluginId = process.argv[3] ?? "draw";
const verbId = process.argv[4] ?? "addLayer";
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const log = (...parts) => console.log("[s6led]", ...parts);

const readShell = (page) =>
  page.evaluate(() => {
    const text = (element) => (element?.innerText ?? "").replace(/\s+/gu, " ").trim();
    return {
      historyEntries: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => `${element.id}:${text(element).slice(0, 50)}`),
      historyPanelIds: [...document.querySelectorAll('[id^="framework.history"]')].map((element) => element.id).slice(0, 20),
      checkin: text(document.querySelector("#s-checkin")),
      checkinIds: [...document.querySelectorAll('[id*="checkin" i], [id*="checkIn"]')].map((element) => `${element.id}:${text(element).slice(0, 30)}`).slice(0, 10),
      panes: [...document.querySelectorAll("[data-surface-id]")].map((element) => `${element.getAttribute("data-surface-id")}:${text(element).length}:${element.querySelectorAll("svg *").length}:${element.querySelectorAll("canvas").length}:${element.querySelectorAll("*").length}`),
      measures: [...document.querySelectorAll('[data-slot="window-measures-body"], [data-slot="window-measure-tree-row"]')].map((element) => text(element).slice(0, 60)),
      treeRows: [...document.querySelectorAll('[role="treeitem"]')].filter((element) => !element.closest('[data-slot="window-action-pane"]')).map((element) => text(element).slice(0, 50)).slice(0, 25),
      undoRow: (() => {
        const row = document.querySelector('[data-slot="window-action-pane"] [id="action.undo"]');
        return row === null ? null : `${text(row)}|aria-disabled=${row.getAttribute("aria-disabled")}|data-disabled=${row.getAttribute("data-disabled")}`;
      })(),
      openPanels: [...document.querySelectorAll('[data-slot="panel"]')].filter((element) => element instanceof HTMLElement && element.offsetParent !== null).map((element) => element.id),
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

const browser = await chromium.launch({ headless: process.env.S6_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const refusals = [];
page.on("console", (message) => {
  const text = message.text();
  if (/refused:|dropped action|rejected/iu.test(text)) refusals.push(text.slice(0, 240));
});
const result = { baseUrl, pluginId, verbId, steps: [] };
const snap = async (label) => {
  const shell = await readShell(page);
  result.steps.push({ label, ...shell });
  log(`${label}: history=${shell.historyEntries.length} checkin=${JSON.stringify(shell.checkin)} measures=${JSON.stringify(shell.measures)} panes=${JSON.stringify(shell.panes)}`);
  return shell;
};
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  if (process.env.S6_SIGN_IN_EMAIL) result.signIn = await signIn(page, process.env.S6_SIGN_IN_EMAIL, process.env.S6_SIGN_IN_PASSWORD ?? "");
  result.studio = await enterStudio(page);
  result.historyPanel = await click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_500);
  result.windowIds = await spawnProgram(page, pluginId);
  log(`spawned ${JSON.stringify(result.windowIds)}`);
  const toggles = page.locator('[id$=".engagement.toggle"]');
  const toggleCount = await toggles.count();
  for (let index = 0; index < toggleCount; index += 1) await toggles.nth(index).click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_500);
  await snap("before");
  result.rowClick = await click(page, `[data-slot="window-action-pane"] [id="action.${verbId}"]`);
  await page.waitForTimeout(1_200);
  result.executeClick = await click(page, `[data-slot="window-action-pane"] [id$=".action.${verbId}.execute"]`);
  await page.waitForTimeout(4_000);
  await snap("after-verb");
  await page.waitForTimeout(4_000);
  await snap("after-verb-settled");
  result.undoClick = await click(page, '[data-slot="window-action-pane"] [id="action.undo"]');
  await page.waitForTimeout(4_000);
  await snap("after-undo");
  result.redoClick = await click(page, '[data-slot="window-action-pane"] [id="action.redo"]');
  await page.waitForTimeout(4_000);
  await snap("after-redo");
  await page.screenshot({ path: `${generated}s6-ledger-${pluginId}.png` });
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.refusals = [...new Set(refusals)].slice(0, 20);
  await browser.close();
}
writeFileSync(`${generated}s6-ledger-${pluginId}.txt`, JSON.stringify(result, null, 2));
log(`clicks row=${result.rowClick} execute=${result.executeClick} undo=${result.undoClick} redo=${result.redoClick}`);
log(`refusals ${JSON.stringify(result.refusals.slice(0, 3))}`);
process.exit(result.fatal ? 1 : 0);
