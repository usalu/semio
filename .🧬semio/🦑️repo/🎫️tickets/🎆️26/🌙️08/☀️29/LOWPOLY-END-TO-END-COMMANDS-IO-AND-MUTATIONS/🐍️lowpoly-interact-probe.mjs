/** 🎛️ Lowpoly interaction probe (react playground 6078): boot → face granularity toggle → click the cube's
 * face in the Model viewport (selection-json componentIds) → Actions pane `extrude` (meshes-json face
 * count grows) → mod+z (face count restored) → `extrude` again after undo (edit still applies) →
 * object granularity click (selection ids) → Actions pane `translateSelection`. Each step records the
 * console delta, fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=lowpoly-interact-1 bun 🐍️lowpoly-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6078/?plugin=lowpoly";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "lowpoly-interact");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not-ui-safe|missing-owned|refused|dropped action|invalid-args|unsupported/.test(l)).map((l) => l.slice(0, 500));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1200)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const world = document.querySelector('[data-surface-id="lowpoly.play.main"]');
  const meshes = parse(world?.getAttribute("data-meshes-json") ?? "[]") ?? [];
  const selection = parse(world?.getAttribute("data-selection-json") ?? "null");
  const rect = world?.getBoundingClientRect();
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    world: rect ? { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height), canvases: world.querySelectorAll("canvas").length, status: parse(world.getAttribute("data-status-json"))?.phase ?? null } : null,
    meshes: meshes.map((m) => ({ id: m.id, faces: (m.data?.faceIds ?? []).length, triangles: (m.data?.indices ?? []).length / 3 })),
    selection: selection ? { mode: selection.selectionMode, targets: selection.targets, ids: selection.ids, componentIds: selection.componentIds, gumballActive: selection.gumballActive, gumballTarget: selection.gumballTarget, active: selection.activeObjectId } : null,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    selectToggles: [...document.querySelectorAll('[id^="lowpoly-select-"]')].map((el) => el.id),
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 400),
  };
});
const faces = (s) => s.meshes.reduce((sum, m) => sum + m.faces, 0);
const settle = async (predicate, seconds = 20) => { let after = null; for (let i = 0; i < seconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };
const clickId = async (id) => { const el = page.locator(`[id="${id}"]`).first(); return (await el.count()) ? el.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const submitAction = async (actionId, args) => {
  const before = await state();
  let toggled = "already-open";
  if (!before.actionRows.length && before.engagements.length) toggled = await clickId(`${before.engagements[0]}.toggle`);
  await page.waitForTimeout(1200);
  const opened = await state();
  let clicked = "already-expanded";
  if (!(await page.locator(`[id$=".action.${actionId}.execute"]`).count())) clicked = opened.actionRows.includes(`action.${actionId}`) ? await clickId(`action.${actionId}`) : `absent in ${opened.actionRows.length} rows`;
  await page.waitForTimeout(1200);
  for (const [key, value] of Object.entries(args)) {
    const input = page.locator(`[id="${key}"], [name="${key}"], input[aria-label="${key}"]`).first();
    if (await input.count()) await input.fill(String(value)).catch(() => {});
  }
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  const submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control (row click executes)";
  return { toggled, clicked, submitted };
};
const clickWorldCenter = async (dx = 0, dy = 0) => {
  const s = await state();
  if (!s.world) return "no-world";
  await page.mouse.click(s.world.x + s.world.w / 2 + dx, s.world.y + s.world.h / 2 + dy);
  return "ok";
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if ((s.ready && s.world && s.meshes.length && i > 8) || s.error) break; }
await note("boot", s, 0);
const baseFaces = faces(s);

{
  const from = lines.length;
  const toggled = await clickId("lowpoly-select-face");
  const after = await settle((x) => x.selection?.mode === "face");
  await note("face-granularity", { toggled, selection: after.selection, toggles: after.selectToggles }, from);
}
{
  const from = lines.length;
  const clicked = await clickWorldCenter();
  const after = await settle((x) => (x.selection?.componentIds ?? []).length > 0);
  await note("pick-face", { clicked, selection: after.selection, interaction: lines.slice(from).filter((l) => /interactionSelect|worldPick/.test(l)).slice(0, 4) }, from);
}
{
  const from = lines.length;
  const a = await submitAction("extrude", {});
  const after = await settle((x) => faces(x) > baseFaces, 30);
  await note("extrude", { ...a, baseFaces, faces: faces(after), grew: faces(after) > baseFaces, meshes: after.meshes }, from);
}
{
  const from = lines.length;
  const before = faces(await state());
  await page.mouse.move(10, 10);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  const after = await settle((x) => faces(x) === baseFaces, 30);
  await note("undo-extrude", { before, faces: faces(after), restored: faces(after) === baseFaces }, from);
}
{
  const from = lines.length;
  const clicked = await clickWorldCenter();
  await settle((x) => (x.selection?.componentIds ?? []).length > 0, 10);
  const a = await submitAction("extrude", {});
  const after = await settle((x) => faces(x) > baseFaces, 30);
  await note("extrude-after-undo", { clicked, ...a, faces: faces(after), grew: faces(after) > baseFaces }, from);
}
{
  const from = lines.length;
  const toggled = await clickId("lowpoly-select-mesh");
  await settle((x) => x.selection?.mode === "mesh", 10);
  const clicked = await clickWorldCenter();
  const after = await settle((x) => (x.selection?.ids ?? []).length > 0);
  await note("pick-object", { toggled, clicked, selection: after.selection }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
