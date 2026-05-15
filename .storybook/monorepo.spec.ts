// #region 🧲Header
// 💻 .storybook/monorepo.spec.ts
// Specs: Smoke-test one representative Storybook story from each aggregated workspace story tree.
// Summary: Verifies the root monorepo Storybook can render elements, semio UI, and algorithms stories without runtime errors.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲Header

import { expect, test, type Page } from "@playwright/test";

function significantConsoleErrors(messages: string[]): string[] {
	return messages.filter((text) => !/Failed to load resource:.*\b404\b/i.test(text));
}

async function expectStoryToRender(page: Page, storyId: string, assertion: (page: Page) => Promise<void>): Promise<void> {
	const pageErrors: Error[] = [];
	const consoleErrors: string[] = [];
	page.on("pageerror", (error) => pageErrors.push(error));
	page.on("console", (message) => {
		if (message.type() === "error") {
			consoleErrors.push(message.text());
		}
	});

	await page.goto(`iframe.html?id=${storyId}&viewMode=story`, { waitUntil: "domcontentloaded" });
	await expect(page.locator("body")).not.toContainText("Couldn't find story matching");
	await expect(page.locator("body")).not.toContainText("Failed to load the Storybook preview file");
	await assertion(page);
	expect(pageErrors.map((error) => error.message)).toEqual([]);
	expect(significantConsoleErrors(consoleErrors)).toEqual([]);
}

test("renders the elements button story", async ({ page }) => {
	await expectStoryToRender(page, "elements-react-button--default", async (storyPage) => {
		await expect(storyPage.getByRole("button").first()).toBeVisible();
	});
});

test("renders the semio vec story", async ({ page }) => {
	await expectStoryToRender(page, "semio-react-vec--default", async (storyPage) => {
		await expect(storyPage.locator("svg").first()).toBeVisible();
	});
});

test("renders the algorithms drag story", async ({ page }) => {
	await expectStoryToRender(page, "semio-algorithms-drag--default", async (storyPage) => {
		await expect(storyPage.getByText("Vec")).toBeVisible();
		await expect(storyPage.getByText("Input")).toBeVisible();
		await expect(storyPage.getByText("Diff")).toBeVisible();
		await expect(storyPage.getByText("Output")).toBeVisible();
	});
});