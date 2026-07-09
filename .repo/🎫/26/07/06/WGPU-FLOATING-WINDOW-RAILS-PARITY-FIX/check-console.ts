#!/usr/bin/env bun
import { chromium } from "playwright";
import { join } from "node:path";
import { PNG } from "pngjs";

const port = process.env.S_OS_PORT ?? "7199";
const ticketDir = import.meta.dir;

const logs: string[] = [];
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
});
const page = await browser.newPage();
page.on("console", (m) => logs.push(`[${m.type()}] ${m.text()}`));
page.on("pageerror", (e) => logs.push(`[pageerror] ${e.message}`));
await page.goto(`http://127.0.0.1:${port}/?plugin=flow`, { waitUntil: "domcontentloaded" });
await page.waitForSelector("#semio-wgpu-canvas", { timeout: 120_000 });
await new Promise<void>((resolve, reject) => {
  const t = setTimeout(() => reject(new Error("boot timeout")), 120_000);
  page.on("console", (m) => {
    if (m.text().includes("[DEBUG] wgpu renderer booted")) {
      clearTimeout(t);
      resolve();
    }
  });
});
await page.waitForTimeout(3000);
const canvas = page.locator("#semio-wgpu-canvas");
const box = await canvas.boundingBox();
if (!box) throw new Error("no canvas box");
const pngBuf = Buffer.from(
  await page.screenshot({
    type: "png",
    clip: { x: box.x, y: box.y, width: box.width, height: box.height },
    animations: "disabled",
    timeout: 60_000,
  }),
);
const pngPath = join(ticketDir, "check-screenshot.png");
await Bun.write(pngPath, pngBuf);
const { data, width: w, height: h } = PNG.sync.read(pngBuf);
let nonZero = 0;
let bgLike = 0;
let maxLuma = 0;
for (let i = 0; i < data.length; i += 4) {
  const r = data[i]!;
  const g = data[i + 1]!;
  const b = data[i + 2]!;
  const l = 0.299 * r + 0.587 * g + 0.114 * b;
  maxLuma = Math.max(maxLuma, l);
  if (r > 0 || g > 0 || b > 0) nonZero += 1;
  if (Math.abs(r - 13) <= 6 && Math.abs(g - 13) <= 6 && Math.abs(b - 15) <= 6) bgLike += 1;
}
const total = w * h;
const webgpuLogs = logs.filter((l) => l.includes("WebGPU") || l.includes("blur_globals") || l.includes("bind group"));
await Bun.write(join(ticketDir, "debug-console.txt"), `${logs.join("\n")}\n\nwebgpu=${webgpuLogs.length}\nscreenshot=${w}x${h} nonZero=${(nonZero / total).toFixed(4)} bgLike=${(bgLike / total).toFixed(4)} maxLuma=${maxLuma.toFixed(1)}\n`);
console.log({
  webgpuErrors: webgpuLogs.length,
  nonZeroRatio: nonZero / total,
  bgLikeRatio: bgLike / total,
  maxLuma,
  pngPath,
});
await browser.close();
