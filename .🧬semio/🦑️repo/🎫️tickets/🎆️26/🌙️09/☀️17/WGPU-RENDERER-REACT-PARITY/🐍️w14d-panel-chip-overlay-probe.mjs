/** ⚛️🔍 W14d: with the Artifact panel OPEN, where React keeps the navbar panel toggles, whether the
 * open panel's cap covers them, and what `elementsFromPoint` answers at each toggle centre. */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
const url = process.env.SEMIO_REACT_URL ?? "http://127.0.0.1:6013/?plugin=puzzle3d";
const out = process.env.SEMIO_W14D_OUT ?? "/tmp/w14d-panel-chip.json";
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--use-angle=metal"] });
const ctx = await browser.newContext({ viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
const page = await ctx.newPage();
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 180000 });
for (let s = 0; s < 420; s += 1) {
  await page.waitForTimeout(1000);
  const state = await page.evaluate(() => ({ ready: document.documentElement.getAttribute("data-semio-os-ready"), controls: document.querySelectorAll("[id],button").length })).catch(() => ({ ready: null, controls: 0 }));
  if (state.ready && state.controls > 40 && s > 4) { console.log(`BOOTED at ${s}s`); break; }
}
const snap = async (label) => page.evaluate((tag) => {
  const box = (el) => { if (!el) return null; const r = el.getBoundingClientRect(); return { id: el.id || null, slot: el.getAttribute?.("data-slot") || null, x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) }; };
  const ids = ["ui.introduction.skip", "framework.panel.artifact", "framework.panel.catalogue", "framework.panel.inspection", "framework.chat"];
  const toggles = ids.map((id) => ({ id, box: box(document.getElementById(id)) }));
  const openPanels = Array.from(document.querySelectorAll('[data-slot="panel"]')).map(box);
  const chains = toggles.filter((t) => t.box).map((t) => ({ id: t.id, chain: document.elementsFromPoint(t.box.x + t.box.w / 2, t.box.y + t.box.h / 2).slice(0, 5).map((el) => `${el.tagName.toLowerCase()}#${el.id || ""}[${el.getAttribute("data-slot") || ""}]`) }));
  return { tag, toggles, openPanels, chains };
}, label);
const report = { beforeDismiss: await snap("boot") };
await page.evaluate(() => document.getElementById("ui.introduction.skip")?.click());
await page.waitForTimeout(1500);
report.afterDismiss = await snap("tour-dismissed");
await page.evaluate(() => document.getElementById("framework.panel.artifact")?.click());
await page.waitForTimeout(2000);
report.artifactOpen = await snap("artifact-open");
await page.evaluate(() => document.getElementById("framework.panel.catalogue")?.click());
await page.waitForTimeout(2000);
report.catalogueOpen = await snap("catalogue-open");
writeFileSync(out, JSON.stringify(report, null, 2));
for (const [key, value] of Object.entries(report)) {
  console.log(`--- ${key} ---`);
  for (const t of value.toggles) console.log("  toggle", t.id, JSON.stringify(t.box));
  for (const p of value.openPanels) console.log("  panel", JSON.stringify(p));
  for (const c of value.chains) console.log("  at", c.id, c.chain.join(" > "));
}
await browser.close();
