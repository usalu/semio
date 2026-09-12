/** ⛓️ wgpu MESH-CHAIN probe — does the `flowEvalTick` reply decode, and does the chain it starts
 * reach meshes on the wgpu World3d surface?
 *
 * `🐍️wgpu-reconcile-probe.mjs` proves the document paints. This one follows the chain PAST the
 * command reply: the typed-operation result pages the reply carries, the `invokeExtension`/`respond`
 * round trips, the `flowEvalResolve`/`flowTessellateResolve` completions, the published mesh counts
 * and every flow node's status. It classifies the console itself so a run's verdict is the log, not
 * a reading of it.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-chain/run-1 bun 🐍️wgpu-chain-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const shotEvery = Number(process.env.SEMIO_PROBE_SHOT_EVERY ?? 30);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-chain/run");
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

const has = (needle) => lines.filter((line) => line.includes(needle));
const last = samples.at(-1) ?? {};
const nodes = last.structure?.nodes ?? [];
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "samples.json"), JSON.stringify(samples, null, 2));
const verdict = {
  seconds: Math.round(at() / 1000),
  alert: last.alert ?? null,
  viewport: last.structure?.viewport ?? null,
  nodeCount: nodes.length,
  scenes: nodes.filter((node) => String(node.kind ?? "").toLowerCase().includes("scene")).map((node) => ({ path: node.path, rect: node.rect })),
  stats: last.stats ?? null,
  counts: {
    unknownTag115: has("unknown tag 115").length,
    decodeAppFrameFailures: has("decodeAppFrame:").length,
    deferredActionFailed: has("deferred action").filter((line) => line.includes("failed")).length,
    typedOperation: has("typed-operation").length,
    invokeExtensionDispatch: has("invokeExtension dispatch").length,
    invokeExtensionFaulted: has("invokeExtension faulted").length,
    extensionRequestAnswered: has("extension request answered").length,
    extensionCompletionSubmitted: has("extension completion submitted").length,
    flowEvalTick: has("flowEvalTick").length,
    flowEvalResolve: has("flowEvalResolve").length,
    flowTessellateResolve: has("flowTessellateResolve").length,
    meshes: has("meshes").length,
  },
  typedOperationLines: has("typed-operation").slice(-10),
  invokeLines: [...has("invokeExtension"), ...has("extension request answered"), ...has("extension completion submitted")].slice(-16),
  resolveLines: [...has("flowEvalResolve"), ...has("flowTessellateResolve")].slice(-10),
  meshLines: has("meshes").slice(-10),
  failureLines: lines.filter((line) => line.includes("failed") || line.includes("error") || line.includes("pageerror")).slice(-14),
};
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log("DONE", JSON.stringify(verdict, null, 2));
await browser.close();
