#!/usr/bin/env bun
/** 🔍️ S15 — why raster's Actions rail reads 0 rows inside `s`: spawn raster, dump toggles, folded state, covering panels. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, spawnProgram, unfoldActionsRail, readShell } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6540/";
const plugin = process.argv[3] ?? "raster";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
const lines = [];
page.on("console", (m) => lines.push(`${m.type()}: ${m.text()}`.slice(0, 400)));
page.on("pageerror", (e) => lines.push(`pageerror: ${String(e)}`.slice(0, 400)));
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
const beacon = await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const cursor = lines.length;
const spawned = await spawnProgram(page, plugin);
await page.waitForTimeout(5_000);
const toggles = () => page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((t) => {
  const r = t.getBoundingClientRect();
  const top = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
  return { id: t.id, folded: t.closest('[data-folded]')?.getAttribute('data-folded') ?? null, rect: [Math.round(r.left), Math.round(r.top), Math.round(r.width), Math.round(r.height)], topIsSelf: top === t || t.contains(top), top: top ? `${top.tagName}#${top.id}.${top.getAttribute('data-slot')}` : null };
}));
const beforeUnfold = await toggles();
const clicked = await unfoldActionsRail(page);
const afterUnfold = await toggles();
const panes = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"]')].map((p) => ({ win: p.closest('[data-slot="window"]')?.id, rows: p.querySelectorAll('[id^="action."]').length, text: (p.textContent ?? '').slice(0, 200) })));
const shell = await readShell(page);
await page.screenshot({ path: fileURLToPath(new URL(`./generated/s15-rail-${plugin}.png`, import.meta.url)) });
await browser.close();
const out = fileURLToPath(new URL(`./generated/s15-rail-${plugin}.json`, import.meta.url));
writeFileSync(out, JSON.stringify({ beacon, spawned, beforeUnfold, clicked, afterUnfold, panes, actions: shell.actions, windowIds: shell.windowIds, console: lines.slice(cursor).filter((l) => !/agent-bridge/.test(l)).slice(0, 60) }, null, 1));
console.log(out);
