/** ⚛️🔍 W14d: measures React's top-left panel box, the dock cap chips beneath it and what
 * `elementFromPoint` answers at each cap centre — the evidence for the panel/cap geometry family. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";

const url = process.env.SEMIO_REACT_URL ?? "http://127.0.0.1:6013/?plugin=puzzle3d";
const out = process.env.SEMIO_W14D_OUT ?? "/tmp/w14d-cap-occlusion.json";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=metal"] });
const ctx = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
const page = await ctx.newPage();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180000 });
for (let s = 0; s < Number(process.env.SEMIO_REACT_BOOT ?? 420); s += 1) {
  await page.waitForTimeout(1000);
  const state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), controls: document.querySelectorAll("[id],button").length })).catch(() => ({ ready: null, controls: 0 }));
  if (s % 20 === 0) console.log(`${s}s ready=${state.ready} controls=${state.controls}`);
  if (state.ready && state.controls > 40 && s > 4) { console.log(`BOOTED at ${s}s`); break; }
}
// open the Catalogue panel the journey opens before the cap steps
for (const id of ["framework.panelTab.catalogue", "framework.panel.catalogue"]) {
  await page.evaluate((cid) => document.getElementById(cid)?.click(), id).catch(() => {});
}
await page.waitForTimeout(2500);
const report = await page.evaluate(() => {
  const box = (el) => { if (!el) return null; const r = el.getBoundingClientRect(); return { id: el.id || null, slot: el.getAttribute?.("data-slot") || null, cls: (el.className && String(el.className)).slice(0, 80), x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) }; };
  const all = Array.from(document.querySelectorAll("[data-slot],[id]"));
  const panels = all.filter((el) => /panel/i.test(`${el.getAttribute("data-slot") || ""} ${el.id || ""}`)).map(box).filter((b) => b && b.w > 100 && b.h > 60);
  const caps = all.filter((el) => /dock-tab|dock\.tab|mode-dock/i.test(`${el.getAttribute("data-slot") || ""} ${el.id || ""}`)).map(box);
  const probePoints = [[59, 46], [84, 46], [48, 35], [73, 35], [300, 46]];
  const hits = probePoints.map(([x, y]) => {
    const chain = document.elementsFromPoint(x, y).slice(0, 6).map((el) => `${el.tagName.toLowerCase()}#${el.id || ""}[${el.getAttribute("data-slot") || ""}]`);
    return { x, y, chain };
  });
  const modeBody = box(document.querySelector('[data-slot="mode-body"],[data-slot="canvas-root"],[data-slot="mode-root"]'));
  return { panels: panels.slice(0, 20), caps: caps.slice(0, 30), hits, modeBody };
});
writeFileSync(out, JSON.stringify(report, null, 2));
console.log(JSON.stringify(report, null, 2).slice(0, 6000));
await browser.close();
