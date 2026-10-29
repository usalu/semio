/** 🎛️ Layout interaction probe: boots the layout react playground (6079), then (1) records the boot state
 * (shell beacon, window hosts, Artifact tree frame/page rows), (2) unfolds the Blueprint window's Actions
 * pane and submits `addFrame` through the pane's own form, checking the Artifact tree gains a frame row,
 * (3) presses mod+z and checks the frame row is removed again (undo proves the mutation landed in the
 * document store), (4) submits `addPage` and checks a page row appears, (5) clicks a frame row to prove the
 * framework-owned selection lane answers. Every step records the console delta, fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=layout-interact-1 bun 🐍️layout-interact-probe.mjs
 */
import { chromium } from "playwright";
import assert from "node:assert/strict";
import { validateMedia } from "./media-oracles.mjs";
import { readFileSync } from "node:fs";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6079/?plugin=layout";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "layout-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const deadline = setTimeout(() => { void browser.close(); }, 300000);
const cancel = () => { void browser.close(); };
process.once("SIGINT", cancel);
process.once("SIGTERM", cancel);
try {
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not-ui-safe|missing-owned|refused/.test(l)).map((l) => l.slice(0, 400));
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
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, textLength: (el.innerText ?? "").length }; });
  const items = [...document.querySelectorAll('[role="treeitem"]')];
  const rows = items.map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 60));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    treeItems: rows.length,
    frameRows: items.filter((el) => /^frame-/.test(el.id)).length,
    frameIds: items.filter((el) => /^frame-/.test(el.id)).map((el) => el.id).sort(),
    pageIds: items.filter((el) => el.id.startsWith("layout-document.page.")).map((el) => el.id).sort(),
    pageRows: items.filter((el) => el.id.startsWith("layout-document.page.")).length,
    rowsHead: rows.slice(0, 24),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    bodyHead: body.slice(0, 600),
  };
});
const submitAction = async (actionId, args) => {
  const from = lines.length;
  const before = await state();
  const engagement = before.engagements.find((id) => /blueprint/i.test(id)) ?? before.engagements[0];
  let toggled = "absent";
  const scope = engagement ? page.locator(`[id="${engagement}"]`) : page;
  const paneOpen = (await scope.locator('[id^="action."]').count()) > 0;
  if (engagement && !paneOpen) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120)) : `no-toggle:${engagement}`;
  } else if (paneOpen) toggled = "already-open";
  await page.waitForTimeout(1200);
  const opened = await state();
  const rowId = `action.${actionId}`;
  let clicked = "absent";
  const expanded = await scope.locator(`[id$=".action.${actionId}.execute"]`).count();
  if (expanded) clicked = "already-expanded";
  else if (opened.actionRows.includes(rowId)) clicked = await scope.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries(args)) {
    const input = page.locator(`[id="${key}"], [name="${key}"], input[aria-label="${key}"], select[aria-label="${key}"]`).first();
    if (await input.count()) { await input.fill(String(value)).catch((e) => filled.push(`${key}:${String(e).slice(0, 60)}`)); filled.push(`${key}=${value}`); } else filled.push(`${key}:absent`);
  }
  const submit = scope.locator(`[id$=".action.${actionId}.execute"]`).first();
  const submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
  return { from, before, engagement, toggled, rowId, clicked, filled, submitted };
};
const settle = async (predicate) => { let after = null; for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

// 🧭️ The Artifact panel overlays the Blueprint window's tab bar (Actions toggle included), so the
// panel is opened only to COUNT rows and closed again before any Actions-pane gesture.
const togglePanel = async () => { const tab = page.locator('[data-tab-id="framework.panel.artifact"]:visible').first(); return (await tab.count()) ? tab.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const countWithPanel = async (predicate) => { const opened = await togglePanel(); const after = await settle(predicate); const closed = await togglePanel(); await page.waitForTimeout(500); return { opened, closed, ...after }; };

let baseline = null;
let archiveState = null;
const archivePath = join(outDir, "edited.semio-archive");
{
  const from = lines.length;
  baseline = await countWithPanel((x) => x.frameRows > 0);
  await note("artifact-panel-baseline", baseline, from);
}
for (const [action, format] of [["exportSvg", "svg"], ["exportPng", "png"], ["exportPdf", "pdf"]]) {
  const pending = page.waitForEvent("download", { timeout: 90000 });
  const a = await submitAction(action, {});
  const download = await pending;
  assert.equal(await download.failure(), null, `${format} download`);
  assert.ok(download.suggestedFilename().endsWith(`.${format}`), "download extension");
  const path = join(outDir, `document.${format}`);
  await download.saveAs(path);
  const observed = await validateMedia(format, readFileSync(path), baseline.pageRows);
  await note(`export-${format}`, observed, a.from);
}
{
  const a = await submitAction("addFrame", {});
  await page.waitForTimeout(3000);
  const after = await countWithPanel((x) => x.frameRows === baseline.frameRows + 1);
  await note("add-frame", { ...a, before: undefined, framesBefore: baseline.frameRows, added: after.frameRows === baseline.frameRows + 1, ...after }, a.from);
  baseline = after;
}
{
  // ⏪️ Undo must follow the mutation directly — every panel toggle lands its own ledger entry
  // (`noteShellCommand` "Toggle Panel") on the shared history and would be what mod+z pops.
  const a = await submitAction("addFrame", {});
  await page.waitForTimeout(3000);
  const created = lines.slice(a.from).filter((l) => /CreateFrame/.test(l)).length;
  await page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  await page.waitForTimeout(3000);
  const undoLines = lines.slice(a.from).filter((l) => /undo handleAction resolved|labels":\["Undo/.test(l)).map((l) => l.slice(0, 260));
  const after = await countWithPanel((x) => x.frameRows === baseline.frameRows);
  await note("add-frame-then-undo", { submitted: a.submitted, created, undoLines, framesBefore: baseline.frameRows, undone: after.frameRows === baseline.frameRows, ...after }, a.from);
  baseline = after;
}
{
  const pending = page.waitForEvent("download", { timeout: 90000 });
  const a = await submitAction("exportArtifactDocument", {});
  const download = await pending;
  assert.equal(await download.failure(), null, "archive download");
  assert.ok(download.suggestedFilename().endsWith(".semio-archive"), "native document archive");
  await download.saveAs(archivePath);
  archiveState = { frameIds: baseline.frameIds, pageIds: baseline.pageIds };
  await note("export-archive", { exported: readFileSync(archivePath).length > 0, ...archiveState }, a.from);
}
{
  const a = await submitAction("addPage", {});
  await page.waitForTimeout(3000);
  const after = await countWithPanel((x) => x.pageRows === baseline.pageRows + 1);
  await note("add-page", { ...a, before: undefined, pagesBefore: baseline.pageRows, added: after.pageRows === baseline.pageRows + 1, ...after }, a.from);
  baseline = after;
}
{
  const from = lines.length;
  await togglePanel();
  await settle((x) => x.frameRows > 0);
  const row = page.locator('[role="treeitem"][id="frame-1"]').first();
  const clicked = (await row.count()) ? await row.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  await page.waitForTimeout(3000);
  const selected = await page.evaluate(() => [...document.querySelectorAll('[role="treeitem"][aria-selected="true"]')].map((el) => el.id));
  await note("select-frame", { clicked, selected, interactionLines: lines.slice(from).filter((l) => /interactionSelect|selection|Select/.test(l)).slice(0, 6).map((l) => l.slice(0, 200)) }, from);
}

await togglePanel();
{
  const pending = page.waitForEvent("filechooser", { timeout: 30000 });
  const a = await submitAction("importArtifactDocument", {});
  await (await pending).setFiles(archivePath);
  await page.waitForTimeout(3000);
  const restored = await countWithPanel((current) => JSON.stringify(current.pageIds) === JSON.stringify(archiveState.pageIds));
  assert.deepEqual(restored.frameIds, archiveState.frameIds, "archive restores created frame IDs");
  assert.deepEqual(restored.pageIds, archiveState.pageIds, "archive restores saved page IDs");
  assert.notDeepEqual(restored.pageIds, baseline.pageIds, "archive differs from modified original document");
  await note("reopen-archive", { restored: true, ...restored }, a.from);
}
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
const contract = JSON.parse(readFileSync(new URL("./browser-journey.json", import.meta.url), "utf8"));
for (const expected of contract.steps) {
  const step = report.steps.find((row) => row.step === expected.step);
  assert.ok(step, expected.step);
  for (const [field, value] of Object.entries(expected.values)) assert.equal(step.detail[field], value, `${expected.step}.${field}`);
  for (const [field, value] of Object.entries(expected.minimum ?? {})) assert.ok(step.detail[field] >= value, `${expected.step}.${field} minimum ${value}`);
  assert.deepEqual(step.faults.filter((line) => !line.includes("contributions push refused empty pack")), [], `${expected.step} faults`);
}
assert.ok(report.steps.find((row) => row.step === "select-frame").detail.selected.includes("frame-1"), "clicked frame selection");
console.log("[DEBUG] Layout browser journey passed");

} finally {
  clearTimeout(deadline);
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
  await browser.close();
}
