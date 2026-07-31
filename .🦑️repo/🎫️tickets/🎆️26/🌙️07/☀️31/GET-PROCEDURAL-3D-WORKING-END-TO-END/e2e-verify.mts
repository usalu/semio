import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";

const outDir = dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({
  executablePath: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  headless: true,
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleMsgs = [];
page.on("console", (msg) => {
  const text = msg.text();
  if (msg.type() === "error" || text.includes("clearGhostWidget") || text.includes("Render error")) {
    consoleMsgs.push({ type: msg.type(), text: text.slice(0, 500) });
  }
});
page.on("pageerror", (e) => consoleMsgs.push({ type: "pageerror", text: e.message.slice(0, 500) }));

await page.goto("http://127.0.0.1:6018/", { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForTimeout(10000);

const bodyText = await page.locator("body").innerText();
const hasRenderError = /Render error|Renderfehler|clearGhostWidget is not a function|Maximum update depth/i.test(bodyText);
const title = await page.title();
const canvasCount = await page.locator("canvas").count();
const hasFlow = /Flow/i.test(bodyText);
const hasPreview = /Preview/i.test(bodyText);
await page.screenshot({ path: outDir + "/e2e-after-fix.png", fullPage: false });

const report = {
  title,
  hasRenderError,
  canvasCount,
  hasFlow,
  hasPreview,
  ghostErrors: consoleMsgs.filter((m) => m.text.includes("clearGhostWidget")),
  pageErrors: consoleMsgs.filter((m) => m.type === "pageerror"),
  consoleSample: consoleMsgs.slice(0, 20),
  bodyStart: bodyText.slice(0, 800),
  ok: !hasRenderError && canvasCount >= 2 && hasFlow && hasPreview,
};
writeFileSync(outDir + "/e2e-report.json", JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
if (!report.ok) process.exit(1);
