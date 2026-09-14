/** 🧷 Page-fault STACK probe: the React battery records `String(error)` only, so the recurring
 * `TypeError: Cannot read properties of null (reading 'addEventListener')` and `FlowMessageRejected`
 * have no call site. This one records `error.stack` plus the console lines around each fault, and
 * exercises the surfaces the fault correlates with (node-graph mount/unmount via role + mode switches).
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6022/?plugin=generation3d SEMIO_PROBE_OUT=react-oracle/fault-stacks bun 🐍️page-fault-stack-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-oracle/fault-stacks");
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
mkdirSync(outDir, { recursive: true });
const lines = [];
const faults = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => {
  faults.push({ t: Date.now() - t0, name: e?.name ?? null, message: String(e?.message ?? e).slice(0, 400), stack: String(e?.stack ?? "").slice(0, 4000), tail: lines.slice(-8) });
  lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 400)}`);
});
await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(25_000);
for (const chord of ["Meta+Alt+ArrowRight", "Meta+Alt+ArrowLeft", "Meta+Alt+V", "Meta+Alt+V"]) {
  await page.keyboard.press(chord);
  await page.waitForTimeout(6000);
}
await page.waitForTimeout(Math.max(0, seconds * 1000 - (Date.now() - t0)));
writeFileSync(join(outDir, "faults.json"), JSON.stringify(faults, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] DONE faults=${faults.length}`);
for (const f of faults) console.log(`[DEBUG] fault ${f.t} ${f.message}\n${f.stack}`);
await browser.close();
