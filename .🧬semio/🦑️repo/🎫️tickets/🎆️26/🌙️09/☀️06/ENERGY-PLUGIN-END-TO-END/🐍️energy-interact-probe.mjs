/** 🎛️ Energy interaction probe: boots the energy react playground, then (1) re-picks an example through
 * the navbar combobox and checks the structure tree follows, (2) unfolds a window's Actions pane and
 * submits one document action (default `create-zone`) through the pane's own form, checking the zones
 * table and the structure count follow, (3) starts the simulation tool run through its framework chord
 * and watches the simulation window's live region + the Tool runs panel. Every step records the console
 * delta, guest/host fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6106/?plugin=energy SEMIO_PROBE_OUT=energy-interact-1 bun 🐍️energy-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6106/?plugin=energy";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "energy-interact");
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "BESTEST 900";
const actionId = process.env.SEMIO_PROBE_ACTION ?? "create-zone";
const actionArgs = JSON.parse(process.env.SEMIO_PROBE_ACTION_ARGS ?? '{"name":"Probe zone","volumeM3":"42"}');
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 10);
const runSeconds = Number(process.env.SEMIO_PROBE_RUN_SECONDS ?? 90);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{/.test(l)).map((l) => l.slice(0, 400));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const text = () => page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
const state = () => page.evaluate(() => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const structure = body.match(/Structure (.{0,60}?) Name: ([^:]*?) Version:/);
  const zones = body.match(/zones: (\d+)/);
  const rows = [...document.querySelectorAll("table tbody tr, [role=\"row\"]")].length;
  const sim = body.match(/busy=(true|false) · ([^·]*?)(?: mod\+| Run:)/);
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    structureRoot: structure?.[1] ?? null,
    modelName: structure?.[2]?.trim() ?? null,
    zonesCount: zones ? Number(zones[1]) : null,
    tableRows: rows,
    simulation: sim ? { busy: sim[1], state: sim[2].trim() } : null,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
  };
});
const settle = async (seconds) => { await page.waitForTimeout(seconds * 1000); return state(); };

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 150; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.modelName && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

// ── example switch ────────────────────────────────────────────────────────
{
  const from = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  let picked = "skipped";
  let options = [];
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch((e) => (picked = String(e).slice(0, 100)));
    await page.waitForTimeout(600);
    options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator('[role="option"]').filter({ hasText: example }).first();
    if (await option.count()) picked = await option.click({ timeout: 5000 }).then(() => `ok:${example}`).catch((e) => String(e).slice(0, 100));
    else { picked = "no-option"; await page.keyboard.press("Escape"); }
  }
  let after = null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (after.modelName && after.modelName.includes(example)) break; }
  // 🧹️ The picker's listbox stays open after a pick and covers the middle window; close it.
  await page.keyboard.press("Escape");
  await page.waitForTimeout(400);
  if (await page.locator('[role="option"]').count()) { await page.mouse.click(800, 700); await page.waitForTimeout(400); }
  await note("example-switch", { picked, options, listboxOpen: await page.locator('[role="option"]').count(), ...after }, from);
}

// ── actions pane + one document action ────────────────────────────────────
{
  const from = lines.length;
  const before = await state();
  const engagement = before.engagements.find((id) => new RegExp(process.env.SEMIO_PROBE_WINDOW ?? "table", "i").test(id)) ?? before.engagements[0];
  let toggled = "absent";
  if (engagement) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    // 🫥️ `force`: the table window's header overlays its own Actions toggle, so an actionability check never passes.
    if (await toggle.count()) toggled = await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120));
    else toggled = `no-toggle:${engagement}`;
  }
  await page.waitForTimeout(1200);
  const opened = await state();
  const rowId = `action.${actionId}`;
  let clicked = "absent";
  if (opened.actionRows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries(actionArgs)) {
    const input = page.locator(`[id="${key}"], [name="${key}"], input[aria-label="${key}"]`).first();
    if (await input.count()) { await input.fill(String(value)).catch((e) => filled.push(`${key}:${String(e).slice(0, 60)}`)); filled.push(`${key}=${value}`); } else filled.push(`${key}:absent`);
  }
  const controls = await page.evaluate(() => [...document.querySelectorAll('input, select, textarea, button[type="submit"], [id$=".submit"], [id$=".apply"], [id$=".execute"]')].map((el) => ({ id: el.id, tag: el.tagName, name: el.getAttribute("name"), type: el.getAttribute("type"), text: (el.innerText ?? el.value ?? "").toString().trim().slice(0, 24) })).filter((c) => c.tag !== "INPUT" || c.type !== "file").slice(0, 20));
  let submitted = "none";
  const camel = actionId.replace(/-([a-z])/g, (_, c) => c.toUpperCase());
  const submit = page.locator(`[id$=".action.${actionId}.execute"], [id$=".action.${camel}.execute"]`).first();
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else { await page.keyboard.press("Enter"); submitted = "enter"; }
  let after = null;
  const expect = process.env.SEMIO_PROBE_EXPECT ?? null;
  for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (expect ? (await text()).includes(expect) : before.zonesCount !== null && after.zonesCount === before.zonesCount + 1) break; }
  await note("action", { engagement, toggled, rowId, clicked, filled, controls, submitted, zonesBefore: before.zonesCount, expect, expectSeen: expect ? (await text()).includes(expect) : null, ...after }, from);
}

// ── simulation tool run through the framework chord ───────────────────────
{
  const from = lines.length;
  // 🛠️ `toolRunStart` needs an ACTIVE tool: pick the simulation tool through the shell's Tool category first.
  let focused = "absent";
  const category = page.locator('[id="framework.category.tool"]').first();
  if (await category.count()) { await category.click({ force: true }).catch(() => {}); await page.waitForTimeout(600); }
  // Opening the category auto-arms its first (only) tool tab — a click on `tool.energySimulation` would be a re-press and DISARM it.
  focused = (await page.locator('[id="tool.energySimulation"]').count()) ? "tool:energySimulation (auto-armed by the category)" : "tool row absent";
  await page.waitForTimeout(800);
  await page.screenshot({ path: join(outDir, "tool-picked.png") }).catch(() => {});
  const activeTool = await page.evaluate(() => ({ toolButton: document.querySelector('[id="framework.category.tool"]')?.innerText?.trim().slice(0, 60) ?? null, dock: [...document.querySelectorAll('[id^="mode-dock-tab"]')].map((el) => ({ id: el.id, selected: el.getAttribute("aria-selected") ?? el.getAttribute("data-state") })) }));
  await note("tool-pick", { focused, activeTool }, from);
  // (no window click here: each click re-dispatches `setActiveTool`, and a toggle would clear the tool again)
  await page.keyboard.press(process.platform === "darwin" ? "Meta+Enter" : "Control+Enter");
  const seen = [];
  let after = null;
  for (let i = 0; i < runSeconds * 2; i++) {
    await page.waitForTimeout(500);
    after = await state();
    const key = after.simulation ? `${after.simulation.busy}:${after.simulation.state}` : "none";
    if (seen[seen.length - 1] !== key) seen.push(key);
    if (after.simulation && after.simulation.busy === "false" && seen.length > 1) break;
  }
  const toolRuns = await page.evaluate(() => [...document.querySelectorAll('[data-surface-id]')].map((el) => el.getAttribute("data-surface-id")));
  await note("simulation-run", { focused, seen, toolRuns, ...after, body: (await text()).slice(0, 1600) }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
