/** 🎛️ Grid2d interaction probe — the end-to-end proof that the 2D-grid editor is alive in the browser:
 *  1. boot (both panes render a painted canvas, no fault lines),
 *  2. the navbar example switcher moves pipes → terrain → pipes without a phantom edit,
 *  3. the `pin` utility armed on the Grid pane + one click = ONE history patch and repainted cells,
 *  4. mod+z reverts it,
 *  5. the `mask` utility armed + one click = one more patch, reverted again,
 *  6. the Preview pane's `solve` action paints the solved VECTOR assignment (pipes),
 *  7. and the same verb on the BITMAP example (terrain) paints its pixel tiles.
 * Every step records the console delta, its fault lines and a screenshot; an EMPTY `faults` array is
 * the pass condition for each step.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6042/?plugin=wfc SEMIO_PROBE_OUT=playground-grid2d/grid2d-interact bun 🐍️grid2d-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6042/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-grid2d/grid2d-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });

const GRID_SURFACE = "window:wfc-grid2d-grid";
const PREVIEW_SURFACE = "window:wfc-grid2d-preview";
const GRID_WINDOW = "framework.window.wfcGrid2dGrid";
const PREVIEW_WINDOW = "framework.window.wfcGrid2dPreview";

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) =>
  lines
    .slice(from)
    .filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey|Render error/.test(l))
    .map((l) => l.slice(0, 400));
const historyLines = (from) => lines.slice(from).filter((l) => /history patch applied|patchCursor/.test(l)).map((l) => l.slice(0, 220));
const state = () =>
  page.evaluate(() => {
    const fingerprint = (el) => {
      const canvas = el.querySelector("canvas");
      if (!canvas) return null;
      try {
        const probe = document.createElement("canvas");
        probe.width = 64;
        probe.height = 64;
        const ctx = probe.getContext("2d");
        ctx.drawImage(canvas, 0, 0, 64, 64);
        const data = ctx.getImageData(0, 0, 64, 64).data;
        const colors = new Set();
        let hash = 0;
        for (let i = 0; i < data.length; i += 4) {
          colors.add(`${data[i]},${data[i + 1]},${data[i + 2]}`);
          hash = (hash * 31 + data[i] * 7 + data[i + 1] * 3 + data[i + 2]) | 0;
        }
        const tally = new Map();
        for (let i = 0; i < data.length; i += 4) {
          const key = `${data[i]},${data[i + 1]},${data[i + 2]}`;
          tally.set(key, (tally.get(key) ?? 0) + 1);
        }
        const top = [...tally.entries()].sort((a, b) => b[1] - a[1]).slice(0, 4).map(([key, count]) => `${key}:${count}`);
        // 🀄️ The pipes tiles are drawn in one ink (`#38bdf8` = 56,189,248); its presence is the
        // honest "this canvas shows SOLVED tiles" signal, which a colour count alone never is.
        const near = (target, tolerance) =>
          [...tally.entries()]
            .filter(([key]) => key.split(",").map(Number).every((channel, index) => Math.abs(channel - target[index]) < tolerance))
            .reduce((sum, [, count]) => sum + count, 0);
        // 🎨️ One counter per CELL STATE, which is what makes a click's effect readable off the
        // bitmap: pinned cells paint `#2563eb`, masked ones `#7f1d1d`, and the pipes tiles' own ink
        // is `#38bdf8` — the honest "this canvas shows SOLVED tiles" signal.
        return { colors: colors.size, hash, top, inkPixels: near([56, 189, 248], 40), pinnedPixels: near([37, 99, 235], 45), maskedPixels: near([127, 29, 29], 45) };
      } catch (error) {
        return { error: String(error).slice(0, 120) };
      }
    };
    const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length, ink: fingerprint(el) }));
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      hosts,
      combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
      actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id).slice(0, 80),
      pressed: [...document.querySelectorAll("button[aria-pressed]")].map((el) => `${el.id}=${el.getAttribute("aria-pressed")}`).filter((t) => !t.startsWith("=")),
      buttonIds: [...document.querySelectorAll("button[id]")].map((el) => el.id).slice(0, 160),
      bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 600),
    };
  });
const ink = (s, id) => s.hosts.find((h) => h.id === id)?.ink ?? null;
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from), history: historyLines(from).slice(0, 8) });
  console.log(`[DEBUG] ${step} faults=${faultLines(from).length} ${JSON.stringify(detail).slice(0, 900)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const click = async (selector) => {
  const target = page.locator(selector).first();
  if (!(await target.count())) return `absent:${selector}`;
  return target.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 140));
};
/** 🧰️ Arms one utility on one window: unfold that window's utility bar, then press the leaf whose id
 *  or accessible name names the utility. The leaf id is the plugin's own utility id. */
const arm = async (windowId, utility) => {
  const unfolded = await click(`[id="${windowId}.utilityBar.unfold"]`);
  await page.waitForTimeout(1200);
  const candidates = await page.evaluate(() => [...document.querySelectorAll("button[id]")].map((el) => ({ id: el.id, label: (el.getAttribute("aria-label") ?? el.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 40), pressed: el.getAttribute("aria-pressed") })));
  const leaf = candidates.find((c) => c.id === utility) ?? candidates.find((c) => c.id.endsWith(`/${utility}`) || c.id.endsWith(`.${utility}`)) ?? candidates.find((c) => new RegExp(`^${utility}$`, "i").test(c.label));
  const pressed = leaf ? await click(`[id="${leaf.id}"]`) : "no-leaf";
  await page.waitForTimeout(1500);
  return { unfolded, leaf: leaf?.id ?? null, pressed, candidates: candidates.map((c) => c.id).slice(0, 60) };
};
/** 🖱️ One press on a surface's own canvas. Playwright's synthesised CDP mouse input never reaches
 *  this canvas in headless Chromium (verified: `elementsFromPoint` puts the canvas on top with
 *  `pointer-events: auto`, yet `page.mouse.down()` produces no guest dispatch at all), so the press
 *  is delivered as real `PointerEvent`s on the canvas element — the same events the browser would
 *  synthesise from a human click, at the same coordinates. */
const clickSurface = async (surfaceId, dx = 0.5, dy = 0.5) => {
  const box = await page.locator(`[data-surface-id="${surfaceId}"]`).first().boundingBox();
  if (!box) return { clicked: `absent:${surfaceId}` };
  const delivered = await page.evaluate(
    ({ surfaceId, dx, dy }) => {
      const canvas = document.querySelector(`[data-surface-id="${surfaceId}"] canvas`);
      if (!canvas) return "no-canvas";
      const rect = canvas.getBoundingClientRect();
      const clientX = rect.left + rect.width * dx;
      const clientY = rect.top + rect.height * dy;
      const base = { bubbles: true, cancelable: true, clientX, clientY, pointerId: 1, isPrimary: true, pointerType: "mouse", button: 0 };
      canvas.dispatchEvent(new PointerEvent("pointermove", { ...base, buttons: 0 }));
      canvas.dispatchEvent(new PointerEvent("pointerdown", { ...base, buttons: 1 }));
      canvas.dispatchEvent(new PointerEvent("pointerup", { ...base, buttons: 0 }));
      return `ok@${Math.round(clientX)},${Math.round(clientY)}`;
    },
    { surfaceId, dx, dy },
  );
  return { clicked: delivered, box: { w: Math.round(box.width), h: Math.round(box.height) } };
};
const patches = (from, pattern) => lines.slice(from).filter((l) => /history patch applied/.test(l) && pattern.test(l)).length;

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length >= 2 && i > 15) break; if (s.error) break; }
await note("boot", { ready: s.ready, error: s.error, hosts: s.hosts, combobox: s.combobox, bodyHead: s.bodyHead }, 0);
const bootInk = { grid: ink(s, GRID_SURFACE), preview: ink(s, PREVIEW_SURFACE) };

// ── 2. navbar example switcher ────────────────────────────────────────────
for (const example of ["Terrain", "Pipes"]) {
  const from = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  let picked = "no-combobox";
  let options = [];
  if (await combo.count()) {
    await combo.click({ timeout: 6000 }).catch(() => {});
    await page.waitForTimeout(700);
    options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator('[role="option"]').filter({ hasText: example }).first();
    if (await option.count()) picked = await option.click({ timeout: 6000 }).then(() => `ok:${example}`).catch((e) => String(e).slice(0, 120));
    else { picked = "no-option"; await page.keyboard.press("Escape"); }
  }
  let after = null;
  for (let i = 0; i < settleSeconds * 3; i++) { await page.waitForTimeout(500); after = await state(); if (after.combobox === example) break; }
  await page.keyboard.press("Escape").catch(() => {});
  await page.waitForTimeout(1500);
  after = await state();
  await note(`example-${example.toLowerCase()}`, { picked, options, combobox: after.combobox, gridInk: ink(after, GRID_SURFACE), previewInk: ink(after, PREVIEW_SURFACE), edits: patches(from, /./) }, from);
  s = after;
}

// ── 3. pin utility + one click on the grid ────────────────────────────────
let armedPin = null;
{
  const from = lines.length;
  armedPin = await arm(GRID_WINDOW, "pin");
  const beforePin = await state();
  const pressed = await clickSurface(GRID_SURFACE, 0.5, 0.5);
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if ((ink(after, GRID_SURFACE)?.pinnedPixels ?? 0) > (ink(beforePin, GRID_SURFACE)?.pinnedPixels ?? 0)) break; }
  await page.waitForTimeout(1200);
  after = await state();
  await note("pin-click", { armedPin, pressed, pinPatches: patches(from, /Pin cell/), totalPatches: patches(from, /./), gridInkBefore: ink(beforePin, GRID_SURFACE), gridInk: ink(after, GRID_SURFACE) }, from);
  s = after;
}

// ── 4. undo ───────────────────────────────────────────────────────────────
{
  const from = lines.length;
  const beforeUndo = await state();
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if ((ink(after, GRID_SURFACE)?.pinnedPixels ?? 0) < (ink(beforeUndo, GRID_SURFACE)?.pinnedPixels ?? 0)) break; }
  await page.waitForTimeout(1200);
  after = await state();
  await note("undo-pin", { undoPatches: patches(from, /Undo|undo/), gridInkBeforeUndo: ink(beforeUndo, GRID_SURFACE), gridInk: ink(after, GRID_SURFACE), bootGridInk: bootInk.grid }, from);
  s = after;
}

// ── 5. mask utility + one click, then undo ────────────────────────────────
{
  const from = lines.length;
  const armedMask = await arm(GRID_WINDOW, "mask");
  const beforeMask = await state();
  // 🎯️ Cell (1, 1), not "somewhere left of centre": a 6 × 6 grid of 32-unit cells fills only ~24 %
  // of a 789 × 907 pane at zoom 1, so 0.35 lands OUTSIDE the grid and is correctly a no-op.
  const pressed = await clickSurface(GRID_SURFACE, 0.46, 0.48);
  let masked = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); masked = await state(); if ((ink(masked, GRID_SURFACE)?.maskedPixels ?? 0) > (ink(beforeMask, GRID_SURFACE)?.maskedPixels ?? 0)) break; }
  await page.waitForTimeout(1200);
  masked = await state();
  await note("mask-click", { armedMask: { leaf: armedMask.leaf, pressed: armedMask.pressed }, pressed, maskPatches: patches(from, /Mask cell|Unmask cell/), gridInkBefore: ink(beforeMask, GRID_SURFACE), gridInk: ink(masked, GRID_SURFACE) }, from);
  const undoFrom = lines.length;
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  let reverted = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); reverted = await state(); if ((ink(reverted, GRID_SURFACE)?.maskedPixels ?? 0) <= (ink(beforeMask, GRID_SURFACE)?.maskedPixels ?? 0)) break; }
  await page.waitForTimeout(1200);
  reverted = await state();
  await note("undo-mask", { gridInk: ink(reverted, GRID_SURFACE), gridInkBeforeMask: ink(beforeMask, GRID_SURFACE) }, undoFrom);
  s = reverted;
}

// ── 6. solve on the preview pane ──────────────────────────────────────────
{
  const from = lines.length;
  const beforeInk = ink(s, PREVIEW_SURFACE);
  const opened = await click(`[id="${PREVIEW_WINDOW}.engagement.toggle"]`);
  await page.waitForTimeout(1800);
  const mid = await state();
  const clicked = await click('[id="action.solve"]');
  await page.waitForTimeout(1500);
  const executed = await click('[id$=".action.solve.execute"]');
  let after = null;
  for (let i = 0; i < settleSeconds * 4; i++) { await page.waitForTimeout(500); after = await state(); const now = ink(after, PREVIEW_SURFACE); if (now && beforeInk && now.hash !== beforeInk.hash) break; }
  await page.waitForTimeout(2000);
  after = await state();
  await note("solve", { opened, clicked, executed, actionRows: mid.actionRows, previewInkBefore: beforeInk, previewInkAfter: ink(after, PREVIEW_SURFACE), bodyHead: after.bodyHead }, from);
}

// ── 7. the OTHER example solved too: terrain's tiles are BITMAPS, a different render path ──
{
  const from = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 6000 }).catch(() => {});
  await page.waitForTimeout(700);
  const option = page.locator('[role="option"]').filter({ hasText: "Terrain" }).first();
  const picked = (await option.count()) ? await option.click({ timeout: 6000 }).then(() => "ok:Terrain").catch((e) => String(e).slice(0, 120)) : "no-option";
  await page.keyboard.press("Escape").catch(() => {});
  let switched = null;
  for (let i = 0; i < settleSeconds * 3; i++) { await page.waitForTimeout(500); switched = await state(); if (switched.combobox === "Terrain") break; }
  await page.waitForTimeout(1500);
  switched = await state();
  const unsolved = ink(switched, PREVIEW_SURFACE);
  // 🎯️ Re-press the SAME Actions row and it disarms instead of running, so the pane is closed and
  // reopened first — the row is then freshly armed by the pane, exactly as in step 6.
  await click(`[id="${PREVIEW_WINDOW}.engagement.toggle"]`);
  await page.waitForTimeout(900);
  await click(`[id="${PREVIEW_WINDOW}.engagement.toggle"]`);
  await page.waitForTimeout(1500);
  const clicked = await click('[id="action.solve"]');
  await page.waitForTimeout(1200);
  // ⌨️ The row is armed, not necessarily run: the framework's own run chord starts the armed action.
  const chord = process.platform === "darwin" ? "Meta+Enter" : "Control+Enter";
  await page.keyboard.press(chord).catch(() => {});
  let after = null;
  for (let i = 0; i < settleSeconds * 4; i++) { await page.waitForTimeout(500); after = await state(); const now = ink(after, PREVIEW_SURFACE); if (now && unsolved && now.hash !== unsolved.hash) break; }
  await page.waitForTimeout(2000);
  after = await state();
  await note("solve-terrain", { picked, clicked, chord, combobox: after.combobox, previewInkUnsolved: unsolved, previewInkSolved: ink(after, PREVIEW_SURFACE) }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE steps", report.steps.length, "faulty steps", report.steps.filter((step) => step.faults.length).map((step) => step.step));
await browser.close();
