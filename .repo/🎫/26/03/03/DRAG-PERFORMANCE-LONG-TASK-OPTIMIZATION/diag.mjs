import { chromium } from 'playwright';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const BASE_URL = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:5173';
const ZIP_PATH = path.resolve('/workspaces/semio/assets/compose/metabolism.zip');
async function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  
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

  await page.goto(BASE_URL);
  await page.waitForLoadState('domcontentloaded');
  await sleep(2000);
  
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
  await fileInput.waitFor({ state: 'attached', timeout: 10000 });
  const [fileChooser] = await Promise.all([
    page.waitForEvent('filechooser', { timeout: 5000 }).catch(() => null),
    fileInput.dispatchEvent('click'),
  ]);
  if (fileChooser) await fileChooser.setFiles(ZIP_PATH);
  else await fileInput.setInputFiles(ZIP_PATH);
  
  await page.getByText('Metabolism', { exact: true }).first().waitFor({ state: 'visible', timeout: 60000 });
  await sleep(500);
  
  const tableRow = page.locator('tr[data-row-id]').filter({ hasText: 'Metabolism' }).first();
  if (await tableRow.isVisible().catch(() => false)) await tableRow.dblclick({ force: true });
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await page.waitForLoadState('networkidle');
  await sleep(3000);
  
  const allRowIds = await page.evaluate(() => Array.from(document.querySelectorAll('[data-row-id]')).map(el => el.getAttribute('data-row-id')).slice(0, 20));
  const designRowIds = allRowIds.filter(id => id?.startsWith('design-'));
  const nakaginRowId = designRowIds.find(id => id?.includes('9a890dd4')) || designRowIds[designRowIds.length - 1];
  await page.evaluate((rowId) => { const row = document.querySelector(`[data-row-id="${rowId}"]`); if (row) row.dispatchEvent(new MouseEvent('dblclick', { bubbles: true, cancelable: true, view: window })); }, nakaginRowId);
  await page.waitForLoadState('networkidle');
  await sleep(5000);
  
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
  
  const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
  if (await leftPanelToggle.isVisible().catch(() => false)) {
    const leftPanelOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
    if (leftPanelOpen) { await leftPanelToggle.click(); await sleep(500); }
  }
  
  await page.evaluate(() => { window.__COMPOSE_PERFORMANCE__.longTasks = []; });
  
  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const startX = nodeBox.x + nodeBox.width / 2;
  const startY = nodeBox.y + nodeBox.height / 2;

  // TEST 1: Single fast mouse.move (no steps)  
  console.log('\n=== TEST 1: Single mouse.move ===');
  await page.evaluate(() => { window.__COMPOSE_PERFORMANCE__.longTasks = []; });
  await page.mouse.move(startX, startY);
  await sleep(100);
  await page.mouse.down();
  await sleep(100);
  await page.evaluate(() => performance.mark('test1-start'));
  await page.mouse.move(startX + 100, startY);
  await page.mouse.up();
  await sleep(3000);
  await page.evaluate(() => performance.mark('test1-end'));
  let lt1 = await page.evaluate(() => {
    const s = window.__COMPOSE_PERFORMANCE__;
    const m = performance.getEntriesByName('test1-start')[0]?.startTime ?? 0;
    const mEnd = performance.getEntriesByName('test1-end')[0]?.startTime ?? 0;
    const tasks = s.longTasks.filter(t => t.startTime >= m - 100 && t.startTime <= mEnd + 100);
    return { count: tasks.length, max: tasks.length ? Math.max(...tasks.map(t=>t.duration)) : 0, total: tasks.reduce((s,t) => s + t.duration, 0) };
  });
  console.log(`  Single move: ${lt1.count} tasks, max=${lt1.max.toFixed(0)}ms, total=${lt1.total.toFixed(0)}ms`);

  // Reset position  
  await sleep(1000);
  
  // TEST 2: Drag with steps:1
  console.log('\n=== TEST 2: steps=1 ===');
  await page.evaluate(() => { window.__COMPOSE_PERFORMANCE__.longTasks = []; performance.clearMarks(); });
  await page.mouse.move(startX, startY); await sleep(100);
  await page.mouse.down(); await sleep(100);
  await page.evaluate(() => performance.mark('test2-start'));
  await page.mouse.move(startX + 50, startY, { steps: 1 });
  await page.mouse.up();
  await sleep(3000);
  await page.evaluate(() => performance.mark('test2-end'));
  let lt2 = await page.evaluate(() => {
    const s = window.__COMPOSE_PERFORMANCE__;
    const m = performance.getEntriesByName('test2-start')[0]?.startTime ?? 0;
    const mEnd = performance.getEntriesByName('test2-end')[0]?.startTime ?? 0;
    const tasks = s.longTasks.filter(t => t.startTime >= m - 100 && t.startTime <= mEnd + 100);
    return { count: tasks.length, max: tasks.length ? Math.max(...tasks.map(t=>t.duration)) : 0, tasks: tasks.sort((a,b)=>b.duration-a.duration).slice(0,3) };
  });
  console.log(`  steps=1: ${lt2.count} tasks, max=${lt2.max.toFixed(0)}ms`);
  lt2.tasks.forEach(t => console.log(`    start=${t.startTime.toFixed(0)}ms dur=${t.duration.toFixed(0)}ms`));

  // TEST 3: Drag with steps:20 (original)
  console.log('\n=== TEST 3: steps=20 ===');
  await page.evaluate(() => { window.__COMPOSE_PERFORMANCE__.longTasks = []; performance.clearMarks(); });  
  await page.mouse.move(startX, startY); await sleep(100);
  await page.mouse.down(); await sleep(100);
  await page.evaluate(() => performance.mark('test3-start'));
  await page.mouse.move(startX + 100, startY, { steps: 20 });
  await page.mouse.up();
  await sleep(3000);
  await page.evaluate(() => performance.mark('test3-end'));
  let lt3 = await page.evaluate(() => {
    const s = window.__COMPOSE_PERFORMANCE__;
    const m = performance.getEntriesByName('test3-start')[0]?.startTime ?? 0;
    const mEnd = performance.getEntriesByName('test3-end')[0]?.startTime ?? 0;
    const tasks = s.longTasks.filter(t => t.startTime >= m - 100 && t.startTime <= mEnd + 100);
    return { count: tasks.length, max: tasks.length ? Math.max(...tasks.map(t=>t.duration)) : 0, tasks: tasks.sort((a,b)=>b.duration-a.duration).slice(0,3) };
  });
  console.log(`  steps=20: ${lt3.count} tasks, max=${lt3.max.toFixed(0)}ms`);
  lt3.tasks.forEach(t => console.log(`    start=${t.startTime.toFixed(0)}ms dur=${t.duration.toFixed(0)}ms`));

  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
