/** 🧮️ Read-only probe for ticket 26/09/02 wave B4: ATTRIBUTES the ~5.4 s of main-thread long tasks a
 * Nakagin example switch pays (W-S2 §2.3 measured the total and explicitly did not split it).
 *
 * Extends `🔍️ws2-scene-latency-probe.ts` with three instruments that need no source edit — a wrapped
 * `Worker.postMessage`/`onmessage`, a wrapped `console`, and a V8 sampling profile over the switch
 * window — all charging into a `globalThis.__b4` span table of `{ ms, n }` per phase, snapshotted at
 * the click and again when the world lands so the report is the DIFF for exactly one switch.
 *
 * The same table is what temporary `[DEBUG]` source spans write into: wave B4 ran it with spans added
 * in `🔌️PluginRuntime` (`intake.advance`, `project.wall`, `project.nodes`, `maintenance.*`),
 * `🗣️Interpreter` (`interp.assemble`) and `🌐️World3dHost` (`world3d.renderToCommit`), then removed
 * them once the attribution was recorded in `📓️2026-09-11-wave-B4-main-thread-refresh.md`. Re-add
 * spans the same way (`__b4[name] ??= { ms: 0, n: 0 }`) to attribute a new phase; the probe needs no
 * change.
 *
 * Drives a SEPARATE headless chromium against an already-running dev target; it never starts a server
 * and never touches an interactive browser tab.
 *
 * `bun 🔍️b4-main-thread-probe.ts [url] [--out <dir>]`
 */
import { chromium } from "../../../../../../../node_modules/playwright/index.mjs";

type Span = { readonly ms: number; readonly n: number };

const url = process.argv.find((argument) => argument.startsWith("http")) ?? "http://127.0.0.1:6013/?plugin=puzzle3d";
const out = process.argv.includes("--out") ? process.argv[process.argv.indexOf("--out") + 1]! : ".";
const started = Date.now();
const log = (line: string) => console.log(`[${((Date.now() - started) / 1000).toFixed(1)}s] ${line}`);

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
page.setDefaultTimeout(180_000);
await page.addInitScript(() => {
  const state = { posts: [] as number[], tasks: [] as { readonly at: number; readonly ms: number }[] };
  (window as unknown as { __ws2: typeof state }).__ws2 = state;
  const spans = ((globalThis as unknown as { __b4?: Record<string, { ms: number; n: number }> }).__b4 ??= {});
  const charge = (name: string, at: number) => { const slot = (spans[name] ??= { ms: 0, n: 0 }); slot.ms += performance.now() - at; slot.n += 1; };
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (this: Worker, ...args: unknown[]) {
    const at = performance.now();
    state.posts.push(at);
    try { return (post as (...rest: unknown[]) => unknown).apply(this, args); } finally { charge("page.workerPost", at); }
  } as typeof Worker.prototype.postMessage;
  const inherited = Object.getOwnPropertyDescriptor(Worker.prototype, "onmessage");
  if (inherited?.set && inherited.get) {
    Object.defineProperty(Worker.prototype, "onmessage", {
      configurable: true,
      get: inherited.get,
      set(this: Worker, handler: unknown) {
        if (typeof handler !== "function") return inherited.set!.call(this, handler);
        inherited.set!.call(this, function (this: unknown, event: unknown) { const at = performance.now(); try { return (handler as (value: unknown) => unknown).call(this, event); } finally { charge("page.workerMessage", at); } });
      },
    });
  }
  for (const level of ["log", "warn", "error", "debug", "info"] as const) {
    const original = console[level].bind(console);
    console[level] = (...args: unknown[]) => { const at = performance.now(); try { original(...args); } finally { charge(`page.console.${level}`, at); } };
  }
  try {
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) state.tasks.push({ at: entry.startTime, ms: entry.duration });
    }).observe({ entryTypes: ["longtask"] });
  } catch {
    /* longtask unsupported */
  }
});
const consoleLines: { readonly at: number; readonly line: string }[] = [];
page.on("console", (message) => consoleLines.push({ at: Date.now(), line: `${message.type()}|${message.text().slice(0, 200)}` }));
await page.goto(url, { waitUntil: "commit", timeout: 180_000 });

/** 🚚️ One entry per mounted world viewport, plus the page clock the instruments share. */
const worlds = async () =>
  page.evaluate(() => ({
    now: performance.now(),
    views: Array.from(document.querySelectorAll("[data-instances-json]")).map((element) => {
      const host = element as HTMLElement;
      let instances: unknown[] = [];
      try { instances = JSON.parse(host.dataset.instancesJson ?? "[]"); } catch { instances = []; }
      return { bytes: (host.dataset.instancesJson ?? "").length, instances: instances.length };
    }),
  }));

const spans = async (): Promise<Record<string, Span>> => page.evaluate(() => JSON.parse(JSON.stringify((globalThis as unknown as { __b4?: Record<string, Span> }).__b4 ?? {})) as Record<string, Span>);
const marks = async () => page.evaluate(() => (window as unknown as { __ws2: { posts: number[]; tasks: { at: number; ms: number }[] } }).__ws2);

/** 🧮️ `after − before`, dropping phases that did not move. */
const diffSpans = (before: Record<string, Span>, after: Record<string, Span>) =>
  Object.entries(after)
    .map(([name, span]) => ({ name, ms: span.ms - (before[name]?.ms ?? 0), n: span.n - (before[name]?.n ?? 0) }))
    .filter((entry) => entry.n > 0 || entry.ms > 0.5)
    .sort((a, b) => b.ms - a.ms);

for (let attempt = 0; attempt < 60; attempt += 1) {
  await page.waitForTimeout(3_000);
  const current = await worlds();
  if (current.views.length > 0) { log(`booted ${JSON.stringify(current.views)}`); break; }
}
const skip = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i });
if (await skip.count()) await skip.first().click().catch(() => {});
await page.waitForTimeout(4_000);
log(`start ${JSON.stringify((await worlds()).views)}`);
log(`boot spans=${JSON.stringify(diffSpans({}, await spans()).map((entry) => `${entry.name}:${Math.round(entry.ms)}ms/${entry.n}`))}`);

/** 🔬️ V8 sampling profiler over the switch window — the only instrument that attributes the long-task
 * CPU no source-level span can see (promise continuations run inside the task that resolved them, not
 * inside the handler this probe wrapped). */
const profiler = await page.context().newCDPSession(page);
await profiler.send("Profiler.enable");
await profiler.send("Profiler.setSamplingInterval", { interval: 500 });
await profiler.send("Profiler.start");

const spansBefore = await spans();
await page.locator('[role="combobox"]').first().click({ timeout: 15_000 });
await page.locator('[role="option"]').filter({ hasText: /nakagin/i }).first().click({ timeout: 15_000 });
const clickedWall = Date.now();
const clicked = (await worlds()).now;
log(`→ nakagin at page ${clicked.toFixed(0)}ms`);

let landed = 0;
for (let second = 0; second < 160; second += 1) {
  await page.waitForTimeout(500);
  const current = await worlds();
  if (current.views.length > 0 && current.views.every((view) => view.bytes > 10_000)) { landed = current.now; log(`landed after ${((landed - clicked) / 1000).toFixed(1)}s: ${JSON.stringify(current.views)}`); break; }
}
if (landed === 0) log(`DID NOT land: ${JSON.stringify((await worlds()).views)}`);
const end = landed || (await worlds()).now;
const profile = (await profiler.send("Profiler.stop")) as { readonly profile: { readonly nodes: readonly { readonly id: number; readonly hitCount?: number; readonly callFrame: { readonly functionName: string; readonly url: string; readonly lineNumber: number } }[]; readonly startTime: number; readonly endTime: number } };
const samples = profile.profile.nodes.reduce((total, node) => total + (node.hitCount ?? 0), 0);
const selfMs = (profile.profile.endTime - profile.profile.startTime) / 1000 / Math.max(samples, 1);
const byFrame = new Map<string, number>();
for (const node of profile.profile.nodes) {
  if (!node.hitCount) continue;
  const file = node.callFrame.url.split("/").slice(-2).join("/").split("?")[0] ?? "";
  const key = `${node.callFrame.functionName || "(anonymous)"} @ ${decodeURIComponent(file)}:${node.callFrame.lineNumber + 1}`;
  byFrame.set(key, (byFrame.get(key) ?? 0) + node.hitCount);
}
log(`profile selfTop:\n${[...byFrame.entries()].sort((a, b) => b[1] - a[1]).slice(0, 30).map(([key, hits]) => `  ${(hits * selfMs).toFixed(0).padStart(7)}ms  ${key}`).join("\n")}`);

const observed = await marks();
const window_ = observed.tasks.filter((task) => task.at >= clicked && task.at <= end);
const posts = observed.posts.filter((at) => at >= clicked && at <= end);
const busy = window_.reduce((total, task) => total + task.ms, 0);
log(`gap=${((end - clicked) / 1000).toFixed(1)}s longtaskCpu=${(busy / 1000).toFixed(1)}s tasks=${window_.length} workerPosts=${posts.length}`);
const switchSpans = diffSpans(spansBefore, await spans());
log(`switch spans (ms/calls):\n${switchSpans.map((entry) => `  ${entry.name.padEnd(28)} ${entry.ms.toFixed(1).padStart(9)}ms  ×${entry.n}`).join("\n")}`);
log(`switch spanTotal=${switchSpans.filter((entry) => !entry.name.startsWith("project.nodes")).reduce((total, entry) => total + entry.ms, 0).toFixed(0)}ms`);
log(`longest=${JSON.stringify(window_.slice().sort((a, b) => b.ms - a.ms).slice(0, 10).map((task) => ({ at: Math.round(task.at - clicked), ms: Math.round(task.ms) })))}`);
const gapLines = consoleLines.filter((entry) => entry.at >= clickedWall);
const families = new Map<string, number>();
for (const entry of gapLines) { const family = entry.line.slice(0, 60); families.set(family, (families.get(family) ?? 0) + 1); }
log(`consoleFamilies=${JSON.stringify([...families.entries()].sort((a, b) => b[1] - a[1]).slice(0, 20))}`);
await page.screenshot({ path: `${out}/b4-nakagin.png` });
await browser.close();
