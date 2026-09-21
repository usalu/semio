/** 🏛️ S10 — Home → studio, the middle leg of outcome 1, with every refusal and window id printed at
 * each step. The sweep reports only "studio spawn dispatched but no new window opened"; this names
 * what the shell said while that happened. */
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";
const SHELL = process.argv[2] ?? "http://127.0.0.1:6071/";
const SETTLE = Number(process.argv[3] ?? 70_000);
const TAG = process.argv[4] ?? "1";
const OUT = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const notes = [];
page.on("pageerror", (e) => notes.push(`pageerror: ${String(e.message).slice(0, 200)}\n${String(e.stack ?? "").split("\n").slice(1, 12).join("\n")}`));
page.on("console", (m) => { const t = m.text(); if (/refused|reject|retired|fault|closed|Uncaught/i.test(t)) notes.push(`${m.type()}: ${t.slice(0, 220)}`); });
const wins = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id")));
const say = (...p) => console.log("[s10]", ...p);
try {
  await page.goto(SHELL, { waitUntil: "commit", timeout: 300_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().waitFor({ state: "visible", timeout: 180_000 });
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const ws = page.locator("[data-semio-hub-workspace]");
  await ws.waitFor({ state: "visible", timeout: 60_000 });
  await ws.locator('input[type="email"]').fill("user1@semio.dev");
  await ws.locator('input[type="password"]').fill("gm1-local-dev-pass-1");
  await ws.locator('button[type="submit"][aria-label="Sign in"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).catch(() => undefined);
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(SETTLE);
  say(`settled windows=${JSON.stringify(await wins())} notes=${notes.length}`);
  say(`NOTES-AFTER-SIGNIN ${JSON.stringify([...new Set(notes)].slice(0, 6), null, 1)}`);
  const before = await wins();
  await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  await page.locator("[role='dialog'] [data-slot='command-input']").first().waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
  const items = await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((e) => `${e.getAttribute("data-command-item-id")}|${(e.textContent ?? "").trim().slice(0, 40)}`).slice(0, 400));
  say(`palette items ${items.length}`);
  say(`palette spawn ${JSON.stringify(items.filter((row) => /spawn\.|studio|space/iu.test(row)).slice(0, 25), null, 1)}`);
  const typed = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await typed.fill("studio").catch(() => undefined);
  await page.waitForTimeout(2_000);
  say(`after typing "studio" ${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((e) => `${e.getAttribute("data-command-item-id")}|${(e.textContent ?? "").trim().slice(0, 40)}`).slice(0, 20)), null, 1)}`);
  const item = page.locator('[data-slot="command-item"]').filter({ hasText: /s\s*·\s*studio/iu }).first();
  say(`studio entry count=${await item.count()}`);
  if ((await item.count()) > 0) await item.click({ force: true }).catch(() => undefined);
  for (let i = 0; i < 24; i += 1) {
    await page.waitForTimeout(5_000);
    const now = await wins();
    if (now.some((id) => !before.includes(id))) { say(`OPENED ${JSON.stringify(now.filter((id) => !before.includes(id)))} after ${(i + 1) * 5}s`); break; }
    if (i === 23) say(`NEVER-OPENED windows still ${JSON.stringify(now)}`);
  }
  say(`NOTES ${JSON.stringify([...new Set(notes)].slice(0, 12), null, 1)}`);
  say(`BODY ${await page.evaluate(() => document.body.innerText.replace(/\s+/gu, " ").slice(0, 400))}`);
  await page.screenshot({ path: `${OUT}s10-studio-entry-${TAG}.png` }).catch(() => undefined);
} catch (error) {
  say(`ABORTED ${String(error).slice(0, 300)}`);
} finally { await browser.close(); }
