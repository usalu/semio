/** 🖱️ Flow node-graph SCROLL cost probe for the procedural 3d REACT door: drives 30 real wheel ticks
 * (zoom) and a 1 s middle-button drag-pan over the Flow canvas at ~60 Hz on two examples and reports,
 * per tick, what the gesture actually cost.
 *
 * 🩺️ The paint marker is the `semio.hop.surface.paint` span the node-graph host publishes around one
 * `renderCanvas` present — NOT the board's `[DEBUG] dag draw …` console line, which the wasm board
 * emits only when the LOD bucket changes (`🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`) and which
 * therefore counts LOD transitions, not frames. Wheel/pointer stamps come from a capture listener
 * installed by `page.addInitScript` before the app boots, so the gesture, the paint and the hop spans
 * all sit on ONE page clock and "time from this tick to the next paint" is one subtraction. Playwright's
 * own `console` event carries the PROBE's clock instead, which is the CDP queue's latency
 * (memory: `feedback-browser-console-buffer-survives-reload`).
 *
 * 🧾️ Per tick: paint latency (tick → next present landing), guest invocations dispatched during the
 * gesture by `actionId` (`semio.hop.invoke`), `refreshUi` passes (`semio.hop.refresh`), React commits
 * (`semio.hop.commit`), CDP `Performance.getMetrics` deltas for `RecalcStyleDuration` /
 * `LayoutDuration` / `ScriptDuration` / `TaskDuration`, and every long task > 50 ms.
 *
 * Gate (`SEMIO_PROBE_GATE=1`): median paint ≤ 16 ms, p95 ≤ 50 ms, 0 guest invocations during the
 * gesture, exactly 1 `nodeGraphViewport` publication at settle.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6022/?plugin=generation3d SEMIO_PROBE_OUT=flow-scroll/before bun 🐍️flow-scroll-render-perf-probe.mjs
 *
 * @see `🐍️react-hop-cost-probe.mjs` — the hop-span reading technique this reuses
 * @see `🐍️wgpu-raf-cost-probe.mjs` — the frame-cost technique on the wgpu door
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6022/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "flow-scroll/cost");
const debugPort = Number(process.env.SEMIO_PROBE_CDP_PORT ?? 9372);
const wheelTicks = Number(process.env.SEMIO_PROBE_WHEEL_TICKS ?? 30);
const panMs = Number(process.env.SEMIO_PROBE_PAN_MS ?? 1000);
const bootWaitS = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 90);
const settleMs = Number(process.env.SEMIO_PROBE_SETTLE_MS ?? 1200);
const gate = process.env.SEMIO_PROBE_GATE === "1";
const examples = (process.env.SEMIO_PROBE_EXAMPLES ?? "hexagonal-mushroom-column,sphere-cut-with-torus").split(",").map((entry) => entry.trim()).filter(Boolean);
mkdirSync(outDir, { recursive: true });

const MEASURE_PREFIX = "semio.hop.";
const PAINT_BUDGET_MEDIAN_MS = Number(process.env.SEMIO_PROBE_MEDIAN_BUDGET_MS ?? 16);
const PAINT_BUDGET_P95_MS = Number(process.env.SEMIO_PROBE_P95_BUDGET_MS ?? 50);

const lines = [];
const t0 = Date.now();

//#region 🪝️PageHooks
/** 🪝️ Installed before ANY app module runs: the draw log, the wheel/pointer stamps and the long-task
 * observer all publish into one page-clock record the probe reads at the end of a gesture. */
const initScript = () => {
  const probe = { draws: [], wheels: [], moves: [], longTasks: [], marks: [] };
  Object.defineProperty(window, "__flowScrollProbe", { value: probe, configurable: true });
  const originalLog = console.log.bind(console);
  console.log = (...args) => {
    const first = args[0];
    if (typeof first === "string" && first.includes("dag draw")) probe.draws.push(performance.now());
    return originalLog(...args);
  };
  try {
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) probe.longTasks.push({ startMs: entry.startTime, durationMs: entry.duration });
    }).observe({ entryTypes: ["longtask"] });
  } catch {
    /* a host without the long-task entry type simply reports none */
  }
  window.addEventListener("wheel", (event) => probe.wheels.push({ atMs: performance.now(), deltaY: event.deltaY, trusted: event.isTrusted }), { capture: true, passive: true });
  window.addEventListener("pointermove", (event) => { if (event.buttons !== 0) probe.moves.push({ atMs: performance.now(), buttons: event.buttons }); }, { capture: true, passive: true });
};
//#endregion 🪝️PageHooks

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", `--remote-debugging-port=${debugPort}`] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.addInitScript(initScript);
page.on("console", (message) => lines.push(`${Date.now() - t0} ${message.type()} ${message.text().slice(0, 600)}`));
page.on("pageerror", (error) => lines.push(`${Date.now() - t0} pageerror ${String(error).slice(0, 600)}`));

const pageSession = await page.context().newCDPSession(page);
await pageSession.send("Performance.enable");
const pageMetrics = async () => Object.fromEntries((await pageSession.send("Performance.getMetrics")).metrics.map((metric) => [metric.name, metric.value]));

//#region 📏️Reads
const resetProbe = () =>
  page.evaluate((prefix) => {
    const probe = window.__flowScrollProbe;
    probe.draws.length = 0;
    probe.wheels.length = 0;
    probe.moves.length = 0;
    probe.longTasks.length = 0;
    for (const entry of performance.getEntriesByType("measure")) if (entry.name.startsWith(prefix)) performance.clearMeasures(entry.name);
    return performance.now();
  }, MEASURE_PREFIX);

const readProbe = () =>
  page.evaluate((prefix) => {
    const probe = window.__flowScrollProbe;
    return {
      nowMs: performance.now(),
      draws: [...probe.draws],
      wheels: [...probe.wheels],
      moves: [...probe.moves],
      longTasks: [...probe.longTasks],
      spans: performance
        .getEntriesByType("measure")
        .filter((entry) => entry.name.startsWith(prefix))
        .map((entry) => ({ stage: entry.name.slice(prefix.length), startMs: entry.startTime, durationMs: entry.duration, detail: entry.detail ?? null })),
    };
  }, MEASURE_PREFIX);

/** 📐️ The Flow node-graph canvas rect, found through the surface host's own probe registry so the
 * gesture lands on the board and never on a sibling window. */
const flowRect = () =>
  page.evaluate(() => {
    const registry = window.__semioFlowGraphProbe ?? {};
    for (const [surfaceId, entry] of Object.entries(registry)) {
      const rect = entry.rect?.();
      if (rect && rect.width > 80 && rect.height > 80) return { surfaceId, ...rect };
    }
    return null;
  });

const graphState = () =>
  page.evaluate(() => {
    const parse = (text) => { try { return JSON.parse(text); } catch { return null; } };
    const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((element) => {
      const status = parse(element.getAttribute("data-status-json"));
      return { surfaceId: element.getAttribute("data-surface-id"), phase: status?.phase ?? null, ratio: status?.progress?.ratio ?? null, fault: status?.fault?.code ?? null };
    });
    const registry = window.__semioFlowGraphProbe ?? {};
    const fixture = Object.values(registry)[0]?.hostSnapshotJson?.() ?? null;
    let widgets = 0;
    try { widgets = (JSON.parse(fixture ?? "{}").widgets ?? []).length; } catch { widgets = 0; }
    const combo = document.querySelector('[role="combobox"]');
    const painted = performance.getEntriesByType("measure").some((entry) => entry.name === "semio.hop.surface.paint");
    return { hosts, widgets, painted, example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null };
  });
//#endregion 📏️Reads

//#region 🧮️Statistics
const quantile = (values, fraction) => {
  if (values.length === 0) return null;
  const sorted = [...values].sort((left, right) => left - right);
  const index = Math.min(sorted.length - 1, Math.max(0, Math.round(fraction * (sorted.length - 1))));
  return sorted[index];
};
/** 🎨️ Time from each gesture event to the END of the FIRST board present that follows it — the
 * user-visible "did the canvas move" latency. A tick with no following paint is reported as a miss,
 * never as 0. */
const paintLatencies = (events, paints) => {
  const latencies = [];
  let misses = 0;
  for (const event of events) {
    const landed = paints.find((at) => at >= event.atMs);
    if (landed === undefined) misses += 1;
    else latencies.push(landed - event.atMs);
  }
  return { latencies, misses };
};
/** 🖼️ The instants board frames actually landed, from the host's own present span. */
const paintSpans = (spans) => spans.filter((span) => span.stage === "surface.paint").slice().sort((left, right) => left.startMs - right.startMs);
const paintInstants = (spans) => paintSpans(spans).map((span) => span.startMs + span.durationMs);
/** 🛰️ Guest invocations the host dispatched inside a window, by `actionId` — a wheel tick that costs
 * a `performInvocation` is the defect this probe exists to name. */
const invocationsByAction = (spans, fromMs, toMs) => {
  const counts = {};
  for (const span of spans) {
    if (span.stage !== "invoke") continue;
    if (span.startMs < fromMs || span.startMs > toMs) continue;
    const action = String(span.detail?.actionId ?? "?");
    counts[action] = (counts[action] ?? 0) + 1;
  }
  return counts;
};
const stageCount = (spans, stage, fromMs, toMs) => spans.filter((span) => span.stage === stage && span.startMs >= fromMs && span.startMs <= toMs).length;
//#endregion 🧮️Statistics

//#region 🤫️Quiescence
/** 🤫️ Waits until the shell stops dispatching on its own before a gesture is driven.
 *
 * A boot tail is not a gesture: the surface re-attaches once more a dozen seconds in and its
 * `applyFlowStartupCamera` publishes the opening camera, so a gesture started too early counts that
 * publication as its own and reads a per-tick hop that never happened. Quiescence is defined as the
 * `semio.hop.invoke` count holding still — the same spans the verdict is read from, so the probe
 * waits on exactly what it measures. */
const settleShell = async (quietMs = 1500, maximumMs = 30_000) => {
  const invocations = () => page.evaluate((prefix) => performance.getEntriesByType("measure").filter((entry) => entry.name === `${prefix}invoke`).length, MEASURE_PREFIX);
  const startedAt = Date.now();
  let previous = await invocations();
  let quietSince = Date.now();
  while (Date.now() - startedAt < maximumMs) {
    await page.waitForTimeout(250);
    const current = await invocations();
    if (current !== previous) {
      previous = current;
      quietSince = Date.now();
      continue;
    }
    if (Date.now() - quietSince >= quietMs) return true;
  }
  return false;
};
//#endregion 🤫️Quiescence

//#region 🖱️Gestures
const wheelGesture = async (rect) => {
  const cx = Math.round(rect.x + rect.width / 2);
  const cy = Math.round(rect.y + rect.height / 2);
  await page.mouse.move(cx, cy);
  const quiet = await settleShell();
  const startedAt = await resetProbe();
  const beforePage = await pageMetrics();
  for (let index = 0; index < wheelTicks; index += 1) {
    await page.mouse.wheel(0, index % 2 === 0 ? -120 : -120);
    await page.waitForTimeout(16);
  }
  const gestureEndedAt = (await readProbe()).nowMs;
  await page.waitForTimeout(settleMs);
  const after = await readProbe();
  const afterPage = await pageMetrics();
  return { kind: "wheel-zoom", quiet, startedAt, gestureEndedAt, before: beforePage, after: afterPage, sample: after, events: after.wheels };
};

const panGesture = async (rect) => {
  const cx = Math.round(rect.x + rect.width / 2);
  const cy = Math.round(rect.y + rect.height / 2);
  const steps = Math.max(1, Math.round(panMs / 16));
  await page.mouse.move(cx - 150, cy - 80);
  const quiet = await settleShell();
  const startedAt = await resetProbe();
  const beforePage = await pageMetrics();
  await page.mouse.down({ button: "middle" });
  for (let index = 1; index <= steps; index += 1) {
    const phase = (index / steps) * Math.PI * 2;
    await page.mouse.move(Math.round(cx - 150 + Math.cos(phase) * 120), Math.round(cy - 80 + Math.sin(phase) * 70));
    await page.waitForTimeout(16);
  }
  await page.mouse.up({ button: "middle" });
  const gestureEndedAt = (await readProbe()).nowMs;
  await page.waitForTimeout(settleMs);
  const after = await readProbe();
  const afterPage = await pageMetrics();
  return { kind: "drag-pan", quiet, startedAt, gestureEndedAt, before: beforePage, after: afterPage, sample: after, events: after.moves };
};
//#endregion 🖱️Gestures

//#region 📊️Rows
const buildRow = (example, gesture) => {
  const { sample, before, after, startedAt } = gesture;
  // 🧭️ The gesture ENDS at its last input event, not at the probe's next CDP round trip: a drag's
  // settled publication is dispatched by `pointerup` itself, so a boundary drawn after the release
  // counts the settle as "during" and reads a per-tick hop that never happened.
  const gestureEndedAt = gesture.events.length > 0 ? Math.max(...gesture.events.map((event) => event.atMs)) : gesture.gestureEndedAt;
  const paints = paintInstants(sample.spans);
  const { latencies, misses } = paintLatencies(gesture.events, paints);
  const duringInvocations = invocationsByAction(sample.spans, startedAt, gestureEndedAt);
  const settleInvocations = invocationsByAction(sample.spans, gestureEndedAt, sample.nowMs);
  const duringTotal = Object.values(duringInvocations).reduce((sum, value) => sum + value, 0);
  const settleTotal = Object.values(settleInvocations).reduce((sum, value) => sum + value, 0);
  return {
    example,
    gesture: gesture.kind,
    quietBefore: gesture.quiet,
    events: gesture.events.length,
    paints: paints.length,
    lodChanges: sample.draws.length,
    paintMedianMs: quantile(latencies, 0.5),
    paintP95Ms: quantile(latencies, 0.95),
    paintMaxMs: latencies.length ? Math.max(...latencies) : null,
    paintMisses: misses,
    paintDurationMedianMs: quantile(paintSpans(sample.spans).map((span) => span.durationMs), 0.5),
    paintDurationMaxMs: paintSpans(sample.spans).length ? Math.max(...paintSpans(sample.spans).map((span) => span.durationMs)) : null,
    guestInvocationsDuring: duringTotal,
    guestInvocationsDuringByAction: duringInvocations,
    guestInvocationsAtSettle: settleTotal,
    guestInvocationsAtSettleByAction: settleInvocations,
    viewportPublicationsDuring: duringInvocations.nodeGraphViewport ?? 0,
    viewportPublicationsAtSettle: settleInvocations.nodeGraphViewport ?? 0,
    refreshPassesDuring: stageCount(sample.spans, "refresh", startedAt, gestureEndedAt),
    refreshPassesAtSettle: stageCount(sample.spans, "refresh", gestureEndedAt, sample.nowMs),
    reactCommitsDuring: stageCount(sample.spans, "commit", startedAt, gestureEndedAt),
    reactCommitsAtSettle: stageCount(sample.spans, "commit", gestureEndedAt, sample.nowMs),
    recalcStyleMs: ((after.RecalcStyleDuration ?? 0) - (before.RecalcStyleDuration ?? 0)) * 1000,
    layoutMs: ((after.LayoutDuration ?? 0) - (before.LayoutDuration ?? 0)) * 1000,
    scriptMs: ((after.ScriptDuration ?? 0) - (before.ScriptDuration ?? 0)) * 1000,
    taskMs: ((after.TaskDuration ?? 0) - (before.TaskDuration ?? 0)) * 1000,
    longTasks: sample.longTasks.filter((entry) => entry.durationMs > 50).map((entry) => Math.round(entry.durationMs)),
    gestureWallMs: gestureEndedAt - startedAt,
  };
};
//#endregion 📊️Rows

//#region ▶️Run
const rows = [];
const settledHost = (host) => host.phase === "idle" && host.ratio === 1 && !host.fault;

for (const example of examples) {
  // 🪞 One console file per DOCUMENT. Accumulating across the examples made one page look like two
  // attaches of the same surface — `node-graph surface ready` twice for `window:procedural-main` —
  // which was read as a re-attach defect and handed on as one
  // (`📓️flow-scroll-render-perf-2026-09-15.md` §9). Two navigations are two documents.
  lines.length = 0;
  await page.goto(`${url}&example=${example}`, { waitUntil: "domcontentloaded" });
  let state = null;
  let rect = null;
  for (let second = 0; second < bootWaitS; second += 1) {
    await page.waitForTimeout(1000);
    state = await graphState();
    rect = await flowRect();
    const previews = state.hosts.filter((host) => host.surfaceId && host.surfaceId.endsWith("-preview"));
    // 🖼️ Readiness is the board having PAINTED and the previews having settled. Widget count is a
    // diagnostic, not a gate: it is read out of the surface's published host snapshot, which a peer's
    // rename can move out from under this probe — and a boot gate that waits on a field nobody
    // publishes any more burns its whole budget before measuring anything.
    if (rect && state.painted && previews.length > 0 && previews.every(settledHost)) break;
  }
  if (!rect) {
    lines.push(`${Date.now() - t0} probe found no flow canvas for ${example}`);
    rows.push({ example, gesture: "n/a", error: "no flow canvas" });
    continue;
  }
  console.log(`[DEBUG] flow-scroll ${example}: surface=${rect.surfaceId} widgets=${state?.widgets} rect=${Math.round(rect.width)}x${Math.round(rect.height)}`);
  const wheel = await wheelGesture(rect);
  rows.push({ ...buildRow(example, wheel), widgets: state?.widgets ?? 0 });
  const pan = await panGesture(rect);
  rows.push({ ...buildRow(example, pan), widgets: state?.widgets ?? 0 });
  const rawSpans = { wheel: wheel.sample.spans, wheelEvents: wheel.events, pan: pan.sample.spans };
  writeFileSync(join(outDir, "scroll.json"), JSON.stringify(rows, null, 2));
  if (process.env.SEMIO_PROBE_RAW_SPANS === "1") writeFileSync(join(outDir, `spans-${example}.json`), JSON.stringify(rawSpans, null, 2));
  for (const row of rows.slice(-2)) {
    console.log(
      `[DEBUG] ${row.example} ${row.gesture}: events=${row.events} paints=${row.paints} lod=${row.lodChanges} paint median=${row.paintMedianMs === null ? "-" : row.paintMedianMs.toFixed(1)}ms ` +
        `p95=${row.paintP95Ms === null ? "-" : row.paintP95Ms.toFixed(1)}ms misses=${row.paintMisses} guestDuring=${row.guestInvocationsDuring} guestSettle=${row.guestInvocationsAtSettle} ` +
        `refresh=${row.refreshPassesDuring}/${row.refreshPassesAtSettle} commits=${row.reactCommitsDuring}/${row.reactCommitsAtSettle} recalc=${row.recalcStyleMs.toFixed(0)}ms long>50ms=${row.longTasks.length}`,
    );
  }
}
//#endregion ▶️Run

//#region 📝️Report
const numeric = (value, digits = 1) => (value === null || value === undefined ? "-" : Number(value).toFixed(digits));
const table = [
  "| example | gesture | events | paints | lod | paint median ms | paint p95 ms | paint max ms | paint dur median ms | misses | guest during | guest settle | viewport during/settle | refresh during/settle | commits during/settle | recalc ms | layout ms | script ms | long>50ms |",
  "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|",
  ...rows.map((row) =>
    row.error
      ? `| ${row.example} | ${row.gesture} | | | | | | | | | | | | | | | | ${row.error} |`
      : `| ${row.example} | ${row.gesture} | ${row.events} | ${row.paints} | ${row.lodChanges} | ${numeric(row.paintMedianMs)} | ${numeric(row.paintP95Ms)} | ${numeric(row.paintMaxMs)} | ${numeric(row.paintDurationMedianMs)} | ${row.paintMisses} | ${row.guestInvocationsDuring} | ${row.guestInvocationsAtSettle} | ${row.viewportPublicationsDuring}/${row.viewportPublicationsAtSettle} | ${row.refreshPassesDuring}/${row.refreshPassesAtSettle} | ${row.reactCommitsDuring}/${row.reactCommitsAtSettle} | ${numeric(row.recalcStyleMs, 0)} | ${numeric(row.layoutMs, 0)} | ${numeric(row.scriptMs, 0)} | ${row.longTasks.length} |`,
  ),
].join("\n");

const gestureRows = rows.filter((row) => !row.error);
const verdicts = gestureRows.map((row) => ({
  row: `${row.example} ${row.gesture}`,
  medianOk: row.paintMedianMs !== null && row.paintMedianMs <= PAINT_BUDGET_MEDIAN_MS,
  p95Ok: row.paintP95Ms !== null && row.paintP95Ms <= PAINT_BUDGET_P95_MS,
  noGuestHops: row.guestInvocationsDuring === 0,
  oneSettlePublication: row.viewportPublicationsAtSettle === 1,
}));
const green = verdicts.every((verdict) => verdict.medianOk && verdict.p95Ok && verdict.noGuestHops && verdict.oneSettlePublication);

writeFileSync(join(outDir, "scroll.json"), JSON.stringify(rows, null, 2));
writeFileSync(join(outDir, "scroll.md"), `${table}\n\n${JSON.stringify(verdicts, null, 2)}\n`);
writeFileSync(join(outDir, `console-${examples.at(-1)}.txt`), lines.join("\n"));
console.log(`\n${table}\n`);
console.log(`[DEBUG] flow-scroll gate ${green ? "GREEN" : "RED"} (median ≤ ${PAINT_BUDGET_MEDIAN_MS} ms, p95 ≤ ${PAINT_BUDGET_P95_MS} ms, 0 guest hops during, 1 viewport publication at settle)`);
console.log(`[DEBUG] flow-scroll wrote ${join(outDir, "scroll.json")}`);
await browser.close();
if (gate && !green) process.exit(1);
//#endregion 📝️Report
