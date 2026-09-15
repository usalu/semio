/** ⏱️ Per-hop cost probe for the procedural 3d REACT door: runs every bundled example on one page and
 * writes the stage breakdown of each `flowEvalTick` hop — guest command dispatch (encode) → worker
 * crossing (channel) → reply decode → ui refresh (guest render / slot resolve / React apply) → React
 * commit → the arming of the next tick.
 *
 * 🩺️ Stage times are read straight out of the live isolate as User Timing `measure` entries published
 * by `🔨️modules/⏱️trace/🟦️.ts` (`semio.hop.*`) — no console parsing. The worker's own busy time and
 * the page's style-recalc time are read over CDP `Performance.getMetrics`, which is what separates
 * "the guest computed" from "the message crossed", and what re-verifies the `RecalcStyleDuration`
 * ceiling (`📓️react-perf-ceilings-audit-2026-09-14.md` §3 items 1 and 6).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d SEMIO_PROBE_OUT=react-hop/before bun 🐍️react-hop-cost-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6021/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-hop/cost");
const stepWait = Number(process.env.SEMIO_PROBE_STEP_WAIT ?? 70);
const debugPort = Number(process.env.SEMIO_PROBE_CDP_PORT ?? 9361);
mkdirSync(outDir, { recursive: true });

/** 🧱️ The declared stage vocabulary — the twin of `🔨️modules/⏱️trace/🧫️fixtures/🪃️hop-stages/🔣️.json`. */
const STAGES = ["encode", "channel", "decode", "invoke", "refresh.guest", "refresh.turn", "refresh.project", "refresh.slots", "refresh.apply", "refresh", "commit", "arm", "mesh.decode", "turn.accept", "turn.decide", "turn.yield", "worker.turn", "worker.receive", "worker.decode", "worker.guest", "worker.reply"];
const MEASURE_PREFIX = "semio.hop.";

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal", `--remote-debugging-port=${debugPort}`] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 900)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 900)}`));

const pageSession = await page.context().newCDPSession(page);
await pageSession.send("Performance.enable");
/** 📊️ The page's own cumulative counters — `RecalcStyleDuration` is the one the 09-12 CSS storm fix is gated on. */
const pageMetrics = async () => Object.fromEntries((await pageSession.send("Performance.getMetrics")).metrics.map((metric) => [metric.name, metric.value]));

//#region 🧵️WorkerMetrics
/** 🧵️ CDP over a raw socket, because the shard workers are their own targets: their cumulative
 * `ScriptDuration` is the guest's compute, and `channel − ScriptDuration` is the message crossing. */
const workerSockets = new Map();
const cdpSend = (socket, method, params) =>
  new Promise((resolve) => {
    const id = Math.floor(Math.random() * 1e6);
    const onMessage = (event) => {
      const payload = JSON.parse(event.data);
      if (payload.id === id) {
        socket.removeEventListener("message", onMessage);
        resolve(payload.result ?? null);
      }
    };
    socket.addEventListener("message", onMessage);
    socket.send(JSON.stringify({ id, method, params }));
    setTimeout(() => resolve(null), 8000);
  });
const refreshWorkerSockets = async () => {
  let targets = [];
  try {
    targets = await fetch(`http://127.0.0.1:${debugPort}/json/list`).then((response) => response.json());
  } catch {
    return;
  }
  for (const target of targets.filter((entry) => entry.type === "worker" && decodeURIComponent(entry.url).includes("shard"))) {
    if (workerSockets.has(target.id)) continue;
    try {
      const socket = new WebSocket(target.webSocketDebuggerUrl);
      await new Promise((resolve, reject) => {
        socket.onopen = resolve;
        socket.onerror = reject;
      });
      await cdpSend(socket, "Performance.enable", {});
      workerSockets.set(target.id, socket);
    } catch {
      /* a worker that went away between listing and attaching is simply not measured */
    }
  }
};
/** 🧵️ Summed cumulative worker metrics across every live shard worker. */
const workerMetrics = async () => {
  await refreshWorkerSockets();
  const totals = { ScriptDuration: 0, TaskDuration: 0, workers: 0 };
  for (const [id, socket] of workerSockets) {
    const result = await cdpSend(socket, "Performance.getMetrics", {});
    if (!result) {
      workerSockets.delete(id);
      continue;
    }
    totals.workers += 1;
    for (const metric of result.metrics) {
      if (metric.name === "ScriptDuration") totals.ScriptDuration += metric.value;
      if (metric.name === "TaskDuration") totals.TaskDuration += metric.value;
    }
  }
  return totals;
};
//#endregion 🧵️WorkerMetrics

//#region 📏️Spans
const clearSpans = () =>
  page.evaluate(
    (names) => {
      for (const name of names) performance.clearMeasures(name);
    },
    STAGES.map((stage) => `${MEASURE_PREFIX}${stage}`),
  );
const readSpans = () =>
  page.evaluate(
    (prefix) =>
      performance
        .getEntriesByType("measure")
        .filter((entry) => entry.name.startsWith(prefix))
        .map((entry) => ({ stage: entry.name.slice(prefix.length), startMs: entry.startTime, durationMs: entry.duration, detail: entry.detail ?? null })),
    MEASURE_PREFIX,
  );
/** 🧮️ The breakdown of one step: per-stage totals, the hop count, and the per-hop mean of each stage. */
const breakdown = (spans) => {
  const invocations = spans.filter((span) => span.stage === "invoke" && span.detail?.actionId === "flowEvalTick").sort((left, right) => left.startMs - right.startMs);
  const hops = invocations.length;
  const totals = {};
  for (const span of spans) {
    const entry = (totals[span.stage] ??= { count: 0, totalMs: 0 });
    entry.count += 1;
    entry.totalMs += span.durationMs;
  }
  const gaps = [];
  for (let index = 1; index < invocations.length; index += 1) {
    const previous = invocations[index - 1];
    gaps.push(Math.max(0, invocations[index].startMs - (previous.startMs + previous.durationMs)));
  }
  const gapTotal = gaps.reduce((sum, value) => sum + value, 0);
  /** 🧵️ The worker crossings this run actually made. `worker.*` spans are measured INSIDE
   * `🟨️shard-worker.js` on the epoch clock both realms share and replayed onto this timeline by
   * `ShardClient` — so `crossings` is the number of worker round trips one hop really costs, which is
   * the term the 09-14 lane could only infer. */
  const crossingSpans = spans.filter((span) => span.stage === "worker.turn");
  const crossings = crossingSpans.length;
  /** 🧾️ WHAT the crossings are. Each `worker.turn` names the event kinds the host posted and how many
   * ui patches the guest published for them, so a hop's round-trip count is attributed to the ladder
   * that produced it instead of merely counted. */
  const crossingKinds = {};
  let crossingPatches = 0;
  /** 🧬️ The two halves of `worker.reply`: the structured CLONE the worker's own `postMessage`
   * performs synchronously (carried on the NEXT reply, since a worker cannot amend a message it has
   * already posted) versus the main thread's pickup latency — "the payload was expensive" and "the
   * page was busy" are different defects with different owners. */
  let replyCloneMs = 0;
  let replyCloneSamples = 0;
  for (const span of spans.filter((entry) => entry.stage === "worker.turn")) {
    const clone = Number(span.detail?.replyCloneMs ?? -1);
    if (clone >= 0) { replyCloneMs += clone; replyCloneSamples += 1; }
  }
  /** 🚚️ How many guest turns the WORKER ran inside one crossing, and why it stopped — the reading
   * that makes "the worker owns the MoreWork drive" a measurement rather than an inference from
   * `worker.guest`'s mean (📓️worker-more-work-drive-2026-09-15.md). */
  const driveStops = {};
  let drivePolls = 0;
  let driveSamples = 0;
  for (const span of crossingSpans) {
    const key = String(span.detail?.eventKinds ?? "").length ? String(span.detail.eventKinds) : "(none)";
    const entry = (crossingKinds[key] ??= { count: 0, totalMs: 0, patches: 0 });
    entry.count += 1;
    entry.totalMs += span.durationMs;
    entry.patches += Number(span.detail?.patches ?? 0);
    crossingPatches += Number(span.detail?.patches ?? 0);
    const polls = Number(span.detail?.drivePolls ?? -1);
    if (polls >= 0) { drivePolls += polls; driveSamples += 1; }
    const stopped = String(span.detail?.driveStopped ?? "");
    if (stopped.length) driveStops[stopped] = (driveStops[stopped] ?? 0) + 1;
  }
  /** 🔎️ What each refresh ASKED for — a hop that pays two full guest re-renders is a different
   * defect from a hop that pays one full and one narrow, and only the scope says which. */
  const refreshScopes = {};
  for (const span of spans.filter((entry) => entry.stage === "refresh")) {
    const key = String(span.detail?.scope ?? "?");
    refreshScopes[key] = (refreshScopes[key] ?? 0) + 1;
  }
  return {
    hops,
    crossings,
    crossingKinds,
    crossingPatches,
    driveStops,
    drivePolls,
    driveSamples,
    replyCloneMs,
    replyCloneSamples,
    refreshScopes,
    tickTotalMs: invocations.reduce((sum, span) => sum + span.durationMs, 0),
    interHopGapTotalMs: gapTotal,
    interHopGapMeanMs: gaps.length ? gapTotal / gaps.length : 0,
    perHopMs: Object.fromEntries(STAGES.map((stage) => [stage, hops ? (totals[stage]?.totalMs ?? 0) / hops : 0])),
    perCrossingMs: Object.fromEntries(STAGES.map((stage) => [stage, crossings ? (totals[stage]?.totalMs ?? 0) / crossings : 0])),
    totals,
  };
};
//#endregion 📏️Spans

//#region ⚖️Convergence
const snap = () =>
  page.evaluate(() => {
    const parse = (text) => {
      try {
        return JSON.parse(text);
      } catch {
        return null;
      }
    };
    const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((element) => {
      let meshes = 0;
      try {
        const value = JSON.parse(element.getAttribute("data-meshes-json") ?? "[]");
        meshes = Array.isArray(value) ? value.length : 0;
      } catch {}
      const status = parse(element.getAttribute("data-status-json"));
      return { surfaceId: element.getAttribute("data-surface-id"), meshes, phase: status?.phase, ratio: status?.progress?.ratio, computing: status?.computing ?? null, fault: status?.fault?.code ?? null, meshesLen: status?.debug?.meshesLen ?? null };
    });
    const combo = document.querySelector('[role="combobox"]');
    return { hosts, meshes: hosts.reduce((sum, host) => sum + host.meshes, 0), example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null };
  });
const settled = (host) => Boolean(host) && host.phase === "idle" && host.ratio === 1 && host.computing !== true && !host.fault;
const converged = (state) => {
  const previews = state.hosts.filter((host) => host.surfaceId && host.surfaceId.endsWith("-preview"));
  return previews.length > 0 && previews.every(settled);
};
//#endregion ⚖️Convergence

const results = [];
const runStep = async (label, seconds) => {
  await clearSpans();
  const beforePage = await pageMetrics();
  const beforeWorker = await workerMetrics();
  const startedAt = Date.now();
  let state = null;
  let stable = 0;
  for (let index = 0; index < seconds; index += 1) {
    await page.waitForTimeout(1000);
    state = await snap();
    if (converged(state)) {
      stable += 1;
      if (stable >= 3) break;
    } else stable = 0;
  }
  const spans = await readSpans();
  const afterPage = await pageMetrics();
  const afterWorker = await workerMetrics();
  const row = {
    label,
    seconds: (Date.now() - startedAt) / 1000,
    converged: converged(state),
    example: state?.example ?? null,
    meshes: state?.meshes ?? 0,
    meshesLen: state?.hosts.find((host) => host.meshesLen !== null)?.meshesLen ?? null,
    ...breakdown(spans),
    workerScriptMs: (afterWorker.ScriptDuration - beforeWorker.ScriptDuration) * 1000,
    workerTaskMs: (afterWorker.TaskDuration - beforeWorker.TaskDuration) * 1000,
    workers: afterWorker.workers,
    pageScriptMs: ((afterPage.ScriptDuration ?? 0) - (beforePage.ScriptDuration ?? 0)) * 1000,
    pageTaskMs: ((afterPage.TaskDuration ?? 0) - (beforePage.TaskDuration ?? 0)) * 1000,
    recalcStyleMs: ((afterPage.RecalcStyleDuration ?? 0) - (beforePage.RecalcStyleDuration ?? 0)) * 1000,
    layoutMs: ((afterPage.LayoutDuration ?? 0) - (beforePage.LayoutDuration ?? 0)) * 1000,
  };
  results.push(row);
  writeFileSync(join(outDir, "hops.json"), JSON.stringify(results, null, 2));
  if (process.env.SEMIO_PROBE_RAW_SPANS === "1") writeFileSync(join(outDir, `spans-${label.replace(/[^a-z0-9]+/gi, "-")}.json`), JSON.stringify(spans, null, 2));
  console.log(
    `[DEBUG] ${label}: converged=${row.converged} ${row.seconds.toFixed(1)}s hops=${row.hops} meshes=${row.meshes} ` +
      `tick=${row.tickTotalMs.toFixed(0)}ms gap=${row.interHopGapTotalMs.toFixed(0)}ms crossings=${row.crossings} guest=${(row.totals["worker.guest"]?.totalMs ?? 0).toFixed(0)}ms recalc=${row.recalcStyleMs.toFixed(0)}ms`,
  );
  return state;
};

await page.goto(url, { waitUntil: "domcontentloaded" });
await runStep("boot", stepWait + 40);

const listOptions = async () => {
  const combo = page.locator('[role="combobox"]').first();
  if (!(await combo.count())) return [];
  await combo.click({ timeout: 6000 });
  await page.waitForTimeout(400);
  const texts = await page.locator('[role="option"]').allInnerTexts();
  await page.keyboard.press("Escape");
  await page.waitForTimeout(200);
  return texts.map((text) => text.replace(/\s+/g, " ").trim()).filter(Boolean);
};
const pick = async (text) => {
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 6000 });
  await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: text }).first().click({ timeout: 6000 });
};

for (const text of await listOptions()) {
  // 🧯 A picker click that reports "waiting for scheduled navigations" is a flake of the select, not a
  // finding: retry once, then move on rather than losing the whole run's table.
  try {
    await pick(text);
  } catch {
    await page.keyboard.press("Escape");
    await page.waitForTimeout(600);
    try {
      await pick(text);
    } catch {
      lines.push(`${Date.now() - t0} probe could not pick ${text}`);
      continue;
    }
  }
  await runStep(`edit:${text}`, stepWait);
}

//#region 📝️Report
const cell = (value, width) => String(value).padStart(width);
const stageColumns = ["channel", "refresh.turn", "refresh.project", "commit", "arm", "worker.turn", "worker.receive", "worker.decode", "worker.guest", "worker.reply"];
const table = [
  `| step | s | ok | hops | crossings | meshes | tick ms/hop | gap ms/hop | ${stageColumns.map((stage) => `${stage} ms/hop`).join(" | ")} | worker cpu ms | recalc ms |`,
  `|---|---:|---|---:|---:|---:|---:|---:|${stageColumns.map(() => "---:").join("|")}|---:|---:|`,
  ...results.map((row) =>
    [
      row.label,
      row.seconds.toFixed(1),
      row.converged ? "yes" : "NO",
      row.hops,
      row.crossings,
      row.meshes,
      row.hops ? (row.tickTotalMs / row.hops).toFixed(0) : "-",
      row.hops > 1 ? row.interHopGapMeanMs.toFixed(0) : "-",
      ...stageColumns.map((stage) => (row.hops ? row.perHopMs[stage].toFixed(0) : "-")),
      row.workerScriptMs.toFixed(0),
      row.recalcStyleMs.toFixed(0),
    ]
      .map((value) => cell(value, 1))
      .join(" | ")
      .replace(/^/, "| ")
      .concat(" |"),
  ),
];
const totalSeconds = results.reduce((sum, row) => sum + row.seconds, 0);
const totalHops = results.reduce((sum, row) => sum + row.hops, 0);
const totalRecalc = results.reduce((sum, row) => sum + row.recalcStyleMs, 0);
const totalCrossings = results.reduce((sum, row) => sum + row.crossings, 0);
table.push("", `totals: ${results.length} steps, ${totalSeconds.toFixed(1)} s, ${totalHops} hops, ${totalCrossings} worker crossings, converged ${results.filter((row) => row.converged).length}/${results.length}`);
const stageTotal = (stage) => results.reduce((sum, row) => sum + (row.totals[stage]?.totalMs ?? 0), 0);
const stageCount = (stage) => results.reduce((sum, row) => sum + (row.totals[stage]?.count ?? 0), 0);
table.push("", "| stage | count | total ms | mean ms | per hop ms |", "|---|---:|---:|---:|---:|");
for (const stage of STAGES) {
  const count = stageCount(stage);
  if (!count) continue;
  const total = stageTotal(stage);
  table.push(`| ${stage} | ${count} | ${total.toFixed(0)} | ${(total / count).toFixed(1)} | ${totalHops ? (total / totalHops).toFixed(0) : "-"} |`);
}
const kindTotals = {};
for (const row of results) for (const [kind, entry] of Object.entries(row.crossingKinds)) {
  const total = (kindTotals[kind] ??= { count: 0, totalMs: 0, patches: 0 });
  total.count += entry.count; total.totalMs += entry.totalMs; total.patches += entry.patches;
}
table.push("", "| worker crossing (posted event kinds) | count | per hop | total ms | mean ms | ui patches |", "|---|---:|---:|---:|---:|---:|");
for (const [kind, entry] of Object.entries(kindTotals).sort((left, right) => right[1].totalMs - left[1].totalMs)) {
  table.push(`| ${kind} | ${entry.count} | ${totalHops ? (entry.count / totalHops).toFixed(1) : "-"} | ${entry.totalMs.toFixed(0)} | ${(entry.totalMs / entry.count).toFixed(1)} | ${entry.patches} |`);
}
const driveStopTotals = {};
for (const row of results) for (const [stop, count] of Object.entries(row.driveStops ?? {})) driveStopTotals[stop] = (driveStopTotals[stop] ?? 0) + count;
const totalDrivePolls = results.reduce((sum, row) => sum + (row.drivePolls ?? 0), 0);
const totalDriveSamples = results.reduce((sum, row) => sum + (row.driveSamples ?? 0), 0);
if (totalDriveSamples) table.push("", `worker MoreWork drive: ${totalDrivePolls} guest turns absorbed over ${totalDriveSamples} crossings (${(totalDrivePolls / totalDriveSamples).toFixed(2)} per crossing, ${totalHops ? (totalDrivePolls / totalHops).toFixed(1) : "-"} per hop); stops ${Object.entries(driveStopTotals).sort((left, right) => right[1] - left[1]).map(([stop, count]) => `${stop}=${count}`).join(" ") || "-"}`);
const totalClone = results.reduce((sum, row) => sum + row.replyCloneMs, 0);
const totalCloneSamples = results.reduce((sum, row) => sum + row.replyCloneSamples, 0);
const totalReply = results.reduce((sum, row) => sum + (row.totals["worker.reply"]?.totalMs ?? 0), 0);
table.push("", `worker.reply split: structured clone ${totalClone.toFixed(0)} ms over ${totalCloneSamples} samples (${totalCloneSamples ? (totalClone / totalCloneSamples).toFixed(2) : "-"} ms each), main-thread pickup ${(totalReply - totalClone).toFixed(0)} ms of ${totalReply.toFixed(0)} ms total`);
table.push("", `worker crossings per hop: ${totalHops ? (totalCrossings / totalHops).toFixed(2) : "-"}; hop wall: ${totalHops ? ((totalSeconds * 1000) / totalHops).toFixed(0) : "-"} ms`);
table.push(`RecalcStyleDuration over the run: ${totalRecalc.toFixed(0)} ms = ${((totalRecalc / (totalSeconds * 1000)) * 100).toFixed(2)}% of wall (gate: under 5%)`);
writeFileSync(join(outDir, "hops.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "table.md"), table.join("\n"));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(table.join("\n"));
//#endregion 📝️Report

for (const socket of workerSockets.values()) socket.close();
await browser.close();
