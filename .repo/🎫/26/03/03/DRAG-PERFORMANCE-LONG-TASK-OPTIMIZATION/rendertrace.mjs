import { chromium } from 'playwright';
import path from 'node:path';
const BASE_URL = process.env.PLAYWRIGHT_BASE_URL || 'http://127.0.0.1:5173';
const ZIP_PATH = path.resolve('/workspaces/semio/assets/compose/metabolism.zip');
async function sleep(ms) { return new Promise(r => setTimeout(r, ms)); }
(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  
  // Hook into React's fiber internals to trace component renders
  await page.addInitScript(() => {
    window.__COMPOSE_PERFORMANCE__ = { longTaskSupported: false, longTasks: [] };
    const s = window.__COMPOSE_PERFORMANCE__;
    const obs = window.PerformanceObserver;
    if (obs && (obs.supportedEntryTypes || []).includes('longtask')) {
      s.longTaskSupported = true;
      new obs(list => { s.longTasks.push(...list.getEntries().map(e => ({ duration: e.duration, startTime: e.startTime }))); }).observe({ entryTypes: ['longtask'] });
    }
    
    // Monkey-patch React devtools hook to trace commits
    window.__RENDER_LOG__ = [];
    window.__TRACKING_RENDERS__ = false;
    
    // Set up a profiling hook
    const origDefineProperty = Object.defineProperty;
    Object.defineProperty = function(obj, prop, desc) {
      if (prop === '__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED' && desc.value) {
        const orig = desc.value;
        // Intercept React internals
      }
      return origDefineProperty.call(this, obj, prop, desc);
    };
  });
  
  await page.goto(BASE_URL);
  await page.waitForLoadState('domcontentloaded');
  await sleep(2000);
  const fileInput = page.locator('[id="compose.sketchpad.app.home.importKit"]');
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
  await page.evaluate(() => { window.__COMPOSE_PERFORMANCE__.longTasks = []; });

  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const sx = nodeBox.x + nodeBox.width / 2;
  const sy = nodeBox.y + nodeBox.height / 2;

  // Use CDP tracing with more detail to capture React commit phases
  const cdp = await context.newCDPSession(page);
  
  // Phase 1: drag start + move (normal)
  await page.mouse.move(sx, sy); await sleep(50);
  await page.mouse.down(); await sleep(200);
  await page.mouse.move(sx + 100, sy, { steps: 1 }); await sleep(200);
  
  // Phase 2: Start tracing for mouseup
  await cdp.send('Tracing.start', {
    traceConfig: { 
      includedCategories: ['devtools.timeline', 'v8.execute', 'blink.user_timing'],
      recordMode: 'recordAsMuchAsPossible'
    }
  });
  
  await page.mouse.up();
  await sleep(5000);
  
  const { stream } = await cdp.send('Tracing.end');
  
  // Collect trace events
  const traceEvents = [];
  await new Promise((resolve) => {
    cdp.on('Tracing.tracingComplete', resolve);
    cdp.on('Tracing.dataCollected', ({ value }) => {
      traceEvents.push(...value);
    });
  });
  
  // Analyze: find all events >50ms
  const bigEvents = traceEvents
    .filter(e => e.dur && e.dur > 50000) // >50ms
    .sort((a, b) => b.dur - a.dur)
    .slice(0, 30);
  
  console.log('\n=== EVENTS >50ms (sorted by duration) ===');
  for (const e of bigEvents) {
    const url = e.args?.data?.url || e.args?.data?.functionName || '';
    const depth = e.args?.data?.stackTrace?.length || 0;
    console.log(`  ${e.name} dur=${(e.dur/1000).toFixed(0)}ms cat=${e.cat} url=${url.substring(0,60)} depth=${depth}`);
  }

  // Find FunctionCall events during mouseup to identify which functions are slow
  const mouseUpEvents = traceEvents.filter(e => e.name === 'EventDispatch' && e.args?.data?.type === 'mouseup');
  if (mouseUpEvents.length > 0) {
    const mouseUpStart = mouseUpEvents[0].ts;
    const mouseUpEnd = mouseUpStart + (mouseUpEvents[0].dur || 0);
    
    // Find all FunctionCall events within the mouseup timeframe
    const functionsInMouseUp = traceEvents
      .filter(e => e.name === 'FunctionCall' && e.ts >= mouseUpStart && e.ts <= mouseUpEnd && e.dur > 10000)
      .sort((a, b) => b.dur - a.dur)
      .slice(0, 20);
    
    console.log(`\n=== FUNCTIONS IN MOUSEUP (>10ms) ===`);
    for (const e of functionsInMouseUp) {
      const url = e.args?.data?.url || '';
      const fn = e.args?.data?.functionName || '';
      console.log(`  FunctionCall dur=${(e.dur/1000).toFixed(0)}ms fn=${fn} url=${url.substring(url.lastIndexOf('/')+1, url.lastIndexOf('/')+40)}`);
    }
    
    // Find UserTiming events (React marks)
    const userTimings = traceEvents
      .filter(e => e.cat?.includes('blink.user_timing') && e.ts >= mouseUpStart && e.ts <= mouseUpEnd)
      .sort((a, b) => a.ts - b.ts);
    
    console.log(`\n=== USER TIMING MARKS IN MOUSEUP ===`);
    for (const e of userTimings.slice(0, 30)) {
      console.log(`  ${e.name} ts=${((e.ts - mouseUpStart)/1000).toFixed(0)}ms dur=${e.dur ? (e.dur/1000).toFixed(0) + 'ms' : ''} ph=${e.ph}`);
    }
  }

  const lt = await page.evaluate(() => {
    const s = window.__COMPOSE_PERFORMANCE__;
    return { count: s.longTasks.length, tasks: s.longTasks.sort((a,b) => b.duration - a.duration).slice(0,3) };
  });
  console.log(`\nLong tasks: ${lt.count}`);
  for (const t of lt.tasks) console.log(`  dur=${t.duration.toFixed(0)}ms start=${t.startTime.toFixed(0)}ms`);

  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
