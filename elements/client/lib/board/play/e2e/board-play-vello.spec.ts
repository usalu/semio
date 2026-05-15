// #region 🧲Header
// 💻 elements/client/lib/board/play/e2e/board-play-vello.spec.ts — Asserts WebGPU Vello path paints (not an empty canvas).
// #endregion 🧲Header

import { expect, test } from "@playwright/test";

test.describe("board play", () => {
	test("each board canvas reaches Vello ready state", async ({ page }) => {
		await page.goto("/");
		await expect(page.getByText("Fixture shelf", { exact: false })).toBeVisible({ timeout: 60_000 });
		const canvases = page.locator('[data-testid="board-canvas"]');
		await expect(canvases).toHaveCount(3, { timeout: 60_000 });
		await expect(page.locator('[data-board-vello-state="ready"]')).toHaveCount(3, { timeout: 90_000 });
		for (const c of await canvases.all()) {
			await expect(c).toHaveAttribute("data-board-raster", "vello");
			await expect(c).toHaveAttribute("data-board-vello-state", "ready");
		}
	});
});
