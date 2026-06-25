import { chromium } from "playwright";
import path from "path";
const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
const page = await browser.newPage();
const errors: string[] = [];
page.on('pageerror', (err) => errors.push(`PAGE_ERROR: ${err.message}`));
page.on('console', (msg) => {
  if (msg.type() === 'error') errors.push(`CONSOLE_ERROR: ${msg.text()}`);
});
await page.goto("http://127.0.0.1:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(2000);
const zipPath = path.resolve(process.cwd(), "assets/compose/metabolism.zip");
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: 'attached', timeout: 10000 });
const [fileChooser] = await Promise.all([
  page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
  fileInput.dispatchEvent("click")
]);
if (fileChooser) {
  await fileChooser.setFiles(zipPath);
} else {
  await fileInput.setInputFiles(zipPath);
  await fileInput.evaluate((el) => { el.dispatchEvent(new Event("change", { bubbles: true })); });
}
const metabolismText = page.getByText("Metabolism", { exact: true }).first();
await metabolismText.waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
await tableRow.dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(2000);
const designRowIds = await page.evaluate(() =>
  Array.from(document.querySelectorAll('[data-row-id^="design-"]')).map(el => el.getAttribute("data-row-id"))
);
const nakaginRowId = designRowIds.find(id => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
if (nakaginRowId) {
  await page.evaluate((rowId) => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, nakaginRowId);
}
await page.waitForLoadState("networkidle");
await page.waitForTimeout(8000);
console.log("180 nodes loaded, starting stabilization...");
// Stabilize
for (let i = 0; i < 20; i++) {
  const cnt = await page.evaluate(() => document.querySelectorAll('.react-flow__node').length);
  if (cnt >= 180) break;
  await page.waitForTimeout(500);
}
await page.waitForTimeout(3000);
// Close left panel
const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await leftPanelToggle.isVisible().catch(() => false)) {
  const leftPanelOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
  if (leftPanelOpen) {
    await leftPanelToggle.click();
    await page.waitForTimeout(500);
  }
}
// Setup long task observer with phases
await page.evaluate(() => {
  const store = (window as any).__PERF__ = { tasks: [] as any[], phase: 'idle', phaseStart: 0 };
  const obs = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      store.tasks.push({ phase: store.phase, duration: entry.duration, startTime: entry.startTime });
    }
  });
  obs.observe({ entryTypes: ['longtask'] });
});
const setPhase = async (phase: string) => {
  await page.evaluate((p) => { const s = (window as any).__PERF__; s.phase = p; s.phaseStart = performance.now(); }, phase);
};
const clearTasks = async () => {
  await page.evaluate(() => { (window as any).__PERF__.tasks = []; });
};
await clearTasks();
// ZOOM IN
const pane = page.locator('#diagram .react-flow__pane').first();
const paneBox = await pane.boundingBox();
const cx = paneBox!.x + paneBox!.width / 2;
const cy = paneBox!.y + paneBox!.height / 2;
await setPhase('ZOOM_IN');
await page.mouse.move(cx, cy);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
// ZOOM OUT
await setPhase('ZOOM_OUT');
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
// MOUSEDOWN
const firstNode = page.locator('.react-flow__node').first();
const nodeBox = await firstNode.boundingBox();
const nx = nodeBox!.x + nodeBox!.width / 2;
const ny = nodeBox!.y + nodeBox!.height / 2;
await page.mouse.move(nx, ny);
await setPhase('MOUSEDOWN');
await page.mouse.down();
await page.waitForTimeout(50);
// DRAG
await setPhase('DRAG');
await page.mouse.move(nx + 100, ny, { steps: 20 });
// MOUSEUP
await setPhase('MOUSEUP');
await page.mouse.up();
await page.waitForTimeout(2000);
// SETTLE
await setPhase('SETTLE');
await page.waitForTimeout(1000);
// Collect results
const tasks = await page.evaluate(() => (window as any).__PERF__.tasks);
// Group by phase
const phases: Record<string, number[]> = {};
for (const t of tasks) {
  if (!phases[t.phase]) phases[t.phase] = [];
  phases[t.phase].push(t.duration);
}
console.log("\n=== LONG TASKS BY PHASE ===");
for (const [phase, durations] of Object.entries(phases)) {
  const max = Math.max(...durations);
  const sum = durations.reduce((a: number, b: number) => a + b, 0);
  console.log(`${phase}: ${durations.length} tasks, max ${max.toFixed(0)}ms, total ${sum.toFixed(0)}ms`);
  if (durations.length <= 10) console.log(`  durations: ${durations.map(d => d.toFixed(0)).join(', ')}ms`);
}
console.log(`\nTOTAL: ${tasks.length} long tasks`);
const maxAll = tasks.length > 0 ? Math.max(...tasks.map((t: any) => t.duration)) : 0;
console.log(`MAX: ${maxAll.toFixed(0)}ms`);
await browser.close();
