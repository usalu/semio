/** 🚦️ S18 §14c: the readiness beacon a fresh page raises at `<url><path>` (ready / error / not-found) and when.
 * usage: bun s18-probe-beacon.mjs <url> <path…> */
import { chromium } from "/Users/ueli/Documents/semio/node_modules/playwright/index.mjs";
const [url = "http://127.0.0.1:6540/", ...paths] = process.argv.slice(2);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
try {
  for (const path of paths.length ? paths : ["/", "/hub", "/nowhere"]) {
    const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
    const errors = [];
    page.on("pageerror", (error) => errors.push(String(error).slice(0, 200)));
    const started = Date.now();
    await page.goto(new URL(path, url).href, { waitUntil: "domcontentloaded", timeout: 240_000 });
    const beacon = await page.waitForFunction(() => { const d = document.documentElement.dataset; return d.semioOsReady !== undefined ? `ready:${d.semioOsReady}` : d.semioOsError !== undefined ? `error:${d.semioOsError}` : d.semioOsNotFound !== undefined ? `not-found:${d.semioOsNotFound}` : null; }, undefined, { timeout: 240_000, polling: 250 }).then((handle) => handle.jsonValue(), (error) => `timeout ${String(error).slice(0, 80)}`);
    await page.waitForTimeout(2_000);
    const workspace = await page.locator("[data-semio-hub-workspace]").count();
    const notFoundPage = await page.getByText(/not found|nicht gefunden/iu).count();
    console.log(path, JSON.stringify({ beacon, ms: Date.now() - started, hubWorkspace: workspace, notFoundText: notFoundPage, pageerrors: errors.length }));
    await page.context().close();
  }
} finally {
  await browser.close();
}
