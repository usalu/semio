import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = dirname(fileURLToPath(import.meta.url));
const URL = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const failed = [];
page.on("requestfailed", (req) => {
  failed.push({ url: req.url(), error: req.failure()?.errorText });
});
page.on("response", (res) => {
  if (res.status() >= 400) failed.push({ url: res.url(), status: res.status() });
});
await page.goto(URL, { waitUntil: "domcontentloaded", timeout: 120_000 });
await page.waitForSelector("canvas", { timeout: 120_000 }).catch(() => {});
await page.waitForTimeout(5_000);
const counts = new Map();
for (const f of failed) {
  const key = ;
  counts.set(key, (counts.get(key) ?? 0) + 1);
}
const top = [...counts.entries()].sort((a, b) => b[1] - a[1]).slice(0, 40);
writeFileSync(join(ROOT, "failed-requests.json"), JSON.stringify({ total: failed.length, top }, null, 2));
console.log(JSON.stringify({ total: failed.length, top }, null, 2));
await browser.close();
