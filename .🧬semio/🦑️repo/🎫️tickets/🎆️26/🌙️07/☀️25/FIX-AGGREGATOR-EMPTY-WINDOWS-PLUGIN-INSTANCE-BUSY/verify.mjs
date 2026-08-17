import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const outDir = "/Users/ueli/Documents/semio/.repo/🎫️/26/07/25/FIX-AGGREGATOR-EMPTY-WINDOWS-PLUGIN-INSTANCE-BUSY";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleMsgs = [];
page.on("console", (msg) => {
  const text = msg.text();
  if (/busy|unreachable|action failed|render failed|panicked|js-sys/i.test(text)) {
    consoleMsgs.push({ type: msg.type(), text: text.slice(0, 500) });
  }
});

await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(1500);
const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
if (await skip.count()) {
  await skip.first().click({ timeout: 5000 }).catch(() => {});
}
await page.waitForTimeout(5000);

const info = await page.evaluate(() => {
  const windows = [...document.querySelectorAll('[data-slot="window"]')].map((el) => ({
    active: el.getAttribute("data-active"),
    hasCanvas: !!el.querySelector("canvas"),
    text: (el.innerText || "").split("\n").slice(0, 8),
  }));
  return {
    title: document.title,
    canvasCount: document.querySelectorAll("canvas").length,
    busyText: (document.body?.innerText || "").includes("plugin instance busy"),
    windows,
  };
});

await page.screenshot({ path: join(outDir, "after-fix.png"), fullPage: true });
const report = { consoleMsgs, info };
writeFileSync(join(outDir, "verify.json"), JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
if (consoleMsgs.length || info.busyText || info.canvasCount < 2) process.exit(1);
