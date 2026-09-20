#!/usr/bin/env bun
/** 🎛️ S7 — does the `s` command palette really offer no `spawn.<plugin>` entry for three kinds?
 *
 * S6 §3.4 recorded `demonstrator`, `mathematical` and `playbook-module-procedural` as having "no
 * palette entry at all", measured by TYPING the plugin id into the palette and looking for
 * `[data-command-item-id="spawn.<id>"]`. The palette filters on the item's LABEL
 * (`Open <breadcrumb>`), not on its id, so a plugin whose breadcrumb does not contain its own id is
 * invisible to that query whether or not the entry exists. This dumps every `spawn.*` item with the
 * query EMPTY, which is the only reading that can tell the two apart.
 *
 * 🏠️ It also answers the Home question DB1 raised: how many space rows does Home list once signed in.
 *
 * Usage: bun 🐍️s7-palette-census.mjs <baseUrl>
 */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6071/";
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const log = (...parts) => console.log("[s7pal]", ...parts);

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
  await page.waitForTimeout(6_000);
  return null;
}
const paletteItems = (page) =>
  page.evaluate(() =>
    [...document.querySelectorAll("[data-slot='command-item']")].map((element) => ({
      id: element.getAttribute("data-command-item-id"),
      label: (element.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 80),
    })),
  );
const homeRows = (page) =>
  page.evaluate(() => ({
    treeRows: [...document.querySelectorAll('[role="treeitem"]')].map((element) => (element.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)).slice(0, 40),
    tableRows: [...document.querySelectorAll('[data-slot="table-row"], [role="row"]')].map((element) => (element.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 60)).slice(0, 40),
  }));

const browser = await chromium.launch({ headless: process.env.S7_HEADED !== "1", args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const result = { baseUrl };
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  result.beacon = await awaitBeacon(page, Date.now() + 300_000);
  if (process.env.S7_SIGN_IN_EMAIL) result.signIn = await signIn(page, process.env.S7_SIGN_IN_EMAIL, process.env.S7_SIGN_IN_PASSWORD ?? "");
  result.home = await homeRows(page);
  await openPalette(page);
  await page.waitForTimeout(1_500);
  const all = await paletteItems(page);
  result.itemCount = all.length;
  result.spawnItems = all.filter((entry) => (entry.id ?? "").startsWith("spawn."));
  // 🔎️ The same reading the S6 sweep took, for the three kinds it scored as absent.
  result.typedQuery = {};
  for (const pluginId of [process.env.S7_PALETTE_PREFIX ?? "Spawn", "demonstrator", "mathematical", "playbook-module-procedural", "draw"]) {
    await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(pluginId);
    await page.waitForTimeout(800);
    const shown = await paletteItems(page);
    result.typedQuery[pluginId] = { shown: shown.length, spawn: shown.filter((entry) => (entry.id ?? "").startsWith("spawn.")).map((entry) => `${entry.id} | ${entry.label}`) };
  }
  await page.screenshot({ path: `${generated}s7-palette.png` });
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  await browser.close();
}
writeFileSync(`${generated}s7-palette-census.txt`, JSON.stringify(result, null, 2));
log(`palette items ${result.itemCount}; spawn entries ${result.spawnItems?.length}`);
for (const entry of result.spawnItems ?? []) log(`  ${entry.id} | ${entry.label}`);
for (const [pluginId, reading] of Object.entries(result.typedQuery ?? {})) log(`typed "${pluginId}" → ${reading.shown} item(s), spawn: ${JSON.stringify(reading.spawn)}`);
log(`home tree rows ${JSON.stringify(result.home?.treeRows?.slice(0, 12))}`);
process.exit(result.fatal ? 1 : 0);
