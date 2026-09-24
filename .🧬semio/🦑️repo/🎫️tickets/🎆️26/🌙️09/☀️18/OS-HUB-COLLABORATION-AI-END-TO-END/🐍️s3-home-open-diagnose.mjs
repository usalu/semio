#!/usr/bin/env bun
/** 🧭️ S3 (session 10) — what the `s` Home offers for opening a NEW foreign-kind artifact: plugin census,
 * palette rows for the named kinds, and the Home window's own affordances, read off a live serve.
 *
 * Usage: bun 🐍️s3-home-open-diagnose.mjs <baseUrl> <query...> */
import { chromium } from "playwright";
import { fileURLToPath } from "node:url";
import { writeFileSync } from "node:fs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6380/";
const queries = process.argv.slice(3).length > 0 ? process.argv.slice(3) : ["raster", "dag"];
const out = fileURLToPath(new URL("./wp-s3/generated/s3-home-open-diagnose.txt", import.meta.url));
const lines = [];
const say = (...parts) => { const line = parts.map((part) => (typeof part === "string" ? part : JSON.stringify(part))).join(" "); lines.push(line); console.log("[s3]", line); };

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const consoleLines = [];
page.on("console", (message) => consoleLines.push(`${message.type()}: ${message.text()}`.slice(0, 300)));
page.on("pageerror", (error) => consoleLines.push(`pageerror: ${String(error)}`.slice(0, 300)));
try {
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  const deadline = Date.now() + 300_000;
  let beacon = null;
  while (Date.now() < deadline && beacon === null) {
    beacon = await page.evaluate(() => { const d = document.documentElement.dataset; return d.semioOsReady !== undefined ? `ready:${d.semioOsReady}` : d.semioOsError !== undefined ? `error:${d.semioOsError}` : null; });
    if (beacon === null) await page.waitForTimeout(1000);
  }
  say("beacon", beacon);
  await page.waitForTimeout(8_000);
  const probe = await page.evaluate(() => { const p = window.__semioOsCatalogProbe; return p ? { shell: p.shellPluginId, plugins: p.plugins.map((r) => `${r.pluginId}:${r.status}`), programs: p.programs.length, keys: Object.keys(p) } : null; });
  say("catalogProbe", probe);
  say("homeIds", await page.evaluate(() => [...document.querySelectorAll("[id]")].map((e) => e.id).filter((id) => /^(s-|window:s-home|action\.)/u.test(id)).slice(0, 80)));
  say("windowIds", await page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((e) => e.getAttribute("data-window-id"))));
  for (const query of queries) {
    await page.locator('[data-slot="navbar"]').first().click({ force: true, position: { x: 4, y: 4 } }).catch(() => undefined);
    await page.keyboard.press("Meta+p");
    const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
    await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
    if ((await input.count()) === 0) { say(query, "palette never opened"); continue; }
    await input.fill(query);
    await page.waitForTimeout(2_000);
    say(`palette[${query}]`, await page.evaluate(() => [...document.querySelectorAll('[data-slot="command-item"]')].map((e) => `${e.getAttribute("data-command-item-id")}|${(e.textContent ?? "").trim().slice(0, 50)}`).slice(0, 40)));
    await page.keyboard.press("Escape");
    await page.waitForTimeout(800);
  }
  await page.screenshot({ path: fileURLToPath(new URL("./wp-s3/generated/s3-home.png", import.meta.url)) });
} finally {
  say("console (filtered)", consoleLines.filter((line) => !/DevTools|\[vite\]/u.test(line)).slice(0, 80));
  await browser.close();
  writeFileSync(out, lines.join("\n") + "\n");
}
