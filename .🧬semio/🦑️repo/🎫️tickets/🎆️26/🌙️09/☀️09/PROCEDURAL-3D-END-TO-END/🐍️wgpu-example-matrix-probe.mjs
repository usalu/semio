/** 🧮️ wgpu EXAMPLE-MATRIX probe — every bundled example of the open dialect, in every boot lane, each
 * one booted DIRECTLY by url rather than through a pointer gesture.
 *
 * The lanes, one page load each (`?plugin=generation3d&example=<id>` plus the lane's own axes):
 *   • `edit`   — edit mode, editor role   (`?mode=edit`)
 *   • `viewer` — the viewer surface       (`?role=viewer`)
 *   • `generate` — generate mode's preview (`?mode=generate`)
 *
 * `?example=` is the axis this lane added (`🧑‍💻dev/🔗️boot-query/🟦️.ts` `resolveBootQueryExampleId` →
 * wgpu `bootDescriptor` → `semioWgpuSetBootExample` → `ShellState::sync_session_chrome` →
 * `apply_boot_example`'s `setActiveExample` dispatch), so no example needs the picker to be clickable —
 * pointer input on 6118 is a different lane's and is deliberately NOT relied on here.
 *
 * CONVERGENCE is the pair `📓️wgpu-resident-budget-settle-2026-09-12.md` §6.1 proved the hexagonal
 * column with, and nothing weaker — all four must hold, stable across two samples:
 *   1. the frame ledger reports `scenePasses > 0` for some window;
 *   2. the renderer's own per-surface World3d census carries `state-meshes > 0` AND a drawable —
 *      `instances > 0` for a solid body, `lines > 0` for a wire one;
 *   3. at least one published mesh carries REAL geometry: a `positions` or `edgePositions` array that
 *      actually starts with a number. generation3d's preview always ships a `…@wire#0` companion whose
 *      `positions` are empty, so "a mesh list that is not `[]`" is not evidence of a rendered body and
 *      was rejected as a predicate; symmetrically `rectangle-wire-preview` is a WIRE example whose only
 *      geometry is `edgePositions` with `instances=0`, so demanding `positions`/`sceneInstances` would
 *      have failed a body that is on screen;
 *   4. no `[role=alert]` is up.
 * The guest's own `facesDone/facesTotal` and `ratio` are recorded beside the verdict. TIME-TO-MESH is
 * measured from `page.goto` to the first sample that answers, so it is comparable with the React journey
 * probe's own per-example seconds.
 *
 * The wgpu tick is INPUT-DRIVEN, so every wait nudges the pointer 1 px rather than sleeping — a bare
 * `waitForTimeout` measures a renderer that was never asked to run.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-examples/matrix bun 🐍️wgpu-example-matrix-probe.mjs
 *   SEMIO_PROBE_LANES=edit,viewer,generate   SEMIO_PROBE_EXAMPLES=<id>,<id>   SEMIO_PROBE_BUDGET=<seconds>
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const outRoot = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-examples/matrix");
const budgetSeconds = Number(process.env.SEMIO_PROBE_BUDGET ?? 150);
const viewport = { width: 1440, height: 900 };

/** 📚️ The eight bundled examples of `s.procedural.generation3d@1/*`, by their own `pub const ID`. */
const ALL_EXAMPLES = ["hexagonal-mushroom-column", "rectangle-extrude-volume", "rectangle-wire-preview", "box-shell-preview", "box-fillet-preview", "sphere-cut-with-torus", "sphere-box-fuse", "face-sweep-extrude"];

/** 🛣️ One lane = one url shape. `generate` opens the three-pane generate layout, whose preview window
 * is the surface `addGeneration` publishes into. */
const LANES = {
  edit: (example) => `${origin}/?plugin=generation3d&mode=edit&example=${encodeURIComponent(example)}`,
  viewer: (example) => `${origin}/?plugin=generation3d&role=viewer&example=${encodeURIComponent(example)}`,
  generate: (example) => `${origin}/?plugin=generation3d&mode=generate&example=${encodeURIComponent(example)}`,
};

const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? ALL_EXAMPLES.join(",")).split(",").filter(Boolean);
const lanes = (process.env.SEMIO_PROBE_LANES ?? "edit,viewer,generate").split(",").filter(Boolean);
mkdirSync(outRoot, { recursive: true });

const results = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });

for (const lane of lanes) {
  for (const example of examples) {
    const outDir = join(outRoot, example, lane);
    mkdirSync(outDir, { recursive: true });
    const lines = [];
    const t0 = Date.now();
    const at = () => Date.now() - t0;
    const page = await browser.newPage({ viewport });
    page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
    page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 1200)}`));

    const has = (needle) => lines.filter((line) => line.includes(needle));

    /** 🌍️ The renderer's own per-surface World3d census (`🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs`): how many
     * instances and meshes the scene actually holds, and the guest's own published status. */
    const world3dTraces = () => {
      const byId = {};
      for (const line of has("world3d surface=")) {
        const surface = /world3d surface=(\S+)/.exec(line)?.[1];
        if (!surface) continue;
        byId[surface] = {
          instances: Number(/ instances=(\d+)/.exec(line)?.[1] ?? 0),
          draws: Number(/ draws=(\d+)/.exec(line)?.[1] ?? 0),
          lines: Number(/ lines=(\d+)/.exec(line)?.[1] ?? 0),
          stateMeshes: Number(/ state-meshes=(\d+)/.exec(line)?.[1] ?? 0),
          meshesHead: /meshesHead="((?:[^"\\]|\\.)*)"/.exec(line)?.[1] ?? null,
          status: /status=Some\("((?:[^"\\]|\\.)*)"\)/.exec(line)?.[1] ?? null,
        };
      }
      return byId;
    };

    /** 📐️ A mesh with an EMPTY `positions` array is a published entry, not a rendered body — the
     * generation3d preview always carries a `@wire` companion whose arrays are empty. Convergence
     * therefore demands at least one mesh whose positions array actually starts with a number. */
    const geometryEvidence = () => {
      for (const line of has("world3d surface=")) {
        const solid = /\\"positions\\":\[-?\d/.test(line);
        const wire = /\\"edgePositions\\":\[-?\d/.test(line);
        if (!solid && !wire) continue;
        const head = line.indexOf("meshesHead");
        return { body: solid ? "solid" : "wire", head: head < 0 ? null : line.slice(head, head + 180) };
      }
      return null;
    };

    /** 🧮️ The guest's OWN convergence, as it publishes it: `facesDone/facesTotal` and `ratio`. */
    const guestProgress = () => {
      const lines2 = has("facesDone");
      const last = lines2.at(-1);
      if (!last) return null;
      const done = Number(/facesDone\\":(\d+)/.exec(last)?.[1] ?? -1);
      const total = Number(/facesTotal\\":(\d+)/.exec(last)?.[1] ?? -1);
      const ratio = Number(/ratio\\":([\d.]+)/.exec(last)?.[1] ?? -1);
      return { facesDone: done, facesTotal: total, ratio };
    };

    const dump = (windowId) =>
      page
        .evaluate(async (id) => {
          const beacon = globalThis.semioWgpuIntrospection;
          if (typeof beacon?.dumpStructure !== "function") return null;
          const parse = async (call) => {
            try {
              const raw = await call();
              return raw ? JSON.parse(raw) : null;
            } catch {
              return null;
            }
          };
          return { structure: await parse(() => beacon.dumpStructure(id)), stats: await parse(() => beacon.dumpFrameStats(id)) };
        }, windowId)
        .catch(() => null);

    const domSignals = () =>
      page
        .evaluate(() => ({
          status: document.querySelector('[role="status"]')?.textContent?.trim() ?? null,
          alert: document.querySelector('[role="alert"]')?.textContent?.trim()?.slice(0, 300) ?? null,
        }))
        .catch(() => ({ status: null, alert: null }));

    const measure = async () => {
      const ids = (await dump(undefined))?.structure?.windowIds ?? [];
      let sceneInstances = 0;
      let sceneDraws = 0;
      let scenePasses = 0;
      let quadCount = 0;
      for (const id of ids) {
        const stats = (await dump(id))?.stats ?? null;
        sceneInstances = Math.max(sceneInstances, stats?.sceneInstances ?? 0);
        sceneDraws = Math.max(sceneDraws, stats?.sceneDraws ?? 0);
        scenePasses = Math.max(scenePasses, stats?.scenePasses ?? 0);
        quadCount = Math.max(quadCount, stats?.quadCount ?? 0);
      }
      const dom = await domSignals();
      const traces = world3dTraces();
      const geometry = geometryEvidence();
      const progress = guestProgress();
      const meshy = Object.entries(traces).filter(([, trace]) => trace.stateMeshes > 0 && (trace.instances > 0 || trace.lines > 0));
      return {
        windowIds: ids,
        sceneInstances,
        sceneDraws,
        scenePasses,
        quadCount,
        dom,
        progress,
        geometry,
        meshSurfaces: meshy.map(([id, trace]) => ({ surface: id, instances: trace.instances, lines: trace.lines, meshes: trace.stateMeshes, draws: trace.draws })),
        converged: scenePasses > 0 && meshy.length > 0 && geometry !== null && !dom.alert,
      };
    };

    const url = LANES[lane](example);
    await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 300)}`));

    let sample = { windowIds: [], sceneInstances: 0, sceneDraws: 0, scenePasses: 0, quadCount: 0, dom: { status: null, alert: null }, progress: null, geometry: null, meshSurfaces: [], converged: false };
    let stable = 0;
    let firstConvergedMs = null;
    let flip = 0;
    for (let elapsed = 0; elapsed < budgetSeconds && stable < 2; elapsed += 1) {
      for (let nudge = 0; nudge < 5; nudge += 1) {
        await page.waitForTimeout(200);
        flip = 1 - flip;
        await page.mouse.move(3 + flip, 3).catch(() => {});
      }
      sample = await measure();
      if (sample.converged) {
        firstConvergedMs ??= at();
        stable += 1;
      } else stable = 0;
    }

    const bootExample = has("wgpu-shell boot example").at(-1) ?? null;
    const row = {
      example,
      lane,
      url,
      verdict: sample.converged ? "pass" : "fail",
      timeToMeshSeconds: firstConvergedMs === null ? null : Math.round(firstConvergedMs / 10) / 100,
      elapsedSeconds: Math.round(at() / 1000),
      windowIds: sample.windowIds,
      sceneInstances: sample.sceneInstances,
      sceneDraws: sample.sceneDraws,
      scenePasses: sample.scenePasses,
      quadCount: sample.quadCount,
      guestProgress: sample.progress,
      geometry: sample.geometry,
      meshSurfaces: sample.meshSurfaces,
      status: sample.dom.status,
      alert: sample.dom.alert,
      bootExampleTrace: bootExample ? bootExample.slice(bootExample.indexOf("[DEBUG]")) : null,
      renderBegin: has("wgpu-shell render begin").length,
      renderLeave: has("wgpu-shell render leave").length,
      capacity: has("Capacity").length,
      faults: has("invokeExtension faulted").length,
    };
    results.push(row);
    console.log(`[DEBUG] ${lane}/${example}: ${row.verdict} t=${row.timeToMeshSeconds}s meshes=${JSON.stringify(row.meshSurfaces)} progress=${JSON.stringify(row.guestProgress)} quads=${row.quadCount} status=${JSON.stringify(row.status)} alert=${JSON.stringify(row.alert)} bootExample=${JSON.stringify(row.bootExampleTrace)}`);
    await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
    writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
    writeFileSync(join(outDir, "result.json"), JSON.stringify(row, null, 2));
    await page.close();
  }
}

writeFileSync(join(outRoot, "results.json"), JSON.stringify({ origin, lanes, examples, results }, null, 2));
await browser.close();
console.log("DONE", JSON.stringify({ pass: results.filter((row) => row.verdict === "pass").length, total: results.length, out: outRoot }, null, 2));
