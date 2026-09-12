/** 🔎 Which flow-core modules the dev serve actually hands the browser, and whether their bytes are
 * the ones on disk. Written for the canvas-paint lane: a staged `🖥️flow-host.js` that never reaches
 * the page looks exactly like a painter that does not work.
 * Usage: cd <ticket> && bun 🐍️flow-module-freshness-probe.mjs
 */
import { chromium } from "playwright";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const marker = process.env.SEMIO_PROBE_MARKER ?? "replayFlowDrawList";
const filter = new RegExp(process.env.SEMIO_PROBE_FILTER ?? "flow", "i");
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 25);

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const seen = new Map();
page.on("response", (response) => {
  const href = response.url();
  if (filter.test(decodeURIComponent(href)) && !href.endsWith(".wasm")) seen.set(href, response);
});
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);

for (const [href, response] of seen) {
  let body = "";
  try { body = await response.text(); } catch { body = ""; }
  console.log(`[DEBUG] ${response.status()} ${body.length}B marker=${body.includes(marker)} ${decodeURIComponent(href)}`);
}
console.log("DONE modules", seen.size);
await browser.close();
