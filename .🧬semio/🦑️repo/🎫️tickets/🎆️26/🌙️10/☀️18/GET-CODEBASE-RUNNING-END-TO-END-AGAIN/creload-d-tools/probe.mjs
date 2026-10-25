// Ticket input script: after a reload, set the address at once and report who rewrites it (dev site on port 6061).
//   node probe.mjs <dir that resolves playwright> [delayMs]
import { createRequire } from "node:module";
const require = createRequire(process.argv[2] + "/");
const { chromium } = require("playwright");
const delay = Number(process.argv[3] ?? 0);
const base = "http://127.0.0.1:6061";
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
await context.addInitScript(() => {
  window.__log = [];
  const note = (what, extra) => window.__log.push([Math.round(performance.now()), what, location.hash, extra ?? ""]);
  for (const name of ["replaceState", "pushState"]) {
    const original = history[name].bind(history);
    history[name] = (...args) => { note(name, `${args[2]} ${new Error().stack.split("\n").slice(2, 5).map((l) => l.trim().replace(/\(.*\/(.*)\)/, "($1)")).join(" < ")}`); return original(...args); };
  }
  window.addEventListener("hashchange", () => note("hashchange"));
  new MutationObserver(() => { if (!window.__main && document.getElementById("course-main")) { window.__main = true; note("course-main attached"); } }).observe(document, { childList: true, subtree: true });
});
const page = await context.newPage();
page.on("pageerror", (e) => console.log("[pageerror]", String(e).slice(0, 300)));
await page.goto(base);
await page.waitForSelector('[data-slot="introduction-info-box"]', { timeout: 120000 });
await page.locator('[data-slot="introduction-close"]').click();
const overview = page.locator('[data-course-grid][data-grid-active] > [data-layered-overview]');
await overview.locator('[data-layered-card="learner"]').locator('[data-overview-card-action="primary"]').click();
const dialog = page.getByRole("dialog").filter({ has: page.locator('input[type="radio"][value="anonymous"]') });
await dialog.locator('input[type="radio"][value="anonymous"]').check();
await dialog.locator('input[type="text"]').fill("Probe " + Math.random().toString(36).slice(2, 8));
await dialog.locator('[data-overview-card-action="primary"]').click();
await page.waitForSelector('[data-course-grid][data-grid-active] > [data-layered-overview]');
await page.waitForTimeout(1500);
await page.reload();
if (delay > 0) await page.waitForTimeout(delay);
console.log("reloaded; main attached:", await page.evaluate(() => !!document.getElementById("course-main")), "waiting card:", await page.evaluate(() => !!document.querySelector('[data-card="waiting"]')));
await page.evaluate(() => { window.__log.push([Math.round(performance.now()), "SET energy", location.hash]); window.location.hash = "energy"; window.__log.push([Math.round(performance.now()), "SET done", location.hash]); });
await page.waitForTimeout(6000);
console.log("hash:", await page.evaluate(() => location.hash || "(none)"), "energy active:", await page.locator('[data-course-grid="energy"][data-grid-active]').count());
console.log((await page.evaluate(() => window.__log)).map((l) => l.join(" | ")).join("\n"));
await browser.close();
