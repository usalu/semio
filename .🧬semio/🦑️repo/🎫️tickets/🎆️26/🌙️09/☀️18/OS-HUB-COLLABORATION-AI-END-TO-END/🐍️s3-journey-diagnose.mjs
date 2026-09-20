/** 🧭️ Slice S3 — measures the real Home → space → studio → palette journey inside `s`, one step at a
 * time, so the foreign-kind probe is written against what the shell actually exposes. The shipped
 * `🎬️studio/🟦️.ts` opens the palette with `Meta+p` after clicking `.semio-node-graph-host`, i.e. from
 * inside a STUDIO — Home has no node graph, which is why `Meta+K` from Home found nothing.
 *
 * Usage: bun 🐍️s3-journey-diagnose.mjs [uiOrigin]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const ACCOUNT = { email: "user1@semio.dev", password: "collab e2e first human phrase" };
const say = (...parts) => console.log("[s3]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("pageerror", (error) => say(`pageerror ${String(error).slice(0, 200)}`));

const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s3-journey-${name}.png`, import.meta.url)) }).catch(() => undefined);
const text = async () => (await page.locator("body").innerText().catch(() => "")).replace(/\s+/g, " ");

try {
  await page.goto(`${uiOrigin}/`, { waitUntil: "domcontentloaded", timeout: 300_000 });
  await page.waitForFunction(() => document.querySelector('[id="s-home-main"]') !== null, undefined, { timeout: 300_000 });
  say("home published");

  // 🔁️ C1c's proven click ladder: a veil or a tour step can own the pointer, so retry until it lands.
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
  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) > 0) {
    await clickThrough(badge, 120_000);
    const form = page.locator("[data-semio-hub-workspace]");
    await form.waitFor({ state: "visible", timeout: 60_000 });
    await form.locator('input[type="email"]').fill(ACCOUNT.email);
    await form.locator('input[type="password"]').fill(ACCOUNT.password);
    await clickThrough(form.locator('button[type="submit"][aria-label="Sign in"]'), 60_000);
    await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 });
    await clickThrough(page.locator("[data-semio-hub-workspace] button[aria-label]").first(), 60_000);
  }
  await page.waitForTimeout(12_000);
  say(`signed in; homeSurface=${await page.locator('[id="s-home-main"]').count()} route=${await page.evaluate(() => location.pathname)}`);
  await shot("signed-in");

  say(`workspace buttons: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] button")].map((element) => element.getAttribute("aria-label") ?? element.textContent?.trim()).slice(0, 25)))}`);
  say(`create button while overlay open: ${await page.locator('[id="s-home-create-space"]').count()}`);
  // 🏠️ `applyShellUri` closes the workspace for any path that is not `/hub`, so popping the history
  // entry the workspace pushed is the shell's own way back to the landing route — no reload, session kept.
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForTimeout(10_000);
  say(`after back: route=${await page.evaluate(() => location.pathname)} homeSurface=${await page.locator('[id="s-home-main"]').count()} overlay=${await page.locator("[data-semio-hub-workspace]").count()}`);
  say(`create button: ${await page.locator('[id="s-home-create-space"]').count()}`);
  say(`window-body ids: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[id="s-home-main"] [id]')].map((element) => element.id).slice(0, 30)))}`);
  // 🏗️ The overlay's own "Create space" is the hub-side path to a studio; Home's button sits under the
  // engagement rail and refuses a synthetic click. Reopen the workspace, create, then go back.
  await clickThrough(page.locator('[id="framework.hub.signIn"], [data-hub-connection], [id*="hubConnection"]').first(), 30_000).catch(() => undefined);
  await page.waitForTimeout(4_000);
  const ws = page.locator("[data-semio-hub-workspace]");
  say(`workspace reopened: ${await ws.count()}`);
  if ((await ws.count()) > 0) {
    const nameField = ws.locator("input").filter({ hasNot: page.locator('[type="email"], [type="password"]') }).first();
    say(`ws inputs: ${await ws.locator("input").count()}`);
    await nameField.fill("S3 Foreign Kind Studio").catch(() => undefined);
    await clickThrough(ws.locator("button", { hasText: /^create space$/i }).last(), 30_000).catch((error) => say(`ws create failed: ${String(error).slice(0, 100)}`));
    await page.waitForTimeout(15_000);
    say(`ws body after create: ${(await text()).slice(0, 300)}`);
  }
  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForTimeout(12_000);
  say(`rows on home: ${await page.locator('[data-row-id^="space:"]').count()}`);
  const createByText = page.locator('[id="s-home-main"] button', { hasText: /create space/i }).first();
  say(`create-by-text count: ${await createByText.count()}`);
  await clickThrough(createByText, 30_000).catch((error) => say(`create click failed: ${String(error).slice(0, 120)}`));
  await page.waitForTimeout(5_000);
  await shot("create-dialog");
  say(`dialogs=${await page.locator("[role='dialog']").count()} inputs=${await page.locator("[role='dialog'] input").count()}`);
  say(`dialog buttons: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[role='dialog'] button")].map((element) => element.getAttribute("aria-label") ?? element.textContent?.trim()).slice(0, 20)))}`);
  const nameInput = page.locator("[role='dialog'] input[type='text'], [role='dialog'] input:not([type])").first();
  if ((await nameInput.count()) > 0) {
    await nameInput.fill("S3 Foreign Kind Studio");
    const submit = page.locator("[role='dialog'] button", { hasText: /create space|create/i }).last();
    await clickThrough(submit, 30_000).catch(() => undefined);
    await page.waitForTimeout(12_000);
  }
  say(`rows after create: ${await page.locator('[data-row-id^="space:"]').count()}`);
  await shot("after-create");
  const open = page.locator('[data-row-id^="space:"] button').first();
  if ((await open.count()) > 0) {
    await clickThrough(open, 30_000).catch(() => undefined);
    await page.waitForTimeout(20_000);
  }
  say(`after open: route=${await page.evaluate(() => location.pathname)} nodeGraph=${await page.locator(".semio-node-graph-host").count()}`);
  await shot("studio");
  await page.locator(".semio-node-graph-host").first().click({ force: true }).catch(() => undefined);
  await page.keyboard.press("Meta+p");
  await page.waitForTimeout(3_000);
  say(`palette dialogs=${await page.locator("[role='dialog'] [data-slot='command-input']").count()}`);
  say(`body: ${(await text()).slice(0, 400)}`);
} finally {
  await browser.close();
}
