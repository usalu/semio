import { chromium } from "playwright";

const browser = await chromium.launch({ headless: true, args: ["--disable-gpu"] });
const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
const page = await context.newPage();

page.on("console", (msg) => {
  const text = msg.text();
  if (text.includes("[DEBUG]") || text.includes("[initDesign]") || text.includes("[TEST]")) {
    console.log(`[BROWSER] ${text}`);
  }
});

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

await page.goto("http://127.0.0.1:5173/", { waitUntil: "networkidle" });
console.log("Page loaded, uploading kit...");

const fileInput = page.locator('input[type="file"]');
await fileInput.setInputFiles("/workspaces/semio/compose/assets/compose/metabolism.zip");
await page.waitForTimeout(5000);
await page.getByText("Metabolism").waitFor({ timeout: 30000 });
console.log("Kit uploaded");

const kitRow = page.locator('[data-row-id]').filter({ hasText: "Metabolism" }).first();
if (await kitRow.isVisible({ timeout: 5000 }).catch(() => false)) {
  await kitRow.dblclick();
} else {
  const allRows = page.locator('[data-row-id]');
  const count = await allRows.count();
  for (let i = 0; i < count; i++) {
    const rowId = await allRows.nth(i).getAttribute("data-row-id");
    if (rowId && rowId.includes("f042c2a4")) {
      await allRows.nth(i).dblclick();
      break;
    }
  }
}
await page.waitForLoadState("networkidle");
await page.waitForTimeout(3000);
console.log(`URL after kit nav: ${page.url()}`);

const designRowIds = await page.evaluate(() => {
  return Array.from(document.querySelectorAll("[data-row-id]"))
    .map(el => el.getAttribute("data-row-id"))
    .filter(id => id?.startsWith("design-"));
});
console.log(`Design rows: ${JSON.stringify(designRowIds)}`);

const nakaginRowId = designRowIds.find(id => id?.includes("9a890dd4")) ?? designRowIds[designRowIds.length - 1];
console.log(`Double-clicking design: ${nakaginRowId}`);
await page.evaluate((rowId) => {
  const row = document.querySelector(`[data-row-id="${rowId}"]`);
  if (row) row.dispatchEvent(new MouseEvent("dblclick", { bubbles: true, cancelable: true, view: window }));
}, nakaginRowId);

await page.waitForLoadState("networkidle");
await page.waitForTimeout(8000);
console.log(`URL after design nav: ${page.url()}`);

const nodeCount = await page.locator("#diagram .react-flow__node").count();
console.log(`Nodes: ${nodeCount}`);

const patched = await page.evaluate(() => (window as any).__COMPOSE_ZUSTAND_PATCHED__);
console.log(`Zustand patched: ${patched}`);

await page.waitForTimeout(5000);

await page.evaluate(() => { (window as any).__COMPOSE_PERFORMANCE__.longTasks = []; });

const diagramBox = await page.locator("#diagram .react-flow__pane").first().boundingBox();
if (!diagramBox) { console.log("No diagram pane"); await browser.close(); process.exit(1); }

console.log("\n--- PHASE 1: ZOOM ---");
await page.mouse.move(diagramBox.x + diagramBox.width / 2, diagramBox.y + diagramBox.height / 2);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);

let tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks];
  s.longTasks = [];
  return t;
});
console.log(`Zoom: ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map((t: any) => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`  ${(t as any).duration.toFixed(0)}ms @ ${(t as any).startTime.toFixed(0)}`);

console.log("\n--- PHASE 2: MOUSEDOWN ON NODE ---");
const firstNode = page.locator("#diagram .react-flow__node").first();
const nb = await firstNode.boundingBox();
if (!nb) { console.log("No node box"); await browser.close(); process.exit(1); }
await page.mouse.move(nb.x + nb.width / 2, nb.y + nb.height / 2);
await page.waitForTimeout(50);

const cdp = await context.newCDPSession(page);
await cdp.send("Profiler.enable");
await cdp.send("Profiler.start");

await page.mouse.down();
await page.waitForTimeout(100);

const profileMousedown = await cdp.send("Profiler.stop");

tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks];
  s.longTasks = [];
  return t;
});
console.log(`Mousedown: ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map((t: any) => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`  ${(t as any).duration.toFixed(0)}ms @ ${(t as any).startTime.toFixed(0)}`);

const topFunctions = new Map<string, number>();
for (const node of profileMousedown.profile.nodes) {
  if (node.hitCount && node.hitCount > 0 && node.callFrame.functionName) {
    const key = `${node.callFrame.functionName} (${node.callFrame.url?.split("/").pop() || "?"})`;
    topFunctions.set(key, (topFunctions.get(key) || 0) + node.hitCount);
  }
}
const sorted = [...topFunctions.entries()].sort((a, b) => b[1] - a[1]).slice(0, 20);
console.log("\nTop functions during mousedown:");
for (const [fn, hits] of sorted) {
  console.log(`  ${hits} hits: ${fn}`);
}

console.log("\n--- PHASE 3: DRAG ---");
await cdp.send("Profiler.start");
await page.mouse.move(nb.x + nb.width / 2 + 100, nb.y + nb.height / 2, { steps: 20 });
await page.waitForTimeout(100);
const profileDrag = await cdp.send("Profiler.stop");
await page.mouse.up();

tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks];
  s.longTasks = [];
  return t;
});
console.log(`Drag: ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map((t: any) => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`  ${(t as any).duration.toFixed(0)}ms @ ${(t as any).startTime.toFixed(0)}`);

const topDragFunctions = new Map<string, number>();
for (const node of profileDrag.profile.nodes) {
  if (node.hitCount && node.hitCount > 0 && node.callFrame.functionName) {
    const key = `${node.callFrame.functionName} (${node.callFrame.url?.split("/").pop() || "?"})`;
    topDragFunctions.set(key, (topDragFunctions.get(key) || 0) + node.hitCount);
  }
}
const sortedDrag = [...topDragFunctions.entries()].sort((a, b) => b[1] - a[1]).slice(0, 20);
console.log("\nTop functions during drag:");
for (const [fn, hits] of sortedDrag) {
  console.log(`  ${hits} hits: ${fn}`);
}

await page.waitForTimeout(1000);
tasks = await page.evaluate(() => {
  const s = (window as any).__COMPOSE_PERFORMANCE__;
  const t = [...s.longTasks];
  s.longTasks = [];
  return t;
});
console.log(`\nPost-drag settle (1s): ${tasks.length} long tasks, max ${tasks.length ? Math.max(...tasks.map((t: any) => t.duration)).toFixed(0) : 0}ms`);
for (const t of tasks) console.log(`  ${(t as any).duration.toFixed(0)}ms @ ${(t as any).startTime.toFixed(0)}`);

await cdp.detach();
await browser.close();
console.log("\nDone.");
