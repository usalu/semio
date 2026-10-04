/** ⏱️ Ticket tool of work package P1: measures the home overview of the architecture quiz while the pointer sweeps across it, once per pets configuration (off, still, calm, lively), and tells how much of each frame the pets cost.
 *
 * Per configuration and repetition a clean run counts animation frames in the page, long tasks and long animation
 * frames (with the forced style and layout of each script, by owner), and takes the renderer's own counters
 * (`Performance.getMetrics`) before and after a sweep of `--seconds` seconds: a time-based Lissajous path over the
 * whole viewport driven by `Input.dispatchMouseEvent`, so every run follows the same path whatever its frame rate.
 * With `--probe` one more run per configuration runs instrumented in a context of its own: a trace with
 * `devtools.timeline` categories (main-thread self time by kind, forced layouts and style recalculations inside
 * scripts by owner, raster and GPU time, drawn frames), a sampled CPU profile (time inside any frame of the pets'
 * modules) and wrappers that count, by owner, layout reads, computed-style reads, animation-frame requests, attribute
 * and style writes, tree walks and the calls of mutation and resize observers.
 *
 * Owners: `pets` is every module under a `🐾️pets` folder (development build) or a chunk of `--dist` that holds the
 * pet layer (release build); the rest is the page (`ui`, `quiz`, `lib`, `other` in a development build).
 *
 * Usage (from the repository root):
 *   node ".../p1_measure.mjs" --url http://127.0.0.1:6245/ --label dev-before [--modes off,still,calm,lively]
 *        [--seconds 10] [--trace-seconds 5] [--repeat 2] [--browser shell|gpu] [--probe] [--probe-only] [--keep-trace]
 *        [--dist <release build dir>] [--out <dir>]
 * `shell` is Playwright's headless shell (software compositing through SwiftShader, as the site's Playwright projects
 * run); `gpu` is Chromium's new headless mode, which composites on the machine's graphics card.
 * Writes `<out>/measure-<label>.jsonl` (one line per run) and prints a summary table.
 *
 * @see https://chromedevtools.github.io/devtools-protocol/tot/Performance/#method-getMetrics
 * @see https://developer.chrome.com/docs/web-platform/long-animation-frames
 */
import { chromium } from "playwright";
import { appendFileSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const option = (name, fallback) => {
  const at = process.argv.indexOf(`--${name}`);
  return at < 0 ? fallback : process.argv[at + 1];
};
const flag = (name) => process.argv.includes(`--${name}`);
const ticket = dirname(fileURLToPath(import.meta.url));
const url = option("url", "http://127.0.0.1:6245/");
const label = option("label", "run");
const modes = option("modes", "off,still,calm,lively").split(",");
const seconds = Number(option("seconds", "10"));
const traceSeconds = Number(option("trace-seconds", "5"));
const repeat = Number(option("repeat", "2"));
const kind = option("browser", "shell");
const probe = flag("probe");
const only = flag("probe-only");
const dist = option("dist", null);
const out = option("out", join(ticket, "🗑️generated", "p1"));
const width = 1440;
const height = 900;
mkdirSync(out, { recursive: true });
const sink = join(out, `measure-${label}.jsonl`);

const petChunks = dist === null ? [] : readdirSync(join(dist, "assets")).filter((name) => name.endsWith(".js") && readFileSync(join(dist, "assets", name), "utf8").includes("pet-layer"));

/** 🏷️ The owner of a script URL. */
function ownerOf(raw) {
  if (!raw) return "";
  let text = raw;
  try {
    text = decodeURI(raw);
  } catch {
    text = raw;
  }
  if (text.includes("🐾️pets") || petChunks.some((name) => text.includes(name) || raw.includes(encodeURI(name)))) return "pets";
  if (petChunks.length > 0) return text.startsWith("http") ? "page" : "";
  if (text.includes("🖱️ui")) return "ui";
  if (text.includes("❓️quiz")) return "quiz";
  if (text.includes("node_modules") || text.includes("/@vite/") || text.includes("/@react-refresh")) return "lib";
  return text.startsWith("http") ? "other" : "";
}

/** 🧪️ In the page, before its scripts: frame counter, long tasks and long animation frames; with `probe`, wrappers that count DOM reads and writes by the owner of their caller. */
function instrument({ probe, petChunks }) {
  const owner = (stack) => {
    const lines = String(stack).split("\n").slice(2);
    for (const line of lines) {
      const found = line.match(/(https?:\/\/[^\s)]+)/u);
      if (found === null) continue;
      let text = found[1];
      try {
        text = decodeURI(text);
      } catch {
        text = found[1];
      }
      if (text.includes("🐾️pets") || petChunks.some((name) => text.includes(name) || found[1].includes(encodeURI(name)))) return "pets";
      if (petChunks.length > 0) return "page";
      if (text.includes("🖱️ui")) return "ui";
      if (text.includes("❓️quiz")) return "quiz";
      if (text.includes("node_modules") || text.includes("/@vite/")) return "lib";
      return "other";
    }
    return "native";
  };
  const state = { on: false, frames: [], longtasks: [], loafs: [], counts: {} };
  const count = (what, who, amount = 1) => {
    if (!state.on) return;
    const bucket = (state.counts[what] ??= {});
    bucket[who] = (bucket[who] ?? 0) + amount;
  };
  const loop = (time) => {
    if (!state.on) return;
    state.frames.push(time);
    requestAnimationFrame(loop);
  };
  try {
    new PerformanceObserver((list) => {
      if (state.on) for (const entry of list.getEntries()) state.longtasks.push(entry.duration);
    }).observe({ type: "longtask" });
  } catch {}
  try {
    new PerformanceObserver((list) => {
      if (!state.on) return;
      for (const entry of list.getEntries())
        state.loafs.push({
          duration: entry.duration,
          blocking: entry.blockingDuration,
          render: entry.renderStart > 0 ? entry.startTime + entry.duration - entry.renderStart : 0,
          styleAndLayout: entry.styleAndLayoutStart > 0 ? entry.startTime + entry.duration - entry.styleAndLayoutStart : 0,
          scripts: entry.scripts.map((script) => ({ url: script.sourceURL, fn: script.sourceFunctionName, invoker: script.invoker, duration: script.duration, forced: script.forcedStyleAndLayoutDuration })),
        });
    }).observe({ type: "long-animation-frame" });
  } catch {}
  if (probe) {
    const wrap = (target, name, what) => {
      const original = target[name];
      if (typeof original !== "function") return;
      target[name] = function (...args) {
        if (state.on) count(what, owner(new Error().stack));
        return original.apply(this, args);
      };
    };
    wrap(Element.prototype, "getBoundingClientRect", "getBoundingClientRect");
    wrap(window, "getComputedStyle", "getComputedStyle");
    wrap(window, "requestAnimationFrame", "requestAnimationFrame");
    wrap(Element.prototype, "setAttribute", "setAttribute");
    wrap(Element.prototype, "removeAttribute", "removeAttribute");
    wrap(Document.prototype, "createTreeWalker", "createTreeWalker");
    wrap(Element.prototype, "closest", "closest");
    wrap(Element.prototype, "matches", "matches");
    wrap(CSSStyleDeclaration.prototype, "setProperty", "style.setProperty");
    for (const property of ["transform", "opacity", "width", "height", "zIndex"]) {
      const described = Object.getOwnPropertyDescriptor(CSSStyleDeclaration.prototype, property);
      if (described?.set === undefined) continue;
      Object.defineProperty(CSSStyleDeclaration.prototype, property, {
        ...described,
        set(value) {
          if (state.on) count(`style.${property}`, owner(new Error().stack));
          described.set.call(this, value);
        },
      });
    }
    for (const name of ["MutationObserver", "ResizeObserver"]) {
      const Original = window[name];
      window[name] = class extends Original {
        constructor(callback) {
          const who = owner(new Error().stack);
          super((records, observer) => {
            count(`${name} calls`, who);
            count(`${name} records`, who, records.length);
            if (name === "MutationObserver") count(`${name} records behind inert or hidden`, who, records.filter((record) => (record.target.nodeType === 1 ? record.target : record.target.parentElement)?.parentElement?.closest("[inert], [hidden]") != null).length);
            return callback(records, observer);
          });
        }
      };
    }
  }
  window.__p1 = {
    start: () => {
      state.frames = [];
      state.longtasks = [];
      state.loafs = [];
      state.counts = {};
      state.on = true;
      performance.mark("p1-sweep-start");
      requestAnimationFrame(loop);
    },
    stop: () => {
      performance.mark("p1-sweep-end");
      state.on = false;
      return { frames: state.frames, longtasks: state.longtasks, loafs: state.loafs, counts: state.counts };
    },
  };
}

const sleep = (milliseconds) => new Promise((done) => setTimeout(done, milliseconds));
const quantile = (values, q) => {
  if (values.length === 0) return 0;
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.floor(q * sorted.length))];
};
const round = (value, digits = 2) => Math.round(value * 10 ** digits) / 10 ** digits;

/** 🖱️ The pointer's place `t` seconds into a sweep. */
function sweepAt(t) {
  return { x: width * (0.5 + 0.46 * Math.sin((2 * Math.PI * t) / 3.7)), y: height * (0.5 + 0.4 * Math.sin((2 * Math.PI * t) / 2.3 + 0.7)) };
}

/** 🧹️ Sweeps the pointer for `length` seconds along {@link sweepAt}, at most one move every 8 ms. */
async function sweep(cdp, length) {
  const begun = Date.now();
  let moves = 0;
  for (;;) {
    const elapsed = Date.now() - begun;
    if (elapsed >= length * 1000) break;
    const at = sweepAt(elapsed / 1000);
    await cdp.send("Input.dispatchMouseEvent", { type: "mouseMoved", x: at.x, y: at.y, button: "none", pointerType: "mouse" });
    moves += 1;
    const spent = Date.now() - begun - elapsed;
    if (spent < 8) await sleep(8 - spent);
  }
  return moves;
}

/** 🚪️ Enters the site anonymously unless the learner is known already, and returns the prefix of the local store. */
async function enter(page) {
  await page.goto(url);
  const intro = page.locator('#quiz-main [data-card="introduction"] [data-overview-card-action="primary"]');
  const overview = page.locator('[data-layered-overview][data-mode="strip"]');
  await Promise.race([intro.waitFor({ timeout: 60_000 }), overview.waitFor({ timeout: 60_000 })]);
  if (await intro.isVisible()) {
    await intro.click();
    await page.locator('#quiz-main [data-card="identity"] input[type="radio"][value="anonymous"]').check();
    await page.locator('#quiz-main [data-card="identity"] [data-overview-card-action="primary"]').click();
  }
  await overview.waitFor({ timeout: 60_000 });
  return page.evaluate(() => Object.keys(localStorage).find((key) => key.startsWith("semio.quiz.") && key.endsWith(".learner"))?.replace(/learner$/u, "") ?? null);
}

/** 🎛️ Chooses the pets' liveliness in the local store, reloads and waits for the home overview to settle. */
async function choose(page, prefix, mode) {
  await page.evaluate(
    ([key, mode]) => {
      let stored = {};
      try {
        stored = JSON.parse(localStorage.getItem(key) ?? "{}") ?? {};
      } catch {
        stored = {};
      }
      localStorage.setItem(key, JSON.stringify({ ...stored, pets: mode, petsChosen: true }));
    },
    [`${prefix}preferences`, mode],
  );
  await page.mouse.move(width / 2, 20);
  await page.reload();
  await page.locator('[data-layered-overview][data-mode="strip"]').waitFor({ timeout: 60_000 });
  await page.locator(`.quiz-app[data-pets="${mode}"]`).waitFor({ timeout: 60_000 });
  if (mode !== "off") await page.locator(".pet-layer svg.pet").first().waitFor({ timeout: 90_000 });
  await sleep(6000);
  const start = sweepAt(0);
  await page.mouse.move(start.x, start.y, { steps: 4 });
  await sleep(1500);
}

const metric = (list, name) => list.metrics.find((entry) => entry.name === name)?.value ?? 0;

/** 🧾️ Main-thread self time by kind, forced layouts and style recalculations by owner, raster and GPU time and drawn frames of a trace, between the sweep's marks. */
function digestTrace(trace) {
  const events = trace.traceEvents ?? trace;
  const names = new Map();
  for (const event of events) if (event.ph === "M" && event.name === "thread_name") names.set(`${event.pid}:${event.tid}`, event.args.name);
  const marks = events.filter((event) => event.name === "p1-sweep-start" || event.name === "p1-sweep-end");
  const startMark = marks.find((event) => event.name === "p1-sweep-start");
  const endMark = marks.find((event) => event.name === "p1-sweep-end");
  if (startMark === undefined || endMark === undefined) return { error: "marks missing" };
  const main = `${startMark.pid}:${startMark.tid}`;
  const from = startMark.ts;
  const to = endMark.ts;
  const windowMs = (to - from) / 1000;
  const byThread = new Map();
  const open = new Map();
  for (const event of events) {
    const key = `${event.pid}:${event.tid}`;
    let complete = null;
    if (event.ph === "X") complete = { name: event.name, ts: event.ts, dur: event.dur ?? 0, args: event.args };
    else if (event.ph === "B") {
      const stack = open.get(key) ?? [];
      stack.push(event);
      open.set(key, stack);
    } else if (event.ph === "E") {
      const begun = open.get(key)?.pop();
      if (begun !== undefined) complete = { name: begun.name, ts: begun.ts, dur: event.ts - begun.ts, args: { ...begun.args, ...event.args } };
    }
    if (complete === null || complete.ts + complete.dur < from || complete.ts > to) continue;
    const list = byThread.get(key) ?? [];
    list.push(complete);
    byThread.set(key, list);
  }
  const JS = new Set(["FunctionCall", "EventDispatch", "TimerFire", "FireAnimationFrame", "FireIdleCallback", "EvaluateScript", "v8.callFunction", "v8.run", "V8.Execute", "RunMicrotasks", "v8.evaluateModule", "XHRReadyStateChange", "XHRLoad", "v8.compile", "v8.compileModule", "V8.ParseFunction"]);
  const STYLE = new Set(["UpdateLayoutTree", "RecalculateStyles"]);
  const PAINT = new Set(["Paint", "PaintImage", "PrePaint", "Layerize", "UpdateLayer", "UpdateLayerTree", "CompositeLayers", "Commit", "PaintSetup", "Decode Image", "ImageDecodeTask"]);
  const kindOf = (name) => (STYLE.has(name) ? "style" : name === "Layout" ? "layout" : PAINT.has(name) ? "paint" : name === "HitTest" ? "hitTest" : /GC/u.test(name) ? "gc" : JS.has(name) || /^v8\./iu.test(name) ? "scripting" : "other");
  const mainEvents = (byThread.get(main) ?? []).sort((a, b) => a.ts - b.ts || b.dur - a.dur);
  const self = { scripting: 0, style: 0, layout: 0, paint: 0, hitTest: 0, gc: 0, other: 0 };
  const forced = { layout: {}, style: {} };
  const forcedFunctions = {};
  let styleElementsMax = 0;
  let styleElements = 0;
  let busy = 0;
  const stack = [];
  const clip = (event) => Math.max(0, Math.min(event.ts + event.dur, to) - Math.max(event.ts, from));
  for (const event of mainEvents) {
    while (stack.length > 0 && stack[stack.length - 1].ts + stack[stack.length - 1].dur <= event.ts) stack.pop();
    const parent = stack[stack.length - 1];
    event.children = 0;
    event.parent = parent;
    if (parent !== undefined) parent.children += clip(event);
    else busy += clip(event);
    stack.push(event);
    if (event.name === "UpdateLayoutTree") {
      const count = event.args?.elementCount ?? event.args?.endData?.elementCount ?? 0;
      styleElements += count;
      styleElementsMax = Math.max(styleElementsMax, count);
    }
    if (event.name === "Layout" || STYLE.has(event.name)) {
      let inside = false;
      for (let above = parent; above !== undefined; above = above.parent) if (JS.has(above.name) || /^v8\./iu.test(above.name)) inside = true;
      if (inside) {
        const frames = event.args?.beginData?.stackTrace ?? event.args?.data?.stackTrace ?? [];
        const first = frames.find((frame) => ownerOf(frame.url) !== "") ?? frames[0];
        const who = first === undefined ? "unknown" : ownerOf(first.url) || "unknown";
        const bucket = (forced[event.name === "Layout" ? "layout" : "style"][who] ??= { count: 0, ms: 0 });
        bucket.count += 1;
        bucket.ms += clip(event) / 1000;
        const signature = `${event.name} ${who} ${first?.functionName ?? "?"}`;
        forcedFunctions[signature] = (forcedFunctions[signature] ?? 0) + clip(event) / 1000;
      }
    }
  }
  for (const event of mainEvents) self[kindOf(event.name)] += Math.max(0, clip(event) - event.children) / 1000;
  let raster = 0;
  let gpu = 0;
  let drawn = 0;
  let compositor = 0;
  for (const [key, list] of byThread) {
    const name = names.get(key) ?? "";
    if (name.startsWith("CompositorTileWorker")) for (const event of list) if (event.name === "RasterTask") raster += clip(event) / 1000;
    if (name === "CrGpuMain") for (const event of list) if (event.name === "GPUTask") gpu += clip(event) / 1000;
    if (name === "Compositor" && key.startsWith(`${startMark.pid}:`)) for (const event of list) if (event.name === "DrawFrame") drawn += 1;
    if (name === "VizCompositorThread") for (const event of list) if (event.name === "Graphics.Pipeline.DrawAndSwap" || event.name === "Display::DrawAndSwap") compositor += 1;
  }
  for (const event of events) if (event.name === "DrawFrame" && event.ts >= from && event.ts <= to && event.pid === startMark.pid && event.ph !== "X" && event.ph !== "B" && event.ph !== "E") drawn += 1;
  const top = Object.entries(forcedFunctions)
    .sort((a, b) => b[1] - a[1])
    .slice(0, 12)
    .map(([signature, ms]) => `${signature}: ${round(ms)} ms`);
  for (const group of Object.values(forced)) for (const bucket of Object.values(group)) bucket.ms = round(bucket.ms);
  return { windowMs: round(windowMs), busyMs: round(busy / 1000), selfMs: Object.fromEntries(Object.entries(self).map(([name, ms]) => [name, round(ms)])), forced, topForced: top, styleElements, styleElementsMax, rasterMs: round(raster), gpuTaskMs: round(gpu), drawFrames: drawn, vizSwaps: compositor };
}

/** 🧮️ Sampled CPU time by owner and the time inside any frame of the pets' modules. */
function digestProfile(profile) {
  const nodes = new Map(profile.nodes.map((node) => [node.id, node]));
  const parent = new Map();
  for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
  const ownerCache = new Map();
  const attributed = (id) => {
    if (ownerCache.has(id)) return ownerCache.get(id);
    const node = nodes.get(id);
    const fn = node.callFrame.functionName;
    let result;
    if (fn === "(idle)") result = "idle";
    else if (fn === "(program)") result = "program";
    else if (fn === "(garbage collector)") result = "gc";
    else {
      const own = ownerOf(node.callFrame.url);
      result = own !== "" ? own : parent.has(id) ? attributed(parent.get(id)) : "native";
      if (result === "idle" || result === "program" || result === "root") result = "native";
    }
    ownerCache.set(id, result);
    return result;
  };
  const petsCache = new Map();
  const insidePets = (id) => {
    if (petsCache.has(id)) return petsCache.get(id);
    const node = nodes.get(id);
    const result = ownerOf(node.callFrame.url) === "pets" || (parent.has(id) && insidePets(parent.get(id)));
    petsCache.set(id, result);
    return result;
  };
  const byOwner = {};
  let pets = 0;
  const petFunctions = {};
  for (let index = 0; index < profile.samples.length; index++) {
    const id = profile.samples[index];
    const delta = (profile.timeDeltas[index + 1] ?? 0) / 1000;
    const who = attributed(id);
    byOwner[who] = (byOwner[who] ?? 0) + delta;
    if (insidePets(id)) {
      pets += delta;
      const node = nodes.get(id);
      const name = `${node.callFrame.functionName || "(anonymous)"} ${ownerOf(node.callFrame.url) || "native"}`;
      petFunctions[name] = (petFunctions[name] ?? 0) + delta;
    }
  }
  return {
    byOwnerMs: Object.fromEntries(Object.entries(byOwner).map(([who, ms]) => [who, round(ms)])),
    insidePetsMs: round(pets),
    petsSelfTop: Object.entries(petFunctions)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 12)
      .map(([name, ms]) => `${name}: ${round(ms)} ms`),
  };
}

/** 📏️ One measured sweep of the page in `mode`. */
async function measure(browser, context, page, cdp, mode, rep, instrumented) {
  const before = await cdp.send("Performance.getMetrics");
  const tracing = instrumented ? await browser.newBrowserCDPSession() : null;
  if (tracing !== null) {
    await tracing.send("Tracing.start", { transferMode: "ReturnAsStream", traceConfig: { recordMode: "recordAsMuchAsPossible", includedCategories: ["devtools.timeline", "disabled-by-default-devtools.timeline", "disabled-by-default-devtools.timeline.frame", "disabled-by-default-devtools.timeline.stack", "blink.user_timing"] } });
    await cdp.send("Profiler.enable");
    await cdp.send("Profiler.setSamplingInterval", { interval: 250 });
    await cdp.send("Profiler.start");
  }
  await page.evaluate(() => window.__p1.start());
  const begun = Date.now();
  const moves = await sweep(cdp, instrumented ? traceSeconds : seconds);
  const wall = (Date.now() - begun) / 1000;
  const seen = await page.evaluate(() => window.__p1.stop());
  const after = await cdp.send("Performance.getMetrics");
  let trace = null;
  let profile = null;
  if (tracing !== null) {
    profile = digestProfile((await cdp.send("Profiler.stop")).profile);
    await cdp.send("Profiler.disable");
    const complete = new Promise((done) => tracing.once("Tracing.tracingComplete", done));
    await tracing.send("Tracing.end");
    const { stream } = await complete;
    const chunks = [];
    for (;;) {
      const read = await tracing.send("IO.read", { handle: stream, size: 16 * 1024 * 1024 });
      chunks.push(read.base64Encoded ? Buffer.from(read.data, "base64") : Buffer.from(read.data, "utf8"));
      if (read.eof) break;
    }
    await tracing.send("IO.close", { handle: stream });
    await tracing.detach();
    const buffer = Buffer.concat(chunks);
    if (flag("keep-trace")) writeFileSync(join(out, `trace-${label}-${mode}.json`), buffer);
    trace = digestTrace(JSON.parse(buffer.toString("utf8")));
  }
  const frames = seen.frames.length;
  const gaps = seen.frames.slice(1).map((time, index) => time - seen.frames[index]);
  const delta = (name) => metric(after, name) - metric(before, name);
  const perFrame = (name) => round((delta(name) * 1000) / Math.max(frames, 1));
  const loafScripts = { pets: { ms: 0, forced: 0 }, page: { ms: 0, forced: 0 } };
  for (const loaf of seen.loafs)
    for (const script of loaf.scripts) {
      const who = ownerOf(script.url) === "pets" ? "pets" : "page";
      loafScripts[who].ms += script.duration;
      loafScripts[who].forced += script.forced;
    }
  const shown = await page.evaluate(() => ({ pets: document.querySelectorAll(".pet-layer svg.pet").length, copies: document.querySelectorAll(".pet-layer [inert]").length, elements: document.getElementsByTagName("*").length, inert: document.querySelectorAll("[inert]").length, revealed: document.querySelectorAll("[data-layered-card][data-revealed]").length }));
  return {
    label,
    browser: kind,
    mode,
    rep,
    instrumented,
    wallSeconds: round(wall),
    moves,
    frames,
    fps: round(frames / wall, 1),
    gapMs: { p50: round(quantile(gaps, 0.5), 1), p95: round(quantile(gaps, 0.95), 1), max: round(Math.max(0, ...gaps), 1) },
    longTasks: { count: seen.longtasks.length, ms: round(seen.longtasks.reduce((sum, value) => sum + value, 0)) },
    loaf: { count: seen.loafs.length, ms: round(seen.loafs.reduce((sum, entry) => sum + entry.duration, 0)), blockingMs: round(seen.loafs.reduce((sum, entry) => sum + entry.blocking, 0)), renderMs: round(seen.loafs.reduce((sum, entry) => sum + entry.render, 0)), styleAndLayoutMs: round(seen.loafs.reduce((sum, entry) => sum + entry.styleAndLayout, 0)), scripts: { pets: { ms: round(loafScripts.pets.ms), forcedMs: round(loafScripts.pets.forced) }, page: { ms: round(loafScripts.page.ms), forcedMs: round(loafScripts.page.forced) } } },
    perFrameMs: { task: perFrame("TaskDuration"), script: perFrame("ScriptDuration"), layout: perFrame("LayoutDuration"), style: perFrame("RecalcStyleDuration") },
    counts: { layouts: delta("LayoutCount"), styles: delta("RecalcStyleCount") },
    shown,
    calls: seen.counts,
    trace,
    profile,
  };
}

const launch = kind === "gpu" ? { channel: "chromium" } : {};
const browser = await chromium.launch(launch);
const runs = [];
const session = async (instrumented) => {
  const context = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: 1, locale: "en-US" });
  await context.addInitScript(instrument, { probe: instrumented, petChunks });
  const page = await context.newPage();
  const cdp = await context.newCDPSession(page);
  await cdp.send("Performance.enable", { timeDomain: "timeTicks" });
  const prefix = await enter(page);
  const renderer = await page.evaluate(() => {
    const gl = document.createElement("canvas").getContext("webgl");
    const info = gl?.getExtension("WEBGL_debug_renderer_info");
    return info ? gl.getParameter(info.UNMASKED_RENDERER_WEBGL) : "no webgl";
  });
  return { context, page, cdp, prefix, renderer };
};

const clean = only ? null : await session(false);
if (clean !== null) {
  process.stdout.write(`${label}: browser ${kind}, renderer ${clean.renderer}, pet chunks ${petChunks.join(" ") || "(development build)"}\n`);
  for (let rep = 1; rep <= repeat; rep++) {
    const order = rep % 2 === 1 ? modes : [...modes].reverse();
    for (const mode of order) {
      await choose(clean.page, clean.prefix, mode);
      const run = await measure(browser, clean.context, clean.page, clean.cdp, mode, rep, false);
      runs.push(run);
      appendFileSync(sink, `${JSON.stringify(run)}\n`);
      process.stdout.write(`${JSON.stringify({ mode, rep, fps: run.fps, frames: run.frames, moves: run.moves, gapMs: run.gapMs, longTasks: run.longTasks, loaf: { count: run.loaf.count, ms: run.loaf.ms, scripts: run.loaf.scripts }, perFrameMs: run.perFrameMs, counts: run.counts, shown: run.shown })}\n`);
    }
  }
  await clean.context.close();
}
if (probe) {
  for (const [counted, traced] of [
    [false, true],
    [true, false],
  ]) {
    const instrumented = await session(counted);
    for (const mode of modes) {
      await choose(instrumented.page, instrumented.prefix, mode);
      const run = { ...(await measure(browser, instrumented.context, instrumented.page, instrumented.cdp, mode, 0, traced)), instrumented: counted ? "counted" : "traced" };
      runs.push(run);
      appendFileSync(sink, `${JSON.stringify(run)}\n`);
      process.stdout.write(`${JSON.stringify({ mode, instrumented: run.instrumented, fps: run.fps, calls: run.calls, trace: run.trace, profile: run.profile })}\n`);
    }
    await instrumented.context.close();
  }
}
await browser.close();

const table = ["| mode | runs | fps | frame p50/p95 ms | long tasks (n, ms) | task/script/layout/style ms per frame | layouts, styles |", "|---|---|---|---|---|---|---|"];
for (const mode of modes) {
  const mine = runs.filter((run) => run.mode === mode && !run.instrumented);
  if (mine.length === 0) continue;
  const mean = (pick) => round(mine.reduce((sum, run) => sum + pick(run), 0) / mine.length, 1);
  table.push(`| ${mode} | ${mine.length} | ${mine.map((run) => run.fps).join(" / ")} | ${mean((run) => run.gapMs.p50)} / ${mean((run) => run.gapMs.p95)} | ${mean((run) => run.longTasks.count)}, ${mean((run) => run.longTasks.ms)} | ${mean((run) => run.perFrameMs.task)} / ${mean((run) => run.perFrameMs.script)} / ${mean((run) => run.perFrameMs.layout)} / ${mean((run) => run.perFrameMs.style)} | ${mean((run) => run.counts.layouts)}, ${mean((run) => run.counts.styles)} |`);
}
writeFileSync(join(out, `summary-${label}.md`), `${table.join("\n")}\n`);
process.stdout.write(`${table.join("\n")}\n`);
