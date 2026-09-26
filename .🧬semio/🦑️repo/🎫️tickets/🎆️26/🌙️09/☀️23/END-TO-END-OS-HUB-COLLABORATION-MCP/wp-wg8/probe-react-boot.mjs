/** 🔎️ WG8: boots the React `s` serve once and reports the shell's hub affordances (no sign-in). */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const url = process.argv[2] ?? "http://127.0.0.1:6590/";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const started = Date.now();
const errors = [];
page.on("pageerror", (error) => errors.push(String(error).slice(0, 300)));
try {
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 300_000 });
  const state = await page.evaluate(() => ({
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    signIn: document.querySelectorAll('[data-semio-hub-sign-in=""]').length,
    hub: document.querySelector("[data-semio-hub-connection]")?.getAttribute("data-semio-hub-connection") ?? null,
    presence: !!document.querySelector('[id="s-presence-peers"]'),
  }));
  console.log(JSON.stringify({ bootMs: Date.now() - started, ...state, errors }));
} catch (error) {
  console.log(JSON.stringify({ bootMs: Date.now() - started, failed: String(error).slice(0, 300), errors }));
} finally {
  await browser.close();
}
