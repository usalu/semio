#!/usr/bin/env bun
/** 🧪️ Procedural 3d OS dev smoke: flow graph + world-3d preview boot without console errors. */

import { chromium } from "playwright";
import { join } from "node:path";
import { PNG } from "pngjs";

const repoRoot = join(import.meta.dir, "../../../../..");
const port = process.env.S_OS_PORT ?? "6018";
const baseUrl = `http://127.0.0.1:${port}/?plugin=procedural3d`;
const bootTimeoutMs = 240_000;

function analyzeBodyRegion(png: Buffer): { bodyNonBgRatio: number; maxLuma: number } {
  const { data, width: w, height: h } = PNG.sync.read(png);
  const y0 = Math.floor(h * 0.08);
  const y1 = Math.floor(h * 0.92);
  const x0 = Math.floor(w * 0.06);
  const x1 = Math.floor(w * 0.94);
  const bgR = 13;
  const bgG = 13;
  const bgB = 15;
  const tolerance = 8;
  const luma = (r: number, g: number, b: number) => 0.299 * r + 0.587 * g + 0.114 * b;
  let bodyNonBg = 0;
  let bodyTotal = 0;
  let maxLuma = 0;
  for (let y = y0; y < y1; y++) {
    for (let x = x0; x < x1; x++) {
      const i = (y * w + x) * 4;
      const r = data[i]!;
      const g = data[i + 1]!;
      const b = data[i + 2]!;
      maxLuma = Math.max(maxLuma, luma(r, g, b));
      bodyTotal += 1;
      if (Math.abs(r - bgR) > tolerance || Math.abs(g - bgG) > tolerance || Math.abs(b - bgB) > tolerance) {
        bodyNonBg += 1;
      }
    }
  }
  return { bodyNonBgRatio: bodyTotal > 0 ? bodyNonBg / bodyTotal : 0, maxLuma };
}

async function waitForDev(url: string): Promise<void> {
  const deadline = Date.now() + bootTimeoutMs;
  while (Date.now() < deadline) {
    try {
      const response = await fetch(url);
      if (response.ok) return;
    } catch {}
    await Bun.sleep(500);
  }
  throw new Error(`dev server not ready at ${url}`);
}

async function main(): Promise<void> {
  await waitForDev(baseUrl);
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const errors: string[] = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => {
    if (message.type() === "error" && !message.text().includes("[DEBUG]")) {
      errors.push(message.text());
    }
  });
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: bootTimeoutMs });
  await page.waitForSelector('[data-slot="navbar"]', { timeout: bootTimeoutMs });
  await page.waitForSelector('[data-slot="footer"]', { timeout: bootTimeoutMs });
  await page.waitForTimeout(4000);

  const graph = page.locator(".semio-node-graph-host").first();
  await graph.waitFor({ state: "visible", timeout: bootTimeoutMs });

  const previewCanvas = page.locator(".semio-world-3d-host canvas").first();
  if ((await previewCanvas.count()) === 0) throw new Error("world-3d preview canvas missing");
  const box = await previewCanvas.boundingBox();
  if (!box || box.width < 8 || box.height < 8) throw new Error("world-3d preview canvas too small");

  const png = Buffer.from(await previewCanvas.screenshot({ type: "png" }));
  const stats = analyzeBodyRegion(png);
  if (stats.bodyNonBgRatio < 0.004) {
    throw new Error(`preview appears blank ratio=${stats.bodyNonBgRatio.toFixed(4)}`);
  }
  if (stats.maxLuma < 28) {
    throw new Error(`preview lacks contrast maxLuma=${stats.maxLuma.toFixed(1)}`);
  }

  if (errors.length > 0) throw new Error(`console errors: ${errors.join(" | ")}`);
  console.log("[DEBUG] procedural3d e2e passed", {
    previewRatio: stats.bodyNonBgRatio.toFixed(4),
    maxLuma: stats.maxLuma.toFixed(1),
  });
  await browser.close();
}

await main();
