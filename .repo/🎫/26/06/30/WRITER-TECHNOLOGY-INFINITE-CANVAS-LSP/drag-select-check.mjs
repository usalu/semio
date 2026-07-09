/** @emoji 🖱️ Drag-select smoke check for writer play (dev server on 6062). */
import { chromium } from "playwright";

const url = process.env.WRITER_PLAY_URL ?? "http://127.0.0.1:6062/";

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();

try {
  await page.goto(url, { waitUntil: "networkidle", timeout: 120_000 });
  await page.waitForSelector("canvas", { timeout: 60_000 });
  await page.waitForTimeout(3000);

  const canvas = page.locator("canvas").first();
  const box = await canvas.boundingBox();
  if (!box) throw new Error("no canvas box");

  const y = box.y + box.height * 0.5 + 17;
  const x1 = box.x + box.width * 0.5 + 72;
  const x2 = box.x + box.width * 0.5 + 280;
  await page.mouse.move(x1, y);
  await page.mouse.down();
  await page.mouse.move(x2, y, { steps: 12 });
  await page.mouse.up();
  await page.waitForTimeout(500);

  const selection = await page.evaluate(() => {
    const ta = document.querySelector("textarea");
    return {
      start: ta?.selectionStart ?? -1,
      end: ta?.selectionEnd ?? -1,
      length: ta?.value.length ?? 0,
    };
  });

  console.log("[DEBUG] drag-select-check", { selection });
  if (selection.end <= selection.start) {
    throw new Error(`drag select failed: ${JSON.stringify(selection)}`);
  }
  if (selection.end - selection.start < 3) {
    throw new Error(`drag select too short: ${JSON.stringify(selection)}`);
  }
} finally {
  await browser.close();
}
