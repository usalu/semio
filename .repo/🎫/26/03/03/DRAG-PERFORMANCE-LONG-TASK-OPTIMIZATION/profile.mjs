import { chromium } from 'playwright';

const BASE_URL = 'http://127.0.0.1:5173';

(async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  
  // Install PerformanceObserver
  await page.addInitScript(() => {
    window.__PERF__ = { tasks: [], marks: [] };
    const obs = new PerformanceObserver((list) => {
      for (const entry of list.getEntries()) {
        window.__PERF__.tasks.push({ start: entry.startTime, dur: entry.duration });
      }
    });
    if (PerformanceObserver.supportedEntryTypes?.includes('longtask')) {
      obs.observe({ entryTypes: ['longtask'] });
    }
  });

  // Navigate
  await page.goto(BASE_URL);
  await page.waitForLoadState('networkidle');
  
  // Load metabolism kit
  const fileInput = page.locator('input[type="file"]');
  if (await fileInput.count() === 0) {
    // Need to trigger file chooser
    const [fileChooser] = await Promise.all([
      page.waitForEvent('filechooser', { timeout: 10000 }),
      page.locator('button:has-text("Open"), [aria-label*="open"], [data-testid*="open"]').first().click().catch(() => {}),
    ]).catch(() => [null]);
    if (fileChooser) {
      await fileChooser.setFiles('/workspaces/semio/compose/assets/compose/metabolism.zip');
    }
  }

  // Wait for kit to load
  await page.waitForTimeout(5000);
  
  // Navigate to design - find and click on Nakagin Capsule Tower
  const rows = page.locator('tr[data-row-id]');
  const rowCount = await rows.count();
  console.log(`Found ${rowCount} rows`);
  
  for (let i = 0; i < rowCount; i++) {
    const row = rows.nth(i);
    const id = await row.getAttribute('data-row-id');
    if (id?.startsWith('design-')) {
      const text = await row.textContent();
      if (text?.includes('Nakagin') || text?.includes('Capsule')) {
        await row.dblclick();
        break;
      }
    }
  }

  // Wait for design to load
  await page.waitForTimeout(5000);
  
  // Wait for diagram stabilization
  const diagram = page.locator('#diagram .react-flow').first();
  await diagram.waitFor({ state: 'visible', timeout: 60000 });
  const nodes = diagram.locator('.react-flow__node');
  await nodes.first().waitFor({ state: 'attached', timeout: 60000 });
  
  // Wait for positions to stabilize
  let lastPositions = '';
  for (let i = 0; i < 30; i++) {
    await page.waitForTimeout(500);
    const pos = await page.evaluate(() => {
      const nodes = document.querySelectorAll('.react-flow__node');
      return Array.from(nodes).slice(0, 5).map(n => n.getAttribute('style')).join('|');
    });
    if (pos === lastPositions && pos.length > 0) break;
    lastPositions = pos;
  }
  
  console.log(`Nodes count: ${await nodes.count()}`);
  
  // Clear long tasks and mark phases
  await page.evaluate(() => { 
    window.__PERF__.tasks = []; 
    window.__PERF__.marks = [];
    performance.mark('clear');
  });
  
  // Phase 1: Mark + Zoom In
  await page.evaluate(() => { performance.mark('zoom-in-start'); window.__PERF__.marks.push({ name: 'zoom-in-start', time: performance.now() }); });
  const pane = diagram.locator('.react-flow__pane').first();
  const paneBox = await pane.boundingBox();
  await page.mouse.move(paneBox.x + paneBox.width / 2, paneBox.y + paneBox.height / 2);
  await page.mouse.wheel(0, -600);
  await page.waitForTimeout(500);
  await page.evaluate(() => { performance.mark('zoom-in-end'); window.__PERF__.marks.push({ name: 'zoom-in-end', time: performance.now() }); });

  // Phase 2: Zoom Out
  await page.evaluate(() => { performance.mark('zoom-out-start'); window.__PERF__.marks.push({ name: 'zoom-out-start', time: performance.now() }); });
  await page.mouse.wheel(0, 600);
  await page.waitForTimeout(500);
  await page.evaluate(() => { performance.mark('zoom-out-end'); window.__PERF__.marks.push({ name: 'zoom-out-end', time: performance.now() }); });

  // Phase 3: Mouse down (start drag)
  const firstNode = nodes.first();
  const nodeBox = await firstNode.boundingBox();
  const startX = nodeBox.x + nodeBox.width / 2;
  const startY = nodeBox.y + nodeBox.height / 2;
  
  await page.evaluate(() => { performance.mark('mousedown-start'); window.__PERF__.marks.push({ name: 'mousedown-start', time: performance.now() }); });
  await page.mouse.move(startX, startY);
  await page.waitForTimeout(50);
  await page.mouse.down();
  await page.waitForTimeout(50);
  await page.evaluate(() => { performance.mark('mousedown-end'); window.__PERF__.marks.push({ name: 'mousedown-end', time: performance.now() }); });

  // Phase 4: Mouse move (drag) - 20 steps
  await page.evaluate(() => { performance.mark('mousemove-start'); window.__PERF__.marks.push({ name: 'mousemove-start', time: performance.now() }); });
  await page.mouse.move(startX + 100, startY, { steps: 20 });
  await page.evaluate(() => { performance.mark('mousemove-end'); window.__PERF__.marks.push({ name: 'mousemove-end', time: performance.now() }); });

  // Phase 5: Mouse up
  await page.evaluate(() => { performance.mark('mouseup-start'); window.__PERF__.marks.push({ name: 'mouseup-start', time: performance.now() }); });
  await page.mouse.up();
  await page.waitForTimeout(200);
  await page.evaluate(() => { performance.mark('mouseup-end'); window.__PERF__.marks.push({ name: 'mouseup-end', time: performance.now() }); });
  
  // Collect results
  const results = await page.evaluate(() => window.__PERF__);
  
  console.log('\n=== PERFORMANCE MARKS ===');
  for (const mark of results.marks) {
    console.log(`  ${mark.name}: ${mark.time.toFixed(0)}ms`);
  }
  
  console.log('\n=== LONG TASKS ===');
  console.log(`Total: ${results.tasks.length}`);
  if (results.tasks.length > 0) {
    const sorted = [...results.tasks].sort((a, b) => b.dur - a.dur);
    console.log(`Max duration: ${sorted[0].dur.toFixed(0)}ms`);
    console.log(`Top 10 tasks:`);
    for (const t of sorted.slice(0, 10)) {
      // Find which phase this task belongs to
      let phase = 'unknown';
      for (let i = 0; i < results.marks.length - 1; i += 2) {
        const startMark = results.marks[i];
        const endMark = results.marks[i + 1];
        if (t.start >= startMark.time - 100 && t.start <= endMark.time + 100) {
          phase = startMark.name.replace('-start', '');
          break;
        }
      }
      console.log(`  start=${t.start.toFixed(0)}ms dur=${t.dur.toFixed(0)}ms phase=${phase}`);
    }
  }
  
  // Also get render count if available
  const renderCount = await page.evaluate(() => window.__DEBUG_PIECE_RENDER_COUNT__ ?? 'N/A');
  console.log(`\nPiece render count: ${renderCount}`);

  await browser.close();
})().catch(e => { console.error(e); process.exit(1); });
