/** ⏱️ Read-only probe for ticket 26/09/02 wave W-S2: WHERE the 15–35 s between the example click and
 * the world-3d repaint is spent. Extends W-P5's `🔍️wp5-world-lane-probe.ts` (which established that
 * the scene does arrive, only late) with three page-level instruments that need no source edit:
 *
 * 1. `longtask` PerformanceObserver — main-thread CPU actually burnt in the gap (intake, projection,
 *    React commit) versus time spent waiting.
 * 2. A `Worker.prototype.postMessage` wrapper — one entry per host→shard message, i.e. one per guest
 *    turn round trip, so the gap can be split into "round trips" and "work".
 * 3. Console capture with page-relative timestamps — the runtime's own `[DEBUG]` marks.
 *
 * Drives a SEPARATE headless chromium against an already-running dev target; it never starts a server
 * and never touches an interactive browser tab.
 *
 * `bun 🔍️ws2-scene-latency-probe.ts [url] [--out <dir>]`
 */
import { chromium } from "../../../../../../../node_modules/playwright/index.mjs";

const url = process.argv.find((argument) => argument.startsWith("http")) ?? "http://127.0.0.1:6013/";
const out = process.argv.includes("--out") ? process.argv[process.argv.indexOf("--out") + 1]! : ".";
const started = Date.now();
const log = (line: string) => console.log(`[${((Date.now() - started) / 1000).toFixed(1)}s] ${line}`);

const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1400, height: 900 } });
page.setDefaultTimeout(180_000);
await page.addInitScript(() => {
  const state = { posts: [] as number[], tasks: [] as { readonly at: number; readonly ms: number }[] };
  (window as unknown as { __ws2: typeof state }).__ws2 = state;
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (this: Worker, ...args: unknown[]) {
    state.posts.push(performance.now());
    return (post as (...rest: unknown[]) => unknown).apply(this, args);
  } as typeof Worker.prototype.postMessage;
  try {
    new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) state.tasks.push({ at: entry.startTime, ms: entry.duration });
    }).observe({ entryTypes: ["longtask"] });
  } catch {
    /* longtask unsupported */
  }
  /** ⏱️ Blocking sampler. A self-reposting `MessageChannel` would saturate the macrotask queue and
   * change what it measures (measured: the page never finished booting), so this samples on a 10 ms
   * timer and counts only the lag beyond that period as main-thread blocking. */
  const busy = { blocked: 0, ticks: 0, last: performance.now() };
  (window as unknown as { __ws2busy: typeof busy }).__ws2busy = busy;
  setInterval(() => {
    const now = performance.now();
    const lag = now - busy.last - 10;
    if (lag > 2) busy.blocked += lag;
    busy.ticks += 1;
    busy.last = now;
  }, 10);
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

const marks = async () => page.evaluate(() => ({ ...(window as unknown as { __ws2: { posts: number[]; tasks: { at: number; ms: number }[] } }).__ws2, busy: (window as unknown as { __ws2busy: { blocked: number; ticks: number } }).__ws2busy }));

for (let attempt = 0; attempt < 60; attempt += 1) {
  await page.waitForTimeout(3_000);
  const current = await worlds();
  if (current.views.length > 0) { log(`booted ${JSON.stringify(current.views)}`); break; }
}
const skip = page.locator("button", { hasText: /^\s*(x\s*)?skip\s*$/i });
if (await skip.count()) await skip.first().click().catch(() => {});
await page.waitForTimeout(4_000);
log(`start ${JSON.stringify((await worlds()).views)}`);

const busyBefore = (await marks()).busy;
await page.locator('[role="combobox"]').first().click({ timeout: 15_000 });
await page.locator('[role="option"]').filter({ hasText: /nakagin/i }).first().click({ timeout: 15_000 });
const clickedWall = Date.now();
const clicked = (await worlds()).now;
log(`→ nakagin at page ${clicked.toFixed(0)}ms`);

let landed = 0;
for (let second = 0; second < 120; second += 1) {
  await page.waitForTimeout(500);
  const current = await worlds();
  if (current.views.length > 0 && current.views.every((view) => view.bytes > 10_000)) { landed = current.now; log(`landed after ${((landed - clicked) / 1000).toFixed(1)}s: ${JSON.stringify(current.views)}`); break; }
}
if (landed === 0) log(`DID NOT land: ${JSON.stringify((await worlds()).views)}`);
const end = landed || (await worlds()).now;
const observed = await marks();
const window_ = observed.tasks.filter((task) => task.at >= clicked && task.at <= end);
const posts = observed.posts.filter((at) => at >= clicked && at <= end);
const busy = window_.reduce((total, task) => total + task.ms, 0);
log(`gap=${((end - clicked) / 1000).toFixed(1)}s longtaskCpu=${(busy / 1000).toFixed(1)}s tasks=${window_.length} workerPosts=${posts.length} mainThreadBusy=${((observed.busy.blocked - busyBefore.blocked) / 1000).toFixed(1)}s ticks=${observed.busy.ticks - busyBefore.ticks}`);
const gapLines = consoleLines.filter((entry) => entry.at >= clickedWall);
const families = new Map<string, number>();
for (const entry of gapLines) { const family = entry.line.slice(0, 60); families.set(family, (families.get(family) ?? 0) + 1); }
log(`consoleFamilies=${JSON.stringify([...families.entries()].sort((a, b) => b[1] - a[1]).slice(0, 25))}`);
log(`longest=${JSON.stringify(window_.slice().sort((a, b) => b.ms - a.ms).slice(0, 10).map((task) => ({ at: Math.round(task.at - clicked), ms: Math.round(task.ms) })))}`);
const buckets = new Map<number, { tasks: number; ms: number; posts: number }>();
for (const task of window_) {
  const bucket = Math.floor((task.at - clicked) / 1000);
  const entry = buckets.get(bucket) ?? { tasks: 0, ms: 0, posts: 0 };
  buckets.set(bucket, { tasks: entry.tasks + 1, ms: entry.ms + task.ms, posts: entry.posts });
}
for (const at of posts) {
  const bucket = Math.floor((at - clicked) / 1000);
  const entry = buckets.get(bucket) ?? { tasks: 0, ms: 0, posts: 0 };
  buckets.set(bucket, { ...entry, posts: entry.posts + 1 });
}
log(`perSecond=${JSON.stringify([...buckets.entries()].sort((a, b) => a[0] - b[0]).map(([second, entry]) => `${second}s:${Math.round(entry.ms)}ms/${entry.tasks}t/${entry.posts}p`))}`);
log(`gap console (${gapLines.length} messages):\n${gapLines.filter((entry) => !/^error\|[\s,\[\]{}0-9:]*$/.test(entry.line)).map((entry) => `+${((entry.at - clickedWall) / 1000).toFixed(1)}s|${entry.line}`).slice(0, 200).join("\n")}`);
await page.screenshot({ path: `${out}/ws2-nakagin.png` });
await browser.close();
