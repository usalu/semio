import { chromium } from "playwright";
import { join } from "path";
const ticketDir = import.meta.dirname;
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleMsgs = [];
const pageErrors = [];
page.on("pageerror", (err) => pageErrors.push(String(err.stack || err).slice(0, 2000)));
page.on("console", (msg) => consoleMsgs.push({ type: msg.type(), text: msg.text().slice(0, 1500) }));
await page.goto("http://127.0.0.1:6078/", { waitUntil: "domcontentloaded", timeout: 60000 });
await page.waitForTimeout(15000);
const loading = await page.getByText("Loading surface").count();
const canvas = await page.locator("canvas").count();
const out = {
  loading,
  canvas,
  pageErrors,
  allConsole: consoleMsgs.slice(0, 120),
  interesting: consoleMsgs.filter(m => /DEBUG|error|fail|wasm|plugin|surface|engine|wgpu|webgpu|lowpoly|load/i.test(m.text)).slice(0, 80),
};
await Bun.write(join(ticketDir, "🧪lowpoly-surface-probe.json"), JSON.stringify(out, null, 2));
console.log(JSON.stringify({ loading, canvas, pageErrors: pageErrors.slice(0,10), interesting: out.interesting.slice(0,40), consoleCount: consoleMsgs.length }, null, 2));
await browser.close();
