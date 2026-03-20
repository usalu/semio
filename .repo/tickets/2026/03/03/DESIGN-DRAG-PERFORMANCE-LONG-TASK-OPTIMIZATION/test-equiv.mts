import { chromium } from "playwright";
import path from "path";
const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
const page = await browser.newPage();
await page.addInitScript(() => {
  (window as any).__SEMIO_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
  const store = (window as any).__SEMIO_PERFORMANCE__;
  const oc = (window as any).PerformanceObserver;
  const sek = oc?.supportedEntryTypes ?? [];
  if (!oc || !sek.includes("longtask")) return;
  store.longTaskSupported = true;
  const obs = new oc((entryList: any) => {
    const entries = entryList.getEntries().map((e: any) => ({ duration: e.duration, startTime: e.startTime }));
    store.longTasks.push(...entries);
  });
  obs.observe({ entryTypes: ["longtask"] });
});
await page.goto("http://127.0.0.1:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(2000);
const zipPath = path.resolve(process.cwd(), "semio/assets/semio/metabolism.zip");
const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: 'attached', timeout: 10000 });
const [fc] = await Promise.all([
  page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null),
  fileInput.dispatchEvent("click")
]);
if (fc) await fc.setFiles(zipPath);
else { await fileInput.setInputFiles(zipPath); await fileInput.evaluate((el) => { el.dispatchEvent(new Event("change", { bubbles: true })); }); }
await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
await tableRow.dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(2000);
const designRowIds = await page.evaluate(() => Array.from(document.querySelectorAll('[data-row-id^="design-"]')).map(el => el.getAttribute("data-row-id")));
const nakaginRowId = designRowIds.find(id => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
if (nakaginRowId) {
  await page.evaluate((rowId) => { const row = document.querySelector(`[data-row-id="${rowId}"]`); if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window })); }, nakaginRowId);
}
await page.waitForLoadState("networkidle");
await page.waitForTimeout(10000);
const leftPanelToggle = page.locator('[id="semio.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await leftPanelToggle.isVisible().catch(() => false)) {
  const leftPanelOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
  if (leftPanelOpen) { await leftPanelToggle.click(); await page.waitForTimeout(500); }
}
await page.waitForTimeout(3000);
// Stabilize
const waitForStab = async () => {
  for (let i = 0; i < 20; i++) {
    const count = await page.evaluate(() => {
      const store = (window as any).__SEMIO_PERFORMANCE__;
      const tasks = store?.longTasks ?? [];
      return tasks.length;
    });
    await page.waitForTimeout(500);
    const count2 = await page.evaluate(() => {
      const store = (window as any).__SEMIO_PERFORMANCE__;
      return (store?.longTasks ?? []).length;
    });
    if (count === count2) break;
  }
};
await waitForStab();
// Clear long tasks - exact same as test
await page.evaluate(() => {
  const store = (window as any).__SEMIO_PERFORMANCE__;
  store.longTasks = [];
});
const getVT = async () => await page.evaluate(() => {
  const vp = document.querySelector("#diagram .react-flow__viewport") as HTMLElement | null;
  const t = vp?.style.transform ?? "";
  const m = t.match(/translate\(([-0-9.]+)px,\s*([-0-9.]+)px\)\s*scale\(([-0-9.]+)\)/);
  if (!m) return { x: 0, y: 0, scale: 1 };
  return { x: Number(m[1]), y: Number(m[2]), scale: Number(m[3]) };
});
const pane = page.locator('#diagram .react-flow__pane').first();
const paneBox = await pane.boundingBox();
const cx = paneBox!.x + paneBox!.width / 2;
const cy = paneBox!.y + paneBox!.height / 2;
// Mark phase
const markPhase = async (phase: string) => {
  await page.evaluate((p) => { (window as any).__SEMIO_PERFORMANCE__.currentPhase = p; (window as any).__SEMIO_PERFORMANCE__.phaseStart = performance.now(); }, phase);
};
// ZOOM IN
await markPhase("ZOOM_IN");
await page.mouse.move(cx, cy);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
// ZOOM OUT
await markPhase("ZOOM_OUT");
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
// Get first node
const diag = page.locator('#diagram .react-flow').first();
const pieceNodes = diag.locator(".react-flow__node");
const firstNode = pieceNodes.first();
const nodeBox = await firstNode.boundingBox();
const startX = nodeBox!.x + nodeBox!.width / 2;
const startY = nodeBox!.y + nodeBox!.height / 2;
// MOUSEDOWN
await markPhase("MOUSEDOWN");
await page.mouse.move(startX, startY);
await page.waitForTimeout(50);
await page.mouse.down();
await page.waitForTimeout(50);
// DRAG + MOUSEUP (exact same as test)
await markPhase("DRAG_AND_MOUSEUP");
await page.mouse.move(startX + 100, startY, { steps: 20 });
await page.mouse.up();
// Wait for node to actually move (same as test's expect.poll)
for (let i = 0; i < 50; i++) {
  const box = await firstNode.boundingBox();
  if (box && Math.abs(box.x - nodeBox!.x) > 10) break;
  await page.waitForTimeout(50);
}
// Read long tasks NOW (same as test)
const longTaskDurations: number[] = await page.evaluate(() => {
  const store = (window as any).__SEMIO_PERFORMANCE__;
  if (!store) return [];
  return (store.longTasks ?? []).map((e: any) => e.duration);
});
const maxLT = longTaskDurations.length > 0 ? Math.max(...longTaskDurations) : 0;
console.log(`\n=== RESULTS (test-equivalent) ===`);
console.log(`Long tasks: ${longTaskDurations.length}`);
console.log(`Max: ${maxLT.toFixed(1)}ms`);
console.log(`All durations: ${longTaskDurations.map(d => d.toFixed(1) + 'ms').join(', ')}`);
console.log(`Budget: 50ms per task`);
console.log(`Pass: ${maxLT <= 50 ? 'YES!' : 'NO (' + (maxLT - 50).toFixed(1) + 'ms over)'}`);
await browser.close();
