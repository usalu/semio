/** 🎛️ Draw interaction probe (ticket 26/09/05/DRAW-PLUGIN-END-TO-END): boots the draw react
 * playground (6064), records the boot state (shell beacon, window hosts, the canvas window's
 * "N layers · M selected" status), then (1) opens the Canvas window's Actions pane and submits
 * `addLayer` through the pane's own form, checking the layer count grows, (2) presses mod+z and
 * checks the count shrinks again (undo proves the mutation landed in the document store), (3) drags
 * a rectangle on the canvas with the `shapeRect` utility active and checks one more layer commits,
 * (4) switches the example through the navbar fixture picker. Every step records the console delta,
 * guest/host fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=draw-interact-1 bun 🐍️draw-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6064/?plugin=draw";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "draw-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
// 🧵️ Guest `eprintln!` lands on the plugin WORKER's console, never on the page's.
page.on("worker", (worker) => { lines.push(`${Date.now() - t0} worker ${worker.url().slice(-80)}`); worker.on("console", (msg) => lines.push(`${Date.now() - t0} worker:${msg.type()} ${msg.text().slice(0, 1200)}`)); });
if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|refused/.test(l)).map((l) => l.slice(0, 400));
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
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length }; });
  const status = body.match(/(\d+) layers? · (\d+) selected/);
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    layers: status ? Number(status[1]) : null,
    selected: status ? Number(status[2]) : null,
    fixture: document.querySelector("#playground\\.navbar\\.fixture")?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    bodyHead: body.slice(0, 400),
  };
});
const canvasRect = () => page.evaluate(() => { const el = document.querySelector('[data-surface-id="window:drawing-composite"] canvas'); if (!el) return null; const r = el.getBoundingClientRect(); return { x: r.x, y: r.y, w: r.width, h: r.height }; });

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && s.layers !== null && i > 8) break; if (s.error) break; }
await note("boot", s, 0);
let before = s;

// ── Actions pane: addLayer ───────────────────────────────────────────────
{
  const from = lines.length;
  const toggle = page.locator('[id="framework.window.drawingComposite.engagement.toggle"]').first();
  let toggled = "absent";
  if (await toggle.count()) toggled = await toggle.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1200);
  const opened = await state();
  let clicked = "absent";
  if (opened.actionRows.includes("action.addLayer")) clicked = await page.locator('[id="action.addLayer"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  const kind = page.locator('[id="kind"], [name="kind"], input[aria-label="kind"]').first();
  if (await kind.count()) { await kind.fill("path").catch((e) => filled.push(`kind:${String(e).slice(0, 60)}`)); filled.push("kind=path"); } else filled.push("kind:absent");
  let submitted = "none";
  const submit = page.locator('[id$=".action.addLayer.execute"]').first();
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else submitted = "no-execute-control";
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.layers === before.layers + 1) break; }
  await note("action-add-layer", { toggled, clicked, filled, submitted, layersBefore: before.layers, ...after }, from);
  before = after;
}

// ── undo through the keybinding ──────────────────────────────────────────
{
  const from = lines.length;
  const r = await canvasRect();
  if (r) await page.mouse.click(r.x + r.w * 0.5, r.y + r.h * 0.95).catch(() => {});
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.layers === before.layers - 1) break; }
  await note("undo", { layersBefore: before.layers, undone: after.layers === before.layers - 1, ...after }, from);
  before = after;
}

// ── canvas gesture: rectangle with the shapeRect utility ────────────────
{
  const from = lines.length;
  const unfold = page.locator('[id="framework.window.drawingComposite.utilityBar.unfold"]').first();
  let unfolded = "absent";
  if (await unfold.count()) unfolded = await unfold.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1000);
  // 🧰️ The utility bar shows its GROUPS first (`ui.utilities.<window>.group.group:Drawing`); the
  // utilities themselves (`#shapeRect`) render once their group is toggled on.
  const drawingGroup = page.locator('[id="ui.utilities.drawing-composite.group.group:Drawing"]').first();
  let grouped = "absent";
  if (await drawingGroup.count()) grouped = await drawingGroup.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1200);
  const utilityIds = await page.evaluate(() => [...document.querySelectorAll('[id*="utilit"], [data-toggle-value]')].map((el) => el.id || el.getAttribute("data-toggle-value")).filter(Boolean).slice(0, 60));
  const rectUtility = page.locator('[id="shapeRect"], [data-toggle-value="shapeRect"]').first();
  let armed = "absent";
  if (await rectUtility.count()) armed = await rectUtility.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const r = await canvasRect();
  let dragged = "no-canvas";
  if (r) {
    const x0 = r.x + r.w * 0.35, y0 = r.y + r.h * 0.35, x1 = r.x + r.w * 0.6, y1 = r.y + r.h * 0.6;
    await page.mouse.move(x0, y0); await page.mouse.down();
    for (let i = 1; i <= 8; i++) { await page.mouse.move(x0 + ((x1 - x0) * i) / 8, y0 + ((y1 - y0) * i) / 8); await page.waitForTimeout(40); }
    await page.mouse.up();
    dragged = "ok";
  }
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.layers === before.layers + 1) break; }
  await note("canvas-rect-drag", { unfolded, grouped, utilityIds, armed, dragged, layersBefore: before.layers, committed: after.layers === before.layers + 1, ...after }, from);
  before = after;
}

// ── example switch through the navbar fixture picker ────────────────────
{
  const from = lines.length;
  const picker = page.locator('[id="playground.navbar.fixture"]').first();
  let opened = "absent";
  if (await picker.count()) opened = await picker.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1000);
  const options = await page.evaluate(() => [...document.querySelectorAll('[role="option"], [role="menuitem"], [role="menuitemradio"]')].map((el) => `${el.id || "-"}|${el.innerText.replace(/\s+/g, " ").trim().slice(0, 40)}`).slice(0, 20));
  let picked = "none";
  const other = page.locator('[role="option"], [role="menuitem"], [role="menuitemradio"]').filter({ hasNotText: before.fixture ?? "Demo" }).first();
  if (await other.count()) picked = await other.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else await page.keyboard.press("Escape");
  await page.waitForTimeout(4000);
  const after = await state();
  await note("example-switch", { opened, options, picked, fixtureBefore: before.fixture, layersBefore: before.layers, changed: after.layers !== before.layers || after.fixture !== before.fixture, ...after }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
