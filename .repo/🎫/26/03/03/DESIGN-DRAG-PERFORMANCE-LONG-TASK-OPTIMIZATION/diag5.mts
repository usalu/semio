import { chromium } from "playwright";
import path from "path";

const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
const page = await context.newPage();

await page.addInitScript(() => {
  (window as any).__COMPOSE_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
  const store = (window as any).__COMPOSE_PERFORMANCE__;
  const obs = (window as any).PerformanceObserver;
  if (!obs || !(obs.supportedEntryTypes ?? []).includes("longtask")) return;
  store.longTaskSupported = true;
  new obs((list: any) => {
    store.longTasks.push(...list.getEntries().map((e: any) => ({ duration: e.duration, startTime: e.startTime })));
  }).observe({ entryTypes: ["longtask"] });
});

await page.goto("http://127.0.0.1:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(2000);

const zipPath = path.resolve("/workspaces/semio/assets/compose/metabolism.zip");
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });

const [fileChooser] = await Promise.all([
  page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
  fileInput.dispatchEvent("click"),
]);
if (fileChooser) await fileChooser.setFiles(zipPath);
else await fileInput.setInputFiles(zipPath);

await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);

const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
await tableRow.dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(3000);
console.log(`Kit URL: ${page.url()}`);

const designRowIds = await page.evaluate(() =>
  Array.from(document.querySelectorAll("[data-row-id]")).map(el => el.getAttribute("data-row-id")).filter(id => id?.startsWith("design-"))
);
const nakaginRowId = designRowIds.find(id => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
console.log(`Clicking design: ${nakaginRowId}`);
await page.evaluate((rowId) => {
  const row = document.querySelector(`[data-row-id="${rowId}"]`);
  if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
}, nakaginRowId);
await page.waitForLoadState("networkidle");
await page.waitForTimeout(8000);

const nodeCount = await page.locator("#diagram .react-flow__node").count();
console.log(`Nodes: ${nodeCount}`);
if (nodeCount === 0) { console.log("ERROR: No nodes"); await browser.close(); process.exit(1); }

await page.waitForTimeout(5000);

const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await leftPanelToggle.isVisible().catch(() => false)) {
  const leftPanelOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
  if (leftPanelOpen) { await leftPanelToggle.click(); await page.waitForTimeout(500); }
}

await page.evaluate(() => { (window as any).__COMPOSE_PERFORMANCE__.longTasks = []; });

const diagramBox = await page.locator("#diagram .react-flow__pane").first().boundingBox();
if (!diagramBox) { console.log("No pane"); await browser.close(); process.exit(1); }

console.log("\n=== ZOOM IN ===");
await page.mouse.move(diagramBox.x + diagramBox.width / 2, diagramBox.y + diagramBox.height / 2);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);

let tasks: any[] = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks]; s.longTasks = [];
  return t.map((e: any) => ({ duration: e.duration, startTime: e.startTime }));
});
console.log(`  ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map(t => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`    ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}`);

console.log("\n=== ZOOM OUT ===");
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);

tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks]; s.longTasks = [];
  return t.map((e: any) => ({ duration: e.duration, startTime: e.startTime }));
});
console.log(`  ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map(t => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`    ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}`);

console.log("\n=== MOUSEDOWN ON NODE ===");
const firstNode = page.locator("#diagram .react-flow__node").first();
const nb = await firstNode.boundingBox();
if (!nb) { console.log("No node box"); await browser.close(); process.exit(1); }
await page.mouse.move(nb.x + nb.width / 2, nb.y + nb.height / 2);
await page.waitForTimeout(50);
await page.mouse.down();
await page.waitForTimeout(100);

tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks]; s.longTasks = [];
  return t.map((e: any) => ({ duration: e.duration, startTime: e.startTime }));
});
console.log(`  ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map(t => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`    ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}`);

console.log("\n=== DRAG 100px ===");
await page.mouse.move(nb.x + nb.width / 2 + 100, nb.y + nb.height / 2, { steps: 20 });
await page.waitForTimeout(100);

tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks]; s.longTasks = [];
  return t.map((e: any) => ({ duration: e.duration, startTime: e.startTime }));
});
console.log(`  ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map(t => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`    ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}`);

console.log("\n=== MOUSEUP ===");
await page.mouse.up();
await page.waitForTimeout(200);

tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks]; s.longTasks = [];
  return t.map((e: any) => ({ duration: e.duration, startTime: e.startTime }));
});
console.log(`  ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map(t => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`    ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}`);

console.log("\n=== SETTLE 1s ===");
await page.waitForTimeout(1000);
tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks]; s.longTasks = [];
  return t.map((e: any) => ({ duration: e.duration, startTime: e.startTime }));
});
console.log(`  ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map(t => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`    ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}`);

await browser.close();
console.log("\nDone.");
