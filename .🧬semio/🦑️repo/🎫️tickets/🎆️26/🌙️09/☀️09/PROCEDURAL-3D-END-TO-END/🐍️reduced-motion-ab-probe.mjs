import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
const url = "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 150);
const outDir = join(import.meta.dir, "🗑️generated", "reconcile-silence", process.env.SEMIO_PROBE_OUT);
mkdirSync(outDir, { recursive: true });
const lines = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, reducedMotion: process.env.SEMIO_REDUCED === "1" ? "reduce" : "no-preference" });
await context.addInitScript(() => { try { window.localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const page = await context.newPage();
const t0 = performance.now();
page.on("console", (m) => lines.push(`${(performance.now() - t0).toFixed(2)} ${m.type()} ${m.text().slice(0, 2000)}`));
const cdp = await context.newCDPSession(page);
await cdp.send("Performance.enable", { timeDomain: "timeTicks" });
await page.goto(url, { waitUntil: "domcontentloaded" });
const read = () => page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => ({ surfaceId: el.getAttribute("data-surface-id"), status: el.getAttribute("data-status-json") ?? "", meshes: (() => { try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); return Array.isArray(v) ? v.length : 0; } catch { return 0; } })() })));
let allOkMs = null, meshesMs = null, meshesSeen = 0;
const deadline = performance.now() + seconds * 1000;
while (performance.now() < deadline) {
  await page.waitForTimeout(250);
  let s = []; try { s = await read(); } catch { continue; }
  const main = s.find((x) => x.surfaceId === "window:procedural-main");
  if (allOkMs === null && main && main.status) { try { const n = Object.values(JSON.parse(main.status)); if (n.length >= 7 && n.every((x) => x && x.status === "ok")) allOkMs = Number((performance.now() - t0).toFixed(2)); } catch {} }
  const best = Math.max(0, ...s.map((x) => x.meshes));
  if (best > meshesSeen) { meshesSeen = best; if (meshesMs === null && best >= 3) meshesMs = Number((performance.now() - t0).toFixed(2)); }
  if (allOkMs !== null && meshesMs !== null) break;
}
const { metrics } = await cdp.send("Performance.getMetrics");
const m = Object.fromEntries(metrics.map((x) => [x.name, x.value]));
const anims = await page.evaluate(() => document.getAnimations().length);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
const out = { reduced: process.env.SEMIO_REDUCED === "1", allOkMs, meshesMs, meshesSeen, animations: anims, recalcCount: m.RecalcStyleCount, recalcMs: Math.round(m.RecalcStyleDuration * 1000), scriptMs: Math.round(m.ScriptDuration * 1000), taskMs: Math.round(m.TaskDuration * 1000) };
writeFileSync(join(outDir, "convergence.json"), JSON.stringify(out, null, 2));
console.log("RESULT", JSON.stringify(out));
await browser.close();
