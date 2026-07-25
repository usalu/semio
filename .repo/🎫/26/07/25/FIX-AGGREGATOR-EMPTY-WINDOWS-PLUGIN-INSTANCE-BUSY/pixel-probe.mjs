import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = process.env.AGGREGATOR_URL ?? "http://127.0.0.1:6023/";
const outDir = new URL(".", import.meta.url).pathname;

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
await page.waitForTimeout(4000);

const pixels = await page.evaluate(() => {
  return [...document.querySelectorAll("canvas")].map((canvas, index) => {
    const ctx = canvas.getContext("2d", { willReadFrequently: true });
    // WebGL canvases won't give 2d context — sample via drawImage to an offscreen 2d canvas.
    const probe = document.createElement("canvas");
    probe.width = Math.min(64, canvas.width);
    probe.height = Math.min(64, canvas.height);
    const pctx = probe.getContext("2d");
    pctx.drawImage(canvas, 0, 0, probe.width, probe.height);
    const data = pctx.getImageData(0, 0, probe.width, probe.height).data;
    let nonBg = 0;
    let samples = 0;
    for (let i = 0; i < data.length; i += 16) {
      samples += 1;
      const r = data[i], g = data[i + 1], b = data[i + 2], a = data[i + 3];
      // Count pixels that aren't near-transparent and aren't the beige window bg-ish.
      if (a > 8 && !(r > 230 && g > 220 && b > 200)) nonBg += 1;
    }
    return { index, w: canvas.width, h: canvas.height, samples, nonBg, ratio: nonBg / samples };
  });
});

await page.screenshot({ path: `${outDir}after-fix.png`, fullPage: true });
const report = { consoleMsgs, pixels, title: await page.title() };
writeFileSync(`${outDir}pixel-probe.json`, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2));
await browser.close();
if (consoleMsgs.length) process.exit(2);
if (!pixels.length || pixels.some((p) => p.ratio < 0.01)) process.exit(3);
