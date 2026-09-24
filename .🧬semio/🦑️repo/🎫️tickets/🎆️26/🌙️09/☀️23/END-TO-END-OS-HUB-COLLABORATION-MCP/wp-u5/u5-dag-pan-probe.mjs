#!/usr/bin/env bun
/** 🤏️ U5 §6a — dag two-finger pan in the running `s` shell: spawns dag, drives a real two-point touch pan (constant
 * finger distance, so no zoom) over the graph canvas through CDP, and reads `data-viewport-camera-json` before and after.
 * A horizontal pan must move `x` by `-Δx / zoom` and leave `y` and `zoom` alone; a vertical one the converse.
 * Usage: bun u5-dag-pan-probe.mjs <baseUrl> <tag> */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dismissIntroduction, spawnProgram } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl = "http://127.0.0.1:6580/", tag = "pan"] = process.argv.slice(2);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, hasTouch: true });
const page = await context.newPage();
const lines = [];
page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 300)));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const spawned = await spawnProgram(page, "dag");
const host = page.locator("[data-viewport-camera-json]").first();
await host.waitFor({ state: "attached", timeout: 120_000 });
await page.waitForTimeout(3_000);
const camera = async () => JSON.parse((await host.getAttribute("data-viewport-camera-json")) ?? "null");
const canvas = host.locator("canvas").first();
const box = await canvas.boundingBox();
const cdp = await context.newCDPSession(page);
const touch = (type, points) => cdp.send("Input.dispatchTouchEvent", { type, touchPoints: points.map(([x, y], id) => ({ x, y, id, radiusX: 4, radiusY: 4, force: 1 })) });
async function pan(dx, dy) {
  const cx = box.x + box.width / 2;
  const cy = box.y + box.height / 2;
  const start = [[cx - 60, cy], [cx + 60, cy]];
  await touch("touchStart", start);
  const steps = 6;
  for (let step = 1; step <= steps; step += 1) {
    await touch("touchMove", start.map(([x, y]) => [x + (dx * step) / steps, y + (dy * step) / steps]));
    await page.waitForTimeout(30);
  }
  await touch("touchEnd", []);
  await page.waitForTimeout(600);
}
const rows = [];
const before = await camera();
await pan(90, 0);
const afterHorizontal = await camera();
await pan(0, 70);
const afterVertical = await camera();
rows.push({ step: "before", camera: before }, { step: "horizontal +90px", camera: afterHorizontal }, { step: "vertical +70px", camera: afterVertical });
const zoom = (value) => value?.zoom ?? value?.[2] ?? null;
const x = (value) => value?.x ?? value?.[0] ?? null;
const y = (value) => value?.y ?? value?.[1] ?? null;
const verdict = {
  horizontalDx: x(afterHorizontal) - x(before),
  horizontalDy: y(afterHorizontal) - y(before),
  horizontalZoomRatio: zoom(afterHorizontal) / zoom(before),
  verticalDx: x(afterVertical) - x(afterHorizontal),
  verticalDy: y(afterVertical) - y(afterHorizontal),
  expectedHorizontalDx: -90 / zoom(before),
  expectedVerticalDy: -70 / zoom(afterHorizontal),
};
await page.screenshot({ path: out(`u5-dag-${tag}.png`) });
writeFileSync(out(`u5-dag-${tag}.json`), JSON.stringify({ baseUrl, spawned, box, rows, verdict, faults: lines.filter((l) => /pageerror|Uncaught|trap|panicked/iu.test(l)).slice(-20) }, null, 1));
console.log(JSON.stringify({ rows, verdict }, null, 1));
await browser.close();
