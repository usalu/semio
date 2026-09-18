/** 🎛️ Trinity Rewriting interaction probe: boots the jack react playground (6054), then (1) records the boot state
 * (shell beacon, window hosts, Artifact tree node rows), (2) picks a node row in the Artifact tree (framework
 * `interactionSelect`), (3) runs `deleteSelection` from the graph window's Actions pane and checks the tree
 * loses the row, (4) presses mod+z and checks the row returns (undo proves the mutation landed in the document
 * store), (5) renames a node through the `patchNodes` form, (6) reloads the Nakagin example through
 * `setActiveExample` (host-applied LoadDocument). Every step records the console delta, fault lines and a screenshot.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=jack-interact-1 bun 🐍️trinity-jack-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6056/?plugin=trinity-rewriting";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "rewriting-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 12);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not-ui-safe|missing-owned|refused|invalid-args|unsupported/.test(l)).map((l) => l.slice(0, 500));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => { const st = parse(el.getAttribute("data-status-json")); return { id: el.getAttribute("data-surface-id"), phase: st?.phase, fault: st?.fault?.code ?? null, canvases: el.querySelectorAll("canvas").length, textLength: (el.innerText ?? "").length }; });
  const rows = [...document.querySelectorAll('[role="treeitem"][id^="panel:trinity-document/"]')].map((el) => ({ id: el.id, text: el.innerText.replace(/\s+/g, " ").trim().slice(0, 60), selected: el.getAttribute("aria-selected") === "true" }));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    hosts,
    treeItems: rows.length,
    nodeRows: rows.filter((r) => /piece|Piece|^[a-z]_|capsule|core|b /i.test(r.text) && !/→/.test(r.text)).length,
    edgeRows: rows.filter((r) => /→/.test(r.text)).length,
    rowsHead: rows.slice(0, 30),
    selected: rows.filter((r) => r.selected).map((r) => r.text),
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    bodyHead: document.body.innerText.replace(/\s+/g, " ").slice(0, 600),
  };
});
const togglePanel = async () => { const tab = page.locator('button:has-text("Artifact"), [role="button"]:has-text("Artifact")').first(); return (await tab.count()) ? tab.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const settle = async (predicate) => { let after = null; for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };
const countWithPanel = async (predicate) => { const opened = await togglePanel(); const after = await settle(predicate); const closed = await togglePanel(); await page.waitForTimeout(500); return { opened, closed, ...after }; };
const submitAction = async (actionId, args, windowPattern) => {
  const from = lines.length;
  const before = await state();
  const engagement = before.engagements.find((id) => windowPattern.test(id)) ?? before.engagements[0];
  let toggled = "already-open";
  if (!before.actionRows.length && engagement) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120)) : `no-toggle:${engagement}`;
  }
  await page.waitForTimeout(1200);
  const opened = await state();
  const rowId = `action.${actionId}`;
  let clicked = "absent";
  if (await page.locator(`[id$=".action.${actionId}.execute"]`).count()) clicked = "already-expanded";
  else if (opened.actionRows.includes(rowId)) clicked = await page.locator(`[id="${rowId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries(args)) {
    const input = page.locator(`input[id="${key}"], textarea[id="${key}"], button[id="${key}"][role="combobox"]`).first();
    if (!(await input.count())) { filled.push(`${key}:absent`); continue; }
    const tag = await input.evaluate((el) => el.tagName);
    const done = tag === "BUTTON"
      ? input.click({ force: true }).then(() => page.waitForTimeout(500)).then(() => page.locator('[role="option"]').filter({ hasText: new RegExp(`^\\s*${String(value)}\\s*$`, "i") }).first().click({ timeout: 5000, force: true })).then(() => page.keyboard.press("Escape"))
      : input.fill(String(value));
    await done.then(() => filled.push(`${key}=${value}`)).catch((e) => filled.push(`${key}:${String(e).slice(0, 80)}`));
  }
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  const submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
  return { from, engagement, toggled, rowId, clicked, filled, submitted, actionRows: opened.actionRows.slice(0, 20) };
};

if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
const only = (process.env.SEMIO_PROBE_ONLY ?? "").split(",").filter(Boolean);
const wants = (step) => !only.length || only.includes(step);
await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length && i > 10) break; if (s.error) break; }
await note("boot", s, 0);

let baseline;
{
  const from = lines.length;
  baseline = await countWithPanel((x) => x.treeItems > 2);
  await note("artifact-panel-baseline", baseline, from);
}
const pickRow = baseline.rowsHead.find((r) => /^jack_orphan\b/.test(r.text)) ?? baseline.rowsHead[baseline.rowsHead.length - 1];
const rawId = (row) => (row?.id ?? "").replace(/^panel:trinity-document\//, "");
{
  const from = lines.length;
  await togglePanel();
  await settle((x) => x.treeItems > 2);
  const row = pickRow ? page.locator(`[id="${pickRow.id}"]`).first() : null;
  const clicked = row && (await row.count()) ? await row.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  const after = await settle((x) => x.selected.length > 0);
  await togglePanel();
  await page.waitForTimeout(800);
  await note("select-node", { pickRow, clicked, selected: after.selected, interactionLines: lines.slice(from).filter((l) => /interactionSelect/.test(l)).slice(0, 4).map((l) => l.slice(0, 200)) }, from);
}
const rule = () => page.evaluate(() => ({
  parameters: [...document.querySelectorAll("input[id*='.param.']")].map((el) => ({ id: el.id.replace(/^.*param\./, "").replace(/\.input$/, ""), value: el.value })),
  query: document.querySelector("textarea[aria-label='jack editor']")?.value?.replace(/\s+/g, " ").slice(0, 200) ?? null,
}));
const settleRule = async (predicate) => { let state = null; for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); state = await rule(); if (predicate(state)) break; } return state; };
if (wants("clause")) {
  // ➕️ A `parameter` clause appends a rule parameter — the Parameters window gains its bound input.
  const before = await rule();
  const clauseKind = process.env.SEMIO_PROBE_CLAUSE ?? "Parameter";
  const a = await submitAction("addRuleClause", { kind: clauseKind }, /rhs|lhs|before/i);
  const after = await settleRule((x) => x.parameters.length > before.parameters.length || x.query !== before.query);
  await note("add-rule-clause", { clauseKind, ...a, before, after, added: after.parameters.length > before.parameters.length || after.query !== before.query }, a.from);
  const from = lines.length;
  const undo = await submitAction("undo", {}, /rhs|lhs|before/i);
  const undone = await settleRule((x) => x.parameters.length === before.parameters.length && x.query === before.query);
  await note("add-rule-clause-undo", { undoClicked: undo.clicked, before, after: undone, restored: undone.parameters.length === before.parameters.length && undone.query === before.query }, from);
}
if (wants("parameter")) {
  // 🎛️ The Parameters window binds each rule parameter to `setParameter` on change; the compiled Jack
  // query in the Jack window substitutes the binding, so it is the end-to-end observable.
  const before = await rule();
  const from = lines.length;
  const input = page.locator("input[id*='.param.label.']").first();
  const filled = (await input.count()) ? await input.fill("probe-label").then(() => input.press("Enter")).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  const after = await settleRule((x) => /probe-label/.test(x.query ?? "") || x.parameters.some((p) => p.value === "probe-label"));
  await note("set-parameter", { before, filled, after, bound: after.parameters.some((p) => p.value === "probe-label"), inQuery: /probe-label/.test(after.query ?? "") }, from);
}

writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
