#!/usr/bin/env bun
import { chromium } from "playwright";
import { PNG } from "pngjs";
import { writeFileSync } from "node:fs";
import { join } from "node:path";

const port = process.env.S_OS_PORT ?? "7199";
const out = join(import.meta.dir, "debug-shot.png");
const browser = await chromium.launch({
	headless: true,
	args: ["--enable-unsafe-webgpu", "--use-angle=swiftshader", "--enable-features=Vulkan"],
});
const page = await browser.newPage();
await page.goto(`http://127.0.0.1:${port}/?plugin=s`, { waitUntil: "domcontentloaded", timeout: 120000 });
await page.waitForSelector("#semio-wgpu-canvas", { timeout: 120000 });
await new Promise((resolve, reject) => {
	const t = setTimeout(() => reject(new Error("boot timeout")), 120000);
	page.on("console", (m) => {
		if (m.text().includes("[DEBUG] wgpu renderer booted")) {
			clearTimeout(t);
			resolve();
		}
	});
});
await page.waitForTimeout(1200);
const png = await page.locator("#semio-wgpu-canvas").screenshot({ type: "png" });
writeFileSync(out, png);
const { data, width, height } = PNG.sync.read(Buffer.from(png));
let minL = 255, maxL = 0, nonBg = 0;
const BG = [13, 13, 15], T = 6;
for (let i = 0; i < data.length; i += 4) {
	const l = 0.299 * data[i] + 0.587 * data[i + 1] + 0.114 * data[i + 2];
	minL = Math.min(minL, l);
	maxL = Math.max(maxL, l);
	if (Math.abs(data[i] - BG[0]) > T || Math.abs(data[i + 1] - BG[1]) > T || Math.abs(data[i + 2] - BG[2]) > T) nonBg++;
}
console.log({ width, height, minL, maxL, nonBgRatio: nonBg / (width * height), out, bytes: png.length });
await browser.close();
