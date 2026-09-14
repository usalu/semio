/** 🫀️ wgpu SETTLE-PUMP probe — boot-before-convergence, pump liveness and refresh cost, in one run.
 *
 * Answers the three questions the settle-pump lane is scored on, off the shell's own `[DEBUG]` traces
 * (screenshots are blank on 6118: the canvas is an OffscreenCanvas owned by the frame Worker):
 *
 *   1. does the boot example converge INSIDE `boot_shell`? — `boot_shell leave`, the first chrome
 *      publication, the first status pill and the first mesh publication, on one clock;
 *   2. does the pump run? — `wgpu-shell settle pump {step}` transitions, and any `settle pump wedge`;
 *   3. what does a converging edit now cost? — `render begin` / `refresh scope=` counts, and the
 *      scopes the guest declared.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-settle/boot-after bun 🐍️wgpu-settle-pump-probe.mjs
 *   SEMIO_PROBE_EXAMPLE=sphere-cut-with-torus   boot straight into an example (the §8.1 axis)
 *   SEMIO_PROBE_SECONDS=120                     how long to watch
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const example = process.env.SEMIO_PROBE_EXAMPLE ?? "";
const url = process.env.SEMIO_PROBE_URL ?? `http://127.0.0.1:6118/?plugin=generation3d${example ? `&example=${example}` : ""}`;
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 120);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-settle/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

const meshes = [];
for (let second = 1; second <= seconds; second += 1) {
  await page.waitForTimeout(1000);
  const sample = await page
    .evaluate(async () => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      try {
        const raw = await beacon.dumpStructure("procedural-preview");
        const structure = raw ? JSON.parse(raw) : null;
        return { nodes: structure?.nodes?.length ?? 0 };
      } catch (error) {
        return { error: String(error).slice(0, 200) };
      }
    })
    .catch(() => null);
  meshes.push({ t: at(), ...(sample ?? {}) });
}

const text = lines.join("\n");
const stamp = (needle) => {
  const hit = lines.find((line) => line.includes(needle));
  return hit ? Number(hit.split(" ")[0]) : null;
};
const count = (needle) => lines.filter((line) => line.includes(needle)).length;
const pumpSteps = lines.filter((line) => line.includes("wgpu-shell settle pump {")).map((line) => line.slice(line.indexOf("{")));
const refreshScopes = lines.filter((line) => line.includes("wgpu-shell refresh scope=")).map((line) => line.slice(line.indexOf("scope=")));
const pills = lines.filter((line) => line.includes("world3d status pill"));

const report = {
  url,
  seconds,
  pageErrors: count("pageerror"),
  // 🚧️ The coordinator's 19:05 broadcast: a guest whose contributions pack no longer fits the command
  // ingress boots with no `brep` flow extension at all, so nothing converges and every number below
  // would be measuring that instead of this lane.
  ingressOverflow: count("exceeds 64 pages"),
  contributionsFailed: count("setContributions command failed"),
  brepMissing: count("contributes the flow extension"),
  boot: {
    bootShellLeaveMs: stamp("boot_shell leave"),
    firstChromeMs: stamp("wgpu-shell render leave surface=procedural-main"),
    firstPreviewRenderMs: stamp("wgpu-shell render begin surface=procedural-preview"),
    firstStatusPillMs: stamp("world3d status pill"),
    firstTessellateMs: stamp("tessellate"),
    firstToolRunStartMs: stamp("toolRunStart"),
  },
  pump: {
    steps: pumpSteps.length,
    firstStepMs: stamp("wgpu-shell settle pump {"),
    wedges: count("settle pump wedge"),
    transitions: pumpSteps,
  },
  cost: {
    renderBegin: count("wgpu-shell render begin"),
    renderBeginPreview: count("wgpu-shell render begin surface=procedural-preview"),
    renderBeginMain: count("wgpu-shell render begin surface=procedural-main"),
    refreshPasses: refreshScopes.length,
    refreshScopes: refreshScopes.slice(0, 200),
    flowEvalTickSettled: count("flowEvalTick settled"),
    frameBuildSuperseded: count("frame build superseded"),
    revisionStale: count("revision is stale"),
  },
  pills: pills.slice(0, 40),
  meshes,
};

writeFileSync(join(outDir, "console.txt"), text);
writeFileSync(join(outDir, "report.json"), `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify({ ...report, pump: { ...report.pump, transitions: report.pump.transitions.slice(0, 12) }, cost: { ...report.cost, refreshScopes: report.cost.refreshScopes.slice(0, 12) }, pills: report.pills.slice(0, 4), meshes: undefined }, null, 2));
await browser.close();
