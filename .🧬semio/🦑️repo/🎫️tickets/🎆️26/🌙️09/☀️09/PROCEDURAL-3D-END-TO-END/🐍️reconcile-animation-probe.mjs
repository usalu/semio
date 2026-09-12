import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 💃️ The console-silent windows recalculate style ~475 times at ~21 ms with ZERO DOM mutations, so
// the invalidation source is a running animation, not a write. This samples `document.getAnimations()`
// alongside the renderer's own `RecalcStyleCount`, so the animation that keeps the document dirty
// names itself and its element.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 130);
const outDir = join(import.meta.dir, "🗑️generated", "reconcile-silence", process.env.SEMIO_PROBE_OUT ?? "animation");
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
await context.addInitScript(() => { try { window.localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const page = await context.newPage();
const t0 = performance.now();
page.on("console", (msg) => lines.push(`${(performance.now() - t0).toFixed(2)} ${msg.type()} ${msg.text().slice(0, 2000)}`));
const cdp = await context.newCDPSession(page);
await cdp.send("Performance.enable", { timeDomain: "timeTicks" });
await page.goto(url, { waitUntil: "domcontentloaded" });
const rows = [];
const deadline = performance.now() + seconds * 1000;
let previous = null;
while (performance.now() < deadline) {
  const { metrics } = await cdp.send("Performance.getMetrics");
  const m = Object.fromEntries(metrics.map((x) => [x.name, x.value]));
  let animations = [];
  try {
    animations = await page.evaluate(() => document.getAnimations().map((a) => {
      const effect = a.effect;
      const target = effect && "target" in effect ? effect.target : null;
      const describe = (el) => el ? `<${el.tagName.toLowerCase()}${el.id ? "#" + el.id : ""}${typeof el.className === "string" && el.className ? "." + el.className.split(/\s+/).slice(0, 4).join(".") : ""}>` : "(none)";
      return `${a.constructor.name}:${a.animationName ?? a.transitionProperty ?? "?"}:${a.playState}:${describe(target)}`;
    }));
  } catch {}
  const tally = animations.reduce((acc, k) => { acc[k] = (acc[k] ?? 0) + 1; return acc; }, {});
  const row = { t: Number((performance.now() - t0).toFixed(0)), recalcs: m.RecalcStyleCount, recalcMs: Number((m.RecalcStyleDuration * 1000).toFixed(0)), scriptMs: Number((m.ScriptDuration * 1000).toFixed(0)), animations: animations.length, tally };
  if (previous) { row.dRecalcs = row.recalcs - previous.recalcs; row.dRecalcMs = row.recalcMs - previous.recalcMs; }
  previous = row;
  rows.push(row);
  await new Promise((r) => setTimeout(r, 500));
}
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "animations.jsonl"), rows.map((r) => JSON.stringify(r)).join("\n"));
console.log("DONE lines", lines.length, "rows", rows.length);
await browser.close();
