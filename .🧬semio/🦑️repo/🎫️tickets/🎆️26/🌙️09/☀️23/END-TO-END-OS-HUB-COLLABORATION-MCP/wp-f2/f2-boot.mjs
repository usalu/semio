#!/usr/bin/env bun
/** 🚀️ F2 — boot timeline of the served `s` React shell, cold (fresh browser profile) then warm (reload in the same profile).
 *
 * Per load: navigation timing (TTFB, DOMContentLoaded, load), first paint / first contentful paint / largest contentful
 * paint, the moment the shell's `data-semio-os-ready` beacon appears (MutationObserver, page clock), the moment every plugin
 * of the catalog probe reports `loaded`, long tasks, a sampled CPU profile of the whole boot (top self + inclusive frames),
 * and the resource timing reduced to counts/bytes per kind + the 12 biggest and 12 slowest requests.
 *
 * usage: bun f2-boot.mjs <baseUrl> <tag> [--warm-runs 1] [--no-profile] */
import { chromium } from "playwright";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { sleep } from "../wp-f1/f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6580/";
const tag = argv[1] ?? "boot";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const warmRuns = Number(valueOf("--warm-runs", "1"));
const profileBoot = !argv.includes("--no-profile");
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const log = (...parts) => console.log(`[f2-boot ${new Date().toISOString().slice(11, 19)}]`, ...parts);

const hooks = () => {
  performance.setResourceTimingBufferSize(100_000);
  const marks = { longTasks: [], paints: {}, lcp: null, ready: null, error: null };
  Object.defineProperty(window, "__f2Boot", { value: marks });
  new MutationObserver(() => {
    const root = document.documentElement;
    if (!root) return;
    if (marks.ready === null && root.hasAttribute("data-semio-os-ready")) marks.ready = performance.now();
    if (marks.error === null && root.hasAttribute("data-semio-os-error")) marks.error = performance.now();
  }).observe(document, { attributes: true, subtree: true, attributeFilter: ["data-semio-os-ready", "data-semio-os-error"] });
  new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) marks.paints[entry.name] = entry.startTime;
  }).observe({ type: "paint", buffered: true });
  new PerformanceObserver((list) => {
    const last = list.getEntries().at(-1);
    if (last) marks.lcp = last.startTime;
  }).observe({ type: "largest-contentful-paint", buffered: true });
  new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) marks.longTasks.push([Math.round(entry.startTime), Math.round(entry.duration)]);
  }).observe({ type: "longtask", buffered: true });
};

async function profileStart(cdp) {
  await cdp.send("Profiler.enable");
  await cdp.send("Profiler.setSamplingInterval", { interval: 1000 });
  await cdp.send("Profiler.start");
}

async function profileStop(cdp) {
  const { profile } = await cdp.send("Profiler.stop");
  await cdp.send("Profiler.disable");
  const byId = new Map(profile.nodes.map((node) => [node.id, node]));
  const parent = new Map();
  for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
  const deltas = profile.timeDeltas ?? [];
  const self = new Map();
  const inclusive = new Map();
  let total = 0;
  let idle = 0;
  const keyOf = (frame) => `${frame.functionName || "(anon)"} ${decodeURIComponent(frame.url.split("?")[0].split("/").slice(-3).join("/")).slice(0, 90)}:${frame.lineNumber + 1}`;
  profile.samples.forEach((id, index) => {
    const delta = deltas[index] ?? 0;
    total += delta;
    const leaf = byId.get(id).callFrame;
    if (leaf.functionName === "(idle)") idle += delta;
    self.set(keyOf(leaf), (self.get(keyOf(leaf)) ?? 0) + delta);
    const seen = new Set();
    for (let cursor = id; cursor !== undefined; cursor = parent.get(cursor)) {
      const frame = byId.get(cursor).callFrame;
      if (!frame.url) continue;
      const key = keyOf(frame);
      if (seen.has(key)) continue;
      seen.add(key);
      inclusive.set(key, (inclusive.get(key) ?? 0) + delta);
    }
  });
  const top = (map, n) => [...map].sort((a, b) => b[1] - a[1]).slice(0, n).map(([key, value]) => `${Math.round(value / 1000)}ms ${key}`);
  return { totalMs: Math.round(total / 1000), idleMs: Math.round(idle / 1000), self: top(self, 25), inclusive: top(inclusive, 30) };
}

async function measureLoad(page, cdp, label, navigate) {
  if (profileBoot) await profileStart(cdp);
  const wallStart = Date.now();
  await navigate();
  const deadline = Date.now() + 300_000;
  let readyAt = null;
  let loadedAt = null;
  let plugins = null;
  while (Date.now() < deadline) {
    const state = await page.evaluate(() => {
      const probe = window.__semioOsCatalogProbe;
      const rows = probe?.plugins ?? [];
      return { ready: window.__f2Boot?.ready ?? null, error: window.__f2Boot?.error ?? null, total: rows.length, loaded: rows.filter((row) => row.status === "loaded").length, statuses: [...new Set(rows.map((row) => row.status))], now: performance.now() };
    }).catch(() => null);
    if (state?.ready !== null && state?.ready !== undefined && readyAt === null) readyAt = state.ready;
    if (state && state.total > 0 && state.loaded === state.total && loadedAt === null) {
      loadedAt = state.now;
      plugins = state;
      break;
    }
    if (state?.error) break;
    plugins = state;
    await sleep(250);
  }
  await sleep(3_000);
  const profile = profileBoot ? await profileStop(cdp) : null;
  const facts = await page.evaluate(() => {
    const nav = performance.getEntriesByType("navigation")[0];
    const resources = performance.getEntriesByType("resource");
    const kindOf = (entry) => {
      const path = decodeURIComponent(new URL(entry.name).pathname);
      if (path.endsWith(".wasm")) return "wasm";
      if (path.includes("/node_modules/.vite/deps/")) return "dep";
      if (/\.(t|j)sx?$|\.mjs$/u.test(path) || entry.initiatorType === "script") return "module";
      if (path.endsWith(".json") || path.includes(".json?")) return "json";
      if (path.startsWith("/_semio/hub")) return "hub";
      return entry.initiatorType || "other";
    };
    const byKind = {};
    for (const entry of resources) {
      const row = (byKind[kindOf(entry)] ??= { count: 0, transferMB: 0, decodedMB: 0 });
      row.count += 1;
      row.transferMB += entry.transferSize / 2 ** 20;
      row.decodedMB += entry.decodedBodySize / 2 ** 20;
    }
    for (const row of Object.values(byKind)) {
      row.transferMB = +row.transferMB.toFixed(2);
      row.decodedMB = +row.decodedMB.toFixed(2);
    }
    const name = (entry) => decodeURIComponent(new URL(entry.name).pathname).split("/").slice(-2).join("/").slice(0, 80);
    const lastResponseEnd = Math.max(0, ...resources.map((entry) => entry.responseEnd));
    return {
      ttfb: nav ? Math.round(nav.responseStart) : null,
      domContentLoaded: nav ? Math.round(nav.domContentLoadedEventEnd) : null,
      load: nav ? Math.round(nav.loadEventEnd) : null,
      paints: Object.fromEntries(Object.entries(window.__f2Boot.paints).map(([key, value]) => [key, Math.round(value)])),
      lcp: window.__f2Boot.lcp === null ? null : Math.round(window.__f2Boot.lcp),
      ready: window.__f2Boot.ready === null ? null : Math.round(window.__f2Boot.ready),
      longTasks: { count: window.__f2Boot.longTasks.length, totalMs: window.__f2Boot.longTasks.reduce((sum, [, duration]) => sum + duration, 0), top: [...window.__f2Boot.longTasks].sort((a, b) => b[1] - a[1]).slice(0, 8) },
      resources: { count: resources.length, lastResponseEnd: Math.round(lastResponseEnd), byKind },
      biggest: [...resources].sort((a, b) => b.decodedBodySize - a.decodedBodySize).slice(0, 12).map((entry) => `${(entry.decodedBodySize / 2 ** 20).toFixed(1)}MB (xfer ${(entry.transferSize / 2 ** 20).toFixed(1)}) ${Math.round(entry.duration)}ms ${name(entry)}`),
      slowest: [...resources].sort((a, b) => b.duration - a.duration).slice(0, 12).map((entry) => `${Math.round(entry.duration)}ms @${Math.round(entry.startTime)} wait ${Math.round(entry.requestStart - entry.startTime)} ${name(entry)}`),
      modulesDone: Math.round(Math.max(0, ...resources.filter((entry) => kindOf(entry) === "module" || kindOf(entry) === "dep").map((entry) => entry.responseEnd))),
    };
  });
  const result = { label, wallMs: Date.now() - wallStart, allLoadedAt: loadedAt === null ? null : Math.round(loadedAt), plugins: plugins ? { total: plugins.total, loaded: plugins.loaded, statuses: plugins.statuses } : null, ...facts, profile };
  log(label, JSON.stringify({ wallMs: result.wallMs, ttfb: result.ttfb, dcl: result.domContentLoaded, fcp: result.paints["first-contentful-paint"], lcp: result.lcp, ready: result.ready, allLoaded: result.allLoadedAt, modulesDone: result.modulesDone, resources: result.resources.count, longTasks: result.longTasks.count, longTaskMs: result.longTasks.totalMs }));
  return result;
}

const profileDir = mkdtempSync(join(tmpdir(), "f2-boot-profile-"));
const context = await chromium.launchPersistentContext(profileDir, { headless: true, viewport: { width: 1440, height: 900 }, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = context.pages()[0] ?? (await context.newPage());
page.setDefaultNavigationTimeout(300_000);
await page.addInitScript(hooks);
const cdp = await context.newCDPSession(page);
const runs = [];
try {
  runs.push(await measureLoad(page, cdp, "cold", () => page.goto(baseUrl, { waitUntil: "commit" })));
  for (let index = 1; index <= warmRuns; index += 1) runs.push(await measureLoad(page, cdp, `warm-${index}`, () => page.reload({ waitUntil: "commit" })));
} finally {
  await context.close();
  rmSync(profileDir, { recursive: true, force: true });
}
const out = `${generated}f2-boot-${tag}.json`;
writeFileSync(out, JSON.stringify({ baseUrl, tag, runs }, null, 1));
log("wrote", out);
