/** 🩺️ CPU profile of the React shell while a puzzle 3d fill run of 5000 objects lands on :6013: starts the run, records a
 * main-thread CDP profile until the run is complete and its trace applied, then prints the top self-time functions.
 * Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. Run from the ticket folder: `bun 🔍️w5-large-run-profile-probe.ts`. */
import { chromium } from "@playwright/test";
import { writeFileSync } from "node:fs";

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
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
const count = page.getByRole("spinbutton").first();
await count.fill("5000");
await count.press("Enter");
await page.waitForTimeout(2000);
const cdp = await page.context().newCDPSession(page);
await cdp.send("Profiler.enable");
await cdp.send("Profiler.setSamplingInterval", { interval: 1000 });
await cdp.send("Profiler.start");
await page.getByRole("button", { name: "Start", exact: true }).first().click({ timeout: 60000 });
await page.waitForFunction(() => Number(document.querySelector('[data-surface-id="window:puzzle3d-main-perspective"]')?.getAttribute("data-tool-run-records") ?? 0) > 1500, undefined, { timeout: 900000, polling: 1000 });
await page.waitForTimeout(5000);
const { profile } = await cdp.send("Profiler.stop") as { profile: { nodes: { id: number; callFrame: { functionName: string; url: string; lineNumber: number }; hitCount?: number; children?: number[] }[]; samples: number[]; timeDeltas: number[] } };
const self = new Map<number, number>();
profile.samples.forEach((id, index) => self.set(id, (self.get(id) ?? 0) + (profile.timeDeltas[index] ?? 0)));
const parent = new Map<number, number>();
for (const node of profile.nodes) for (const child of node.children ?? []) parent.set(child, node.id);
const byId = new Map(profile.nodes.map((node) => [node.id, node]));
const label = (id: number) => { const frame = byId.get(id)!.callFrame; return `${frame.functionName || "(anonymous)"} ${decodeURIComponent(frame.url).split("/").slice(-3).join("/")}:${frame.lineNumber + 1}`; };
const selfByLabel = new Map<string, number>();
const totalByLabel = new Map<string, number>();
for (const [id, micros] of self) {
  selfByLabel.set(label(id), (selfByLabel.get(label(id)) ?? 0) + micros);
  const seen = new Set<string>();
  for (let cursor: number | undefined = id; cursor !== undefined; cursor = parent.get(cursor)) {
    const name = label(cursor);
    if (seen.has(name)) continue;
    seen.add(name);
    totalByLabel.set(name, (totalByLabel.get(name) ?? 0) + micros);
  }
}
const top = (map: Map<string, number>) => [...map].sort((a, b) => b[1] - a[1]).slice(0, 40).map(([name, micros]) => `${(micros / 1000).toFixed(0).padStart(8)} ms  ${name}`).join("\n");
const report = `SELF\n${top(selfByLabel)}\n\nTOTAL\n${top(totalByLabel)}\n`;
writeFileSync(`${import.meta.dir}/🗑️generated/W5-mac-react-e2e/large-run-profile.txt`, report);
console.log(report);
await browser.close();
