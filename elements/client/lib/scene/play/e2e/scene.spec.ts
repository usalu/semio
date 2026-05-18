import { expect, test } from "@playwright/test";

test("scene play loads canvas and fixture", async ({ page }) => {
	await page.goto("/");
	await expect(page.locator("canvas")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible();
});

test("scene selection hook updates label", async ({ page }) => {
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	const id = await page.evaluate(() => {
		const w = window as unknown as { __scenePlaySelect?: (id: string) => void };
		w.__scenePlaySelect?.("01890804-66f2-4544-98f0-b6f0c0615492");
		return "01890804-66f2-4544-98f0-b6f0c0615492";
	});
	await expect(page.locator("[data-e2e-selected]")).toContainText(id.slice(0, 8), { timeout: 10_000 });
});
