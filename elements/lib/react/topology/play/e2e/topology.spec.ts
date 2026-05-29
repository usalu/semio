import { expect, test } from "@playwright/test";

test("topology play mounts board and scene shells", async ({ page }) => {
	await page.goto("/");
	await expect(page.locator("[data-topology-board-root]")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator('[data-measure-id="topology-board-lod"]')).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-topology-scene-root]")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible({ timeout: 120_000 });
	await expect
		.poll(
			async () =>
				(await page.locator('[data-measure-id="topology-scene-lod"]').isVisible()) ||
				(await page.locator('[data-measure-id="topology-scene-auto"]').isVisible()),
			{ timeout: 120_000 },
		)
		.toBe(true);
});
