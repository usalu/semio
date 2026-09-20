/** 🔬️ Slice S3 — why `s-home-main` never reaches the DOM inside the real `s` host.
 *
 * Traces ONE refresh end to end without guessing: every console line and pageerror the shell emits,
 * the shell's own catalog probe (install status + `routerFault` per plugin), the window/pane chrome
 * that DID render, and the ids actually in the DOM. Run signed out (the fault reproduces before
 * sign-in, which is the cheap half) and optionally signed in.
 *
 * Usage: bun 🐍️s3-home-surface-diagnose.mjs [uiOrigin] [--signin]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";
import { writeFileSync } from "node:fs";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const signIn = process.argv.includes("--signin");
const settleMs = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const ACCOUNT = { email: "user1@semio.dev", password: "collab e2e first human phrase" };

const lines = [];
const say = (text) => {
  console.log(text);
  lines.push(text);
};

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
const page = await context.newPage();
const consoleLines = [];
page.on("console", (message) => consoleLines.push(`${message.type()} ${message.text()}`.slice(0, 1400)));
page.on("pageerror", (error) => consoleLines.push(`PAGEERROR ${error.message}`.slice(0, 1400)));

const census = () =>
  page.evaluate(() => {
    const probe = window.__semioOsCatalogProbe;
    return {
      ready: probe?.ready ?? null,
      shellPluginId: probe?.shellPluginId ?? null,
      pluginCount: probe?.plugins?.length ?? 0,
      failed: (probe?.plugins ?? []).filter((entry) => entry.status !== "loaded").map((entry) => `${entry.pluginId}:${entry.status}`),
      routerFaults: (probe?.plugins ?? []).filter((entry) => entry.routerFault).map((entry) => `${entry.pluginId}:${entry.routerFault.code}:${entry.routerFault.message}`),
      programs: probe?.programs?.length ?? 0,
      spawned: probe?.spawned ?? [],
      homeSurface: document.querySelector('[id="s-home-main"]') !== null,
      tableHosts: document.querySelectorAll(".semio-table-host").length,
      createButton: document.querySelector('[id="s-home-create-space"]') !== null,
      ids: [...document.querySelectorAll("[id]")].map((element) => element.id).filter((id) => id.startsWith("s-") || id.startsWith("framework.")),
      windowTabs: [...document.querySelectorAll('[role="tab"]')].map((element) => element.textContent?.trim()).slice(0, 40),
      dataSlots: [...new Set([...document.querySelectorAll("[data-slot]")].map((element) => element.getAttribute("data-slot")))].slice(0, 60),
      bodyText: document.body.innerText.replace(/\s+/g, " ").slice(0, 1200),
    };
  });

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.waitForFunction(() => document.querySelector("[data-semio-os-ready]") !== null || document.body.innerText.length > 40, undefined, { timeout: 300_000 }).catch(() => undefined);
  await page.waitForTimeout(settleMs);
  const before = await census();
  say(`[before] ${JSON.stringify(before, null, 1)}`);
  await page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/s3-home-before.png", import.meta.url)) });

  if (signIn) {
    const clickThrough = async (locator, budgetMs) => {
      const deadline = Date.now() + budgetMs;
      for (;;) {
        try {
          await locator.click({ timeout: 4_000 });
          return;
        } catch (error) {
          if (Date.now() >= deadline) throw error;
          await page.keyboard.press("Escape").catch(() => undefined);
          await page.waitForTimeout(300);
        }
      }
    };
    await clickThrough(page.locator('[data-semio-hub-sign-in=""]').first(), 120_000);
    const form = page.locator("[data-semio-hub-workspace]");
    await form.waitFor({ state: "visible", timeout: 60_000 });
    await form.locator('input[type="email"]').fill(ACCOUNT.email);
    await form.locator('input[type="password"]').fill(ACCOUNT.password);
    await clickThrough(form.locator('button[type="submit"][aria-label="Sign in"]'), 60_000);
    await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 });
    await clickThrough(page.locator("[data-semio-hub-workspace] button[aria-label]").first(), 60_000);
    await page.waitForTimeout(settleMs);
    say(`[after-workspace] route=${await page.evaluate(() => location.pathname)}`);
    const after = await census();
    say(`[after-workspace-census] homeSurface=${after.homeSurface} tableHosts=${after.tableHosts}`);
    // 🏠️ Opening the hub workspace navigates a host-mode shell to `/hub`, so the census above reads a
    // shell that is no longer showing its host app at all. Returning to the landing route is what
    // makes "does Home publish for a SIGNED-IN human" answerable.
    await page.keyboard.press("Escape").catch(() => undefined);
    await page.locator('button:has-text("Home")').first().click({ timeout: 10_000 }).catch(() => undefined);
    await page.waitForTimeout(settleMs);
    say(`[home-route] route=${await page.evaluate(() => location.pathname)}`);
    const home = await census();
    say(`[home] ${JSON.stringify(home, null, 1)}`);
    await page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/s3-home-signed-in.png", import.meta.url)) });
    say(`[after] ${JSON.stringify(after, null, 1)}`);
    await page.screenshot({ path: fileURLToPath(new URL("./🗑️generated/s3-home-after.png", import.meta.url)) });
  }
} finally {
  say(`[console] ${consoleLines.length} lines`);
  for (const line of consoleLines) lines.push(line);
  writeFileSync(fileURLToPath(new URL("./🗑️generated/s3-home-diagnose.txt", import.meta.url)), `${lines.join("\n")}\n`);
  await browser.close();
}
