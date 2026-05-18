import { expect, test, type Page } from "@playwright/test";

function collectSceneConsole(page: Page): string[] {
	const messages: string[] = [];
	page.on("console", (message) => messages.push(message.text()));
	page.on("pageerror", (error) => messages.push(error.message));
	return messages;
}

test("scene play loads canvas and fixture", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await expect(page.locator("canvas")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible();
	expect(messages.join("\n")).not.toContain("Multiple instances of Three.js");
	expect(messages.join("\n")).not.toContain("indirectCount is not defined");
});

test("scene selection hook updates label", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	const id = await page.evaluate(() => {
		const w = window as unknown as { __scenePlaySelect?: (id: string) => void };
		w.__scenePlaySelect?.("01890804-66f2-4544-98f0-b6f0c0615492");
		return "01890804-66f2-4544-98f0-b6f0c0615492";
	});
	await expect(page.locator("[data-e2e-selected]")).toContainText(id.slice(0, 8), { timeout: 10_000 });
	expect(messages.join("\n")).not.toContain("Multiple instances of Three.js");
	expect(messages.join("\n")).not.toContain("indirectCount is not defined");
});
