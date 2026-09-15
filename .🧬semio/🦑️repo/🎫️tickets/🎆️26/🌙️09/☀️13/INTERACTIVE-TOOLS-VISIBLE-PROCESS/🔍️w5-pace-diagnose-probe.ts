/** 🩺️ Pace stall diagnosis for the puzzle 3d fill ToolRun (:6013): starts fill from the ToolRun panel, then for 30 s logs
 * every World3d window's `data-tool-run-*` counters and the console lines about `toolRunPace` wakes and trace cursors.
 * Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. Run from the ticket folder: `bun 🔍️w5-pace-diagnose-probe.ts`. */
import { chromium } from "@playwright/test";
import { writeFileSync } from "node:fs";

const out = `${import.meta.dir}/🗑️generated/W5-mac-react-e2e/pace-diagnose-${new Date().toISOString().replace(/[:.]/g, "-")}.txt`;
const lines: string[] = [];
const t0 = Date.now();
const log = (line: string) => lines.push(`[${((Date.now() - t0) / 1000).toFixed(1)}s] ${line}`);
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
page.on("console", (msg) => {
  const text = msg.text();
  if (/toolRunPace presented|SemioFault|panicked/i.test(text)) log(`console ${text.slice(0, 300)}`);
});
await page.goto("http://127.0.0.1:6013/?plugin=puzzle3d");
await page.waitForFunction(() => document.querySelectorAll("[data-tool-run-records]").length >= 1, undefined, { timeout: 300000 });
await page.waitForTimeout(8000);
await page.getByText("Skip", { exact: true }).first().click({ force: true, timeout: 3000 }).catch(() => {});
const tab = page.getByRole("button", { name: "Tool runs", exact: true }).first();
if ((await tab.getAttribute("aria-pressed")) === "false") await tab.click();
const utility = () => page.evaluate(() => (JSON.parse(document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]')?.getAttribute("data-interaction-json") ?? "{}") as { activeUtility?: string }).activeUtility);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Tool", exact: true }).first().click();
await page.waitForTimeout(3000);
if ((await utility()) !== "fill") await page.getByRole("button", { name: "Fill", exact: true }).first().click();
await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 60000 });
log("clicked Start");
for (let tick = 0; tick < 60; tick++) {
  const hosts = await page.evaluate(() => [...document.querySelectorAll("[data-tool-run-records]")].map((host) => `${host.getAttribute("data-surface-id")}: run=${host.getAttribute("data-tool-run-run")} page=${host.getAttribute("data-tool-run-page")} rec=${host.getAttribute("data-tool-run-records")} t=${host.getAttribute("data-tool-run-testing")} s=${host.getAttribute("data-tool-run-success")} d=${host.getAttribute("data-tool-run-danger")}`).join(" | ")).catch((error) => String(error));
  log(`hosts ${hosts}`);
  await page.waitForTimeout(500);
}
writeFileSync(out, `${lines.join("\n")}\n`);
console.log(out);
await browser.close();
