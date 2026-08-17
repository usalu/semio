import { chromium } from "playwright";
import path from "path";
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox"] });
const page = await browser.newPage();
await page.addInitScript(() => {
  (window as any).__COMPOSE_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
  const store = (window as any).__COMPOSE_PERFORMANCE__;
  const oc = (window as any).PerformanceObserver;
  const sek = oc?.supportedEntryTypes ?? [];
  if (!oc || !sek.includes("longtask")) return;
  store.longTaskSupported = true;
  const obs = new oc((entryList: any) => {
    const entries = entryList.getEntries().map((e: any) => ({ duration: e.duration, startTime: e.startTime, name: e.name }));
    store.longTasks.push(...entries);
  });
  obs.observe({ entryTypes: ["longtask"] });
  (window as any).__phases = [];
  (window as any).__markPhase = (name: string) => {
    (window as any).__phases.push({ name, time: performance.now() });
  };
});
await page.goto("http://127.0.0.1:5173/");
await page.waitForLoadState("domcontentloaded");
await page.waitForTimeout(2000);
const zipPath = path.resolve(process.cwd(), "assets/compose/metabolism.zip");
const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
await fileInput.waitFor({ state: "attached", timeout: 10000 });
const [fc] = await Promise.all([page.waitForEvent("filechooser", { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent("click")]);
if (fc) await fc.setFiles(zipPath);
else {
  await fileInput.setInputFiles(zipPath);
  await fileInput.evaluate((el) => {
    el.dispatchEvent(new Event("change", { bubbles: true }));
  });
}
await page.getByText("Metabolism", { exact: true }).first().waitFor({ state: "visible", timeout: 60000 });
await page.waitForTimeout(500);
const tableRow = page.locator("tr[data-row-id]").filter({ hasText: "Metabolism" }).first();
await tableRow.dblclick({ force: true });
await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
await page.waitForLoadState("networkidle");
await page.waitForTimeout(2000);
const designRowIds = await page.evaluate(() => Array.from(document.querySelectorAll('[data-row-id^="design-"]')).map((el) => el.getAttribute("data-row-id")));
const nakaginRowId = designRowIds.find((id) => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
if (nakaginRowId) {
  await page.evaluate((rowId) => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
  }, nakaginRowId);
}
await page.waitForLoadState("networkidle");
await page.waitForTimeout(10000);
const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await leftPanelToggle.isVisible().catch(() => false)) {
  const leftPanelOpen = await page
    .locator('[data-panel="leftSidePanel"]')
    .isVisible()
    .catch(() => false);
  if (leftPanelOpen) {
    await leftPanelToggle.click();
    await page.waitForTimeout(500);
  }
}
await page.waitForTimeout(3000);
// Stabilize
await page.evaluate(() => {
  const store = (window as any).__COMPOSE_PERFORMANCE__;
  store.longTasks = [];
  (window as any).__phases = [];
});
const getVT = async () =>
  await page.evaluate(() => {
    const vp = document.querySelector("#diagram .react-flow__viewport") as HTMLElement | null;
    const t = vp?.style.transform ?? "";
    const m = t.match(/translate\(([-0-9.]+)px,\s*([-0-9.]+)px\)\s*scale\(([-0-9.]+)\)/);
    if (!m) return { x: 0, y: 0, scale: 1 };
    return { x: Number(m[1]), y: Number(m[2]), scale: Number(m[3]) };
  });
const pane = page.locator("#diagram .react-flow__pane").first();
const paneBox = await pane.boundingBox();
const cx = paneBox!.x + paneBox!.width / 2;
const cy = paneBox!.y + paneBox!.height / 2;
// ZOOM IN
await page.evaluate(() => (window as any).__markPhase("ZOOM_IN_START"));
await page.mouse.move(cx, cy);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.evaluate(() => (window as any).__markPhase("ZOOM_OUT_START"));
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
// Get node for drag
const diag = page.locator("#diagram .react-flow").first();
const firstNode = diag.locator(".react-flow__node").first();
const nodeBox = await firstNode.boundingBox();
const startX = nodeBox!.x + nodeBox!.width / 2;
const startY = nodeBox!.y + nodeBox!.height / 2;
await page.evaluate(() => (window as any).__markPhase("MOUSE_MOVE"));
await page.mouse.move(startX, startY);
await page.waitForTimeout(50);
await page.evaluate(() => (window as any).__markPhase("MOUSE_DOWN"));
await page.mouse.down();
await page.waitForTimeout(50);
await page.evaluate(() => (window as any).__markPhase("DRAG_START"));
await page.mouse.move(startX + 100, startY, { steps: 20 });
await page.evaluate(() => (window as any).__markPhase("MOUSE_UP"));
await page.mouse.up();
// Wait for node to move
for (let i = 0; i < 50; i++) {
  const box = await firstNode.boundingBox();
  if (box && Math.abs(box.x - nodeBox!.x) > 10) break;
  await page.waitForTimeout(50);
}
await page.evaluate(() => (window as any).__markPhase("READ_TASKS"));
// Read tasks and phases
const result = await page.evaluate(() => {
  const store = (window as any).__COMPOSE_PERFORMANCE__;
  const phases = (window as any).__phases as { name: string; time: number }[];
  const tasks = (store?.longTasks ?? []) as { duration: number; startTime: number }[];
  return { phases, tasks };
});
console.log("\n=== PHASE TIMESTAMPS ===");
for (const p of result.phases) {
  console.log(`${p.time.toFixed(1)}\t${p.name}`);
}
console.log("\n=== LONG TASKS (with phase context) ===");
for (const t of result.tasks) {
  const end = t.startTime + t.duration;
  let phase = "UNKNOWN";
  for (let i = result.phases.length - 1; i >= 0; i--) {
    if (result.phases[i].time <= t.startTime) {
      phase = result.phases[i].name;
      break;
    }
  }
  console.log(`${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)} - ${end.toFixed(0)} [${phase}]`);
}
const maxDur = result.tasks.length > 0 ? Math.max(...result.tasks.map((t) => t.duration)) : 0;
console.log(`\nTotal: ${result.tasks.length} tasks, Max: ${maxDur.toFixed(0)}ms, Pass: ${maxDur <= 50}`);
await browser.close();
