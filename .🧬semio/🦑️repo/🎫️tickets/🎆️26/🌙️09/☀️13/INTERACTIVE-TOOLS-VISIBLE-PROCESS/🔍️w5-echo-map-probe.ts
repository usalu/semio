/** 🩺️ Reads the React trace cursor echo map during a live puzzle 3d fill run on :6013: the module-level map
 * `toolRunTraceCursorViewState` answers from, loaded through the page's own vite module URL.
 * Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS. Run from the ticket folder: `bun 🔍️w5-echo-map-probe.ts`. */
import { chromium } from "@playwright/test";

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--enable-gpu", "--ignore-gpu-blocklist"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.routeWebSocket(/\/\?token=/, () => {});
const modules: string[] = [];
page.on("request", (request) => { if (request.url().includes("tool-run-trace")) modules.push(decodeURIComponent(request.url())); });
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
await page.waitForTimeout(4000);
console.log("modules", JSON.stringify([...new Set(modules)]));
for (const url of new Set(modules)) {
  const result = await page.evaluate(async (href) => {
    const module = await import(/* @vite-ignore */ new URL(href).pathname + new URL(href).search);
    return { keys: Object.keys(module).filter((key) => /Cursor/.test(key)), map: module.toolRunTraceCursorViewState?.(["puzzle3d-main-top", "puzzle3d-main-perspective"]) };
  }, url).catch((error) => String(error));
  console.log("module", url.slice(-120), JSON.stringify(result));
}
await browser.close();
