/** 🌳️ wgpu DOCUMENT-RECONCILE probe — does the published document reach the paintable arena, and
 * does the arena paint?
 *
 * `🐍️wgpu-paint-probe.mjs` breaks out the moment `dumpStructure` reports a node, which is exactly the
 * signal the reconcile lane needed to see land — but nothing after it. This one runs the full window,
 * prints the whole structure dump (node paths, rects, kinds), the frame stats, and every
 * `renderSurface`/`invokeExtension`/`respond`/effects line, and screenshots on a cadence.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-reconcile/run-2 bun 🐍️wgpu-reconcile-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 60);
const shotEvery = Number(process.env.SEMIO_PROBE_SHOT_EVERY ?? 15);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-reconcile/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

const samples = [];
for (let second = 1; second <= seconds; second += 1) {
  await page.waitForTimeout(1000);
  const sample = await page
    .evaluate(async () => {
      const beacon = globalThis.semioWgpuIntrospection;
      let structure = null;
      let stats = null;
      if (typeof beacon?.dumpStructure === "function") {
        try { const raw = await beacon.dumpStructure(); structure = raw ? JSON.parse(raw) : null; } catch (error) { structure = { error: String(error) }; }
        try { const raw = await beacon.dumpFrameStats(); stats = raw ? JSON.parse(raw) : null; } catch (error) { stats = { error: String(error) }; }
      }
      return { structure, stats, alert: document.querySelector('[role="alert"]')?.textContent?.slice(0, 1200) ?? null };
    })
    .catch((error) => ({ error: String(error).slice(0, 300) }));
  sample.t = at();
  samples.push(sample);
  if (second % shotEvery === 0 || second === seconds) {
    await page.screenshot({ path: join(outDir, `shot-${String(second).padStart(3, "0")}s.png`), type: "png" }).catch(() => {});
  }
  if (sample.alert) { lines.push(`${at()} PROBE fault banner ${sample.alert}`); break; }
}

const last = samples.at(-1) ?? {};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify(samples, null, 2));
const nodes = last.structure?.nodes ?? [];
console.log(
  "DONE",
  JSON.stringify(
    {
      seconds: Math.round(at() / 1000),
      viewport: last.structure?.viewport ?? null,
      nodeCount: nodes.length,
      nodes: nodes.slice(0, 40).map((node) => ({ path: node.path, kind: node.kind, rect: node.rect, visible: node.visible })),
      stats: last.stats ?? null,
      alert: last.alert ?? null,
      effects: lines.filter((line) => line.includes("effects=")).slice(-6),
      invokes: lines.filter((line) => line.includes("invokeExtension") || line.includes("respond")).slice(-6),
    },
    null,
    2,
  ),
);
await browser.close();
