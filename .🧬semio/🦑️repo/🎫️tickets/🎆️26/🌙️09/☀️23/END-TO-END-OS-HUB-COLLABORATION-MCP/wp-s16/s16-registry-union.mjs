#!/usr/bin/env bun
/** 🧮️ S15 — live proof of the registry union (audit P1-7) on a front whose build has one plugin REMOVED from every local
 * registry (`s15-unstaged-front.ts … unregister`): the palette offers no program of it, the Marketplace lists it under the
 * serving hub's trusted catalog generation (`Source: hub · <generation>`), Install acquires it from the hub (install band +
 * progress, verified into the module store), the row turns loaded, the palette spawns it and one verb → undo → redo
 * round-trips inside `s`.
 * usage: bun s15-registry-union.mjs <frontUrl> <pluginId> [--locale de] [--tag t] */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6542/";
const pluginId = argv[1] ?? "draw";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const locale = valueOf("--locale", "en");
const tag = valueOf("--tag", `${pluginId}-${locale}`);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
mkdirSync(generated, { recursive: true });
const out = `${generated}s16-registry-union-${tag}.json`;
const t0 = Date.now();
const at = () => Date.now() - t0;

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
const refusals = [];
const faults = [];
page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 240)));
page.on("response", (response) => {
  if (response.status() >= 400) faults.push(`http ${response.status()}: ${decodeURIComponent(response.url()).slice(0, 200)}`);
});
page.on("console", (message) => {
  const text = message.text();
  if (/refused|dispatch-failed|panicked|unreachable|trap/iu.test(text)) refusals.push(text.slice(0, 240));
  if (message.type() === "error") faults.push(`error: ${text}`.slice(0, 240));
});
const result = { baseUrl, pluginId, locale, tag, started: new Date().toISOString(), timeline: [] };
const flush = () => writeFileSync(out, JSON.stringify(result, null, 1));
const mark = (label, detail) => {
  result.timeline.push({ atMs: at(), label, detail });
  console.log(`[s15-union] @${at()} ${label} ${detail === undefined ? "" : JSON.stringify(detail).slice(0, 400)}`);
};

const paletteOffers = async () => {
  await sweep.openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.fill(pluginId);
  await page.waitForTimeout(1_500);
  const rows = await page.evaluate((id) => [...document.querySelectorAll('[data-slot="command-item"]')].map((row) => row.getAttribute("data-command-item-id") ?? "").filter((row) => row.startsWith(`spawn.${id}`)), pluginId);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(400);
  return rows;
};

const marketplace = () =>
  page.evaluate((id) => {
    const sections = [...document.querySelectorAll('[id^="framework.marketplace.source."]')].map((section) => ({ id: section.id, text: (section.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 80) }));
    const row = document.getElementById(`framework.marketplace.plugin.${id}`);
    const install = document.getElementById(`framework.marketplace.plugin.${id}.install`);
    const band = document.querySelector("[data-semio-plugin-install]");
    const progress = document.querySelector("[data-semio-plugin-install-progress]");
    const section = row?.closest('[id^="framework.marketplace.source."]') ?? null;
    return {
      order: [...document.querySelectorAll('[id^="framework.marketplace.source."], [id^="framework.marketplace.plugin."]')].filter((entry) => !/\.(install|reload|uninstall|enable)$/u.test(entry.id)).map((entry) => entry.id.replace("framework.marketplace.", "")),
      sections: sections.map((entry) => entry.id),
      rowSection: section?.id ?? (row ? [...document.querySelectorAll('[id^="framework.marketplace.source."]')].find((entry) => row.compareDocumentPosition(entry) & Node.DOCUMENT_POSITION_FOLLOWING)?.id ?? null : null),
      row: row ? (row.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120) : null,
      install: install ? (install.textContent ?? "").trim() : null,
      band: band ? (band.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 200) : null,
      progress: progress ? `${progress.getAttribute("value")}/${progress.getAttribute("max")}` : null,
      notice: [...document.querySelectorAll('[role="alert"], [data-semio-transient-notice]')].map((entry) => (entry.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 160)).filter(Boolean),
    };
  }, pluginId);

async function openMarketplace() {
  const tab = page.locator('[id="framework.panelTab.framework.marketplace"], [data-tab-id="framework.marketplace"]').first();
  if (await tab.count()) {
    await tab.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_200);
    if (await page.locator('[id^="framework.marketplace.source."]').count()) return "panel tab";
  }
  await sweep.openPalette(page);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.fill(locale === "de" ? "Marktplatz" : "Marketplace");
  await page.waitForTimeout(1_200);
  const item = page.locator('[data-slot="command-item"]').first();
  const itemId = (await item.count()) ? await item.getAttribute("data-command-item-id") : null;
  if (itemId) await item.click({ force: true }).catch(() => undefined);
  else await page.keyboard.press("Escape");
  await page.waitForTimeout(1_500);
  if (await page.locator('[id^="framework.marketplace.source."]').count()) return `palette ${itemId}`;
  return `not reachable (palette first row ${itemId})`;
}

try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await sweep.awaitBeacon(page, Date.now() + 300_000);
  await sweep.dismissIntroduction(page);
  await page.waitForTimeout(2_000);
  mark("booted");
  if (locale !== "en") mark("locale", await sweep.seatLocale(page, locale));
  await page.keyboard.press("Escape").catch(() => undefined);
  result.paletteBefore = await paletteOffers();
  mark("palette before install", result.paletteBefore);
  result.openedVia = await openMarketplace();
  mark("marketplace", result.openedVia);
  let listed = await marketplace();
  const listDeadline = Date.now() + 90_000;
  while (listed.row === null && Date.now() < listDeadline) {
    await page.waitForTimeout(1_000);
    listed = await marketplace();
  }
  result.listed = listed;
  mark("listed", listed);
  if (listed.install !== null) {
    await page.locator(`[id="framework.marketplace.plugin.${pluginId}.install"]`).first().click({ force: true });
    mark("install pressed");
    const deadline = Date.now() + 240_000;
    let last = "";
    const seen = [];
    while (Date.now() < deadline) {
      await page.waitForTimeout(300);
      const state = await marketplace();
      const summary = `${state.row}|${state.band}|${state.progress}`;
      if (summary !== last) {
        seen.push({ atMs: at(), row: state.row, band: state.band, progress: state.progress, notice: state.notice });
        last = summary;
      }
      if (state.band === null && state.row !== null && state.install === null && seen.length > 1) break;
      if (state.install !== null && seen.length > 1 && state.band === null) break;
    }
    result.install = seen;
    result.afterInstall = await marketplace();
    mark("install settled", result.afterInstall);
  }
  await page.screenshot({ path: `${generated}s16-registry-union-${tag}-marketplace.png` }).catch(() => undefined);
  result.paletteAfter = await paletteOffers();
  mark("palette after install", result.paletteAfter);
  const spawned = await sweep.spawnProgram(page, pluginId);
  result.spawn = spawned;
  mark("spawn", spawned);
  if (spawned.windowIds.length > 0) {
    await page.waitForTimeout(4_000);
    const mutation = await sweep.mutateUndoRedo(page, refusals, pluginId);
    result.mutation = { verb: mutation.mutation, detail: mutation.mutationDetail, edits: mutation.edits, railRows: mutation.railRows, attempts: (mutation.attempts ?? []).slice(0, 4) };
    mark("mutate/undo/redo", { verb: mutation.mutation, detail: mutation.mutationDetail, edits: mutation.edits });
  }
  await page.screenshot({ path: `${generated}s16-registry-union-${tag}-program.png` }).catch(() => undefined);
  result.pass = result.paletteBefore.length === 0 && /hub/u.test(result.listed?.rowSection ?? "") && (result.paletteAfter ?? []).length > 0 && result.mutation?.verb != null && result.mutation?.detail === null;
} catch (error) {
  result.error = String(error).slice(0, 400);
  mark("error", result.error);
} finally {
  result.faults = faults.slice(0, 12);
  result.refusals = refusals.slice(0, 12);
  result.finished = new Date().toISOString();
  flush();
  await browser.close();
}
console.log(`=== ${tag} pass=${result.pass ?? false} → ${out}`);
