/** ➕️ GENERATE-MODE probe — boots one example straight into the three-pane generate layout
 * (`?plugin=generation3d&mode=generate&example=<id>`), fires `Add Generation`, and measures whether the
 * generation's own preview window converges to a rendered mesh.
 *
 * The row is NEVER clicked at a guessed pixel. Its page rect is derived from the two published sources
 * the way `🐍️wgpu-hit-probe.mjs:62-100` proved: the retained body's own `mounted_layout` rect from
 * `dumpStructure(windowId)`, offset by that window's rect in the shell's own `[DEBUG] wgpu-shell dock
 * plan` trace. Every click carries the pointer-delivery witness, so "the shell did not dispatch" is
 * never confused with "the browser delivered no event".
 *
 * A generation's preview has converged on exactly the evidence
 * `🐍️wgpu-example-matrix-probe.mjs` uses: a scene pass, a census carrying meshes and a drawable, and a
 * published mesh whose `positions` or `edgePositions` array actually starts with a number.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_EXAMPLES=<id>,<id> SEMIO_PROBE_OUT=wgpu-examples/generate bun 🐍️wgpu-add-generation-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const origin = process.env.SEMIO_PROBE_ORIGIN ?? "http://127.0.0.1:6118";
const outRoot = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-examples/generate");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 60);
const afterSeconds = Number(process.env.SEMIO_PROBE_AFTER ?? 120);
const rowNeedle = process.env.SEMIO_PROBE_ROW ?? "addGeneration";
const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? "hexagonal-mushroom-column").split(",").filter(Boolean);
const viewport = { width: 1440, height: 900 };
mkdirSync(outRoot, { recursive: true });

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const results = [];

for (const example of examples) {
  const outDir = join(outRoot, example);
  mkdirSync(outDir, { recursive: true });
  const lines = [];
  const t0 = Date.now();
  const at = () => Date.now() - t0;
  const page = await browser.newPage({ viewport });
  page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
  page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 1200)}`));
  const has = (needle) => lines.filter((line) => line.includes(needle));

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

  const dockPlan = () => {
    const line = has("wgpu-shell dock plan").at(-1);
    if (!line) return {};
    const plan = {};
    for (const token of line.split(" ").slice(1)) {
      const match = /^(.+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)$/.exec(token);
      if (match) plan[match[1]] = { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) };
    }
    return plan;
  };

  const world3dTraces = () => {
    const byId = {};
    for (const line of has("world3d surface=")) {
      const surface = /world3d surface=(\S+)/.exec(line)?.[1];
      if (!surface) continue;
      byId[surface] = {
        instances: Number(/ instances=(\d+)/.exec(line)?.[1] ?? 0),
        lines: Number(/ lines=(\d+)/.exec(line)?.[1] ?? 0),
        stateMeshes: Number(/ state-meshes=(\d+)/.exec(line)?.[1] ?? 0),
      };
    }
    return byId;
  };

  const geometryEvidence = (surfaceNeedle) => {
    for (const line of has("world3d surface=")) {
      if (surfaceNeedle && !line.includes(surfaceNeedle)) continue;
      if (/\\"positions\\":\[-?\d/.test(line) || /\\"edgePositions\\":\[-?\d/.test(line)) return line.slice(line.indexOf("meshesHead"), line.indexOf("meshesHead") + 180);
    }
    return null;
  };

  const pump = async (seconds, park = [3, 3]) => {
    for (let tick = 0; tick < seconds * 5; tick += 1) {
      await page.waitForTimeout(200);
      await page.mouse.move(park[0] + (tick % 2), park[1]).catch(() => {});
    }
  };

  const converged = async (surfaceNeedle) => {
    const ids = (await dump(undefined))?.structure?.windowIds ?? [];
    let scenePasses = 0;
    for (const id of ids) scenePasses = Math.max(scenePasses, (await dump(id))?.stats?.scenePasses ?? 0);
    const traces = world3dTraces();
    const meshy = Object.entries(traces).filter(([id, trace]) => (!surfaceNeedle || id.includes(surfaceNeedle)) && trace.stateMeshes > 0 && (trace.instances > 0 || trace.lines > 0));
    const geometry = geometryEvidence(surfaceNeedle);
    const alert = await page.evaluate(() => document.querySelector('[role="alert"]')?.textContent?.trim()?.slice(0, 200) ?? null).catch(() => null);
    return { scenePasses, meshy: meshy.map(([id, trace]) => ({ surface: id, ...trace })), geometry, alert, ok: scenePasses > 0 && meshy.length > 0 && geometry !== null && !alert };
  };

  const url = `${origin}/?plugin=generation3d&mode=generate&example=${encodeURIComponent(example)}`;
  await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 300)}`));
  await pump(settleSeconds);

  const windowIds = (await dump(undefined))?.structure?.windowIds ?? [];
  const plan = dockPlan();
  lines.push(`${at()} PROBE windowIds ${JSON.stringify(windowIds)} dockPlan ${JSON.stringify(plan)}`);

  /** 🎯️ Every retained node whose published path or text names the row, in PAGE coordinates. */
  const targets = [];
  for (const id of windowIds) {
    const body = plan[id];
    const sample = await dump(id);
    for (const node of sample?.structure?.nodes ?? []) {
      const path = String(node.path ?? "");
      const text = String(node.text ?? "");
      const matches = path.includes(rowNeedle) || text.toLowerCase().includes("add generation");
      if (!matches || !body) continue;
      const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
      if (rw <= 0 || rh <= 0) continue;
      targets.push({ windowId: id, path, text: node.text ?? null, rect: node.rect, page: [body.x + rx + rw / 2, body.y + ry + rh / 2] });
    }
  }
  lines.push(`${at()} PROBE targets ${JSON.stringify(targets)}`);

  await page.evaluate(() => {
    const canvas = document.querySelector("canvas") ?? document.body;
    globalThis.__semioProbePointer = { downs: 0 };
    canvas.addEventListener("pointerdown", () => { globalThis.__semioProbePointer.downs += 1; }, true);
  }).catch(() => {});

  const before = await converged("generate-preview");
  const generationsBefore = has("addGeneration").length;
  let clicked = null;
  if (targets.length > 0) {
    const point = targets[0].page;
    const downsBefore = await page.evaluate(() => globalThis.__semioProbePointer?.downs ?? 0).catch(() => 0);
    await page.mouse.move(point[0], point[1]);
    await page.waitForTimeout(250);
    const hitUnderPointer = has("os_host pointer hit").at(-1) ?? null;
    await page.mouse.click(point[0], point[1]);
    await page.waitForTimeout(600);
    const downsAfter = await page.evaluate(() => globalThis.__semioProbePointer?.downs ?? 0).catch(() => 0);
    clicked = { point, delivered: downsAfter - downsBefore, hitUnderPointer: hitUnderPointer ? hitUnderPointer.slice(hitUnderPointer.indexOf("os_host")) : null };
    lines.push(`${at()} PROBE click ${JSON.stringify(clicked)}`);
  }

  let after = before;
  let firstMs = null;
  let stable = 0;
  for (let second = 0; second < afterSeconds && stable < 2; second += 1) {
    await pump(1);
    after = await converged("generate-preview");
    if (after.ok) {
      firstMs ??= at();
      stable += 1;
    } else stable = 0;
  }

  const row = {
    example,
    url,
    targetsFound: targets.length,
    target: targets[0] ?? null,
    clicked,
    dispatched: has("addGeneration").length - generationsBefore,
    verdict: after.ok ? "pass" : targets.length === 0 ? "blocked-no-row" : clicked && clicked.delivered === 0 ? "blocked-no-pointer" : "fail",
    timeToMeshSeconds: firstMs === null ? null : Math.round(firstMs / 10) / 100,
    before: { ok: before.ok, meshy: before.meshy },
    after: { ok: after.ok, meshy: after.meshy, geometry: after.geometry, alert: after.alert },
    windowIds,
  };
  results.push(row);
  console.log(`[DEBUG] generate/${example}: ${row.verdict} targets=${row.targetsFound} delivered=${clicked?.delivered ?? null} dispatched=${row.dispatched} t=${row.timeToMeshSeconds}s after=${JSON.stringify(row.after.meshy)}`);
  await page.screenshot({ path: join(outDir, "final.png") }).catch(() => {});
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(outDir, "result.json"), JSON.stringify(row, null, 2));
  await page.close();
}

writeFileSync(join(outRoot, "results.json"), JSON.stringify({ origin, results }, null, 2));
await browser.close();
console.log("DONE", JSON.stringify({ pass: results.filter((row) => row.verdict === "pass").length, total: results.length, out: outRoot }, null, 2));
