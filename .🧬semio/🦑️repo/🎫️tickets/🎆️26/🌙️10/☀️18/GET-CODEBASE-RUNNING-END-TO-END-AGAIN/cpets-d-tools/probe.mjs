// Ticket input script: probes the page of the 200% text spec on a running dev site (port 6061) and reports who removes elements outside the pet layer.
import { createRequire } from "node:module";
const require = createRequire(process.argv[2] + "/");
const { chromium } = require("playwright");
const base = "http://127.0.0.1:6061";
const castLog = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: Number(process.env.PROBE_WIDTH ?? 320), height: 1200 } });
if (process.env.PROBE_TRACE) await context.tracing.start({ screenshots: true, snapshots: true, sources: false });
const page = await context.newPage();
if (process.env.PROBE_THROTTLE) await (await context.newCDPSession(page)).send('Emulation.setCPUThrottlingRate', { rate: Number(process.env.PROBE_THROTTLE) });
page.on("console", (m) => { if (m.type() === "error" || m.type() === "warning" || m.text().startsWith("[DEBUG]")) console.log("[console]", m.text().slice(0, 400)); });
page.on("pageerror", (e) => console.log("[pageerror]", String(e).slice(0, 400)));
await page.goto(base);
await page.waitForSelector('[data-slot="introduction-info-box"]', { timeout: 60000 });
await page.locator('[data-slot="introduction-close"]').click();
const overview = page.locator('[data-course-grid][data-grid-active] > [data-layered-overview]');
await overview.locator('[data-layered-card="learner"]').locator('[data-overview-card-action="primary"]').click();
const dialog = page.getByRole("dialog").filter({ has: page.locator('input[type="radio"][value="anonymous"]') });
await dialog.locator('input[type="radio"][value="anonymous"]').check();
await dialog.locator('[data-overview-card-action="primary"]').click();
await page.waitForSelector(".pet-layer svg.pet", { timeout: 25000 }).catch(async (e) => { await page.screenshot({ path: process.env.PROBE_SHOT ?? "/dev/null" }); console.log("layer:", await page.evaluate(() => document.querySelector(".pet-layer")?.outerHTML.slice(0, 300))); throw e; });
await page.evaluate(() => (window.location.hash = "prefs"));
if (process.env.PROBE_SCROLL) { await page.locator("[data-pets-player]").first().waitFor(); await page.locator("[data-pets-player]").first().scrollIntoViewIfNeeded(); }
await page.waitForSelector('[data-layered-pane="prefs"][data-opened]', { timeout: 30000 });
await page.evaluate(() => {
  const root = document.documentElement;
  root.style.fontSize = `${Number.parseFloat(getComputedStyle(root).fontSize) * 2}px`;
});
await page.evaluate(() => { window.__casts = []; let last = ''; setInterval(() => { const now = [...document.querySelectorAll('.pet-layer svg.pet')].map((p) => p.getAttribute('data-pet')).join(' '); if (now !== last) { window.__casts.push([Math.round(performance.now()), now]); last = now; } }, 100); });
const report = await page.evaluate(() => new Promise((resolve) => {
  const seen = new Map();
  const started = performance.now();
  let timer = setTimeout(done, 800);
  const describe = (node) => { const e = node instanceof Element ? node : node.parentElement; const chain = []; for (let n = e, i = 0; n && i < 4; n = n.parentElement, i++) chain.push(n.tagName.toLowerCase() + (n.id ? "#" + n.id : "") + (n.className && typeof n.className === "string" ? "." + n.className.split(" ").slice(0, 2).join(".") : "")); return chain.join(" < "); };
  const observer = new MutationObserver((records) => {
    let removed = false;
    for (const r of records) if (r.removedNodes.length > 0 && !(r.target instanceof Element ? r.target : r.target.parentElement)?.closest(".pet-layer")) { removed = true; const k = describe(r.target) + " | " + [...r.removedNodes].map((n) => n.nodeName).join(","); seen.set(k, (seen.get(k) ?? 0) + 1); }
    if (removed) { clearTimeout(timer); timer = setTimeout(done, 800); }
  });
  observer.observe(document.body, { childList: true, subtree: true });
  setTimeout(done, 30000);
  function done() { observer.disconnect(); resolve({ quietAfterMs: Math.round(performance.now() - started), culprits: [...seen.entries()].sort((a, b) => b[1] - a[1]).slice(0, 8) }); }
}));
console.log(JSON.stringify(report, null, 1));
console.log("casts:", JSON.stringify(await page.evaluate(() => window.__casts.slice(0, 40))));
console.log("pets:", await page.locator(".pet-layer svg.pet").evaluateAll((e) => e.map((p) => p.getAttribute("data-pet"))));
await browser.close();
