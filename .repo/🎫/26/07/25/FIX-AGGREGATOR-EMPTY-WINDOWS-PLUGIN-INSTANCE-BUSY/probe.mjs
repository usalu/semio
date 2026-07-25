import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const outDir = new URL(".", import.meta.url).pathname;

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
const pageErrors = [];
const consoleMsgs = [];

page.on("pageerror", (err) => {
  pageErrors.push({ message: err.message, stack: err.stack });
});
page.on("console", (msg) => {
  const text = msg.text();
  if (/busy|error|Error|DEBUG|Fehlendes|empty|payload/i.test(text)) {
    consoleMsgs.push({ type: msg.type(), text });
  }
});

await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
await page.waitForTimeout(2500);

// Skip intro if present
const skip = page.getByRole("button", { name: /Überspringen|Skip/i });
if (await skip.count()) {
  await skip.first().click({ timeout: 5000 }).catch(() => {});
  await page.waitForTimeout(3000);
}

const info = await page.evaluate(() => {
  const canvases = [...document.querySelectorAll("canvas")].map((c) => ({
    w: c.width,
    h: c.height,
    cw: c.clientWidth,
    ch: c.clientHeight,
  }));
  const bodies = [...document.querySelectorAll('[data-slot="window"]')].map((el) => ({
    kind: el.getAttribute("data-kind"),
    active: el.getAttribute("data-active"),
    text: (el.innerText || "").slice(0, 200),
    hasCanvas: !!el.querySelector("canvas"),
    childCount: el.children.length,
  }));
  const missing = [...document.querySelectorAll("body *")]
    .map((el) => el.textContent || "")
    .filter((t) => /Fehlendes Fenster|plugin instance busy|\[object Object\]/i.test(t))
    .slice(0, 20);
  return {
    title: document.title,
    canvasCount: canvases.length,
    canvases,
    bodies,
    missing,
    bodySnippet: (document.body?.innerText || "").slice(0, 800),
  };
});

await page.screenshot({ path: `${outDir}probe.png`, fullPage: true });

const report = { url, pageErrors, consoleMsgs, info };
writeFileSync(`${outDir}probe.json`, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
