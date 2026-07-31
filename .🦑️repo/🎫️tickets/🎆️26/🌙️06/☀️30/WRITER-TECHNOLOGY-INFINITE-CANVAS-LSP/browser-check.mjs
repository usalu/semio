/** @emoji 🌐️ Browser smoke check for writer play (requires dev server on 6062). */
import { chromium } from "playwright";

const url = process.env.WRITER_PLAY_URL ?? "http://127.0.0.1:6062/";
const pageErrors = [];

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
page.on("pageerror", (err) => pageErrors.push(String(err)));

try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
  await page.waitForSelector("canvas", { timeout: 60_000, state: "attached" });
  await page.waitForTimeout(3000);

  const metrics = await page.evaluate(() => {
    const canvases = [...document.querySelectorAll("canvas")];
    return canvases.map((canvas) => ({ w: canvas.width, h: canvas.height }));
  });
  if (metrics.length === 0 || metrics.every((m) => m.w < 2)) {
    throw new Error(`invalid canvas metrics: ${JSON.stringify(metrics)}`);
  }

  const writerErrors = pageErrors.filter((e) => e.includes("WriterCanvas is not defined") || e.includes("ReferenceError"));
  if (writerErrors.length > 0) throw new Error(writerErrors.join(" | "));

  const editor = page.locator('[role="textbox"]').first();
  await editor.click();
  await page.keyboard.press("End");
  await page.keyboard.type(" RETURN a.name");
  await page.waitForTimeout(1500);

  console.log("[DEBUG] browser-check ok", { metrics, pageErrors });
} finally {
  await browser.close();
}
