/** 🎯 Hover / select / inspect / orbit / fit over ALL EIGHT bundled examples, every step decided by a
 * VALUE read off the pane's own published state — never by a regex over the run's console log.
 *
 * What it replaces: the battery's `interact` lane ran `🔍️browser-probe.ts` against the single
 * hardcoded `box-shell-preview` and graded it with `/hoverTarget|interactionHover/` etc. over the
 * whole accumulated console dump (`🐍️react-battery.mjs:100-108`) — a text-existence check, unscoped
 * to the step, blind to WHICH target was hit, and silent for the other seven examples
 * (`📓️example-oracle-strength-audit-2026-09-14.md` §1, §3).
 *
 * What each hop now asserts, per example:
 *   `payload`   — the delivered meshes match the committed fixture ({@link gradeMeshes}).
 *   `hover`     — the pane publishes a `hoverTarget` whose ID IS one of the instance/mesh ids this
 *                 very example published; the probe sweeps a grid over the canvas until the pointer
 *                 is over geometry, and reports the point it hit.
 *   `select`    — clicking that same point leaves `selectedIds` naming the hovered id.
 *   `inspector` — the Inspection panel's own `…procedural-play-inspector.id` row then names the
 *                 selected widget, so a canvas pick provably reaches the panels a user reads.
 *   `orbit`     — a drag moves the camera POSITION while keeping its target, i.e. an orbit rather
 *                 than a pan or a no-op.
 *   `fit`       — the `Frame visible` overlay button reproduces the product's own fit rule over the
 *                 payload ({@link gradeCameraFit}), which — with `payload` green — means the camera
 *                 frames the fixture's committed bounding box.
 *
 * The DOM lanes and the fixture schema are documented in `🐍️example-oracle.mjs`.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6022/?plugin=generation3d SEMIO_PROBE_OUT=react-oracle/interaction bun 🐍️interaction-matrix-probe.mjs
 * Env: SEMIO_PROBE_EXAMPLES (comma separated labels, default all), SEMIO_PROBE_MESH_WAIT (seconds).
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { ORACLES, gradeCameraFit, gradeMeshes, meshStatsScript } from "./🐍️example-oracle.mjs";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-oracle/interaction");
const meshWait = Number(process.env.SEMIO_PROBE_MESH_WAIT ?? 90);
const wanted = (process.env.SEMIO_PROBE_EXAMPLES ?? "").split(",").map((s) => s.trim()).filter(Boolean);
const oracles = Object.values(ORACLES).filter((oracle) => wanted.length === 0 || wanted.includes(oracle.label) || wanted.includes(oracle.slug));
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const faults = [];
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => {
  faults.push({ t: Date.now() - t0, message: String(e?.message ?? e).slice(0, 300), stack: String(e?.stack ?? "").slice(0, 2500) });
  lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`);
});

/** 🔦️ Everything one pane publishes about what it holds and what is picked in it. The mesh numbers
 * come from the shared page-side reader and the pick state from the same elements in the same
 * document order, so the two line up by index. */
const paneSnap = async (previewMeshId) => {
  const stats = await page.evaluate(meshStatsScript, previewMeshId);
  const dom = await page.evaluate(() => {
    const parse = (raw) => { try { return JSON.parse(raw ?? "null"); } catch { return null; } };
    const panes = [...document.querySelectorAll("[data-meshes-json], [data-status-json]")].map((el) => {
      const status = parse(el.getAttribute("data-status-json"));
      return {
        selection: parse(el.getAttribute("data-selection-json")),
        guestSelection: parse(el.getAttribute("data-guest-selection-json")),
        camera: parse(el.getAttribute("data-viewport-camera-json")),
        sceneCamera: parse(el.getAttribute("data-camera-json")),
        phase: status?.phase ?? null,
        ratio: status?.progress?.ratio ?? null,
        fault: status?.fault?.code ?? null,
      };
    });
    const combo = document.querySelector('[role="combobox"]');
    return {
      panes,
      example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null,
      inspector: [...document.querySelectorAll('[data-slot="panel"] [id*="procedural-play-inspector"]')].map((el) => ({ id: el.id, text: (el.textContent ?? "").trim().slice(0, 80), value: el.value ?? null })),
    };
  });
  return { ...dom, panes: dom.panes.map((pane, index) => ({ ...(stats[index] ?? {}), ...pane })) };
};

const previewPane = (snap) => snap.panes.find((pane) => pane.surfaceId && pane.surfaceId.endsWith("-preview")) ?? null;
const settled = (pane) => Boolean(pane) && pane.phase === "idle" && pane.ratio === 1 && !pane.fault;
/** 🪪️ Every id this example's own payload offers a pick — instance ids, the `interactionId` an
 * instance reports when picked, and the mesh ids they draw. A hovered or selected value outside this
 * set is a target that does not exist in the example the user is looking at. */
const publishedIds = (pane) => new Set([...(pane?.instanceIds ?? []), ...(pane?.interactionIds ?? []), ...(pane?.meshIds ?? [])]);

const results = [];
const record = async (example, step, ok, detail) => {
  const row = { example, step, ok, t: Date.now() - t0, detail };
  results.push(row);
  console.log(`[DEBUG] ${example} ${step}: ok=${ok} ${JSON.stringify(detail).slice(0, 420)}`);
  return row;
};

const pick = async (label) => {
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 6000 });
  await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: label }).first().click({ timeout: 6000 });
};

const waitPayload = async (oracle) => {
  let last = null;
  for (let i = 0; i < meshWait; i += 1) {
    await page.waitForTimeout(1000);
    last = await paneSnap(oracle.previewMeshId);
    const pane = previewPane(last);
    if (last.example === oracle.label && settled(pane) && gradeMeshes(oracle, pane).ok) return last;
  }
  return last;
};

const canvasBox = async () => {
  const canvas = page.locator('[data-meshes-json] canvas, .semio-world-3d-host canvas').last();
  if (!(await canvas.count())) return null;
  return canvas.boundingBox();
};

/** 🖱️ Sweep the pointer over the pane until it is over this example's geometry. A single fixed
 * centre pick lands on background for every example whose solid is not centred on the canvas, which
 * is exactly how a gesture step can "pass" while touching nothing. */
const sweepForHover = async (oracle, box) => {
  const points = [];
  for (let row = 1; row <= 5; row += 1) for (let col = 1; col <= 7; col += 1) points.push([box.x + (box.width * col) / 8, box.y + (box.height * row) / 6]);
  points.sort((a, b) => Math.hypot(a[0] - (box.x + box.width / 2), a[1] - (box.y + box.height / 2)) - Math.hypot(b[0] - (box.x + box.width / 2), b[1] - (box.y + box.height / 2)));
  for (let swept = 0; swept < points.length; swept += 1) {
    const [x, y] = points[swept];
    await page.mouse.move(x, y);
    await page.waitForTimeout(260);
    const snap = await paneSnap(oracle.previewMeshId);
    const target = previewPane(snap)?.selection?.hoverTarget ?? null;
    if (target?.id) return { point: [Math.round(x), Math.round(y)], target, snap, swept: swept + 1 };
  }
  return { point: null, target: null, snap: await paneSnap(oracle.previewMeshId), swept: points.length };
};

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(20_000);
await page.locator('[id="framework.panel.inspection"]').first().click({ timeout: 8000 }).catch(() => {});
await page.waitForTimeout(1500);

for (const oracle of oracles) {
  await pick(oracle.label).catch((error) => lines.push(`pick ${oracle.label} failed ${String(error).slice(0, 200)}`));
  const settledSnap = await waitPayload(oracle);
  const pane = previewPane(settledSnap);
  const grade = gradeMeshes(oracle, pane);
  await record(oracle.label, "payload", grade.ok && settledSnap.example === oracle.label, {
    example: settledSnap.example, oracleMeshes: oracle.meshes, meshes: pane?.meshCount ?? 0, instances: pane?.instanceCount ?? 0,
    previewMeshId: oracle.previewMeshId, triangles: pane?.preview?.triangles ?? null, edgeSegments: pane?.preview?.edgeSegments ?? null,
    bounds: pane?.preview?.bounds ?? null, oracleBounds: { min: oracle.boundingBoxMin, max: oracle.boundingBoxMax, tolerance: oracle.boundingBoxTolerance }, reasons: grade.reasons,
  });

  /** 🧹 Switching the example swaps the whole document, so a selection left standing can only name a
   * widget that no longer exists — and the Inspection panel then describes a node the user cannot
   * see. Every id still selected must belong to the example now on screen. */
  const switched = previewPane(settledSnap)?.selection?.selectedIds ?? [];
  const offeredAfterSwitch = publishedIds(previewPane(settledSnap));
  await record(oracle.label, "selection-reset", switched.every((id) => offeredAfterSwitch.has(id)), { selectedIdsAfterSwitch: switched, guestSelectedIdsAfterSwitch: previewPane(settledSnap)?.guestSelection?.selectedIds ?? null, offered: [...offeredAfterSwitch] });

  const box = await canvasBox();
  if (!box) {
    await record(oracle.label, "hover", false, { error: "no preview canvas" });
    await record(oracle.label, "select", false, { error: "no preview canvas" });
    await record(oracle.label, "inspector", false, { error: "no preview canvas" });
    await record(oracle.label, "orbit", false, { error: "no preview canvas" });
    await record(oracle.label, "fit", false, { error: "no preview canvas" });
    continue;
  }

  const hover = await sweepForHover(oracle, box);
  const offered = publishedIds(previewPane(hover.snap));
  const hoverOk = Boolean(hover.target?.id) && offered.has(hover.target.id);
  await record(oracle.label, "hover", hoverOk, { point: hover.point, target: hover.target, offered: [...offered], sweptPoints: hover.swept });

  const before = previewPane(hover.snap)?.selection?.selectedIds ?? [];
  let selectedIds = [];
  let guestSelectedIds = null;
  let activeObjectId = null;
  if (hover.point) {
    await page.mouse.click(hover.point[0], hover.point[1]);
    await page.waitForTimeout(2500);
    const after = previewPane(await paneSnap(oracle.previewMeshId));
    selectedIds = after?.selection?.selectedIds ?? [];
    guestSelectedIds = after?.guestSelection?.selectedIds ?? null;
    activeObjectId = after?.selection?.activeObjectId ?? null;
  }
  /** 🎯 A plain click is a REPLACE merge, so the pane owes exactly the id that was hovered — not
   * "contains it somewhere". An `includes` check passes just as happily on a selection that has been
   * accumulating ids across example switches. */
  const selectOk = Boolean(hover.target?.id) && selectedIds.length === 1 && selectedIds[0] === hover.target.id && activeObjectId === hover.target.id;
  await record(oracle.label, "select", selectOk, { clickedPoint: hover.point, hovered: hover.target?.id ?? null, selectedIdsBefore: before, selectedIds, guestSelectedIds, activeObjectId, offered: [...offered] });

  const inspected = await paneSnap(oracle.previewMeshId);
  const widget = (activeObjectId ?? selectedIds[0] ?? "").split("@")[0].replace(/#\d+$/, "").replace(/^eval-/, "");
  const idRow = inspected.inspector.find((row) => row.id.endsWith("procedural-play-inspector.id")) ?? null;
  await record(oracle.label, "inspector", Boolean(widget) && Boolean(idRow) && idRow.text.includes(widget), { widget, idRow, rows: inspected.inspector.map((row) => row.id) });

  const beforeOrbit = previewPane(await paneSnap(oracle.previewMeshId))?.camera ?? null;
  const ox = box.x + box.width * 0.6;
  const oy = box.y + box.height * 0.45;
  // 🖱️ Alt + RIGHT drag is the product's orbit binding (`resolveWorldOrbitRightMouseAction`,
  // `🎨️r3f/🟦️.tsx:3282` — plain right opens the context menu, shift+right pans, middle pans, and the
  // LEFT button is the marquee, never the camera). The old `interact` lane left-dragged and still
  // passed, because its verdict only asked whether the string `setCamera` appeared anywhere in the
  // run — which it does at boot, before any gesture.
  // 🧭️ Drag AWAY from the polar clamp the camera is nearest. `OrbitControls` clamps the polar
  // angle at 0 and π, and a drag that pushes into a clamp is a legitimate no-op — eight examples all
  // dragged the same way walked the camera onto the +Z pole, where every further drag correctly did
  // nothing and read as a dead orbit.
  const distance = beforeOrbit ? Math.hypot(...beforeOrbit.position.map((v, axis) => v - beforeOrbit.target[axis])) : 1;
  const polar = beforeOrbit ? Math.acos(Math.min(1, Math.max(-1, (beforeOrbit.position[2] - beforeOrbit.target[2]) / (distance || 1)))) : Math.PI / 2;
  const dy = polar < Math.PI / 4 ? -40 : 40;
  await page.keyboard.down("Alt");
  await page.mouse.move(ox, oy);
  await page.mouse.down({ button: "right" });
  await page.mouse.move(ox + 110, oy + dy, { steps: 14 });
  await page.mouse.up({ button: "right" });
  await page.keyboard.up("Alt");
  await page.waitForTimeout(2500);
  const afterOrbit = previewPane(await paneSnap(oracle.previewMeshId))?.camera ?? null;
  const moved = Boolean(beforeOrbit && afterOrbit) && JSON.stringify(beforeOrbit.position) !== JSON.stringify(afterOrbit.position);
  const keptTarget = Boolean(beforeOrbit && afterOrbit) && Math.hypot(...afterOrbit.target.map((v, axis) => v - beforeOrbit.target[axis])) < 1e-3;
  const keptDistance =
    Boolean(beforeOrbit && afterOrbit) &&
    Math.abs(Math.hypot(...afterOrbit.position.map((v, axis) => v - afterOrbit.target[axis])) - Math.hypot(...beforeOrbit.position.map((v, axis) => v - beforeOrbit.target[axis]))) < 0.05;
  await record(oracle.label, "orbit", moved && keptTarget && keptDistance, { before: beforeOrbit, after: afterOrbit, polarBefore: polar, dragDy: dy, moved, keptTarget, keptDistance });

  const fitButton = page.locator('[data-slot="world-frame-instances"]').last();
  const fitPresent = (await fitButton.count()) > 0;
  /** 🚧️ What is ON TOP of the `Frame visible` button. A button that is in the DOM but covered is a
   * button no user can press, and the click then fails with a timeout rather than a wrong camera —
   * two different defects that must not read the same in the report. */
  const obstruction = fitPresent
    ? await page.evaluate(() => {
        const button = document.querySelector('[data-slot="world-frame-instances"]');
        if (!button) return null;
        const rect = button.getBoundingClientRect();
        const top = document.elementFromPoint(rect.x + rect.width / 2, rect.y + rect.height / 2);
        if (!top || top === button || button.contains(top)) return null;
        const panel = top.closest('[data-slot="panel"]');
        return { tag: top.tagName, id: (panel ?? top).id || null, slot: (panel ?? top).getAttribute("data-slot"), anchor: (panel ?? top).getAttribute("data-anchor"), text: ((panel ?? top).innerText ?? "").replace(/\s+/g, " ").slice(0, 60) };
      })
    : null;
  let fitClickError = null;
  if (fitPresent) await fitButton.click({ timeout: 6000 }).catch((error) => { fitClickError = String(error).split("\n")[0].slice(0, 160); });
  await page.waitForTimeout(2500);
  const fitted = previewPane(await paneSnap(oracle.previewMeshId));
  const fitGrade = gradeCameraFit(fitted?.camera ?? null, fitted?.allBounds ?? null);
  await record(oracle.label, "fit", fitPresent && !fitClickError && fitGrade.ok, { fitPresent, obstruction, fitClickError, camera: fitted?.camera ?? null, framedBounds: fitted?.allBounds ?? null, centre: fitGrade.centre, radius: fitGrade.radius, offCentre: fitGrade.offCentre, distance: fitGrade.distance, expectedDistance: fitGrade.expected, reasons: fitGrade.reasons });

  await page.screenshot({ path: join(outDir, `${oracle.slug}.png`) });
}

writeFileSync(join(outDir, "results.json"), JSON.stringify({ url, examples: oracles.map((oracle) => oracle.label), results, faults }, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "faults.json"), JSON.stringify(faults, null, 2));
console.log(`[DEBUG] DONE rows=${results.length} green=${results.filter((row) => row.ok).length} pageerrors=${faults.length}`);
await browser.close();
