import { expect, test, type Page } from "@playwright/test";

function collectSceneConsole(page: Page): string[] {
	const messages: string[] = [];
	page.on("console", (message) => messages.push(message.text()));
	page.on("pageerror", (error) => messages.push(error.message));
	return messages;
}

function expectCleanSceneConsole(messages: string[]): void {
	const text = messages.join("\n");
	expect(text).not.toContain("Multiple instances of Three.js");
	expect(text).not.toContain("indirectCount is not defined");
	expect(text).not.toContain("React does not recognize the `asChild` prop");
	expect(text).not.toContain("computeBoundingSphere");
	expect(text).not.toContain("Could not load /meshes/");
	expect(text).not.toContain("An error occurred in the <CanvasImpl> component");
	expect(text).not.toContain("THREE.WebGLRenderer: Context Lost");
}

test("scene play loads canvas and fixture", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await expect(page.locator("canvas")).toBeVisible({ timeout: 120_000 });
	await expect(page.locator("[data-scene-root]")).toBeVisible();
	await page.waitForLoadState("networkidle");
	await page.waitForTimeout(500);
	expectCleanSceneConsole(messages);
});

test("scene selection hook updates label", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	await page.locator("canvas").first().waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	const id = await page.evaluate(() => {
		const w = window as unknown as { __scenePlaySelect?: (id: string) => void };
		w.__scenePlaySelect?.("01890804-66f2-4544-98f0-b6f0c0615492");
		return "01890804-66f2-4544-98f0-b6f0c0615492";
	});
	await expect(page.locator("[data-e2e-selected]")).toContainText(id.slice(0, 8), { timeout: 10_000 });
	expectCleanSceneConsole(messages);
});

test("scene click keeps chunked meshes mounted", async ({ page }) => {
	const messages = collectSceneConsole(page);
	await page.goto("/");
	const canvas = page.locator("canvas").first();
	await canvas.waitFor({ state: "visible", timeout: 120_000 });
	await page.waitForLoadState("networkidle");
	await page.waitForTimeout(500);
	const before = await page.locator("canvas").count();
	await canvas.click({ position: { x: 320, y: 240 } });
	await page.waitForTimeout(500);
	await expect(page.locator("[data-scene-root]")).toBeVisible();
	await expect(page.locator("canvas")).toHaveCount(before);
	expectCleanSceneConsole(messages);
});

test("scene play serves placeholder mesh as binary glb", async ({ request }) => {
	const response = await request.get("/meshes/placeholder.glb");
	expect(response.ok()).toBe(true);
	expect(response.headers()["content-type"]).toContain("model/gltf-binary");
	const body = await response.body();
	expect(body.subarray(0, 4).toString("ascii")).toBe("glTF");
});
