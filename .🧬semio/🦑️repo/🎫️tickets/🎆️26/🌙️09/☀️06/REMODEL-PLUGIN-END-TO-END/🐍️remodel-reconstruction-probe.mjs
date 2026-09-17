/** 🏗️ Remodel reconstruction probe: boots the remodel react playground (6063), loads the "Synthetic Orbit"
 * example (10 frames), arms the Reconstruction tool through the shell's Tool category and starts the run with
 * the framework chord (⌘️⏎ `toolRunStart`), then samples the Tool runs panel until the run reaches a terminal
 * state. Records every stage transition, the Results panel text, the Model window mesh count (a finalized run
 * must publish geometry) and every fault line, with screenshots per step.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=remodel-reconstruction-1 bun 🐍️remodel-reconstruction-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6063/?plugin=remodel";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "remodel-reconstruction");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 20);
const runSeconds = Number(process.env.SEMIO_PROBE_RUN ?? 240);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 4000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|refused|DuplicateSiblingKey|Faulted/.test(l)).map((l) => l.slice(0, 400));
const text = () => page.evaluate(() => document.body.innerText);

/** 🔬️ Everything one sample of the running app reports: hosts (canvas/mesh counts), the tool-run pill rows and the panel text. */
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const st = parse(el.getAttribute("data-status-json"));
    let meshes = 0;
    try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    return { id: el.getAttribute("data-surface-id"), fault: st?.fault ?? null, canvases: el.querySelectorAll("canvas").length, meshes };
  });
  const runRows = [...document.querySelectorAll("[id]")].filter((el) => /\bframework\.toolRun\b/.test(el.id)).map((el) => `${el.id}=${(el.innerText ?? "").replace(/\s+/g, " ").trim().slice(0, 240)}`);
  const treeRows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 80));
  const body = document.body.innerText;
  const stage = (body.match(/(?:Stage|stage)\s*(\d+)[^\n]{0,60}/) ?? [null])[0];
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    hosts,
    runRows,
    stage,
    streamSummary: treeRows.find((t) => /^Streams: \d+/.test(t)) ?? null,
  };
});

const note = async (step, detail, from) => {
  const faults = faultLines(from);
  report.steps.push({ step, ...detail, faults });
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step}.png`), fullPage: false }).catch(() => {});
  console.log(`[DEBUG] ${step}`, JSON.stringify({ ...detail, faults: faults.length }).slice(0, 900));
};

// ── boot ──────────────────────────────────────────────────────────────────
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });
for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); if ((await state()).ready) break; }
await page.waitForTimeout(3000);
await note("boot", await state(), 0);

// ── load the Synthetic Orbit example (10 frames to reconstruct) ───────────
{
  const from = lines.length;
  // 📚️ The navbar example picker is a `[role="combobox"]` popover, not a <select> (see 🐍️remodel-interact-probe.mjs).
  const example = process.env.SEMIO_PROBE_EXAMPLE ?? "Synthetic Orbit";
  const combo = page.locator('[role="combobox"]').first();
  let picked = "absent";
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
  for (let i = 0; i < settleSeconds * 3; i++) { await page.waitForTimeout(500); after = await state(); if (after.streamSummary && !/^Streams: 0/.test(after.streamSummary)) break; }
  await page.keyboard.press("Escape");
  await page.waitForTimeout(400);
  if (await page.locator('[role="option"]').count()) { await page.mouse.click(800, 700); await page.waitForTimeout(400); }
  await note("example-synthetic-orbit", { picked, options, ...(await state()) }, from);
}

// ── arm the Reconstruction tool, then start the run with the framework chord ──
{
  const from = lines.length;
  let armed = "absent";
  const category = page.locator('[id="framework.category.tool"]').first();
  if (await category.count()) { await category.click({ force: true }).catch(() => {}); await page.waitForTimeout(800); }
  // 📚️ Opening the category auto-arms its first tool; clicking the row again would be a re-press and DISARM it.
  const rows = await page.evaluate(() => [...document.querySelectorAll('[id^="tool."]')].map((el) => el.id));
  if (rows.includes("tool.reconstruction")) armed = "tool.reconstruction (auto-armed by the category)";
  else if (rows.length) {
    armed = await page.locator('[id="tool.reconstruction"]').first().click({ force: true, timeout: 4000 }).then(() => "clicked").catch((e) => String(e).slice(0, 120));
  }
  await note("tool-arm", { armed, rows: rows.slice(0, 20) }, from);
}

// ── run ───────────────────────────────────────────────────────────────────
{
  const from = lines.length;
  await page.evaluate(() => document.getElementById("framework.panelTab.framework.panel.toolRun")?.click());
  await page.waitForTimeout(800);
  await page.keyboard.press(process.platform === "darwin" ? "Meta+Enter" : "Control+Enter");
  const seen = [];
  let after = null;
  let terminal = null;
  for (let i = 0; i < runSeconds * 2; i++) {
    await page.waitForTimeout(500);
    after = await state();
    const key = JSON.stringify(after.runRows);
    if (seen[seen.length - 1] !== key) seen.push(key);
    const body = await text();
    if (/Faulted|Finalized|Completed|Failed/.test(body)) { terminal = (body.match(/Faulted|Finalized|Completed|Failed/) ?? [null])[0]; break; }
  }
  await note("reconstruction-run", { terminal, samples: seen.length, seen: seen.slice(-8), ...after, body: (await text()).slice(0, 2000) }, from);
}

// ── results ───────────────────────────────────────────────────────────────
{
  const from = lines.length;
  for (const id of ["framework.panelTab.remodeling.results", "framework.panelTab.remodeling.tracks"]) {
    await page.evaluate((i) => document.getElementById(i)?.click(), id);
    await page.waitForTimeout(1200);
  }
  await note("results", { ...(await state()), body: (await text()).slice(0, 2000) }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] RECONSTRUCTION DONE", report.steps.length);
await browser.close();
