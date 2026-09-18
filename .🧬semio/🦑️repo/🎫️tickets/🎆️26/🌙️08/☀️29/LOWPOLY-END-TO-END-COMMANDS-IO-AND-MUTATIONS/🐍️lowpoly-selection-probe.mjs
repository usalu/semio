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
  // 🎛️ Engagement quick-action rail buttons (`ActionGroupItem`): pressed state rides `aria-pressed`/`data-state`.
  const toggle = (id) => { const el = document.querySelector(`[id="${id}"]`); return el ? { present: true, published: el.getAttribute("aria-pressed") ?? el.getAttribute("data-state") ?? el.getAttribute("data-pressed") ?? null, attrs: [...el.attributes].map((a) => a.name).join(",") } : { present: false }; };
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    world: rect ? { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height) } : null,
    selection: selection ? { mode: selection.selectionMode, ids: selection.selectedIds ?? selection.ids, componentIds: selection.componentIds, hoveredComponent: selection.hoveredComponent ?? null, hoveredId: selection.hoveredId ?? null, gumballActive: selection.gumballActive, gumballConfig: selection.gumballConfig ?? null } : null,
    toggles: Object.fromEntries(["face", "edge", "vertex", "mesh"].map((g) => [g, toggle(`lowpoly.opt.select-${g}`)])),
    gumball: Object.fromEntries(["move", "rotate", "scale"].map((g) => [g, toggle(`lowpoly.opt.gumball-${g}`)])),
  };
});
const settle = async (predicate, seconds = 15) => { let after = null; for (let i = 0; i < seconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };
// 🖱️ Rows live in scrollable panes: scroll the target into view first, or a forced click at an
// off-screen centre lands on whatever is painted there.
const clickId = async (id) => { const el = page.locator(`[id="${id}"]`).first(); if (!(await el.count())) return "absent"; await el.evaluate((node) => node.scrollIntoView({ block: "center", inline: "nearest" })).catch(() => {}); await page.waitForTimeout(150); return el.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)); };
// 🪟️ The Window Options popover closes on a world click; a toggle must be VISIBLE, not merely mounted.
const openOptions = async () => { const face = page.locator('[id="lowpoly-main/lowpoly-select-face"]').first(); if ((await face.count()) && (await face.isVisible())) return "open"; const b = page.locator('button:has-text("Window Options")').first(); if (await b.count()) await b.click({ force: true }); await page.waitForTimeout(800); return "clicked"; };

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
  const toggled = await clickId(`lowpoly.opt.select-${granularity}`);
  const armed = await settle((x) => x.selection?.mode === granularity && ["true", "on"].includes(x.toggles[granularity].published), 10);
  const [cx, cy] = await centre();
  // 🖱️ Sweep across the mesh so a vertex/edge hit lands even when the exact centre is inside a face.
  let hovered = null;
  let at = [cx, cy];
  for (const [dx, dy] of [[0, 0], [12, 0], [0, 12], [-12, 0], [0, -12], [24, 24], [-24, -24], [40, 0], [0, 40], [-40, 0], [0, -40], [60, 30], [-60, -30]]) {
    await page.mouse.move(cx + dx, cy + dy, { steps: 4 });
    hovered = await settle((x) => x.selection?.hoveredComponent?.mode === granularity, 3);
    if (hovered.selection?.hoveredComponent?.mode === granularity) { at = [cx + dx, cy + dy]; await page.screenshot({ path: join(outDir, `hover-${granularity}.png`), clip: { x: cx - 200, y: cy - 200, width: 400, height: 400 } }); break; }
  }
  await page.mouse.click(at[0], at[1]);
  const picked = await settle((x) => (x.selection?.componentIds ?? []).length > 0 && x.selection?.mode === granularity, 10);
  await note(`granularity-${granularity}`, { toggled, armedMode: armed.selection?.mode, pressed: armed.toggles[granularity], hovered: hovered?.selection?.hoveredComponent, picked: picked.selection }, from);
  verdict(`${granularity}: toggle reads pressed`, ["true", "on"].includes(armed.toggles[granularity].published), JSON.stringify(armed.toggles[granularity]));
  verdict(`${granularity}: hover echoes hoveredComponent`, hovered?.selection?.hoveredComponent?.mode === granularity, JSON.stringify(hovered?.selection?.hoveredComponent));
  verdict(`${granularity}: click selects a component`, (picked.selection?.componentIds ?? []).length > 0, JSON.stringify(picked.selection?.componentIds));
  // 🧹️ Deselect for the next granularity.
  await page.keyboard.press("Escape");
}
{
  const from = lines.length;
  await openOptions();
  await clickId("lowpoly.opt.select-mesh");
  await settle((x) => x.selection?.mode === "mesh", 10);
  const [cx, cy] = await centre();
  await page.mouse.click(cx, cy);
  const picked = await settle((x) => (x.selection?.ids ?? []).length > 0 && x.selection?.gumballActive, 10);
  const before = picked.selection?.gumballConfig;
  await openOptions();
  const toggled = await clickId("lowpoly.opt.gumball-rotate");
  // 🕰️ The scene and the engagement rail land in separate refresh sections; wait for both.
  const after = await settle((x) => x.selection?.gumballConfig?.rotate === false && ["false", "off"].includes(x.gumball.rotate.published), 15);
  await page.screenshot({ path: join(outDir, `gumball.png`), clip: { x: cx - 300, y: cy - 300, width: 600, height: 600 } });
  await note("gumball", { picked: picked.selection, before, toggled, after: after.selection?.gumballConfig, toggle: after.gumball }, from);
  verdict("object pick arms the gumball with every group", picked.selection?.gumballActive === true && before?.moveAxes === true && before?.rotate === true && before?.scaleAxes === true, JSON.stringify(before));
  verdict("Rotate toggle turns the rotate handles off", after.selection?.gumballConfig?.rotate === false && ["false", "off"].includes(after.gumball.rotate.published), JSON.stringify(after.selection?.gumballConfig) + " " + JSON.stringify(after.gumball));
}
// ⌨️ Delete on a picked face, ⌘D on the object, Delete on the copy.
{
  const from = lines.length;
  const faces = (x) => { const world = document; return 0; };
  const meshFaces = () => page.evaluate(() => { const world = document.querySelector('[data-surface-id="window:lowpoly-main"]'); const meshes = JSON.parse(world?.getAttribute("data-meshes-json") ?? "[]"); return meshes.map((m) => [m.id, (m.data?.faceIds ?? []).length]); });
  await openOptions();
  await clickId("lowpoly.opt.select-face");
  await settle((x) => x.selection?.mode === "face", 10);
  const [cx, cy] = await centre();
  await page.mouse.move(cx, cy, { steps: 3 });
  await page.mouse.click(cx, cy);
  await settle((x) => (x.selection?.componentIds ?? []).length > 0, 10);
  const before = await meshFaces();
  await page.keyboard.press("Delete");
  let after = before;
  for (let i = 0; i < 30; i++) { await page.waitForTimeout(500); after = await meshFaces(); if (JSON.stringify(after) !== JSON.stringify(before)) break; }
  // 🔺️ The host counts triangles; one n-gon face is one or more of them.
  verdict("Delete removes the picked face", after[0]?.[1] < before[0]?.[1], `${JSON.stringify(before)} → ${JSON.stringify(after)}`);
  await openOptions();
  await clickId("lowpoly.opt.select-mesh");
  await settle((x) => x.selection?.mode === "mesh", 10);
  await page.mouse.click(cx, cy);
  await settle((x) => (x.selection?.ids ?? []).length > 0, 10);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+d" : "Control+d");
  let dup = after;
  for (let i = 0; i < 30; i++) { await page.waitForTimeout(500); dup = await meshFaces(); if (dup.length === 2) break; }
  verdict("⌘D duplicates the object", dup.length === 2 && dup[1]?.[1] === after[0]?.[1], JSON.stringify(dup));
  await page.keyboard.press("Delete");
  let del = dup;
  for (let i = 0; i < 30; i++) { await page.waitForTimeout(500); del = await meshFaces(); if (del.length === 1) break; }
  verdict("Delete removes the copy ⌘D selected", del.length === 1 && del[0]?.[0] === "obj-1", JSON.stringify(del));
  await note("keyboard-delete-duplicate", { before, after, dup, del }, from);
}
// 🧲️ A real gumball drag: project the pivot and the +X arrow through the pane's camera, press on the
// arrow shaft and drag along its screen direction — the object's mesh must move (mesh JSON changes).
{
  const from = lines.length;
  const meshDigest = () => page.evaluate(() => { const world = document.querySelector('[data-surface-id="window:lowpoly-main"]'); const meshes = JSON.parse(world?.getAttribute("data-meshes-json") ?? "[]"); const m = meshes[0]; const pos = m?.data?.positions ?? []; let sum = 0; for (let i = 0; i < pos.length; i += 1) sum += pos[i] * (1 + (i % 7)); return { id: m?.id, sum: Number(sum.toFixed(3)), n: pos.length }; });
  await openOptions();
  await clickId("lowpoly.opt.select-mesh");
  await settle((x) => x.selection?.mode === "mesh", 10);
  const [cx, cy] = await centre();
  await page.mouse.click(cx, cy);
  const armed = await settle((x) => x.selection?.gumballActive === true, 10);
  const geometry = await page.evaluate(() => {
    const world = document.querySelector('[data-surface-id="window:lowpoly-main"]');
    const camera = JSON.parse(world.getAttribute("data-viewport-camera-json") ?? world.getAttribute("data-camera-json") ?? "{}");
    const selection = JSON.parse(world.getAttribute("data-selection-json") ?? "{}");
    const rect = world.getBoundingClientRect();
    return { camera, target: selection.gumballTarget, rect: { x: rect.x, y: rect.y, w: rect.width, h: rect.height } };
  });
  const project = (p) => {
    const { position, target, up = [0, 0, 1], fov = 45 } = geometry.camera;
    const sub = (a, b) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    const cross = (a, b) => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    const norm = (a) => { const l = Math.hypot(...a) || 1; return [a[0] / l, a[1] / l, a[2] / l]; };
    const f = norm(sub(target, position)); const r = norm(cross(f, up)); const u = cross(r, f);
    const d = sub(p, position); const x = dot(d, r), y = dot(d, u), z = dot(d, f);
    const tanHalf = Math.tan((fov * Math.PI) / 360); const aspect = geometry.rect.w / geometry.rect.h;
    return [geometry.rect.x + ((x / (z * tanHalf * aspect) + 1) / 2) * geometry.rect.w, geometry.rect.y + ((1 - y / (z * tanHalf)) / 2) * geometry.rect.h];
  };
  let dragged = null;
  if (geometry.target && geometry.camera.position) {
    const dist = Math.hypot(...geometry.target.map((v, i) => v - geometry.camera.position[i]));
    const arrow = (0.98 * dist) / 8;
    const pivot = project(geometry.target);
    const tip = project([geometry.target[0] + arrow, geometry.target[1], geometry.target[2]]);
    const dir = [tip[0] - pivot[0], tip[1] - pivot[1]]; const len = Math.hypot(...dir) || 1;
    const before = await meshDigest();
    let after = before;
    let grab = null;
    // 🎯️ The drawn arrow is shorter than the pinhole estimate (the pane's own camera pose differs from the
    // published one); walk the shaft from the pivot outwards until a press moves the mesh.
    for (const t of [0.3, 0.2, 0.4, 0.15, 0.5]) {
      grab = [pivot[0] + dir[0] * t, pivot[1] + dir[1] * t];
      await page.mouse.move(grab[0], grab[1], { steps: 4 }); await page.waitForTimeout(300);
      await page.mouse.down();
      for (let i = 1; i <= 12; i++) { await page.mouse.move(grab[0] + (dir[0] / len) * 10 * i, grab[1] + (dir[1] / len) * 10 * i, { steps: 2 }); await page.waitForTimeout(40); }
      await page.mouse.up();
      for (let i = 0; i < 16; i++) { await page.waitForTimeout(500); after = await meshDigest(); if (after.sum !== before.sum) break; }
      if (after.sum !== before.sum) break;
    }
    dragged = { pivot: pivot.map(Math.round), tip: tip.map(Math.round), grab: grab.map(Math.round), before, after };
    await page.screenshot({ path: join(outDir, "gumball-drag.png"), clip: { x: cx - 300, y: cy - 300, width: 600, height: 600 } });
  }
  await note("gumball-drag", { armed: armed.selection, geometry, dragged }, from);
  verdict("gumball X-arrow drag moves the object", !!dragged && dragged.after.sum !== dragged.before.sum, JSON.stringify(dragged));
}
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] SELECTION DONE", report.verdicts.filter((v) => v.ok).length, "/", report.verdicts.length);
await browser.close();
