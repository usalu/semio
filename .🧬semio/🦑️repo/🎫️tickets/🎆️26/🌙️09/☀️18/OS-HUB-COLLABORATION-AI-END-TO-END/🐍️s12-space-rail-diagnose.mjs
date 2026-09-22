#!/usr/bin/env bun
/** 🎛️ S12 — why the 🪐️space index's Actions rail reads zero rows when the app is SPAWNED as a
 * program, while the same app mounted by `/spaces/{id}` renders 24. Dumps every engagement chip with
 * its pane's own `data-folded` before and after each press, so a toggle-parity race and an
 * action-pane that was never built are distinguishable. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6071/";
const tag = process.argv[3] ?? "r1";
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const log = (...p) => console.log("[s12-rail]", ...p);

const chips = (page) =>
  page.evaluate(() =>
    [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((toggle) => {
      const folded = toggle.closest("[data-folded]");
      const pane = toggle.closest('[data-slot*="engagement"], [data-slot="pane"]');
      return {
        id: toggle.id,
        foldedAncestor: folded === null ? null : folded.getAttribute("data-folded"),
        paneSlot: pane === null ? null : pane.getAttribute("data-slot"),
        ariaExpanded: toggle.getAttribute("aria-expanded"),
      };
    }),
  );
const railRows = (page) => page.evaluate(() => [...new Set([...document.querySelectorAll('[data-slot="window-action-pane"] [id^="action."]')].map((e) => e.id))].length);
const panes = (page) => page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"], [data-slot="window-engagement-overlay"], [data-slot="window-engagement-body"]')].map((e) => `${e.getAttribute("data-slot")}|folded=${e.getAttribute("data-folded")}|children=${e.querySelectorAll("*").length}`));

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.setDefaultNavigationTimeout(180_000);
const out = { baseUrl, tag, rounds: [] };
try {
  await page.goto(baseUrl, { waitUntil: "commit" });
  const dl = Date.now() + 300_000;
  while (Date.now() < dl) { if (await page.evaluate(() => document.documentElement.dataset.semioOsReady !== undefined)) break; await page.waitForTimeout(1_000); }
  for (let i = 0; i < 60; i += 1) { if ((await page.locator('[data-slot="introduction-veil"]').count()) === 0) break; await page.keyboard.press("Escape").catch(() => undefined); await page.waitForTimeout(500); }
  await page.locator('[data-semio-hub-sign-in=""]').first().click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  await form.locator('input[type="email"]').fill("user1@semio.dev").catch(() => undefined);
  await form.locator('input[type="password"]').fill("gm1-local-dev-pass-1").catch(() => undefined);
  await form.locator('button[type="submit"][aria-label="Sign in"]').click({ force: true }).catch(() => undefined);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(6_000);
  for (const [query, itemId] of [["studio", "spawn.space.s.space.studio@1/*#editor"], ["space", "spawn.space.s.space.space@1/*#editor"]]) {
    await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
    await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
    await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
    await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(query).catch(() => undefined);
    await page.waitForTimeout(2_000);
    await page.locator(`[data-slot="command-item"][data-command-item-id="${itemId}"]`).first().click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(12_000);
  }
  out.windowIds = await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id")));
  for (let round = 0; round < 4; round += 1) {
    const before = { chips: await chips(page), rows: await railRows(page), panes: await panes(page) };
    const target = before.chips[0];
    if (target !== undefined) await page.locator(`[id="${target.id}"]`).first().click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(2_500);
    out.rounds.push({ round, before, pressed: target?.id ?? null, after: { chips: await chips(page), rows: await railRows(page), panes: await panes(page) } });
    log(`round ${round}: rows ${before.rows} → ${out.rounds[round].after.rows} chips ${JSON.stringify(before.chips)}`);
  }
  await page.screenshot({ path: `${generated}s12-space-rail-${tag}.png` }).catch(() => undefined);
} catch (e) { out.fatal = String(e).slice(0, 300); log(`FATAL ${out.fatal}`); }
writeFileSync(`${generated}s12-space-rail-${tag}.txt`, JSON.stringify(out, null, 2));
log(`=== ${generated}s12-space-rail-${tag}.txt ===`);
await browser.close();
