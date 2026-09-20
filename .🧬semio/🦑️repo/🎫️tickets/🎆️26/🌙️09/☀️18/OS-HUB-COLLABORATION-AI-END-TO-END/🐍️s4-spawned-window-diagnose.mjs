/** 🪟️ Slice S4 — after a foreign program is spawned into the `s` host, the canvas says "Drag windows
 * from Display in the navbar, or restore a saved layout". This measures what the shell then offers:
 * the palette's `window.*` items and the Display menu's entries, so the next step is driven by a real
 * affordance rather than a guess.
 *
 * Usage: bun 🐍️s4-spawned-window-diagnose.mjs [uiOrigin] [pluginId]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const plugin = process.argv[3] ?? "draw";
const ACCOUNT = { email: process.env.S2_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S2_SIGN_IN_PASSWORD ?? "collab e2e first human phrase" };
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const say = (...parts) => console.log("[s4]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const faults = [];
page.on("console", (message) => { const text = message.text(); if (/dropped|refus|reject|not-ui-safe|fault|layout/iu.test(text)) faults.push(text.slice(0, 350)); });
const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s4-spawned-${name}.png`, import.meta.url)) }).catch(() => undefined);
const clickThrough = async (locator, budgetMs) => {
  const deadline = Date.now() + budgetMs;
  for (;;) {
    try { await locator.click({ timeout: 4_000 }); return true; }
    catch (error) { if (Date.now() >= deadline) { say(`  click gave up: ${String(error).slice(0, 120)}`); return false; } await page.keyboard.press("Escape").catch(() => undefined); await page.waitForTimeout(300); }
  }
};
const paletteItems = () => page.evaluate(() => [...document.querySelectorAll('[role="dialog"] [data-slot="command-item"]')].map((element) => `${element.getAttribute("data-command-item-id")}|${element.textContent?.trim()?.slice(0, 44)}`));
const openPalette = async () => {
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press("Meta+p");
  await page.waitForTimeout(2_500);
};

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null, undefined, { timeout: 300_000 }).catch(() => undefined);
  await clickThrough(page.locator('[data-semio-hub-sign-in=""]').first(), 120_000);
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill(ACCOUNT.email);
  await workspace.locator('input[type="password"]').fill(ACCOUNT.password);
  await clickThrough(workspace.locator('button[type="submit"][aria-label="Sign in"]'), 60_000);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForTimeout(SETTLE / 4);
  say(`signed in; windows=${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id"))))}`);

  await openPalette();
  // 🔎️ The unfiltered palette is capped at ~20 rows, so the 148 spawnable programs only appear once
  // the input narrows them — that cap, not a missing entry, is why an unfiltered dump shows only
  // `spawn.space`.
  await page.locator("[role='dialog'] [data-slot='command-input']").first().fill(plugin);
  await page.waitForTimeout(1_500);
  const target = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${plugin}"]`).first();
  say(`spawn entry for ${plugin}: ${await target.count()}`);
  await target.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(SETTLE / 2);
  say(`after spawn: windows=${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id"))))}`);
  say(`body: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 400))}`);
  await shot("after-spawn");

  await openPalette();
  say(`palette after spawn: ${JSON.stringify((await paletteItems()).slice(0, 60))}`);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(500);

  // 🖥️ The canvas' own instruction: "Drag windows from Display in the navbar".
  const display = page.locator('[id="framework.category.display"], [aria-label="Display"], button:has-text("Display")').first();
  say(`display control: ${await display.count()}`);
  await clickThrough(display, 20_000);
  await page.waitForTimeout(3_000);
  say(`display entries: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[role="menu"] [role="menuitem"], [data-slot="panel"] button, [role="dialog"] button')].map((element) => `${element.id || ""}|${element.getAttribute("aria-label") ?? element.textContent?.trim()?.slice(0, 40)}`).slice(0, 50)))}`);
  say(`windows submenu: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id^="framework.display.windows"], [id^="framework.display.windows"] *')].map((e) => `${e.id}|${e.textContent?.trim()?.slice(0, 30)}`).slice(0, 20)))}`);
  await clickThrough(page.locator('[id="framework.display.windows"]').first(), 15_000);
  await page.waitForTimeout(3_000);
  say(`after Windows: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[role="menuitem"], [data-slot="panel"] [id], [draggable="true"]')].map((e) => `${e.id || ""}|${e.getAttribute("aria-label") ?? e.textContent?.trim()?.slice(0, 32)}`).slice(0, 40)))}`);
  say(`body: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 700))}`);
  say(`windows now: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id"))))}`);
  await shot("display");
  say(`body with display open: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 700))}`);

  say("=== faults ===");
  for (const line of [...new Set(faults)].slice(0, 12)) say(`   ${line}`);
} finally {
  await browser.close();
}
