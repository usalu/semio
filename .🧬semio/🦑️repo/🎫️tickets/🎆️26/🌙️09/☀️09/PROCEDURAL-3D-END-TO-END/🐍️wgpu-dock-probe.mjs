/** 🛰️ wgpu DOCK-LAYOUT probe — does the shell lay out EVERY window instance of the active mode, and
 * does the `procedural-preview` World3d surface paint the meshes the chain delivers?
 *
 * `🐍️wgpu-chain-probe.mjs` proves the extension chain reaches the guest. This one reads the dock:
 * the per-window arena subtrees, the per-window `dumpFrameStats` (which now takes an optional window
 * id), the engine scene nodes and their rects, and the window ids the shell says it rendered. It
 * classifies the console itself so a run's verdict is the log, not a reading of it.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-dock/run-1 bun 🐍️wgpu-dock-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 90);
const shotEvery = Number(process.env.SEMIO_PROBE_SHOT_EVERY ?? 30);
const windows = (process.env.SEMIO_PROBE_WINDOWS ?? "procedural-main,procedural-preview").split(",").filter(Boolean);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-dock/run");
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
    .evaluate(async (ids) => {
      const beacon = globalThis.semioWgpuIntrospection;
      const parse = async (call) => {
        try {
          const raw = await call();
          return raw ? JSON.parse(raw) : null;
        } catch (error) {
          return { error: String(error) };
        }
      };
      if (typeof beacon?.dumpStructure !== "function") return { structure: null, stats: null, perWindow: null, alert: null };
      const structure = await parse(() => beacon.dumpStructure());
      const stats = await parse(() => beacon.dumpFrameStats());
      const perWindow = {};
      for (const id of ids) {
        perWindow[id] = {
          structure: await parse(() => beacon.dumpStructure(id)),
          stats: await parse(() => beacon.dumpFrameStats(id)),
        };
      }
      return { structure, stats, perWindow, alert: document.querySelector('[role="alert"]')?.textContent?.slice(0, 1200) ?? null };
    }, windows)
    .catch((error) => ({ error: String(error).slice(0, 300) }));
  sample.t = at();
  samples.push(sample);
  if (second % shotEvery === 0 || second === seconds) {
    await page.screenshot({ path: join(outDir, `shot-${String(second).padStart(3, "0")}s.png`), type: "png" }).catch(() => {});
  }
  if (sample.alert) {
    lines.push(`${at()} PROBE fault banner ${sample.alert}`);
    break;
  }
}

const has = (needle) => lines.filter((line) => line.includes(needle));
const last = samples.at(-1) ?? {};
const nodes = last.structure?.nodes ?? [];
const sceneNodes = (list) => (list ?? []).filter((node) => String(node.kind ?? "").toLowerCase().includes("scene")).map((node) => ({ path: node.path, rect: node.rect }));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify(samples.slice(-6), null, 2));
const perWindow = {};
for (const id of windows) {
  const entry = last.perWindow?.[id] ?? {};
  perWindow[id] = {
    viewport: entry.structure?.viewport ?? null,
    nodeCount: entry.structure?.nodes?.length ?? 0,
    scenes: sceneNodes(entry.structure?.nodes),
    stats: entry.stats ?? null,
    paths: (entry.structure?.nodes ?? []).slice(0, 8).map((node) => `${node.path} ${JSON.stringify(node.rect)}`),
  };
}
const verdict = {
  url,
  seconds: Math.round(at() / 1000),
  alert: last.alert ?? null,
  primary: { viewport: last.structure?.viewport ?? null, nodeCount: nodes.length, stats: last.stats ?? null },
  perWindow,
  renderedSurfaces: [...new Set(has("wgpu-shell render begin").map((line) => line.split("surface=")[1]?.split(" ")[0]))],
  counts: {
    dockPlan: has("wgpu-shell dock plan").length,
    world3dSurface: has("world3d surface").length,
    unknownTag115: has("unknown tag 115").length,
    invokeExtensionFaulted: has("invokeExtension faulted").length,
    extensionRequestAnswered: has("extension request answered").length,
    paintFault: has("ui-doc paint fault").length,
  },
  dockLines: has("wgpu-shell dock plan").slice(-8),
  worldLines: has("world3d").slice(-12),
  failureLines: lines.filter((line) => line.includes("failed") || line.includes("pageerror") || line.includes("panicked")).slice(-14),
};
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log("DONE", JSON.stringify(verdict, null, 2));
await browser.close();
