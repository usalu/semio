/**
 * @emoji 🔍 Runtime probe: boot demonstrator and assert loading chrome markers (no text placeholder row).
 * Run: `bun .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️04/CONSISTENT-LOADING-SKELETONS-AND-BORDERS-ACROSS-THE-UI/probe-loading-chrome.mjs`
 */
import { chromium } from "playwright";

const baseUrl = process.env.DEMONSTRATOR_URL ?? "http://127.0.0.1:5173/";

const browser = await chromium.launch();
const page = await browser.newPage();
const logs = [];
page.on("console", (msg) => logs.push(`[${msg.type()}] ${msg.text()}`));

await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 60_000 });
await page.waitForTimeout(3000);

const markers = await page.evaluate(() => ({
  loadingPluginsText: document.body.innerText.includes("Loading plugins"),
  wirdVorbereitetRow: /wird vorbereitet/.test(document.body.innerText),
  canvasSkeleton: Boolean(document.querySelector('[role="status"][aria-busy="true"]')),
  loadingStatusAttr: document.querySelectorAll('[data-ui-status="loading"]').length,
  silhouetteLoading: document.querySelectorAll('[data-window-silhouette-border][data-kind="loading"]').length,
}));

console.log("[DEBUG] markers", JSON.stringify(markers, null, 2));
console.log("[DEBUG] console tail", logs.slice(-20).join("\n"));

const ok = !markers.loadingPluginsText && !markers.wirdVorbereitetRow && markers.canvasSkeleton;
process.exitCode = ok ? 0 : 1;
await browser.close();
