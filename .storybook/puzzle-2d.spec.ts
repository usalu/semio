// #region 🧲Header
// 💻 .storybook/puzzle-2d.spec.ts
// Specs: End-to-end checks for the framework renderer's puzzle 2d board host inside the aggregated Storybook static build.
// Summary: Drives the WASM `BoardSession` through real pointer/wheel/keyboard input and asserts against the story's `puzzle2d-board-debug` readout (selection/camera/utility/counts) — GPU pixels are not asserted, only the CPU-side session state the debug readout reflects.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Locator, type Page } from "@playwright/test";

type Puzzle2dBoardDebug = {
  readonly selection: readonly string[];
  readonly camera: { readonly x: number; readonly y: number; readonly zoom: number };
  readonly activeUtility: string;
  readonly nodeCount: number;
  readonly edgeCount: number;
};

type Box = { readonly x: number; readonly y: number; readonly width: number; readonly height: number };

function significantConsoleErrors(messages: string[]): string[] {
  return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function readPuzzle2dDebug(debug: Locator): Promise<Puzzle2dBoardDebug> {
  const text = await debug.innerText();
  return JSON.parse(text) as Puzzle2dBoardDebug;
}

/** @emoji 📐 Canonical `screenX = (worldX - camera.x) * zoom + width / 2` transform shared across board renderers — used to click a known fixture node by its world position. */
function worldToClientPoint(box: Box, camera: Puzzle2dBoardDebug["camera"], world: { readonly x: number; readonly y: number }): { readonly clientX: number; readonly clientY: number } {
  return {
    clientX: box.x + (world.x - camera.x) * camera.zoom + box.width / 2,
    clientY: box.y + (world.y - camera.y) * camera.zoom + box.height / 2,
  };
}

async function expectBoardStory(page: Page, storyId: string): Promise<{ readonly canvas: Locator; readonly debug: Locator }> {
  const pageErrors: Error[] = [];
  const consoleErrors: string[] = [];
  page.on("pageerror", (error) => pageErrors.push(error));
  page.on("console", (message) => {
    if (message.type() === "error") consoleErrors.push(message.text());
  });

  await page.goto(`iframe.html?id=${storyId}&viewMode=story`, { waitUntil: "domcontentloaded" });
  await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
  await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
  await page.waitForFunction(() => {
    const root = document.querySelector("#storybook-root");
    return !!root && root.childElementCount > 0;
  });

  const debug = page.getByTestId("puzzle2d-board-debug");
  await expect(debug).toBeVisible();
  await expect(page.locator(".semio-board-2d-host canvas")).toBeVisible();
  const canvas = page.locator(".semio-board-2d-host canvas");

  expect(pageErrors.map((error) => error.message)).toEqual([]);
  expect(significantConsoleErrors(consoleErrors)).toEqual([]);
  return { canvas, debug };
}

test("puzzle2d overview select: loads with the initial fixture reflected in the debug readout", async ({ page }) => {
  const { debug } = await expectBoardStory(page, "puzzle-2d--overview-select");
  const state = await readPuzzle2dDebug(debug);
  expect(state.selection).toEqual([]);
  expect(state.activeUtility).toBe("select");
  expect(state.nodeCount).toBe(2);
  expect(state.edgeCount).toBe(1);
});

test("puzzle2d overview select: clicking a node selects it, clicking empty space clears it", async ({ page }) => {
  const { canvas, debug } = await expectBoardStory(page, "puzzle-2d--overview-select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  const alphaPoint = worldToClientPoint(box!, before.camera, { x: 0, y: 0 });
  await page.mouse.click(alphaPoint.clientX, alphaPoint.clientY);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual(["alpha"]);

  await page.mouse.click(box!.x + 4, box!.y + 4);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual([]);
});

test("puzzle2d overview select: wheel zoom updates the camera in the debug readout", async ({ page }) => {
  const { canvas, debug } = await expectBoardStory(page, "puzzle-2d--overview-select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
  for (let index = 0; index < 12; index += 1) {
    await page.mouse.wheel(0, -120);
  }
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).camera.zoom).toBeGreaterThan(before.camera.zoom);
});

test("puzzle2d overview select: Delete removes the selected node", async ({ page }) => {
  const { canvas, debug } = await expectBoardStory(page, "puzzle-2d--overview-select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);

  const betaPoint = worldToClientPoint(box!, before.camera, { x: 280, y: 120 });
  await page.mouse.move(betaPoint.clientX, betaPoint.clientY);
  await page.mouse.click(betaPoint.clientX, betaPoint.clientY);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual(["beta"]);

  await page.keyboard.press("Delete");
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).nodeCount).toBe(1);
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection).toEqual([]);
});

test("puzzle2d overview select: Ctrl/Cmd+A selects every node", async ({ page }) => {
  const { canvas, debug } = await expectBoardStory(page, "puzzle-2d--overview-select");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
  await page.keyboard.press("ControlOrMeta+a");
  await expect.poll(async () => (await readPuzzle2dDebug(debug)).selection.length).toBe(2);
});

test("puzzle2d lasso select: story boots with the lasso selection method", async ({ page }) => {
  const { debug } = await expectBoardStory(page, "puzzle-2d--lasso-select");
  const state = await readPuzzle2dDebug(debug);
  expect(state.selection).toEqual([]);
  expect(state.nodeCount).toBe(2);
});

test("puzzle2d brush utility: story boots with the brush utility active", async ({ page }) => {
  const { debug } = await expectBoardStory(page, "puzzle-2d--brush-utility");
  const state = await readPuzzle2dDebug(debug);
  expect(state.activeUtility).toBe("brush");
});

test("puzzle2d forced lod pane: non-interactive pane ignores pointer input", async ({ page }) => {
  const { canvas, debug } = await expectBoardStory(page, "puzzle-2d--forced-lod-pane");
  const box = await canvas.boundingBox();
  expect(box).toBeTruthy();
  const before = await readPuzzle2dDebug(debug);
  await page.mouse.click(box!.x + box!.width / 2, box!.y + box!.height / 2);
  const after = await readPuzzle2dDebug(debug);
  expect(after.selection).toEqual(before.selection);
});
