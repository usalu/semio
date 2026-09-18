/** 🎯️ Lowpoly component selection/hover/gumball probe (6078): for each granularity (face, edge, vertex)
 * toggle it in Window Options (toggle must read pressed), hover the mesh centre (selection JSON must
 * echo `hoveredComponent`, the pane paints the highlight), click (componentIds), then read the composable
 * gumball config and flip its Rotate toggle off.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=lowpoly-selection-1 bun 🐍️lowpoly-selection-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6078/?plugin=lowpoly";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "lowpoly-selection");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [], verdicts: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|pageerror|unreachable|Fault \{|refused|dropped action/.test(l)).filter((l) => !/setActiveExample/.test(l)).map((l) => l.slice(0, 400));
const note = async (step, detail, from) => { report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) }); console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 700)}`); writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2)); writeFileSync(join(outDir, "console.txt"), lines.join("\n")); await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {}); };
const verdict = (name, ok, detail) => { report.verdicts.push({ name, ok, detail }); console.log(`[DEBUG] VERDICT ${ok ? "PASS" : "FAIL"} ${name} ${detail ?? ""}`); };
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const world = document.querySelector('[data-surface-id="window:lowpoly-main"]');
  const selection = parse(world?.getAttribute("data-selection-json") ?? "null");
  const rect = world?.getBoundingClientRect();
  const toggle = (id) => { const el = document.querySelector(`[id="${id}"]`); return el ? { present: true, pressed: el.getAttribute("aria-pressed") ?? el.getAttribute("data-state") ?? null } : { present: false }; };
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    world: rect ? { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height) } : null,
    selection: selection ? { mode: selection.selectionMode, ids: selection.selectedIds ?? selection.ids, componentIds: selection.componentIds, hoveredComponent: selection.hoveredComponent ?? null, hoveredId: selection.hoveredId ?? null, gumballActive: selection.gumballActive, gumballConfig: selection.gumballConfig ?? null } : null,
    toggles: Object.fromEntries(["face", "edge", "vertex", "mesh"].map((g) => [g, toggle(`lowpoly-main/lowpoly-select-${g}`)])),
    gumball: Object.fromEntries(["move", "rotate", "scale"].map((g) => [g, toggle(`lowpoly-main/lowpoly-gumball-${g}`)])),
  };
});
const settle = async (predicate, seconds = 15) => { let after = null; for (let i = 0; i < seconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };
const clickId = async (id) => { const el = page.locator(`[id="${id}"]`).first(); return (await el.count()) ? el.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const openOptions = async () => { if (await page.locator('[id="lowpoly-main/lowpoly-select-face"]').count()) return "open"; const b = page.locator('button:has-text("Window Options")').first(); if (await b.count()) await b.click({ force: true }); await page.waitForTimeout(800); return "clicked"; };

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if ((s.ready && s.world && i > 8)) break; }
await note("boot", s, 0);
verdict("boot", s.ready === "lowpoly", "");
await openOptions();
const centre = async () => { const w = (await state()).world; return [w.x + w.w / 2, w.y + w.h / 2]; };

for (const granularity of ["face", "edge", "vertex"]) {
  const from = lines.length;
  await openOptions();
  const toggled = await clickId(`lowpoly-main/lowpoly-select-${granularity}`);
  const armed = await settle((x) => x.selection?.mode === granularity && x.toggles[granularity].pressed === "true", 10);
  const [cx, cy] = await centre();
  // 🖱️ Sweep across the mesh so a vertex/edge hit lands even when the exact centre is inside a face.
  let hovered = null;
  for (const [dx, dy] of [[0, 0], [12, 0], [0, 12], [-12, 0], [0, -12], [24, 24], [-24, -24]]) {
    await page.mouse.move(cx + dx, cy + dy, { steps: 4 });
    hovered = await settle((x) => x.selection?.hoveredComponent?.mode === granularity, 3);
    if (hovered.selection?.hoveredComponent?.mode === granularity) { await page.screenshot({ path: join(outDir, `hover-${granularity}.png`), clip: { x: cx - 200, y: cy - 200, width: 400, height: 400 } }); break; }
  }
  await page.mouse.click(page.mouse._x ?? cx, page.mouse._y ?? cy).catch(() => {});
  const picked = await settle((x) => (x.selection?.componentIds ?? []).length > 0 && x.selection?.mode === granularity, 10);
  await note(`granularity-${granularity}`, { toggled, armedMode: armed.selection?.mode, pressed: armed.toggles[granularity], hovered: hovered?.selection?.hoveredComponent, picked: picked.selection }, from);
  verdict(`${granularity}: toggle reads pressed`, armed.toggles[granularity].pressed === "true", JSON.stringify(armed.toggles[granularity]));
  verdict(`${granularity}: hover echoes hoveredComponent`, hovered?.selection?.hoveredComponent?.mode === granularity, JSON.stringify(hovered?.selection?.hoveredComponent));
  verdict(`${granularity}: click selects a component`, (picked.selection?.componentIds ?? []).length > 0, JSON.stringify(picked.selection?.componentIds));
  // 🧹️ Deselect for the next granularity.
  await page.keyboard.press("Escape");
}
{
  const from = lines.length;
  await openOptions();
  await clickId("lowpoly-main/lowpoly-select-mesh");
  await settle((x) => x.selection?.mode === "mesh", 10);
  const [cx, cy] = await centre();
  await page.mouse.click(cx, cy);
  const picked = await settle((x) => (x.selection?.ids ?? []).length > 0 && x.selection?.gumballActive, 10);
  const before = picked.selection?.gumballConfig;
  await openOptions();
  const toggled = await clickId("lowpoly-main/lowpoly-gumball-rotate");
  const after = await settle((x) => x.selection?.gumballConfig?.rotate === false, 10);
  await page.screenshot({ path: join(outDir, `gumball.png`), clip: { x: cx - 300, y: cy - 300, width: 600, height: 600 } });
  await note("gumball", { picked: picked.selection, before, toggled, after: after.selection?.gumballConfig, toggle: after.gumball }, from);
  verdict("object pick arms the gumball with every group", picked.selection?.gumballActive === true && before?.moveAxes === true && before?.rotate === true && before?.scaleAxes === true, JSON.stringify(before));
  verdict("Rotate toggle turns the rotate handles off", after.selection?.gumballConfig?.rotate === false && after.gumball.rotate.pressed === "false", JSON.stringify(after.selection?.gumballConfig) + " " + JSON.stringify(after.gumball));
}
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] SELECTION DONE", report.verdicts.filter((v) => v.ok).length, "/", report.verdicts.length);
await browser.close();
