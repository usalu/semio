import { chromium } from "playwright";
const [locale, w, h] = [process.argv[2] ?? "en", +(process.argv[3] ?? 1600), +(process.argv[4] ?? 1000)];
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const context = await browser.newContext({ viewport: { width: w, height: h }, locale });
const page = await context.newPage();
await page.goto(`http://127.0.0.1:6064/?plugin=draw&lang=${locale}`, { waitUntil: "domcontentloaded" });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady, null, { timeout: 120000 });
await page.waitForTimeout(4000);
const button = '[id^="framework.window."][id$=".utilityBar.unfold"]';
const hit = () => page.evaluate((sel) => {
  const b = document.querySelector(sel); const r = b.getBoundingClientRect();
  const e = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2);
  const f = [...document.querySelectorAll('[data-slot^="canvas-footer-"]')].map(x => x.getBoundingClientRect());
  return { lang: document.documentElement.lang, rect: [r.x, r.y, r.width, r.height].map(Math.round), hit: b === e || b.contains(e), footerTop: Math.min(...f.map(x => x.top)), buttonBottom: r.bottom };
}, button);
const before = await hit();
let opened = false;
try { await page.locator(button).first().click({ timeout: 8000 }); await page.waitForTimeout(400); opened = (await page.locator('[id^="framework.window."][id$=".utilityBar.fold"]').count()) > 0; } catch (e) { opened = String(e).slice(0, 120); }
console.log(JSON.stringify({ locale, viewport: [w, h], ...before, opened }));
await browser.close();
