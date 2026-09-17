/** 🎛️ Note interaction probe: boots the note react playground (6080) and proves document mutations through the shell.
 * Block counts are the distinct `data-ink-block-id`s inside the Canvas window host, so counting never toggles a panel
 * (panel toggles land their own shell ledger entries, which mod+z would pop instead of the document edit).
 * Steps: (1) boot, (2) Canvas Actions pane `addBlock` (kind=text) → +1, (3) `addBlock` + immediate mod+z → back,
 * (4) Artifact panel quick-add row "Add Math" → +1 and a new tree row, (5) click a block tree row → aria-selected,
 * (6) Delete key (`deleteSelection`) → −1, (7) mod+z → restored, (8) page reload → block count survives.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=note-interact-1 bun 🐍️note-interact-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6080/?plugin=note";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "note-interact");
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 15);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const faultLines = (from) => lines.slice(from).filter((l) => /trapped|panicked|action failed|shell fault|faults=|pageerror|unreachable|Fault \{|not-ui-safe|missing-owned|refused|invalid-args|unsupported/.test(l)).map((l) => l.slice(0, 500));
const mod = process.platform === "darwin" ? "Meta" : "Control";
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, faults: faultLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1200)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => {
  const body = document.body.innerText.replace(/\s+/g, " ");
  const canvas = document.querySelector('[data-surface-id="window:note-composite"]');
  const rows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => ({ text: el.innerText.replace(/\s+/g, " ").trim().slice(0, 60), selected: el.getAttribute("aria-selected") }));
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => ({ id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length, svg: el.querySelectorAll("svg *").length, text: (el.innerText ?? "").slice(0, 80) }));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    error: document.documentElement.getAttribute("data-semio-os-error"),
    blocks: canvas ? new Set([...canvas.querySelectorAll('[data-ink-block-id]')].map((el) => el.getAttribute('data-ink-block-id'))).size : null,
    noScene: (body.match(/No scene/g) ?? []).length,
    docBlocks: (() => { const row = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim()).find((t) => /^Blocks \d+$/.test(t)); return row ? Number(row.split(" ")[1]) : null; })(),
    utility: [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim()).find((t) => /^Utility /.test(t)) ?? null,
    rows,
    hosts,
    engagements: [...document.querySelectorAll('[id$=".engagement"]')].map((el) => el.id),
    actionRows: [...document.querySelectorAll('[id^="action."]')].map((el) => el.id),
    example: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null,
  };
});
const settle = async (predicate) => { let after = null; for (let i = 0; i < settleSeconds * 2; i++) { await page.waitForTimeout(500); after = await state(); if (predicate(after)) break; } return after; };
const togglePanel = async (label) => { const tab = page.locator(`button:has-text("${label}")`).first(); return (await tab.count()) ? tab.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
const submitAction = async (actionId, args) => {
  const from = lines.length;
  const before = await state();
  const engagement = before.engagements.find((id) => /composite|canvas/i.test(id)) ?? before.engagements[0];
  let toggled = "already-open";
  if (!before.actionRows.length) {
    const toggle = page.locator(`[id="${engagement}.toggle"]`).first();
    toggled = (await toggle.count()) ? await toggle.click({ timeout: 8000, force: true }).then(() => `ok:${engagement}`).catch((e) => String(e).slice(0, 120)) : `no-toggle:${engagement}`;
    await page.waitForTimeout(1200);
  }
  const opened = await state();
  let clicked = "absent";
  if (await page.locator(`[id$=".action.${actionId}.execute"]`).count()) clicked = "already-expanded";
  else if (opened.actionRows.includes(`action.${actionId}`)) clicked = await page.locator(`[id="action.${actionId}"]`).first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1500);
  const filled = [];
  for (const [key, value] of Object.entries(args)) {
    const select = page.locator(`select[id$="${key}"], select[name="${key}"]`).first();
    if (await select.count()) { filled.push(await select.selectOption(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).slice(0, 60)}`)); continue; }
    const input = page.locator(`[id$="${key}"]:is(input,textarea), [name="${key}"]`).first();
    filled.push((await input.count()) ? await input.fill(String(value)).then(() => `${key}=${value}`).catch((e) => `${key}:${String(e).slice(0, 60)}`) : `${key}:absent`);
  }
  const submit = page.locator(`[id$=".action.${actionId}.execute"]`).first();
  const submitted = (await submit.count()) ? await submit.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "no-execute-control";
  return { from, before: before.blocks, engagement, toggled, clicked, filled, submitted, actionRows: opened.actionRows.slice(0, 40) };
};

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.blocks !== null && i > 10) break; if (s.error) break; }
await page.waitForTimeout(4000);
s = await state();
await note("boot", s, 0);
let count = s.blocks ?? 0;

{
  const a = await submitAction("addBlock", { kind: "text" });
  let after = await settle((x) => x.blocks === count + 1);
  if (after.blocks !== count + 1) after = await settle((x) => x.blocks === count + 1);
  await note("add-block-text", { ...a, after: after.blocks, added: after.blocks === count + 1 }, a.from);
  count = after.blocks ?? count;
}
{
  const a = await submitAction("addBlock", { kind: "table" });
  const created = await settle((x) => x.blocks === count + 1);
  await page.keyboard.press(`${mod}+z`);
  const after = await settle((x) => x.blocks === count);
  await note("add-block-then-undo", { submitted: a.submitted, created: created.blocks, after: after.blocks, undone: created.blocks === count + 1 && after.blocks === count }, a.from);
  count = after.blocks ?? count;
}
{
  const from = lines.length;
  const closedActions = await page.locator('[id="framework.window.noteComposite.engagement.toggle"]').first().click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120));
  await page.waitForTimeout(1000);
  const opened = await togglePanel("Artifact");
  await page.waitForTimeout(2000);
  const rowsBefore = (await state()).rows.length;
  const add = page.locator('[role="treeitem"]', { hasText: "Add Math" }).first();
  const clicked = (await add.count()) ? await add.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  const after = await settle((x) => x.blocks === count + 1 && x.rows.length > rowsBefore);
  await note("panel-add-math", { closedActions, opened, clicked, before: count, after: after.blocks, rowsBefore, rowsAfter: after.rows.length, added: after.blocks === count + 1, rows: after.rows.map((r) => r.text) }, from);
  count = after.blocks ?? count;
}
const blockRow = (rows) => rows.find((r) => /^(Text|Table|Math|Image|Group|Ink) /.test(r.text));
{
  const from = lines.length;
  const target = blockRow((await state()).rows);
  const row = target ? page.locator('[role="treeitem"]', { hasText: target.text }).first() : null;
  const clicked = row && (await row.count()) ? await row.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent";
  const after = await settle((x) => x.rows.some((r) => r.text === target?.text && r.selected === "true"));
  await note("select-block-row", { target: target?.text ?? null, clicked, selected: after.rows.filter((r) => r.selected === "true").map((r) => r.text), selectLines: lines.slice(from).filter((l) => /interactionSelect/.test(l)).slice(0, 3).map((l) => l.slice(0, 200)) }, from);
  const del = lines.length;
  await page.keyboard.press("Delete");
  const deleted = await settle((x) => x.blocks === count - 1);
  await note("delete-selection", { before: count, after: deleted.blocks, deleted: deleted.blocks === count - 1, rows: deleted.rows.map((r) => r.text) }, del);
  if (deleted.blocks === count - 1) {
    const und = lines.length;
    await page.keyboard.press(`${mod}+z`);
    const restored = await settle((x) => x.blocks === count);
    await note("undo-delete", { after: restored.blocks, restored: restored.blocks === count }, und);
    count = restored.blocks ?? count;
  } else count = deleted.blocks ?? count;
}
{
  const from = lines.length;
  await togglePanel("Artifact");
  await page.waitForTimeout(800);
  const host = page.locator('[data-surface-id="window:note-composite"]').first();
  const clickId = async (id) => { const el = page.locator(`[id="${id}"]`).first(); return (await el.count()) ? el.click({ timeout: 8000, force: true }).then(() => "ok").catch((e) => String(e).slice(0, 120)) : "absent"; };
  const utilityToggle = await clickId("framework.window.noteComposite.utilityBar.unfold");
  await page.waitForTimeout(1200);
  const drawGroup = await clickId("ui.utilities.note-composite.group.group:Draw");
  await page.waitForTimeout(1200);
  const armed = `${drawGroup}/${await clickId("pencil")}`;
  await page.waitForTimeout(1500);
  const beforeDoc = (await state()).docBlocks;
  const box = await host.boundingBox();
  if (box) {
    const x0 = box.x + box.width * 0.4, y0 = box.y + box.height * 0.6;
    await page.mouse.move(x0, y0);
    await page.mouse.down();
    for (let i = 1; i <= 12; i++) { await page.mouse.move(x0 + i * 12, y0 + Math.sin(i / 2) * 30); await page.waitForTimeout(30); }
    await page.mouse.up();
  }
  const after = await settle((x) => beforeDoc !== null && x.docBlocks === beforeDoc + 1);
  await note("canvas-pencil-stroke", { utilityToggle, armed, utility: after.utility, before: beforeDoc, after: after.docBlocks, drawn: beforeDoc !== null && after.docBlocks === beforeDoc + 1, inkLines: lines.slice(from).filter((l) => /inkApplyEvents|ink-events/.test(l)).slice(0, 4).map((l) => l.slice(0, 240)) }, from);
  const und = lines.length;
  await page.keyboard.press(`${mod}+z`);
  const undone = await settle((x) => x.docBlocks === beforeDoc);
  await note("undo-pencil-stroke", { after: undone.docBlocks, undone: undone.docBlocks === beforeDoc }, und);
  count = undone.blocks ?? count;
}
{
  const from = lines.length;
  await page.reload({ waitUntil: "domcontentloaded" });
  let after = null;
  for (let i = 0; i < 120; i++) { await page.waitForTimeout(1000); after = await state(); if (after.ready && after.blocks !== null && after.blocks > 0 && i > 5) break; if (i > 60 && after.ready) break; }
  await page.waitForTimeout(3000);
  after = await state();
  await note("reload", { before: count, after: after.blocks, survived: after.blocks === count, example: after.example }, from);
}
{
  const final = await state();
  await note("final", final, 0);
}
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] INTERACT DONE", report.steps.length);
await browser.close();
