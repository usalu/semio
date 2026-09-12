import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

// 📈️ Attributes the console-silent windows to the renderer's own accounting buckets. The sampling
// profile parks 95 % of the silence in `(program)`, which is not a frame — `Performance.getMetrics`
// splits the same wall clock into ScriptDuration / RecalcStyleDuration / LayoutDuration /
// TaskDuration / GC, so the silence names its own owner.
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 130);
const outDir = join(import.meta.dir, "🗑️generated", "reconcile-silence", process.env.SEMIO_PROBE_OUT ?? "metrics");
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
const samples = [];
const deadline = performance.now() + seconds * 1000;
while (performance.now() < deadline) {
  const { metrics } = await cdp.send("Performance.getMetrics");
  const row = { t: Number((performance.now() - t0).toFixed(1)) };
  for (const m of metrics) row[m.name] = m.value;
  samples.push(row);
  await new Promise((r) => setTimeout(r, 250));
}
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "metrics.jsonl"), samples.map((s) => JSON.stringify(s)).join("\n"));
console.log("DONE lines", lines.length, "samples", samples.length);
await browser.close();
