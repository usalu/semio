/** 🎛️ Raster interaction probe: boots the raster react playground (6060), then (1) records the boot state
 * (shell beacon, Paint2d window hosts, Artifact-panel layer rows), (2) clicks the Artifact panel's own
 * "Add pixel layer" tree row (`raster-play-layers.add.pixel` → `addLayer {kind:"pixel"}`) and checks the
 * layer tree gains a row, (3) presses mod+z and checks the row is gone again (undo proves the mutation
 * landed in the document store), (4) clicks the "Add group layer" row, (5) hovers the composite (framework `interactionHover`, no
 * undeclared-action refusals), (6) wheel-zooms the composite canvas (`setCamera`, config lane) and reads the camera back.
 * Every step records the console delta, guest/host fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=raster-interact-1 bun 🐍️raster-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6060/?plugin=raster";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "raster-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|refused|DuplicateSiblingKey|not a framework-reserved/.test(l)).map((l) => l.slice(0, 400));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); const r = el.getBoundingClientRect(); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, w: Math.round(r.width), h: Math.round(r.height), canvases: el.querySelectorAll("canvas").length, camera: el.getAttribute("data-camera-json")?.slice(0, 120) ?? null }; });
  const rows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => ({ id: el.id, text: el.innerText.replace(/\s+/g, " ").trim().slice(0, 50) }));
  const layerRows = rows.filter((r) => /raster-play-layers\./.test(r.id) && !/\.add\./.test(r.id));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    treeItems: rows.length,
    layerRows: layerRows.length,
    layerLabels: layerRows.map((r) => r.text).slice(0, 12),
    addRows: rows.filter((r) => /\.add\./.test(r.id)).map((r) => r.id),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    bodyHead: body.slice(0, 500),
  };
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 8) break; if (s.error) break; }
await note("boot", s, 0);
// ── open the Artifact panel (the panel tabs boot collapsed) ──────────────
{
  const from = lines.length;
  const tab = page.locator('button:has-text("Artifact"), [role="tab"]:has-text("Artifact")').first();
  let opened = "absent";
  if (await tab.count()) opened = await tab.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  let after = null;
  for (let i = 0; i < 20; i++) { await page.waitForTimeout(500); after = await state(); if (after.layerRows > 0) break; }
  await note("open-artifact-panel", { opened, ...after }, from);
  s = after;
}
let before = s;

// ── Artifact panel "Add pixel layer" tree row ─────────────────────────────
{
  const from = lines.length;
  const row = page.locator('[id$="raster-play-layers.add.pixel"]').first();
  let clicked = "absent";
  if (await row.count()) clicked = await row.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.layerRows === before.layerRows + 1) break; }
  await note("tree-add-pixel-layer", { clicked, layersBefore: before.layerRows, added: after.layerRows === before.layerRows + 1, ...after }, from);
  before = after;
}

// ── undo through the keybinding ──────────────────────────────────────────
{
  const from = lines.length;
  await page.mouse.click(600, 500).catch(() => {});
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.layerRows === before.layerRows - 1) break; }
  await note("undo", { layersBefore: before.layerRows, undone: after.layerRows === before.layerRows - 1, ...after }, from);
  before = after;
}

// ── Artifact panel "Add group layer" tree row ─────────────────────────────
{
  const from = lines.length;
  const row = page.locator('[id$="raster-play-layers.add.group"]').first();
  let clicked = "absent";
  if (await row.count()) clicked = await row.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.layerRows === before.layerRows + 1) break; }
  await note("tree-add-group-layer", { clicked, layersBefore: before.layerRows, added: after.layerRows === before.layerRows + 1, ...after }, from);
  before = after;
}

// ── pointer hover over the composite → interactionHover (no undeclared-action refusals) ──
{
  const from = lines.length;
  const host = page.locator('[data-surface-id="window:raster-composite"]').first();
  let hovered = "absent";
  if (await host.count()) {
    const box = await host.boundingBox();
    if (box) { for (let i = 0; i < 6; i++) { await page.mouse.move(box.x + box.width * (0.3 + i * 0.08), box.y + box.height * 0.5); await page.waitForTimeout(150); } hovered = "ok"; }
  }
  await page.waitForTimeout(2500);
  const refused = lines.slice(from).filter((l) => /undeclared-action|setHover|setSelection/.test(l)).map((l) => l.slice(0, 200));
  const hoverLines = lines.slice(from).filter((l) => /interactionHover/.test(l)).length;
  await note("hover-interaction-verbs", { hovered, refused: refused.slice(0, 4), interactionHoverLines: hoverLines }, from);
}

// ── composite canvas wheel zoom → setCamera (config lane) ─────────────────
{
  const from = lines.length;
  const host = page.locator('[data-surface-id]').first();
  let wheeled = "absent";
  if (await host.count()) {
    const box = await host.boundingBox();
    if (box) { await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2); wheeled = await page.mouse.wheel(0, -300).then(() => "ok").catch((e) => String(e).slice(0, 120)); }
  }
  await page.waitForTimeout(3000);
  const cameraLines = lines.slice(from).filter((l) => /setCamera/.test(l)).map((l) => l.slice(0, 200));
  await note("wheel-set-camera", { wheeled, cameraLines: cameraLines.slice(0, 5), ...(await state()) }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
