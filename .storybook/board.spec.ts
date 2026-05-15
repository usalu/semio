// #region 🧲Header
// 💻 .storybook/board.spec.ts
// Specs: End-to-end checks for the elements board canvas inside the aggregated Storybook static build.
// Summary: Covers selection, wheel zoom, LOD labels, and monolithic vs world-clip raster paths.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Locator, type Page } from "@playwright/test";

function significantConsoleErrors(messages: string[]): string[] {
	return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function clickCanvasNormalized(page: Page, canvas: Locator, nx: number, ny: number): Promise<void> {
	const box = await canvas.boundingBox();
	expect(box).toBeTruthy();
	const clientX = box!.x + nx * box!.width;
	const clientY = box!.y + ny * box!.height;
	await page.evaluate(
		({ nextClientX, nextClientY }) => {
			const element = document.querySelector('[data-testid="board-canvas"]');
			if (!(element instanceof HTMLCanvasElement)) {
				throw new Error('Board canvas not found.');
			}
			const move = new PointerEvent('pointermove', {
				bubbles: true,
				button: 0,
				buttons: 0,
				cancelable: true,
				clientX: nextClientX,
				clientY: nextClientY,
				composed: true,
				pointerId: 1,
				pointerType: 'mouse',
			});
			const down = new PointerEvent('pointerdown', {
				bubbles: true,
				button: 0,
				buttons: 1,
				cancelable: true,
				clientX: nextClientX,
				clientY: nextClientY,
				composed: true,
				pointerId: 1,
				pointerType: 'mouse',
			});
			const up = new PointerEvent('pointerup', {
				bubbles: true,
				button: 0,
				buttons: 0,
				cancelable: true,
				clientX: nextClientX,
				clientY: nextClientY,
				composed: true,
				pointerId: 1,
				pointerType: 'mouse',
			});
			element.dispatchEvent(move);
			element.dispatchEvent(down);
			element.dispatchEvent(up);
		},
		{ nextClientX: clientX, nextClientY: clientY },
	);
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
	await expect(canvas).toHaveAttribute("data-board-raster", "none");
	await expect(canvas).toHaveAttribute("data-board-lod", "full");

	const initialZoom = Number(await canvas.getAttribute("data-board-zoom"));
	expect(initialZoom).toBeCloseTo(1, 1);

	await clickCanvasNormalized(page, canvas, 0.5, 0.5);
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

test("board world-clip: raster mode, node selection, handle hit", async ({ page }) => {
	const canvas = await expectBoardStory(page, "elements-board--world-tile-clip");
	await expect(canvas).toHaveAttribute("data-board-raster", "world-clip");

	await clickCanvasNormalized(page, canvas, 0.5, 0.5);
	await expect(canvas).toHaveAttribute("data-board-selection", "alpha");

	const box = await canvas.boundingBox();
	expect(box).toBeTruthy();
	const nx = 0.5 + 44 / box!.width;
	await clickCanvasNormalized(page, canvas, nx, 0.5);
	await expect(canvas).toHaveAttribute("data-board-selection", "alpha.out");
});
