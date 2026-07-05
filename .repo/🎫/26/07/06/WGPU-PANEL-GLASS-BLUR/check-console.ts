#!/usr/bin/env bun
import { chromium } from "playwright";
import { join } from "node:path";

const port = process.env.S_OS_PORT ?? "7199";
const ticketDir = import.meta.dir;

const errors: string[] = [];
const browser = await chromium.launch({
  headless: true,
  args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
});
const page = await browser.newPage();
page.on("console", (m) => {
  if (m.type() === "error") errors.push(m.text());
});
await page.goto(`http://127.0.0.1:${port}/?plugin=flow`, { waitUntil: "domcontentloaded" });
await page.waitForSelector("#semio-wgpu-canvas", { timeout: 120_000 });
await new Promise((r) => setTimeout(r, 8000));
const sample = await page.evaluate(async () => {
  const canvas = document.querySelector("#semio-wgpu-canvas") as HTMLCanvasElement | null;
  if (!canvas) return { missing: true };
  const bmp = await createImageBitmap(canvas);
  const c = document.createElement("canvas");
  c.width = bmp.width;
  c.height = bmp.height;
  const g = c.getContext("2d")!;
  g.drawImage(bmp, 0, 0);
  const nav = g.getImageData(Math.floor(bmp.width * 0.5), 11, 1, 1).data;
  const mid = g.getImageData(Math.floor(bmp.width * 0.5), Math.floor(bmp.height * 0.5), 1, 1).data;
  return { w: bmp.width, h: bmp.height, nav: [nav[0], nav[1], nav[2]], mid: [mid[0], mid[1], mid[2]] };
});
await Bun.write(join(ticketDir, "debug-console.txt"), `${errors.join("\n") || "(no errors)"}\n\nsample=${JSON.stringify(sample)}`);
console.log("errors:", errors.length, "sample:", sample);
await browser.close();
