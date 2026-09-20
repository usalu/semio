/** 🏗️ Slice S4 — creates a space through the hub workspace overlay (the only "Create a space" form the
 * signed-in shell actually exposes) and reports, step by step, what the hub answered and what the
 * shell then showed. Dumps the overlay's own form markup first so the next author can see the real
 * field identities instead of guessing selectors.
 *
 * Usage: bun 🐍️s4-space-create-diagnose.mjs [uiOrigin]
 */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";

const uiOrigin = process.argv[2] ?? "http://127.0.0.1:6071";
const ACCOUNT = { email: process.env.S2_SIGN_IN_EMAIL ?? "user1@semio.dev", password: process.env.S2_SIGN_IN_PASSWORD ?? "collab e2e first human phrase" };
const SETTLE = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 60_000);
const SPACE_NAME = process.env.S4_SPACE_NAME ?? `S4 Studio ${Date.now() % 100000}`;
const say = (...parts) => console.log("[s4]", ...parts);

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const net = [];
page.on("requestfinished", async (request) => {
  const url = request.url();
  if (!/\/_semio\/hub\/|:7501\//u.test(url)) return;
  const response = await request.response().catch(() => null);
  net.push(`${request.method()} ${url.replace(uiOrigin, "")} → ${response?.status() ?? "?"}`);
});
const faults = [];
page.on("console", (message) => { const text = message.text(); if (/S4PROBE|fail|fault|refus|reject|error|denied|unauthor|forbidden|authority/iu.test(text)) faults.push(`${message.type()} ${text.slice(0, 900)}`); });
page.on("pageerror", (error) => faults.push(`pageerror ${String(error).slice(0, 350)}`));

const shot = (name) => page.screenshot({ path: fileURLToPath(new URL(`./🗑️generated/s4-create-${name}.png`, import.meta.url)) }).catch(() => undefined);
const clickThrough = async (locator, budgetMs) => {
  const deadline = Date.now() + budgetMs;
  for (;;) {
    try { await locator.click({ timeout: 4_000 }); return true; }
    catch (error) { if (Date.now() >= deadline) { say(`click gave up: ${String(error).slice(0, 140)}`); return false; } await page.keyboard.press("Escape").catch(() => undefined); await page.waitForTimeout(300); }
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
  say("signed in, overlay open");

  // 🔍️ The overlay's own create form, as the DOM really spells it.
  say(`form fields: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] input, [data-semio-hub-workspace] select, [data-semio-hub-workspace] textarea")].map((element) => ({ tag: element.tagName, type: element.getAttribute("type"), name: element.getAttribute("name"), label: element.getAttribute("aria-label"), placeholder: element.getAttribute("placeholder"), id: element.id }))), null, 1)}`);
  say(`buttons: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] button")].map((element) => ({ label: element.getAttribute("aria-label"), text: element.textContent?.trim()?.slice(0, 40), disabled: element.disabled }))), null, 1)}`);
  await shot("form");

  const createButton = workspace.locator('button[aria-label="Create space"]').first();
  const nameField = workspace.locator('input[name="spaceName"]').first();
  say(`name field placeholder: ${JSON.stringify(await nameField.getAttribute("placeholder").catch(() => null))}`);
  await nameField.fill(SPACE_NAME).catch((error) => say(`fill failed: ${String(error).slice(0, 140)}`));
  say(`filled name: ${JSON.stringify(await nameField.inputValue().catch(() => null))}, create disabled=${await createButton.isDisabled().catch(() => "?")}`);
  await page.waitForTimeout(1_000);
  say(`after typing: create disabled=${await createButton.isDisabled().catch(() => "?")}`);
  const kind = workspace.locator("select").nth(1);
  const visibility = workspace.locator("select").nth(2);
  say(`kind options: ${JSON.stringify(await kind.locator("option").allTextContents().catch(() => []))} value=${await kind.inputValue().catch(() => null)}`);
  say(`visibility options: ${JSON.stringify(await visibility.locator("option").allTextContents().catch(() => []))} value=${await visibility.inputValue().catch(() => null)}`);
  const before = net.length;
  await clickThrough(createButton, 30_000);
  await page.waitForTimeout(SETTLE / 3);

  say("=== hub calls since the create click ===");
  for (const row of net.slice(before)) say(`   ${row}`);
  say(`workspace text after create: ${JSON.stringify((await workspace.first().innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 900))}`);
  await shot("after-create");

  say(`spaces list entries: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] [data-space-id], [data-semio-hub-workspace] li, [data-semio-hub-workspace] [role='option']")].map((element) => `${element.getAttribute("data-space-id") ?? ""}|${element.textContent?.trim()?.slice(0, 60)}`).slice(0, 20)))}`);
  say(`buttons now: ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] button")].map((element) => element.getAttribute("aria-label") ?? element.textContent?.trim()?.slice(0, 40)).slice(0, 30)))}`);

  // 🌐️ What the hub itself holds, independent of what the shell painted.
  say(`hub /directory/spaces (unauthenticated): ${JSON.stringify(await page.evaluate(async () => { const response = await fetch("/_semio/hub/directory/spaces"); return { status: response.status, body: (await response.text()).slice(0, 400) }; }))}`);

  await page.goBack({ waitUntil: "commit" }).catch(() => undefined);
  await page.waitForTimeout(SETTLE / 3);
  say(`home after create: rows=${await page.locator('[data-row-id^="space:"]').count()} allRows=${await page.locator("[data-row-id]").count()}`);
  say(`home body: ${JSON.stringify((await page.locator("body").innerText().catch(() => "")).replace(/\s+/gu, " ").slice(0, 500))}`);
  await shot("home");

  say("=== faults ===");
  for (const line of [...new Set(faults)].slice(0, 25)) say(`   ${line}`);
} finally {
  await browser.close();
}
