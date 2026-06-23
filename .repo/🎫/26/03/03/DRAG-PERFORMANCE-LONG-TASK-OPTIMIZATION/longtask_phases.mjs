import { chromium } from "playwright";
import { writeFileSync } from "fs";
const browser = await chromium.launch({ headless: true, args: ["--no-sandbox", "--disable-gpu"] });
const page = await browser.newPage();
await page.addInitScript(() => {
  window.__LT = { tasks: [], markers: [], logs: [] };
  const obs = new PerformanceObserver((list) => {
    for (const entry of list.getEntries()) {
      window.__LT.tasks.push({ d: entry.duration, s: entry.startTime });
    }
  });
  try { obs.observe({ entryTypes: ["longtask"] }); } catch(e) {}
});
await page.goto("http://127.0.0.1:5173", { waitUntil: "domcontentloaded" });
await page.waitForTimeout(2000);
const kitRow = page.locator("table tbody tr").first();
await kitRow.dblclick();
await page.waitForTimeout(2000);
const designRows = page.locator('table tbody tr[id^="design-"]');
const designRow = designRows.first();
await designRow.waitFor({ state: "visible", timeout: 30000 });
await designRow.dblclick();
await page.waitForTimeout(5000);
const diag = page.locator("#diagram .react-flow").first();
await diag.waitFor({ state: "visible", timeout: 60000 });
const nodes = diag.locator(".react-flow__node");
await nodes.first().waitFor({ state: "attached", timeout: 60000 });
const nodeCount = await nodes.count();
console.log(`nodes: ${nodeCount}`);
for (let i = 0; i < 10; i++) {
  await page.waitForTimeout(2000);
  const c = await nodes.count();
  if (c >= 170) break;
}
await page.waitForTimeout(3000);
await page.evaluate(() => { window.__LT.tasks = []; window.__LT.markers.push({ name: "START", t: performance.now() }); });
const toggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
if (await toggle.isVisible().catch(() => false)) {
  const leftOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
  if (leftOpen) { await toggle.click(); await page.waitForTimeout(500); }
}
await page.evaluate(() => { window.__LT.markers.push({ name: "PANEL_CLOSED", t: performance.now() }); });
const pane = diag.locator(".react-flow__pane").first();
const paneBox = await pane.boundingBox();
const cx = paneBox.x + paneBox.width / 2;
const cy = paneBox.y + paneBox.height / 2;
await page.mouse.move(cx, cy);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.evaluate(() => { window.__LT.markers.push({ name: "ZOOM_IN", t: performance.now() }); });
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
await page.evaluate(() => { window.__LT.markers.push({ name: "ZOOM_OUT", t: performance.now() }); });
const firstNode = nodes.first();
const box = await firstNode.boundingBox();
const sx = box.x + box.width / 2, sy = box.y + box.height / 2;
await page.mouse.move(sx, sy);
await page.waitForTimeout(50);
await page.evaluate(() => {
  window.__LT.mutCount = 0;
  window.__LT.mutPhases = [];
  const nodesEl = document.querySelector('.react-flow__nodes');
  if (nodesEl) {
    const mo = new MutationObserver((muts) => {
      window.__LT.mutCount += muts.length;
      window.__LT.mutPhases.push({ t: performance.now(), n: muts.length, types: muts.slice(0,5).map(m => m.type + ':' + m.attributeName).join(',') });
    });
    mo.observe(nodesEl, { childList: true, subtree: true, attributes: true, attributeFilter: ['style', 'class', 'transform'] });
    window.__LT._mo = mo;
  }
});
await page.evaluate(() => { window.__LT.markers.push({ name: "BEFORE_DOWN", t: performance.now() }); });
await page.mouse.down();
await page.waitForTimeout(50);
await page.evaluate(() => { window.__LT.markers.push({ name: "AFTER_DOWN", t: performance.now() }); });
await page.mouse.move(sx + 100, sy, { steps: 20 });
await page.evaluate(() => { window.__LT.markers.push({ name: "AFTER_MOVE", t: performance.now() }); });
await page.evaluate(() => { window.__LT.markers.push({ name: "BEFORE_UP", t: performance.now() }); });
await page.mouse.up();
await page.waitForTimeout(200);
await page.evaluate(() => { window.__LT.markers.push({ name: "AFTER_UP", t: performance.now() }); });
await page.waitForTimeout(5000);
await page.evaluate(() => { window.__LT.markers.push({ name: "SETTLE", t: performance.now() }); });
const data = await page.evaluate(() => ({
  tasks: window.__LT.tasks,
  markers: window.__LT.markers,
  mutCount: window.__LT.mutCount || 0,
  mutPhases: (window.__LT.mutPhases || []).slice(0, 50)
}));
console.log("\n=== MARKERS ===");
for (const m of data.markers) console.log(`${m.name}: ${m.t.toFixed(0)}ms`);
console.log(`\n=== LONG TASKS (${data.tasks.length}) ===`);
data.tasks.sort((a,b) => a.s - b.s);
for (const t of data.tasks) {
  let phase = "UNKNOWN";
  for (let i = data.markers.length - 1; i >= 0; i--) {
    if (t.s >= data.markers[i].t) { phase = data.markers[i].name; break; }
  }
  console.log(`  start=${t.s.toFixed(0)}ms dur=${t.d.toFixed(0)}ms phase=${phase}`);
}
const max = data.tasks.length > 0 ? Math.max(...data.tasks.map(t => t.d)) : 0;
const total = data.tasks.reduce((s,t) => s + t.d, 0);
console.log(`\nMAX: ${max.toFixed(0)}ms  TOTAL: ${data.tasks.length} tasks  SUM: ${total.toFixed(0)}ms`);
console.log(`\nDOM Mutations: ${data.mutCount}`);
if (data.mutPhases?.length > 0) {
  console.log("\n=== MUTATION BURSTS (first 30) ===");
  for (const mp of data.mutPhases.slice(0, 30)) {
    let phase = "UNKNOWN";
    for (let i = data.markers.length - 1; i >= 0; i--) {
      if (mp.t >= data.markers[i].t) { phase = data.markers[i].name; break; }
    }
    console.log(`  t=${mp.t.toFixed(0)}ms n=${mp.n} phase=${phase} ${mp.types}`);
  }
}
console.log(`Node moved: ${movedPx.toFixed(1)}px`);
await browser.close();
