// #region 🧲Header
// 💻 .storybook/puzzle-2d.spec.ts
// Specs: End-to-end checks for the puzzle 2d canvas inside the aggregated Storybook static build.
// Summary: Covers selection, wheel zoom, LOD labels, and monolithic vs world-clip raster paths.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Locator, type Page } from "@playwright/test";

type Puzzle2dCanvasDebugElement = HTMLCanvasElement & {
  __puzzle2dRenderer?: {
    resolveHit?: (point: { x: number; y: number }) => { id: string } | null;
    scene: {
      edges: Map<string, { curve: { p0: Point; p1: Point; p2: Point; p3: Point } }>;
      getObjectById: (id: string) => { position?: { x: number; y: number }; x?: number; y?: number } | undefined;
    };
    worldToScreen: (point: { x: number; y: number }) => { x: number; y: number };
  };
};

interface Point {
  x: number;
  y: number;
}

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function clickCanvasNormalized(page: Page, canvas: Locator, nx: number, ny: number): Promise<void> {
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  await canvas.click({
    force: true,
    position: {
      x: nx * box!.width,
      y: ny * box!.height,
    },
  });
}

async function puzzle2dObjectClientPoint(page: Page, objectId: string): Promise<{ clientX: number; clientY: number }> {
  const point = await page.evaluate((nextObjectId) => {
    const element = document.querySelector('[data-testid="puzzle2d-canvas"]');
    if (!(element instanceof HTMLCanvasElement)) {
      return null;
    }
    const puzzle2dElement = element as Puzzle2dCanvasDebugElement;
    const renderer = puzzle2dElement.__puzzle2dRenderer;
    const object = renderer?.scene.getObjectById(nextObjectId);
    if (!renderer || !object) {
      return null;
    }
    const worldPoint = object.position ?? (typeof object.x === "number" && typeof object.y === "number" ? { x: object.x, y: object.y } : null);
    if (!worldPoint) {
      return null;
    }
    const screenPoint = renderer.worldToScreen(worldPoint);
    const rect = puzzle2dElement.getBoundingClientRect();
    return {
      clientX: rect.left + screenPoint.x,
      clientY: rect.top + screenPoint.y,
    };
  }, objectId);
  expect(point).toBeTruthy();
  return point!;
}

async function expectPuzzle2dSceneObjectHit(page: Page, objectId: string): Promise<void> {
  await expect
    .poll(async () =>
      page.evaluate((nextObjectId) => {
        const element = document.querySelector('[data-testid="puzzle2d-canvas"]');
        if (!(element instanceof HTMLCanvasElement)) {
          return null;
        }
        const puzzle2dElement = element as Puzzle2dCanvasDebugElement;
        const renderer = puzzle2dElement.__puzzle2dRenderer;
        const object = renderer?.scene.getObjectById(nextObjectId);
        if (!renderer || !object || !renderer.resolveHit) {
          return null;
        }
        const worldPoint = object.position ?? (typeof object.x === "number" && typeof object.y === "number" ? { x: object.x, y: object.y } : null);
        if (!worldPoint) {
          return null;
        }
        return renderer.resolveHit(worldPoint)?.id ?? null;
      }, objectId),
    )
    .toBe(objectId);
}

async function clickPuzzle2dSceneObject(page: Page, objectId: string): Promise<void> {
  await expectPuzzle2dSceneObjectHit(page, objectId);
  const point = await puzzle2dObjectClientPoint(page, objectId);
  await page.mouse.click(point.clientX, point.clientY);
}

function cubicBezierPoint(curve: { p0: Point; p1: Point; p2: Point; p3: Point }, step: number): Point {
  const oneMinusStep = 1 - step;
  const oneMinusSquared = oneMinusStep * oneMinusStep;
  const oneMinusCubed = oneMinusSquared * oneMinusStep;
  const stepSquared = step * step;
  const stepCubed = stepSquared * step;
  return {
    x: curve.p0.x * oneMinusCubed + 3 * curve.p1.x * oneMinusSquared * step + 3 * curve.p2.x * oneMinusStep * stepSquared + curve.p3.x * stepCubed,
    y: curve.p0.y * oneMinusCubed + 3 * curve.p1.y * oneMinusSquared * step + 3 * curve.p2.y * oneMinusStep * stepSquared + curve.p3.y * stepCubed,
  };
}

async function puzzle2dEdgeMidClientPoint(page: Page, edgeId: string): Promise<{ clientX: number; clientY: number }> {
  const point = await page.evaluate((nextEdgeId) => {
    const element = document.querySelector('[data-testid="puzzle2d-canvas"]');
    if (!(element instanceof HTMLCanvasElement)) {
      return null;
    }
    const puzzle2dElement = element as Puzzle2dCanvasDebugElement;
    const renderer = puzzle2dElement.__puzzle2dRenderer;
    const edge = renderer?.scene.edges.get(nextEdgeId);
    if (!renderer || !edge) {
      return null;
    }
    const mid = cubicBezierPoint(edge.curve, 0.5);
    const screenPoint = renderer.worldToScreen(mid);
    const rect = puzzle2dElement.getBoundingClientRect();
    return {
      clientX: rect.left + screenPoint.x,
      clientY: rect.top + screenPoint.y,
    };
  }, edgeId);
  expect(point).toBeTruthy();
  return point!;
}

async function wheelOnCanvasNormalized(page: Page, canvas: Locator, nx: number, ny: number, deltaY: number): Promise<void> {
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const x = box!.x + nx * box!.width;
  const y = box!.y + ny * box!.height;
  await page.mouse.move(x, y);
  await page.mouse.wheel(0, deltaY);
}

function viewportCenterOfCanvasBox(box: { height: number; width: number; x: number; y: number }): [number, number] {
  return [box.x + box.width / 2, box.y + box.height / 2];
}

async function expectBoardStory(page: Page, storyId: string): Promise<Locator> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") {
      consoleErrors.push(message.text());
    }
  });

  await page.goto(`iframe.html?id=${storyId}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });
  const canvas = page.getByTestId("puzzle2d-canvas");
  await expect(canvas).toBeVisible();
  await expect.poll(async () => canvas.getAttribute("data-puzzle2d-zoom"), { timeout: 30000 }).toMatch(/\d+(\.\d+)?/);
  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  return canvas;
}

test("puzzle2d default: selection, zoom in stays on detail LOD while raising zoom", async ({ page }) => {
  const canvas = await expectBoardStory(page, "puzzle-2d--default");
  await expect(canvas).toHaveAttribute("data-puzzle2d-raster", "gpu");
  await expect(canvas).toHaveAttribute("data-puzzle2d-world-tiling", "none");
  await expect(canvas).toHaveAttribute("data-puzzle2d-lod", "detail");

  const initialZoom = Number(await canvas.getAttribute("data-puzzle2d-zoom"));
  expect(initialZoom).toBeCloseTo(1, 1);

  await clickPuzzle2dSceneObject(page, "alpha");
  await expect(canvas).toHaveAttribute("data-2d-selection", "alpha");

  for (let index = 0; index < 18; index += 1) {
    await wheelOnCanvasNormalized(page, canvas, 0.5, 0.5, -120);
  }
  await expect.poll(async () => canvas.getAttribute("data-puzzle2d-lod")).toBe("detail");
  const zoomed = Number(await canvas.getAttribute("data-puzzle2d-zoom"));
  expect(zoomed).toBeGreaterThan(initialZoom);

  await clickCanvasNormalized(page, canvas, 0.04, 0.04);
  await expect(canvas).toHaveAttribute("data-2d-selection", "");
});

test("puzzle2d default: deletes selected node after Delete and keeps scene in sync", async ({ page }) => {
  const canvas = await expectBoardStory(page, "puzzle-2d--default");
  await clickPuzzle2dSceneObject(page, "beta");
  await expect(canvas).toHaveAttribute("data-2d-selection", "beta");
  await page.keyboard.press("Delete");
  await expect
    .poll(async () =>
      page.evaluate(() => {
        const element = document.querySelector('[data-testid="puzzle2d-canvas"]');
        if (!(element instanceof HTMLCanvasElement)) {
          return null;
        }
        const puzzle2dElement = element as Puzzle2dCanvasDebugElement;
        return puzzle2dElement.__puzzle2dRenderer?.scene.getObjectById("beta") ? "present" : "absent";
      }),
    )
    .toBe("absent");
  await expect(canvas).toHaveAttribute("data-2d-selection", "");
});

test("puzzle2d default: deletes selected edge after Delete", async ({ page }) => {
  const canvas = await expectBoardStory(page, "puzzle-2d--default");
  const mid = await puzzle2dEdgeMidClientPoint(page, "link-1");
  await page.mouse.click(mid.clientX, mid.clientY);
  await expect(canvas).toHaveAttribute("data-2d-selection", "link-1");
  await page.keyboard.press("Delete");
  await expect
    .poll(async () =>
      page.evaluate(() => {
        const element = document.querySelector('[data-testid="puzzle2d-canvas"]');
        if (!(element instanceof HTMLCanvasElement)) {
          return null;
        }
        const puzzle2dElement = element as Puzzle2dCanvasDebugElement;
        return puzzle2dElement.__puzzle2dRenderer?.scene.edges.has("link-1") ? "present" : "absent";
      }),
    )
    .toBe("absent");
  await expect(canvas).toHaveAttribute("data-2d-selection", "");
});

test("puzzle2d default: zoom out to overview LOD while preserving wheel anchor", async ({ page }) => {
  const canvas = await expectBoardStory(page, "puzzle-2d--default");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const [cx, cy] = viewportCenterOfCanvasBox(box!);

  const worldBefore = await page.evaluate(
    ([px, py]) => {
      const el = document.querySelector('[data-testid="puzzle2d-canvas"]') as HTMLCanvasElement | null;
      if (!el) {
        return null;
      }
      const rect = el.getBoundingClientRect();
      const screenX = px - rect.left;
      const screenY = py - rect.top;
      const width = el.clientWidth;
      const height = el.clientHeight;
      const zoom = Number(el.dataset.puzzle2dZoom ?? "1");
      const [camXRaw, camYRaw] = (el.getAttribute("data-puzzle2d-camera") ?? "0,0").split(",");
      const camX = Number(camXRaw);
      const camY = Number(camYRaw);
      return {
        x: (screenX - width / 2) / zoom + camX,
        y: (screenY - height / 2) / zoom + camY,
      };
    },
    [cx, cy],
  );
  expect(worldBefore).not.toBeNull();

  for (let index = 0; index < 40; index += 1) {
    await page.mouse.move(cx, cy);
    await page.mouse.wheel(0, 140);
  }
  await expect.poll(async () => canvas.getAttribute("data-puzzle2d-lod")).toBe("overview");

  const worldAfter = await page.evaluate(
    ([px, py]) => {
      const el = document.querySelector('[data-testid="puzzle2d-canvas"]') as HTMLCanvasElement | null;
      if (!el) {
        return null;
      }
      const rect = el.getBoundingClientRect();
      const screenX = px - rect.left;
      const screenY = py - rect.top;
      const width = el.clientWidth;
      const height = el.clientHeight;
      const zoom = Number(el.dataset.puzzle2dZoom ?? "1");
      const [camXRaw, camYRaw] = (el.getAttribute("data-puzzle2d-camera") ?? "0,0").split(",");
      const camX = Number(camXRaw);
      const camY = Number(camYRaw);
      return {
        x: (screenX - width / 2) / zoom + camX,
        y: (screenY - height / 2) / zoom + camY,
      };
    },
    [cx, cy],
  );
  expect(worldAfter).not.toBeNull();
  expect(Math.abs((worldAfter as { x: number }).x - (worldBefore as { x: number }).x)).toBeLessThan(2.5);
  expect(Math.abs((worldAfter as { y: number }).y - (worldBefore as { y: number }).y)).toBeLessThan(2.5);
});

const nakaginCapsuleTowerHubPieceId = "9d18882e-d90b-40de-a171-47cb4564ffa6";

test("puzzle2d nakagin fixture: json scene hub piece selects", async ({ page }) => {
  const canvas = await expectBoardStory(page, "puzzle-2d--nakagin-capsule-tower-flat-selection");
  await expect(canvas).toHaveAttribute("data-puzzle2d-raster", "gpu");
  await expect(canvas).toHaveAttribute("data-puzzle2d-world-tiling", "none");
  await clickPuzzle2dSceneObject(page, nakaginCapsuleTowerHubPieceId);
  await expect(canvas).toHaveAttribute("data-2d-selection", nakaginCapsuleTowerHubPieceId);
});

test("puzzle2d world-clip: raster mode, node selection, handle hit", async ({ page }) => {
  const canvas = await expectBoardStory(page, "puzzle-2d--world-tile-clip");
  await expect(canvas).toHaveAttribute("data-puzzle2d-raster", "gpu");
  await expect(canvas).toHaveAttribute("data-puzzle2d-world-tiling", "world-clip");

  await clickPuzzle2dSceneObject(page, "alpha");
  await expect(canvas).toHaveAttribute("data-2d-selection", "alpha");

  await clickPuzzle2dSceneObject(page, "alpha.out");
  await expect(canvas).toHaveAttribute("data-2d-selection", "alpha.out");
});
