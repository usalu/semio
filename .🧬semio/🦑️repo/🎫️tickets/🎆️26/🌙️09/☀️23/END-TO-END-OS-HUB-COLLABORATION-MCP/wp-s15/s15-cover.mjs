#!/usr/bin/env bun
/** 🔍️ S15 — which element covers a spawned program's Actions chip: ancestor chain of elementFromPoint. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { awaitBeacon, dismissIntroduction, spawnProgram } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const baseUrl = process.argv[2] ?? "http://127.0.0.1:6540/";
const plugin = process.argv[3] ?? "raster";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await awaitBeacon(page, Date.now() + 300_000);
await dismissIntroduction(page);
await page.waitForTimeout(3_000);
const spawned = await spawnProgram(page, plugin);
await page.waitForTimeout(6_000);
const chain = () => page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((t) => {
  const r = t.getBoundingClientRect();
  const x = r.left + r.width / 2, y = r.top + r.height / 2;
  const stack = document.elementsFromPoint(x, y).slice(0, 8).map((el) => `${el.tagName}#${el.id}[slot=${el.getAttribute('data-slot')}][cls=${String(el.className).slice(0, 60)}]`);
  const up = [];
  for (let el = document.elementFromPoint(x, y); el && up.length < 14; el = el.parentElement) up.push(`${el.tagName}#${el.id}[slot=${el.getAttribute('data-slot')}]`);
  const tUp = [];
  for (let el = t; el && tUp.length < 8; el = el.parentElement) tUp.push(`${el.tagName}#${el.id}[slot=${el.getAttribute('data-slot')}][folded=${el.getAttribute('data-folded')}]`);
  return { id: t.id, tag: t.tagName, html: t.outerHTML.slice(0, 400), stack, up, tUp };
}));
const before = await chain();
await page.locator('[id$=".engagement.toggle"]').first().click({ timeout: 5000 }).catch((e) => console.log('click err', String(e).slice(0, 300)));
await page.waitForTimeout(2000);
const after = await chain();
const panes = await page.evaluate(() => document.querySelectorAll('[data-slot="window-action-pane"]').length);
await browser.close();
const out = fileURLToPath(new URL(`./generated/s15-cover-${plugin}.json`, import.meta.url));
writeFileSync(out, JSON.stringify({ spawned, before, after, panes }, null, 1));
console.log(out);
