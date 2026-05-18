import { expect, test } from "@playwright/test";

test("topology play mounts board and scene shells", async ({ page }) => {
	await page.goto("/");
	await expect(page.locator("[data-topology-board-root]")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-topology-scene-root]")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible({ timeout: 120_000 });
});
