#!/usr/bin/env bun
/** 🔥️ S15: main-thread CPU profile of ONE opened program left idle (no input) — where an idle `s` spends its frames.
 * usage: bun s15-idle-profile.mjs <baseUrl> <tag> <pluginId> <appId> <query> [seconds] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl = "http://127.0.0.1:6540/", tag = "jack", pluginId = "trinity", appId = "s.trinity.jack@1/*#editor", query = "jack", seconds = "5"] = process.argv.slice(2);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await page.waitForTimeout(3_000);
await page.evaluate(() => document.body.focus());
await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.waitFor({ state: "visible", timeout: 15_000 });
await input.fill(query);
await page.waitForTimeout(1_200);
await page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${pluginId}.${appId}"], [data-slot="command-item"][data-command-item-id="spawn.${pluginId}"]`).first().click({ timeout: 10_000 });
await page.waitForTimeout(15_000);
const cdp = await page.context().newCDPSession(page);
await cdp.send("Profiler.enable");
await cdp.send("Profiler.setSamplingInterval", { interval: 500 });
await cdp.send("Profiler.start");
const frames = await page.evaluate((ms) => new Promise((resolve) => { let count = 0; const end = performance.now() + ms; const tick = () => { count += 1; if (performance.now() < end) requestAnimationFrame(tick); else resolve(count); }; requestAnimationFrame(tick); }), Number(seconds) * 1000);
const { profile } = await cdp.send("Profiler.stop");
const byId = new Map(profile.nodes.map((node) => [node.id, node]));
const parent = new Map();
for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
const deltas = profile.timeDeltas ?? [];
let idle = 0, total = 0;
const inclusive = new Map();
profile.samples.forEach((id, index) => {
  total += deltas[index] ?? 0;
  const name = byId.get(id).callFrame.functionName;
  if (name === "(idle)") idle += deltas[index] ?? 0;
  const seen = new Set();
  for (let cursor = id; cursor !== undefined; cursor = parent.get(cursor)) {
    const frame = byId.get(cursor).callFrame;
    if (!/\.(tsx?|js|mjs)(\?|$)/u.test(frame.url)) continue;
    const key = `${frame.functionName || "(anon)"} ${decodeURIComponent(frame.url.split("/").slice(-3).join("/")).slice(0, 90)}:${frame.lineNumber + 1}`;
    if (seen.has(key)) continue;
    seen.add(key);
    inclusive.set(key, (inclusive.get(key) ?? 0) + (deltas[index] ?? 0));
  }
});
const result = { tag, seconds: Number(seconds), rafFrames: frames, totalMs: Math.round(total / 1000), idleMs: Math.round(idle / 1000), busyPercent: Math.round(100 * (1 - idle / Math.max(1, total))), top: [...inclusive].sort((a, b) => b[1] - a[1]).slice(0, 12).map(([key, value]) => `${Math.round(value / 1000)}ms ${key}`) };
writeFileSync(fileURLToPath(new URL(`./generated/s15-idle-profile-${tag}.json`, import.meta.url)), JSON.stringify(result, null, 1));
console.log(JSON.stringify(result, null, 1));
await browser.close();
