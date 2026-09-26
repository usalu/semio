#!/usr/bin/env bun
/** ⏯️ S15 — the tool-run column of the all-kinds matrix inside ONE served `s`: for every program that declares a tool, spawn it
 * from the palette, arm the tool through the shell's Tool category, and drive two real runs from the ToolRun panel / Tasks window
 * (selectors from U5's `u5-toolrun-probe.mjs`):
 *   run A: Start → progress → Pause → (frozen?) → Resume → terminal → result committed (`Check in` +1 or coalesced, a `↶` History row)
 *          → rail Undo reverts it;
 *   run B: Start → Abort at once → terminal Aborted → nothing committed.
 * Each row records what the person sees (panel status + progress text, Tasks row state + buttons) at every change.
 * usage: bun s16-toolrun-matrix.mjs <baseUrl> --tag <t> [--locale de] [--only <pluginId/appId-substring>,...] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6540/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const locale = valueOf("--locale", "en");
const only = valueOf("--only", "").split(",").filter(Boolean);
const noPause = argv.includes("--no-pause");
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
const out = fileURLToPath(new URL(`./generated/s16-toolrun-${tag}.json`, import.meta.url));

/** 🧰️ Every program that declares a tool (read from the plugin manifests, 2026-09-26), with the example a run needs. */
const PROGRAMS = [
  { pluginId: "wfc", appId: "s.wfc.bitmap@1/*#editor", toolId: "fill", mutating: false },
  { pluginId: "wfc", appId: "s.wfc.grid2d@1/*#editor", toolId: "fill", mutating: false },
  { pluginId: "wfc", appId: "s.wfc.wfc2d@1/*#editor", toolId: "fill", mutating: false },
  { pluginId: "wfc", appId: "s.wfc.grid3d@1/*#editor", toolId: "fill", mutating: false },
  { pluginId: "wfc", appId: "s.wfc.wfc3d@1/*#editor", toolId: "fill", mutating: false },
  { pluginId: "procedural", appId: "s.procedural.generation2d@1/*#editor", toolId: "previewEval", mutating: false },
  { pluginId: "procedural", appId: "s.procedural.generation3d@1/*#editor", toolId: "previewEval", mutating: false },
  { pluginId: "procedural", appId: "s.procedural.generation3d@1/*#viewer", toolId: "previewEval", mutating: false },
  { pluginId: "demonstrator", appId: "s.procedural.generation3d@1/*#editor", toolId: "previewEval", mutating: false },
  { pluginId: "demonstrator", appId: "s.puzzle.puzzle3d@1/*#editor", toolId: "fill", mutating: true, example: "first" },
  { pluginId: "reasoning", appId: "s.reasoning.wires@1/*#editor", toolId: "reorganize", mutating: true, example: "first" },
  { pluginId: "remodel", appId: "s.remodel.remodeling@1/*#editor", toolId: "reconstruction", mutating: true, example: "synthetic-orbit|Synthetic Orbit" },
  { pluginId: "energy", appId: "s.energy.model@1/*#editor", toolId: "energySimulation", mutating: false, example: "first" },
  { pluginId: "trinity", appId: "s.trinity.jack@1/*#editor", toolId: "reorganize", mutating: true },
  { pluginId: "dag", appId: "s.dag.dag@1/*#editor", toolId: "reorganize", mutating: true },
  { pluginId: "puzzle", appId: "s.puzzle.puzzle2d@1/*#editor", toolId: "fill", mutating: true, example: "concrete|Concrete" },
  { pluginId: "puzzle", appId: "s.puzzle.puzzle3d@1/*#editor", toolId: "fill", mutating: true, example: "first" },
  { pluginId: "puzzle", appId: "s.puzzle.puzzle5d@1/*#editor", toolId: "fill", mutating: true, example: "first" },
].filter((row) => only.length === 0 || only.some((entry) => `${row.pluginId}/${row.appId}`.includes(entry)));

const READY_TO_FINALIZE = /^(Complete, ready to finalize|Fertig, bereit zum Abschließen)/u;
const TERMINAL = /^(Finalized|Abgeschlossen|Aborted|Abgebrochen|Failed|Fehlgeschlagen)/u;
const ABORTED = /^(Aborted|Abgebrochen)/u;
const COMMITTED = /^(Finalized|Abgeschlossen)/u;

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const result = { baseUrl, tag, locale, started: new Date().toISOString(), rows: [] };
const flush = () => writeFileSync(out, JSON.stringify(result, null, 1));

const sample = (page) =>
  page.evaluate(() => {
    const runs = [...document.querySelectorAll('[data-ui-node-key^="framework.toolRun."][data-ui-node-key$=".status"]')].filter((status) => !status.getAttribute("data-ui-node-key").startsWith("framework.toolRun.ready")).map((status) => {
      const scope = status.getAttribute("data-ui-node-key").slice(0, -".status".length);
      const bar = document.querySelector(`[data-ui-node-key="${scope}.progress"]`);
      return { scope, status: (status.textContent ?? "").trim(), value: bar?.getAttribute("aria-valuenow") ?? null, text: bar?.getAttribute("aria-valuetext") ?? null };
    });
    const tasks = [...document.querySelectorAll("[data-semio-task-manager-task]")].filter((row) => row.getAttribute("data-semio-task-manager-lane") === "toolRun").map((row) => ({ id: row.getAttribute("data-semio-task-manager-task"), state: row.getAttribute("data-semio-task-manager-state"), progress: row.querySelector("[role='progressbar']")?.getAttribute("aria-valuetext") ?? null, buttons: [...row.querySelectorAll("button")].map((button) => `${button.id}${button.disabled ? "(disabled)" : ""}`) }));
    const checkin = (document.querySelector("#s-checkin")?.textContent ?? "").trim();
    const history = [...document.querySelectorAll('[id^="framework.history.entry."]')].filter((element) => !element.id.endsWith(".revert")).map((element) => (element.innerText ?? "").replace(/\s+/gu, " ").trim().slice(0, 60));
    const stall = [...document.querySelectorAll("[data-semio-command-stall-id]")].map((element) => (element.textContent ?? "").trim().slice(0, 120));
    return { runs, tasks, checkin, history, stall };
  });
const edits = (state) => Number(/\((\d+)\)\s*$/u.exec(state.checkin)?.[1] ?? 0);
const latestRun = (state) => state.runs.at(-1) ?? null;

async function openTools(page, program) {
  await sweep.dismissIntroduction(page);
  await page.evaluate(() => document.body.focus());
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(program.appId)?.[1] ?? program.pluginId);
  await page.waitForTimeout(1_200);
  const item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${program.pluginId}.${program.appId}"], [data-slot="command-item"][data-command-item-id="spawn.${program.pluginId}"]`).first();
  if ((await item.count()) === 0) return "no palette row";
  await item.click({ force: true });
  await page.waitForTimeout(12_000);
  if (program.example) {
    const picker = page.locator('[id="playground.navbar.fixture"]').first();
    if (await picker.count()) {
      await picker.click({ force: true }).catch(() => undefined);
      await page.waitForTimeout(800);
      const options = await page.locator('[role="option"]').evaluateAll((rows) => rows.map((row) => ({ value: row.getAttribute("data-value") ?? "", text: (row.textContent ?? "").trim(), selected: row.getAttribute("aria-selected") === "true" })));
      const wanted = program.example === "first" ? options.find((option) => !option.selected && option.value !== "" && !/^(empty|leer|none|keine)$/iu.test(option.text)) : options.find((option) => new RegExp(program.example, "u").test(`${option.value} ${option.text}`));
      program.exampleOptions = options.map((option) => `${option.value}=${option.text}${option.selected ? "*" : ""}`).slice(0, 12);
      program.examplePicked = wanted ? `${wanted.value}=${wanted.text}` : null;
      if (wanted) await page.locator('[role="option"]').nth(options.indexOf(wanted)).click({ force: true }).catch(() => undefined);
      else await page.keyboard.press("Escape");
      await page.waitForTimeout(10_000);
    } else program.examplePicked = "no fixture picker";
  }
  const category = page.locator('[id="framework.category.tool"]').first();
  if (await category.count()) {
    await category.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(1_000);
  }
  const tool = page.locator(`[id="tool.${program.toolId}"]`).first();
  if ((await tool.count()) === 0) return "no tool row";
  const pressed = await tool.getAttribute("aria-pressed").catch(() => null);
  if (pressed !== "true") await tool.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(800);
  const history = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
  if (!history) await sweep.click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(800);
  await page.locator('[id="os.task-manager"]').first().click({ timeout: 10_000 }).catch(() => undefined);
  await page.waitForTimeout(800);
  await page.locator('[data-tab-id="framework.panel.toolRun"]').first().click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(1_200);
  return null;
}

/** 🧾️ The document's pending edits and History rows, read from the History panel (raised for the reading, then the Tool runs
 * panel is raised again so the run's own controls stay in reach). */
async function readHistory(page) {
  const open = await page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"]')].some((element) => element instanceof HTMLElement && element.offsetParent !== null && /framework\.panel\.history/u.test(element.id)));
  if (!open) await sweep.click(page, '[data-slot="panel-tab-button"][id="framework.panel.history"], [id="framework.panel.history"]');
  await page.waitForTimeout(1_000);
  const state = await sample(page);
  const render = sweep.witness(await sweep.readShell(page)).render;
  await page.locator('[data-tab-id="framework.panel.toolRun"]').first().click({ force: true }).catch(() => undefined);
  await page.evaluate(() => document.getElementById("framework.panelTab.framework.panel.toolRun")?.click());
  await page.waitForTimeout(800);
  return { edits: edits(state), history: state.history.slice(-4), undoableRows: state.history.filter((row) => row.includes("↶")).length, checkin: state.checkin, render };
}

async function start(page) {
  const button = page.locator('[data-ui-node-key="framework.toolRun.ready.toolRunStart"]').first();
  if (await button.count()) {
    await button.click({ force: true }).catch(() => undefined);
    return "ready Start button";
  }
  await page.keyboard.press(process.platform === "darwin" ? "Meta+Enter" : "Control+Enter");
  return "chord";
}

async function control(page, action, taskId) {
  const tasks = page.locator(`[id="os.task-manager.${action === "pause" ? "suspend" : action === "abort" ? "cancel" : action}.${taskId}"]`).first();
  if (taskId && (await tasks.count())) {
    await tasks.click({ timeout: 5_000 }).catch(() => undefined);
    return "tasks";
  }
  const panelAction = { pause: "toolRunPause", resume: "toolRunResume", abort: "toolRunAbort" }[action];
  const panel = page.locator(`[data-ui-node-key$=".${panelAction}"]`).first();
  if (await panel.count()) {
    await panel.click({ force: true }).catch(() => undefined);
    return "panel";
  }
  return "absent";
}

async function runRow(program) {
  const context = await browser.newContext({ viewport: { width: 1600, height: 1000 } });
  const page = await context.newPage();
  const faults = [];
  page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 240)));
  page.on("console", (message) => {
    if (/refused|dispatch-failed|panicked|unreachable|trap|command stalled/iu.test(message.text())) faults.push(`${message.type()}: ${message.text()}`.slice(0, 240));
  });
  const row = { key: `${program.pluginId}/${program.appId}`, toolId: program.toolId, a: { timeline: [] }, b: { timeline: [] } };
  const note = (run, label, state) => {
    const last = run.timeline.at(-1);
    const summary = `${latestRun(state)?.status ?? "-"}|${latestRun(state)?.text ?? "-"}|${state.tasks.map((task) => `${task.state}:${task.progress}`).join(",") || "-"}|${state.checkin}`;
    if (last?.summary !== summary || label !== "sample") run.timeline.push({ label, summary, stall: state.stall });
  };
  try {
    await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
    await sweep.awaitBeacon(page, Date.now() + 300_000);
    await sweep.dismissIntroduction(page);
    await page.waitForTimeout(2_000);
    if (locale !== "en") row.locale = await sweep.seatLocale(page, locale);
    await page.keyboard.press("Escape").catch(() => undefined);
    row.open = await openTools(page, program);
    row.mutating = program.mutating;
    row.example = program.examplePicked ?? null;
    row.exampleOptions = program.exampleOptions ?? null;
    if (row.open) return row;
    const before = await readHistory(page);
    row.a.editsBefore = before.edits;
    row.a.checkinBefore = before.checkin;
    row.a.via = await start(page);
    const deadline = Date.now() + Number(valueOf("--run-ms", "120000"));
    let paused = false;
    while (Date.now() < deadline) {
      await page.waitForTimeout(400);
      const state = await sample(page);
      note(row.a, "sample", state);
      const run = latestRun(state);
      if (run) row.a.started = true;
      if (run?.text && /\d/u.test(run.text)) row.a.progressSeen = (row.a.progressSeen ?? new Set()).add(run.text);
      const task = state.tasks[0];
      if (!paused && !noPause && run && !TERMINAL.test(run.status)) {
        paused = true;
        row.a.pauseVia = await control(page, "pause", task?.id);
        await page.waitForTimeout(2_000);
        const heldA = await sample(page);
        await page.waitForTimeout(2_000);
        const heldB = await sample(page);
        note(row.a, "paused", heldB);
        row.a.pausedFrozen = JSON.stringify(latestRun(heldA)?.text) === JSON.stringify(latestRun(heldB)?.text) && !TERMINAL.test(latestRun(heldB)?.status ?? "");
        row.a.pausedState = `${latestRun(heldB)?.status ?? "-"} / ${heldB.tasks.map((entry) => entry.state).join(",")}`;
        row.a.resumeVia = await control(page, "resume", heldB.tasks[0]?.id ?? task?.id);
        await page.waitForTimeout(1_500);
        note(row.a, "resumed", await sample(page));
      }
      if (run && READY_TO_FINALIZE.test(run.status) && !row.a.finalizeVia) {
        row.a.readyToFinalize = run.status;
        const finalize = page.locator('[data-ui-node-key$=".toolRunFinalize"]').first();
        row.a.finalizeVia = (await finalize.count()) ? await finalize.click({ force: true }).then(() => "panel", () => "panel-unclickable") : "absent";
        await page.waitForTimeout(1_500);
        continue;
      }
      if (run && TERMINAL.test(run.status)) {
        row.a.terminal = run.status;
        break;
      }
    }
    if (!paused) row.a.pauseVia = "finished before a pause";
    await page.waitForTimeout(2_500);
    const after = await readHistory(page);
    row.a.editsAfter = after.edits;
    row.a.checkinAfter = after.checkin;
    row.a.historyTail = after.history;
    row.a.documentChanged = after.render !== before.render;
    row.a.newUndoableRows = after.undoableRows - before.undoableRows;
    row.a.committed = COMMITTED.test(row.a.terminal ?? "") && (row.a.documentChanged || row.a.editsAfter > row.a.editsBefore) && row.a.newUndoableRows > 0;
    if (COMMITTED.test(row.a.terminal ?? "")) {
      row.a.undo = await sweep.clickUncovered(page, '[data-slot="window-action-pane"] [id="action.undo"]');
      if (row.a.undo !== "ok") {
        await page.keyboard.press(process.platform === "darwin" ? "Meta+KeyZ" : "Control+KeyZ");
        row.a.undo = "chord";
      }
      await page.waitForTimeout(3_000);
      const undone = await readHistory(page);
      row.a.editsAfterUndo = undone.edits;
      row.a.undoRestored = undone.render === before.render;
      row.a.undone = row.a.committed && row.a.undoRestored && row.a.editsAfterUndo === row.a.editsBefore;
    }
    if (row.a.progressSeen) row.a.progressSeen = [...row.a.progressSeen].slice(0, 6);
    const beforeB = await sample(page);
    row.b.editsBefore = (await readHistory(page)).edits;
    const knownScopes = new Set(beforeB.runs.map((run) => run.scope));
    row.b.via = await start(page);
    const deadlineB = Date.now() + 60_000;
    let aborted = false;
    while (Date.now() < deadlineB) {
      await page.waitForTimeout(250);
      const state = await sample(page);
      note(row.b, "sample", state);
      const run = state.runs.filter((entry) => !knownScopes.has(entry.scope)).at(-1) ?? null;
      if (run && !aborted && !TERMINAL.test(run.status)) {
        aborted = true;
        row.b.abortVia = await control(page, "abort", state.tasks[0]?.id);
      }
      if (run && TERMINAL.test(run.status)) {
        row.b.terminal = run.status;
        break;
      }
    }
    if (!aborted) row.b.abortVia = "finished before an abort";
    await page.waitForTimeout(2_000);
    row.b.editsAfter = (await readHistory(page)).edits;
    row.b.aborted = ABORTED.test(row.b.terminal ?? "") && row.b.editsAfter === row.b.editsBefore;
    row.verdict = program.mutating
      ? (row.a.committed && row.a.undone ? "commit+undo PASS" : `commit ${row.a.committed ? "ok" : "MISSING"} / undo ${row.a.undone ? "ok" : "MISSING"}`)
      : (COMMITTED.test(row.a.terminal ?? "") && row.a.editsAfter === row.a.editsBefore ? "read-only PASS (finalized, nothing written)" : `read-only ${row.a.terminal ?? "no terminal"}`);
    await page.screenshot({ path: fileURLToPath(new URL(`./generated/s16-toolrun-${tag}-${row.key.replace(/[^A-Za-z0-9]+/gu, "-")}.png`, import.meta.url)) }).catch(() => undefined);
  } catch (error) {
    row.error = String(error).slice(0, 300);
  } finally {
    row.faults = faults.slice(0, 8);
    await context.close();
  }
  return row;
}

for (const program of PROGRAMS) {
  const row = await runRow(program);
  result.rows.push(row);
  flush();
  console.log(`[s15-tool] ${row.key} open=${row.open ?? "ok"} A: start=${row.a.started ?? false} progress=${(row.a.progressSeen ?? []).length} pause=${row.a.pauseVia}${row.a.pausedFrozen === undefined ? "" : row.a.pausedFrozen ? "(frozen)" : "(NOT frozen)"} terminal=${row.a.terminal ?? "-"} changed=${row.a.documentChanged ?? "-"} ↶rows=${row.a.newUndoableRows ?? "-"} committed=${row.a.committed ?? false} undone=${row.a.undone ?? "-"} | B: abort=${row.b.abortVia ?? "-"} terminal=${row.b.terminal ?? "-"} aborted=${row.b.aborted ?? false} | faults=${row.faults.length} | ex=${row.example ?? "-"} | ${row.verdict ?? "-"} ${row.error ?? ""}`);
}
result.finished = new Date().toISOString();
flush();
await browser.close();
console.log(`=== ${tag} → ${out}`);
