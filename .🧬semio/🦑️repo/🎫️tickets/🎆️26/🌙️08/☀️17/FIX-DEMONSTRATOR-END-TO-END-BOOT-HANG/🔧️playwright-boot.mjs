import { chromium } from "playwright";

const url = process.env.DEMO_URL ?? "http://127.0.0.1:6029/";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const logs = [];
page.on("console", (msg) => logs.push({ type: msg.type(), text: msg.text() }));
page.on("pageerror", (err) => logs.push({ type: "pageerror", text: String(err) }));

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(25_000);

const bootErrors = logs.filter((l) => l.text.includes("resolvePlaygroundBoot") && l.text.includes("not installed"));
const bootFailed = logs.filter((l) => l.text.includes("framework os boot failed") || l.text.includes("does not resolve"));
const wireErrors = logs.filter((l) => l.text.includes("parseBackboneWorkerWire") || (l.type === "pageerror" && l.text.includes("length")));
const workers = logs.filter((l) => l.text.includes("plugin worker +"));
const deferrals = logs.filter((l) => l.text.includes("establishPrimarySession deferring"));
const bodyText = await page.locator("body").innerText().catch(() => "");
const canvasCount = await page.locator("canvas").count().catch(() => 0);

const result = {
  url,
  bootErrorCount: bootErrors.length,
  bootFailedCount: bootFailed.length,
  bootFailed: bootFailed.slice(0, 8).map((l) => l.text.slice(0, 240)),
  wireErrorCount: wireErrors.length,
  workerCount: workers.length,
  workers: workers.map((l) => l.text),
  deferrals: deferrals.map((l) => l.text),
  pageErrorCount: logs.filter((l) => l.type === "pageerror").length,
  pageErrors: logs.filter((l) => l.type === "pageerror").slice(0, 8).map((l) => l.text.slice(0, 240)),
  bodyPreview: bodyText.slice(0, 500),
  canvasCount,
  debugSample: logs.filter((l) => l.text.includes("[DEBUG]")).slice(0, 40).map((l) => l.text.slice(0, 200)),
};
console.log(JSON.stringify(result, null, 2));
await browser.close();
const ok = bootErrors.length === 0 && bootFailed.length === 0 && result.pageErrorCount === 0 && workers.length > 2;
process.exit(ok ? 0 : 1);
