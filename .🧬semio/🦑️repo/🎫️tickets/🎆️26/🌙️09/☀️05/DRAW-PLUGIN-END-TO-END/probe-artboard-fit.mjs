/** 🖼️ Reads the booted Draw canvas and reports whether the painted board is fitted inside the viewport. */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const serve = process.argv[2] ?? "http://127.0.0.1:6064/";
const output = process.argv[3];
if (!output) throw Error("output directory required");
mkdirSync(output, { recursive: true });
const url = new URL(serve);
url.searchParams.set("plugin", "draw");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader", "--enable-unsafe-webgpu"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
try {
  await page.goto(url.href, { waitUntil: "domcontentloaded", timeout: 60000 });
  await page.waitForFunction(() => document.documentElement.dataset.semioOsReady || document.documentElement.dataset.semioOsError, undefined, { timeout: 90000 });
  const canvas = page.locator('[data-surface-role="editor"] [data-ui-node-key="drawing.play.composite"] canvas').first();
  await canvas.waitFor({ state: "visible", timeout: 30000 });
  await page.waitForTimeout(1500);
  await canvas.screenshot({ path: join(output, "boot-canvas.png") });
  const fit = await canvas.evaluate(element => {
    const ctx = element.getContext("2d");
    const width = element.width;
    const height = element.height;
    const data = ctx.getImageData(0, 0, width, height).data;
    const at = (x, y) => {
      const i = (y * width + x) * 4;
      return [data[i], data[i + 1], data[i + 2]];
    };
    const corner = at(2, 2);
    const far = (x, y) => {
      const pixel = at(x, y);
      return Math.abs(pixel[0] - corner[0]) + Math.abs(pixel[1] - corner[1]) + Math.abs(pixel[2] - corner[2]) > 48;
    };
    const step = 12;
    let minX = width, minY = height, maxX = -1, maxY = -1, filled = 0;
    for (let y = step; y < height - step; y += step) {
      for (let x = step; x < width - step; x += step) {
        let ink = 0;
        for (let dy = -2; dy <= 2; dy++) for (let dx = -2; dx <= 2; dx++) if (far(x + dx, y + dy)) ink++;
        if (ink < 12) continue;
        filled++;
        minX = Math.min(minX, x);
        minY = Math.min(minY, y);
        maxX = Math.max(maxX, x);
        maxY = Math.max(maxY, y);
      }
    }
    const box = filled ? { minX, minY, maxX, maxY, filled } : null;
    const centerX = box ? (box.minX + box.maxX) / 2 / width : null;
    const centerY = box ? (box.minY + box.maxY) / 2 / height : null;
    const fitted = !!box && box.minX < width * 0.4 && box.minY < height * 0.4 && box.maxX > width * 0.6 && box.maxY > height * 0.6 && Math.abs(centerX - 0.5) < 0.12 && Math.abs(centerY - 0.5) < 0.12;
    return { width, height, corner, box, centerX, centerY, fitted };
  });
  writeFileSync(join(output, "fit.json"), JSON.stringify({ url: url.href, ...fit }, null, 2));
  console.log(JSON.stringify(fit));
  if (!fit.fitted) process.exitCode = 1;
} finally {
  await browser.close();
}
