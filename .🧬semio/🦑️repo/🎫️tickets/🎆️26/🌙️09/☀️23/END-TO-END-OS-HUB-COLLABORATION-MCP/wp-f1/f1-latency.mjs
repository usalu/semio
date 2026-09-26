#!/usr/bin/env bun
/** ⏱️ F1 — interaction latency inside `s`: input → paint for one program window and one gesture.
 *
 * Latency of one input = from the input event's own timestamp to the end of the first frame after it (a capture-phase
 * listener arms `requestAnimationFrame` → `MessageChannel`, which runs after that frame's rAF work and paint). Every
 * `renderFrame` of the wasm canvas modules (`framework_editor`, `framework_surface`, the node-graph / board surfaces) is
 * timed by wrapping the exported classes' prototypes; the browser's own Event Timing entries (keydown/pointerdown/up, ≥16 ms)
 * are recorded too. A 3 s CPU profile covers the gesture's middle.
 *
 * usage: bun f1-latency.mjs <baseUrl> --tag <t> --plugin <pluginId> --app <appId> --window <window-id suffix>
 *        --action type|drag|wheel [--n 24] [--at 0.5,0.5] [--select <css inside the window body>] */
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { bootShell, launch, openProgramByPalette, profileWindow, quantile, settleMain, sleep, traceWindow } from "./f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6620/";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const tag = valueOf("--tag", "adhoc");
const program = { pluginId: valueOf("--plugin", "trinity"), appId: valueOf("--app", "s.trinity.jack@1/*#editor") };
const windowSuffix = valueOf("--window", "trinity-jack-editor");
const action = valueOf("--action", "type");
const n = Number(valueOf("--n", "24"));
const [ax, ay] = valueOf("--at", "0.5,0.5").split(",").map(Number);
const select = valueOf("--select", "canvas");
const profileMs = Number(valueOf("--profile-ms", "3000"));
const out = fileURLToPath(new URL(`./generated/f1-latency-${tag}.json`, import.meta.url));

const { browser, page, cdp } = await launch();
const result = { tag, program, windowSuffix, action, n };
try {
  result.beacon = await bootShell(page, cdp, baseUrl);
  const opened = await openProgramByPalette(page, program);
  result.windowIds = opened.windowIds;
  if (opened.windowIds.length === 0) throw new Error(`open failed: ${opened.detail}`);
  result.settle = await settleMain(cdp, 30_000);
  result.hooked = await page.evaluate(async () => {
    const urls = [...new Set(performance.getEntriesByType("resource").map((entry) => entry.name).filter((name) => /\.js(\?|$)/u.test(name) && /\/pkg\/|bindings\//u.test(decodeURIComponent(name))))];
    const hooked = [];
    const state = { paints: [], instances: new Map(), rows: [], events: [] };
    Object.defineProperty(window, "__f1lat", { value: state });
    for (const url of urls) {
      let module;
      try {
        module = await import(url);
      } catch {
        continue;
      }
      for (const [name, value] of Object.entries(module)) {
        if (typeof value !== "function" || typeof value.prototype?.renderFrame !== "function") continue;
        const original = value.prototype.renderFrame;
        value.prototype.renderFrame = function (...args) {
          const t0 = performance.now();
          try {
            return original.apply(this, args);
          } finally {
            state.paints.push({ cls: name, t0, ms: performance.now() - t0 });
            state.instances.set(name, this);
          }
        };
        hooked.push(`${name} @ ${decodeURIComponent(url).split("/").slice(-3).join("/")}`);
      }
    }
    for (const type of ["keydown", "pointerdown", "pointermove", "pointerup", "wheel"]) {
      addEventListener(
        type,
        (event) => {
          if (!event.isTrusted) return;
          const t0 = event.timeStamp;
          requestAnimationFrame(() => {
            const frame = performance.now();
            const channel = new MessageChannel();
            channel.port1.onmessage = () => state.rows.push({ type, t0, toFrame: frame - t0, toAfterFrame: performance.now() - t0 });
            channel.port2.postMessage(0);
          });
        },
        { capture: true, passive: true },
      );
    }
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) state.events.push({ name: entry.name, duration: entry.duration, processing: entry.processingEnd - entry.processingStart, delay: entry.processingStart - entry.startTime });
    }).observe({ type: "event", durationThreshold: 16, buffered: false });
    return hooked;
  });
  const target = page.locator(`[id$="${windowSuffix}"] [data-slot="window-body"] ${select}`).first();
  await target.waitFor({ state: "visible", timeout: 30_000 });
  const box = await target.boundingBox();
  result.target = box;
  const x = box.x + box.width * ax;
  const y = box.y + box.height * ay;
  if (!argv.includes("--no-focus-click")) await page.mouse.click(x, y);
  await sleep(1_500);
  await page.screenshot({ path: out.replace(/\.json$/u, "-before.png") });
  await page.evaluate(() => {
    window.__f1lat.rows.length = 0;
    window.__f1lat.paints.length = 0;
    window.__f1lat.events.length = 0;
  });
  const profile = argv.includes("--trace") ? traceWindow(cdp, profileMs + 2_000, { longEventsMs: 30 }) : profileWindow(cdp, profileMs + 2_000);
  await sleep(2_000);
  await page.evaluate(() => {
    window.__f1lat.rows.length = 0;
    window.__f1lat.paints.length = 0;
    window.__f1lat.events.length = 0;
  });
  if (action === "type") {
    for (let i = 0; i < n; i += 1) {
      await page.keyboard.press(i % 2 === 0 ? "a" : "Backspace");
      await sleep(180);
    }
  } else if (action === "drag") {
    await page.mouse.move(x, y);
    await page.mouse.down();
    for (let i = 1; i <= n; i += 1) {
      await page.mouse.move(x + 3 * i, y + 2 * i);
      await sleep(33);
    }
    await page.mouse.up();
  } else if (action === "click") {
    for (let i = 0; i < n; i += 1) {
      await page.mouse.click(x + (i % 3) * 7, y + (i % 2) * 5);
      await sleep(700);
    }
  } else if (action === "wheel") {
    await page.mouse.move(x, y);
    for (let i = 0; i < n; i += 1) {
      await page.mouse.wheel(0, i % 2 === 0 ? 120 : -120);
      await sleep(150);
    }
  }
  result.profile = await profile;
  await sleep(1_000);
  const state = await page.evaluate(() => ({ rows: window.__f1lat.rows, paints: window.__f1lat.paints, events: window.__f1lat.events }));
  const byType = {};
  for (const row of state.rows) (byType[row.type] ??= []).push(row.toAfterFrame);
  result.latency = Object.fromEntries(Object.entries(byType).map(([type, values]) => [type, { n: values.length, p50: quantile(values, 0.5), p95: quantile(values, 0.95), max: quantile(values, 1) }]));
  const byClass = {};
  for (const paint of state.paints) (byClass[paint.cls] ??= []).push(paint.ms);
  result.paints = Object.fromEntries(Object.entries(byClass).map(([cls, values]) => [cls, { n: values.length, p50: quantile(values, 0.5), p95: quantile(values, 0.95), max: quantile(values, 1), totalMs: Math.round(values.reduce((a, b) => a + b, 0)) }]));
  const byEvent = {};
  for (const entry of state.events) (byEvent[entry.name] ??= []).push(entry.duration);
  result.eventTiming = Object.fromEntries(Object.entries(byEvent).map(([name, values]) => [name, { n: values.length, p50: quantile(values, 0.5), p95: quantile(values, 0.95) }]));
  await page.screenshot({ path: out.replace(/\.json$/u, ".png") });
} catch (error) {
  result.fatal = String(error).split("\n")[0];
} finally {
  writeFileSync(out, JSON.stringify(result, null, 1));
  console.log(JSON.stringify({ ...result, profile: result.profile ? { busyPct: result.profile.busyPct, inclusive: result.profile.inclusive?.slice(0, 10), longEvents: result.profile.longEvents } : null }, null, 1));
  await browser.close();
}
