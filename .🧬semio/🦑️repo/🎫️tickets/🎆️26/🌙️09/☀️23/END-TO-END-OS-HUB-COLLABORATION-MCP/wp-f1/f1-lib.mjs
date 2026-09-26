/** 🧰️ F1 — shared browser measurement helpers for the `s` idle-cost census and the interaction-latency probe.
 *
 * Main-thread busy is read from a CDP trace (every top-level task per renderer thread, wall `dur` and thread-CPU `tdur`),
 * frames from the same trace (`BeginMainThreadFrame`, `FireAnimationFrame`, `DrawFrame`, `Paint`), the requester of every
 * animation frame from an in-page `requestAnimationFrame` hook (call-site histogram), JS heap from `Performance.getMetrics`,
 * wasm linear memory of the main thread from a `WebAssembly` instantiate hook, and every dedicated worker's heap + backing
 * store (wasm memories are array-buffer backing stores) from `Runtime.getHeapUsage` through a non-flat auto-attach. */
import { chromium } from "playwright";

export const TRACE_CATEGORIES = ["toplevel", "devtools.timeline", "disabled-by-default-devtools.timeline.frame", "v8.execute", "blink.user_timing"].join(",");

/** 🪝️ In-page hooks: rAF call sites, timer call sites, wasm memories. */
export function installHooks() {
  performance.setResourceTimingBufferSize(100_000);
  const sites = new Map();
  const timerSites = new Map();
  const memories = [];
  let rafRequests = 0;
  let rafCallbacks = 0;
  const siteOf = () => {
    const stack = (new Error().stack ?? "").split("\n").slice(3, 6).map((line) => line.trim().replace(/^at /u, "").replace(/\?[^:)]*/u, "").replace(/https?:\/\/[^/]+\//u, "")).join(" < ");
    return stack.slice(0, 400);
  };
  const raf = window.requestAnimationFrame.bind(window);
  window.requestAnimationFrame = (callback) => {
    rafRequests += 1;
    const site = siteOf();
    sites.set(site, (sites.get(site) ?? 0) + 1);
    return raf((time) => {
      rafCallbacks += 1;
      callback(time);
    });
  };
  const wrapTimer = (name) => {
    const original = window[name].bind(window);
    window[name] = (handler, delay, ...rest) => {
      if (typeof handler === "function") {
        const site = `${name}(${delay ?? 0}) ${siteOf()}`;
        const wrapped = (...args) => {
          timerSites.set(site, (timerSites.get(site) ?? 0) + 1);
          return handler(...args);
        };
        return original(wrapped, delay, ...rest);
      }
      return original(handler, delay, ...rest);
    };
  };
  wrapTimer("setTimeout");
  wrapTimer("setInterval");
  const keep = (instance) => {
    for (const value of Object.values(instance?.exports ?? {})) if (value instanceof WebAssembly.Memory) memories.push(new WeakRef(value));
  };
  const instantiate = WebAssembly.instantiate;
  WebAssembly.instantiate = async (...args) => {
    const result = await instantiate(...args);
    keep(result instanceof WebAssembly.Instance ? result : result.instance);
    return result;
  };
  const streaming = WebAssembly.instantiateStreaming;
  if (streaming) {
    WebAssembly.instantiateStreaming = async (...args) => {
      const result = await streaming(...args);
      keep(result.instance);
      return result;
    };
  }
  const OriginalMemory = WebAssembly.Memory;
  WebAssembly.Memory = new Proxy(OriginalMemory, {
    construct(target, args) {
      const memory = Reflect.construct(target, args);
      memories.push(new WeakRef(memory));
      return memory;
    },
  });
  Object.defineProperty(window, "__f1", {
    value: {
      read() {
        const live = new Set();
        let bytes = 0;
        for (const ref of memories) {
          const memory = ref.deref();
          if (!memory || live.has(memory)) continue;
          live.add(memory);
          bytes += memory.buffer.byteLength;
        }
        return { rafRequests, rafCallbacks, wasmMemories: live.size, wasmBytes: bytes };
      },
      sites(reset) {
        const out = { raf: [...sites].sort((a, b) => b[1] - a[1]).slice(0, 8), timers: [...timerSites].sort((a, b) => b[1] - a[1]).slice(0, 8) };
        if (reset) {
          sites.clear();
          timerSites.clear();
        }
        return out;
      },
    },
  });
}

/** 🚀️ One browser + page with the hooks installed and a CDP session. */
export async function launch({ headless = true, width = 1440, height = 900 } = {}) {
  const browser = await chromium.launch({ headless, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
  const page = await browser.newPage({ viewport: { width, height } });
  page.setDefaultNavigationTimeout(300_000);
  await page.addInitScript(installHooks);
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Performance.enable", { timeDomain: "timeTicks" });
  const workers = await attachWorkers(cdp);
  return { browser, page, cdp, workers };
}

/** 👷️ Non-flat auto-attach to the page's dedicated workers: `heap()` answers every live worker's `Runtime.getHeapUsage`. */
export async function attachWorkers(cdp) {
  const sessions = new Map();
  const pending = new Map();
  let nextId = 1;
  cdp.on("Target.attachedToTarget", (event) => sessions.set(event.sessionId, event.targetInfo));
  cdp.on("Target.detachedFromTarget", (event) => sessions.delete(event.sessionId));
  cdp.on("Target.receivedMessageFromTarget", (event) => {
    const message = JSON.parse(event.message);
    const resolve = pending.get(message.id);
    if (resolve) {
      pending.delete(message.id);
      resolve(message);
    }
  });
  await cdp.send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: false, flatten: false });
  const call = (sessionId, method, params = {}) =>
    new Promise((resolve) => {
      const id = nextId++;
      pending.set(id, resolve);
      cdp.send("Target.sendMessageToTarget", { sessionId, message: JSON.stringify({ id, method, params }) }).catch(() => resolve({ error: "send failed" }));
      setTimeout(() => {
        if (pending.delete(id)) resolve({ error: "timeout" });
      }, 5_000);
    });
  return {
    count: () => sessions.size,
    async heap() {
      const rows = [];
      for (const [sessionId, info] of sessions) {
        if (info.type !== "worker") continue;
        const answer = await call(sessionId, "Runtime.getHeapUsage");
        if (answer.result) rows.push({ url: (info.url ?? "").split("/").slice(-2).join("/").slice(0, 80), used: answer.result.usedSize, total: answer.result.totalSize, backing: answer.result.backingStorageSize ?? null });
      }
      return rows;
    },
  };
}

export async function metrics(cdp) {
  const { metrics: rows } = await cdp.send("Performance.getMetrics");
  return Object.fromEntries(rows.map((row) => [row.name, row.value]));
}

/** 🎞️ Records one CDP trace for `ms` and reduces it to per-thread busy/cpu and frame counts. */
export async function traceWindow(cdp, ms, options = {}) {
  const chunks = [];
  const done = new Promise((resolve) => cdp.once("Tracing.tracingComplete", resolve));
  await cdp.send("Tracing.start", { categories: TRACE_CATEGORIES, transferMode: "ReturnAsStream", streamFormat: "json" });
  const t0 = Date.now();
  await new Promise((resolve) => setTimeout(resolve, ms));
  await cdp.send("Tracing.end");
  const { stream } = await done;
  for (;;) {
    const { data, eof, base64Encoded } = await cdp.send("IO.read", { handle: stream, size: 4 << 20 });
    chunks.push(base64Encoded ? Buffer.from(data, "base64").toString("utf8") : data);
    if (eof) break;
  }
  await cdp.send("IO.close", { handle: stream });
  const text = chunks.join("");
  const parsed = JSON.parse(text);
  const events = Array.isArray(parsed) ? parsed : parsed.traceEvents;
  const reduced = reduceTrace(events, Date.now() - t0);
  if (options.longEventsMs) reduced.longEvents = longEvents(events, options.longEventsMs);
  return reduced;
}

/** 🐢️ Every complete event of at least `ms` on any thread, with its thread name, in time order (the anatomy of a stall). */
export function longEvents(events, ms) {
  const names = new Map();
  for (const event of events) if (event.ph === "M" && event.name === "thread_name") names.set(`${event.pid}:${event.tid}`, event.args?.name ?? "?");
  const t0 = Math.min(...events.filter((event) => typeof event.ts === "number" && event.ph !== "M").map((event) => event.ts));
  return events
    .filter((event) => event.ph === "X" && (event.dur ?? 0) >= ms * 1000)
    .sort((a, b) => a.ts - b.ts)
    .slice(0, 80)
    .map((event) => `${((event.ts - t0) / 1000).toFixed(0)}ms +${((event.dur ?? 0) / 1000).toFixed(0)}ms ${names.get(`${event.pid}:${event.tid}`) ?? "?"} ${event.name} ${JSON.stringify(event.args?.data ?? event.args ?? {}).slice(0, 160)}`);
}

const FRAME_NAMES = ["BeginMainThreadFrame", "FireAnimationFrame", "DrawFrame", "Paint", "UpdateLayoutTree", "Layout", "TimerFire", "FunctionCall", "RequestAnimationFrame", "BeginFrame", "Commit", "ParseHTML", "GPUTask", "RunMicrotasks", "EventDispatch", "MajorGC", "MinorGC", "V8.GCScavenger", "V8.GC_MC_BACKGROUND_MARKING"];

export function reduceTrace(events, wallMs) {
  const names = new Map();
  for (const event of events) if (event.ph === "M" && event.name === "thread_name") names.set(`${event.pid}:${event.tid}`, event.args?.name ?? "?");
  const processNames = new Map();
  for (const event of events) if (event.ph === "M" && event.name === "process_name") processNames.set(event.pid, event.args?.name ?? "?");
  let tsMin = Infinity;
  let tsMax = -Infinity;
  for (const event of events) {
    if (typeof event.ts !== "number" || event.ph === "M") continue;
    tsMin = Math.min(tsMin, event.ts);
    tsMax = Math.max(tsMax, event.ts + (event.dur ?? 0));
  }
  const spanUs = Math.max(1, tsMax - tsMin);
  const threads = new Map();
  const threadOf = (event) => {
    const key = `${event.pid}:${event.tid}`;
    let row = threads.get(key);
    if (!row) {
      row = { key, name: names.get(key) ?? "?", process: processNames.get(event.pid) ?? "?", intervals: [], cpuUs: 0, counts: {}, sources: new Map() };
      threads.set(key, row);
    }
    return row;
  };
  const opened = new Map();
  for (const event of events) {
    if (event.ph === "M") continue;
    const row = threadOf(event);
    if (FRAME_NAMES.includes(event.name) && (event.ph === "X" || event.ph === "B" || event.ph === "I" || event.ph === "i" || event.ph === "n")) row.counts[event.name] = (row.counts[event.name] ?? 0) + 1;
    const top = event.name === "ThreadControllerImpl::RunTask" || event.name === "RunTask" || event.name === "ThreadPool_RunTask";
    if (!top) continue;
    const source = `${event.args?.src_func ?? event.args?.data?.src_func ?? "?"}@${String(event.args?.src_file ?? event.args?.data?.src_file ?? "?").split("/").slice(-2).join("/")}`;
    const tally = row.sources.get(source) ?? { n: 0, us: 0 };
    tally.n += 1;
    tally.us += event.dur ?? 0;
    row.sources.set(source, tally);
    if (event.ph === "X") {
      row.intervals.push([event.ts, event.ts + (event.dur ?? 0)]);
      row.cpuUs += event.tdur ?? 0;
    } else if (event.ph === "B") opened.set(row.key, event);
    else if (event.ph === "E" && opened.has(row.key)) {
      const begin = opened.get(row.key);
      row.intervals.push([begin.ts, event.ts]);
      if (typeof begin.tts === "number" && typeof event.tts === "number") row.cpuUs += event.tts - begin.tts;
      opened.delete(row.key);
    }
  }
  const out = [];
  for (const row of threads.values()) {
    row.intervals.sort((a, b) => a[0] - b[0]);
    let busy = 0;
    let end = -Infinity;
    for (const [s, e] of row.intervals) {
      if (s >= end) {
        busy += e - s;
        end = e;
      } else if (e > end) {
        busy += e - end;
        end = e;
      }
    }
    if (busy === 0 && Object.keys(row.counts).length === 0) continue;
    out.push({ thread: row.name, process: row.process, key: row.key, busyMs: Math.round(busy / 1000), busyPct: +((100 * busy) / spanUs).toFixed(2), cpuMs: Math.round(row.cpuUs / 1000), cpuPct: +((100 * row.cpuUs) / spanUs).toFixed(2), tasks: row.intervals.length, counts: row.counts, sources: [...row.sources].sort((a, b) => b[1].us - a[1].us).slice(0, 8).map(([name, tally]) => `${tally.n}× ${Math.round(tally.us / 1000)}ms ${name}`) });
  }
  out.sort((a, b) => b.busyMs - a.busyMs);
  return { spanMs: Math.round(spanUs / 1000), wallMs, events: events.length, threads: out };
}

/** 🧮️ The renderer main thread of the traced page + the sum over its dedicated workers + the compositor/viz frame counts. */
export function summarize(trace, seconds) {
  const main = trace.threads.filter((row) => row.thread === "CrRendererMain").sort((a, b) => b.tasks - a.tasks)[0] ?? null;
  const rendererPid = main?.key.split(":")[0];
  const workers = trace.threads.filter((row) => row.key.split(":")[0] === rendererPid && /DedicatedWorker/iu.test(row.thread));
  const compositor = trace.threads.find((row) => row.key.split(":")[0] === rendererPid && /Compositor/u.test(row.thread) && !/Tile|Raster/u.test(row.thread));
  const viz = trace.threads.filter((row) => /VizCompositor/u.test(row.thread));
  const per = (value) => +(value / seconds).toFixed(2);
  const c = main?.counts ?? {};
  return {
    mainBusyPct: main?.busyPct ?? null,
    mainCpuPct: main?.cpuPct ?? null,
    mainTasks: main?.tasks ?? 0,
    mainFramesPerSec: per(c.BeginMainThreadFrame ?? 0),
    rafPerSec: per(c.FireAnimationFrame ?? 0),
    paintsPerSec: per(c.Paint ?? 0),
    styleRecalcsPerSec: per(c.UpdateLayoutTree ?? 0),
    layoutsPerSec: per(c.Layout ?? 0),
    timersPerSec: per(c.TimerFire ?? 0),
    functionCallsPerSec: per(c.FunctionCall ?? 0),
    compositorDrawsPerSec: per((compositor?.counts.DrawFrame ?? 0) + viz.reduce((sum, row) => sum + (row.counts.DrawFrame ?? 0), 0)),
    workers: workers.length,
    workersBusyPct: +workers.reduce((sum, row) => sum + row.busyPct, 0).toFixed(2),
    workersCpuPct: +workers.reduce((sum, row) => sum + row.cpuPct, 0).toFixed(2),
    busiestWorker: workers.sort((a, b) => b.busyPct - a.busyPct)[0]?.busyPct ?? 0,
    mainSources: main?.sources ?? [],
    gpuMain: trace.threads.find((row) => row.thread === "CrGpuMain")?.busyPct ?? null,
  };
}

/** 🔥️ Main-thread CPU profile for `ms`, reduced to the top inclusive JS frames. */
export async function profileWindow(cdp, ms) {
  await cdp.send("Profiler.enable");
  await cdp.send("Profiler.setSamplingInterval", { interval: 500 });
  await cdp.send("Profiler.start");
  await new Promise((resolve) => setTimeout(resolve, ms));
  const { profile } = await cdp.send("Profiler.stop");
  await cdp.send("Profiler.disable");
  const byId = new Map(profile.nodes.map((node) => [node.id, node]));
  const parent = new Map();
  for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
  const deltas = profile.timeDeltas ?? [];
  let idle = 0;
  let total = 0;
  const inclusive = new Map();
  const self = new Map();
  profile.samples.forEach((id, index) => {
    const delta = deltas[index] ?? 0;
    total += delta;
    const leaf = byId.get(id).callFrame;
    if (leaf.functionName === "(idle)") idle += delta;
    const leafKey = `${leaf.functionName || "(anon)"} ${decodeURIComponent(leaf.url.split("/").slice(-3).join("/")).slice(0, 80)}:${leaf.lineNumber + 1}`;
    self.set(leafKey, (self.get(leafKey) ?? 0) + delta);
    const seen = new Set();
    for (let cursor = id; cursor !== undefined; cursor = parent.get(cursor)) {
      const frame = byId.get(cursor).callFrame;
      if (!frame.url) continue;
      const key = `${frame.functionName || "(anon)"} ${decodeURIComponent(frame.url.split("/").slice(-3).join("/")).slice(0, 80)}:${frame.lineNumber + 1}`;
      if (seen.has(key)) continue;
      seen.add(key);
      inclusive.set(key, (inclusive.get(key) ?? 0) + delta);
    }
  });
  const top = (map, n) => [...map].sort((a, b) => b[1] - a[1]).slice(0, n).map(([key, value]) => `${Math.round(value / 1000)}ms ${key}`);
  return { totalMs: Math.round(total / 1000), idleMs: Math.round(idle / 1000), busyPct: Math.round(100 * (1 - idle / Math.max(1, total))), inclusive: top(inclusive, 40), self: top(self, 25) };
}

export const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

export const kindOf = (appId) => /^s\.[^.]+\.([^@]+)@/u.exec(appId)?.[1] ?? appId;
export const windowIdsOf = (page) => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id) => typeof id === "string"));

/** ⏳️ Waits until the main thread's task time stays under 5 % for 3 consecutive seconds (at most `budgetMs`). */
export async function settleMain(cdp, budgetMs) {
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

/** 🚪️ Boots `s` on a launched page: beacon, introduction dismissed, main thread settled. */
export async function bootShell(page, cdp, baseUrl) {
  const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");
  await page.goto(baseUrl, { waitUntil: "commit" });
  const beacon = await sweep.awaitBeacon(page, Date.now() + 300_000);
  await sweep.dismissIntroduction(page);
  await page.keyboard.press("Escape").catch(() => undefined);
  await settleMain(cdp, 20_000);
  return beacon;
}

/** 🎛️ Opens one program from the palette by the shell chord; answers the fresh window ids once their bodies render. */
export async function openProgramByPalette(page, program) {
  const before = await windowIdsOf(page);
  await page.evaluate(() => {
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    document.body.focus();
  });
  await page.keyboard.press(process.platform === "darwin" ? "Meta+p" : "Control+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 }).catch(() => undefined);
  if ((await input.count()) === 0) return { windowIds: [], detail: "command palette never opened" };
  await input.fill(kindOf(program.appId));
  await sleep(1_200);
  let item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${program.pluginId}.${program.appId}"]`).first();
  await item.waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
  if ((await item.count()) === 0) item = page.locator(`[data-slot="command-item"][data-command-item-id="spawn.${program.pluginId}"]`).first();
  if ((await item.count()) === 0) {
    await page.keyboard.press("Escape");
    return { windowIds: [], detail: "no palette row" };
  }
  await item.click({ timeout: 8_000 }).catch(() => item.click({ force: true }).catch(() => undefined));
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    const fresh = (await windowIdsOf(page)).filter((id) => !before.includes(id));
    if (fresh.length > 0) {
      await sleep(3_000);
      return { windowIds: (await windowIdsOf(page)).filter((id) => !before.includes(id)), detail: null };
    }
    await sleep(250);
  }
  return { windowIds: [], detail: "no new window" };
}

export const quantile = (values, q) => {
  if (values.length === 0) return null;
  const sorted = [...values].sort((a, b) => a - b);
  return +sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))].toFixed(1);
};
