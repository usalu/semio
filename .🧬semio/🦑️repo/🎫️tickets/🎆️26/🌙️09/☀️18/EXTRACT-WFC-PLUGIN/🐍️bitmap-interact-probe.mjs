/** 🎛️ Bitmap interaction probe (react playground 6041). Steps:
 *  1. boot — shell beacon, both canvas hosts, the pane chrome ids;
 *  2. solve — run `solve` from the Output window's Actions pane and MEASURE the output pane's pixels
 *     (a blank output is one flat colour; a real collapse is the input's palette);
 *  3. arm + paint — `set-active-color`, `stroke-begin`, `stroke-extend`, `stroke-commit` from the Input
 *     window's Actions pane, asserting EXACTLY ONE `set-input-pixels` history patch for the whole drag;
 *  4. undo — `mod+z` reverts the paint (the edit really landed in the document store);
 *  5. pin — `pin-pixel` on the Output window;
 *  6. example switch — the navbar combobox picks the other example and the document really changes.
 * Every step records its console delta, its fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=playground-bitmap/interact bun 🐍️bitmap-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6041/?plugin=wfc";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "playground-bitmap/interact");
const settle = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 8000 : 800)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const FAULT = /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not a framework-reserved|do not decode|refused|DuplicateSiblingKey/;
const faultLines = (from) => lines.slice(from).filter((l) => FAULT.test(l)).map((l) => l.slice(0, 500));
const historyLines = (from) => lines.slice(from).filter((l) => /history patch applied/.test(l)).map((l) => l.slice(0, 260));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from), history: historyLines(from).slice(0, 12) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1800)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};

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
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    argRows: [...document.querySelectorAll('[id*=".arg."]')].map((el) => el.id),
    executeIds: [...document.querySelectorAll('[id$=".execute"]')].map((el) => el.id),
    panelIds: [...document.querySelectorAll('[id*="panel"], [id*="history"], [id*="History"]')].map((el) => el.id).slice(0, 60),
    treeRows: [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 90)).slice(0, 60),
    undoDisabled: document.getElementById("action.undo")?.getAttribute("aria-disabled") ?? null,
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 400),
  };
});

/** 🎨️ Distinct-colour census of one screen region, decoded through a 2D canvas inside the page
 * (a WebGL drawing buffer is not readable after compositing, a screenshot always is). A blank output
 * pane is ONE flat colour at ~100 % share; a real collapse carries the input's palette. */
const measure = async (rect, name) => {
  if (!rect || rect.w < 8 || rect.h < 8) return { name, error: "no-rect" };
  const clip = { x: rect.x + 4, y: rect.y + 4, width: rect.w - 8, height: rect.h - 8 };
  const buffer = await page.screenshot({ clip });
  writeFileSync(join(outDir, `region-${name}.png`), buffer);
  const stats = await page.evaluate(async (dataUrl) => {
    const image = new Image();
    image.src = dataUrl;
    await image.decode();
    const canvas = document.createElement("canvas");
    canvas.width = image.width;
    canvas.height = image.height;
    const ctx = canvas.getContext("2d");
    ctx.drawImage(image, 0, 0);
    const data = ctx.getImageData(0, 0, canvas.width, canvas.height).data;
    const counts = new Map();
    for (let i = 0; i < data.length; i += 4) {
      const key = (data[i] << 16) | (data[i + 1] << 8) | data[i + 2];
      counts.set(key, (counts.get(key) ?? 0) + 1);
    }
    const total = data.length / 4;
    const top = [...counts.entries()].sort((a, b) => b[1] - a[1]).slice(0, 6).map(([key, n]) => ({ hex: `#${key.toString(16).padStart(6, "0")}`, share: Number((n / total).toFixed(4)) }));
    return { distinct: counts.size, total, top };
  }, `data:image/png;base64,${buffer.toString("base64")}`);
  return { name, ...stats };
};

const rectOf = (s, surfaceId) => s.hosts.find((h) => h.id === surfaceId)?.rect ?? null;

/** 🆔️ The shell normalises every element-id segment to camelCase (`childElementId`/`elementIdSegment`),
 * so a kebab action id like `stroke-begin` names its execute control `…action.strokeBegin.execute`.
 * Addressing the raw kebab id finds nothing and the staged form is never submitted — which looks
 * exactly like a verb that ran and did nothing. */
const idSegment = (raw) => {
  let segment = "";
  let capitalize = false;
  for (const ch of raw) {
    if (ch === "-" || ch === "_" || ch === " " || ch === ".") { capitalize = true; continue; }
    if (!/[a-zA-Z0-9]/.test(ch)) continue;
    if (segment.length === 0) segment += ch.toLowerCase();
    else if (capitalize) { segment += ch.toUpperCase(); capitalize = false; }
    else segment += ch;
  }
  return segment;
};

/** 🎛️ The Actions pane of ONE window. Two readbacks are unusable: `aria-pressed` on the engagement
 * toggle never flips, and the pane is NOT a DOM descendant of its window element — so the pane's real
 * state is read from the presence of a row only THAT window declares, with exactly one pane open at a
 * time. Toggling blind closes a pane that was already open, and every later row lookup then answers
 * `absent-row`, which looks exactly like an undeclared action. */
const PANE_WITNESS = { wfcBitmapInput: "change-seed", wfcBitmapOutput: "solve" };
const rowCount = (actionId) => page.locator(`[id="action.${actionId}"]`).count();
const anyRowCount = () => page.locator('[id^="action."]').count();

const openPane = async (windowKey) => {
  const witness = PANE_WITNESS[windowKey];
  for (let attempt = 0; attempt < 4; attempt += 1) {
    if ((await rowCount(witness)) > 0) return `open:${windowKey}`;
    await page.locator(`[id="framework.window.${windowKey}.engagement.toggle"]`).first().click({ timeout: 8000, force: true }).catch(() => {});
    await page.waitForTimeout(1600);
  }
  return `failed-open:${windowKey}`;
};

const closePane = async (windowKey) => {
  for (let attempt = 0; attempt < 4; attempt += 1) {
    if ((await anyRowCount()) === 0) return `closed:${windowKey}`;
    await page.locator(`[id="framework.window.${windowKey}.engagement.toggle"]`).first().click({ timeout: 8000, force: true }).catch(() => {});
    await page.waitForTimeout(1600);
  }
  return `failed-close:${windowKey}`;
};

/** 🎬️ Runs one action from an open Actions pane. An arg-less row executes on the row click itself; a
 * staged row expands, takes its argument values and then presses its own execute control. */
const runAction = async (windowKey, actionId, args = {}) => {
  const trace = { windowKey, actionId, args };
  const scope = "";
  let row = page.locator(`${scope}[id="action.${actionId}"]`).first();
  for (let attempt = 0; attempt < 6 && !(await row.count()); attempt += 1) {
    await page.waitForTimeout(1000);
    row = page.locator(`${scope}[id="action.${actionId}"]`).first();
  }
  if (!(await row.count())) return { ...trace, clicked: "absent-row" };
  trace.clicked = await row.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 140));
  await page.waitForTimeout(900);
  const staged = [];
  for (const [key, value] of Object.entries(args)) {
    const holder = page.locator(`${scope}[id="action.${actionId}.arg.${key}"]`).first();
    if (!(await holder.count())) { staged.push(`${key}:absent`); continue; }
    const field = holder.locator("input, textarea, select").first();
    if (!(await field.count())) { staged.push(`${key}:no-control`); continue; }
    const type = await field.getAttribute("type");
    if (type === "checkbox") {
      const checked = await field.isChecked();
      if (checked !== Boolean(value)) await field.click({ force: true }).catch(() => {});
      staged.push(`${key}:toggled`);
      continue;
    }
    staged.push(await field.fill(String(value), { timeout: 5000 }).then(() => `${key}:ok`).catch((e) => `${key}:${String(e).slice(0, 60)}`));
    await field.press("Tab").catch(() => {});
  }
  trace.staged = staged;
  await page.waitForTimeout(600);
  const execute = page.locator(`${scope}[id$=".action.${idSegment(actionId)}.execute"]`).first();
  trace.executed = (await execute.count()) ? await execute.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 140)) : "row-click-only";
  await page.waitForTimeout(1200);
  return trace;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length >= 2 && i > 10) break; if (s.error) break; }
const inputRect0 = rectOf(s, "window:wfc-bitmap-input");
const outputRect0 = rectOf(s, "window:wfc-bitmap-output");
await note("boot", { ...s, inputPixels: await measure(inputRect0, "boot-input"), outputPixels: await measure(outputRect0, "boot-output") }, 0);

// ── 1b. change-seed — one single-mutation document verb, to prove the artifact lane ────────
{
  const from = lines.length;
  const opened = await openPane("wfcBitmapInput");
  const run = await runAction("wfcBitmapInput", "change-seed", { seed: 4242 });
  await page.waitForTimeout(3000);
  await closePane("wfcBitmapInput");
  await page.waitForTimeout(1500);
  const after = await state();
  await note("change-seed", { opened, run, undoDisabled: after.undoDisabled, hosts: after.hosts }, from);
}

// ── 2. solve ───────────────────────────────────────────────────────────────
{
  const from = lines.length;
  const opened = await openPane("wfcBitmapOutput");
  const rows = (await state()).actionRows;
  const run = await runAction("wfcBitmapOutput", "solve");
  let after = null;
  for (let i = 0; i < settle * 3; i++) { await page.waitForTimeout(500); after = await state(); }
  await closePane("wfcBitmapOutput");
  await page.waitForTimeout(2000);
  after = await state();
  const outputPixels = await measure(rectOf(after, "window:wfc-bitmap-output"), "after-solve-output");
  const inputPixels = await measure(rectOf(after, "window:wfc-bitmap-input"), "after-solve-input");
  await note("solve", { opened, rows, run, outputPixels, inputPixels, hosts: after.hosts }, from);
}

// ── 3. arm the palette and paint one stroke ────────────────────────────────
let paintedHistory = [];
{
  const from = lines.length;
  const opened = await openPane("wfcBitmapInput");
  const rows = (await state()).actionRows;
  const arm = await runAction("wfcBitmapInput", "set-active-color", { index: 2 });
  const begin = await runAction("wfcBitmapInput", "stroke-begin", { x: 6, y: 6 });
  const extend = await runAction("wfcBitmapInput", "stroke-extend", { x: 9, y: 9 });
  const commit = await runAction("wfcBitmapInput", "stroke-commit");
  let after = null;
  for (let i = 0; i < settle * 2; i++) { await page.waitForTimeout(500); after = await state(); }
  await closePane("wfcBitmapInput");
  await page.waitForTimeout(2000);
  after = await state();
  paintedHistory = historyLines(from).filter((l) => /set-input-pixels/.test(l));
  const inputPixels = await measure(rectOf(after, "window:wfc-bitmap-input"), "after-paint-input");
  await note("paint-stroke", { opened, rows, arm, begin, extend, commit, setInputPixelsPatches: paintedHistory.length, undoDisabled: after.undoDisabled, treeRows: after.treeRows, inputPixels, hosts: after.hosts }, from);
}

// ── 4. undo ────────────────────────────────────────────────────────────────
{
  const from = lines.length;
  const presses = [];
  let inputPixels = null;
  for (let attempt = 0; attempt < 3; attempt += 1) {
    await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
    presses.push(attempt + 1);
    await page.waitForTimeout(2500);
    inputPixels = await measure(rectOf(await state(), "window:wfc-bitmap-input"), `after-undo-${attempt + 1}-input`);
    if (historyLines(from).some((l) => /Undo/i.test(l))) break;
  }
  const after = await state();
  await note("undo", { presses: presses.length, undoPatches: historyLines(from).filter((l) => /Undo/i.test(l)).length, inputPixels, hosts: after.hosts }, from);
}

// ── 5. pin a pixel on the output window ────────────────────────────────────
{
  const from = lines.length;
  const opened = await openPane("wfcBitmapOutput");
  const run = await runAction("wfcBitmapOutput", "pin-pixel", { x: 3, y: 3, color: 2 });
  await page.waitForTimeout(2500);
  await closePane("wfcBitmapOutput");
  await page.waitForTimeout(2000);
  const after = await state();
  await note("pin-pixel", { opened, run, pinPatches: historyLines(from).filter((l) => /pin-pixel/.test(l)).length, undoDisabled: after.undoDisabled, outputPixels: await measure(rectOf(after, "window:wfc-bitmap-output"), "after-pin-output"), hosts: after.hosts }, from);
}

// ── 6. example switch through the navbar combobox ──────────────────────────
{
  const from = lines.length;
  const wanted = process.env.SEMIO_PROBE_EXAMPLE ?? "Flowers 24";
  const combo = page.locator('[role="combobox"]').first();
  let picked = "absent";
  let options = [];
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch((e) => (picked = String(e).slice(0, 120)));
    await page.waitForTimeout(800);
    options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator('[role="option"]').filter({ hasText: wanted }).first();
    if (await option.count()) picked = await option.click({ timeout: 5000 }).then(() => `ok:${wanted}`).catch((e) => String(e).slice(0, 120));
    else { picked = "no-option"; await page.keyboard.press("Escape"); }
  }
  let after = null;
  for (let i = 0; i < settle * 4; i++) { await page.waitForTimeout(500); after = await state(); if (after.combobox && after.combobox.includes(wanted.split(" ")[0])) break; }
  await page.keyboard.press("Escape").catch(() => {});
  await page.waitForTimeout(1500);
  after = await state();
  const inputPixels = await measure(rectOf(after, "window:wfc-bitmap-input"), "after-example-input");
  await note("example-switch", { picked, options, combobox: after.combobox, appliedPatches: historyLines(from).length, inputPixels, hosts: after.hosts }, from);
}

// ── 7. solve the newly loaded example ──────────────────────────────────────
{
  const from = lines.length;
  const opened = await openPane("wfcBitmapOutput");
  const run = await runAction("wfcBitmapOutput", "solve");
  let after = null;
  for (let i = 0; i < settle * 4; i++) { await page.waitForTimeout(500); after = await state(); }
  await closePane("wfcBitmapOutput");
  await page.waitForTimeout(2000);
  after = await state();
  await note("solve-second-example", { opened, run, outputPixels: await measure(rectOf(after, "window:wfc-bitmap-output"), "after-solve-2-output"), inputPixels: await measure(rectOf(after, "window:wfc-bitmap-input"), "after-solve-2-input"), hosts: after.hosts }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE steps", report.steps.length, "faults", report.steps.reduce((n, s) => n + s.faults.length, 0));
await browser.close();
