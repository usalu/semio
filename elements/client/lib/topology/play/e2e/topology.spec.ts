import { expect, test } from "@playwright/test";

test("topology play mounts board and scene shells", async ({ page }) => {
	await page.goto("/");
	await expect(page.locator("[data-topology-board-root]")).toBeVisible({ timeout: 120_000 });
	await page.evaluate(() => {
		const titles = [...document.querySelectorAll("span.lm_title")] as HTMLElement[];
		const t = titles.find((el) => el.textContent?.trim() === "Spatial scene");
		const tab = t?.closest(".lm_tab") as HTMLElement | undefined;
		tab?.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
		tab?.dispatchEvent(new MouseEvent("mouseup", { bubbles: true }));
		tab?.click();
	});
	await expect(page.locator("[data-topology-scene-root]")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible({ timeout: 120_000 });
});
