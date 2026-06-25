import { chromium } from 'playwright';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const SKETCHPAD_URL = 'http://127.0.0.1:5173';

async function main() {
  const browser = await chromium.launch({ headless: true, args: ['--no-sandbox'] });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();

  const browserLogs: string[] = [];
  page.on('console', msg => {
    const text = msg.text();
    if (text.includes('[DEBUG]')) browserLogs.push(text);
  });

  await page.addInitScript(() => {
    (window as any).__COMPOSE_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
    const store = (window as any).__COMPOSE_PERFORMANCE__;
    const obs = (window as any).PerformanceObserver;
    if (!obs || !(obs.supportedEntryTypes ?? []).includes('longtask')) return;
    store.longTaskSupported = true;
    new obs((list: any) => {
      for (const e of list.getEntries()) store.longTasks.push({ duration: e.duration, startTime: e.startTime });
    }).observe({ entryTypes: ['longtask'] });
  });

  console.log('[DIAG] Step 1: Navigate to home...');
  await page.goto(SKETCHPAD_URL, { waitUntil: 'domcontentloaded', timeout: 30000 });
  await page.waitForTimeout(2000);

  console.log('[DIAG] Step 2: Import kit...');
  const zipPath = path.resolve('/workspaces/semio/assets/compose/metabolism.zip');
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
  await fileInput.waitFor({ state: 'attached', timeout: 10000 });
  await fileInput.setInputFiles(zipPath);
  await fileInput.evaluate((el: any) => { el.dispatchEvent(new Event('change', { bubbles: true })); });

  console.log('[DIAG] Step 3: Wait for Metabolism...');
  const metaText = page.getByText('Metabolism', { exact: true }).first();
  await metaText.waitFor({ state: 'visible', timeout: 60000 });
  await page.waitForTimeout(500);

  console.log('[DIAG] Step 4: Navigate to kit...');
  const tableRow = page.locator('tr[data-row-id]').filter({ hasText: 'Metabolism' }).first();
  if (await tableRow.isVisible().catch(() => false)) {
    await tableRow.dblclick({ force: true });
  } else {
    await metaText.dblclick({ force: true });
  }
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(3000);

  console.log('[DIAG] Step 5: Navigate to design...');
  const allRowIds = await page.evaluate(() =>
    Array.from(document.querySelectorAll('[data-row-id]')).map(el => el.getAttribute('data-row-id')).slice(0, 20)
  );
  const designRowIds = allRowIds.filter(id => id?.startsWith('design-'));
  const nakaginRowId = designRowIds.find(id => id?.includes('9a890dd4')) ?? designRowIds[designRowIds.length - 1];
  if (nakaginRowId) {
    await page.evaluate(rowId => {
      const row = document.querySelector(`[data-row-id="${rowId}"]`);
      if (row) row.dispatchEvent(new MouseEvent('dblclick', { bubbles: true, cancelable: true, view: window }));
    }, nakaginRowId);
  }
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(5000);

  console.log('[DIAG] Step 6: Wait for diagram...');
  const diag = page.locator('#diagram .react-flow').first();
  await diag.waitFor({ state: 'visible', timeout: 60000 });
  const pieceNodes = diag.locator('.react-flow__node');
  await pieceNodes.first().waitFor({ state: 'attached', timeout: 60000 });
  console.log(`[DIAG] Found ${await pieceNodes.count()} nodes`);
  await page.waitForTimeout(5000);

  // Close left panel
  const leftToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
  if (await leftToggle.isVisible().catch(() => false)) {
    const leftOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
    if (leftOpen) { await leftToggle.click(); await page.waitForTimeout(500); }
  }

  // Clear long tasks and browser logs
  await page.evaluate(() => { (window as any).__COMPOSE_PERFORMANCE__.longTasks = []; });
  browserLogs.length = 0;

  console.log('[DIAG] === STARTING MEASUREMENT ===');

  // Zoom in
  const pane = diag.locator('.react-flow__pane').first();
  const paneBox = await pane.boundingBox();
  const cx = paneBox!.x + paneBox!.width / 2;
  const cy = paneBox!.y + paneBox!.height / 2;
  await page.mouse.move(cx, cy);
  console.log('[DIAG] Zoom in...');
  await page.mouse.wheel(0, -600);
  await page.waitForTimeout(500);
  console.log('[DIAG] Zoom out...');
  await page.mouse.wheel(0, 600);
  await page.waitForTimeout(500);

  // Drag
  const firstNode = pieceNodes.first();
  const nodeBox = await firstNode.boundingBox();
  const startX = nodeBox!.x + nodeBox!.width / 2;
  const startY = nodeBox!.y + nodeBox!.height / 2;
  await page.mouse.move(startX, startY);
  await page.waitForTimeout(50);
  await page.mouse.down();
  await page.waitForTimeout(50);

  console.log('[DIAG] Dragging 100px in 20 steps...');
  const dragStart = Date.now();
  await page.mouse.move(startX + 100, startY, { steps: 20 });
  const dragEnd = Date.now();
  console.log(`[DIAG] Drag moves took ${dragEnd - dragStart}ms`);
  
  console.log('[DIAG] Mouse up...');
  await page.mouse.up();

  // Poll for movement
  for (let i = 0; i < 10; i++) {
    const box = await firstNode.boundingBox();
    if (box && Math.abs(box.x - nodeBox!.x) > 10) break;
    await page.waitForTimeout(100);
  }

  // Read long tasks immediately
  const result = await page.evaluate(() => {
    const store = (window as any).__COMPOSE_PERFORMANCE__;
    return store.longTasks as Array<{duration: number, startTime: number}>;
  });

  console.log(`\n[DIAG] === RESULTS ===`);
  console.log(`[DIAG] Total long tasks: ${result.length}`);
  const sorted = [...result].sort((a, b) => b.duration - a.duration);
  console.log(`[DIAG] Top 10 long tasks:`);
  for (const t of sorted.slice(0, 10)) {
    console.log(`  ${t.duration.toFixed(1)}ms @ ${t.startTime.toFixed(1)}ms`);
  }
  const max = sorted[0];
  console.log(`\n[DIAG] MAX: ${max?.duration.toFixed(1)}ms @ ${max?.startTime.toFixed(1)}ms`);

  console.log(`\n[DIAG] === BROWSER [DEBUG] LOGS (${browserLogs.length}) ===`);
  for (const log of browserLogs.slice(0, 100)) {
    console.log(`  ${log}`);
  }

  // Wait 5 seconds for post-drag tasks
  await page.waitForTimeout(5000);
  const laterResult = await page.evaluate(() => (window as any).__COMPOSE_PERFORMANCE__.longTasks);
  const newTasks = laterResult.filter((t: any) => !result.find((r: any) => r.startTime === t.startTime && r.duration === t.duration));
  if (newTasks.length > 0) {
    console.log(`\n[DIAG] === POST-DRAG TASKS (after 5s wait): ${newTasks.length} ===`);
    for (const t of newTasks.sort((a: any, b: any) => b.duration - a.duration).slice(0, 10)) {
      console.log(`  ${t.duration.toFixed(1)}ms @ ${t.startTime.toFixed(1)}ms`);
    }
  }

  await browser.close();
}

main().catch(e => { console.error(e); process.exit(1); });
