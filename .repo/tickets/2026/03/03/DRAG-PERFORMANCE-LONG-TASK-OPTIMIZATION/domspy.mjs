import { chromium } from 'playwright';
import path from 'node:path';
const BASE_URL = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:5173';
const ZIP_PATH = path.resolve('/workspaces/semio/semio/assets/semio/metabolism.zip');
async function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }
(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  await page.addInitScript(() => {
    window.__SEMIO_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
    const s = window.__SEMIO_PERFORMANCE__;
    const obs = window.PerformanceObserver;
    if (obs && (obs.supportedEntryTypes || []).includes('longtask')) {
      s.longTaskSupported = true;
      new obs(list => { s.longTasks.push(...list.getEntries().map(e => ({ duration: e.duration, startTime: e.startTime }))); }).observe({ entryTypes: ['longtask'] });
    }
  });
  await page.goto(BASE_URL);
  await page.waitForLoadState('domcontentloaded');
  await sleep(2000);
  const fileInput = page.locator('[id="semio.sketchpad.app.home.importKit"]');
  await fileInput.waitFor({ state: 'attached', timeout: 10000 });
  const [fc] = await Promise.all([page.waitForEvent('filechooser', { timeout: 5000 }).catch(() => null), fileInput.dispatchEvent('click')]);
  if (fc) await fc.setFiles(ZIP_PATH); else await fileInput.setInputFiles(ZIP_PATH);
  await page.getByText('Metabolism', { exact: true }).first().waitFor({ state: 'visible', timeout: 60000 });
  await sleep(500);
  const tableRow = page.locator('tr[data-row-id]').filter({ hasText: 'Metabolism' }).first();
  if (await tableRow.isVisible().catch(() => false)) await tableRow.dblclick({ force: true });
  await page.waitForURL(/.*kits\/.+/, { timeout: 30000 });
  await sleep(3000);
  const allRowIds = await page.evaluate(() => Array.from(document.querySelectorAll('[data-row-id]')).map(el => el.getAttribute('data-row-id')).slice(0, 20));
  const designRowIds = allRowIds.filter(id => id?.startsWith('design-'));
  const nakaginRowId = designRowIds.find(id => id?.includes('9a890dd4')) || designRowIds[designRowIds.length - 1];
  await page.evaluate((rowId) => { const row = document.querySelector(`[data-row-id="${rowId}"]`); if (row) row.dispatchEvent(new MouseEvent('dblclick', { bubbles: true, cancelable: true, view: window })); }, nakaginRowId);
  await sleep(8000);
  const diagram = page.locator('#diagram .react-flow').first();
  await diagram.waitFor({ state: 'visible', timeout: 60000 });
  const nodes = diagram.locator('.react-flow__node');
  await nodes.first().waitFor({ state: 'attached', timeout: 60000 });
  let lastPos = '';
  for (let i = 0; i < 30; i++) {
    await sleep(500);
    const pos = await page.evaluate(() => Array.from(document.querySelectorAll('.react-flow__node')).slice(0, 5).map(n => n.getAttribute('style')).join('|'));
    if (pos === lastPos && pos.length > 0) break;
    lastPos = pos;
  }
  console.log(`Nodes: ${await nodes.count()}`);
  await page.evaluate(() => { window.__SEMIO_PERFORMANCE__.longTasks = []; });

  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const sx = nodeBox.x + nodeBox.width / 2;
  const sy = nodeBox.y + nodeBox.height / 2;

  // Setup detailed mutation observer
  await page.evaluate(() => {
    window.__DOM_LOG__ = [];
    window.__DOM_OBS__ = new MutationObserver((mutations) => {
      for (const m of mutations) {
        if (m.type === 'childList') {
          for (const n of m.addedNodes) {
            if (n.nodeType === 1) {
              const tag = n.tagName?.toLowerCase();
              const cls = n.className?.toString?.()?.substring(0, 80) || '';
              const dataId = n.getAttribute?.('data-id') || '';
              const parentCls = m.target?.className?.toString?.()?.substring(0, 60) || '';
              window.__DOM_LOG__.push({ op: 'add', tag, cls, dataId, parentCls });
            }
          }
          for (const n of m.removedNodes) {
            if (n.nodeType === 1) {
              const tag = n.tagName?.toLowerCase();
              const cls = n.className?.toString?.()?.substring(0, 80) || '';
              const dataId = n.getAttribute?.('data-id') || '';
              const parentCls = m.target?.className?.toString?.()?.substring(0, 60) || '';
              window.__DOM_LOG__.push({ op: 'rem', tag, cls, dataId, parentCls });
            }
          }
        }
      }
    });
  });

  // Start observing just before mouseup
  await page.mouse.move(sx, sy); await sleep(50);
  await page.mouse.down(); await sleep(200);
  await page.mouse.move(sx + 100, sy, { steps: 1 }); await sleep(200);
  
  // Now observe mouseup
  await page.evaluate(() => {
    window.__DOM_LOG__ = [];
    window.__DOM_OBS__.observe(document.body, { childList: true, subtree: true });
  });
  await page.mouse.up();
  await sleep(5000);
  await page.evaluate(() => { window.__DOM_OBS__.disconnect(); });

  const log = await page.evaluate(() => window.__DOM_LOG__);
  console.log(`\n=== DOM MUTATIONS DURING MOUSEUP (${log.length} total) ===`);
  
  // Group by class pattern
  const groups = {};
  for (const entry of log) {
    const key = `${entry.op}:${entry.tag}:${entry.cls.substring(0, 40)}`;
    if (!groups[key]) groups[key] = { count: 0, dataIds: [] };
    groups[key].count++;
    if (entry.dataId && groups[key].dataIds.length < 3) groups[key].dataIds.push(entry.dataId);
  }
  const sorted = Object.entries(groups).sort((a,b) => b[1].count - a[1].count);
  for (const [key, val] of sorted.slice(0, 30)) {
    console.log(`  ${key}: ${val.count}x ${val.dataIds.length > 0 ? 'ids:' + val.dataIds.join(',') : ''}`);
  }

  // Also show the first 10 entries to understand order
  console.log('\n=== FIRST 15 ENTRIES ===');
  for (const entry of log.slice(0, 15)) {
    console.log(`  ${entry.op} <${entry.tag}> cls=${entry.cls.substring(0,50)} dataId=${entry.dataId} parent=${entry.parentCls.substring(0,40)}`);
  }

  const lt = await page.evaluate(() => {
    const s = window.__SEMIO_PERFORMANCE__;
    return { count: s.longTasks.length, tasks: s.longTasks.sort((a,b) => b.duration - a.duration).slice(0,3) };
  });
  console.log(`\nLong tasks: ${lt.count}`);
  for (const t of lt.tasks) console.log(`  dur=${t.duration.toFixed(0)}ms start=${t.startTime.toFixed(0)}ms`);

  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
