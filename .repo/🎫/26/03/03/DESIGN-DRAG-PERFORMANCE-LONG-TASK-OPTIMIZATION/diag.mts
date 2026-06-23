import { chromium } from 'playwright';

const SKETCHPAD_URL = 'http://localhost:5173';

async function main() {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();

  await page.addInitScript(() => {
    (window as any).__COMPOSE_PERFORMANCE__ = {
      longTaskSupported: false,
      longTasks: [],
      markers: [],
    };
    const store = (window as any).__COMPOSE_PERFORMANCE__;
    const observerConstructor = (window as any).PerformanceObserver;
    const supportedEntryKinds = observerConstructor?.supportedEntryTypes ?? [];
    if (!observerConstructor || !supportedEntryKinds.includes("longtask")) return;
    store.longTaskSupported = true;
    const observer = new observerConstructor((entryList: any) => {
      const entries = entryList.getEntries().map((entry: any) => ({
        duration: entry.duration,
        startTime: entry.startTime,
      }));
      store.longTasks.push(...entries);
    });
    observer.observe({ entryTypes: ["longtask"] });
  });

  console.log('[DIAG] Navigating...');
  await page.goto(SKETCHPAD_URL, { waitUntil: 'networkidle', timeout: 60000 });

  const designNavItem = page.locator('text=Nakagin').first();
  if (await designNavItem.isVisible({ timeout: 5000 }).catch(() => false)) {
    await designNavItem.click();
  }

  console.log('[DIAG] Waiting for diagram...');
  const diagramContainer = page.locator('#diagram .react-flow').first();
  await diagramContainer.waitFor({ state: 'visible', timeout: 60000 });
  const pieceNodes = diagramContainer.locator('.react-flow__node');
  await pieceNodes.first().waitFor({ state: 'attached', timeout: 60000 });
  console.log(`[DIAG] Found ${await pieceNodes.count()} nodes`);

  await page.waitForTimeout(5000);

  const leftPanelToggle = page.locator('[id="compose.sketchpad.navbar.panelToggle.leftSidePanel"]');
  if (await leftPanelToggle.isVisible().catch(() => false)) {
    const leftPanelOpen = await page.locator('[data-panel="leftSidePanel"]').isVisible().catch(() => false);
    if (leftPanelOpen) {
      await leftPanelToggle.click();
      await page.waitForTimeout(500);
    }
  }

  // Clear long tasks
  await page.evaluate(() => {
    const store = (window as any).__COMPOSE_PERFORMANCE__;
    store.longTasks = [];
    store.markers = [];
  });

  // Mark time before zoom
  await page.evaluate(() => {
    (window as any).__COMPOSE_PERFORMANCE__.markers.push({ event: 'before_zoom_in', time: performance.now() });
  });

  // Zoom in
  const pane = diagramContainer.locator('.react-flow__pane').first();
  const paneBox = await pane.boundingBox();
  const cx = paneBox!.x + paneBox!.width / 2;
  const cy = paneBox!.y + paneBox!.height / 2;
  await page.mouse.move(cx, cy);
  await page.mouse.wheel(0, -600);
  await page.waitForTimeout(500);

  await page.evaluate(() => {
    (window as any).__COMPOSE_PERFORMANCE__.markers.push({ event: 'after_zoom_in', time: performance.now() });
  });

  // Zoom out
  await page.mouse.wheel(0, 600);
  await page.waitForTimeout(500);

  await page.evaluate(() => {
    (window as any).__COMPOSE_PERFORMANCE__.markers.push({ event: 'after_zoom_out', time: performance.now() });
  });

  // Drag
  const firstNode = pieceNodes.first();
  const nodeBox = await firstNode.boundingBox();
  const startX = nodeBox!.x + nodeBox!.width / 2;
  const startY = nodeBox!.y + nodeBox!.height / 2;

  await page.mouse.move(startX, startY);
  await page.waitForTimeout(50);
  await page.mouse.down();
  await page.waitForTimeout(50);

  await page.evaluate(() => {
    (window as any).__COMPOSE_PERFORMANCE__.markers.push({ event: 'before_drag', time: performance.now() });
  });

  await page.mouse.move(startX + 100, startY, { steps: 20 });

  await page.evaluate(() => {
    (window as any).__COMPOSE_PERFORMANCE__.markers.push({ event: 'after_drag_move', time: performance.now() });
  });

  await page.mouse.up();

  await page.evaluate(() => {
    (window as any).__COMPOSE_PERFORMANCE__.markers.push({ event: 'after_mouse_up', time: performance.now() });
  });

  // Read immediately
  const result = await page.evaluate(() => {
    const store = (window as any).__COMPOSE_PERFORMANCE__;
    return {
      longTasks: store.longTasks,
      markers: store.markers,
    };
  });

  console.log('\n[DIAG] === MARKERS ===');
  for (const m of result.markers) {
    console.log(`  ${m.event}: ${m.time.toFixed(1)}ms`);
  }

  const markerMap = new Map(result.markers.map((m: any) => [m.event, m.time]));
  const beforeZoom = markerMap.get('before_zoom_in') || 0;
  const afterZoomIn = markerMap.get('after_zoom_in') || 0;
  const afterZoomOut = markerMap.get('after_zoom_out') || 0;
  const beforeDrag = markerMap.get('before_drag') || 0;
  const afterDragMove = markerMap.get('after_drag_move') || 0;
  const afterMouseUp = markerMap.get('after_mouse_up') || 0;

  console.log('\n[DIAG] === LONG TASKS BY PHASE ===');
  const zoomTasks: any[] = [];
  const dragTasks: any[] = [];
  const postDragTasks: any[] = [];
  const otherTasks: any[] = [];

  for (const task of result.longTasks) {
    const taskEnd = task.startTime + task.duration;
    if (task.startTime >= beforeZoom && taskEnd <= afterZoomOut + 100) {
      zoomTasks.push(task);
    } else if (task.startTime >= beforeDrag && task.startTime < afterMouseUp) {
      dragTasks.push(task);
    } else if (task.startTime >= afterMouseUp) {
      postDragTasks.push(task);
    } else {
      otherTasks.push(task);
    }
  }

  console.log(`\n  ZOOM tasks (${zoomTasks.length}):`);
  for (const t of zoomTasks.sort((a: any, b: any) => b.duration - a.duration).slice(0, 10)) {
    console.log(`    ${t.duration.toFixed(1)}ms @ ${t.startTime.toFixed(1)}ms`);
  }

  console.log(`\n  DRAG tasks (${dragTasks.length}):`);
  for (const t of dragTasks.sort((a: any, b: any) => b.duration - a.duration).slice(0, 10)) {
    console.log(`    ${t.duration.toFixed(1)}ms @ ${t.startTime.toFixed(1)}ms`);
  }

  console.log(`\n  POST-DRAG tasks (${postDragTasks.length}):`);
  for (const t of postDragTasks.sort((a: any, b: any) => b.duration - a.duration).slice(0, 10)) {
    console.log(`    ${t.duration.toFixed(1)}ms @ ${t.startTime.toFixed(1)}ms`);
  }

  console.log(`\n  OTHER tasks (${otherTasks.length}):`);
  for (const t of otherTasks.sort((a: any, b: any) => b.duration - a.duration).slice(0, 5)) {
    console.log(`    ${t.duration.toFixed(1)}ms @ ${t.startTime.toFixed(1)}ms`);
  }

  const maxTask = result.longTasks.reduce((max: any, t: any) => t.duration > max.duration ? t : max, { duration: 0, startTime: 0 });
  console.log(`\n[DIAG] MAX TASK: ${maxTask.duration.toFixed(1)}ms @ ${maxTask.startTime.toFixed(1)}ms`);
  console.log(`[DIAG] TOTAL TASKS: ${result.longTasks.length}`);

  // Wait 10 seconds and check for post-drag cascades
  await page.waitForTimeout(10000);
  const laterResult = await page.evaluate(() => {
    const store = (window as any).__COMPOSE_PERFORMANCE__;
    return store.longTasks;
  });
  
  const newTasks = laterResult.filter((t: any) => t.startTime > afterMouseUp + 1000);
  console.log(`\n[DIAG] TASKS AFTER 1s+ post-mouseup (${newTasks.length}):`);
  for (const t of newTasks.sort((a: any, b: any) => b.duration - a.duration).slice(0, 10)) {
    console.log(`    ${t.duration.toFixed(1)}ms @ ${t.startTime.toFixed(1)}ms`);
  }

  await browser.close();
}

main().catch(console.error);
