import { chromium } from 'playwright';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const BASE_URL = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:5173';
const ZIP_PATH = path.resolve('/workspaces/semio/assets/compose/metabolism.zip');

async function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  
  // Install PerformanceObserver (same as test)
  await page.addInitScript(() => {
    window.__COMPOSE_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
    const store = window.__COMPOSE_PERFORMANCE__;
    const obs = window.PerformanceObserver;
    if (!obs || !(obs.supportedEntryTypes || []).includes('longtask')) return;
    store.longTaskSupported = true;
    new obs((list) => {
      store.longTasks.push(...list.getEntries().map(e => ({ duration: e.duration, startTime: e.startTime })));
    }).observe({ entryTypes: ['longtask'] });
  });

  // Navigate
  await page.goto(BASE_URL);
  await page.waitForLoadState('domcontentloaded');
  await sleep(2000);
  
  // Upload zip (same as initHome)
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
  await fileInput.waitFor({ state: 'attached', timeout: 10000 });
  
  const [fileChooser] = await Promise.all([
    page.waitForEvent('filechooser', { timeout: 5000 }).catch(() => null),
    fileInput.dispatchEvent('click'),
  ]);
  if (fileChooser) await fileChooser.setFiles(ZIP_PATH);
  else await fileInput.setInputFiles(ZIP_PATH);
  
  console.log('Waiting for Metabolism text...');
  await page.getByText('Metabolism', { exact: true }).first().waitFor({ state: 'visible', timeout: 60000 });
  await sleep(500);
  
  // Double-click on Metabolism row
  const tableRow = page.locator('tr[data-row-id]').filter({ hasText: 'Metabolism' }).first();
  if (await tableRow.isVisible().catch(() => false)) {
    await tableRow.dblclick({ force: true });
  }
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await page.waitForLoadState('networkidle');
  await sleep(3000);
  
  console.log('Kit loaded. Navigating to design...');
  
  // Navigate to design (same as initDesign)
  const allRowIds = await page.evaluate(() => 
    Array.from(document.querySelectorAll('[data-row-id]')).map(el => el.getAttribute('data-row-id')).slice(0, 20)
  );
  const designRowIds = allRowIds.filter(id => id?.startsWith('design-'));
  const nakaginRowId = designRowIds.find(id => id?.includes('9a890dd4')) || designRowIds[designRowIds.length - 1];
  
  await page.evaluate((rowId) => {
    const row = document.querySelector(`[data-row-id="${rowId}"]`);
    if (row) row.dispatchEvent(new MouseEvent('dblclick', { bubbles: true, cancelable: true, view: window }));
  }, nakaginRowId);
  
  await page.waitForLoadState('networkidle');
  await sleep(5000);
  console.log('Design loaded.');
  
  // Wait for diagram stabilization
  const diagram = page.locator('#diagram .react-flow').first();
  await diagram.waitFor({ state: 'visible', timeout: 60000 });
  const nodes = diagram.locator('.react-flow__node');
  await nodes.first().waitFor({ state: 'attached', timeout: 60000 });
  
  let lastPos = '';
  for (let i = 0; i < 30; i++) {
    await sleep(500);
    const pos = await page.evaluate(() => {
      const ns = document.querySelectorAll('.react-flow__node');
      return Array.from(ns).slice(0, 5).map(n => n.getAttribute('style')).join('|');
    });
    if (pos === lastPos && pos.length > 0) break;
    lastPos = pos;
  }
  console.log(`Nodes: ${await nodes.count()}`);
  
  // Close left panel
  const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
  if (await leftPanelToggle.isVisible().catch(() => false)) {
    const leftPanelOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
    if (leftPanelOpen) {
      await leftPanelToggle.click();
      await sleep(500);
    }
  }
  
  // CLEAR long tasks
  await page.evaluate(() => { window.__COMPOSE_PERFORMANCE__.longTasks = []; });
  
  // Use CDP to capture a performance profile  
  const cdp = await page.context().newCDPSession(page);
  
  // Mark timing in page
  await page.evaluate(() => { performance.mark('test-begin'); });
  const t0 = await page.evaluate(() => performance.now());
  
  // Phase 1: Zoom in
  await page.evaluate(() => { performance.mark('zoom-in-start'); });
  const pane = diagram.locator('.react-flow__pane').first();
  const paneBox = await pane.boundingBox();
  await page.mouse.move(paneBox.x + paneBox.width / 2, paneBox.y + paneBox.height / 2);
  await page.mouse.wheel(0, -600);
  await sleep(500);
  await page.evaluate(() => { performance.mark('zoom-in-end'); });
  
  // Phase 2: Zoom out  
  await page.evaluate(() => { performance.mark('zoom-out-start'); });
  await page.mouse.wheel(0, 600);
  await sleep(500);
  await page.evaluate(() => { performance.mark('zoom-out-end'); });
  
  // Phase 3: Drag
  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const startX = nodeBox.x + nodeBox.width / 2;
  const startY = nodeBox.y + nodeBox.height / 2;
  
  await page.evaluate(() => { performance.mark('drag-start'); });
  await page.mouse.move(startX, startY);
  await sleep(50);
  await page.mouse.down();
  await sleep(50);
  await page.evaluate(() => { performance.mark('drag-moving'); });
  await page.mouse.move(startX + 100, startY, { steps: 20 });
  await page.evaluate(() => { performance.mark('drag-moved'); });
  await page.mouse.up();
  await sleep(200);
  await page.evaluate(() => { performance.mark('drag-end'); });
  
  const t1 = await page.evaluate(() => performance.now());
  
  // Collect results
  const marks = await page.evaluate(() => {
    const entries = performance.getEntriesByType('mark');
    return entries.map(e => ({ name: e.name, time: e.startTime }));
  });
  
  const longTasks = await page.evaluate(() => window.__COMPOSE_PERFORMANCE__.longTasks);
  const renderCount = await page.evaluate(() => window.__DEBUG_PIECE_RENDER_COUNT__ || 0);
  
  console.log('\n=== PERFORMANCE MARKS ===');
  const markTimes = {};
  for (const m of marks) {
    if (m.name.startsWith('test-') || m.name.startsWith('zoom') || m.name.startsWith('drag')) {
      console.log(`  ${m.name}: ${m.time.toFixed(0)}ms`);
      markTimes[m.name] = m.time;
    }
  }
  
  console.log(`\n=== LONG TASKS (total: ${longTasks.length}, max: ${longTasks.length ? Math.max(...longTasks.map(t=>t.duration)).toFixed(0) : 0}ms) ===`);
  
  // Bin tasks by phase
  const phases = [
    { name: 'zoom-in', start: markTimes['zoom-in-start'], end: markTimes['zoom-in-end'] },
    { name: 'zoom-out', start: markTimes['zoom-out-start'], end: markTimes['zoom-out-end'] },
    { name: 'drag-setup', start: markTimes['drag-start'], end: markTimes['drag-moving'] },
    { name: 'drag-moving', start: markTimes['drag-moving'], end: markTimes['drag-moved'] },
    { name: 'drag-end', start: markTimes['drag-moved'], end: markTimes['drag-end'] },
  ];
  
  for (const phase of phases) {
    const phaseTasks = longTasks.filter(t => t.startTime >= (phase.start - 100) && t.startTime <= (phase.end + 100));
    const maxDur = phaseTasks.length ? Math.max(...phaseTasks.map(t => t.duration)) : 0;
    const totalDur = phaseTasks.reduce((sum, t) => sum + t.duration, 0);
    console.log(`  ${phase.name}: ${phaseTasks.length} tasks, max=${maxDur.toFixed(0)}ms, total=${totalDur.toFixed(0)}ms`);
    if (phaseTasks.length > 0) {
      for (const t of phaseTasks.sort((a,b) => b.duration - a.duration).slice(0, 3)) {
        console.log(`    start=${t.startTime.toFixed(0)}ms dur=${t.duration.toFixed(0)}ms`);
      }
    }
  }
  
  // Unphased tasks
  const allPhasedStarts = new Set();
  for (const phase of phases) {
    longTasks.filter(t => t.startTime >= (phase.start - 100) && t.startTime <= (phase.end + 100))
      .forEach(t => allPhasedStarts.add(t.startTime));
  }
  const unphasedTasks = longTasks.filter(t => !allPhasedStarts.has(t.startTime));
  if (unphasedTasks.length > 0) {
    console.log(`  unphased: ${unphasedTasks.length} tasks, max=${Math.max(...unphasedTasks.map(t=>t.duration)).toFixed(0)}ms`);
    for (const t of unphasedTasks.sort((a,b) => b.duration - a.duration).slice(0, 5)) {
      console.log(`    start=${t.startTime.toFixed(0)}ms dur=${t.duration.toFixed(0)}ms`);
    }
  }
  
  console.log(`\nPiece render count (since page load): ${renderCount}`);
  console.log(`Total wall time: ${(t1 - t0).toFixed(0)}ms`);
  
  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
