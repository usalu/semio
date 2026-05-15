// #region 🧲Header
// 💻 elements/client/lib/board/play/e2e/board-play-vello.spec.ts — Asserts WebGPU Vello path paints (not an empty canvas).
// #endregion 🧲Header

import { expect, test } from "@playwright/test";

test.describe("board play", () => {
	test.beforeEach(async ({ page }) => {
		await page.setViewportSize({ width: 1600, height: 900 });
	});

	test("each board canvas reaches Vello ready state", async ({ page }) => {
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
							els.map((el) => `${el.getAttribute("data-board-vello-state") ?? "?"}/${el.getAttribute("data-board-raster") ?? "?"}`),
						);
					},
					{ timeout: 120_000 },
				)
				.toEqual(["ready/vello", "ready/vello", "ready/vello"]);
		} catch (cause) {
			const detail =
				(await page.locator('[data-testid="board-canvas"]').first().getAttribute("data-board-vello-failure")) ?? "(no data-board-vello-failure)";
			throw new Error(`Expected three ready Vello canvases. data-board-vello-failure (first canvas): ${detail}`, { cause });
		}
		for (const c of await canvases.all()) {
			await expect(c).toHaveAttribute("data-board-raster", "vello");
			await expect(c).toHaveAttribute("data-board-vello-state", "ready");
		}
	});
});
