#!/usr/bin/env bun
/** 🔍️ S15 — one-off DOM exploration inside `s`: spawn one program, dump window/tab close controls. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, spawnProgram, windowIds } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6540/";
const plugin = process.argv[3] ?? "dag";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const lines = [];
page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 300)));
page.on("pageerror", (e) => lines.push(`pageerror: ${String(e)}`.slice(0, 300)));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
const beacon = await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const before = await windowIds(page);
const spawned = await spawnProgram(page, plugin);
const dump = await page.evaluate(() => {
  const describe = (el) => ({ tag: el.tagName, id: el.id, slot: el.getAttribute("data-slot"), aria: el.getAttribute("aria-label"), title: el.getAttribute("title"), text: (el.textContent ?? "").trim().slice(0, 40), win: el.getAttribute("data-window-id") });
  return {
    closeish: [...document.querySelectorAll("[id*='close' i], [aria-label*='close' i], [aria-label*='schließen' i], [title*='close' i]")].map(describe).slice(0, 60),
    windowIdEls: [...document.querySelectorAll("[data-window-id]")].map(describe).slice(0, 30),
    windowSlots: [...document.querySelectorAll("[data-slot='window']")].map(describe).slice(0, 30),
    tabs: [...document.querySelectorAll("[role='tab']")].map(describe).slice(0, 40),
  };
});
await browser.close();
const out = fileURLToPath(new URL(`./generated/s15-explore-${plugin}.json`, import.meta.url));
writeFileSync(out, JSON.stringify({ beacon, before, spawned, dump, console: lines.slice(-40) }, null, 1));
console.log(out);
