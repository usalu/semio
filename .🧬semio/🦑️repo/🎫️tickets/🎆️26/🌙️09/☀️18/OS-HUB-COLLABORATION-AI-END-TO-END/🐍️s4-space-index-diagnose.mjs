/** 🗂️ Slice S4 — inside an opened space, measures what the space index offers: the `Create Artifact`
 * catalog (the affordance the hub's `artifactAuthority` is suspected to gate), the palette's verbs,
 * and whether a `studio`-kind space reaches a different window kind than an `atelier` one.
 *
 * Usage: bun 🐍️s4-space-index-diagnose.mjs [uiOrigin]
 *   S4_SPACE_KIND=atelier|studio   which space kind to create (default studio)
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const ACCOUNT = { email: process.env.S2_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S2_SIGN_IN_PASSWORD ?? "collab e2e first human phrase" };
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const KIND = process.env.S4_SPACE_KIND ?? "studio";
const SPACE_NAME = process.env.S4_SPACE_NAME ?? `S4 ${KIND} ${Date.now() % 100000}`;
const say = (...parts) => console.log("[s4]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const net = [];
page.on("requestfinished", async (request) => {
  const url = request.url();
  if (!/\/_semio\/hub\/|:7501\//u.test(url) || /event-page/u.test(url)) return;
  const response = await request.response().catch(() => null);
  net.push(`${request.method()} ${url.replace(uiOrigin, "")} → ${response?.status() ?? "?"}`);
});
const faults = [];
page.on("console", (message) => { const text = message.text(); if (/S4PROBE|catalog|artifact|authority|refus|reject|denied|fail|dropped/iu.test(text)) faults.push(text.slice(0, 500)); });
page.on("pageerror", (error) => faults.push(`pageerror ${String(error).slice(0, 300)}`));

const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s4-index-${name}.png`, import.meta.url)) }).catch(() => undefined);
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
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => say("badge never cleared"));
  await page.waitForTimeout(6_000);

  await workspace.locator('input[name="spaceName"]').first().fill(SPACE_NAME);
  await workspace.locator("select").nth(1).selectOption(KIND);
  say(`creating ${KIND} space "${SPACE_NAME}"`);
  await clickThrough(workspace.locator('button[aria-label="Create space"]').first(), 30_000);
  await page.waitForTimeout(SETTLE / 4);
  const opens = await page.evaluate(() => [...document.querySelectorAll('[data-semio-hub-workspace] button[aria-label^="Open "]')].map((element) => element.getAttribute("aria-label")));
  say(`open affordances: ${JSON.stringify(opens)}`);
  await clickThrough(workspace.locator('button[aria-label^="Open "]').last(), 60_000);
  await page.waitForTimeout(SETTLE / 2);

  say(`route: ${await page.evaluate(() => location.pathname)}`);
  say(`ids: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[id]")].map((element) => element.id).filter((id) => /^(s-|window:|framework\.)/u.test(id)).slice(0, 40)))}`);
  say(`body: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 500))}`);
  await shot("opened");

  // 🗿️ The space index's own artifact-creation affordance — the hub authority question.
  const create = page.locator('[id="window:framework.window.table/s-space-create-artifact"], [id$="s-space-create-artifact"]').first();
  say(`create-artifact affordance: ${await create.count()}`);
  const netBefore = net.length;
  await clickThrough(create, 40_000);
  await page.waitForTimeout(SETTLE / 2);
  say(`dialogs: ${await page.locator("[role='dialog']").count()}`);
  say(`dialog text: ${JSON.stringify((await page.locator("[role='dialog']").first().innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 900))}`);
  say(`body after create-artifact: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 700))}`);
  await shot("create-artifact");
  say("=== hub calls since create-artifact ===");
  for (const row of net.slice(netBefore)) say(`   ${row}`);

  await page.keyboard.press("Escape");
  await page.waitForTimeout(2_000);
  await page.locator("[id^='window:']").first().click({ force: true }).catch(() => undefined);
  await page.keyboard.press("Meta+p");
  await page.waitForTimeout(3_000);
  say(`palette items: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[role='dialog'] [data-value], [role='dialog'] [cmdk-item]")].map((element) => element.getAttribute("data-value") ?? element.textContent?.trim()?.slice(0, 40)).slice(0, 60)))}`);

  say("=== faults ===");
  for (const line of [...new Set(faults)].slice(0, 20)) say(`   ${line}`);
} finally {
  await browser.close();
}
