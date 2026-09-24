#!/usr/bin/env bun
/** 👻️ S15 — inspects the panel stack that hit-tests over a spawned window's Actions chip: per-ancestor opacity,
 * visibility, pointer-events, rect, data-state. */
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
const home = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].map((p) => ({ id: p.id, rect: [...Object.values(p.getBoundingClientRect().toJSON())].map(Math.round), op: getComputedStyle(p).opacity })));
const spawned = await spawnProgram(page, plugin);
await page.waitForTimeout(6_000);
const facts = await page.evaluate((index) => {
  const t = [...document.querySelectorAll('[id$=".engagement.toggle"]')].at(index);
  const r = t.getBoundingClientRect();
  const top = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
  const chain = [];
  for (let el = top; el && chain.length < 30; el = el.parentElement) {
    const cs = getComputedStyle(el);
    const b = el.getBoundingClientRect();
    chain.push(`${el.tagName}#${el.id}[slot=${el.getAttribute('data-slot')}][state=${el.getAttribute('data-state')}] op=${cs.opacity} vis=${cs.visibility} pe=${cs.pointerEvents} z=${cs.zIndex} pos=${cs.position} bg=${cs.backgroundColor} rect=${Math.round(b.left)},${Math.round(b.top)},${Math.round(b.width)}x${Math.round(b.height)} clip=${cs.clipPath} tr=${cs.transform}`);
  }
  const panels = [...document.querySelectorAll('[data-slot="panel"]')].map((p) => { const b = p.getBoundingClientRect(); const cs = getComputedStyle(p); return `${p.id} rect=${Math.round(b.left)},${Math.round(b.top)},${Math.round(b.width)}x${Math.round(b.height)} op=${cs.opacity} vis=${cs.visibility} off=${p.offsetParent !== null}`; });
  return { id: t.id, chain, panels };
}, Number(process.env.S15_TOGGLE ?? 0));
await page.screenshot({ path: fileURLToPath(new URL(`./generated/s15-ghost-${plugin}.png`, import.meta.url)) });
await browser.close();
const out = fileURLToPath(new URL(`./generated/s15-ghost-${plugin}.json`, import.meta.url));
writeFileSync(out, JSON.stringify({ home, spawned, facts }, null, 1));
console.log(out);
