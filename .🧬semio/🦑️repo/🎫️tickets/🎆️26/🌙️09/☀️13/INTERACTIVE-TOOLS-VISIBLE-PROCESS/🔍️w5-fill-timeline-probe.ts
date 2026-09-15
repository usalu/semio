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
    w.__tl.push([Date.now(), Number(host?.getAttribute("data-tool-run-records") ?? 0), Number(host?.getAttribute("data-tool-run-danger") ?? 0), Number(host?.getAttribute("data-tool-run-success") ?? 0), provisional]);
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
});
const start = Date.now();
const mark = consoleLines.length;
await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 60000 });
const ended = await page.waitForFunction(() => /Complete|Failed/.test([...document.querySelectorAll('[aria-live="polite"],[aria-live="assertive"]')].map((node) => node.textContent ?? "").join(" ")), undefined, { timeout: 120000, polling: 200 }).then(() => true).catch(() => false);
console.log(`ended=${ended}`);
await page.waitForTimeout(3000);
const timeline = (await page.evaluate(() => (window as unknown as { __tl: number[][] }).__tl)).filter((row) => row[0]! >= start);
const changes: string[] = [];
let previous = "";
for (const [t, records, danger, success, provisional] of timeline) {
  const key = `${records}/${danger}/${success}/${provisional}`;
  if (key !== previous) changes.push(`+${t! - start}ms records=${records} danger=${danger} success=${success} provisional=${provisional}`);
  previous = key;
}
const relevant = consoleLines.slice(mark).filter((line) => /slice|refresh|drain|toolRun|typed-operation|Invocation|ui scope|uiScope|error|fault|stall/i.test(line.text)).map((line) => `+${line.t - start}ms ${line.text}`);
const records = timeline.map((row) => row[1]!);
const jumps = records.slice(1).map((value, index) => value - records[index]!);
const summary = `count=${count} frames=${timeline.length} firstRecordAt=+${(timeline.find((row) => row[1]! > 0)?.[0] ?? start) - start}ms distinctRecordCounts=${new Set(records).size} largestJump=${Math.max(0, ...jumps)} finalRecords=${records.at(-1)}`;
writeFileSync(`${import.meta.dir}/🗑️generated/W5-mac-react-e2e/timeline-${count}-${Date.now()}.txt`, `${summary}\n\nCHANGES\n${changes.join("\n")}\n\nCONSOLE\n${relevant.join("\n")}\n`);
console.log(summary);
console.log(changes.slice(0, 40).join("\n"));
console.log(relevant.slice(0, 60).join("\n"));
await browser.close();
