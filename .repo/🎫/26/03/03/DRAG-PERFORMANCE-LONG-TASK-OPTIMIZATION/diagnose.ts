import { test, expect, type Page } from "@playwright/test";

async function waitForDiagramStabilization(page: Page, maxWaitMs: number = 5000): Promise<void> {
  const startTime = Date.now();
  let lastPositions: Map<string, { x: number; y: number }> = new Map();
  while (Date.now() - startTime < maxWaitMs) {
    await page.waitForTimeout(500);
    const currentPositions = await page.evaluate(() => {
      const nodes = document.querySelectorAll(".react-flow__node");
      const positions: Record<string, { x: number; y: number }> = {};
      nodes.forEach((node) => {
        const id = node.getAttribute("data-id");
        if (id) {
          const style = (node as HTMLElement).style;
          const transform = style.transform;
          const match = transform.match(/translate\(([^,]+)px,\s*([^)]+)px\)/);
          if (match) {
            positions[id] = { x: parseFloat(match[1]), y: parseFloat(match[2]) };
          }
        }
      });
      return positions;
    });
    const currentMap = new Map(Object.entries(currentPositions));
    let stable = true;
    if (lastPositions.size > 0 && currentMap.size === lastPositions.size) {
      for (const [id, pos] of currentMap.entries()) {
        const lastPos = lastPositions.get(id);
        if (lastPos && (Math.abs(pos.x - lastPos.x) > 1 || Math.abs(pos.y - lastPos.y) > 1)) {
          stable = false;
          break;
        }
      }
      if (stable) return;
    }
    lastPositions = currentMap;
  }
}

async function initConsole(page: Page) {
  const messages: string[] = [];
  const warnings: string[] = [];
  const errors: string[] = [];
  page.on("console", (msg) => {
    if (msg.type() === "warning") warnings.push(msg.text());
    else if (msg.type() === "error") errors.push(msg.text());
    else messages.push(msg.text());
  });
  page.on("pageerror", (error) => errors.push(String(error)));
  return { messages, warnings, errors };
}

async function initDesign(page: Page) {
  await page.goto("http://127.0.0.1:5173");
  await page.waitForLoadState("networkidle");
  const fileInput = page.locator('input[type="file"]');
  if ((await fileInput.count()) > 0) {
    await fileInput.setInputFiles("/workspaces/semio/assets/compose/metabolism.zip");
    await page.waitForTimeout(3000);
  }
  const kitLink = page.locator('a[href*="/kits/"]').first();
  if (await kitLink.isVisible({ timeout: 5000 }).catch(() => false)) {
    await kitLink.click();
    await page.waitForTimeout(2000);
  }
  const designLink = page.locator('a[href*="/designs/"]').first();
  if (await designLink.isVisible({ timeout: 5000 }).catch(() => false)) {
    await designLink.click();
    await page.waitForTimeout(2000);
  }
}

test("Diagnose Long Tasks", async ({ page }) => {
  test.setTimeout(120000);
  const { errors } = await initConsole(page);
  await initDesign(page);
  await page.waitForLoadState("networkidle");
  await page.waitForTimeout(3000);

  const diagramContainer = page.locator("#diagram .react-flow").first();
  await diagramContainer.isVisible({ timeout: 30000 }).catch(() => false);
  const pieceNodes = diagramContainer.locator(".react-flow__node");
  await pieceNodes.first().waitFor({ state: "attached", timeout: 60000 });
  await waitForDiagramStabilization(page, 10000);

  // Get render count before
  const renderCountBefore = await page.evaluate(() => (globalThis as any).__DEBUG_PIECE_RENDER_COUNT__ ?? 0);
  console.log(`[DEBUG] Render count before: ${renderCountBefore}`);

  // Clear long task log
  await page.evaluate(() => {
    (globalThis as any).__DEBUG_LONG_TASK_LOG__ = [];
  });

  // Phase 1: Zoom
  const pane = diagramContainer.locator(".react-flow__pane").first();
  const paneBox = await pane.boundingBox();
  const zoomAnchorX = paneBox!.x + paneBox!.width / 2;
  const zoomAnchorY = paneBox!.y + paneBox!.height / 2;
  await page.mouse.move(zoomAnchorX, zoomAnchorY);
  await page.mouse.wheel(0, -600);
  await page.waitForTimeout(500);

  const renderCountAfterZoomIn = await page.evaluate(() => (globalThis as any).__DEBUG_PIECE_RENDER_COUNT__ ?? 0);
  const zoomInLogs = await page.evaluate(() => JSON.stringify((globalThis as any).__DEBUG_LONG_TASK_LOG__ ?? []));
  console.log(`[DEBUG] After zoom-in: renders=${renderCountAfterZoomIn}, longTasks=${zoomInLogs}`);
  await page.evaluate(() => {
    (globalThis as any).__DEBUG_LONG_TASK_LOG__ = [];
  });

  await page.mouse.wheel(0, 600);
  await page.waitForTimeout(500);

  const renderCountAfterZoomOut = await page.evaluate(() => (globalThis as any).__DEBUG_PIECE_RENDER_COUNT__ ?? 0);
  const zoomOutLogs = await page.evaluate(() => JSON.stringify((globalThis as any).__DEBUG_LONG_TASK_LOG__ ?? []));
  console.log(`[DEBUG] After zoom-out: renders=${renderCountAfterZoomOut}, longTasks=${zoomOutLogs}`);
  await page.evaluate(() => {
    (globalThis as any).__DEBUG_LONG_TASK_LOG__ = [];
  });

  // Phase 2: Drag
  const firstNode = pieceNodes.first();
  const nodeBox = await firstNode.boundingBox();
  const startX = nodeBox!.x + nodeBox!.width / 2;
  const startY = nodeBox!.y + nodeBox!.height / 2;
  const targetX = startX + 100;
  const targetY = startY;

  // Move to node
  await page.mouse.move(startX, startY);
  await page.waitForTimeout(50);

  const renderCountBeforeDrag = await page.evaluate(() => (globalThis as any).__DEBUG_PIECE_RENDER_COUNT__ ?? 0);
  console.log(`[DEBUG] Before drag: renders=${renderCountBeforeDrag}`);
  await page.evaluate(() => {
    (globalThis as any).__DEBUG_LONG_TASK_LOG__ = [];
  });

  // Mouse down
  await page.mouse.down();
  await page.waitForTimeout(100);

  const renderCountAfterDown = await page.evaluate(() => (globalThis as any).__DEBUG_PIECE_RENDER_COUNT__ ?? 0);
  const downLogs = await page.evaluate(() => JSON.stringify((globalThis as any).__DEBUG_LONG_TASK_LOG__ ?? []));
  console.log(`[DEBUG] After mouse.down: renders=${renderCountAfterDown}, longTasks=${downLogs}`);
  await page.evaluate(() => {
    (globalThis as any).__DEBUG_LONG_TASK_LOG__ = [];
  });

  // Drag move
  await page.mouse.move(targetX, targetY, { steps: 20 });
  await page.waitForTimeout(200);

  const renderCountAfterMove = await page.evaluate(() => (globalThis as any).__DEBUG_PIECE_RENDER_COUNT__ ?? 0);
  const moveLogs = await page.evaluate(() => JSON.stringify((globalThis as any).__DEBUG_LONG_TASK_LOG__ ?? []));
  console.log(`[DEBUG] After mouse.move: renders=${renderCountAfterMove}, longTasks=${moveLogs}`);

  // Mouse up
  await page.mouse.up();
  await page.waitForTimeout(500);

  const renderCountAfterUp = await page.evaluate(() => (globalThis as any).__DEBUG_PIECE_RENDER_COUNT__ ?? 0);
  const upLogs = await page.evaluate(() => JSON.stringify((globalThis as any).__DEBUG_LONG_TASK_LOG__ ?? []));
  console.log(`[DEBUG] After mouse.up: renders=${renderCountAfterUp}, longTasks=${upLogs}`);
});
