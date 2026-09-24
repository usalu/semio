/** 🔎️ C10 zero-touch probe: a fresh browser context on each serve must end up signed in with NO manual step. */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const urls = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
for (const url of urls) {
  const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
  const started = Date.now();
  await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180000 });
  let state = null;
  for (let i = 0; i < 120; i += 1) {
    state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), signIn: document.querySelectorAll('[data-semio-hub-sign-in=""]').length, badge: (document.querySelector('[data-semio-hub-session]')?.getAttribute("data-semio-hub-session")) ?? null, footer: document.body.innerText.match(/signed (in|out)|angemeldet|abgemeldet/gi)?.slice(0, 3) ?? null }));
    if (state.ready && state.signIn === 0 && i > 3) break;
    await page.waitForTimeout(1000);
  }
  console.log(url, JSON.stringify({ ...state, ms: Date.now() - started }));
}
await browser.close();
