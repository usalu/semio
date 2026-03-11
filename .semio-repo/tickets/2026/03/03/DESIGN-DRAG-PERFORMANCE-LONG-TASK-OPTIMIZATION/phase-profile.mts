import { chromium, CDPSession } from "playwright";
import path from "path";
import fs from "fs";
const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
const page = await browser.newPage();
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
// Setup
const diag = page.locator('#diagram .react-flow').first();
const pieceNodes = diag.locator(".react-flow__node");
const firstNode = pieceNodes.first();
const nodeBox = await firstNode.boundingBox();
const startX = nodeBox!.x + nodeBox!.width / 2;
const startY = nodeBox!.y + nodeBox!.height / 2;
// Zoom first (to match test)
const pane = page.locator('#diagram .react-flow__pane').first();
const paneBox = await pane.boundingBox();
await page.mouse.move(paneBox!.x + paneBox!.width / 2, paneBox!.y + paneBox!.height / 2);
await page.mouse.wheel(0, -600);
await page.waitForTimeout(500);
await page.mouse.wheel(0, 600);
await page.waitForTimeout(500);
// MOUSEDOWN first (without profiling)
await page.mouse.move(startX, startY);
await page.waitForTimeout(50);
// PROFILE MOUSEDOWN
const cdp: CDPSession = await page.context().newCDPSession(page);
await cdp.send('Profiler.enable');
await cdp.send('Profiler.start');
await page.mouse.down();
await page.waitForTimeout(200); // capture the mousedown processing
const { profile: p1 } = await cdp.send('Profiler.stop');
fs.writeFileSync(path.resolve(process.cwd(), ".semio-repo/tickets/2026/03/03/DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION/mousedown.cpuprofile"), JSON.stringify(p1));
// PROFILE DRAG
await cdp.send('Profiler.start');
await page.mouse.move(startX + 100, startY, { steps: 20 });
await page.waitForTimeout(200);
const { profile: p2 } = await cdp.send('Profiler.stop');
fs.writeFileSync(path.resolve(process.cwd(), ".semio-repo/tickets/2026/03/03/DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION/drag.cpuprofile"), JSON.stringify(p2));
// PROFILE MOUSEUP
await cdp.send('Profiler.start');
await page.mouse.up();
await page.waitForTimeout(500);
const { profile: p3 } = await cdp.send('Profiler.stop');
fs.writeFileSync(path.resolve(process.cwd(), ".semio-repo/tickets/2026/03/03/DESIGN-DRAG-PERFORMANCE-LONG-TASK-OPTIMIZATION/mouseup3.cpuprofile"), JSON.stringify(p3));
function analyzeProfile(name: string, profile: any) {
  const nodes2 = profile.nodes;
  const totalSamples = profile.samples?.length ?? 0;
  const hitCounts: Record<string, number> = {};
  for (const node of nodes2) {
    const fn = node.callFrame.functionName || '(anonymous)';
    const url = node.callFrame.url || '';
    const line = node.callFrame.lineNumber;
    const key = fn === '(anonymous)' ? `(anon)@${url.split('/').pop()}:${line}` : `${fn}@${url.split('/').pop()}:${line}`;
    hitCounts[key] = (hitCounts[key] ?? 0) + (node.hitCount ?? 0);
  }
  const sorted = Object.entries(hitCounts).sort((a, b) => b[1] - a[1]).slice(0, 20);
  console.log(`\n=== ${name} (${totalSamples} samples) ===`);
  for (const [fn, hits] of sorted) {
    const pct = (hits / totalSamples * 100).toFixed(1);
    console.log(`${pct}% (${hits}) - ${fn}`);
  }
}
analyzeProfile("MOUSEDOWN", p1);
analyzeProfile("DRAG", p2);
analyzeProfile("MOUSEUP (500ms)", p3);
await browser.close();
