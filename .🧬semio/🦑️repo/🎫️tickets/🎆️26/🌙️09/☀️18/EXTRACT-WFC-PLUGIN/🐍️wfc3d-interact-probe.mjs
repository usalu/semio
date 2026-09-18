/** 🎛️ wfc3d interaction probe — the editor's real gestures, one step at a time:
 * (1) boot, (2) the navbar example picker (`setActiveExample`) across all three bundled examples,
 * (3) the `wfc-graph` Actions pane (`change-seed`), (4) a node DRAG (`nodeGraphEdit`/`move` →
 * ONE `move-slot`, z kept), (5) a connect gesture between two ports (`connect-slots`),
 * (6) `pin-slot`/`unpin-slot` from the Actions pane, (7) undo.
 * Every step records the console delta, its fault lines and a screenshot; a step PASSES only when its
 * fault array is empty.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=playground-wfc3d/interact-1 bun 🐍️wfc3d-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6045/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-wfc3d/interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 8000 : 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|interactive-ceiling|quarantine/.test(l)).map((l) => l.slice(0, 400));
const historyLines = (from) => lines.slice(from).filter((l) => /history patch applied|patchCursor/.test(l)).map((l) => l.slice(0, 260));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from), history: historyLines(from).slice(0, 8) });
  console.log(`[DEBUG] ${step} faults=${faultLines(from).length} ${JSON.stringify(detail).slice(0, 1400)}`);
  if (faultLines(from).length) console.log(`[DEBUG]   FAULT ${JSON.stringify(faultLines(from).slice(0, 3)).slice(0, 1200)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};

const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const rect = el.getBoundingClientRect();
    const status = parse(el.getAttribute("data-status-json"));
    const instances = parse(el.getAttribute("data-instances-json"));
    return {
      id: el.getAttribute("data-surface-id"),
      rect: { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height) },
      canvases: el.querySelectorAll("canvas").length,
      meshes: (() => { const v = parse(el.getAttribute("data-meshes-json")); return Array.isArray(v) ? v.length : 0; })(),
      instances: Array.isArray(instances) ? instances.length : 0,
      instanceIds: Array.isArray(instances) ? instances.map((i) => `${i.id}=${i.meshId}@${(i.position ?? []).join(",")}`) : [],
      status: status?.message ?? null,
      fault: status?.fault?.code ?? null,
      selection: parse(el.getAttribute("data-selection-json")),
    };
  });
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    hosts,
    combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    engagements: [...document.querySelectorAll('[id*="engagement"]')].map((el) => el.id),
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 500),
  };
});


/** 🔎 Where a node REALLY is on the canvas, asked of the canvas itself: the wasm graph surface answers
 * every pointer move with its own hover target, which the host mirrors onto the surface element's
 * `data-selection-json`. Scanning a row and reading that back beats guessing screen coordinates from
 * the projection — the camera pans, and a stale guess silently turns a wire gesture into a pan. */
const hoverTargetAt = async (x, y) => {
  await page.mouse.move(x, y);
  await page.waitForTimeout(45);
  return page.evaluate(() => {
    try {
      const raw = document.querySelector('[data-surface-id="window:wfc-graph"]')?.getAttribute("data-selection-json");
      const parsed = raw ? JSON.parse(raw) : null;
      const target = parsed?.hoverTarget;
      if (!target) return null;
      return typeof target === "string" ? target : (target.id ?? null);
    } catch { return null; }
  });
};

/** 🔎 The screen span of every node the canvas reports along one row, as {id: {min, max, y}}. */
const scanRow = async (rect, y) => {
  const spans = {};
  for (let x = rect.x + 8; x < rect.x + rect.w - 8; x += 6) {
    const id = await hoverTargetAt(x, y);
    if (!id) continue;
    const span = spans[id] ?? { min: x, max: x, y };
    span.min = Math.min(span.min, x);
    span.max = Math.max(span.max, x);
    spans[id] = span;
  }
  return spans;
};

// 📐️ The `two-room-corridor` graph is three nodes on a line at document x 0/1/2, y 0, projected at
// `WFC_3D_GRAPH_UNIT` (120) canvas units per document unit, with the canvas opening centred on the
// document origin — so the nodes sit at the pane's own centre, 120 px apart.
const nodeCentre = (rect, index) => [rect.x + rect.w / 2 + index * 120, rect.y + rect.h / 2];
const graphRect = (s) => s.hosts.find((h) => h.id === "window:wfc-graph")?.rect ?? { x: 0, y: 40, w: 780, h: 900 };
const previewStatus = (s) => s.hosts.find((h) => h.id === "window:wfc-3d-preview")?.status ?? null;
const previewInstances = (s) => s.hosts.find((h) => h.id === "window:wfc-3d-preview")?.instances ?? 0;

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 8) break; }
await note("boot", s, 0);

// ── the navbar example picker → all three bundled examples ───────────────────
const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? "Wall And Roof Facade Strip|Tower With A Cantilever|Two Rooms And A Corridor").split("|");
for (const example of examples) {
  const from = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  let picked = "no-combobox";
  let options = [];
  if (await combo.count()) {
    await combo.click({ timeout: 8000 }).catch((e) => (picked = String(e).slice(0, 120)));
    await page.waitForTimeout(700);
    options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator('[role="option"]').filter({ hasText: example }).first();
    if (await option.count()) picked = await option.click({ timeout: 8000 }).then(() => `ok:${example}`).catch((e) => String(e).slice(0, 120));
    else { picked = `no-option:${example}`; await page.keyboard.press("Escape"); }
  }
  let after = null;
  for (let i = 0; i < settleSeconds * 3; i++) { await page.waitForTimeout(500); after = await state(); if (after.combobox?.includes(example)) break; }
  await page.waitForTimeout(2500);
  after = await state();
  await note(`example-${example.replace(/[^a-z0-9]+/gi, "-").toLowerCase()}`, { picked, options, combobox: after.combobox, status: previewStatus(after), instances: previewInstances(after), instanceIds: after.hosts.find((h) => h.id === "window:wfc-3d-preview")?.instanceIds }, from);
  s = after;
}

// ── the graph window Actions pane → change-seed ──────────────────────────────
{
  const from = lines.length;
  const engagement = s.engagements.find((id) => /wfcGraph/i.test(id) && id.endsWith(".engagement"));
  let toggled = "absent";
  if (engagement) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120)) : `no-toggle:${engagement}`;
  }
  await page.waitForTimeout(2000);
  const opened = await state();
  let clicked = "absent";
  let submitted = "none";
  if (opened.actionRows.includes("action.change-seed")) {
    clicked = await page.locator('[id="action.change-seed"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
    await page.waitForTimeout(1500);
    const submit = page.locator('[id$=".action.change-seed.execute"]').first();
    submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
  }
  await page.waitForTimeout(3000);
  const after = await state();
  await note("action-change-seed", { engagement, toggled, actionRows: opened.actionRows, clicked, submitted, status: previewStatus(after), instanceIds: after.hosts.find((h) => h.id === "window:wfc-3d-preview")?.instanceIds }, from);
  s = after;
}

// ── a connect gesture between two node ports → connect-slots ────────────────
// 🔗 Run BEFORE the drag, while all three nodes still sit on one row. The wire goes from `room-a`'s
// `adjacent-out` port (the right port glyph inside the node box) to `room-b`'s `adjacent-in` port.
// The new adjacency closes an ODD cycle over a two-tile alternating rule set, so the honest proof
// that it landed is the preview's own verdict flipping to a contradiction.
const WFC3D_PORT_OFFSET = Number(process.env.SEMIO_PROBE_PORT_OFFSET ?? 14);
{
  const from = lines.length;
  const rect = graphRect(s);
  const [aX, aY] = nodeCentre(rect, 0);
  const [bX, bY] = nodeCentre(rect, 2);
  const sourceX = aX + WFC3D_PORT_OFFSET;
  const targetX = bX - WFC3D_PORT_OFFSET;
  await page.mouse.move(sourceX, aY);
  await page.mouse.down();
  const steps = 12;
  for (let i = 1; i <= steps; i++) { await page.mouse.move(sourceX + ((targetX - sourceX) * i) / steps, aY + ((bY - aY) * i) / steps); await page.waitForTimeout(70); }
  await page.mouse.up();
  await page.waitForTimeout(6000);
  const after = await state();
  await note("connect-gesture", {
    from: [sourceX, aY],
    to: [targetX, bY],
    status: previewStatus(after),
    instances: previewInstances(after),
    instanceIds: after.hosts.find((h) => h.id === "window:wfc-3d-preview")?.instanceIds,
    expected: "a contradiction verdict, because room-a=room-b=corridor and the new edge closes an odd cycle",
  }, from);
  s = after;
}

// ── a node DRAG on the graph canvas → one move-slot, z kept ─────────────────
// 📐️ The `two-room-corridor` graph is three nodes on a line at document x 0/1/2, y 0, projected at
// `WFC_3D_GRAPH_UNIT` (120) canvas units per document unit, with the canvas opening centred on the
// document origin — so the nodes sit at the pane's own centre, 120 px apart.
{
  const from = lines.length;
  const rect = graphRect(s);
  const [nodeX, nodeY] = nodeCentre(rect, 0);
  const dropY = nodeY + 240;
  await page.mouse.move(nodeX, nodeY);
  await page.mouse.down();
  for (let i = 1; i <= 8; i++) { await page.mouse.move(nodeX, nodeY + (240 * i) / 8); await page.waitForTimeout(80); }
  await page.mouse.up();
  await page.waitForTimeout(5000);
  const after = await state();
  const moved = after.hosts.find((h) => h.id === "window:wfc-3d-preview")?.instanceIds?.find((entry) => entry.startsWith("room-a=")) ?? null;
  await note("drag-node", { from: [nodeX, nodeY], to: [nodeX, dropY], movedInstance: moved, expected: "room-a=tile:corridor@0,2,0", instanceIds: after.hosts.find((h) => h.id === "window:wfc-3d-preview")?.instanceIds, status: previewStatus(after) }, from);
  s = after;
}

// ── pin-slot / unpin-slot from the Actions pane ──────────────────────────────
for (const verb of ["pin-slot", "unpin-slot"]) {
  const from = lines.length;
  let clicked = "absent";
  let submitted = "none";
  if (s.actionRows.includes(`action.${verb}`)) {
    clicked = await page.locator(`[id="action.${verb}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
    await page.waitForTimeout(1500);
    const submit = page.locator(`[id$=".action.${verb}.execute"]`).first();
    submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
  }
  await page.waitForTimeout(3000);
  const after = await state();
  await note(`action-${verb}`, { clicked, submitted, status: previewStatus(after), instances: previewInstances(after) }, from);
  s = after;
}

// ── undo ────────────────────────────────────────────────────────────────────
{
  const from = lines.length;
  const before = previewStatus(s);
  let presses = 0;
  for (let attempt = 0; attempt < 3; attempt++) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
    presses += 1;
    await page.waitForTimeout(2500);
  }
  const after = await state();
  const undone = lines.slice(from).filter((l) => /history patch applied.*Undo|"Undo"/.test(l)).length;
  await note("undo", { presses, undoPatches: undone, statusBefore: before, statusAfter: previewStatus(after), instances: previewInstances(after), instanceIds: after.hosts.find((h) => h.id === "window:wfc-3d-preview")?.instanceIds }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
const failing = report.steps.filter((step) => step.faults.length);
console.log("[DEBUG] INTERACT DONE steps", report.steps.length, "failing", failing.length, JSON.stringify(failing.map((s) => s.step)));
await browser.close();
