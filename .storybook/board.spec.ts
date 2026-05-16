// #region 🧲Header
// 💻 .storybook/board.spec.ts
// Specs: End-to-end checks for the elements board canvas inside the aggregated Storybook static build.
// Summary: Covers selection, wheel zoom, LOD labels, and monolithic vs world-clip raster paths.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Locator, type Page } from "@playwright/test";

type BoardCanvasDebugElement = HTMLCanvasElement & {
	__boardRenderer?: {
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

async function boardObjectClientPoint(page: Page, objectId: string): Promise<{ clientX: number; clientY: number }> {
	const point = await page.evaluate((nextObjectId) => {
		const element = document.querySelector('[data-testid="board-canvas"]');
		if (!(element instanceof HTMLCanvasElement)) {
			return null;
		}
		const boardElement = element as BoardCanvasDebugElement;
		const renderer = boardElement.__boardRenderer;
		const object = renderer?.scene.getObjectById(nextObjectId);
		if (!renderer || !object) {
			return null;
		}
		const worldPoint = object.position ?? (typeof object.x === "number" && typeof object.y === "number" ? { x: object.x, y: object.y } : null);
		if (!worldPoint) {
			return null;
		}
		const screenPoint = renderer.worldToScreen(worldPoint);
		const rect = boardElement.getBoundingClientRect();
		return {
			clientX: rect.left + screenPoint.x,
			clientY: rect.top + screenPoint.y,
		};
	}, objectId);
	expect(point).toBeTruthy();
	return point!;
}

async function expectBoardObjectHit(page: Page, objectId: string): Promise<void> {
	await expect
		.poll(async () =>
			page.evaluate((nextObjectId) => {
				const element = document.querySelector('[data-testid="board-canvas"]');
				if (!(element instanceof HTMLCanvasElement)) {
					return null;
				}
				const boardElement = element as BoardCanvasDebugElement;
				const renderer = boardElement.__boardRenderer;
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

async function clickBoardObject(page: Page, objectId: string): Promise<void> {
	await expectBoardObjectHit(page, objectId);
	const point = await boardObjectClientPoint(page, objectId);
	await page.mouse.click(point.clientX, point.clientY);
}

function cubicBezierPoint(curve: { p0: Point; p1: Point; p2: Point; p3: Point }, step: number): Point {
	const oneMinusStep = 1 - step;
	const oneMinusSquared = oneMinusStep * oneMinusStep;
	const oneMinusCubed = oneMinusSquared * oneMinusStep;
	const stepSquared = step * step;
	const stepCubed = stepSquared * step;
	return {
		x:
			curve.p0.x * oneMinusCubed +
			3 * curve.p1.x * oneMinusSquared * step +
			3 * curve.p2.x * oneMinusStep * stepSquared +
			curve.p3.x * stepCubed,
		y:
			curve.p0.y * oneMinusCubed +
			3 * curve.p1.y * oneMinusSquared * step +
			3 * curve.p2.y * oneMinusStep * stepSquared +
			curve.p3.y * stepCubed,
	};
}

async function boardEdgeMidClientPoint(page: Page, edgeId: string): Promise<{ clientX: number; clientY: number }> {
	const point = await page.evaluate((nextEdgeId) => {
		const element = document.querySelector('[data-testid="board-canvas"]');
		if (!(element instanceof HTMLCanvasElement)) {
			return null;
		}
		const boardElement = element as BoardCanvasDebugElement;
		const renderer = boardElement.__boardRenderer;
		const edge = renderer?.scene.edges.get(nextEdgeId);
		if (!renderer || !edge) {
			return null;
		}
		const mid = cubicBezierPoint(edge.curve, 0.5);
		const screenPoint = renderer.worldToScreen(mid);
		const rect = boardElement.getBoundingClientRect();
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
	const canvas = page.getByTestId("board-canvas");
	await expect(canvas).toBeVisible();
	await expect
		.poll(async () => canvas.getAttribute("data-board-zoom"), { timeout: 30000 })
		.toMatch(/\d+(\.\d+)?/);
	expect(pageErrors.map((error) => error.message)).toEqual([]);
	expect(significantConsoleErrors(consoleErrors)).toEqual([]);
	return canvas;
}

test("board default: selection, zoom in to fine LOD, clear selection", async ({ page }) => {
	const canvas = await expectBoardStory(page, "elements-board--default");
	await expect(canvas).toHaveAttribute("data-board-raster", "gpu");
	await expect(canvas).toHaveAttribute("data-board-world-tiling", "none");
	await expect(canvas).toHaveAttribute("data-board-lod", "full");

	const initialZoom = Number(await canvas.getAttribute("data-board-zoom"));
	expect(initialZoom).toBeCloseTo(1, 1);

	await clickBoardObject(page, "alpha");
	await expect(canvas).toHaveAttribute("data-board-selection", "alpha");

	for (let index = 0; index < 18; index += 1) {
		await wheelOnCanvasNormalized(page, canvas, 0.5, 0.5, -120);
	}
	await expect.poll(async () => canvas.getAttribute("data-board-lod")).toBe("fine");
	const zoomed = Number(await canvas.getAttribute("data-board-zoom"));
	expect(zoomed).toBeGreaterThan(initialZoom);

	await clickCanvasNormalized(page, canvas, 0.04, 0.04);
	await expect(canvas).toHaveAttribute("data-board-selection", "");
});

test("board default: deletes selected node after Delete and keeps scene in sync", async ({ page }) => {
	const canvas = await expectBoardStory(page, "elements-board--default");
	await clickBoardObject(page, "beta");
	await expect(canvas).toHaveAttribute("data-board-selection", "beta");
	await page.keyboard.press("Delete");
	await expect
		.poll(async () =>
			page.evaluate(() => {
				const element = document.querySelector('[data-testid="board-canvas"]');
				if (!(element instanceof HTMLCanvasElement)) {
					return null;
				}
				const boardElement = element as BoardCanvasDebugElement;
				return boardElement.__boardRenderer?.scene.getObjectById("beta") ? "present" : "absent";
			}),
		)
		.toBe("absent");
	await expect(canvas).toHaveAttribute("data-board-selection", "");
});

test("board default: deletes selected edge after Delete", async ({ page }) => {
	const canvas = await expectBoardStory(page, "elements-board--default");
	const mid = await boardEdgeMidClientPoint(page, "link-1");
	await page.mouse.click(mid.clientX, mid.clientY);
	await expect(canvas).toHaveAttribute("data-board-selection", "link-1");
	await page.keyboard.press("Delete");
	await expect
		.poll(async () =>
			page.evaluate(() => {
				const element = document.querySelector('[data-testid="board-canvas"]');
				if (!(element instanceof HTMLCanvasElement)) {
					return null;
				}
				const boardElement = element as BoardCanvasDebugElement;
				return boardElement.__boardRenderer?.scene.edges.has("link-1") ? "present" : "absent";
			}),
		)
		.toBe("absent");
	await expect(canvas).toHaveAttribute("data-board-selection", "");
});

test("board default: zoom out to grid-only LOD while preserving wheel anchor", async ({ page }) => {
	const canvas = await expectBoardStory(page, "elements-board--default");
	const box = await canvas.boundingBox();
	expect(box).toBeTruthy();
	const [cx, cy] = viewportCenterOfCanvasBox(box!);

	const worldBefore = await page.evaluate(([px, py]) => {
		const el = document.querySelector("[data-testid=\"board-canvas\"]") as HTMLCanvasElement | null;
		if (!el) {
			return null;
		}
		const rect = el.getBoundingClientRect();
		const screenX = px - rect.left;
		const screenY = py - rect.top;
		const width = el.clientWidth;
		const height = el.clientHeight;
		const zoom = Number(el.dataset.boardZoom ?? "1");
		const [camXRaw, camYRaw] = (el.getAttribute("data-board-camera") ?? "0,0").split(",");
		const camX = Number(camXRaw);
		const camY = Number(camYRaw);
		return {
			x: (screenX - width / 2) / zoom + camX,
			y: (screenY - height / 2) / zoom + camY,
		};
	}, [cx, cy]);
	expect(worldBefore).not.toBeNull();

	for (let index = 0; index < 40; index += 1) {
		await page.mouse.move(cx, cy);
		await page.mouse.wheel(0, 140);
	}
	await expect.poll(async () => canvas.getAttribute("data-board-lod")).toBe("grid-only");

	const worldAfter = await page.evaluate(([px, py]) => {
		const el = document.querySelector("[data-testid=\"board-canvas\"]") as HTMLCanvasElement | null;
		if (!el) {
			return null;
		}
		const rect = el.getBoundingClientRect();
		const screenX = px - rect.left;
		const screenY = py - rect.top;
		const width = el.clientWidth;
		const height = el.clientHeight;
		const zoom = Number(el.dataset.boardZoom ?? "1");
		const [camXRaw, camYRaw] = (el.getAttribute("data-board-camera") ?? "0,0").split(",");
		const camX = Number(camXRaw);
		const camY = Number(camYRaw);
		return {
			x: (screenX - width / 2) / zoom + camX,
			y: (screenY - height / 2) / zoom + camY,
		};
	}, [cx, cy]);
	expect(worldAfter).not.toBeNull();
	expect(Math.abs((worldAfter as { x: number }).x - (worldBefore as { x: number }).x)).toBeLessThan(2.5);
	expect(Math.abs((worldAfter as { y: number }).y - (worldBefore as { y: number }).y)).toBeLessThan(2.5);
});

const nakaginCapsuleTowerHubPieceId = "9d18882e-d90b-40de-a171-47cb4564ffa6";

test("board nakagin fixture: json scene hub piece selects", async ({ page }) => {
	const canvas = await expectBoardStory(page, "elements-board--nakagin-capsule-tower-flat-selection");
	await expect(canvas).toHaveAttribute("data-board-raster", "gpu");
	await expect(canvas).toHaveAttribute("data-board-world-tiling", "none");
	await clickBoardObject(page, nakaginCapsuleTowerHubPieceId);
	await expect(canvas).toHaveAttribute("data-board-selection", nakaginCapsuleTowerHubPieceId);
});

test("board world-clip: raster mode, node selection, handle hit", async ({ page }) => {
	const canvas = await expectBoardStory(page, "elements-board--world-tile-clip");
	await expect(canvas).toHaveAttribute("data-board-raster", "gpu");
	await expect(canvas).toHaveAttribute("data-board-world-tiling", "world-clip");

	await clickBoardObject(page, "alpha");
	await expect(canvas).toHaveAttribute("data-board-selection", "alpha");

	await clickBoardObject(page, "alpha.out");
	await expect(canvas).toHaveAttribute("data-board-selection", "alpha.out");
});
