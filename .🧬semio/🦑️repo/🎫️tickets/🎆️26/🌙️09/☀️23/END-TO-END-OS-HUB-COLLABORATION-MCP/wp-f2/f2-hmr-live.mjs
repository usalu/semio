#!/usr/bin/env bun
/** 🔥️ F2-3 live proof on an HMR-on dev serve: boots `s`, rewrites one served module with its own bytes and records every
 * `[vite]` console line until quiet (the edit must arrive as hot updates, no full reload); then rewrites the serve's config
 * entry with its own bytes (Vite restarts the server under the connected page) and records restart → reconnected → served.
 * usage: bun f2-hmr-live.mjs <baseUrl> <module-repo-path> <config-repo-path> <serveLog> */
import { chromium } from "playwright";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [baseUrl, modulePath, configPath, serveLog] = process.argv.slice(2);
const repo = "/Users/ueli/Documents/semio/";
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
page.setDefaultNavigationTimeout(300_000);
const lines = [];
page.on("console", (message) => { if (message.text().startsWith("[vite]")) lines.push({ at: Date.now(), text: message.text().slice(0, 300) }); });
let navigations = 0;
page.on("framenavigated", (frame) => { if (frame === page.mainFrame()) navigations += 1; });
const quiet = async (ms) => { let count = lines.length; let since = Date.now(); while (Date.now() - since < ms) { await page.waitForTimeout(100); if (lines.length !== count) [count, since] = [lines.length, Date.now()]; } };
const report = { baseUrl, modulePath, configPath };
await page.goto(baseUrl, { waitUntil: "commit" });
report.beacon = await sweep.awaitBeacon(page, Date.now() + 300_000);
await sweep.dismissIntroduction(page);
await quiet(3_000);
report.bootLines = lines.map(({ text }) => text);
const rewrite = (path) => { const bytes = readFileSync(repo + path); writeFileSync(repo + path, bytes); return Date.now(); };
const navigationsBeforeEdit = navigations;
const cursor = lines.length;
const edited = rewrite(modulePath);
await quiet(3_000);
report.edit = { lines: lines.slice(cursor).map(({ at, text }) => `${at - edited} ms ${text}`), navigations: navigations - navigationsBeforeEdit, hotUpdates: lines.slice(cursor).filter(({ text }) => text.startsWith("[vite] hot updated")).length, firstUpdateMs: (lines.slice(cursor).find(({ text }) => text.startsWith("[vite] hot updated"))?.at ?? NaN) - edited };
console.log(JSON.stringify(report.edit));
const logBefore = readFileSync(serveLog, "utf8").length;
const restartCursor = lines.length;
const navigationsBeforeRestart = navigations;
const restarted = rewrite(configPath);
let served = 0;
const deadline = Date.now() + 60_000;
while (Date.now() < deadline) {
  await page.waitForTimeout(200);
  if (navigations > navigationsBeforeRestart && lines.slice(restartCursor).some(({ text }) => text === "[vite] connected.")) break;
}
const reconnectedMs = Date.now() - restarted;
served = await fetch(baseUrl, { signal: AbortSignal.timeout(10_000) }).then((response) => response.status, () => 0);
await quiet(2_000);
report.restart = { reconnectedMs, served, navigations: navigations - navigationsBeforeRestart, lines: lines.slice(restartCursor).map(({ at, text }) => `${at - restarted} ms ${text}`), serveLog: readFileSync(serveLog, "utf8").slice(logBefore).split("\n").filter((line) => /restart|changed|error|ready in/iu.test(line)).slice(0, 12) };
console.log(JSON.stringify(report.restart));
writeFileSync(`${generated}f2-hmr-live.json`, JSON.stringify(report, null, 1));
await browser.close();
