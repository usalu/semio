/** 🏗️ Slice S4 — finds how a human actually reaches `createStudio` (the local, hub-free studio) inside
 * the signed-in `s` host: the command palette's full item list, the window's own action controls, and
 * the Actions panel — then dispatches it and reports where the shell went.
 *
 * Usage: bun 🐍️s4-create-studio-diagnose.mjs [uiOrigin]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const ACCOUNT = { email: process.env.S2_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S2_SIGN_IN_PASSWORD ?? "collab e2e first human phrase" };
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const say = (...parts) => console.log("[s4]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const faults = [];
page.on("console", (message) => { const text = message.text(); if (/dropped|refus|reject|not-ui-safe|typed-operation|fault/iu.test(text)) faults.push(text.slice(0, 400)); });
page.on("pageerror", (error) => faults.push(`pageerror ${String(error).slice(0, 300)}`));
const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s4-studio-${name}.png`, import.meta.url)) }).catch(() => undefined);
const clickThrough = async (locator, budgetMs) => {
  const deadline = Date.now() + budgetMs;
  for (;;) {
    try { await locator.click({ timeout: 4_000 }); return true; }
    catch (error) { if (Date.now() >= deadline) { say(`  click gave up: ${String(error).slice(0, 120)}`); return false; } await page.keyboard.press("Escape").catch(() => undefined); await page.waitForTimeout(300); }
  }
};

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null, undefined, { timeout: 300_000 }).catch(() => say("home never published"));
  await clickThrough(page.locator('[data-semio-hub-sign-in=""]').first(), 120_000);
  const workspace = page.locator("[data-semio-hub-workspace]");
  await workspace.waitFor({ state: "visible", timeout: 60_000 });
  await workspace.locator('input[type="email"]').fill(ACCOUNT.email);
  await workspace.locator('input[type="password"]').fill(ACCOUNT.password);
  await clickThrough(workspace.locator('button[type="submit"][aria-label="Sign in"]'), 60_000);
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForTimeout(SETTLE / 4);
  say(`route=${await page.evaluate(() => location.pathname)} home=${await page.locator('[id="s-home-main"]').count()}`);

  say(`action controls anywhere: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-action-id]")].map((element) => `${element.getAttribute("data-action-id")}@${element.id || element.tagName}`).slice(0, 60)))}`);

  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press("Meta+p");
  await page.waitForTimeout(3_000);
  say(`palette items (unfiltered): ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[role="dialog"] [data-slot="command-item"]')].map((element) => `${element.getAttribute("data-command-item-id")}|${element.textContent?.trim()?.slice(0, 40)}`).slice(0, 80)))}`);
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.fill("Create");
  await page.waitForTimeout(1_500);
  say(`palette items for "Create": ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[role="dialog"] [data-slot="command-item"]')].map((element) => `${element.getAttribute("data-command-item-id")}|${element.textContent?.trim()?.slice(0, 40)}`).slice(0, 40)))}`);
  await page.keyboard.press("Escape");
  await page.waitForTimeout(500);

  // ⌨️ Home binds `mod+n` to createStudio (`.keybinding("mod+n", "createStudio")`).
  await page.locator('[id="s-home-main"]').first().click({ force: true, position: { x: 8, y: 8 } }).catch(() => undefined);
  await page.keyboard.press("Meta+n");
  await page.waitForTimeout(SETTLE / 3);
  say(`after Meta+n: route=${await page.evaluate(() => location.pathname)} windows=${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id"))))}`);
  say(`body: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 400))}`);
  await shot("after-meta-n");

  say("=== faults ===");
  for (const line of [...new Set(faults)].slice(0, 15)) say(`   ${line}`);
} finally {
  await browser.close();
}
