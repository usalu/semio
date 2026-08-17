#!/usr/bin/env bun
import { chromium } from "playwright";
import { writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
const ticketDir = path.dirname(fileURLToPath(import.meta.url));
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const logs: string[] = [];
page.on("console", (m) => logs.push(`[${m.type()}] ${m.text()}`));
await page.addInitScript(() => {
  (window as unknown as { __semioContribDump?: unknown }).__semioContribDump = null;
  const orig = console.log;
  console.log = (...args: unknown[]) => {
    const text = args.map(String).join(" ");
    if (text.includes("setContributions") || text.includes("contributionsJson") || text.includes("flowExtension")) {
      (window as unknown as { __semioContribDump?: unknown }).__semioContribDump = text;
    }
    orig.apply(console, args as []);
  };
});
await page.goto(`http://127.0.0.1:6018/?plugin=procedural3d&bust=${Date.now()}`, { waitUntil: "domcontentloaded", timeout: 240000 });
await page.waitForSelector(".semio-node-graph-host", { timeout: 240000 });
await page.waitForTimeout(5000);
// Monkeypatch via evaluate: try to find React fiber / dump from DOM attributes if any
const dump = await page.evaluate(async () => {
  // Hook into any global debug
  const w = window as unknown as Record<string, unknown>;
  return {
    keys: Object.keys(w).filter((k) => /semio|plugin|contrib/i.test(k)).slice(0, 50),
  };
});
// Instrument handleAction by evaluating in page after load - intercept fetch? 
// Instead: look at setContributions skip warnings and push a debug by reading performance
const relevant = logs.filter((l) => /setContributions|contribution|flowExtension|manifestJson|unknown kind|invokeExtension/i.test(l));
await writeFile(path.join(ticketDir, "contributions-dump.json"), JSON.stringify({ dump, relevant, sampleLogs: logs.slice(0, 80) }, null, 2));
console.log(JSON.stringify({ dump, relevant, sampleLogs: logs.slice(0, 40) }, null, 2));
await browser.close();
