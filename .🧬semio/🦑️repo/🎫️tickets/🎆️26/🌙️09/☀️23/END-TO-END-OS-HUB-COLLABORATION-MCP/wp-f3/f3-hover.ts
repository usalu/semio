/** 🖱️ F3 — hover latency on a world-3d surface: opens a program, finds a point whose hover paints an id, then moves the
 * pointer on/off it N times; per hover: pointermove timestamp → `data-hover-paint-id` set (the painted local hover) → end of
 * the next frame; plus `semio.hop.*` measures and long tasks during the run. usage: bun f3-hover.ts <baseUrl> <pluginId> <appId> <windowIdSuffix> [n] */
import { chromium } from "playwright";
import { writeFileSync } from "node:fs";
import { loadavg } from "node:os";
import { awaitBeacon, dismissIntroduction, windowIds } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const [baseUrl, pluginId, appId, windowSuffix, nArg] = process.argv.slice(2) as [string, string, string, string, string?];
const hovers = Number(nArg ?? 12);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
  await page.addInitScript(() => {
    const state = { armed: false, moves: [] as { t: number }[], paints: [] as { t: number; id: string | null }[], frames: [] as number[], longTasks: [] as number[] };
    Object.defineProperty(window, "__f3Hover", { value: state });
    addEventListener("pointermove", (event) => { if (state.armed && event.isTrusted) state.moves.push({ t: event.timeStamp }); }, { capture: true, passive: true });
    new PerformanceObserver((list) => { if (state.armed) for (const entry of list.getEntries()) state.longTasks.push(Math.round(entry.duration)); }).observe({ type: "longtask" });
  });
  await page.goto(baseUrl, { waitUntil: "commit", timeout: 300_000 });
  await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  const before = await windowIds(page);
  await page.keyboard.press("Meta+p");
  const input = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await input.waitFor({ state: "visible", timeout: 15_000 });
  await input.fill(/^s\.[^.]+\.([^@]+)@/u.exec(appId)![1]!);
  await page.waitForTimeout(1_500);
  for (const id of [`spawn.${pluginId}.${appId}`, `spawn.${pluginId}`]) {
    const item = page.locator(`[data-slot="command-item"][data-command-item-id="${id}"]`).first();
    if ((await item.count()) > 0) { await item.click({ force: true }); break; }
  }
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline && (await windowIds(page)).filter((id) => !before.includes(id)).length === 0) await page.waitForTimeout(500);
  await page.waitForTimeout(8_000);
  const host = page.locator(`[id$="${windowSuffix}"] .semio-world-3d-host`).first();
  await host.waitFor({ state: "visible", timeout: 60_000 });
  const box = (await host.boundingBox())!;
  await page.evaluate((suffix) => {
    const root = document.querySelector(`[id$="${suffix}"] .semio-world-3d-host`)!;
    const state = (window as unknown as { __f3Hover: { armed: boolean; paints: { t: number; id: string | null }[] } }).__f3Hover;
    new MutationObserver(() => { if (state.armed) state.paints.push({ t: performance.now(), id: root.getAttribute("data-hover-paint-id") }); }).observe(root, { attributes: true, attributeFilter: ["data-hover-paint-id"] });
  }, windowSuffix);
  let target: { x: number; y: number } | null = null;
  for (let gy = 0.3; gy <= 0.8 && !target; gy += 0.05) for (let gx = 0.2; gx <= 0.8 && !target; gx += 0.05) {
    const x = box.x + box.width * gx, y = box.y + box.height * gy;
    await page.mouse.move(x, y);
    await page.waitForTimeout(120);
    if (await page.evaluate((suffix) => document.querySelector(`[id$="${suffix}"] .semio-world-3d-host`)!.getAttribute("data-hover-paint-id"), windowSuffix)) target = { x, y };
  }
  if (!target) throw new Error("no hoverable point found");
  const empty = { x: box.x + 8, y: box.y + box.height - 60 };
  await page.mouse.move(empty.x, empty.y);
  await page.waitForTimeout(1_500);
  const cdp = await page.context().newCDPSession(page);
  await cdp.send("Profiler.enable");
  await cdp.send("Profiler.setSamplingInterval", { interval: 500 });
  await cdp.send("Profiler.start");
  await page.waitForTimeout(2_000);
  await page.evaluate(() => { performance.clearMeasures(); (window as unknown as { __f3Hover: { armed: boolean } }).__f3Hover.armed = true; });
  for (let index = 0; index < hovers; index += 1) {
    await page.mouse.move(target.x, target.y);
    await page.waitForTimeout(400);
    await page.mouse.move(empty.x, empty.y);
    await page.waitForTimeout(400);
  }
  const { profile } = (await cdp.send("Profiler.stop")) as { profile: { nodes: { id: number; callFrame: { functionName: string; url: string; lineNumber: number }; children?: number[] }[]; samples: number[]; timeDeltas: number[] } };
  const byId = new Map(profile.nodes.map((node) => [node.id, node]));
  const parent = new Map<number, number>();
  for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
  const self = new Map<string, number>(), inclusive = new Map<string, number>();
  const keyOf = (frame: { functionName: string; url: string; lineNumber: number }) => `${frame.functionName || "(anon)"} ${decodeURIComponent(frame.url.split("?")[0]!.split("/").slice(-3).join("/")).slice(0, 80)}:${frame.lineNumber + 1}`;
  profile.samples.forEach((id, index) => {
    const delta = profile.timeDeltas[index] ?? 0;
    const leaf = byId.get(id)!.callFrame;
    self.set(keyOf(leaf), (self.get(keyOf(leaf)) ?? 0) + delta);
    const seen = new Set<string>();
    for (let cursor: number | undefined = id; cursor !== undefined; cursor = parent.get(cursor)) {
      const frame = byId.get(cursor)!.callFrame;
      if (!frame.url) continue;
      const key = keyOf(frame);
      if (seen.has(key)) continue;
      seen.add(key);
      inclusive.set(key, (inclusive.get(key) ?? 0) + delta);
    }
  });
  const top = (map: Map<string, number>, n: number) => [...map].sort((a, b) => b[1] - a[1]).slice(0, n).map(([key, value]) => `${Math.round(value / 1000)}ms ${key}`);
  const cpu = { self: top(self, 60), inclusive: top(inclusive, 400) };
  const result = await page.evaluate(() => {
    const state = (window as unknown as { __f3Hover: { armed: boolean; moves: { t: number }[]; paints: { t: number; id: string | null }[]; longTasks: number[] } }).__f3Hover;
    state.armed = false;
    const hops: Record<string, number[]> = {};
    for (const entry of performance.getEntriesByType("measure")) if (entry.name.startsWith("semio.hop")) (hops[entry.name] ??= []).push(Math.round(entry.duration));
    return { moves: state.moves.map((row) => row.t), paints: state.paints, longTasks: state.longTasks, hops };
  });
  const latencies = result.paints.map((paint) => { const cause = [...result.moves].reverse().find((t) => t <= paint.t); return cause === undefined ? null : +(paint.t - cause).toFixed(1); }).filter((value): value is number => value !== null);
  const sorted = [...latencies].sort((a, b) => a - b);
  const q = (p: number) => sorted.length ? sorted[Math.min(sorted.length - 1, Math.floor(p * sorted.length))] : null;
  const timeline = [...result.moves.map((t) => ({ t: Math.round(t), what: "move" })), ...result.paints.map((row) => ({ t: Math.round(row.t), what: `paint ${row.id}` }))].sort((a, b) => a.t - b.t);
  const report = { cpu, timeline: timeline.slice(0, 80), appId, windowSuffix, target, hovers, load: +loadavg()[0]!.toFixed(1), paintChanges: result.paints.length, hoverToPaint: { p50: q(0.5), p95: q(0.95), max: q(1) }, latencies, longTasks: result.longTasks, hops: Object.fromEntries(Object.entries(result.hops).map(([name, values]) => [name, { n: values.length, p50: [...values].sort((a, b) => a - b)[Math.floor(values.length / 2)], max: Math.max(...values) }])) };
  writeFileSync(`/Users/ueli/Documents/semio/.tmp-ticket/wp-f3/generated/f3-hover-${pluginId}.json`, JSON.stringify(report, null, 1));
  console.log(JSON.stringify(report, null, 1));
} finally {
  await browser.close();
}
