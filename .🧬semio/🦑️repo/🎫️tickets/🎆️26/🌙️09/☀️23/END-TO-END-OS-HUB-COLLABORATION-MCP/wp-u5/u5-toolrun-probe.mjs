#!/usr/bin/env bun
/** ⏯️ U5 S12-2 — a REAL long-running guest run inside `s`: spawns `<pluginId>.<appId>` from the palette, arms
 * `<toolId>` through the shell's Tool category, starts the run with the framework chord (⌘⏎ `toolRunStart`),
 * then samples the ToolRun panel (status, progressbar) and the Tasks window rows every 500 ms; with `--control`
 * it presses the Tasks window's Suspend, Resume and Cancel on the run's row and records what the panel says.
 * Usage: bun u5-toolrun-probe.mjs <baseUrl> <pluginId> <appId|-> <toolId> <tag> [--locale=de] [--control] [--seconds=60] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dismissIntroduction, openPalette, seatLocale } from "../../../☀️18/OS-HUB-COLLABORATION-AI-END-TO-END/🐍️s6-all-kinds-sweep.mjs";

const [baseUrl, pluginId, appId, toolId, tag, ...flags] = process.argv.slice(2);
const flag = (name) => flags.find((value) => value === `--${name}` || value.startsWith(`--${name}=`));
const value = (name) => flag(name)?.split("=")[1];
const seconds = Number(value("seconds") ?? 60);
const out = (name) => fileURLToPath(new URL(`./generated/${name}`, import.meta.url));
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
const lines = [];
const t0 = Date.now();
page.on("console", (m) => { lines.push(`${Date.now() - t0} ${m.type()}: ${m.text()}`.slice(0, 400)); if (m.text().includes("[DEBUG] u5")) console.log(m.text().slice(0, 600)); });
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror: ${String(e)}`.slice(0, 400)));
const report = { baseUrl, pluginId, appId, toolId, tag, steps: [], samples: [] };
const step = (at, extra = {}) => { report.steps.push({ at, ms: Date.now() - t0, ...extra }); console.log(JSON.stringify({ at, ...extra }).slice(0, 700)); };
const sample = () => page.evaluate(() => {
  const runs = [...document.querySelectorAll('[data-ui-node-key^="framework.toolRun."][data-ui-node-key$=".status"]')].filter((status) => !status.getAttribute("data-ui-node-key").startsWith("framework.toolRun.ready")).map((status) => {
    const scope = status.getAttribute("data-ui-node-key").slice(0, -".status".length);
    const bar = document.querySelector(`[data-ui-node-key="${scope}.progress"]`);
    return { scope, status: (status.textContent ?? "").trim(), value: bar?.getAttribute("aria-valuenow") ?? null, max: bar?.getAttribute("aria-valuemax") ?? null, text: bar?.getAttribute("aria-valuetext") ?? bar?.getAttribute("aria-label") ?? null };
  });
  const tasks = [...document.querySelectorAll("[data-semio-task-manager-task]")].map((row) => ({ id: row.getAttribute("data-semio-task-manager-task"), lane: row.getAttribute("data-semio-task-manager-lane"), state: row.getAttribute("data-semio-task-manager-state"), progress: row.querySelector("[role='progressbar']")?.getAttribute("aria-valuetext") ?? null, buttons: [...row.querySelectorAll("button")].map((button) => `${button.getAttribute("aria-label") ?? button.textContent?.trim()}${button.disabled ? " (disabled)" : ""}`) }));
  return { runs, tasks };
});

await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
await page.waitForFunction(() => document.documentElement.dataset.semioOsReady !== undefined, undefined, { timeout: 300_000 });
await dismissIntroduction(page);
await page.waitForTimeout(2_000);
if (value("locale")) step("locale", { seated: await seatLocale(page, value("locale")) });
await openPalette(page);
const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? pluginId);
const item = page.locator(`[data-slot="command-item"][data-command-item-id="${appId === "-" ? `spawn.${pluginId}` : `spawn.${pluginId}.${appId}`}"]`).first();
await item.waitFor({ state: "visible", timeout: 20_000 }).catch(() => undefined);
step("spawn", { found: await item.count() });
await item.click({ force: true }).catch(() => undefined);
await page.waitForTimeout(12_000);
if (value("example")) {
  const picker = page.locator('[id="playground.navbar.fixture"]').first();
  const present = await picker.count();
  if (present) {
    await picker.click({ force: true }).catch(() => undefined);
    await page.waitForTimeout(800);
    const options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator(`[role="option"][data-value="${value("example")}"], [role="option"]:has-text("${value("example").replaceAll("-", " ")}")`).first();
    const picked = (await option.count()) ? await option.click({ force: true }).then(() => "ok").catch((error) => String(error).slice(0, 120)) : "no-option";
    step("example", { present, options, picked });
    await page.waitForTimeout(8_000);
  } else step("example", { present });
}
const category = page.locator('[id="framework.category.tool"]').first();
if (await category.count()) { await category.click({ force: true }).catch(() => undefined); await page.waitForTimeout(1_000); }
const rows = await page.evaluate(() => [...document.querySelectorAll('[id^="tool."]')].map((element) => ({ id: element.id, pressed: element.getAttribute("aria-pressed") ?? element.getAttribute("data-state") })));
const wanted = rows.find((row) => row.id === `tool.${toolId}`);
if (wanted && wanted.pressed !== "true" && wanted.pressed !== "on") await page.locator(`[id="tool.${toolId}"]`).first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(800);
step("tool armed", { rows: rows.slice(0, 12), wanted: wanted ?? null });
await page.evaluate(() => document.getElementById("framework.panelTab.framework.panel.toolRun")?.click());
await page.locator('[id="os.task-manager"]').first().click({ timeout: 10_000 }).catch(() => undefined);
await page.waitForTimeout(800);
await page.locator(`[data-tab-id="framework.panel.toolRun"]`).first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(1_500);
const startButton = page.locator('[data-ui-node-key="framework.toolRun.ready.toolRunStart"]').first();
if (await startButton.count()) await startButton.click({ force: true }).catch(() => undefined);
else await page.keyboard.press(process.platform === "darwin" ? "Meta+Enter" : "Control+Enter");
step("started", { via: (await startButton.count()) ? "ready Start button" : "chord" });
await page.waitForTimeout(1_500);
if (!(await page.locator("[data-semio-task-manager-window]").count())) await page.locator('[id="os.task-manager"]').first().click({ force: true }).catch(() => undefined);
await page.waitForTimeout(1_000);
await page.waitForTimeout(2_000);
const rowGeometry = await page.evaluate(() => {
  const row = document.querySelector("[data-semio-task-manager-task]");
  const rect = (element) => element ? (({ left, right, top, bottom }) => ({ left: Math.round(left), right: Math.round(right), top: Math.round(top), bottom: Math.round(bottom) }))(element.getBoundingClientRect()) : null;
  let clip = row?.parentElement ?? null;
  while (clip && !/(hidden|auto|scroll|clip)/u.test(getComputedStyle(clip).overflowX)) clip = clip.parentElement;
  return { row: rect(row), controls: [...(row?.querySelectorAll("button") ?? [])].map((button) => ({ name: button.getAttribute("aria-label"), ...rect(button) })), clip: rect(clip) };
});
if (await page.locator("[data-semio-task-manager-task]").count()) await page.locator("[data-semio-task-manager-task]").first().screenshot({ path: out(`u5-tasks-row-${tag}.png`) }).catch(() => undefined);
step("tasks window", { open: await page.locator("[data-semio-task-manager-window]").count(), rowGeometry });
const deadline = Date.now() + seconds * 1_000;
let controlled = false;
while (Date.now() < deadline) {
  await page.waitForTimeout(500);
  const state = await sample();
  const key = JSON.stringify(state);
  if (report.samples.at(-1)?.key !== key) report.samples.push({ ms: Date.now() - t0, key, ...state });
  const run = state.tasks.find((task) => task.lane === "toolRun");
  if (flag("control") && run && !controlled) {
    controlled = true;
    const plan = flag("cancel-only") ? [["cancel", 10_000]] : flag("panel-resume") ? [["suspend", 3_000], ["resume", 3_000], ["panel-resume", 3_000], ["cancel", 3_000]] : [["suspend", 4_000], ["resume", 12_000], ["suspend", 4_000], ["cancel", 12_000]];
    for (const [action, wait] of plan) {
      if (action === "panel-resume") {
        await page.locator(`[data-tab-id="framework.panel.toolRun"]`).first().click({ force: true }).catch(() => undefined);
        await page.waitForTimeout(800);
        const resume = page.locator('[data-ui-node-key$=".toolRunResume"]').first();
        const present = await resume.count();
        const bindingArgs = await page.evaluate(() => document.querySelector('[data-ui-node-key$=".toolRunResume"]')?.outerHTML.slice(0, 300) ?? null);
        if (present) await resume.click({ force: true }).catch(() => undefined);
        await page.waitForTimeout(wait);
        step(action, { present, bindingArgs, ...(await sample()) });
        continue;
      }
      const button = page.locator(`[id="os.task-manager.${action}.${run.id}"]`);
      const present = await button.count();
      const panelControls = await page.evaluate(() => [...document.querySelectorAll('[data-ui-node-key*="framework.toolRun."][data-ui-node-key$=".toolRunAbort"], [data-ui-node-key*="framework.toolRun."][data-ui-node-key$=".toolRunResume"], [data-ui-node-key*="framework.toolRun."][data-ui-node-key$=".toolRunPause"]')].map((element) => `${element.getAttribute("data-ui-node-key")}:${element.hasAttribute("disabled") || element.getAttribute("aria-disabled") === "true" ? "disabled" : "enabled"}`));
      const clickedAt = Date.now();
      let clickError = null;
      if (present) await button.click({ timeout: 5_000 }).catch((error) => { clickError = String(error).slice(0, 1600); });
      const seen = [];
      while (Date.now() - clickedAt < wait) {
        await page.waitForTimeout(500);
        const now = await sample();
        const task = now.tasks.find((candidate) => candidate.id === run.id);
        const summary = task ? `${task.state}|${task.progress?.split(" · ")[0]}` : "gone";
        if (seen.at(-1)?.summary !== summary) seen.push({ ms: Date.now() - clickedAt, summary });
      }
      const hops = await page.evaluate(({ fromEpoch }) => {
        const from = fromEpoch - performance.timeOrigin;
        const rows = {};
        for (const entry of performance.getEntriesByType("measure")) {
          if (!entry.name.startsWith("semio.hop.") || entry.startTime < from) continue;
          const stage = entry.name.slice("semio.hop.".length);
          if (!/^refresh|^turn\.accept$/u.test(stage)) continue;
          const detail = entry.detail ?? {};
          const key = `${stage}#${detail.instanceId ?? detail.actorId ?? "?"}`;
          const row = (rows[key] ??= { count: 0, maxMs: 0, lastEndMs: 0 });
          row.count += 1;
          row.maxMs = Math.max(row.maxMs, Math.round(entry.duration));
          row.lastEndMs = Math.max(row.lastEndMs, Math.round(entry.startTime + entry.duration - from));
        }
        return rows;
      }, { fromEpoch: clickedAt });
      step(action, { present, clickError, panelControls, seen, hops });
    }
  }
  if (state.runs.length > 0 && state.runs.every((entry) => /Complete|Abgeschlossen|Aborted|Abgebrochen|Faulted|Fehler|Finalized|Übernommen/u.test(entry.status)) && (!flag("control") || controlled)) break;
}
await page.screenshot({ path: out(`u5-toolrun-${tag}.png`) });
report.lines = lines.filter((line) => /error|warn|surface|refused|toolRun/iu.test(line)).slice(-60);
report.faults = lines.filter((line) => /pageerror|trap|unreachable|panicked|refused|dispatch-failed|fault/iu.test(line)).slice(-30);
writeFileSync(out(`u5-toolrun-${tag}.json`), JSON.stringify(report, null, 1));
console.log(`samples ${report.samples.length}; last ${JSON.stringify(report.samples.at(-1)?.runs ?? []).slice(0, 400)}; faults ${report.faults.length}`);
await browser.close();
