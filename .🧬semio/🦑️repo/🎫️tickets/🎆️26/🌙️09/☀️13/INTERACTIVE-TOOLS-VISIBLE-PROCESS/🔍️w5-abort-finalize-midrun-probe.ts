/** 🩺️ Mid-run Abort and Finalize of a puzzle 3d fill in the React shell (:6013, Metal). For each action: start a long fill,
 * wait until placements appear, press the panel button while the run is still running, then sample for a few seconds the
 * panel status, the Finalize/Abort button states, the committed and provisional instance counts, and every console
 * error/warning. Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. Run from the ticket folder:
 * `bun 🔍️w5-abort-finalize-midrun-probe.ts [--count=2000] [--wait=1500]`. */
import { chromium, type Page } from "@playwright/test";
import { writeFileSync } from "node:fs";

const count = process.argv.find((arg) => arg.startsWith("--count="))?.slice(8) ?? "2000";
const wait = Number(process.argv.find((arg) => arg.startsWith("--wait="))?.slice(7) ?? "1500");
const out = `${import.meta.dir}/🗑️generated/W5-mac-react-e2e`;
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const t0 = Date.now();
const lines: string[] = [];
page.on("console", (msg) => {
  if (msg.type() === "error" || /toolRun|tool run|reject|fault|stale|illegal/i.test(msg.text())) lines.push(`+${Date.now() - t0}ms ${msg.type()} ${msg.text().slice(0, 300)}`);
});
page.on("pageerror", (error) => lines.push(`+${Date.now() - t0}ms pageerror ${String(error).slice(0, 300)}`));
await page.goto("http://127.0.0.1:6013/?plugin=puzzle3d");
await page.waitForFunction(() => document.querySelectorAll("[data-tool-run-records]").length >= 1, undefined, { timeout: 600000 });
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
await page.waitForTimeout(2000);

type Sample = { t: number; panel: string; finalizeDisabled: boolean | null; abortDisabled: boolean | null; committed: number; provisional: number };
const sample = (page: Page, start: number): Promise<Sample> =>
  page.evaluate((start) => {
    const panel = document.querySelector('[id^="panel:framework.toolRun"]') as HTMLElement | null;
    const button = (name: string) => [...(panel?.querySelectorAll("button") ?? [])].find((node) => node.textContent?.trim() === name) as HTMLButtonElement | undefined;
    const state = (node: HTMLButtonElement | undefined) => (node ? node.disabled || node.getAttribute("aria-disabled") === "true" : null);
    let instances: { provisional?: boolean }[] = [];
    try {
      instances = JSON.parse(document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]')?.getAttribute("data-instances-json") ?? "[]");
    } catch {}
    return { t: Date.now() - start, panel: (panel?.innerText ?? "").split("\n").filter(Boolean).slice(0, 3).join(" | "), finalizeDisabled: state(button("Finalize")), abortDisabled: state(button("Abort")), committed: instances.filter((instance) => !instance.provisional).length, provisional: instances.filter((instance) => instance.provisional).length };
  }, start);

const report: string[] = [];
const only = process.argv.find((arg) => arg.startsWith("--only="))?.slice(7);
for (const action of (["Abort", "Finalize"] as const).filter((candidate) => !only || candidate === only)) {
  const before = await sample(page, Date.now());
  report.push(`== ${action}: before start ${JSON.stringify(before)}`);
  const started = Date.now();
  await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 30000 }).catch((error) => report.push(`start click failed: ${String(error).slice(0, 200)}`));
  await page.waitForTimeout(wait);
  const pressed = await sample(page, started);
  report.push(`before ${action} ${JSON.stringify(pressed)}`);
  const clicked = Date.now();
  await page.locator('[id^="panel:framework.toolRun"] button', { hasText: action }).first().click({ timeout: 5000 }).then(() => report.push(`${action} clicked`), (error) => report.push(`${action} click failed: ${String(error).slice(0, 200)}`));
  let last = "";
  for (let index = 0; index < 40; index += 1) {
    const now = await sample(page, clicked);
    const key = JSON.stringify({ ...now, t: 0 });
    if (key !== last) report.push(`after ${action} ${JSON.stringify(now)}`);
    last = key;
    await page.waitForTimeout(250);
  }
  await page.locator('[id^="panel:framework.toolRun"] button', { hasText: "Dismiss" }).first().click({ timeout: 2000 }).catch(() => {});
  await page.waitForTimeout(1500);
}
report.push("== console", ...lines);
writeFileSync(`${out}/abort-finalize-midrun-${Date.now()}.txt`, report.join("\n"));
console.log(report.join("\n"));
await browser.close();
