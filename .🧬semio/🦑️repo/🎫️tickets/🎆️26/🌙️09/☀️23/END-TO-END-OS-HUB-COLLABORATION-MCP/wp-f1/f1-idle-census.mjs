#!/usr/bin/env bun
/** 💤️ F1 — idle-cost census of every spawnable program inside ONE served `s` (React shell).
 *
 * Per program: opened from Home by the palette chord, waited until its body renders, settled (main-thread task time
 * < 5 % for 3 consecutive seconds, at most 30 s), then 10 s with NO input are traced (CDP) → main-thread busy % (wall) and
 * CPU % (thread time), main frames/s, animation-frame callbacks/s, paints/s, style recalcs/s, timers/s, compositor draws/s,
 * dedicated-worker busy %; the in-page hooks name the requesters of animation frames and timers. After a forced GC: JS heap
 * of the page, wasm linear memory of the main thread, every worker's heap + backing store. An offender (≥ 2 % busy or ≥ 1
 * frame/s) also gets a 3 s CPU profile. The program is closed by its dock tab and the page is re-measured for 4 s
 * (a loop that survives its window is a leak).
 *
 * usage: bun f1-idle-census.mjs <baseUrl> --tag <t> [--roles editor,viewer] [--only plugin|plugin/kind,...] [--skip …]
 *        [--seconds 10] [--reload-every 12] [--resume] */
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { launch, metrics, profileWindow, sleep, summarize, traceWindow } from "./f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6620/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const roles = valueOf("--roles", "editor,viewer").split(",");
const only = valueOf("--only", "").split(",").filter(Boolean);
const skip = valueOf("--skip", "").split(",").filter(Boolean);
const seconds = Number(valueOf("--seconds", "10"));
const reloadEvery = Number(valueOf("--reload-every", "12"));
const resume = argv.includes("--resume");
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const out = `${generated}f1-idle-${tag}.json`;
const log = (...parts) => console.log(`[f1 ${new Date().toISOString().slice(11, 19)}]`, ...parts);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");

const kindOf = (appId) => /^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId;
const roleOf = (appId) => appId.split("#").at(-1);
const subsetOf = (appId) => /@[^/]+\/([^#]+)#/u.exec(appId)?.[1] ?? "*";
const keyOf = (program) => `${program.pluginId}/${kindOf(program.appId)}${subsetOf(program.appId) === "*" ? "" : `/${subsetOf(program.appId)}`}`;

let browser;
let page;
let cdp;
let workers;
let census = null;

const windowIds = () => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));

async function boot() {
  if (browser) await browser.close().catch(() => undefined);
  ({ browser, page, cdp, workers } = await launch());
  const started = Date.now();
  await page.goto(baseUrl, { waitUntil: "commit" });
  const beacon = await sweep.awaitBeacon(page, Date.now() + 300_000);
  await sweep.dismissIntroduction(page);
  await page.keyboard.press("Escape").catch(() => undefined);
  await settle(20_000);
  const probe = await page.evaluate(() => window.__semioOsCatalogProbe ?? null);
  census = probe?.programs ?? census;
  const heap = await heapFacts();
  return { beacon, bootMs: Date.now() - started, programs: probe?.programs?.length ?? null, loaded: (probe?.plugins ?? []).filter((row) => row.status === "loaded").length, heap };
}

/** ⏳️ Waits until the main thread's task time stays under 5 % for 3 consecutive seconds (at most `budgetMs`). */
async function settle(budgetMs) {
  const started = Date.now();
  let quiet = 0;
  let last = (await metrics(cdp)).TaskDuration;
  while (Date.now() - started < budgetMs) {
    await sleep(1_000);
    const now = (await metrics(cdp)).TaskDuration;
    quiet = now - last < 0.05 ? quiet + 1 : 0;
    last = now;
    if (quiet >= 3) return { settled: true, ms: Date.now() - started };
  }
  return { settled: false, ms: Date.now() - started };
}

async function heapFacts() {
  await cdp.send("HeapProfiler.collectGarbage").catch(() => undefined);
  const m = await metrics(cdp);
  const main = await page.evaluate(() => window.__f1.read());
  const rows = await workers.heap();
  const mb = (bytes) => +(bytes / 2 ** 20).toFixed(1);
  return {
    jsUsedMB: mb(m.JSHeapUsedSize),
    mainWasmMB: mb(main.wasmBytes),
    mainWasmMemories: main.wasmMemories,
    workers: rows.length,
    workersHeapMB: mb(rows.reduce((sum, row) => sum + row.used, 0)),
    workersBackingMB: mb(rows.reduce((sum, row) => sum + (row.backing ?? 0), 0)),
    nodes: m.Nodes,
    listeners: m.JSEventListeners,
  };
}

async function openPalette() {
  await sweep.dismissIntroduction(page);
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    document.body.focus();
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  return input;
}

async function openProgram(program) {
  const before = await windowIds();
  const input = await openPalette();
  if ((await input.count()) === 0) return { windowIds: [], detail: "command palette never opened" };
  await input.fill(kindOf(program.appId));
  await sleep(1_200);
  const ids = [`spawn.${program.pluginId}.${program.appId}`];
  if (census?.find((entry) => entry.pluginId === program.pluginId)?.appId === program.appId) ids.push(`spawn.${program.pluginId}`);
  let pressed = false;
  for (const id of ids) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
    if ((await item.count()) > 0) {
      await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
      pressed = true;
      break;
    }
  }
  if (!pressed) {
    await page.keyboard.press("Escape");
    return { windowIds: [], detail: `no ${ids.join(" | ")} palette row` };
  }
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    const fresh = (await windowIds()).filter((id) => !before.includes(id));
    if (fresh.length > 0) return { windowIds: fresh, detail: null };
    await sleep(250);
  }
  return { windowIds: [], detail: "palette row pressed, no new window" };
}

const bodyFacts = (ids) =>
  page.evaluate((wanted) =>
    wanted.map((id) => {
      const host = document.getElementById(id);
      const body = host?.querySelector('[data-slot="window-body"]') ?? null;
      if (!host || !body) return { id, present: false };
      const canvases = [...body.querySelectorAll("canvas")].map((canvas) => `${canvas.width}x${canvas.height}`);
      return { id, present: true, canvases, svg: body.querySelectorAll("svg *").length, elements: body.querySelectorAll("*").length, skeleton: body.querySelector('[aria-busy="true"]') !== null && body.querySelectorAll("*").length < 20, fault: host.querySelector("[data-semio-window-fault]")?.getAttribute("data-semio-window-fault") ?? null };
    }),
  ids);

async function closeProgram(ids) {
  for (const id of ids) {
    await page.evaluate((windowId) => {
      const tab = [...document.querySelectorAll('[data-slot="mode-dock-tab"]')].find((element) => element.getAttribute("data-window-id") === windowId);
      const button = tab?.querySelector('[data-slot="mode-dock-tab-close"]') ?? tab?.parentElement?.querySelector('[data-slot="mode-dock-tab-close"]');
      if (button instanceof HTMLElement) button.click();
    }, id);
    await sleep(400);
  }
  await sleep(1_500);
  return (await windowIds()).filter((id) => ids.includes(id));
}

async function onHome() {
  const ids = await windowIds().catch(() => null);
  return ids !== null && ids.length === 1 && ids[0] === "s-home-main";
}

async function measureRow(program) {
  const started = Date.now();
  const row = { key: keyOf(program), role: roleOf(program.appId), appId: program.appId };
  const opened = await openProgram(program);
  row.windowIds = opened.windowIds;
  row.openDetail = opened.detail;
  if (opened.windowIds.length === 0) return row;
  const renderDeadline = Date.now() + 60_000;
  let facts = await bodyFacts(opened.windowIds);
  while (Date.now() < renderDeadline && !facts.every((entry) => entry.present && !entry.skeleton)) {
    await sleep(500);
    facts = await bodyFacts(opened.windowIds);
  }
  row.openMs = Date.now() - started;
  row.settle = await settle(30_000);
  row.render = await bodyFacts(opened.windowIds);
  await page.evaluate(() => window.__f1.sites(true));
  const trace = await traceWindow(cdp, seconds * 1000);
  row.idle = summarize(trace, seconds);
  row.sites = await page.evaluate(() => window.__f1.sites(false));
  row.heap = await heapFacts();
  row.offender = row.idle.mainBusyPct >= 2 || row.idle.mainFramesPerSec >= 1 || row.idle.rafPerSec >= 1 || row.idle.workersBusyPct >= 2 || row.idle.compositorDrawsPerSec >= 1;
  if (row.offender) row.profile = await profileWindow(cdp, 3_000);
  row.remaining = await closeProgram(opened.windowIds);
  const f0 = await page.evaluate(() => window.__f1.read());
  const m0 = await metrics(cdp);
  await sleep(4_000);
  const m1 = await metrics(cdp);
  const f1 = await page.evaluate(() => window.__f1.read());
  row.afterClose = { taskPct: +((100 * (m1.TaskDuration - m0.TaskDuration)) / 4).toFixed(2), rafPerSec: +((f1.rafCallbacks - f0.rafCallbacks) / 4).toFixed(2) };
  row.totalMs = Date.now() - started;
  return row;
}

const previous = resume ? JSON.parse(readFileSync(out, "utf8")) : null;
const result = { baseUrl, tag, roles, seconds, started: previous?.started ?? new Date().toISOString(), boots: previous?.boots ?? [], rows: (previous?.rows ?? []).filter((row) => row.idle || row.hung) };
const done = new Set(result.rows.map((row) => `${row.key}#${row.role}`));
const flush = () => writeFileSync(out, JSON.stringify(result, null, 1));
let current = null;
let lastProgress = Date.now();
setInterval(() => {
  if (Date.now() - lastProgress < 360_000) return;
  if (current) result.rows.push({ key: keyOf(current), role: roleOf(current.appId), appId: current.appId, hung: true, openDetail: "hung: no progress for 360 s (process watchdog; the run restarts with --resume)" });
  result.fatal = "process watchdog";
  flush();
  log(`HUNG ${current ? keyOf(current) : "(boot)"} — exiting for a resume`);
  process.exit(3);
}, 30_000).unref();
try {
  result.boots.push(await boot());
  log(`boot ${JSON.stringify(result.boots.at(-1))}`);
  const programs = census.filter((program) => roles.includes(roleOf(program.appId)) && !(program.pluginId === "space" && /\.home@/u.test(program.appId))).filter((program) => (only.length === 0 || only.some((entry) => entry === program.pluginId || entry === keyOf(program))) && !skip.some((entry) => entry === program.pluginId || entry === keyOf(program)));
  result.selected = programs.length;
  let sinceBoot = 0;
  for (const program of programs) {
    if (done.has(`${keyOf(program)}#${roleOf(program.appId)}`)) continue;
    current = program;
    lastProgress = Date.now();
    if (sinceBoot >= reloadEvery || !(await onHome())) {
      result.boots.push(await boot());
      sinceBoot = 0;
    }
    sinceBoot += 1;
    let row;
    const guarded = (entry) => Promise.race([measureRow(entry), new Promise((_, reject) => setTimeout(() => reject(new Error("row watchdog: 240 s")), 240_000)), new Promise((_, reject) => browser.once("disconnected", () => reject(new Error("browser disconnected"))))]);
    try {
      row = await guarded(program);
    } catch (error) {
      log(`row ${keyOf(program)} interrupted (${String(error).split("\n")[0].slice(0, 140)}); re-booting and retrying once`);
      result.boots.push(await boot());
      sinceBoot = 1;
      row = await guarded(program).catch((retry) => ({ key: keyOf(program), role: roleOf(program.appId), appId: program.appId, openDetail: `probe error: ${String(retry).split("\n")[0].slice(0, 160)}` }));
      row.retried = true;
    }
    result.rows.push(row);
    lastProgress = Date.now();
    flush();
    const i = row.idle;
    log(`${row.offender ? "OFFENDER" : row.idle ? "clean   " : "NO-OPEN "} ${row.key}#${row.role} busy=${i?.mainBusyPct ?? "-"}% cpu=${i?.mainCpuPct ?? "-"}% frames=${i?.mainFramesPerSec ?? "-"}/s raf=${i?.rafPerSec ?? "-"}/s draws=${i?.compositorDrawsPerSec ?? "-"}/s workers=${i?.workersBusyPct ?? "-"}% heap=${row.heap?.jsUsedMB ?? "-"}MB wasm=${row.heap?.mainWasmMB ?? "-"}MB wBacking=${row.heap?.workersBackingMB ?? "-"}MB afterClose=${JSON.stringify(row.afterClose ?? null)} ${row.openDetail ?? ""} ${Math.round((row.totalMs ?? 0) / 1000)}s`);
    if (!row.idle) {
      result.boots.push(await boot());
      sinceBoot = 0;
    }
  }
} catch (error) {
  result.fatal = String(error);
  log(`FATAL ${result.fatal}`);
} finally {
  result.finished = new Date().toISOString();
  flush();
  await browser?.close().catch(() => undefined);
}
log(`=== ${tag} → ${out}: ${result.rows.filter((row) => row.idle).length} measured, ${result.rows.filter((row) => row.offender).length} offenders ===`);
process.exit(result.fatal ? 1 : 0);
