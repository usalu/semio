/** 🔎 How the Flow window's accessible outline rows are actually marked up. */
import { chromium } from "playwright";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const b = await chromium.launch({ headless: true });
const p = await b.newPage({ viewport: { width: 1600, height: 1000 } });
await p.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 150; i++) { await p.waitForTimeout(1000); if (await p.locator('[data-surface-id="window:procedural-main"]').count()) break; }
await p.waitForTimeout(9000);
const d = await p.evaluate(() => {
  const win = document.querySelector('[data-slot="window"][id="procedural-main"]');
  const outline = document.getElementById("window:procedural-main/procedural-play-graph") ?? [...(win?.querySelectorAll('[id*="procedural-play-graph"]') ?? [])][0];
  return {
    winFound: Boolean(win),
    outlineId: outline?.id ?? null,
    roles: [...new Set([...(win?.querySelectorAll("[role]") ?? [])].map((e) => e.getAttribute("role")))],
    idsUnderOutline: [...(outline?.querySelectorAll("[id]") ?? [])].map((e) => ({ id: e.id, role: e.getAttribute("role"), slot: e.getAttribute("data-slot"), t: (e.textContent||"").replace(/\s+/g," ").trim().slice(0,28) })).slice(0, 16),
    graphIds: [...(win?.querySelectorAll('[id*="procedural-play-graph"]') ?? [])].map((e) => e.id).slice(0, 12),
  };
});
console.log(JSON.stringify(d, null, 1).slice(0, 4000));
await b.close();
