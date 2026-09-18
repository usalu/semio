/** 🎛️ grid3d interaction probe. Boots the wfc grid3d react playground and proves, step by step, that
 * the editor is genuinely interactive in the browser:
 *   1. boot — both World3d hosts render, instance counts match the document, no fault line;
 *   2. arm a tile (`setActiveTile`, window-config lane) from the Grid window's Actions pane;
 *   3. arm the `pin` utility, then PICK A CELL ON THE CANVAS — the scene drops its `domainId` under a
 *      writing utility, so the host's plugin-private `worldSelect` carries the cell key to `pickCell`;
 *   4. mask a cell through the Actions pane (`maskCell`);
 *   5. undo — both edits revert, proving they landed in the document store, not in a view;
 *   6. switch examples with the navbar picker and switch back; the boot example must re-select with NO
 *      phantom edit (`canUndo` stays false).
 * Every step records its console delta, its fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6044/?plugin=wfc SEMIO_PROBE_OUT=playground-grid3d/interact bun 🐍️grid3d-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6044/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-grid3d/interact");
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 8000 : 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) =>
  lines
    .slice(from)
    .filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|undeclared-action|app\.command\.unsupported|refused|DuplicateSiblingKey|interactive-ceiling|quarantin/i.test(l))
    .map((l) => l.slice(0, 500));
const patchLines = (from) => lines.slice(from).filter((l) => /history patch applied/.test(l)).map((l) => l.slice(0, 220));

const state = () =>
  page.evaluate(() => {
    const parse = (s) => {
      try {
        return JSON.parse(s);
      } catch {
        return null;
      }
    };
    const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
      const instances = parse(el.getAttribute("data-instances-json")) ?? [];
      const byMesh = {};
      for (const row of Array.isArray(instances) ? instances : []) byMesh[row.meshId] = (byMesh[row.meshId] ?? 0) + 1;
      return {
        id: el.getAttribute("data-surface-id"),
        meshes: (parse(el.getAttribute("data-meshes-json")) ?? []).length,
        instances: Array.isArray(instances) ? instances.length : 0,
        byMesh,
        status: parse(el.getAttribute("data-status-json")),
        canvases: el.querySelectorAll("canvas").length,
      };
    });
    const pressed = [...document.querySelectorAll("button[aria-pressed]")].map((el) => ({ id: el.id, pressed: el.getAttribute("aria-pressed") })).filter((t) => t.id);
    const undo = document.querySelector('[id="undo"], [id$=".undo"], button[aria-label="Undo"]');
    return {
      ready: document.documentElement.getAttribute("data-semio-os-ready"),
      error: document.documentElement.getAttribute("data-semio-os-error"),
      hosts,
      canUndo: undo ? undo.getAttribute("aria-disabled") !== "true" && !undo.hasAttribute("disabled") : null,
      example: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
      actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id).slice(0, 80),
      engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
      utilityButtons: pressed.filter((t) => /utilit|select|pin|mask/i.test(t.id)).slice(0, 30),
      pressedIds: pressed.slice(0, 60),
      bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 700),
    };
  });

const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from), patches: patchLines(from).slice(0, 8) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1600)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};

const click = async (selector, label) => {
  const target = page.locator(selector).first();
  if (!(await target.count())) return `absent:${label}`;
  return await target
    .click({ timeout: 8000, force: true })
    .then(() => `ok:${label}`)
    .catch((e) => `${label}:${String(e).slice(0, 120)}`);
};

const GRID_WINDOW = "framework.window.wfcGrid3dGrid";

/** 🎬️ Opens the Grid window's Actions pane (idempotently — the toggle collapses an open pane), fills
 * one action's staged form and presses its Execute control. */
const runAction = async (actionId, fields) => {
  const toggle = page.locator(`[id="${GRID_WINDOW}.engagement.toggle"]`).first();
  let opened = "absent";
  if (await toggle.count()) {
    const pressed = await toggle.getAttribute("aria-pressed");
    const rowVisible = await page.locator(`[id="action.${actionId}"]`).count();
    opened = pressed === "true" || rowVisible ? "already-open" : await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  }
  await page.waitForTimeout(1200);
  // 🧾️ Selecting a row unfolds its staged form; selecting another while one is open only switches the
  // selection, so the form may need a second press before its Execute control exists.
  let row = await click(`[id="action.${actionId}"]`, `row.${actionId}`);
  await page.waitForTimeout(1500);
  for (let attempt = 0; attempt < 3 && !(await page.locator(`[id="${GRID_WINDOW}.action.${actionId}.execute"]`).count()); attempt += 1) {
    row = `${row}+retry${attempt}:${await click(`[id="action.${actionId}"]`, `row.${actionId}`)}`;
    await page.waitForTimeout(1500);
  }
  const filled = {};
  for (const [name, value] of Object.entries(fields)) {
    const input = page.locator(`[id="action.${actionId}.arg.${name}"] input, [id="action.${actionId}.arg.${name}"] select, [id="action.${actionId}.arg.${name}"] textarea`).first();
    if (!(await input.count())) {
      filled[name] = "absent";
      continue;
    }
    filled[name] = await input
      .fill(String(value), { timeout: 5000 })
      .then(() => "ok")
      .catch((e) => String(e).slice(0, 100));
  }
  await page.waitForTimeout(500);
  const executed = await click(`[id="${GRID_WINDOW}.action.${actionId}.execute"]`, `execute.${actionId}`);
  await page.waitForTimeout(2500);
  return { opened, row, filled, executed };
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) {
  await page.waitForTimeout(1000);
  s = await state();
  if (s.ready && s.hosts.length && s.hosts.every((h) => h.instances > 0) && i > 10) break;
  if (s.error) break;
}
await note("boot", s, 0);
const bootExample = s.example;

// ── 2. arm a tile (window-config lane, never the document) ────────────────
{
  const from = lines.length;
  const tile = process.env.SEMIO_PROBE_TILE ?? "floor";
  const ran = await runAction("setActiveTile", { tileId: tile });
  const after = await state();
  await note("arm-tile", { tile, ...ran, actionRows: after.actionRows, hosts: after.hosts }, from);
}

// ── 3. arm the pin utility, then pick a cell ON THE CANVAS ────────────────
{
  const from = lines.length;
  const utilities = await click(`[id="${GRID_WINDOW}.utilityBar.unfold"]`, "utilityBar");
  await page.waitForTimeout(1500);
  const before = await state();
  const pin = await click(`[id="${GRID_WINDOW}.utilityBar.pin"], [id$="utilityBar.pin"], [id="pin"], button[aria-label="Pin"]`, "pin");
  await page.waitForTimeout(2000);
  const box = await page.locator('[data-surface-id="window:wfc-grid3d-grid"] canvas').first().boundingBox();
  let picked = "no-canvas";
  if (box) {
    await page.mouse.move(box.x + box.width * 0.5, box.y + box.height * 0.45);
    await page.waitForTimeout(500);
    await page.mouse.down();
    await page.waitForTimeout(80);
    await page.mouse.up();
    picked = `ok@${Math.round(box.x + box.width * 0.5)},${Math.round(box.y + box.height * 0.45)}`;
  }
  let after = null;
  for (let i = 0; i < settle * 2; i++) {
    await page.waitForTimeout(500);
    after = await state();
    if (patchLines(from).length) break;
  }
  const dispatched = lines.slice(from).filter((l) => /worldSelect|pickCell|interactionSelect/.test(l)).map((l) => l.slice(0, 260));
  await note("pin-pick-on-canvas", { utilities, pin, picked, utilityButtons: before.utilityButtons, dispatched: dispatched.slice(0, 8), hosts: after.hosts, canUndo: after.canUndo }, from);
}

// ── 4. arm the mask utility and pick a second cell on the canvas ──────────
{
  const from = lines.length;
  const before = await state();
  const masked = (st) => st.hosts.find((h) => /grid3d-grid/.test(h.id ?? ""))?.byMesh?.["cell-masked"] ?? 0;
  const start = masked(before);
  const mask = await click(`[id="${GRID_WINDOW}.utilityBar.mask"], [id$="utilityBar.mask"], [id="mask"]`, "mask");
  await page.waitForTimeout(2000);
  const box = await page.locator('[data-surface-id="window:wfc-grid3d-grid"] canvas').first().boundingBox();
  let picked = "no-canvas";
  if (box) {
    const x = box.x + box.width * 0.56;
    const y = box.y + box.height * 0.38;
    await page.mouse.move(x, y);
    await page.waitForTimeout(500);
    await page.mouse.down();
    await page.waitForTimeout(80);
    await page.mouse.up();
    picked = `ok@${Math.round(x)},${Math.round(y)}`;
  }
  let after = null;
  for (let i = 0; i < settle * 2; i++) {
    await page.waitForTimeout(500);
    after = await state();
    if (masked(after) > start) break;
  }
  await note("mask-pick-on-canvas", { mask, picked, maskedBefore: start, maskedAfter: masked(after), hosts: after.hosts, canUndo: after.canUndo }, from);
}

// ── 5. undo ──────────────────────────────────────────────────────────────
{
  const from = lines.length;
  // 🧮️ The grid window paints one instance per cell, keyed by mesh: a pin is `cell-pin:<tile>` and a
  // mask is `cell-masked`, so "did the edit revert" is a count over the published scene, not a guess.
  const edited = (st) => {
    const grid = st.hosts.find((h) => /grid3d-grid/.test(h.id ?? ""))?.byMesh ?? {};
    return Object.entries(grid).reduce((total, [mesh, count]) => (mesh.startsWith("cell-pin:") || mesh === "cell-masked" ? total + count : total), 0);
  };
  let after = await state();
  const start = edited(after);
  let presses = 0;
  const target = start - 2;
  for (let attempt = 0; attempt < 8 && edited(after) > target; attempt += 1) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
    presses += 1;
    for (let i = 0; i < settle; i++) {
      await page.waitForTimeout(500);
      after = await state();
      if (edited(after) <= target) break;
    }
  }
  await note("undo", { presses, editedBefore: start, editedAfter: edited(after), bothEditsReverted: edited(after) <= target, undoPatches: patchLines(from).length, hosts: after.hosts }, from);
}

// ── 6. example picker — switch away and back, with no phantom edit ────────
{
  const from = lines.length;
  const target = process.env.SEMIO_PROBE_EXAMPLE ?? "Pipes";
  const pick = async (text) => {
    const combo = page.locator('[role="combobox"]').first();
    if (!(await combo.count())) return "no-combobox";
    await combo.click({ timeout: 5000 }).catch(() => {});
    await page.waitForTimeout(700);
    const options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator('[role="option"]').filter({ hasText: text }).first();
    if (!(await option.count())) {
      await page.keyboard.press("Escape");
      return `no-option:${text}:${options.join("|")}`;
    }
    const clicked = await option
      .click({ timeout: 5000 })
      .then(() => `ok:${text}`)
      .catch((e) => String(e).slice(0, 120));
    return clicked;
  };
  const away = await pick(target);
  let after = null;
  for (let i = 0; i < settle * 3; i++) {
    await page.waitForTimeout(500);
    after = await state();
    if (after.example && new RegExp(target, "i").test(after.example)) break;
  }
  await note("example-switch-away", { away, example: after.example, hosts: after.hosts }, from);

  const from2 = lines.length;
  const back = await pick(bootExample ?? "Building");
  for (let i = 0; i < settle * 3; i++) {
    await page.waitForTimeout(500);
    after = await state();
    if (after.example && after.example === bootExample) break;
  }
  await page.waitForTimeout(2500);
  after = await state();
  await note("example-switch-back", { back, bootExample, example: after.example, canUndo: after.canUndo, patches: patchLines(from2).length, hosts: after.hosts }, from2);

  // 📚️ Re-selecting the example ALREADY open must emit nothing at all — a picker that answers a
  // document swap here mints a phantom edit on a document nobody changed.
  const from3 = lines.length;
  const again = await pick(bootExample ?? "Building");
  await page.waitForTimeout(4000);
  const settled = await state();
  await note("example-reselect-is-inert", { again, example: settled.example, patches: patchLines(from3).length, hosts: settled.hosts }, from3);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length, "faulty steps:", report.steps.filter((s) => s.faults.length).map((s) => s.step).join(",") || "none");
await browser.close();
