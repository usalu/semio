import { expect, test, type Page } from "@playwright/test";

/** Must match `TOPOLOGY_PLAY_WINDOW_LABELS.scene` in `../index.tsx` (Golden Layout tab title). */
const TOPOLOGY_SCENE_TAB_TITLE = "Spatial scene";

async function focusGoldenLayoutTabByTitle(page: Page, title: string): Promise<void> {
	await page.evaluate((tabTitle) => {
		const titles = [...document.querySelectorAll("span.lm_title")] as HTMLElement[];
		const el = titles.find((e) => e.textContent?.trim() === tabTitle);
		const tab = el?.closest(".lm_tab") as HTMLElement | undefined;
		tab?.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
		tab?.dispatchEvent(new MouseEvent("mouseup", { bubbles: true }));
		tab?.click();
	}, title);
}

test("topology play mounts board and scene shells", async ({ page }) => {
	await page.goto("/");
	await expect(page.locator("[data-topology-board-root]")).toBeVisible({ timeout: 120_000 });
	await focusGoldenLayoutTabByTitle(page, TOPOLOGY_SCENE_TAB_TITLE);
	await expect(page.locator("[data-topology-scene-root]")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible({ timeout: 120_000 });
});
