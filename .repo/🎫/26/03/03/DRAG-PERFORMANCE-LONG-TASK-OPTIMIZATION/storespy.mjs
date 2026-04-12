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

  // Intercept the Zustand store setState to log what changes during mouseup
  await page.evaluate(() => {
    window.__STORE_CALLS__ = [];
    window.__TRACKING__ = false;
    // Find the ReactFlow store - it's on the internals
    const rfWrapper = document.querySelector('.react-flow');
    // Try to access via __reactInternalInstance or traverse
    // Alternative: monkey-patch the Zustand store
    // The ReactFlow store is accessed through useStoreApi which returns store.getState/setState
    // We can find it through the __reactFiber
    function findReactStore(el) {
      const keys = Object.keys(el);
      for (const key of keys) {
        if (key.startsWith('__reactFiber$') || key.startsWith('__reactInternalInstance$')) {
          let fiber = el[key];
          for (let i = 0; i < 50 && fiber; i++) {
            if (fiber.memoizedState) {
              let state = fiber.memoizedState;
              while (state) {
                if (state.queue?.lastRenderedReducer && state.memoizedState?.getState) {
                  return state.memoizedState;
                }
                // Check hooks chain
                if (state.memoizedState?.current?.getState) {
                  return state.memoizedState.current;
                }
                state = state.next;
              }
            }
            fiber = fiber.return;
          }
        }
      }
      return null;
    }
    const store = findReactStore(rfWrapper);
    if (store) {
      const origSetState = store.setState.bind(store);
      store.setState = function(partial, replace) {
        if (window.__TRACKING__) {
          const keys = typeof partial === 'function' ? ['<function>'] : Object.keys(partial);
          window.__STORE_CALLS__.push({ keys, ts: performance.now() });
        }
        return origSetState(partial, replace);
      };
      console.log('[DEBUG] Store intercepted successfully');
    } else {
      console.log('[DEBUG] Could not find ReactFlow store');
    }
  });

  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const sx = nodeBox.x + nodeBox.width / 2;
  const sy = nodeBox.y + nodeBox.height / 2;

  // Start tracking
  await page.evaluate(() => { window.__TRACKING__ = true; window.__STORE_CALLS__ = []; });
  
  // Perform drag: down → move → up
  await page.mouse.move(sx, sy);
  await sleep(50);
  await page.mouse.down();
  await sleep(100);
  
  // Clear store calls from mousedown
  const downCalls = await page.evaluate(() => { const c = [...window.__STORE_CALLS__]; window.__STORE_CALLS__ = []; return c; });
  console.log('\n=== MOUSEDOWN STORE CALLS ===');
  for (const c of downCalls) console.log(`  keys=${c.keys.join(',')}`);

  await page.mouse.move(sx + 100, sy, { steps: 1 });
  await sleep(100);
  
  const moveCalls = await page.evaluate(() => { const c = [...window.__STORE_CALLS__]; window.__STORE_CALLS__ = []; return c; });
  console.log('\n=== MOUSEMOVE STORE CALLS ===');
  for (const c of moveCalls) console.log(`  keys=${c.keys.join(',')}`);

  await page.mouse.up();
  await sleep(5000);
  
  const upCalls = await page.evaluate(() => { const c = [...window.__STORE_CALLS__]; window.__STORE_CALLS__ = []; return c; });
  console.log('\n=== MOUSEUP STORE CALLS ===');
  for (const c of upCalls) console.log(`  keys=${c.keys.join(',')}`);

  await page.evaluate(() => { window.__TRACKING__ = false; });

  const lt = await page.evaluate(() => {
    const s = window.__SEMIO_PERFORMANCE__;
    return { count: s.longTasks.length, tasks: s.longTasks.sort((a,b) => b.duration - a.duration).slice(0,3) };
  });
  console.log(`\nLong tasks: ${lt.count}`);
  for (const t of lt.tasks) console.log(`  dur=${t.duration.toFixed(0)}ms start=${t.startTime.toFixed(0)}ms`);

  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
