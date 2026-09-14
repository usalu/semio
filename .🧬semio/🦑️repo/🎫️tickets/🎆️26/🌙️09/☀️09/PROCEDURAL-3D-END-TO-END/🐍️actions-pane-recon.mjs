/** 🎛️ Why the flow window's Actions pane stopped answering after the 2026-09-14 restage.
 *
 * Three readings, each naming what the DOM actually holds rather than what a click returned:
 *   pane        — the engagement pane's fold state, its toggle's rect, and what `elementFromPoint`
 *                 finds at that rect's centre (i.e. WHO would receive the click).
 *   staged      — after unfolding, every `[id^="action."]` row, then a click on `action.exportDocument`
 *                 and whether the staged `#format` control mounts.
 *   panels      — with the three side panels open, what covers each panel tab button and the toggle.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=generate-chord/actions bun 🐍️actions-pane-recon.mjs
 * @see 🐍️react-gap-probe.mjs steps `export-after-generate` / `actions-pane-de`
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "generate-chord/actions");
mkdirSync(outDir, { recursive: true });
const SURFACE = "window:procedural-main";

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 500)}`));

const report = { url, steps: [] };
const note = async (step, detail) => {
  report.steps.push({ step, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1600)}`);
  writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step}.png`) }).catch(() => {});
};

/** 🔎️ Who would actually receive a click at an element's centre — the honest answer to "is it covered". */
const coverage = (id) => page.evaluate((elementId) => {
  const el = document.getElementById(elementId);
  if (!el) return { id: elementId, present: false };
  const r = el.getBoundingClientRect();
  const x = Math.round(r.x + r.width / 2);
  const y = Math.round(r.y + r.height / 2);
  const hit = document.elementFromPoint(x, y);
  return { id: elementId, present: true, rect: { x: Math.round(r.x), y: Math.round(r.y), w: Math.round(r.width), h: Math.round(r.height) }, covered: !(el === hit || el.contains(hit)), hit: hit ? `${hit.tagName}#${hit.id}` : null, hitAncestor: hit?.closest?.("[id]")?.id ?? null };
}, id);

const pane = () => page.evaluate(() => {
  const el = document.getElementById("framework.window.proceduralMain.engagement");
  return {
    paneFound: Boolean(el),
    folded: el?.getAttribute("data-folded") ?? null,
    rows: [...(el ?? document).querySelectorAll('[id^="action."]')].map((row) => row.id),
    activeWindow: document.querySelector('[data-slot="window"][data-active="true"]')?.id ?? null,
    engagementIds: [...document.querySelectorAll('[id*="engagement"]')].map((row) => row.id).slice(0, 12),
  };
});
const until = async (read, pred, seconds) => { let v = await read(); for (let i = 0; i < seconds && !pred(v); i++) { await page.waitForTimeout(1000); v = await read(); } return v; };

await page.goto(url, { waitUntil: "domcontentloaded" });
await until(() => page.evaluate(() => document.querySelectorAll("[data-meshes-json]").length), (n) => n > 0, 180);
await page.waitForTimeout(4000);
await note("boot", { ...(await pane()), toggle: await coverage("framework.window.proceduralMain.engagement.toggle") });

// ── the pane, with nothing in the way ───────────────────────────────────────
{
  await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } }).catch((e) => lines.push(`surface click ${String(e).slice(0, 120)}`));
  await page.waitForTimeout(1500);
  const before = await pane();
  const toggle = await coverage("framework.window.proceduralMain.engagement.toggle");
  let clicked = "skipped";
  if (toggle.present) clicked = await page.locator('[id="framework.window.proceduralMain.engagement.toggle"]').first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(2500);
  await note("pane", { before, toggle, clicked, after: await pane() });
}

// ── the staged export form ──────────────────────────────────────────────────
{
  const rowId = "action.exportDocument";
  const row = await coverage(rowId);
  let clicked = "skipped";
  if (row.present) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(2500);
  const staged = await page.evaluate(() => ({
    format: document.querySelectorAll("#format").length,
    selects: [...document.querySelectorAll('[data-slot="select-trigger"], select')].map((el) => el.id || el.getAttribute("aria-label") || el.tagName),
    expanded: [...document.querySelectorAll('[id^="framework.window.proceduralMain.action."]')].map((el) => el.id).slice(0, 20),
  }));
  await note("staged", { row, clicked, staged, pane: await pane() });
}

// ── the same pane, AFTER a generate round trip (the battery's own order) ────
{
  await page.keyboard.press("Meta+Alt+ArrowRight");
  await page.waitForTimeout(6000);
  const add = page.locator('[id="action.addGeneration"], :text-is("Add Generation")').first();
  if (await add.count()) await add.click({ timeout: 8000 }).catch((e) => lines.push(`add ${String(e).slice(0, 120)}`));
  await page.waitForTimeout(8000);
  await page.keyboard.press("Meta+Alt+ArrowLeft");
  await page.waitForTimeout(8000);
  const afterHop = await pane();
  await page.locator(`[data-surface-id="${SURFACE}"]`).first().click({ position: { x: 20, y: 20 } }).catch(() => {});
  await page.waitForTimeout(1500);
  const toggle = await coverage("framework.window.proceduralMain.engagement.toggle");
  const toggled = await page.locator('[id="framework.window.proceduralMain.engagement.toggle"]').first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(2500);
  const paneNow = await pane();
  const rowCount = await page.locator('[id="action.exportDocument"]').count();
  const clicked = rowCount ? await page.locator('[id="action.exportDocument"]').first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  await page.waitForTimeout(2500);
  const format = await page.locator("#format").count();
  await note("after-generate", { afterHop: { folded: afterHop.folded, rows: afterHop.rows.length, activeWindow: afterHop.activeWindow }, toggle, toggled, paneNow: { folded: paneNow.folded, rows: paneNow.rows.length, activeWindow: paneNow.activeWindow }, rowCount, clicked, format });
}

// ── the three side panels, and whether they still fold ──────────────────────
{
  const tabs = ["framework.panel.artifact", "framework.panel.catalogue", "framework.panel.inspection"];
  const opened = [];
  for (const tab of tabs) opened.push({ tab, ...(await coverage(tab)), clicked: await page.locator(`[id="${tab}"]`).first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 100)) });
  await page.waitForTimeout(2500);
  const folded = [];
  for (const tab of tabs) folded.push({ tab, ...(await coverage(tab)), clicked: await page.locator(`[id="${tab}"]`).first().click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 100)) });
  await page.waitForTimeout(2500);
  await note("panels", { opened, folded, toggleAfter: await coverage("framework.window.proceduralMain.engagement.toggle"), pane: await pane() });
}

writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] ACTIONS RECON DONE");
await browser.close();
