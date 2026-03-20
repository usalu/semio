import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true });
const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
const page = await context.newPage();

page.on("console", (msg) => {
  if (msg.type() === "warning" && msg.text().includes("[DEBUG]")) {
    console.log(`[BROWSER WARN] ${msg.text()}`);
  }
});

await page.addInitScript(() => {
  (window as any).__SEMIO_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
  const store = (window as any).__SEMIO_PERFORMANCE__;
  const obs = (window as any).PerformanceObserver;
  if (!obs || !(obs.supportedEntryTypes ?? []).includes("longtask")) return;
  store.longTaskSupported = true;
  new obs((list: any) => {
    store.longTasks.push(...list.getEntries().map((e: any) => ({ duration: e.duration, startTime: e.startTime })));
  }).observe({ entryTypes: ["longtask"] });
});

await page.goto("http://127.0.0.1:5173/", { waitUntil: "networkidle" });
console.log("Page loaded");

const fileInput = page.locator('input[type="file"]');
await fileInput.setInputFiles("/workspaces/semio/semio/assets/semio/metabolism.zip");
await page.waitForTimeout(3000);
await page.getByText("Metabolism").waitFor({ timeout: 30000 });
const row = page.locator('[data-row-id^="kit-"]').first();
if (await row.isVisible().catch(() => false)) {
  await row.dblclick();
} else {
  const rows = page.locator("tr[data-row-id]");
  for (let i = 0; i < await rows.count(); i++) {
    const rid = await rows.nth(i).getAttribute("data-row-id");
    if (rid && rid.includes("f042c2a4")) { await rows.nth(i).dblclick(); break; }
  }
}
await page.waitForTimeout(2000);

const designRow = page.locator('tr[data-row-id^="design-"]').last();
await designRow.waitFor({ timeout: 10000 });
await designRow.dblclick();
await page.waitForTimeout(5000);

const nodeCount = await page.locator("#diagram .react-flow__node").count();
console.log(`Nodes: ${nodeCount}`);

await page.waitForTimeout(3000);

const patched = await page.evaluate(() => (window as any).__SEMIO_ZUSTAND_PATCHED__);
console.log(`Zustand patched: ${patched}`);

await page.evaluate(() => { (window as any).__SEMIO_PERFORMANCE__.longTasks = []; });

const pane = page.locator("#diagram .react-flow__pane").first();
const box = await pane.boundingBox();
if (!box) { console.log("No pane box"); process.exit(1); }

console.log("Zooming in...");
await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);

const afterZoomTasks = await page.evaluate(() => {
  const s = (window as any).__SEMIO_PERFORMANCE__;
  return s.longTasks.map((t: any) => t.duration);
});
console.log(`After zoom in: ${afterZoomTasks.length} long tasks, max ${afterZoomTasks.length ? Math.max(...afterZoomTasks).toFixed(0) : 0}ms`);
console.log(`  All: [${afterZoomTasks.map((d: number) => d.toFixed(0)).join(", ")}]`);

console.log("Zooming out...");
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);

const afterZoomOutTasks = await page.evaluate(() => {
  const s = (window as any).__SEMIO_PERFORMANCE__;
  return s.longTasks.map((t: any) => t.duration);

});
console.log(`After zoom out: ${afterZoomOutTasks.length} long tasks, max ${afterZoomOutTasks.length ? Math.max(...afterZoomOutTasks).toFixed(0) : 0}ms`);

await page.evaluate(() => { (window as any).__SEMIO_PERFORMANCE__.longTasks = []; });

const firstNode = page.locator("#diagram .react-flow__node").first();
const nb = await firstNode.boundingBox();
if (!nb) { console.log("No node box"); process.exit(1); }
const sx = nb.x + nb.width / 2;
const sy = nb.y + nb.height / 2;

console.log("Moving to node...");
await page.mouse.move(sx, sy);
await page.waitForTimeout(50);

console.log("Mousedown...");
await page.mouse.down();
await page.waitForTimeout(50);

const postMousedownTasks = await page.evaluate(() => {
  const s = (window as any).__SEMIO_PERFORMANCE__;
  const tasks = [...s.longTasks];
  s.longTasks = [];
  return tasks.map((t: any) => ({ duration: t.duration, startTime: t.startTime }));
});
console.log(`After mousedown: ${postMousedownTasks.length} long tasks`);
for (const t of postMousedownTasks) {
  console.log(`  ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}ms`);
}

console.log("Dragging 100px...");
await page.mouse.move(sx + 100, sy, { steps: 20 });
await page.waitForTimeout(100);

const postDragTasks = await page.evaluate(() => {
  const s = (window as any).__SEMIO_PERFORMANCE__;
  const tasks = [...s.longTasks];
  s.longTasks = [];
  return tasks.map((t: any) => ({ duration: t.duration, startTime: t.startTime }));
});
console.log(`After drag: ${postDragTasks.length} long tasks`);
for (const t of postDragTasks) {
  console.log(`  ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}ms`);
}

console.log("Mouseup...");
await page.mouse.up();
await page.waitForTimeout(200);

const postUpTasks = await page.evaluate(() => {
  const s = (window as any).__SEMIO_PERFORMANCE__;
  const tasks = [...s.longTasks];
  s.longTasks = [];
  return tasks.map((t: any) => ({ duration: t.duration, startTime: t.startTime }));
});
console.log(`After mouseup: ${postUpTasks.length} long tasks`);
for (const t of postUpTasks) {
  console.log(`  ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}ms`);
}

await page.waitForTimeout(1000);
const lateTasks = await page.evaluate(() => {
  const s = (window as any).__SEMIO_PERFORMANCE__;
  const tasks = [...s.longTasks];
  s.longTasks = [];
  return tasks.map((t: any) => ({ duration: t.duration, startTime: t.startTime }));
});
console.log(`After 1s settle: ${lateTasks.length} long tasks`);
for (const t of lateTasks) {
  console.log(`  ${t.duration.toFixed(0)}ms @ ${t.startTime.toFixed(0)}ms`);
}

await browser.close();
