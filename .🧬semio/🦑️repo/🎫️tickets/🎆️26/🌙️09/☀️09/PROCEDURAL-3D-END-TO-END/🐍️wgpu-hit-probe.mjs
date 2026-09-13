/** 🎯️ wgpu TREE ROW HIT-TEST probe.
 *
 * Proves the defect named in `📓️wgpu-resident-budget-settle-2026-09-12.md` §8 is closed: in generate
 * mode the Generations window's `Add Generation` row is hit where it is PAINTED, so one pointer click
 * dispatches `addGeneration`, and the generate preview then carries a mesh.
 *
 * Every click point is DERIVED, never guessed: the window's on-screen body comes from the shell's own
 * `dock plan` trace (`<id>@<w>x<h>+<x>,<y>`) and the row's rect from `dumpStructure(windowId)`, whose
 * rects are the published `mounted_layout` — the same rectangles `events::hit_test` reads.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-hit/run-1 bun 🐍️wgpu-hit-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&mode=generate";
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 45);
const afterSeconds = Number(process.env.SEMIO_PROBE_AFTER ?? 25);
const rowNeedle = process.env.SEMIO_PROBE_ROW ?? "add-generation";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-hit/run");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const has = (needle) => lines.filter((line) => line.includes(needle));
const nudge = async (n) => page.mouse.move(3 + (n % 4), 3 + (n % 4)).catch(() => {});
const dropOverlay = () =>
  page
    .evaluate(() => {
      const overlay = Array.from(document.body.children).find((node) => node.tagName === "DIV" && node.id !== "root");
      if (!overlay) return null;
      const text = (overlay.textContent ?? "").slice(0, 160);
      overlay.remove();
      return text;
    })
    .catch(() => null);

const dump = (windowId) =>
  page
    .evaluate(async (id) => {
      const beacon = globalThis.semioWgpuIntrospection;
      if (typeof beacon?.dumpStructure !== "function") return null;
      const parse = async (call) => {
        try {
          const raw = await call();
          return raw ? JSON.parse(raw) : null;
        } catch (error) {
          return { error: String(error) };
        }
      };
      return { structure: await parse(() => beacon.dumpStructure(id)), stats: await parse(() => beacon.dumpFrameStats(id)) };
    }, windowId)
    .catch((error) => ({ error: String(error).slice(0, 300) }));

/** 🪟️ The shell's last dock plan, as `{ id: {x, y, w, h} }` — the window body rects on screen. */
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

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));

for (let second = 1; second <= settleSeconds; second += 1) {
  await page.waitForTimeout(1000);
  await nudge(second);
}
await dropOverlay();
await page.screenshot({ path: join(outDir, "shot-before.png"), type: "png" }).catch(() => {});

const windowIds = (await dump(undefined))?.structure?.windowIds ?? [];
lines.push(`${at()} PROBE windowIds ${JSON.stringify(windowIds)}`);
const plan = dockPlan();
lines.push(`${at()} PROBE dockPlan ${JSON.stringify(plan)}`);

/** 🎯️ Finds the row the deliverable clicks, in page coordinates, from the two published sources. */
const rowTargets = [];
const structures = {};
for (const id of windowIds) {
  const sample = await dump(id);
  structures[id] = sample;
  const body = plan[id];
  for (const node of sample?.structure?.nodes ?? []) {
    if (!String(node.path ?? "").includes(rowNeedle) || !body) continue;
    const [rx, ry, rw, rh] = node.rect ?? [0, 0, 0, 0];
    rowTargets.push({ windowId: id, path: node.path, rect: node.rect, page: [body.x + rx + rw / 2, body.y + ry + rh / 2] });
  }
}
lines.push(`${at()} PROBE rowTargets ${JSON.stringify(rowTargets)}`);

/** 🖱️ Counts what the BROWSER delivers to the canvas, so "no dispatch" can be told apart from
 * "no pointer event ever arrived" — the two have identical console evidence otherwise. */
await page.evaluate(() => {
  const canvas = document.querySelector("canvas");
  const host = canvas ?? document.body;
  globalThis.__semioProbePointer = { downs: 0, last: null, canvas: canvas ? canvas.getBoundingClientRect().toJSON() : null, tag: host.tagName };
  host.addEventListener("pointerdown", (event) => {
    globalThis.__semioProbePointer.downs += 1;
    globalThis.__semioProbePointer.last = { x: event.clientX, y: event.clientY, offsetX: event.offsetX, offsetY: event.offsetY };
  }, true);
});

/** 👆️ Hover witness: `dumpStructure`'s own `state.hovered`, sampled with the pointer parked on the
 * row, then 300 px away, then back — the three samples §6 of
 * `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` measured as `[]`, `[]`, `[]`. */
const hoveredIds = async (windowId) => {
  const sample = await dump(windowId);
  return (sample?.structure?.nodes ?? []).filter((node) => node?.state?.hovered === true).map((node) => node.path);
};

const hoverTarget = rowTargets.at(-1);
const hoverWitness = { row: hoverTarget?.path ?? null, idle: [], onRow: [], away: [], back: [] };
if (hoverTarget) {
  hoverWitness.idle = await hoveredIds(hoverTarget.windowId);
  await page.mouse.move(hoverTarget.page[0], hoverTarget.page[1]);
  await page.waitForTimeout(2500);
  hoverWitness.onRow = await hoveredIds(hoverTarget.windowId);
  await page.mouse.move(hoverTarget.page[0], hoverTarget.page[1] + 300);
  await page.waitForTimeout(2500);
  hoverWitness.away = await hoveredIds(hoverTarget.windowId);
  await page.mouse.move(hoverTarget.page[0], hoverTarget.page[1]);
  await page.waitForTimeout(2500);
  hoverWitness.back = await hoveredIds(hoverTarget.windowId);
}
lines.push(`${at()} PROBE hoverWitness ${JSON.stringify(hoverWitness)}`);

const beforeRenderBegin = has("render begin").length;
const beforeDispatch = has(rowNeedle).length;
const target = rowTargets.at(-1);
if (target) {
  await page.mouse.click(target.page[0], target.page[1]);
  lines.push(`${at()} PROBE clicked ${target.path} at ${target.page.join(",")}`);
} else {
  lines.push(`${at()} PROBE no row target for ${rowNeedle}`);
}

for (let second = 1; second <= afterSeconds; second += 1) {
  await page.waitForTimeout(1000);
  await nudge(second);
}
const pointerWitness = await page.evaluate(() => globalThis.__semioProbePointer ?? null).catch(() => null);
lines.push(`${at()} PROBE pointerWitness ${JSON.stringify(pointerWitness)}`);
await dropOverlay();
await page.screenshot({ path: join(outDir, "shot-after.png"), type: "png" }).catch(() => {});

/** 🖱️ Two more pointer landings the deliverable asks for: the Flow node-graph and a Preview instance. */
const elsewhere = [];
for (const [id, kindNeedle] of [
  [process.env.SEMIO_PROBE_FLOW_WINDOW ?? "generation3d-flow", "scene"],
  [process.env.SEMIO_PROBE_PREVIEW_WINDOW ?? "generation3d-generate-preview", "scene"],
]) {
  const body = dockPlan()[id];
  const sample = await dump(id);
  const scene = (sample?.structure?.nodes ?? []).find((node) => String(node.kind ?? "").toLowerCase().includes(kindNeedle));
  if (!body || !scene) {
    elsewhere.push({ windowId: id, hit: null, why: !body ? "no dock body" : "no scene node" });
    continue;
  }
  const [rx, ry, rw, rh] = scene.rect;
  const point = [body.x + rx + rw / 2, body.y + ry + rh / 2];
  const before = has("render begin").length;
  await page.mouse.click(point[0], point[1]);
  await page.waitForTimeout(3000);
  await nudge(1);
  await page.waitForTimeout(2000);
  elsewhere.push({ windowId: id, path: scene.path, rect: scene.rect, page: point, renderBeginDelta: has("render begin").length - before });
  lines.push(`${at()} PROBE clicked ${id} ${scene.path} at ${point.join(",")}`);
}
await dropOverlay();
await page.screenshot({ path: join(outDir, "shot-elsewhere.png"), type: "png" }).catch(() => {});

/** 🎥️ Wheel witness: the world3d camera the renderer traces per surface, before and after a wheel
 * over the Preview. `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6 measured it byte-identical
 * across all 121 traces of a run that wheeled twice, because the wheel gate refused the ScrollRegion
 * the window answered. */
const cameraTrace = () => {
  const line = has("world3d surface").at(-1) ?? "";
  const match = /\{"fov".*?\}/.exec(line);
  return match ? match[0] : null;
};
const previewWindow = process.env.SEMIO_PROBE_PREVIEW_WINDOW ?? "generation3d-generate-preview";
const previewBody = dockPlan()[previewWindow];
const wheelWitness = { window: previewWindow, body: previewBody ?? null, before: cameraTrace(), after: null, deltaLines: 0 };
if (previewBody) {
  const point = [previewBody.x + previewBody.w / 2, previewBody.y + previewBody.h / 2];
  const beforeCount = has("world3d surface").length;
  await page.mouse.move(point[0], point[1]);
  await page.waitForTimeout(800);
  for (let tick = 0; tick < 4; tick += 1) {
    await page.mouse.wheel(0, 160);
    await page.waitForTimeout(700);
    await nudge(tick);
  }
  await page.waitForTimeout(3000);
  wheelWitness.after = cameraTrace();
  wheelWitness.deltaLines = has("world3d surface").length - beforeCount;
  wheelWitness.changed = wheelWitness.before !== null && wheelWitness.after !== null && wheelWitness.before !== wheelWitness.after;
  lines.push(`${at()} PROBE wheeled ${previewWindow} at ${point.join(",")}`);
}
lines.push(`${at()} PROBE wheelWitness ${JSON.stringify(wheelWitness)}`);
await page.screenshot({ path: join(outDir, "shot-after-wheel.png"), type: "png" }).catch(() => {});

const afterStructures = {};
for (const id of windowIds) afterStructures[id] = await dump(id);

const verdict = {
  url,
  seconds: Math.round(at() / 1000),
  windowIds,
  dockPlan: plan,
  rowTargets,
  clicked: target ?? null,
  pointerWitness,
  renderBeginBefore: beforeRenderBegin,
  renderBeginAfter: has("render begin").length,
  rowMentionsBefore: beforeDispatch,
  rowMentionsAfter: has(rowNeedle).length,
  addGenerationLines: lines.filter((line) => line.toLowerCase().includes("addgeneration")).slice(-10),
  commandSettled: has("wgpu-shell command").slice(-8),
  previewStats: afterStructures[process.env.SEMIO_PROBE_PREVIEW_WINDOW ?? "generation3d-generate-preview"]?.stats ?? null,
  hoverWitness,
  wheelWitness,
  pointerHitLines: has("os_host pointer hit").slice(-12),
  elsewhere,
  rowRectsAfter: Object.fromEntries(
    Object.entries(afterStructures).map(([id, sample]) => [id, (sample?.structure?.nodes ?? []).filter((node) => String(node.kind ?? "") === "stack").slice(0, 24).map((node) => ({ path: node.path, rect: node.rect }))]),
  ),
  failureLines: lines.filter((line) => line.includes("pageerror") || line.includes("panicked") || line.includes("surface fault")).slice(-12),
};
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "structures.json"), JSON.stringify({ before: structures, after: afterStructures }, null, 2));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log("DONE", JSON.stringify({ ...verdict, rowRectsAfter: undefined }, null, 2));
await browser.close();
