// #region 🧲Header
// 💻 elements/client/lib/board/play/e2e/board-play-gpu.spec.ts — Asserts WebGPU raster path paints (not an empty canvas).
// #endregion 🧲Header

import { expect, test } from "@playwright/test";

test.describe("board play", () => {
	test.beforeEach(async ({ page }) => {
		await page.setViewportSize({ width: 1600, height: 900 });
	});

	test("each board canvas reaches GPU ready state", async ({ page }) => {
		await page.goto("/", { waitUntil: "load", timeout: 180_000 });
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
			throw new Error(`Expected three ready GPU canvases. data-board-surface-failure (first canvas): ${detail}`, { cause });
		}
		for (const c of await canvases.all()) {
			await expect(c).toHaveAttribute("data-board-raster", "gpu");
			await expect(c).toHaveAttribute("data-board-surface-state", "ready");
		}
	});
});
