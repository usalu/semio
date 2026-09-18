/** 🎛️ WFC 2D interaction probe. Boots the wfc2d react playground, opens the History panel (the one
 * DOM-readable ledger of what actually landed in the document) and then:
 *  1. records the boot state (shell beacon, both window hosts, the Actions rosters),
 *  2. runs `solve` from the Preview pane's Actions and proves the preview REPAINTS — screenshotted
 *     with the Actions pane CLOSED on both sides, so the panel's own pixels cannot fake the diff,
 *  3. switches the navbar example to the bitmap `terrain-ring` and proves both panes repaint,
 *  4. re-selects the boot example and proves it mints NO history entry (no phantom edit),
 *  5. finds a slot node by hovering the canvas, drags it, and proves ONE `Move slot` entry landed,
 *  6. undoes and proves the entry is gone,
 *  7. draws a wire between two slot nodes and proves a `Connect` entry landed,
 *  8. runs `pin-slot` then `unpin-slot` from the Graph pane's Actions.
 * Every step records its console delta and `faultLines()`; a zero-length fault array is the pass
 * condition. `shell.*` ids are framework shell-command replays that no plugin in this repo declares
 * (see `📓️playground-wfc2d.md` §fault-ledger) — they are counted separately as `shellNotes`, never
 * hidden.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=playground-wfc2d/interact bun 🐍️wfc2d-interact-probe.mjs
 */
import { chromium } from "playwright";
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6043/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-wfc2d/interact");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 8000 : 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const FAULT = /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|undeclared-action/;
const SHELL_REPLAY = /"?shell\.[a-zA-Z]+"? refused/;
const faultLines = (from, expected) => lines.slice(from).filter((l) => FAULT.test(l) && !SHELL_REPLAY.test(l) && !(expected && expected.test(l))).map((l) => l.slice(0, 400));
const shellNotes = (from) => lines.slice(from).filter((l) => SHELL_REPLAY.test(l)).map((l) => l.slice(0, 200));
/** 🏛️ History rows the FRAMEWORK owns — shell chrome commands and its own selection/undo bookkeeping.
 * A document edit of this artifact is never one of these, so a "no phantom edit" law filters them out. */
const FRAMEWORK_ROW = /Toggle Panel|Switch Panel Tab|Activate Window|Move Window|Resize Window|Close Window|Clear Selection|Select All|Undo|Redo|Selection Mode|Granularity/i;

const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const st = parse(el.getAttribute("data-status-json"));
    const r = el.getBoundingClientRect();
    return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, rect: { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) } };
  });
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    history: [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((el) => !el.id.endsWith(".revert")).map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 60)),
  };
});
const shot = async (name, rect) => page.screenshot({ path: join(outDir, `${name}.png`), clip: rect ? { x: rect.x, y: rect.y, width: rect.w, height: rect.h } : undefined }).catch(() => null);
const signature = async (rect) => createHash("sha256").update(await page.screenshot({ clip: { x: rect.x, y: rect.y, width: rect.w, height: rect.h } })).digest("hex").slice(0, 16);
/** 📓️ One step's record. `expected` names a refusal the step is PROVING (a guard firing on purpose),
 * which is evidence rather than a fault; everything else that matches `FAULT` fails the step. */
const note = async (step, detail, from, expected) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from, expected), shellNotes: shellNotes(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1100)} faults=${faultLines(from, expected).length}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
};

/** 🎬️ Opens a pane's Actions engagement, activates one row, fills its staged argument form, executes
 * it, and closes the pane again so a later screenshot sees the window and not the panel. A row with
 * no form runs on activation and offers no Execute control — both shapes are reported. */
const runAction = async (paneId, actionId, args = {}) => {
  const toggle = page.locator(`[id="${paneId}.engagement.toggle"]`).first();
  const toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  await page.waitForTimeout(1300);
  const rows = (await state()).actionRows;
  const rowId = `action.${actionId}`;
  const clicked = rows.includes(rowId) ? await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  await page.waitForTimeout(1600);
  const filled = {};
  for (const [key, value] of Object.entries(args)) {
    const field = page.locator(`[id="${rowId}.arg.${key}"] input, [id="${rowId}.arg.${key}"] textarea`).first();
    filled[key] = (await field.count()) ? await field.fill(String(value), { timeout: 6000 }).then(() => "ok").catch((e) => String(e).slice(0, 80)) : "absent";
  }
  await page.waitForTimeout(500);
  const execute = page.locator(`[id^="${paneId}.action."][id$=".execute"]`).first();
  const executed = (await execute.count()) ? await execute.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-form-runs-on-activate";
  await page.waitForTimeout(3500);
  if ((await toggle.count()) && toggled === "ok") await toggle.click({ timeout: 8000, force: true }).catch(() => {});
  await page.waitForTimeout(1500);
  return { pane: paneId, toggled, rowId, clicked, filled, executed, offered: rows.filter((id) => !id.startsWith("action.category.")).length };
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length >= 2 && i > 10) break; if (s.error) break; }
await page.locator('[id="framework.panel.history"]').first().click({ force: true }).catch(() => {});
await page.waitForTimeout(2500);
s = await state();
await shot("1-boot");
await note("boot", s, 0);

const graphRect = s.hosts.find((h) => /wfc-graph/.test(h.id ?? ""))?.rect;
const previewRect = s.hosts.find((h) => /preview/.test(h.id ?? ""))?.rect;

// ── 2. solve → the preview repaints ──────────────────────────────────────
{
  const from = lines.length;
  const before = await signature(previewRect);
  await shot("2a-preview-unsolved", previewRect);
  const run = await runAction("framework.window.wfc2dPreview", "solve");
  let after = before;
  for (let i = 0; i < 20; i++) { await page.waitForTimeout(500); after = await signature(previewRect); if (after !== before) break; }
  await shot("2b-preview-solved", previewRect);
  await note("solve", { ...run, before, after, repainted: after !== before, history: (await state()).history }, from);
}

// ── 3. example switch → the bitmap terrain-ring ──────────────────────────
const pickExample = async (label) => {
  const combo = page.locator('[role="combobox"]').first();
  if (!(await combo.count())) return "no-combobox";
  await combo.click({ timeout: 6000 }).catch(() => {});
  await page.waitForTimeout(800);
  const option = page.locator('[role="option"]').filter({ hasText: label }).first();
  if (!(await option.count())) { await page.keyboard.press("Escape"); return "no-option"; }
  const picked = await option.click({ timeout: 6000 }).then(() => `ok`).catch((e) => String(e).slice(0, 100));
  await page.waitForTimeout(4000);
  await page.keyboard.press("Escape").catch(() => {});
  return picked;
};
{
  const from = lines.length;
  const beforePreview = await signature(previewRect);
  const beforeGraph = await signature(graphRect);
  const picked = await pickExample(/Terrain Ring/i);
  await page.waitForTimeout(2500);
  const s2 = await state();
  await shot("3-terrain-ring");
  await note("example-switch", { picked, combobox: s2.combobox, previewRepainted: (await signature(previewRect)) !== beforePreview, graphRepainted: (await signature(graphRect)) !== beforeGraph, history: s2.history }, from);
}

// ── 4. re-select the boot example → no phantom edit ──────────────────────
{
  const from = lines.length;
  const before = (await state()).history;
  const picked = await pickExample(/Two Rooms/i);
  await page.waitForTimeout(3000);
  const s4 = await state();
  const minted = s4.history.filter((row) => !before.includes(row) && !FRAMEWORK_ROW.test(row));
  await note("example-reselect-boot", { picked, combobox: s4.combobox, historyBefore: before, historyAfter: s4.history, documentEntriesMinted: minted }, from);
}

// ── 5. find a slot node, drag it → one Move slot entry ───────────────────
/** 🔍️ The wasm node-graph canvas exposes no DOM node, so a slot node is FOUND by hovering: the dag
 * host logs `dag hover changed: <id>` the moment the pointer enters a node. The scan sweeps the pane,
 * then refines around the first hit and answers the CENTROID of every point that hovered the same
 * node — pressing a node's dead-centre is what starts a drag rather than a marquee. */
const findNode = async (skip = []) => {
  const cx = graphRect.x + graphRect.w / 2;
  const cy = graphRect.y + graphRect.h / 2;
  const span = Number(process.env.SEMIO_PROBE_SCAN_SPAN ?? 340);
  const step = Number(process.env.SEMIO_PROBE_SCAN_STEP ?? 18);
  const hoverAt = async (x, y) => {
    const mark = lines.length;
    await page.mouse.move(x, y);
    await page.waitForTimeout(65);
    return lines.slice(mark).map((l) => /dag hover changed: (\S+)/.exec(l)?.[1]).filter(Boolean).pop() ?? null;
  };
  let seed = null;
  scan: for (let dy = -span; dy <= span; dy += step) {
    for (let dx = -span; dx <= span; dx += step) {
      const hit = await hoverAt(cx + dx, cy + dy);
      if (hit && hit !== "—" && !skip.includes(hit)) { seed = { id: hit, x: cx + dx, y: cy + dy }; break scan; }
    }
  }
  if (!seed) return null;
  const points = [];
  let current = seed.id;
  for (let dy = -step * 2; dy <= step * 2; dy += 6) {
    for (let dx = -step * 2; dx <= step * 2; dx += 6) {
      const hit = await hoverAt(seed.x + dx, seed.y + dy);
      if (hit && hit !== "—") current = hit;
      if (current === seed.id) points.push({ x: seed.x + dx, y: seed.y + dy });
    }
  }
  if (points.length === 0) return seed;
  const xs = points.map((q) => q.x);
  const ys = points.map((q) => q.y);
  return { id: seed.id, x: Math.round((Math.min(...xs) + Math.max(...xs)) / 2), y: Math.round((Math.min(...ys) + Math.max(...ys)) / 2), samples: points.length, box: { w: Math.max(...xs) - Math.min(...xs), h: Math.max(...ys) - Math.min(...ys) } };
};
let firstNode = null;
{
  const from = lines.length;
  firstNode = await findNode();
  // 🫳️ The node box on this canvas is all ports, and a press on a port starts a WIRE instead of a
  // drag; a press on the node body drags it. The body is whatever is left, so the gesture is tried
  // from a few offsets around the node's centre until one lands the `move-slot` edit.
  const attempts = [];
  let moves = [];
  if (firstNode) {
    for (const [dx, dy] of [[0, 0], [0, -16], [0, 16], [-20, 0], [20, 0], [0, -26], [0, 26]]) {
      const start = { x: firstNode.x + dx, y: firstNode.y + dy };
      await page.mouse.move(start.x, start.y);
      await page.waitForTimeout(220);
      await page.mouse.down();
      await page.waitForTimeout(160);
      for (let i = 1; i <= 14; i++) { await page.mouse.move(start.x + i * 9, start.y + i * 6); await page.waitForTimeout(55); }
      await page.waitForTimeout(240);
      await page.mouse.up();
      await page.waitForTimeout(2500);
      moves = (await state()).history.filter((row) => /Move slot/i.test(row));
      attempts.push({ dx, dy, moves: moves.length });
      if (moves.length) break;
    }
  }
  await shot("5-graph-dragged", graphRect);
  await note("graph-drag", { node: firstNode, attempts, moves, historyAfter: (await state()).history }, from);
}

// ── 6. undo the drag ─────────────────────────────────────────────────────
{
  const from = lines.length;
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  let after = [];
  for (let i = 0; i < 20; i++) { await page.waitForTimeout(500); after = (await state()).history; if (!after.some((row) => /Move slot/i.test(row))) break; }
  await shot("6-after-undo", graphRect);
  await note("undo", { moveEntriesLeft: after.filter((row) => /Move slot/i.test(row)).length, historyAfter: after }, from);
}

// ── 7. wire two slot nodes on the canvas → one Connect entry ────────────
{
  const from = lines.length;
  // 🔭️ Zoom out first: the boot example's three slots do not all fit the pane at the default camera,
  // and a wire needs BOTH endpoints on screen.
  await page.mouse.move(graphRect.x + graphRect.w / 2, graphRect.y + graphRect.h / 2);
  for (let i = 0; i < Number(process.env.SEMIO_PROBE_ZOOM_OUT ?? 4); i++) { await page.mouse.wheel(0, 120); await page.waitForTimeout(220); }
  await page.waitForTimeout(1200);
  const found = [];
  for (let i = 0; i < 3; i++) {
    const next = await findNode(found.map((n) => n.id));
    if (!next) break;
    found.push(next);
  }
  // 🔗️ Pick a pair the document does not already wire, so the canvas' own acyclic/duplicate guard
  // cannot silently swallow the gesture.
  const a = found.find((n) => n.id === "room-a") ?? found[0];
  const b = found.find((n) => n.id === "room-b") ?? found.find((n) => n.id !== a?.id);
  const attempts = [];
  let connects = [];
  if (a && b) {
    for (const reach of [14, 18, 22, 26, 30, 34, 38, 42]) {
      const mark = lines.length;
      const out = { x: a.x + reach, y: a.y };
      const into = { x: b.x - reach, y: b.y };
      await page.mouse.move(out.x, out.y);
      await page.waitForTimeout(220);
      await page.mouse.down();
      await page.waitForTimeout(160);
      for (let i = 1; i <= 14; i++) { await page.mouse.move(out.x + ((into.x - out.x) * i) / 14, out.y + ((into.y - out.y) * i) / 14); await page.waitForTimeout(55); }
      await page.waitForTimeout(250);
      await page.mouse.up();
      await page.waitForTimeout(2500);
      const drawEdge = lines.slice(mark).some((l) => /draw-edge/.test(l));
      const rows = (await state()).history;
      connects = rows.filter((row) => /^Connect/i.test(row));
      attempts.push({ reach, drawEdge, connects: connects.length, disconnects: rows.filter((row) => /^Disconnect/i.test(row)).length });
      if (connects.length) break;
    }
  }
  await shot("7-graph-wired", graphRect);
  await note("graph-connect", { a, b, found: found.map((n) => n.id), attempts, connects, historyAfter: (await state()).history }, from);
}

// ── 8. pin / unpin from the Graph pane's Actions ─────────────────────────
{
  const from = lines.length;
  const run = await runAction("framework.window.wfcGraph", "pin-slot", { id: "room-a", tileId: "room" });
  const s8 = await state();
  await note("action-pin-slot", { ...run, pinned: s8.history.filter((row) => /Pin/i.test(row)), history: s8.history }, from);
}
{
  const from = lines.length;
  const run = await runAction("framework.window.wfcGraph", "unpin-slot", { id: "room-a" });
  const s8 = await state();
  await note("action-unpin-slot", { ...run, unpinned: s8.history.filter((row) => /Unpin/i.test(row)), history: s8.history }, from);
}
// ── 9. connect-slots through the Actions form (the canvas wire's typed twin) ──
{
  const from = lines.length;
  const run = await runAction("framework.window.wfcGraph", "connect-slots", { id: "edge-a-b", fromSlotId: "room-a", toSlotId: "room-b", relation: "" });
  const s9 = await state();
  await note("action-connect-slots", { ...run, connects: s9.history.filter((row) => /Connect/i.test(row)), history: s9.history }, from);
}
// ── 10. a verb aimed at an id the document does not hold is REFUSED by name ──
{
  const from = lines.length;
  const run = await runAction("framework.window.wfcGraph", "delete-slot", { id: "no-such-slot" });
  const s10 = await state();
  const refused = lines.slice(from).filter((l) => /unknown-slot/.test(l)).map((l) => l.slice(0, 200));
  await note("guarded-unknown-id", { ...run, refused, mintedNothing: !s10.history.some((row) => /Delete slot/i.test(row)) }, from, /unknown-slot/);
}

await shot("9-final");
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE steps", report.steps.length, "faults", report.steps.reduce((n, x) => n + x.faults.length, 0), "shellNotes", report.steps.reduce((n, x) => n + x.shellNotes.length, 0));
await browser.close();
