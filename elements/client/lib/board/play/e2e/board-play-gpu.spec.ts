// #region 🧲Header
// 💻 elements/client/lib/board/play/e2e/board-play-gpu.spec.ts — Asserts WebGPU raster path paints (not an empty canvas).
// #endregion 🧲Header

import { expect, test } from "@playwright/test";

test.describe("board play", () => {
	test.beforeEach(async ({ page }) => {
		await page.setViewportSize({ width: 1600, height: 900 });
	});

	test("opens board background context menu on overview canvas", async ({ page }) => {
		await page.goto("/", { waitUntil: "load", timeout: 180_000 });
		await expect(page.getByText("Fixture shelf", { exact: false })).toBeVisible({ timeout: 120_000 });
		const canvas = page.locator('[data-testid="board-canvas"]').first();
		await expect(canvas).toBeVisible({ timeout: 120_000 });
			await canvas.click({ button: "right", position: { x: 24, y: 24 } });
		await expect(page.getByRole("menuitem", { name: "Board background menu" })).toBeVisible({ timeout: 30_000 });
	});

	test("each board canvas reaches GPU ready state", async ({ page }, testInfo) => {
		await page.goto("/", { waitUntil: "load", timeout: 180_000 });
		const adapterOk = await page.evaluate(async () => {
			const gpu = globalThis.navigator?.gpu;
			if (!gpu) return false;
			const adapter = await gpu.requestAdapter();
			return adapter != null;
		});
		if (!adapterOk) {
			testInfo.skip(true, "No WebGPU adapter reported by the browser");
		}
		await expect(page.getByText("Fixture shelf", { exact: false })).toBeVisible({ timeout: 120_000 });
		const canvases = page.locator('[data-testid="board-canvas"]');
		await expect(canvases).toHaveCount(3, { timeout: 180_000 });
		try {
			await expect
				.poll(
					async () => {
						const loc = page.locator('[data-testid="board-canvas"]');
						return await loc.evaluateAll((els) =>
							els.map((el) => `${el.getAttribute("data-board-surface-state") ?? "?"}/${el.getAttribute("data-board-raster") ?? "?"}`),
						);
					},
					{ timeout: 120_000 },
				)
				.toEqual(["ready/gpu", "ready/gpu", "ready/gpu"]);
		} catch (cause) {
			const detail =
				(await page.locator('[data-testid="board-canvas"]').first().getAttribute("data-board-surface-failure")) ?? "(no data-board-surface-failure)";
			if (detail === "NoCompatibleDevice") {
				testInfo.skip(true, `WebGPU surface unavailable: ${detail}`);
			}
			throw new Error(`Expected three ready GPU canvases. data-board-surface-failure (first canvas): ${detail}`, { cause });
		}
		for (const c of await canvases.all()) {
			await expect(c).toHaveAttribute("data-board-raster", "gpu");
			await expect(c).toHaveAttribute("data-board-surface-state", "ready");
		}
	});
});
