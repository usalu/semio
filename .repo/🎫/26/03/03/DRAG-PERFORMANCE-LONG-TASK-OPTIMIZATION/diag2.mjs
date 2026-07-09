import { chromium } from "playwright";
import path from "path";
import { fileURLToPath } from "url";
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const zipPath = "/workspaces/semio/assets/compose/metabolism.zip";
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox", "--disable-gpu"] });
const page = await browser.newPage();
const debugLogs = [];
let pieceNodeRenderCount = 0;
page.on("console", (msg) => {
  const txt = msg.text();
  if (txt.includes("[DEBUG] PieceNode render")) pieceNodeRenderCount++;
  if (txt.includes("[DEBUG]")) debugLogs.push({ t: Date.now(), type: msg.type(), txt: txt.slice(0, 400) });
});
await page.addInitScript(() => {
  window.__COMPOSE_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
  const store = window.__COMPOSE_PERFORMANCE__;
  const oc = window.PerformanceObserver;
  const types = oc?.supportedEntryTypes ?? [];
  if (!oc || !types.includes("longtask")) return;
  store.longTaskSupported = true;
  const obs = new oc((list) => {
    const entries = list.getEntries().map((e) => ({ duration: e.duration, startTime: e.startTime }));
    store.longTasks.push(...entries);
  });
  obs.observe({ entryTypes: ["longtask"] });
});
await page.goto("http://127.0.0.1:5173/", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(2000);
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });
const [fileChooser] = await Promise.all([page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent("click")]);
if (fileChooser) {
  await fileChooser.setFiles(zipPath);
} else {
  await fileInput.setInputFiles(zipPath);
  await fileInput.evaluate((el) => el.dispatchEvent(new Event("change", { bubbles: true })));
}
const metText = page.getByText("Metabolism", { exact: true }).first();
await metText.waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
const isVis = await tableRow.isVisible().catch(() => false);
if (isVis) {
  await tableRow.dblclick({ force: true });
} else {
  await metText.dblclick({ force: true });
}
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(5000);
// Now navigate to design
const designRowIds = await page.evaluate(() =>
  Array.from(document.querySelectorAll("[data-row-id]"))
    .map((el) => el.getAttribute("data-row-id"))
    .filter((id) => id?.startsWith("design-"))
    .slice(0, 5),
);
console.log("designRowIds:", designRowIds);
if (designRowIds.length > 0) {
  const rowId = designRowIds.find((id) => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
  await page.evaluate((rid) => {
    const row = document.querySelector(`[data-row-id="${rid}"]`);
    if (row) {
      row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
    }
  }, rowId);
}
await page.waitForTimeout(8000);
const diag = page.locator("#diagram .react-flow").first();
await diag.waitFor({ state: "visible", timeout: 60000 });
const nodes = diag.locator(".react-flow__node");
await nodes.first().waitFor({ state: "attached", timeout: 60000 });
for (let i = 0; i < 10; i++) {
  await page.waitForTimeout(2000);
  if ((await nodes.count()) >= 170) break;
}
await page.waitForTimeout(3000);
console.log("nodes:", await nodes.count());
// Close panel
const toggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await toggle.isVisible().catch(() => false)) {
  const leftOpen = await page
    .locator('[data-panel="leftSidePanel"]')
    .isVisible()
    .catch(() => false);
  if (leftOpen) {
    await toggle.click();
    await page.waitForTimeout(500);
  }
}
// Clear long tasks & debug logs
await page.evaluate(() => {
  window.__COMPOSE_PERFORMANCE__.longTasks = [];
});
debugLogs.length = 0;
pieceNodeRenderCount = 0;
// Zoom
const pane = diag.locator(".react-flow__pane").first();
const paneBox = await pane.boundingBox();
const cx = paneBox.x + paneBox.width / 2,
  cy = paneBox.y + paneBox.height / 2;
await page.mouse.move(cx, cy);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
// Clear again after zoom
await page.evaluate(() => {
  window.__COMPOSE_PERFORMANCE__.longTasks = [];
});
await page.evaluate(() => {
  if (window.__COMPOSE_DEBUG__) window.__COMPOSE_DEBUG__.pieceNodeRenders = 0;
});
debugLogs.length = 0;
pieceNodeRenderCount = 0;
// Drag
const firstNode = nodes.first();
const box = await firstNode.boundingBox();
const sx = box.x + box.width / 2,
  sy = box.y + box.height / 2;
await page.mouse.move(sx, sy);
await page.waitForTimeout(100);
const t0 = Date.now();
await page.mouse.down();
await page.waitForTimeout(50);
await page.mouse.move(sx + 100, sy, { steps: 20 });
await page.mouse.up();
await page.waitForTimeout(6000);
const longTasks = await page.evaluate(() => window.__COMPOSE_PERFORMANCE__.longTasks);
const maxLT = longTasks.length > 0 ? Math.max(...longTasks.map((e) => e.duration)) : 0;
console.log(`\n=== Long tasks: ${longTasks.length}, max: ${maxLT.toFixed(1)}ms ===`);
for (const lt of longTasks.slice(0, 10)) {
  console.log(`  start: ${lt.startTime.toFixed(1)}ms, dur: ${lt.duration.toFixed(1)}ms`);
}
if (longTasks.length > 10) console.log(`  ... and ${longTasks.length - 10} more`);
console.log(`\n=== Debug logs during drag (${debugLogs.length}) ===`);
for (const log of debugLogs.slice(0, 30)) {
  console.log(`  +${log.t - t0}ms [${log.type}]: ${log.txt}`);
}
if (debugLogs.length > 30) console.log(`  ... and ${debugLogs.length - 30} more`);
console.log(`\n=== PieceNode renders during drag (from console): ${pieceNodeRenderCount} ===`);
const debugCounters = await page.evaluate(() => window.__COMPOSE_DEBUG__ ?? {});
console.log(`=== PieceNode renders during drag (from counter): ${debugCounters.pieceNodeRenders ?? "N/A"} ===`);
// Sort long tasks by duration and show top 5
const sortedByDur = longTasks
  .slice()
  .sort((a, b) => b.duration - a.duration)
  .slice(0, 5);
console.log(`\n=== Top 5 long tasks by duration ===`);
for (const lt of sortedByDur) {
  console.log(`  start: ${lt.startTime.toFixed(1)}ms, dur: ${lt.duration.toFixed(1)}ms`);
}
await browser.close();
