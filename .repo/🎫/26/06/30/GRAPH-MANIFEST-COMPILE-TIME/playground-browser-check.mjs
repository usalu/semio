#!/usr/bin/env bun
/** @emoji 🌐 Browser smoke check for playgrounds (requires dev servers). */
import { chromium } from "playwright";

const targets = [
  { name: "trinity-jack", url: process.env.TRINITY_JACK_PLAY_URL ?? "http://127.0.0.1:6054/", canvasMin: 2, svgMin: 0 },
  { name: "writer", url: process.env.WRITER_PLAY_URL ?? "http://127.0.0.1:6062/", canvasMin: 1, svgMin: 0 },
  { name: "draw", url: process.env.DRAW_PLAY_URL ?? "http://127.0.0.1:6064/", canvasMin: 0, svgMin: 1 },
  { name: "raster", url: process.env.RASTER_PLAY_URL ?? "http://127.0.0.1:6060/", canvasMin: 1, svgMin: 0 },
  { name: "forms", url: process.env.FORMS_PLAY_URL ?? "http://127.0.0.1:6058/", canvasMin: 0, svgMin: 0 },
];

const fatalPatterns = [/missing field [`']manifest[`']/i, /TrinityCanvas.*error/i, /lsp client disposed/i];

const browser = await chromium.launch({ headless: true });

for (const target of targets) {
  const pageErrors = [];
  const page = await browser.newPage();
  page.on("pageerror", (err) => pageErrors.push(String(err)));
  page.on("console", (msg) => {
    if (msg.type() === "error") pageErrors.push(msg.text());
  });

  try {
    const res = await page.goto(target.url, { waitUntil: "domcontentloaded", timeout: 30_000 });
    if (!res?.ok()) throw new Error(`HTTP ${res?.status()}`);
  } catch (err) {
    if (String(err).includes("ERR_CONNECTION_REFUSED")) {
      console.log(`[DEBUG] ${target.name} skip (dev server not running)`);
      await page.close();
      continue;
    }
    throw err;
  }

  try {
    await page.waitForTimeout(4000);

    const metrics = await page.evaluate(() =>
      [...document.querySelectorAll("canvas")].map((c) => ({ w: c.width, h: c.height }))
    );
    const svgCount = await page.evaluate(() => document.querySelectorAll("svg").length);
    const validCanvases = metrics.filter((m) => m.w >= 2 && m.h >= 2).length;
    if (validCanvases < target.canvasMin) {
      throw new Error(`canvas count ${validCanvases} < ${target.canvasMin}: ${JSON.stringify(metrics)}`);
    }
    if (svgCount < target.svgMin) {
      throw new Error(`svg count ${svgCount} < ${target.svgMin}`);
    }

    const fatal = pageErrors.filter((e) => fatalPatterns.some((p) => p.test(e)));
    if (fatal.length) throw new Error(fatal.join(" | "));

    console.log(`[DEBUG] ${target.name} ok canvases=${validCanvases} svgs=${svgCount} warnings=${pageErrors.length}`);
  } catch (err) {
    console.error(`[DEBUG] ${target.name} FAIL: ${String(err)}`);
    if (pageErrors.length) console.error(`[DEBUG] ${target.name} pageErrors:`, pageErrors.slice(0, 5));
    throw err;
  } finally {
    await page.close();
  }
}

await browser.close();
console.log("[DEBUG] playground browser smoke check passed");
