/** 🩺️ Frame-level timeline of a puzzle 3d fill run in the React shell (:6013, Metal): from the Start click, every animation frame
 * records the perspective window's `data-tool-run-*` counters and provisional instance count; the console lines about refreshes,
 * typed-operation drains and tool run actions are timestamped on the same clock. Prints when the renderer first showed a record,
 * how many distinct record counts it showed and the largest jump.
 * Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. Run from the ticket folder: `bun 🔍️w5-fill-timeline-probe.ts [--count=100]`. */
import { chromium } from "@playwright/test";
import { writeFileSync } from "node:fs";

const count = process.argv.find((arg) => arg.startsWith("--count="))?.slice(8) ?? "100";
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const consoleLines: { t: number; text: string }[] = [];
page.on("console", (msg) => consoleLines.push({ t: Date.now(), text: msg.text().slice(0, 260) }));
await page.goto("http://127.0.0.1:6013/?plugin=puzzle3d");
await page.waitForFunction(() => document.querySelectorAll("[data-tool-run-records]").length >= 1, undefined, { timeout: 600000 });
console.log("booted");
await page.waitForTimeout(8000);
await page.getByText("Skip", { exact: true }).first().click({ force: true, timeout: 3000 }).catch(() => {});
const tab = page.getByRole("button", { name: "Tool runs", exact: true }).first();
if ((await tab.getAttribute("aria-pressed")) === "false") await tab.click();
const utility = () => page.evaluate(() => (JSON.parse(document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]')?.getAttribute("data-interaction-json") ?? "{}") as { activeUtility?: string }).activeUtility);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Tool", exact: true }).first().click();
await page.waitForTimeout(3000);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Fill", exact: true }).first().click();
const spin = page.getByRole("spinbutton").first();
await spin.fill(count);
await spin.press("Enter");
await page.waitForTimeout(3000);
await page.evaluate(() => {
  const w = window as unknown as { __tl: unknown[]; __tlStart: number };
  w.__tl = [];
  const tick = () => {
    const host = document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]');
    let provisional = 0;
    try {
      provisional = (JSON.parse(host?.getAttribute("data-instances-json") ?? "[]") as { provisional?: boolean }[]).filter((instance) => instance.provisional).length;
    } catch {}
    const top = document.querySelector('[data-surface-id="window:puzzle3d-main-top"]');
    w.__tl.push([Date.now(), Number(host?.getAttribute("data-tool-run-records") ?? 0), Number(host?.getAttribute("data-tool-run-danger") ?? 0), Number(host?.getAttribute("data-tool-run-success") ?? 0), provisional, Number(top?.getAttribute("data-tool-run-records") ?? -1), document.querySelectorAll("[data-surface-id^=\"window:\"][data-tool-run-records]").length]);
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
});
await page.evaluate(() => performance.clearMeasures());
const profiling = process.argv.includes("--profile");
const cdp = profiling ? await page.context().newCDPSession(page) : null;
if (cdp) {
  await cdp.send("Profiler.enable");
  await cdp.send("Profiler.setSamplingInterval", { interval: 200 });
  await cdp.send("Profiler.start");
}
const start = Date.now();
const mark = consoleLines.length;
await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 60000 });
const ended = await page.waitForFunction(() => /Complete, ready to finalize|Failed|Faulted/.test([...document.querySelectorAll('[id^="panel:framework.toolRun"]')].map((node) => (node as HTMLElement).innerText ?? "").join(" ")), undefined, { timeout: 120000, polling: 50 }).then(() => true).catch(() => false);
const endedAt = Date.now();
console.log(`ended=${ended} runMs=${endedAt - start}`);
if (cdp) {
  const { profile } = (await cdp.send("Profiler.stop")) as { profile: { nodes: { id: number; callFrame: { functionName: string; url: string; lineNumber: number }; children?: number[] }[]; samples: number[]; timeDeltas: number[] } };
  const byId = new Map(profile.nodes.map((node) => [node.id, node]));
  const parent = new Map<number, number>();
  for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
  const self = new Map<string, number>();
  const total = new Map<string, number>();
  const label = (id: number) => { const frame = byId.get(id)!.callFrame; return `${frame.functionName || "(anon)"} ${decodeURIComponent(frame.url.split("/").slice(-2).join("/")).slice(-60)}:${frame.lineNumber + 1}`; };
  profile.samples.forEach((id, index) => {
    const ms = (profile.timeDeltas[index] ?? 0) / 1000;
    self.set(label(id), (self.get(label(id)) ?? 0) + ms);
    const seen = new Set<string>();
    for (let cursor: number | undefined = id; cursor !== undefined; cursor = parent.get(cursor)) { const key = label(cursor); if (!seen.has(key)) { seen.add(key); total.set(key, (total.get(key) ?? 0) + ms); } }
  });
  const callers = new Map<string, number>();
  const hot = ["exports.jsx", "setProgram", "cloneUniforms", "set scrollLeft", "cn", "(program)", "(garbage collector)"];
  profile.samples.forEach((id, index) => {
    const ms = (profile.timeDeltas[index] ?? 0) / 1000;
    const name = byId.get(id)!.callFrame.functionName || "(anon)";
    if (!hot.includes(name)) return;
    const chain: string[] = [];
    for (let cursor = parent.get(id); cursor !== undefined && chain.length < 4; cursor = parent.get(cursor)) chain.push(label(cursor));
    const key = `${name} <- ${chain.join(" <- ")}`;
    callers.set(key, (callers.get(key) ?? 0) + ms);
  });
  const top = (map: Map<string, number>) => [...map].sort((a, b) => b[1] - a[1]).slice(0, 45).map(([key, ms]) => `${ms.toFixed(0).padStart(6)}ms ${key}`).join("\n");
  writeFileSync(`${import.meta.dir}/🗑️generated/W5-mac-react-e2e/profile-${Date.now()}.txt`, `SELF\n${top(self)}\n\nTOTAL\n${top(total)}\n\nCALLERS\n${top(callers)}\n`);
}
console.log(`panel=${JSON.stringify((await page.locator('[id^="panel:framework.toolRun"]').first().innerText().catch(() => "")).slice(0, 200))} live=${JSON.stringify(await page.evaluate(() => [...document.querySelectorAll('[aria-live="polite"],[aria-live="assertive"]')].map((node) => node.textContent ?? "").filter(Boolean).slice(0, 6)))}`);
await page.waitForTimeout(3000);
const timeline = (await page.evaluate(() => (window as unknown as { __tl: number[][] }).__tl)).filter((row) => row[0]! >= start);
const changes: string[] = [];
let previous = "";
for (const [t, records, danger, success, provisional, topRecords, windows] of timeline) {
  const key = `${records}/${danger}/${success}/${provisional}/${topRecords}`;
  if (key !== previous) changes.push(`+${t! - start}ms records=${records} danger=${danger} success=${success} provisional=${provisional} top=${topRecords} traceWindows=${windows}`);
  previous = key;
}
const relevant = consoleLines.slice(mark).filter((line) => /slice|refresh|drain|toolRun|typed-operation|Invocation|ui scope|uiScope|error|fault|stall/i.test(line.text)).map((line) => `+${line.t - start}ms ${line.text}`);
const stages = await page.evaluate(() => {
  const totals: Record<string, { count: number; totalMs: number; maxMs: number }> = {};
  for (const entry of performance.getEntriesByType("measure")) {
    if (!entry.name.startsWith("semio.hop.")) continue;
    const stage = entry.name.slice("semio.hop.".length);
    const row = (totals[stage] ??= { count: 0, totalMs: 0, maxMs: 0 });
    row.count += 1;
    row.totalMs += entry.duration;
    row.maxMs = Math.max(row.maxMs, entry.duration);
  }
  return Object.entries(totals).sort((a, b) => b[1].totalMs - a[1].totalMs).map(([stage, row]) => `${stage.padEnd(22)} n=${String(row.count).padStart(5)} total=${row.totalMs.toFixed(0).padStart(7)}ms max=${row.maxMs.toFixed(0)}ms`);
});
const records = timeline.map((row) => row[1]!);
const jumps = records.slice(1).map((value, index) => value - records[index]!);
const summary = `count=${count} frames=${timeline.length} firstRecordAt=+${(timeline.find((row) => row[1]! > 0)?.[0] ?? start) - start}ms distinctRecordCounts=${new Set(records).size} largestJump=${Math.max(0, ...jumps)} finalRecords=${records.at(-1)}`;
writeFileSync(`${import.meta.dir}/🗑️generated/W5-mac-react-e2e/timeline-${count}-${Date.now()}.txt`, `${summary}\n\nCHANGES\n${changes.join("\n")}\n\nCONSOLE\n${relevant.join("\n")}\n`);
console.log(summary);
console.log(changes.slice(0, 40).join("\n"));
console.log(stages.join("\n"));
const details = await page.evaluate(() => performance.getEntriesByType("measure").filter((entry) => /semio\.hop\.(refresh\.turn|turn\.accept|worker\.reply|worker\.guest|commit|intake\.[a-z.]+|surface\.[a-z.]+)$/.test(entry.name)).map((entry) => `${Math.round(entry.startTime)} ${entry.name.slice(10)} ${entry.duration.toFixed(0)}ms ${JSON.stringify((entry as PerformanceMeasure).detail)}`));
writeFileSync(`${import.meta.dir}/🗑️generated/W5-mac-react-e2e/timeline-details-${Date.now()}.txt`, details.join("\n"));
console.log(details.slice(0, 20).join("\n"));
console.log(relevant.slice(0, 60).join("\n"));
await browser.close();
