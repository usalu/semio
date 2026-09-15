/** 🩺️ Page responsiveness around a large puzzle 3d fill run on :6013: starts fill with count 5000, then every 2 s measures
 * the evaluate round trip, the main-thread frame interval (rAF), the trace counters and the long tasks, through the run's
 * end, an abort and 30 s after it. Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS.
 * Run from the ticket folder: `bun 🔍️w5-large-run-responsiveness-probe.ts`. */
import { chromium } from "@playwright/test";

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const t0 = Date.now();
const log = (line: string) => console.log(`[${((Date.now() - t0) / 1000).toFixed(1)}s] ${line}`);
await page.goto("http://127.0.0.1:6013/?plugin=puzzle3d");
await page.waitForFunction(() => document.querySelectorAll("[data-tool-run-records]").length >= 1, undefined, { timeout: 300000 });
await page.waitForTimeout(8000);
await page.getByText("Skip", { exact: true }).first().click({ force: true, timeout: 3000 }).catch(() => {});
await page.evaluate(() => {
  const w = window as unknown as { __long: number[] };
  w.__long = [];
  new PerformanceObserver((list) => list.getEntries().forEach((entry) => w.__long.push(Math.round(entry.duration)))).observe({ type: "longtask", buffered: false });
});
const tab = page.getByRole("button", { name: "Tool runs", exact: true }).first();
if ((await tab.getAttribute("aria-pressed")) === "false") await tab.click();
const utility = () => page.evaluate(() => (JSON.parse(document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]')?.getAttribute("data-interaction-json") ?? "{}") as { activeUtility?: string }).activeUtility);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Tool", exact: true }).first().click();
await page.waitForTimeout(3000);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Fill", exact: true }).first().click();
const count = page.getByRole("spinbutton").first();
await count.fill("5000");
await count.press("Enter");
await page.waitForTimeout(2000);
await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 60000 });
log("started 5000");
const sample = async (label: string) => {
  const started = Date.now();
  const reading = await page
    .evaluate(async () => {
      const frame = await new Promise<number>((resolve) => {
        const a = performance.now();
        requestAnimationFrame(() => requestAnimationFrame(() => resolve(Math.round(performance.now() - a))));
      });
      const host = document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]');
      const status = [...document.querySelectorAll('[aria-live="polite"],[aria-live="assertive"]')].map((node) => node.textContent ?? "").find((text) => /·/.test(text)) ?? "";
      const w = window as unknown as { __long: number[] };
      const longs = w.__long.splice(0);
      return { frame, rec: host?.getAttribute("data-tool-run-records"), status, longTasks: longs.length, longestMs: Math.max(0, ...longs), totalLongMs: longs.reduce((a, b) => a + b, 0) };
    })
    .catch((error) => ({ error: String(error).slice(0, 120) }));
  log(`${label} evaluateMs=${Date.now() - started} ${JSON.stringify(reading)}`);
};
for (let tick = 0; tick < 30; tick++) {
  await sample("run");
  await page.waitForTimeout(2000);
}
await page.getByRole("button", { name: "Abort", exact: true }).first().click({ timeout: 60000 }).catch((error) => log(`abort click ${String(error).slice(0, 80)}`));
log("abort clicked");
for (let tick = 0; tick < 15; tick++) {
  await sample("after-abort");
  await page.waitForTimeout(2000);
}
await browser.close();
