/** 🎛️ Layout interaction probe: boots the layout react playground (6079), then (1) records the boot state
 * (shell beacon, window hosts, Artifact tree frame/page rows), (2) unfolds the Blueprint window's Actions
 * pane and submits `addFrame` through the pane's own form, checking the Artifact tree gains a frame row,
 * (3) presses mod+z and checks the frame row is removed again (undo proves the mutation landed in the
 * document store), (4) submits `addPage` and checks a page row appears, (5) clicks a frame row to prove the
 * framework-owned selection lane answers. Every step records the console delta, fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=layout-interact-1 bun 🐍️layout-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6079/?plugin=layout";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "layout-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
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
  const rows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 60));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    treeItems: rows.length,
    frameRows: rows.filter((r) => /^frame-/.test(r)).length,
    pageRows: rows.filter((r) => /^Page \d+|^Seite \d+/.test(r)).length,
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
  const paneOpen = before.actionRows.length > 0;
  if (engagement && !paneOpen) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120)) : `no-toggle:${engagement}`;
  } else if (paneOpen) toggled = "already-open";
  await page.waitForTimeout(1200);
  const opened = await state();
  const rowId = `action.${actionId}`;
  let clicked = "absent";
  const expanded = await page.locator(`[id$=".action.${actionId}.execute"]`).count();
  if (expanded) clicked = "already-expanded";
  else if (opened.actionRows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries(args)) {
    const input = page.locator(`[id="${key}"], [name="${key}"], input[aria-label="${key}"], select[aria-label="${key}"]`).first();
    if (await input.count()) { await input.fill(String(value)).catch((e) => filled.push(`${key}:${String(e).slice(0, 60)}`)); filled.push(`${key}=${value}`); } else filled.push(`${key}:absent`);
  }
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  const submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
  return { from, before, engagement, toggled, rowId, clicked, filled, submitted };
};
const settle = async (predicate) => { let after = null; for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && s.treeItems > 0 && i > 8) break; if (s.error) break; }
await note("boot", s, 0);

// 🧭️ The Artifact panel overlays the Blueprint window's tab bar (Actions toggle included), so the
// panel is opened only to COUNT rows and closed again before any Actions-pane gesture.
const togglePanel = async () => { const tab = page.locator('button:has-text("Artifact"), [role="button"]:has-text("Artifact")').first(); return (await tab.count()) ? tab.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const countWithPanel = async (predicate) => { const opened = await togglePanel(); const after = await settle(predicate); const closed = await togglePanel(); await page.waitForTimeout(500); return { opened, closed, ...after }; };

let baseline = null;
{
  const from = lines.length;
  baseline = await countWithPanel((x) => x.frameRows > 0);
  await note("artifact-panel-baseline", baseline, from);
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
  const row = page.locator('[role="treeitem"]', { hasText: "frame-1 Page 1" }).first();
  const clicked = (await row.count()) ? await row.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  await page.waitForTimeout(3000);
  const selected = await page.evaluate(() => [...document.querySelectorAll('[role="treeitem"][aria-selected="true"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 40)));
  await note("select-frame", { clicked, selected, interactionLines: lines.slice(from).filter((l) => /interactionSelect|selection|Select/.test(l)).slice(0, 6).map((l) => l.slice(0, 200)) }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
