/** 🎛️ Fem interaction probe: boots a fem playground, then (1) re-picks the example through the navbar
 * combobox, (2) unfolds the model window's Actions pane and lists its rows, (3) stages + submits one
 * document action (default `addNode`) through the pane's own form, (4) stages `setResultDisplay` on the
 * results window. Every step records the console delta, the guest `[DEBUG]` lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6086/?plugin=fem2d SEMIO_PROBE_OUT=fem2d-interact-1 SEMIO_PROBE_WINDOW=fem2dModel bun 🐍️fem-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6086/?plugin=fem2d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "fem-interact");
const windowId = process.env.SEMIO_PROBE_WINDOW ?? "fem2dModel";
const resultsWindowId = process.env.SEMIO_PROBE_RESULTS_WINDOW ?? "fem2dResults";
const actionId = process.env.SEMIO_PROBE_ACTION ?? "addNode";
const actionArgs = JSON.parse(process.env.SEMIO_PROBE_ACTION_ARGS ?? '{"x":"9","y":"1.5"}');
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const guestLines = (from) => lines.slice(from).filter((l) => /\[DEBUG\] fem|trapped|panicked|action failed|shell fault|faults=/.test(l)).map((l) => l.slice(0, 400));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, guest: guestLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1200)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => ({
  ready: document.documentElement.getAttribute("data-semio-os-ready"),
  error: document.documentElement.getAttribute("data-semio-os-error"),
  hosts: [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length, text: (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 80) })),
  combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
}));
const pane = (id) => page.evaluate((windowId) => {
  const el = document.getElementById(`framework.window.${windowId}.engagement`);
  return { found: Boolean(el), folded: el?.getAttribute("data-folded") ?? null, rows: [...(el ?? document).querySelectorAll('[id^="action."]')].map((row) => row.id) };
}, id);
const unfold = async (id) => {
  const toggle = page.locator(`[id="framework.window.${id}.engagement.toggle"]`).first();
  if (await toggle.count()) await toggle.click({ timeout: 8000 }).catch((e) => lines.push(`toggle ${String(e).slice(0, 120)}`));
  await page.waitForTimeout(1500);
  return pane(id);
};
const settle = async (seconds) => { await page.waitForTimeout(seconds * 1000); return state(); };

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 120; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length >= 2 && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

// ── example switch ────────────────────────────────────────────────────────
{
  const from = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  let picked = "skipped";
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch((e) => (picked = String(e).slice(0, 100)));
    await page.waitForTimeout(500);
    const options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const wanted = process.env.SEMIO_PROBE_EXAMPLE ?? "Demo";
    const option = page.locator('[role="option"]').filter({ hasText: wanted }).first();
    if (await option.count()) picked = await option.click({ timeout: 5000 }).then(() => `ok:${wanted} of ${options.join("|")}`).catch((e) => String(e).slice(0, 100));
    else { picked = "no-options"; await page.keyboard.press("Escape"); }
  }
  await note("example-switch", { picked, ...(await settle(settleSeconds)) }, from);
}

// ── actions pane + one document action ────────────────────────────────────
{
  const from = lines.length;
  await page.locator(`[data-surface-id="window:${windowId.replace(/([A-Z])/g, "-$1").toLowerCase()}"]`).first().click({ position: { x: 320, y: 320 } }).catch(() => {});
  await page.waitForTimeout(800);
  const opened = await unfold(windowId);
  const rowId = `action.${actionId}`;
  let clicked = "absent";
  if (opened.rows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries(actionArgs)) {
    const input = page.locator(`[id="${key}"], [name="${key}"], input[aria-label="${key}"]`).first();
    if (await input.count()) { await input.fill(String(value)).catch((e) => filled.push(`${key}:${String(e).slice(0, 60)}`)); filled.push(`${key}=${value}`); } else filled.push(`${key}:absent`);
  }
  const controls = await page.evaluate(() => [...document.querySelectorAll('input, select, textarea, button[type="submit"], [id$=".submit"], [id$=".apply"]')].map((el) => ({ id: el.id, tag: el.tagName, name: el.getAttribute("name"), type: el.getAttribute("type"), text: (el.innerText ?? el.value ?? "").toString().trim().slice(0, 24) })).filter((c) => c.tag !== "INPUT" || c.type !== "file").slice(0, 16));
  let submitted = "none";
  const submit = page.locator(`[id="framework.window.${windowId}.action.${actionId}.execute"]`).first();
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else { await page.keyboard.press("Enter"); submitted = "enter"; }
  await note("action", { opened, rowId, clicked, filled, controls, submitted, ...(await settle(settleSeconds)) }, from);
}

// ── results window: setResultDisplay ──────────────────────────────────────
{
  const from = lines.length;
  await page.locator(`[data-surface-id="window:${resultsWindowId.replace(/([A-Z])/g, "-$1").toLowerCase()}"]`).first().click({ position: { x: 320, y: 320 } }).catch(() => {});
  await page.waitForTimeout(800);
  const opened = await unfold(resultsWindowId);
  const rowId = "action.setResultDisplay";
  let clicked = "absent";
  if (opened.rows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const fields = await page.evaluate(() => [...document.querySelectorAll('input, select, [data-slot="select-trigger"], [role="combobox"]')].map((el) => ({ id: el.id, tag: el.tagName, type: el.getAttribute("type"), value: el.value ?? el.innerText?.slice(0, 40) })).slice(0, 20));
  const mode = page.locator('[id="mode"], [name="mode"]').first();
  let modeSet = "absent";
  if (await mode.count()) modeSet = await mode.fill("modal").then(() => "filled").catch(async () => { try { await mode.click(); await page.waitForTimeout(300); const opt = page.locator('[role="option"]').filter({ hasText: /modal/i }).first(); if (await opt.count()) { await opt.click(); return "picked"; } return "no-option"; } catch (e) { return String(e).slice(0, 80); } });
  const submit = page.locator(`[id="framework.window.${resultsWindowId}.action.setResultDisplay.execute"]`).first();
  let submitted = "none";
  if (await submit.count()) submitted = await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  else { await page.keyboard.press("Enter"); submitted = "enter"; }
  await note("result-display", { opened, rowId, clicked, fields, modeSet, submitted, ...(await settle(settleSeconds)) }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
