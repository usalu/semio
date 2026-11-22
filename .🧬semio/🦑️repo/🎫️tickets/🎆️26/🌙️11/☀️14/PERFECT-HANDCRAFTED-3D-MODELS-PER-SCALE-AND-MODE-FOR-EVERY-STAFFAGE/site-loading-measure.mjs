/** ⏱️ Measures time-to-figure of staffage asset pages on a running site and lists the asset modules each page requested.
 * Usage: `node site-loading-measure.mjs <origin> <timeoutS> <category/slug>...` — prints one JSON line per page.
 * Each page opens in a fresh browser context (cold browser cache); the dev server keeps whatever it already transformed.
 * On the development server an asset module is named by its catalog folder (`assetModulesRequested`) and its findings stream by its revision (`findingsStreamsRequested`);
 * in a built site every asset is one chunk `assets/🔣️.json-<hash>.js` (`builtAssetChunksRequested`). `figureMs` is the time from navigation until the opened page holds the drawn figure of its asset. */
import { createRequire } from "node:module";

const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { chromium } = require("playwright");
const [origin, timeoutS, ...routes] = process.argv.slice(2);
const browser = await chromium.launch({ channel: "chrome", headless: true });
const slugOf = (url) => { const text = decodeURIComponent(url); const kind = text.match(/staffage-(asset|findings)/u); const slug = text.match(/\/([a-z0-9-]+)\/[^/]*\.json/u); return kind && slug ? [text, slug[1], kind[1]] : null; };
for (const route of routes) {
  const slug = route.split("/")[1];
  const context = await browser.newContext({ locale: "en-US", viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  const requested = [];
  const chunks = [];
  const findings = [];
  const finished = [];
  const notices = new Set();
  page.on("request", (request) => {
    const url = decodeURIComponent(request.url());
    const hit = slugOf(request.url());
    if (hit) requested.push(hit[1]);
    if (/\/assets\/[^/]*\.json-[^/]*\.js$/u.test(url)) chunks.push(url.split("/").at(-1));
    if (url.includes("@staffage-findings/")) findings.push(url.split("/").at(-1).slice(0, 12));
  });
  page.on("requestfinished", (request) => { const hit = slugOf(request.url()); if (hit) finished.push(hit[1]); });
  const start = Date.now();
  await page.goto(`${origin}/#/${route}`, { waitUntil: "commit" });
  const selector = `[data-page="asset:${slug}"] [data-viewer] [data-staffage-figure][data-asset="${slug}"] svg`;
  let figureMs = null;
  let planned = false;
  const poll = setInterval(async () => {
    try {
      const text = await page.evaluate(() => [...document.querySelectorAll('[data-page^="asset:"] [role="status"], [data-page^="asset:"] [data-notice]')].map((node) => node.textContent ?? "").join(" | "));
      if (/Planned, not yet authored|Geplant, noch nicht erstellt/u.test(await page.evaluate(() => document.body.innerText))) planned = true;
      if (text) notices.add(text.slice(0, 160));
    } catch {}
  }, 500);
  try {
    await page.waitForSelector(selector, { timeout: Number(timeoutS) * 1000, state: "attached" });
    figureMs = Date.now() - start;
  } catch {}
  await page.waitForTimeout(4000);
  clearInterval(poll);
  console.log(JSON.stringify({ route, figureMs, timedOutAfterS: figureMs === null ? Number(timeoutS) : undefined, assetModulesRequested: [...new Set(requested)], builtAssetChunksRequested: [...new Set(chunks)], findingsStreamsRequested: [...new Set(findings)], finishedBeforeEnd: [...new Set(finished)].length, plannedNoticeSeenWhileLoading: planned, statuses: [...notices].slice(0, 6) }));
  await context.close();
}
await browser.close();
